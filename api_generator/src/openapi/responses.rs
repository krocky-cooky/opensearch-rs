/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! Builds an intermediate representation (IR) of typed response models
//! from the `components/responses` section of the OpenAPI specification.
//!
//! For each requested operation group, the 200 response schema is resolved
//! and its transitive schema closure is converted into [IrType]s, from
//! which the `response_types` code generation backend renders Rust types
//! into `opensearch/src/types/`.
//!
//! # Union strategies
//!
//! JSON carries no discriminator for `oneOf`/`anyOf` values, so unions are
//! classified into one of three shapes:
//!
//! - **String enums**: unions whose variants are all fixed string values
//!   become open-set Rust enums (unknown values fall into an `Unknown`
//!   variant instead of failing the whole response).
//! - **Dispatch unions**: when every variant has a distinct JSON kind
//!   (object / array / string / number / boolean), a Rust enum with a
//!   generated `Deserialize` visitor dispatches on the kind of the
//!   incoming value. No trial-and-error decoding takes place.
//! - **Lazy unions**: when variant shapes overlap (notably aggregations,
//!   where dozens of variants are all objects), the raw JSON is captured
//!   unparsed and only decoded when a branch accessor is called by the
//!   caller, who knows which branch to expect having written the request.

use anyhow::{anyhow, bail};
use inflector::Inflector;
use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// The intermediate representation of the typed response models for a set
/// of operation groups
#[derive(Debug, Default)]
pub struct ResponseIr {
    /// operation group name -> IR name of the root response type
    pub endpoints: BTreeMap<String, String>,
    /// IR name (`namespace___TypeName`) -> type definition
    pub types: BTreeMap<String, IrType>,
}

/// A type in the response IR, keyed by its IR name
#[derive(Debug)]
pub enum IrType {
    Struct(IrStruct),
    /// A union whose variants have pairwise distinct JSON kinds
    DispatchUnion(IrUnion),
    /// A union whose variants cannot be told apart cheaply; decoded lazily
    LazyUnion(IrUnion),
    /// An open set of fixed string values
    StrEnum(IrStrEnum),
    /// A named alias of a scalar, list, map or raw type
    Alias(IrAlias),
}

#[derive(Debug)]
pub struct IrStruct {
    pub description: Option<String>,
    pub fields: Vec<IrField>,
    /// Typed `additionalProperties` alongside fixed fields: the remaining
    /// keys are collected into a map field with this (name, value type)
    pub residual: Option<(String, IrRef)>,
}

