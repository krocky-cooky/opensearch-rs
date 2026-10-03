/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! Generates typed response models under `opensearch/src/types/` from a
//! [ResponseIr] built by the OpenAPI response reader.
//!
//! One module is generated per specification namespace (`_core.search`
//! becomes `types/search.rs`, `_common.aggregations` becomes
//! `types/common_aggregations.rs`, ...). All cross-type references use
//! fully qualified `crate::types::...` paths, so the modules need no use
//! declarations and cannot conflict.

use crate::openapi::responses::{
    namespace, type_name, IrField, IrRef, IrStrEnum, IrStruct, IrType, IrUnion, JsonKind,
    ResponseIr, Scalar,
};
use inflector::Inflector;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

/// Generates the response type modules: returns `(module name, source)`
/// pairs and the content of the `response-modules` section of
/// `types/mod.rs`
pub fn generate(ir: &ResponseIr) -> anyhow::Result<(Vec<(String, String)>, String)> {
    let boxed = boxed_edges(ir);

    // render each type into its module
    let mut modules: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    for (ir_name, ty) in &ir.types {
        let module = module_name(namespace(ir_name));
        let rust_name = rust_type_name(ir_name);
        let source = match ty {
            IrType::Struct(s) => render_struct(ir_name, &rust_name, s, &boxed),
            IrType::DispatchUnion(u) => render_dispatch_union(ir_name, &rust_name, u, &boxed),
            IrType::LazyUnion(u) => render_lazy_union(&rust_name, u),
            IrType::StrEnum(e) => render_str_enum(&rust_name, e),
            IrType::Alias(a) => {
                let mut out = String::new();
                doc(&mut out, a.description.as_deref());
                let _ = writeln!(out, "pub type {} = {};", rust_name, render_ref(&a.ty));
                out
            }
        };
        modules.entry(module).or_default().push((rust_name, source));
    }

    // per-endpoint response aliases, e.g. `IndexResponse` for the `index`
    // operation group whose root schema is the shared WriteResponseBase
    for (group, root) in &ir.endpoints {
        let (desired_module, desired_name) =
            crate::openapi::responses::endpoint_response_alias(group);
        let root_module = module_name(namespace(root));
        let root_name = rust_type_name(root);
        if root_module == desired_module && root_name == desired_name {
            continue;
        }
        let mut out = String::new();
        doc(
            &mut out,
            Some(&format!("The response of the `{}` API", group)),
        );
        let _ = writeln!(
            out,
            "pub type {} = crate::types::{}::{};",
            desired_name, root_module, root_name
        );
        modules
            .entry(desired_module)
            .or_default()
            .push((desired_name, out));
    }

    let mut files = Vec::new();
    let mut section = String::new();
    for (module, mut types) in modules {
        types.sort_by(|a, b| a.0.cmp(&b.0));
        let mut source = String::new();
        let _ = writeln!(
            source,
            "//! Response types of the `{}` specification namespace\n",
            module
        );
        source.push_str("#![allow(missing_docs)]\n\n");
        for (_, ty_source) in types {
            source.push_str(&ty_source);
            source.push('\n');
        }
        let _ = writeln!(section, "pub mod {};", module);
        files.push((module, source));
    }

    Ok((files, section))
}

/// The Rust module name of a specification namespace
pub fn module_name(ns: &str) -> String {
    let ns = ns.strip_prefix("_core.").unwrap_or(ns);
    let ns = ns.strip_prefix('_').unwrap_or(ns);
    let ns = if ns.is_empty() { "common" } else { ns };
    // `cluster._common` -> `cluster_common`, not `cluster__common`
    ns.replace("._", "_").replace('.', "_")
}

/// The Rust type name of an IR name
fn rust_type_name(ir_name: &str) -> String {
    let name = type_name(ir_name);
    if name.chars().next().is_some_and(|c| c.is_lowercase()) {
        name.to_pascal_case()
    } else {
        name.to_string()
    }
}