#[derive(Debug)]
pub struct IrField {
    /// The JSON property name
    pub json_name: String,
    pub ty: IrRef,
    /// Present in the schema's `required` list
    pub required: bool,
    /// The schema allows an explicit `null` value
    pub nullable: bool,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct IrUnion {
    pub description: Option<String>,
    pub variants: Vec<IrVariant>,
    /// The union has a `null` variant
    pub nullable: bool,
}

#[derive(Debug)]
pub struct IrVariant {
    /// Variant name in snake_case, from the variant `title` or `$ref` name
    pub name: String,
    pub ty: IrRef,
    /// JSON kind, filled in by strategy resolution; `None` when the kind
    /// cannot be determined statically
    pub kind: Option<JsonKind>,
}

#[derive(Debug)]
pub struct IrStrEnum {
    pub description: Option<String>,
    pub values: Vec<String>,
}

#[derive(Debug)]
pub struct IrAlias {
    pub description: Option<String>,
    pub ty: IrRef,
}

/// A reference to a type from a field, item, or variant position
#[derive(Debug, Clone, PartialEq)]
pub enum IrRef {
    /// Another type in the IR, by IR name
    Named(String),
    Scalar(Scalar),
    List(Box<IrRef>),
    /// An object with arbitrary keys and typed values
    Map(Box<IrRef>),
    /// Untyped JSON: the escape hatch for shapes decided at runtime
    /// (documents, generic type parameters, untyped `additionalProperties`)
    Raw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scalar {
    Bool,
    I32,
    I64,
    U64,
    /// JSON numbers are 64-bit: `float` schemas also map here to keep
    /// round-trips lossless
    F64,
    String,
}

/// The JSON value kind a schema accepts, used for union dispatch
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonKind {
    Object,
    Array,
    String,
    Number,
    Bool,
}

/// Reads the response IR for the given operation groups from a parsed
/// OpenAPI document
pub fn read_response_ir(spec: &Value, operation_groups: &[&str]) -> anyhow::Result<ResponseIr> {
    let schemas = spec["components"]["schemas"]
        .as_object()
        .ok_or_else(|| anyhow!("OpenAPI document has no components/schemas"))?;
    let responses = spec["components"]["responses"]
        .as_object()
        .ok_or_else(|| anyhow!("OpenAPI document has no components/responses"))?;

    let mut builder = IrBuilder {
        schemas,
        ir: ResponseIr::default(),
        queue: VecDeque::new(),
        queued: BTreeSet::new(),
    };

    for group in operation_groups {
        let response = responses
            .get(&format!("{}___200", group))
            .ok_or_else(|| anyhow!("no 200 response found for operation group {}", group))?;
        let schema = &response["content"]["application/json"]["schema"];
        if schema.is_null() {
            bail!("200 response of {} has no application/json schema", group);
        }

        let root = match schema["$ref"].as_str() {
            // a plain reference: the referenced schema is the root type
            Some(r) => {
                let name = schema_ref_name(r)?;
                builder.enqueue(name);
                name.to_string()
            }
            // an inline schema: synthesize a root type named after the group
            None => {
                let name = format!(
                    "{}___{}Response",
                    root_namespace(group),
                    group.split('.').next_back().unwrap().to_pascal_case()
                );
                let ty = builder.convert_schema(schema, &name)?;
                builder.ir.types.insert(name.clone(), ty);
                name
            }
        };
        builder.ir.endpoints.insert(group.to_string(), root);
    }

    // drain the worklist of referenced schemas
    while let Some(name) = builder.queue.pop_front() {
        let schema = builder
            .schemas
            .get(&name)
            .ok_or_else(|| anyhow!("unresolved schema reference: {}", name))?
            .clone();
        let ty = builder.convert_schema(&schema, &name)?;
        builder.ir.types.insert(name, ty);
    }

    resolve_union_strategies(&mut builder.ir);
    prune_unreachable(&mut builder.ir);

    Ok(builder.ir)
}

/// Drops types not reachable from any endpoint root: merging specialized
/// property declarations with their generic bases can leave intermediate
/// synthesized types without references
fn prune_unreachable(ir: &mut ResponseIr) {
    fn mark(ty: &IrRef, ir: &ResponseIr, reachable: &mut BTreeSet<String>) {
        match ty {
            IrRef::Named(n) => {
                if reachable.insert(n.clone()) {
                    if let Some(t) = ir.types.get(n) {
                        mark_type(t, ir, reachable);
                    }
                }
            }
            IrRef::List(t) | IrRef::Map(t) => mark(t, ir, reachable),
            _ => {}
        }
    }
    fn mark_type(ty: &IrType, ir: &ResponseIr, reachable: &mut BTreeSet<String>) {
        match ty {
            IrType::Struct(s) => {
                for f in &s.fields {
                    mark(&f.ty, ir, reachable);
                }
                if let Some((_, t)) = &s.residual {
                    mark(t, ir, reachable);
                }
            }
            IrType::DispatchUnion(u) | IrType::LazyUnion(u) => {
                for v in &u.variants {
                    mark(&v.ty, ir, reachable);
                }
            }
            IrType::Alias(a) => mark(&a.ty, ir, reachable),
            IrType::StrEnum(_) => {}
        }
    }

    let mut reachable = BTreeSet::new();
    for root in ir.endpoints.values() {
        mark(&IrRef::Named(root.clone()), ir, &mut reachable);
    }
    ir.types.retain(|name, _| reachable.contains(name));
}

/// The namespace used for a synthesized root response type of an operation
/// group, e.g. `get` -> `_core.get`
fn root_namespace(group: &str) -> String {
    match group.split_once('.') {
        Some((ns, _)) => ns.to_string(),
        None => format!("_core.{}", group),
    }
}

/// Extracts the schema name from a `#/components/schemas/` reference
fn schema_ref_name(r: &str) -> anyhow::Result<&str> {
    r.strip_prefix("#/components/schemas/")
        .ok_or_else(|| anyhow!("unsupported schema $ref: {}", r))
}

struct IrBuilder<'a> {
    schemas: &'a Map<String, Value>,
    ir: ResponseIr,
    queue: VecDeque<String>,
    queued: BTreeSet<String>,
}

impl IrBuilder<'_> {
    /// Adds a named schema to the conversion worklist
    fn enqueue(&mut self, name: &str) {
        if self.queued.insert(name.to_string()) {
            self.queue.push_back(name.to_string());
        }
    }

    /// The variant list a schema effectively is, unwrapping the
    /// `allOf [generic union $ref, specialized oneOf]` pattern used to
    /// specialize generic containers (e.g. `Buckets` of a concrete bucket
    /// type). Returns `None` when any part contributes fields, in which
    /// case the schema is a struct.
    ///
    /// Variant titles of a superseded generic union are inherited
    /// positionally by untitled variants of the specialization.
    fn effective_union(&self, schema: &Value, depth: usize) -> anyhow::Result<Option<Vec<Value>>> {
        if depth > 8 {
            return Ok(None);
        }
        if let Some(variants) = union_variants(schema) {
            return Ok(Some(variants));
        }
        let parts = match schema["allOf"].as_array() {
            Some(parts) => parts,
            None => return Ok(None),
        };
        let mut last_union: Option<Vec<Value>> = None;
        for part in parts {
            let resolved;
            let part = match part["$ref"].as_str() {
                Some(r) => {
                    let name = schema_ref_name(r)?;
                    resolved = self
                        .schemas
                        .get(name)
                        .ok_or_else(|| anyhow!("unresolved schema $ref: {}", r))?;
                    resolved
                }
                None => part,
            };
            // any part with fields makes this a struct, not a union
            if part["properties"].is_object() || part["additionalProperties"].is_object() {
                return Ok(None);
            }
            if let Some(mut variants) = self.effective_union(part, depth + 1)? {
                // inherit titles positionally from the superseded union
                if let Some(previous) = &last_union {
                    if previous.len() == variants.len() {
                        for (variant, prev) in variants.iter_mut().zip(previous) {
                            if variant["title"].is_null() && prev["title"].is_string() {
                                variant["title"] = prev["title"].clone();
                            }
                        }
                    }
                }
                // the most specialized (last) union wins
                last_union = Some(variants);
            }
        }
        Ok(last_union)
    }

    /// Converts a schema into an [IrType]. `name` is the IR name the type
    /// will be registered under, also used to derive names for synthesized
    /// inline types.
    fn convert_schema(&mut self, schema: &Value, name: &str) -> anyhow::Result<IrType> {
        let description = schema_description(schema);

        // unions of fixed string values become open string enums
        if let Some(values) = collect_const_values(schema, self.schemas)? {
            return Ok(IrType::StrEnum(IrStrEnum {
                description,
                values,
            }));
        }

        if let Some(variants) = self.effective_union(schema, 0)? {
            let union = self.convert_union(&variants, name, description)?;
            // strategy is resolved globally once the closure is complete;
            // start as lazy, resolve_union_strategies may upgrade
            return Ok(IrType::LazyUnion(union));
        }

        if is_object_like(schema) {
            return Ok(IrType::Struct(self.expand_object(schema, name)?));
        }

        // named scalar / list / map aliases, and unshaped schemas (Raw)
        let (ty, _nullable) = self.convert_ref(schema, name)?;
        Ok(IrType::Alias(IrAlias { description, ty }))
    }

    /// Converts a `oneOf`/`anyOf` into an [IrUnion], synthesizing types for
    /// inline object variants
    fn convert_union(
        &mut self,
        variants: &[Value],
        name: &str,
        description: Option<String>,
    ) -> anyhow::Result<IrUnion> {
        let mut ir_variants = Vec::new();
        let mut nullable = false;

        for variant in variants {
            if variant["type"].as_str() == Some("null") {
                nullable = true;
                continue;
            }
            let variant_name = match variant["title"].as_str() {
                Some(t) => t.to_snake_case(),
                None => match variant["$ref"].as_str() {
                    Some(r) => type_name(schema_ref_name(r)?).to_snake_case(),
                    None => variant_kind_name(variant),
                },
            };
            let ctx = format!("{}{}", name, variant_name.to_pascal_case());
            let (ty, _) = self.convert_ref(variant, &ctx)?;
            ir_variants.push(IrVariant {
                name: variant_name,
                ty,
                kind: None,
            });
        }

        Ok(IrUnion {
            description,
            variants: ir_variants,
            nullable,
        })
    }

    /// Expands an object schema — including its transitive `allOf` parts —
    /// into a flat list of fields (physical expansion; no flatten and no
    /// embedded base struct)
    fn expand_object(&mut self, schema: &Value, name: &str) -> anyhow::Result<IrStruct> {
        let mut props: Vec<(String, Vec<Value>)> = Vec::new();
        let mut required: BTreeSet<String> = BTreeSet::new();
        let mut residual: Option<Value> = None;

        self.collect_object_parts(schema, name, &mut props, &mut required, &mut residual, 0)?;

        let mut fields = Vec::new();
        for (prop_name, schemas) in props {
            let ctx = format!("{}{}", name, prop_name.to_pascal_case());
            let merged = self.merge_property_schemas(schemas)?;
            let (ty, nullable) = self.convert_ref(&merged, &ctx)?;
            fields.push(IrField {
                json_name: prop_name.clone(),
                ty,
                required: required.contains(&prop_name),
                nullable,
                description: schema_description(&merged),
            });
        }

        let residual = match residual {
            None => None,
            Some(Value::Bool(true)) => Some(("additional".to_string(), IrRef::Raw)),
            Some(ap) => {
                let ctx = format!("{}Additional", name);
                let (ty, _) = self.convert_ref(&ap, &ctx)?;
                // sub-aggregations keep their conventional field name
                let field_name = match &ty {
                    IrRef::Named(n) if n.ends_with("___Aggregate") => "aggregations",
                    _ => "additional",
                };
                Some((field_name.to_string(), ty))
            }
        };

        Ok(IrStruct {
            description: schema_description(schema),
            fields,
            residual,
        })
    }