/// Renders an [IrRef] as a fully qualified Rust type
fn render_ref(ty: &IrRef) -> String {
    match ty {
        IrRef::Named(n) => format!(
            "crate::types::{}::{}",
            module_name(namespace(n)),
            rust_type_name(n)
        ),
        IrRef::Scalar(s) => scalar_type(*s).to_string(),
        IrRef::List(t) => format!("Vec<{}>", render_ref(t)),
        IrRef::Map(t) => format!("std::collections::BTreeMap<String, {}>", render_ref(t)),
        IrRef::Raw(_) => "Box<serde_json::value::RawValue>".to_string(),
    }
}

fn scalar_type(s: Scalar) -> &'static str {
    match s {
        Scalar::Bool => "bool",
        Scalar::I32 => "i32",
        Scalar::I64 => "i64",
        Scalar::U64 => "u64",
        Scalar::F64 => "f64",
        Scalar::String => "String",
    }
}

/// Appends a `#[doc = "..."]` attribute
fn doc(out: &mut String, description: Option<&str>) {
    if let Some(d) = description {
        let escaped = d
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', " ");
        let _ = writeln!(out, "#[doc = \"{}\"]", escaped);
    }
}

/// Rust keywords that require a raw identifier
const KEYWORDS: &[&str] = &[
    "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "do", "dyn",
    "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in", "let", "loop",
    "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref", "return", "static",
    "struct", "trait", "true", "try", "type", "typeof", "unsafe", "unsized", "use", "virtual",
    "where", "while", "yield",
];

/// Converts a JSON property name into a Rust field identifier
fn field_ident(json_name: &str, taken: &BTreeSet<String>) -> String {
    let trimmed = json_name.trim_start_matches('_');
    let mut ident = trimmed.to_snake_case();
    if ident.is_empty() || taken.contains(&ident) {
        // e.g. "_type" alongside "type": keep the underscore prefix
        ident = format!("_{}", trimmed.to_snake_case());
    }
    if KEYWORDS.contains(&ident.as_str()) {
        ident = format!("r#{}", ident);
    }
    if ident.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        ident = format!("_{}", ident);
    }
    ident
}

/// A struct field prepared for rendering
struct Field<'a> {
    ir: &'a IrField,
    ident: String,
    /// The payload type (without the `Option` of optional fields)
    payload: String,
    optional: bool,
}

/// Prepares the fields of a struct: identifiers, payload types (boxed
/// when part of a containment cycle) and optionality
fn prepare_fields<'a>(
    ir_name: &str,
    s: &'a IrStruct,
    boxed: &BTreeSet<(String, String)>,
) -> Vec<Field<'a>> {
    let mut taken: BTreeSet<String> = BTreeSet::new();
    let mut fields = Vec::new();
    for f in &s.fields {
        let ident = field_ident(&f.json_name, &taken);
        taken.insert(ident.clone());
        let mut payload = render_ref(&f.ty);
        if let IrRef::Named(target) = &f.ty {
            if boxed.contains(&(ir_name.to_string(), target.clone())) {
                payload = format!("Box<{}>", payload);
            }
        }
        fields.push(Field {
            ir: f,
            ident,
            payload,
            optional: !f.required || f.nullable,
        });
    }
    fields
}