    /// Merges the schemas a property was declared with across `allOf`
    /// parts:
    ///
    /// - arrays merge element-wise (a re-declared item field specializes
    ///   the item type)
    /// - object shapes merge field-wise via `allOf` (partial overlays such
    ///   as an added `_source` field)
    /// - anything else is a specialization: the most derived declaration —
    ///   the last one — replaces the base entirely (e.g. a concrete bucket
    ///   array replacing the generic `Buckets` union)
    fn merge_property_schemas(&self, mut schemas: Vec<Value>) -> anyhow::Result<Value> {
        schemas.dedup();
        if schemas.len() == 1 {
            return Ok(schemas.pop().unwrap());
        }

        let resolved: Vec<Value> = schemas
            .iter()
            .map(|s| self.resolve_shallow(s).cloned())
            .collect::<anyhow::Result<_>>()?;

        if resolved.iter().all(|s| s["type"].as_str() == Some("array")) {
            let items: Vec<Value> = resolved.iter().map(|s| s["items"].clone()).collect();
            let merged_items = self.merge_property_schemas(items)?;
            return Ok(serde_json::json!({ "type": "array", "items": merged_items }));
        }

        let object_like = |s: &Value| s["properties"].is_object() || s["allOf"].is_array();
        if resolved.iter().all(object_like) {
            return Ok(serde_json::json!({ "allOf": schemas }));
        }

        Ok(schemas.pop().unwrap())
    }

    /// Resolves a `$ref` chain one level deep enough to inspect the kind
    /// of a schema
    fn resolve_shallow<'a>(&'a self, schema: &'a Value) -> anyhow::Result<&'a Value> {
        let mut current = schema;
        for _ in 0..8 {
            match current["$ref"].as_str() {
                Some(r) => {
                    let name = schema_ref_name(r)?;
                    current = self
                        .schemas
                        .get(name)
                        .ok_or_else(|| anyhow!("unresolved schema $ref: {}", r))?;
                }
                None => return Ok(current),
            }
        }
        bail!("schema $ref chain too deep or cyclic");
    }

    /// Recursively collects property declarations, required sets and
    /// `additionalProperties` from a schema and its `allOf` parts. Every
    /// declaration of a property is kept, in declaration order, so that
    /// redefinitions can be merged with their base definitions.
    fn collect_object_parts(
        &mut self,
        schema: &Value,
        name: &str,
        props: &mut Vec<(String, Vec<Value>)>,
        required: &mut BTreeSet<String>,
        residual: &mut Option<Value>,
        depth: usize,
    ) -> anyhow::Result<()> {
        if depth > 16 {
            bail!("allOf expansion too deep (cycle?) in {}", name);
        }

        // a $ref part: resolve and recurse
        if let Some(r) = schema["$ref"].as_str() {
            let ref_name = schema_ref_name(r)?;
            let resolved = self
                .schemas
                .get(ref_name)
                .ok_or_else(|| anyhow!("unresolved schema $ref: {}", r))?
                .clone();
            return self.collect_object_parts(
                &resolved,
                name,
                props,
                required,
                residual,
                depth + 1,
            );
        }

        if let Some(parts) = schema["allOf"].as_array() {
            for part in parts.clone() {
                self.collect_object_parts(&part, name, props, required, residual, depth + 1)?;
            }
        }

        if let Some(properties) = schema["properties"].as_object() {
            for (prop_name, prop_schema) in properties.clone() {
                match props.iter_mut().find(|(n, _)| *n == prop_name) {
                    Some((_, schemas)) => schemas.push(prop_schema),
                    None => props.push((prop_name, vec![prop_schema])),
                }
            }
        }

        if let Some(req) = schema["required"].as_array() {
            for r in req {
                if let Some(r) = r.as_str() {
                    required.insert(r.to_string());
                }
            }
        }

        match &schema["additionalProperties"] {
            Value::Null | Value::Bool(false) => {}
            ap => {
                residual.get_or_insert(ap.clone());
            }
        }

        Ok(())
    }

    /// Converts a schema in a field / item / variant position into an
    /// [IrRef], synthesizing named types for inline objects, unions and
    /// enums. Returns the reference and whether `null` is allowed.
    fn convert_ref(&mut self, schema: &Value, ctx: &str) -> anyhow::Result<(IrRef, bool)> {
        // a reference to a named schema
        if let Some(r) = schema["$ref"].as_str() {
            let name = schema_ref_name(r)?;
            let target = self
                .schemas
                .get(name)
                .ok_or_else(|| anyhow!("unresolved schema $ref: {}", r))?;
            // generic type parameters (e.g. TDocument) have no schema; the
            // caller decides the type, so hold raw JSON
            if target["x-is-generic-type-parameter"].as_bool().unwrap_or(false) {
                return Ok((IrRef::Raw, false));
            }
            self.enqueue(name);
            return Ok((IrRef::Named(name.to_string()), false));
        }

        // inline unions, including allOf-specialized generic unions
        if let Some(variants) = self.effective_union(schema, 0)? {
            let mut nullable = false;
            let mut non_null: Vec<Value> = Vec::new();
            for v in &variants {
                if v["type"].as_str() == Some("null") {
                    nullable = true;
                } else {
                    non_null.push(v.clone());
                }
            }
            // [null, X] is just a nullable X
            if non_null.len() == 1 {
                let (ty, _) = self.convert_ref(&non_null[0], ctx)?;
                return Ok((ty, true));
            }
            // a union of fixed strings is an open string enum
            if let Some(values) = collect_const_values(schema, self.schemas)? {
                let ty = IrType::StrEnum(IrStrEnum {
                    description: schema_description(schema),
                    values,
                });
                self.ir.types.insert(ctx.to_string(), ty);
                return Ok((IrRef::Named(ctx.to_string()), nullable));
            }
            // otherwise synthesize a named union
            let union = self.convert_union(&variants, ctx, schema_description(schema))?;
            self.ir
                .types
                .insert(ctx.to_string(), IrType::LazyUnion(union));
            return Ok((IrRef::Named(ctx.to_string()), nullable));
        }

        // inline string enums
        if schema["enum"].is_array() || schema["const"].is_string() {
            if let Some(values) = collect_const_values(schema, self.schemas)? {
                let ty = IrType::StrEnum(IrStrEnum {
                    description: schema_description(schema),
                    values,
                });
                self.ir.types.insert(ctx.to_string(), ty);
                return Ok((IrRef::Named(ctx.to_string()), false));
            }
        }

        // inline objects with fixed fields (also allOf compositions)
        if is_object_like(schema) {
            let has_props = schema["properties"]
                .as_object()
                .map(|p| !p.is_empty())
                .unwrap_or(false)
                || schema["allOf"].is_array();
            if has_props {
                let ty = IrType::Struct(self.expand_object(schema, ctx)?);
                self.ir.types.insert(ctx.to_string(), ty);
                return Ok((IrRef::Named(ctx.to_string()), false));
            }
            // pure map: additionalProperties without fixed fields
            return match &schema["additionalProperties"] {
                Value::Null | Value::Bool(_) => Ok((IrRef::Map(Box::new(IrRef::Raw)), false)),
                ap => {
                    let (value, _) = self.convert_ref(ap, &format!("{}Value", ctx))?;
                    Ok((IrRef::Map(Box::new(value)), false))
                }
            };
        }

        Ok(match schema["type"].as_str() {
            Some("boolean") => (IrRef::Scalar(Scalar::Bool), false),
            Some("integer") => match schema["format"].as_str() {
                Some("int64") => (IrRef::Scalar(Scalar::I64), false),
                Some("uint64") => (IrRef::Scalar(Scalar::U64), false),
                _ => (IrRef::Scalar(Scalar::I32), false),
            },
            Some("number") => (IrRef::Scalar(Scalar::F64), false),
            Some("string") => (IrRef::Scalar(Scalar::String), false),
            Some("array") => {
                let items = &schema["items"];
                if items.is_null() {
                    (IrRef::List(Box::new(IrRef::Raw)), false)
                } else {
                    let (item, _) = self.convert_ref(items, &format!("{}Item", ctx))?;
                    (IrRef::List(Box::new(item)), false)
                }
            }
            Some("null") => (IrRef::Raw, true),
            // no type at all: untyped JSON
            _ => (IrRef::Raw, false),
        })
    }
}