/// Renders a struct. Structs with typed residual keys (sub-aggregations)
/// get hand-rolled `Deserialize`/`Serialize` impls that route unknown keys
/// into the residual map; plain structs derive both.
fn render_struct(
    ir_name: &str,
    rust_name: &str,
    s: &IrStruct,
    boxed: &BTreeSet<(String, String)>,
) -> String {
    // an object with no fixed fields is just a map
    if s.fields.is_empty() {
        let mut out = String::new();
        doc(&mut out, s.description.as_deref());
        let value = match &s.residual {
            Some((_, ty)) => render_ref(ty),
            None => "serde_json::Value".to_string(),
        };
        let _ = writeln!(
            out,
            "pub type {} = std::collections::BTreeMap<String, {}>;",
            rust_name, value
        );
        return out;
    }

    let fields = prepare_fields(ir_name, s, boxed);
    let mut out = String::new();

    doc(&mut out, s.description.as_deref());
    if s.residual.is_some() {
        out.push_str("#[derive(Debug, Clone)]\n");
    } else {
        out.push_str("#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]\n");
    }
    let _ = writeln!(out, "pub struct {} {{", rust_name);
    for f in &fields {
        doc(&mut out, f.ir.description.as_deref());
        if s.residual.is_none() {
            let rename = f.ident.trim_start_matches("r#");
            if rename != f.ir.json_name {
                let _ = writeln!(out, "    #[serde(rename = \"{}\")]", f.ir.json_name);
            }
            if f.optional {
                out.push_str("    #[serde(default, skip_serializing_if = \"Option::is_none\")]\n");
            }
        }
        if f.optional {
            let _ = writeln!(out, "    pub {}: Option<{}>,", f.ident, f.payload);
        } else {
            let _ = writeln!(out, "    pub {}: {},", f.ident, f.payload);
        }
    }
    if let Some((residual_name, residual_ty)) = &s.residual {
        let _ = writeln!(
            out,
            "    pub {}: std::collections::BTreeMap<String, {}>,",
            residual_name,
            render_ref(residual_ty)
        );
    }
    out.push_str("}\n");

    if let Some((residual_name, residual_ty)) = &s.residual {
        render_residual_impls(&mut out, rust_name, &fields, residual_name, residual_ty);
    }

    out
}

/// Renders `Deserialize`/`Serialize` for a struct with residual keys:
/// known keys go to their fields, all remaining keys are collected into
/// the residual map (e.g. sub-aggregations inside a bucket)
fn render_residual_impls(
    out: &mut String,
    rust_name: &str,
    fields: &[Field<'_>],
    residual_name: &str,
    residual_ty: &IrRef,
) {
    let residual_value = render_ref(residual_ty);

    let _ = writeln!(
        out,
        r#"impl<'de> serde::Deserialize<'de> for {name} {{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {{
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {{
            type Value = {name};
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                f.write_str("a {name} object")
            }}
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {{"#,
        name = rust_name
    );
    for f in fields {
        let _ = writeln!(
            out,
            "                let mut f_{}: Option<{}> = None;",
            f.ident.trim_start_matches("r#"),
            f.payload
        );
    }
    let _ = writeln!(
        out,
        "                let mut residual: std::collections::BTreeMap<String, {}> = Default::default();",
        residual_value
    );
    out.push_str(
        "                while let Some(k) = map.next_key::<String>()? {\n                    match k.as_str() {\n",
    );
    for f in fields {
        let var = f.ident.trim_start_matches("r#");
        if f.optional {
            let _ = writeln!(
                out,
                "                        {:?} => f_{} = map.next_value::<Option<{}>>()?,",
                f.ir.json_name, var, f.payload
            );
        } else {
            let _ = writeln!(
                out,
                "                        {:?} => f_{} = Some(map.next_value::<{}>()?),",
                f.ir.json_name, var, f.payload
            );
        }
    }
    out.push_str(
        "                        _ => {\n                            residual.insert(k, map.next_value()?);\n                        }\n                    }\n                }\n",
    );
    let _ = writeln!(out, "                Ok({} {{", rust_name);
    for f in fields {
        let var = f.ident.trim_start_matches("r#");
        if f.optional {
            let _ = writeln!(out, "                    {}: f_{},", f.ident, var);
        } else {
            let _ = writeln!(
                out,
                "                    {}: f_{}.ok_or_else(|| serde::de::Error::missing_field({:?}))?,",
                f.ident, var, f.ir.json_name
            );
        }
    }
    let _ = writeln!(out, "                    {}: residual,", residual_name);
    out.push_str(
        "                })\n            }\n        }\n        deserializer.deserialize_map(ResidualVisitor)\n    }\n}\n",
    );

    // Serialize: fixed fields first, then the residual entries
    let _ = writeln!(
        out,
        r#"impl serde::Serialize for {name} {{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {{
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;"#,
        name = rust_name
    );
    for f in fields {
        if f.optional {
            let _ = writeln!(
                out,
                "        if let Some(v) = &self.{} {{\n            map.serialize_entry({:?}, v)?;\n        }}",
                f.ident, f.ir.json_name
            );
        } else {
            let _ = writeln!(
                out,
                "        map.serialize_entry({:?}, &self.{})?;",
                f.ir.json_name, f.ident
            );
        }
    }
    let _ = writeln!(
        out,
        "        for (k, v) in &self.{} {{\n            map.serialize_entry(k, v)?;\n        }}\n        map.end()\n    }}\n}}",
        residual_name
    );
}

/// Renders an open string enum: unknown values deserialize into the
/// `Unknown` variant instead of failing the whole response
fn render_str_enum(rust_name: &str, e: &IrStrEnum) -> String {
    let mut out = String::new();
    doc(&mut out, e.description.as_deref());
    out.push_str(
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]\n",
    );
    let _ = writeln!(out, "pub enum {} {{", rust_name);
    // `Unknown` is reserved for the catch-all variant below
    let mut taken: BTreeSet<String> = ["Unknown".to_string()].iter().cloned().collect();
    for value in &e.values {
        let mut variant = value.to_pascal_case();
        if variant.is_empty() || !taken.insert(variant.clone()) {
            variant = format!("Value{}", taken.len());
            taken.insert(variant.clone());
        }
        let _ = writeln!(out, "    #[serde(rename = {:?})]", value);
        let _ = writeln!(out, "    {},", variant);
    }
    out.push_str("    #[doc = \"A value not defined by the specification; the original string is not preserved.\"]\n");
    out.push_str("    #[serde(other)]\n    Unknown,\n}\n");
    out
}

/// Renders a dispatch union: an enum whose `Deserialize` visitor routes
/// on the JSON kind of the incoming value, so no branch is decoded by
/// trial and error and no intermediate buffering takes place
fn render_dispatch_union(
    ir_name: &str,
    rust_name: &str,
    u: &IrUnion,
    boxed: &BTreeSet<(String, String)>,
) -> String {
    let mut out = String::new();
    doc(&mut out, u.description.as_deref());
    out.push_str("#[derive(Debug, Clone, serde::Serialize)]\n#[serde(untagged)]\n");
    let _ = writeln!(out, "pub enum {} {{", rust_name);
    let mut variant_idents = Vec::new();
    for v in &u.variants {
        let ident = v.name.to_pascal_case();
        let mut payload = render_ref(&v.ty);
        if let IrRef::Named(target) = &v.ty {
            if boxed.contains(&(ir_name.to_string(), target.clone())) {
                payload = format!("Box<{}>", payload);
            }
        }
        let _ = writeln!(out, "    {}({}),", ident, payload);
        variant_idents.push((ident, payload));
    }
    if u.nullable {
        out.push_str("    #[doc = \"An explicit null value.\"]\n    Null,\n}\n");
    } else {
        out.push_str("}\n");
    }

    let _ = writeln!(
        out,
        r#"impl<'de> serde::Deserialize<'de> for {name} {{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {{
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {{
            type Value = {name};
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                f.write_str("a {name} value")
            }}"#,
        name = rust_name
    );

    for (v, (ident, _)) in u.variants.iter().zip(&variant_idents) {
        match v.kind {
            Some(JsonKind::Bool) => {
                let _ = writeln!(
                    out,
                    r#"            fn visit_bool<E>(self, v: bool) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {{
                serde::Deserialize::deserialize(serde::de::value::BoolDeserializer::new(v))
                    .map({name}::{ident})
            }}"#,
                    name = rust_name,
                    ident = ident
                );
            }
            Some(JsonKind::Number) => {
                let _ = writeln!(
                    out,
                    r#"            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {{
                serde::Deserialize::deserialize(serde::de::value::I64Deserializer::new(v))
                    .map({name}::{ident})
            }}
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {{
                serde::Deserialize::deserialize(serde::de::value::U64Deserializer::new(v))
                    .map({name}::{ident})
            }}
            fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {{
                serde::Deserialize::deserialize(serde::de::value::F64Deserializer::new(v))
                    .map({name}::{ident})
            }}"#,
                    name = rust_name,
                    ident = ident
                );
            }
            Some(JsonKind::String) => {
                let _ = writeln!(
                    out,
                    r#"            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {{
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map({name}::{ident})
            }}"#,
                    name = rust_name,
                    ident = ident
                );
            }
            Some(JsonKind::Array) => {
                let _ = writeln!(
                    out,
                    r#"            fn visit_seq<A>(self, seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {{
                serde::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
                    .map({name}::{ident})
            }}"#,
                    name = rust_name,
                    ident = ident
                );
            }
            Some(JsonKind::Object) => {
                let _ = writeln!(
                    out,
                    r#"            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {{
                serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map({name}::{ident})
            }}"#,
                    name = rust_name,
                    ident = ident
                );
            }
            None => {}
        }
    }

    if u.nullable {
        let _ = writeln!(
            out,
            r#"            fn visit_unit<E>(self) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {{
                Ok({name}::Null)
            }}
            fn visit_none<E>(self) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {{
                Ok({name}::Null)
            }}"#,
            name = rust_name
        );
    }

    out.push_str(
        r#"        }
        deserializer.deserialize_any(KindVisitor)
    }
}
"#,
    );
    out
}