/// The `oneOf`/`anyOf` variant list of a schema, if it is a union
fn union_variants(schema: &Value) -> Option<Vec<Value>> {
    schema["oneOf"]
        .as_array()
        .or_else(|| schema["anyOf"].as_array())
        .cloned()
}

/// Whether a schema describes an object (fixed fields, an allOf
/// composition, or a map)
fn is_object_like(schema: &Value) -> bool {
    schema["type"].as_str() == Some("object")
        || schema["properties"].is_object()
        || schema["allOf"].is_array()
        || schema["additionalProperties"].is_object()
}

fn schema_description(schema: &Value) -> Option<String> {
    schema["description"].as_str().map(str::to_string)
}

/// A placeholder variant name derived from the JSON kind of an untitled
/// inline variant
fn variant_kind_name(schema: &Value) -> String {
    match schema["type"].as_str() {
        Some(t) => t.to_string(),
        None => "value".to_string(),
    }
}

/// The type-name part of an IR name (`namespace___TypeName`)
pub fn type_name(ir_name: &str) -> &str {
    match ir_name.rsplit_once("___") {
        Some((_, n)) => n,
        None => ir_name,
    }
}

/// The namespace part of an IR name (`namespace___TypeName`)
pub fn namespace(ir_name: &str) -> &str {
    match ir_name.rsplit_once("___") {
        Some((ns, _)) => ns,
        None => "",
    }
}

/// Collects the fixed string values of a schema whose variants are all
/// consts / string enums, following `$ref`s. Returns `None` when any
/// variant allows other values.
fn collect_const_values(
    schema: &Value,
    schemas: &Map<String, Value>,
) -> anyhow::Result<Option<Vec<String>>> {
    fn collect(
        schema: &Value,
        schemas: &Map<String, Value>,
        depth: usize,
        out: &mut Vec<String>,
    ) -> anyhow::Result<bool> {
        if depth > 4 {
            return Ok(false);
        }
        let resolved;
        let schema = match schema["$ref"].as_str() {
            Some(r) => {
                let name = schema_ref_name(r)?;
                resolved = schemas
                    .get(name)
                    .ok_or_else(|| anyhow!("unresolved schema $ref: {}", r))?;
                resolved
            }
            None => schema,
        };

        if let Some(values) = schema["enum"].as_array() {
            if values.is_empty() || !values.iter().all(|v| v.is_string()) {
                return Ok(false);
            }
            out.extend(values.iter().map(|v| v.as_str().unwrap().to_string()));
            return Ok(true);
        }
        match &schema["const"] {
            Value::String(s) => {
                out.push(s.clone());
                return Ok(true);
            }
            Value::Null => {}
            _ => return Ok(false),
        }
        if let Some(variants) = union_variants(schema) {
            for variant in &variants {
                if !collect(variant, schemas, depth + 1, out)? {
                    return Ok(false);
                }
            }
            return Ok(true);
        }
        Ok(false)
    }

    let mut values = Vec::new();
    Ok(match collect(schema, schemas, 0, &mut values)? {
        true if !values.is_empty() => {
            let mut seen = BTreeSet::new();
            let deduped: Vec<String> = values
                .into_iter()
                .filter(|v| seen.insert(v.to_lowercase()))
                .collect();
            Some(deduped)
        }
        _ => None,
    })
}

/// Decides the decoding strategy of every union in the IR: unions whose
/// variants have pairwise distinct JSON kinds become dispatch unions,
/// everything else stays lazy
fn resolve_union_strategies(ir: &mut ResponseIr) {
    // compute variant kinds against the completed closure
    let kinds: BTreeMap<String, Option<JsonKind>> = ir
        .types
        .keys()
        .map(|name| (name.clone(), kind_of_named(name, ir, 0)))
        .collect();

    let mut upgrades: Vec<String> = Vec::new();
    for (name, ty) in &ir.types {
        if let IrType::LazyUnion(union) = ty {
            let variant_kinds: Vec<Option<JsonKind>> = union
                .variants
                .iter()
                .map(|v| kind_of_ref(&v.ty, &kinds))
                .collect();
            let all_known = variant_kinds.iter().all(|k| k.is_some());
            let distinct = {
                let mut seen = Vec::new();
                variant_kinds.iter().flatten().all(|k| {
                    if seen.contains(k) {
                        false
                    } else {
                        seen.push(*k);
                        true
                    }
                })
            };
            if all_known && distinct {
                upgrades.push(name.clone());
            }
        }
    }

    for name in upgrades {
        let kinds_snapshot = kinds.clone();
        if let Some(IrType::LazyUnion(mut union)) = ir.types.remove(&name) {
            for variant in &mut union.variants {
                variant.kind = kind_of_ref(&variant.ty, &kinds_snapshot);
            }
            ir.types.insert(name, IrType::DispatchUnion(union));
        }
    }
}