/// Renders a lazily decoded union: the raw JSON is captured unparsed and
/// decoded only when a branch accessor is called
fn render_lazy_union(rust_name: &str, u: &IrUnion) -> String {
    let mut out = String::new();
    doc(&mut out, u.description.as_deref());
    let _ = writeln!(
        out,
        r#"#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct {name} {{
    raw: Box<serde_json::value::RawValue>,
}}
impl {name} {{
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {{
        &self.raw
    }}
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {{
        self.raw.get() == "null"
    }}"#,
        name = rust_name
    );
    let mut taken = BTreeSet::new();
    for v in &u.variants {
        let mut accessor = v.name.to_snake_case();
        if accessor.is_empty() || !taken.insert(accessor.clone()) {
            continue;
        }
        if KEYWORDS.contains(&accessor.as_str()) {
            accessor = format!("{}_", accessor);
        }
        let payload = render_ref(&v.ty);
        let _ = writeln!(
            out,
            r#"    #[doc = "Parses the value as the `{branch}` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_{accessor}(&self) -> std::result::Result<{payload}, crate::types::UnionBranchError> {{
        crate::types::union::parse_branch(&self.raw, "{branch}")
    }}"#,
            branch = v.name,
            accessor = accessor,
            payload = payload
        );
    }
    out.push_str("}\n");
    out
}

/// Computes the `(container, contained)` edges that must be boxed to keep
/// recursive types finite: an edge is boxed when the contained type can
/// reach the container back through direct (non-indirected) containment
fn boxed_edges(ir: &ResponseIr) -> BTreeSet<(String, String)> {
    // direct containment edges: a contained value stored inline (fields
    // and dispatch variants); Vec / BTreeMap / lazy raw already indirect
    let mut edges: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for (name, ty) in &ir.types {
        let mut targets = BTreeSet::new();
        match ty {
            IrType::Struct(s) => {
                for f in &s.fields {
                    if let IrRef::Named(n) = &f.ty {
                        targets.insert(n.as_str());
                    }
                }
            }
            IrType::DispatchUnion(u) => {
                for v in &u.variants {
                    if let IrRef::Named(n) = &v.ty {
                        targets.insert(n.as_str());
                    }
                }
            }
            IrType::Alias(a) => {
                if let IrRef::Named(n) = &a.ty {
                    targets.insert(n.as_str());
                }
            }
            IrType::LazyUnion(_) | IrType::StrEnum(_) => {}
        }
        edges.insert(name.as_str(), targets);
    }

    fn reaches(
        from: &str,
        to: &str,
        edges: &BTreeMap<&str, BTreeSet<&str>>,
        visited: &mut BTreeSet<String>,
    ) -> bool {
        if from == to {
            return true;
        }
        if !visited.insert(from.to_string()) {
            return false;
        }
        edges
            .get(from)
            .map(|targets| targets.iter().any(|t| reaches(t, to, edges, visited)))
            .unwrap_or(false)
    }

    let mut boxed = BTreeSet::new();
    for (container, targets) in &edges {
        for target in targets {
            let mut visited = BTreeSet::new();
            if reaches(target, container, &edges, &mut visited) {
                boxed.insert((container.to_string(), target.to_string()));
            }
        }
    }
    boxed
}