/// The JSON kind of a named type, resolved through aliases; `None` when
/// the type accepts multiple kinds (unions) or any kind (Raw)
fn kind_of_named(name: &str, ir: &ResponseIr, depth: usize) -> Option<JsonKind> {
    if depth > 8 {
        return None;
    }
    match ir.types.get(name)? {
        IrType::Struct(_) => Some(JsonKind::Object),
        IrType::StrEnum(_) => Some(JsonKind::String),
        IrType::DispatchUnion(_) | IrType::LazyUnion(_) => None,
        IrType::Alias(alias) => match &alias.ty {
            IrRef::Named(n) => kind_of_named(n, ir, depth + 1),
            other => kind_of_shallow(other),
        },
    }
}

/// The JSON kind of a non-named [IrRef]
fn kind_of_shallow(ty: &IrRef) -> Option<JsonKind> {
    match ty {
        IrRef::Scalar(Scalar::Bool) => Some(JsonKind::Bool),
        IrRef::Scalar(Scalar::String) => Some(JsonKind::String),
        IrRef::Scalar(_) => Some(JsonKind::Number),
        IrRef::List(_) => Some(JsonKind::Array),
        IrRef::Map(_) => Some(JsonKind::Object),
        IrRef::Raw => None,
        IrRef::Named(_) => None,
    }
}

/// The JSON kind of an [IrRef] against precomputed named-type kinds
fn kind_of_ref(ty: &IrRef, kinds: &BTreeMap<String, Option<JsonKind>>) -> Option<JsonKind> {
    match ty {
        IrRef::Named(n) => kinds.get(n).copied().flatten(),
        other => kind_of_shallow(other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Reads the IR of the four core APIs from the vendored specification
    fn core_ir() -> ResponseIr {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/opensearch-openapi.yaml");
        let file = std::fs::File::open(path).unwrap();
        let spec: Value = serde_yaml::from_reader(file).unwrap();
        read_response_ir(&spec, &["search", "index", "bulk", "get"]).unwrap()
    }

    #[test]
    fn maps_endpoints_to_root_types() {
        let ir = core_ir();
        assert_eq!(ir.endpoints["search"], "_core.search___SearchResponse");
        assert_eq!(ir.endpoints["index"], "_common___WriteResponseBase");
        assert_eq!(ir.endpoints["bulk"], "_core.bulk___BulkResponse");
        // get's 200 schema is inline: a root type is synthesized
        assert_eq!(ir.endpoints["get"], "_core.get___GetResponse");
        assert!(ir.types.contains_key("_core.get___GetResponse"));
    }

    #[test]
    fn expands_all_of_physically() {
        let ir = core_ir();
        // StringTermsBucket = TermsBucketBase(+MultiBucketBase) + {key}
        let bucket = match &ir.types["_common.aggregations___StringTermsBucket"] {
            IrType::Struct(s) => s,
            other => panic!("expected struct, got {:?}", other),
        };
        let names: Vec<&str> = bucket.fields.iter().map(|f| f.json_name.as_str()).collect();
        assert!(names.contains(&"key"), "own field: {:?}", names);
        assert!(names.contains(&"doc_count"), "from MultiBucketBase: {:?}", names);
        assert!(
            names.contains(&"doc_count_error_upper_bound"),
            "from TermsBucketBase: {:?}",
            names
        );
        // doc_count is required through the base
        let doc_count = bucket.fields.iter().find(|f| f.json_name == "doc_count").unwrap();
        assert!(doc_count.required);
        // sub-aggregations: residual keys from MultiBucketBase
        let (residual_name, residual_ty) = bucket.residual.as_ref().unwrap();
        assert_eq!(residual_name, "aggregations");
        assert_eq!(
            residual_ty,
            &IrRef::Named("_common.aggregations___Aggregate".to_string())
        );
    }

    #[test]
    fn aggregate_union_stays_lazy() {
        let ir = core_ir();
        let agg = match &ir.types["_common.aggregations___Aggregate"] {
            IrType::LazyUnion(u) => u,
            other => panic!("expected lazy union, got {:?}", other),
        };
        assert!(agg.variants.len() > 50);
        assert!(agg.variants.iter().any(|v| v.name == "avg"));
        assert!(agg.variants.iter().any(|v| v.name == "sterms"));
    }

    #[test]
    fn kind_disjoint_unions_become_dispatch() {
        let ir = core_ir();
        // buckets are specialized from the generic Buckets union per
        // aggregate: keyed map | array
        let buckets = match &ir.types["_common.aggregations___DateHistogramAggregateBuckets"] {
            IrType::DispatchUnion(u) => u,
            other => panic!("expected dispatch union, got {:?}", other),
        };
        assert_eq!(buckets.variants.len(), 2);
        assert_eq!(buckets.variants[0].kind, Some(JsonKind::Object));
        assert_eq!(buckets.variants[1].kind, Some(JsonKind::Array));

        // FieldValue: bool | number | string (+ null)
        let field_value = match &ir.types["_common___FieldValue"] {
            IrType::DispatchUnion(u) => u,
            other => panic!("expected dispatch union, got {:?}", other),
        };
        assert!(field_value.nullable);
        assert_eq!(field_value.variants.len(), 3);
    }

    #[test]
    fn overlapping_object_unions_stay_lazy() {
        let ir = core_ir();
        // GeoLocation: 2 object variants overlap
        assert!(matches!(
            &ir.types["_common___GeoLocation"],
            IrType::LazyUnion(_)
        ));
        // Suggest: 3 object variants
        assert!(matches!(
            &ir.types["_core.search___Suggest"],
            IrType::LazyUnion(_)
        ));
    }

    #[test]
    fn const_unions_become_open_string_enums() {
        let ir = core_ir();
        let relation = match &ir.types["_core.search___TotalHitsRelation"] {
            IrType::StrEnum(e) => e,
            other => panic!("expected string enum, got {:?}", other),
        };
        assert_eq!(relation.values, vec!["eq", "gte"]);
    }

    #[test]
    fn generic_type_parameters_are_raw() {
        let ir = core_ir();
        // the synthesized hit type merges Hit with the inline {_source: TDocument}
        let hits_metadata = match &ir.types["_core.search___HitsMetadata"] {
            IrType::Struct(s) => s,
            other => panic!("expected struct, got {:?}", other),
        };
        let hits = hits_metadata.fields.iter().find(|f| f.json_name == "hits").unwrap();
        let item = match &hits.ty {
            IrRef::List(item) => item.as_ref(),
            other => panic!("expected list, got {:?}", other),
        };
        let hit_name = match item {
            IrRef::Named(n) => n,
            other => panic!("expected named hit type, got {:?}", other),
        };
        let hit = match &ir.types[hit_name] {
            IrType::Struct(s) => s,
            other => panic!("expected struct, got {:?}", other),
        };
        let source = hit.fields.iter().find(|f| f.json_name == "_source").unwrap();
        assert_eq!(source.ty, IrRef::Raw);
        // fields from Hit itself were merged in
        assert!(hit.fields.iter().any(|f| f.json_name == "_index"));
    }

    #[test]
    fn nullable_one_of_collapses_to_option() {
        let ir = core_ir();
        let hits_metadata = match &ir.types["_core.search___HitsMetadata"] {
            IrType::Struct(s) => s,
            other => panic!("expected struct, got {:?}", other),
        };
        // max_score: oneOf [null, float]
        let max_score = hits_metadata
            .fields
            .iter()
            .find(|f| f.json_name == "max_score")
            .unwrap();
        assert!(max_score.nullable);
        assert_eq!(max_score.ty, IrRef::Scalar(Scalar::F64));
        // total: oneOf [TotalHits, i64] synthesized as a union
        let total = hits_metadata.fields.iter().find(|f| f.json_name == "total").unwrap();
        match &total.ty {
            IrRef::Named(n) => {
                assert!(
                    matches!(&ir.types[n], IrType::DispatchUnion(_)),
                    "total should dispatch on object vs number"
                );
            }
            other => panic!("expected named union, got {:?}", other),
        }
    }

    #[test]
    fn closure_is_complete() {
        let ir = core_ir();
        // every named reference resolves within the IR
        fn check(ty: &IrRef, ir: &ResponseIr) {
            match ty {
                IrRef::Named(n) => {
                    assert!(ir.types.contains_key(n), "dangling reference: {}", n);
                }
                IrRef::List(t) | IrRef::Map(t) => check(t, ir),
                _ => {}
            }
        }
        for ty in ir.types.values() {
            match ty {
                IrType::Struct(s) => {
                    for f in &s.fields {
                        check(&f.ty, &ir);
                    }
                    if let Some((_, t)) = &s.residual {
                        check(t, &ir);
                    }
                }
                IrType::DispatchUnion(u) | IrType::LazyUnion(u) => {
                    for v in &u.variants {
                        check(&v.ty, &ir);
                    }
                }
                IrType::Alias(a) => check(&a.ty, &ir),
                IrType::StrEnum(_) => {}
            }
        }
    }
}
