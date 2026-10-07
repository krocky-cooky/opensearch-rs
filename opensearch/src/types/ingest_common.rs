/*
 * Licensed to Elasticsearch B.V. under one or more contributor
 * license agreements. See the NOTICE file distributed with
 * this work for additional information regarding copyright
 * ownership. Elasticsearch B.V. licenses this file to you under
 * the Apache License, Version 2.0 (the "License"); you may
 * not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

// -----------------------------------------------
// This file is generated, Please do not edit it manually.
// Run the following in the root of the repo to regenerate:
//
// cargo make generate-api
// -----------------------------------------------

//! Response types of the `ingest_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct AppendProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    #[doc = "If `false`, the processor does not append values already present in the field."]
    pub allow_duplicates: Option<bool>,
    pub field: crate::types::common::Field,
    #[doc = "The value to be appended. Supports template snippets."]
    pub value: Vec<Box<serde_json::value::RawValue>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AppendProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AppendProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AppendProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_allow_duplicates: Option<bool> = None;
                let mut seen_allow_duplicates = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_value: Option<Vec<Box<serde_json::value::RawValue>>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "allow_duplicates" => {
                            if seen_allow_duplicates {
                                return Err(serde::de::Error::duplicate_field("allow_duplicates"));
                            }
                            seen_allow_duplicates = true;
                            f_allow_duplicates = map.next_value::<Option<bool>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "value" => {
                            if f_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            f_value =
                                Some(map.next_value::<Vec<Box<serde_json::value::RawValue>>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AppendProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    allow_duplicates: f_allow_duplicates,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    value: f_value.ok_or_else(|| serde::de::Error::missing_field("value"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AppendProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        if let Some(v) = &self.allow_duplicates {
            map.serialize_entry("allow_duplicates", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        map.serialize_entry("value", &self.value)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct AttachmentProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and field does not exist, the processor quietly exits without modifying the document."]
    pub ignore_missing: Option<bool>,
    #[doc = "The number of chars being used for extraction to prevent huge fields. Use `-1` for no limit."]
    pub indexed_chars: Option<i64>,
    pub indexed_chars_field: Option<crate::types::common::Field>,
    #[doc = "Array of properties to select to be stored. Can be `content`, `title`, `name`, `author`, `keywords`, `date`, `content_type`, `content_length`, `language`."]
    pub properties: Option<Vec<String>>,
    #[doc = "Field containing the name of the resource to decode. If specified, the processor passes this resource name to the underlying Tika library to enable Resource Name Based Detection."]
    pub resource_name: Option<String>,
    pub target_field: Option<crate::types::common::Field>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AttachmentProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AttachmentProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AttachmentProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_ignore_missing: Option<bool> = None;
                let mut seen_ignore_missing = false;
                let mut f_indexed_chars: Option<i64> = None;
                let mut seen_indexed_chars = false;
                let mut f_indexed_chars_field: Option<crate::types::common::Field> = None;
                let mut seen_indexed_chars_field = false;
                let mut f_properties: Option<Vec<String>> = None;
                let mut seen_properties = false;
                let mut f_resource_name: Option<String> = None;
                let mut seen_resource_name = false;
                let mut f_target_field: Option<crate::types::common::Field> = None;
                let mut seen_target_field = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "ignore_missing" => {
                            if seen_ignore_missing {
                                return Err(serde::de::Error::duplicate_field("ignore_missing"));
                            }
                            seen_ignore_missing = true;
                            f_ignore_missing = map.next_value::<Option<bool>>()?;
                        }
                        "indexed_chars" => {
                            if seen_indexed_chars {
                                return Err(serde::de::Error::duplicate_field("indexed_chars"));
                            }
                            seen_indexed_chars = true;
                            f_indexed_chars = map.next_value::<Option<i64>>()?;
                        }
                        "indexed_chars_field" => {
                            if seen_indexed_chars_field {
                                return Err(serde::de::Error::duplicate_field(
                                    "indexed_chars_field",
                                ));
                            }
                            seen_indexed_chars_field = true;
                            f_indexed_chars_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "resource_name" => {
                            if seen_resource_name {
                                return Err(serde::de::Error::duplicate_field("resource_name"));
                            }
                            seen_resource_name = true;
                            f_resource_name = map.next_value::<Option<String>>()?;
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AttachmentProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    ignore_missing: f_ignore_missing,
                    indexed_chars: f_indexed_chars,
                    indexed_chars_field: f_indexed_chars_field,
                    properties: f_properties,
                    resource_name: f_resource_name,
                    target_field: f_target_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AttachmentProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.ignore_missing {
            map.serialize_entry("ignore_missing", v)?;
        }
        if let Some(v) = &self.indexed_chars {
            map.serialize_entry("indexed_chars", v)?;
        }
        if let Some(v) = &self.indexed_chars_field {
            map.serialize_entry("indexed_chars_field", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.resource_name {
            map.serialize_entry("resource_name", v)?;
        }
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct BytesProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist or is `null`, the processor quietly exits without modifying the document."]
    pub ignore_missing: Option<bool>,
    pub target_field: Option<crate::types::common::Field>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for BytesProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = BytesProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a BytesProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_ignore_missing: Option<bool> = None;
                let mut seen_ignore_missing = false;
                let mut f_target_field: Option<crate::types::common::Field> = None;
                let mut seen_target_field = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "ignore_missing" => {
                            if seen_ignore_missing {
                                return Err(serde::de::Error::duplicate_field("ignore_missing"));
                            }
                            seen_ignore_missing = true;
                            f_ignore_missing = map.next_value::<Option<bool>>()?;
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(BytesProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    ignore_missing: f_ignore_missing,
                    target_field: f_target_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for BytesProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.ignore_missing {
            map.serialize_entry("ignore_missing", v)?;
        }
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct CircleProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    #[doc = "The difference between the resulting inscribed distance from center to side and the circle's radius (measured in meters for `geo_shape`, unit-less for `shape`)."]
    pub error_distance: f64,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist, the processor quietly exits without modifying the document."]
    pub ignore_missing: Option<bool>,
    pub shape_type: crate::types::ingest_common::ShapeType,
    pub target_field: Option<crate::types::common::Field>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CircleProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CircleProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CircleProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_error_distance: Option<f64> = None;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_ignore_missing: Option<bool> = None;
                let mut seen_ignore_missing = false;
                let mut f_shape_type: Option<crate::types::ingest_common::ShapeType> = None;
                let mut f_target_field: Option<crate::types::common::Field> = None;
                let mut seen_target_field = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "error_distance" => {
                            if f_error_distance.is_some() {
                                return Err(serde::de::Error::duplicate_field("error_distance"));
                            }
                            f_error_distance = Some(map.next_value::<f64>()?);
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "ignore_missing" => {
                            if seen_ignore_missing {
                                return Err(serde::de::Error::duplicate_field("ignore_missing"));
                            }
                            seen_ignore_missing = true;
                            f_ignore_missing = map.next_value::<Option<bool>>()?;
                        }
                        "shape_type" => {
                            if f_shape_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("shape_type"));
                            }
                            f_shape_type =
                                Some(map.next_value::<crate::types::ingest_common::ShapeType>()?);
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CircleProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    error_distance: f_error_distance
                        .ok_or_else(|| serde::de::Error::missing_field("error_distance"))?,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    ignore_missing: f_ignore_missing,
                    shape_type: f_shape_type
                        .ok_or_else(|| serde::de::Error::missing_field("shape_type"))?,
                    target_field: f_target_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CircleProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("error_distance", &self.error_distance)?;
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.ignore_missing {
            map.serialize_entry("ignore_missing", v)?;
        }
        map.serialize_entry("shape_type", &self.shape_type)?;
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ConvertProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist or is `null`, the processor quietly exits without modifying the document."]
    pub ignore_missing: Option<bool>,
    pub target_field: Option<crate::types::common::Field>,
    pub r#type: crate::types::ingest_common::ConvertType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ConvertProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ConvertProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ConvertProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_ignore_missing: Option<bool> = None;
                let mut seen_ignore_missing = false;
                let mut f_target_field: Option<crate::types::common::Field> = None;
                let mut seen_target_field = false;
                let mut f_type: Option<crate::types::ingest_common::ConvertType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "ignore_missing" => {
                            if seen_ignore_missing {
                                return Err(serde::de::Error::duplicate_field("ignore_missing"));
                            }
                            seen_ignore_missing = true;
                            f_ignore_missing = map.next_value::<Option<bool>>()?;
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type =
                                Some(map.next_value::<crate::types::ingest_common::ConvertType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ConvertProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    ignore_missing: f_ignore_missing,
                    target_field: f_target_field,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ConvertProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.ignore_missing {
            map.serialize_entry("ignore_missing", v)?;
        }
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConvertType {
    #[doc = "`auto`"]
    Auto,
    #[doc = "`boolean`"]
    Boolean,
    #[doc = "`double`"]
    Double,
    #[doc = "`float`"]
    Float,
    #[doc = "`integer`"]
    Integer,
    #[doc = "`long`"]
    Long,
    #[doc = "`string`"]
    String,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ConvertType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ConvertType::Auto => "auto",
            ConvertType::Boolean => "boolean",
            ConvertType::Double => "double",
            ConvertType::Float => "float",
            ConvertType::Integer => "integer",
            ConvertType::Long => "long",
            ConvertType::String => "string",
            ConvertType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ConvertType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ConvertType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "auto" => ConvertType::Auto,
            "boolean" => ConvertType::Boolean,
            "double" => ConvertType::Double,
            "float" => ConvertType::Float,
            "integer" => ConvertType::Integer,
            "long" => ConvertType::Long,
            "string" => ConvertType::String,
            _ => ConvertType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct CsvProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    #[doc = "Value used to fill empty fields. Empty fields are skipped if this is not provided. An empty field is one with no value (2 consecutive separators) or empty quotes (`\"\"`)."]
    pub empty_value: Option<Box<serde_json::value::RawValue>>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist, the processor quietly exits without modifying the document."]
    pub ignore_missing: Option<bool>,
    #[doc = "Quote used in CSV, has to be single character string."]
    pub quote: Option<String>,
    #[doc = "Separator used in CSV, has to be single character string."]
    pub separator: Option<String>,
    pub target_fields: crate::types::common::Fields,
    #[doc = "Trim white spaces in unquoted fields."]
    pub trim: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CsvProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CsvProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CsvProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_empty_value: Option<Box<serde_json::value::RawValue>> = None;
                let mut seen_empty_value = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_ignore_missing: Option<bool> = None;
                let mut seen_ignore_missing = false;
                let mut f_quote: Option<String> = None;
                let mut seen_quote = false;
                let mut f_separator: Option<String> = None;
                let mut seen_separator = false;
                let mut f_target_fields: Option<crate::types::common::Fields> = None;
                let mut f_trim: Option<bool> = None;
                let mut seen_trim = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "empty_value" => {
                            if seen_empty_value {
                                return Err(serde::de::Error::duplicate_field("empty_value"));
                            }
                            seen_empty_value = true;
                            f_empty_value =
                                map.next_value::<Option<Box<serde_json::value::RawValue>>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "ignore_missing" => {
                            if seen_ignore_missing {
                                return Err(serde::de::Error::duplicate_field("ignore_missing"));
                            }
                            seen_ignore_missing = true;
                            f_ignore_missing = map.next_value::<Option<bool>>()?;
                        }
                        "quote" => {
                            if seen_quote {
                                return Err(serde::de::Error::duplicate_field("quote"));
                            }
                            seen_quote = true;
                            f_quote = map.next_value::<Option<String>>()?;
                        }
                        "separator" => {
                            if seen_separator {
                                return Err(serde::de::Error::duplicate_field("separator"));
                            }
                            seen_separator = true;
                            f_separator = map.next_value::<Option<String>>()?;
                        }
                        "target_fields" => {
                            if f_target_fields.is_some() {
                                return Err(serde::de::Error::duplicate_field("target_fields"));
                            }
                            f_target_fields =
                                Some(map.next_value::<crate::types::common::Fields>()?);
                        }
                        "trim" => {
                            if seen_trim {
                                return Err(serde::de::Error::duplicate_field("trim"));
                            }
                            seen_trim = true;
                            f_trim = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CsvProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    empty_value: f_empty_value,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    ignore_missing: f_ignore_missing,
                    quote: f_quote,
                    separator: f_separator,
                    target_fields: f_target_fields
                        .ok_or_else(|| serde::de::Error::missing_field("target_fields"))?,
                    trim: f_trim,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CsvProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        if let Some(v) = &self.empty_value {
            map.serialize_entry("empty_value", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.ignore_missing {
            map.serialize_entry("ignore_missing", v)?;
        }
        if let Some(v) = &self.quote {
            map.serialize_entry("quote", v)?;
        }
        if let Some(v) = &self.separator {
            map.serialize_entry("separator", v)?;
        }
        map.serialize_entry("target_fields", &self.target_fields)?;
        if let Some(v) = &self.trim {
            map.serialize_entry("trim", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct DateIndexNameProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    #[doc = "An array of the expected date formats for parsing dates/timestamps in the document being preprocessed. Can be a java time pattern or one of the following formats: `ISO8601`, `UNIX`, `UNIX_MS`, or `TAI64N`."]
    pub date_formats: Vec<String>,
    #[doc = "How to round the date when formatting the date into the index name. Valid values are: `y` (year), `M` (month), `w` (week), `d` (day), `h` (hour), `m` (minute) and `s` (second). Supports template snippets."]
    pub date_rounding: String,
    pub field: crate::types::common::Field,
    #[doc = "The format to be used when printing the parsed date into the index name. A valid java time pattern is expected here. Supports template snippets."]
    pub index_name_format: Option<String>,
    #[doc = "A prefix of the index name to be prepended before the printed date. Supports template snippets."]
    pub index_name_prefix: Option<String>,
    #[doc = "The locale to use when parsing the date from the document being preprocessed, relevant when parsing month names or week days."]
    pub locale: Option<String>,
    #[doc = "The time zone to use when parsing the date and when date math index supports resolves expressions into concrete index names."]
    pub timezone: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DateIndexNameProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DateIndexNameProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DateIndexNameProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_date_formats: Option<Vec<String>> = None;
                let mut f_date_rounding: Option<String> = None;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_index_name_format: Option<String> = None;
                let mut seen_index_name_format = false;
                let mut f_index_name_prefix: Option<String> = None;
                let mut seen_index_name_prefix = false;
                let mut f_locale: Option<String> = None;
                let mut seen_locale = false;
                let mut f_timezone: Option<String> = None;
                let mut seen_timezone = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "date_formats" => {
                            if f_date_formats.is_some() {
                                return Err(serde::de::Error::duplicate_field("date_formats"));
                            }
                            f_date_formats = Some(map.next_value::<Vec<String>>()?);
                        }
                        "date_rounding" => {
                            if f_date_rounding.is_some() {
                                return Err(serde::de::Error::duplicate_field("date_rounding"));
                            }
                            f_date_rounding = Some(map.next_value::<String>()?);
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "index_name_format" => {
                            if seen_index_name_format {
                                return Err(serde::de::Error::duplicate_field("index_name_format"));
                            }
                            seen_index_name_format = true;
                            f_index_name_format = map.next_value::<Option<String>>()?;
                        }
                        "index_name_prefix" => {
                            if seen_index_name_prefix {
                                return Err(serde::de::Error::duplicate_field("index_name_prefix"));
                            }
                            seen_index_name_prefix = true;
                            f_index_name_prefix = map.next_value::<Option<String>>()?;
                        }
                        "locale" => {
                            if seen_locale {
                                return Err(serde::de::Error::duplicate_field("locale"));
                            }
                            seen_locale = true;
                            f_locale = map.next_value::<Option<String>>()?;
                        }
                        "timezone" => {
                            if seen_timezone {
                                return Err(serde::de::Error::duplicate_field("timezone"));
                            }
                            seen_timezone = true;
                            f_timezone = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DateIndexNameProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    date_formats: f_date_formats
                        .ok_or_else(|| serde::de::Error::missing_field("date_formats"))?,
                    date_rounding: f_date_rounding
                        .ok_or_else(|| serde::de::Error::missing_field("date_rounding"))?,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    index_name_format: f_index_name_format,
                    index_name_prefix: f_index_name_prefix,
                    locale: f_locale,
                    timezone: f_timezone,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DateIndexNameProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("date_formats", &self.date_formats)?;
        map.serialize_entry("date_rounding", &self.date_rounding)?;
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.index_name_format {
            map.serialize_entry("index_name_format", v)?;
        }
        if let Some(v) = &self.index_name_prefix {
            map.serialize_entry("index_name_prefix", v)?;
        }
        if let Some(v) = &self.locale {
            map.serialize_entry("locale", v)?;
        }
        if let Some(v) = &self.timezone {
            map.serialize_entry("timezone", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct DateProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "An array of the expected date formats. Can be a java time pattern or one of the following formats: ISO8601, UNIX, UNIX_MS, or TAI64N."]
    pub formats: Vec<String>,
    #[doc = "The locale to use when parsing the date, relevant when parsing month names or week days. Supports template snippets."]
    pub locale: Option<String>,
    pub target_field: Option<crate::types::common::Field>,
    #[doc = "The time zone to use when parsing the date. Supports template snippets."]
    pub timezone: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DateProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DateProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DateProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_formats: Option<Vec<String>> = None;
                let mut f_locale: Option<String> = None;
                let mut seen_locale = false;
                let mut f_target_field: Option<crate::types::common::Field> = None;
                let mut seen_target_field = false;
                let mut f_timezone: Option<String> = None;
                let mut seen_timezone = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "formats" => {
                            if f_formats.is_some() {
                                return Err(serde::de::Error::duplicate_field("formats"));
                            }
                            f_formats = Some(map.next_value::<Vec<String>>()?);
                        }
                        "locale" => {
                            if seen_locale {
                                return Err(serde::de::Error::duplicate_field("locale"));
                            }
                            seen_locale = true;
                            f_locale = map.next_value::<Option<String>>()?;
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        "timezone" => {
                            if seen_timezone {
                                return Err(serde::de::Error::duplicate_field("timezone"));
                            }
                            seen_timezone = true;
                            f_timezone = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DateProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    formats: f_formats.ok_or_else(|| serde::de::Error::missing_field("formats"))?,
                    locale: f_locale,
                    target_field: f_target_field,
                    timezone: f_timezone,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DateProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        map.serialize_entry("formats", &self.formats)?;
        if let Some(v) = &self.locale {
            map.serialize_entry("locale", v)?;
        }
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        if let Some(v) = &self.timezone {
            map.serialize_entry("timezone", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct DissectProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    #[doc = "The character(s) that separate the appended fields."]
    pub append_separator: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist or is `null`, the processor quietly exits without modifying the document."]
    pub ignore_missing: Option<bool>,
    #[doc = "The pattern to apply to the field."]
    pub pattern: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DissectProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DissectProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DissectProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_append_separator: Option<String> = None;
                let mut seen_append_separator = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_ignore_missing: Option<bool> = None;
                let mut seen_ignore_missing = false;
                let mut f_pattern: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "append_separator" => {
                            if seen_append_separator {
                                return Err(serde::de::Error::duplicate_field("append_separator"));
                            }
                            seen_append_separator = true;
                            f_append_separator = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "ignore_missing" => {
                            if seen_ignore_missing {
                                return Err(serde::de::Error::duplicate_field("ignore_missing"));
                            }
                            seen_ignore_missing = true;
                            f_ignore_missing = map.next_value::<Option<bool>>()?;
                        }
                        "pattern" => {
                            if f_pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("pattern"));
                            }
                            f_pattern = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DissectProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    append_separator: f_append_separator,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    ignore_missing: f_ignore_missing,
                    pattern: f_pattern.ok_or_else(|| serde::de::Error::missing_field("pattern"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DissectProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        if let Some(v) = &self.append_separator {
            map.serialize_entry("append_separator", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.ignore_missing {
            map.serialize_entry("ignore_missing", v)?;
        }
        map.serialize_entry("pattern", &self.pattern)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct DotExpanderProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "The field that contains the field to expand. Only required if the field to expand is part another object field, because the `field` option can only understand leaf fields."]
    pub path: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DotExpanderProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DotExpanderProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DotExpanderProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_path: Option<String> = None;
                let mut seen_path = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "path" => {
                            if seen_path {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            seen_path = true;
                            f_path = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DotExpanderProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    path: f_path,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DotExpanderProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.path {
            map.serialize_entry("path", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct DropProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DropProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DropProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DropProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DropProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DropProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct FailProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    #[doc = "The error message thrown by the processor. Supports template snippets."]
    pub message: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FailProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FailProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FailProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_message: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "message" => {
                            if f_message.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            f_message = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FailProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    message: f_message.ok_or_else(|| serde::de::Error::missing_field("message"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FailProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("message", &self.message)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ForeachProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true`, the processor silently exits without changing the document if the `field` is `null` or missing."]
    pub ignore_missing: Option<bool>,
    pub processor: Box<crate::types::ingest_common::ProcessorContainer>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ForeachProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ForeachProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ForeachProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_ignore_missing: Option<bool> = None;
                let mut seen_ignore_missing = false;
                let mut f_processor: Option<Box<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "ignore_missing" => {
                            if seen_ignore_missing {
                                return Err(serde::de::Error::duplicate_field("ignore_missing"));
                            }
                            seen_ignore_missing = true;
                            f_ignore_missing = map.next_value::<Option<bool>>()?;
                        }
                        "processor" => {
                            if f_processor.is_some() {
                                return Err(serde::de::Error::duplicate_field("processor"));
                            }
                            f_processor = Some(map.next_value::<Box<crate::types::ingest_common::ProcessorContainer>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ForeachProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    ignore_missing: f_ignore_missing,
                    processor: f_processor
                        .ok_or_else(|| serde::de::Error::missing_field("processor"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ForeachProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.ignore_missing {
            map.serialize_entry("ignore_missing", v)?;
        }
        map.serialize_entry("processor", &self.processor)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct GeoIpProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    #[doc = "The database filename referring to a database the module ships with (`GeoLite2-City.mmdb`, `GeoLite2-Country.mmdb`, or `GeoLite2-ASN.mmdb`) or a custom database in the `ingest-geoip` config directory."]
    pub database_file: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true`, only the first found geoip data will be returned, even if the field contains an array."]
    pub first_only: Option<bool>,
    #[doc = "If `true` and `field` does not exist, the processor quietly exits without modifying the document."]
    pub ignore_missing: Option<bool>,
    #[doc = "Controls what properties are added to the `target_field` based on the geoip lookup."]
    pub properties: Option<Vec<String>>,
    pub target_field: Option<crate::types::common::Field>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GeoIpProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GeoIpProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoIpProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_database_file: Option<String> = None;
                let mut seen_database_file = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_first_only: Option<bool> = None;
                let mut seen_first_only = false;
                let mut f_ignore_missing: Option<bool> = None;
                let mut seen_ignore_missing = false;
                let mut f_properties: Option<Vec<String>> = None;
                let mut seen_properties = false;
                let mut f_target_field: Option<crate::types::common::Field> = None;
                let mut seen_target_field = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "database_file" => {
                            if seen_database_file {
                                return Err(serde::de::Error::duplicate_field("database_file"));
                            }
                            seen_database_file = true;
                            f_database_file = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "first_only" => {
                            if seen_first_only {
                                return Err(serde::de::Error::duplicate_field("first_only"));
                            }
                            seen_first_only = true;
                            f_first_only = map.next_value::<Option<bool>>()?;
                        }
                        "ignore_missing" => {
                            if seen_ignore_missing {
                                return Err(serde::de::Error::duplicate_field("ignore_missing"));
                            }
                            seen_ignore_missing = true;
                            f_ignore_missing = map.next_value::<Option<bool>>()?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GeoIpProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    database_file: f_database_file,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    first_only: f_first_only,
                    ignore_missing: f_ignore_missing,
                    properties: f_properties,
                    target_field: f_target_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GeoIpProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        if let Some(v) = &self.database_file {
            map.serialize_entry("database_file", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.first_only {
            map.serialize_entry("first_only", v)?;
        }
        if let Some(v) = &self.ignore_missing {
            map.serialize_entry("ignore_missing", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct GrokProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist or is `null`, the processor quietly exits without modifying the document."]
    pub ignore_missing: Option<bool>,
    #[doc = "A map of pattern-name and pattern tuples defining custom patterns to be used by the current processor. Patterns matching existing names will override the pre-existing definition."]
    pub pattern_definitions: Option<std::collections::BTreeMap<String, String>>,
    #[doc = "An ordered list of grok expression to match and extract named captures with. Returns on the first expression in the list that matches."]
    pub patterns: Vec<String>,
    #[doc = "When `true`, `_ingest._grok_match_index` will be inserted into your matched document's metadata with the index into the pattern found in `patterns` that matched."]
    pub trace_match: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GrokProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GrokProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GrokProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_ignore_missing: Option<bool> = None;
                let mut seen_ignore_missing = false;
                let mut f_pattern_definitions: Option<std::collections::BTreeMap<String, String>> =
                    None;
                let mut seen_pattern_definitions = false;
                let mut f_patterns: Option<Vec<String>> = None;
                let mut f_trace_match: Option<bool> = None;
                let mut seen_trace_match = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "ignore_missing" => {
                            if seen_ignore_missing {
                                return Err(serde::de::Error::duplicate_field("ignore_missing"));
                            }
                            seen_ignore_missing = true;
                            f_ignore_missing = map.next_value::<Option<bool>>()?;
                        }
                        "pattern_definitions" => {
                            if seen_pattern_definitions {
                                return Err(serde::de::Error::duplicate_field(
                                    "pattern_definitions",
                                ));
                            }
                            seen_pattern_definitions = true;
                            f_pattern_definitions = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "patterns" => {
                            if f_patterns.is_some() {
                                return Err(serde::de::Error::duplicate_field("patterns"));
                            }
                            f_patterns = Some(map.next_value::<Vec<String>>()?);
                        }
                        "trace_match" => {
                            if seen_trace_match {
                                return Err(serde::de::Error::duplicate_field("trace_match"));
                            }
                            seen_trace_match = true;
                            f_trace_match = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GrokProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    ignore_missing: f_ignore_missing,
                    pattern_definitions: f_pattern_definitions,
                    patterns: f_patterns
                        .ok_or_else(|| serde::de::Error::missing_field("patterns"))?,
                    trace_match: f_trace_match,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GrokProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.ignore_missing {
            map.serialize_entry("ignore_missing", v)?;
        }
        if let Some(v) = &self.pattern_definitions {
            map.serialize_entry("pattern_definitions", v)?;
        }
        map.serialize_entry("patterns", &self.patterns)?;
        if let Some(v) = &self.trace_match {
            map.serialize_entry("trace_match", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct GsubProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist or is `null`, the processor quietly exits without modifying the document."]
    pub ignore_missing: Option<bool>,
    #[doc = "The pattern to be replaced."]
    pub pattern: String,
    #[doc = "The string to replace the matching patterns with."]
    pub replacement: String,
    pub target_field: Option<crate::types::common::Field>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GsubProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GsubProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GsubProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_ignore_missing: Option<bool> = None;
                let mut seen_ignore_missing = false;
                let mut f_pattern: Option<String> = None;
                let mut f_replacement: Option<String> = None;
                let mut f_target_field: Option<crate::types::common::Field> = None;
                let mut seen_target_field = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "ignore_missing" => {
                            if seen_ignore_missing {
                                return Err(serde::de::Error::duplicate_field("ignore_missing"));
                            }
                            seen_ignore_missing = true;
                            f_ignore_missing = map.next_value::<Option<bool>>()?;
                        }
                        "pattern" => {
                            if f_pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("pattern"));
                            }
                            f_pattern = Some(map.next_value::<String>()?);
                        }
                        "replacement" => {
                            if f_replacement.is_some() {
                                return Err(serde::de::Error::duplicate_field("replacement"));
                            }
                            f_replacement = Some(map.next_value::<String>()?);
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GsubProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    ignore_missing: f_ignore_missing,
                    pattern: f_pattern.ok_or_else(|| serde::de::Error::missing_field("pattern"))?,
                    replacement: f_replacement
                        .ok_or_else(|| serde::de::Error::missing_field("replacement"))?,
                    target_field: f_target_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GsubProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.ignore_missing {
            map.serialize_entry("ignore_missing", v)?;
        }
        map.serialize_entry("pattern", &self.pattern)?;
        map.serialize_entry("replacement", &self.replacement)?;
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct JoinProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "The separator character."]
    pub separator: String,
    pub target_field: Option<crate::types::common::Field>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for JoinProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = JoinProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a JoinProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_separator: Option<String> = None;
                let mut f_target_field: Option<crate::types::common::Field> = None;
                let mut seen_target_field = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "separator" => {
                            if f_separator.is_some() {
                                return Err(serde::de::Error::duplicate_field("separator"));
                            }
                            f_separator = Some(map.next_value::<String>()?);
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(JoinProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    separator: f_separator
                        .ok_or_else(|| serde::de::Error::missing_field("separator"))?,
                    target_field: f_target_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for JoinProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        map.serialize_entry("separator", &self.separator)?;
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct JsonProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    #[doc = "Flag that forces the parsed JSON to be added at the top level of the document. `target_field` must not be set when this option is chosen."]
    pub add_to_root: Option<bool>,
    pub add_to_root_conflict_strategy:
        Option<crate::types::ingest_common::JsonProcessorConflictStrategy>,
    #[doc = "When set to `true`, the JSON parser will not fail if the JSON contains duplicate keys. Instead, the last encountered value for any duplicate key wins."]
    pub allow_duplicate_keys: Option<bool>,
    pub field: crate::types::common::Field,
    pub target_field: Option<crate::types::common::Field>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for JsonProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = JsonProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a JsonProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_add_to_root: Option<bool> = None;
                let mut seen_add_to_root = false;
                let mut f_add_to_root_conflict_strategy: Option<
                    crate::types::ingest_common::JsonProcessorConflictStrategy,
                > = None;
                let mut seen_add_to_root_conflict_strategy = false;
                let mut f_allow_duplicate_keys: Option<bool> = None;
                let mut seen_allow_duplicate_keys = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_target_field: Option<crate::types::common::Field> = None;
                let mut seen_target_field = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "add_to_root" => {
                            if seen_add_to_root {
                                return Err(serde::de::Error::duplicate_field("add_to_root"));
                            }
                            seen_add_to_root = true;
                            f_add_to_root = map.next_value::<Option<bool>>()?;
                        }
                        "add_to_root_conflict_strategy" => {
                            if seen_add_to_root_conflict_strategy {
                                return Err(serde::de::Error::duplicate_field(
                                    "add_to_root_conflict_strategy",
                                ));
                            }
                            seen_add_to_root_conflict_strategy = true;
                            f_add_to_root_conflict_strategy = map
                                .next_value::<Option<
                                    crate::types::ingest_common::JsonProcessorConflictStrategy,
                                >>()?;
                        }
                        "allow_duplicate_keys" => {
                            if seen_allow_duplicate_keys {
                                return Err(serde::de::Error::duplicate_field(
                                    "allow_duplicate_keys",
                                ));
                            }
                            seen_allow_duplicate_keys = true;
                            f_allow_duplicate_keys = map.next_value::<Option<bool>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(JsonProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    add_to_root: f_add_to_root,
                    add_to_root_conflict_strategy: f_add_to_root_conflict_strategy,
                    allow_duplicate_keys: f_allow_duplicate_keys,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    target_field: f_target_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for JsonProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        if let Some(v) = &self.add_to_root {
            map.serialize_entry("add_to_root", v)?;
        }
        if let Some(v) = &self.add_to_root_conflict_strategy {
            map.serialize_entry("add_to_root_conflict_strategy", v)?;
        }
        if let Some(v) = &self.allow_duplicate_keys {
            map.serialize_entry("allow_duplicate_keys", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsonProcessorConflictStrategy {
    #[doc = "`merge`"]
    Merge,
    #[doc = "`replace`"]
    Replace,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl JsonProcessorConflictStrategy {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            JsonProcessorConflictStrategy::Merge => "merge",
            JsonProcessorConflictStrategy::Replace => "replace",
            JsonProcessorConflictStrategy::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for JsonProcessorConflictStrategy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for JsonProcessorConflictStrategy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "merge" => JsonProcessorConflictStrategy::Merge,
            "replace" => JsonProcessorConflictStrategy::Replace,
            _ => JsonProcessorConflictStrategy::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct KeyValueProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    #[doc = "List of keys to exclude from document."]
    pub exclude_keys: Option<Vec<String>>,
    pub field: crate::types::common::Field,
    #[doc = "Regex pattern to use for splitting key-value pairs."]
    pub field_split: String,
    #[doc = "If `true` and `field` does not exist or is `null`, the processor quietly exits without modifying the document."]
    pub ignore_missing: Option<bool>,
    #[doc = "List of keys to filter and insert into document. Defaults to including all keys."]
    pub include_keys: Option<Vec<String>>,
    #[doc = "Prefix to be added to extracted keys."]
    pub prefix: Option<String>,
    #[doc = "If `true`. strip brackets `()`, `<>`, `[]` as well as quotes `'` and `\"` from extracted values."]
    pub strip_brackets: Option<bool>,
    pub target_field: Option<crate::types::common::Field>,
    #[doc = "String of characters to trim from extracted keys."]
    pub trim_key: Option<String>,
    #[doc = "String of characters to trim from extracted values."]
    pub trim_value: Option<String>,
    #[doc = "Regex pattern to use for splitting the key from the value within a key-value pair."]
    pub value_split: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for KeyValueProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = KeyValueProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a KeyValueProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_exclude_keys: Option<Vec<String>> = None;
                let mut seen_exclude_keys = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_field_split: Option<String> = None;
                let mut f_ignore_missing: Option<bool> = None;
                let mut seen_ignore_missing = false;
                let mut f_include_keys: Option<Vec<String>> = None;
                let mut seen_include_keys = false;
                let mut f_prefix: Option<String> = None;
                let mut seen_prefix = false;
                let mut f_strip_brackets: Option<bool> = None;
                let mut seen_strip_brackets = false;
                let mut f_target_field: Option<crate::types::common::Field> = None;
                let mut seen_target_field = false;
                let mut f_trim_key: Option<String> = None;
                let mut seen_trim_key = false;
                let mut f_trim_value: Option<String> = None;
                let mut seen_trim_value = false;
                let mut f_value_split: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "exclude_keys" => {
                            if seen_exclude_keys {
                                return Err(serde::de::Error::duplicate_field("exclude_keys"));
                            }
                            seen_exclude_keys = true;
                            f_exclude_keys = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "field_split" => {
                            if f_field_split.is_some() {
                                return Err(serde::de::Error::duplicate_field("field_split"));
                            }
                            f_field_split = Some(map.next_value::<String>()?);
                        }
                        "ignore_missing" => {
                            if seen_ignore_missing {
                                return Err(serde::de::Error::duplicate_field("ignore_missing"));
                            }
                            seen_ignore_missing = true;
                            f_ignore_missing = map.next_value::<Option<bool>>()?;
                        }
                        "include_keys" => {
                            if seen_include_keys {
                                return Err(serde::de::Error::duplicate_field("include_keys"));
                            }
                            seen_include_keys = true;
                            f_include_keys = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "prefix" => {
                            if seen_prefix {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            seen_prefix = true;
                            f_prefix = map.next_value::<Option<String>>()?;
                        }
                        "strip_brackets" => {
                            if seen_strip_brackets {
                                return Err(serde::de::Error::duplicate_field("strip_brackets"));
                            }
                            seen_strip_brackets = true;
                            f_strip_brackets = map.next_value::<Option<bool>>()?;
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        "trim_key" => {
                            if seen_trim_key {
                                return Err(serde::de::Error::duplicate_field("trim_key"));
                            }
                            seen_trim_key = true;
                            f_trim_key = map.next_value::<Option<String>>()?;
                        }
                        "trim_value" => {
                            if seen_trim_value {
                                return Err(serde::de::Error::duplicate_field("trim_value"));
                            }
                            seen_trim_value = true;
                            f_trim_value = map.next_value::<Option<String>>()?;
                        }
                        "value_split" => {
                            if f_value_split.is_some() {
                                return Err(serde::de::Error::duplicate_field("value_split"));
                            }
                            f_value_split = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(KeyValueProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    exclude_keys: f_exclude_keys,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    field_split: f_field_split
                        .ok_or_else(|| serde::de::Error::missing_field("field_split"))?,
                    ignore_missing: f_ignore_missing,
                    include_keys: f_include_keys,
                    prefix: f_prefix,
                    strip_brackets: f_strip_brackets,
                    target_field: f_target_field,
                    trim_key: f_trim_key,
                    trim_value: f_trim_value,
                    value_split: f_value_split
                        .ok_or_else(|| serde::de::Error::missing_field("value_split"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for KeyValueProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        if let Some(v) = &self.exclude_keys {
            map.serialize_entry("exclude_keys", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        map.serialize_entry("field_split", &self.field_split)?;
        if let Some(v) = &self.ignore_missing {
            map.serialize_entry("ignore_missing", v)?;
        }
        if let Some(v) = &self.include_keys {
            map.serialize_entry("include_keys", v)?;
        }
        if let Some(v) = &self.prefix {
            map.serialize_entry("prefix", v)?;
        }
        if let Some(v) = &self.strip_brackets {
            map.serialize_entry("strip_brackets", v)?;
        }
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        if let Some(v) = &self.trim_key {
            map.serialize_entry("trim_key", v)?;
        }
        if let Some(v) = &self.trim_value {
            map.serialize_entry("trim_value", v)?;
        }
        map.serialize_entry("value_split", &self.value_split)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct LowercaseProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist or is `null`, the processor quietly exits without modifying the document."]
    pub ignore_missing: Option<bool>,
    pub target_field: Option<crate::types::common::Field>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for LowercaseProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = LowercaseProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a LowercaseProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_ignore_missing: Option<bool> = None;
                let mut seen_ignore_missing = false;
                let mut f_target_field: Option<crate::types::common::Field> = None;
                let mut seen_target_field = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "ignore_missing" => {
                            if seen_ignore_missing {
                                return Err(serde::de::Error::duplicate_field("ignore_missing"));
                            }
                            seen_ignore_missing = true;
                            f_ignore_missing = map.next_value::<Option<bool>>()?;
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(LowercaseProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    ignore_missing: f_ignore_missing,
                    target_field: f_target_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for LowercaseProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.ignore_missing {
            map.serialize_entry("ignore_missing", v)?;
        }
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "Description of the ingest pipeline."]
    pub description: Option<String>,
    #[doc = "Processors to run immediately after a processor failure."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Processors used to perform transformations on documents before indexing. Processors run sequentially in the order specified."]
    pub processors: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    pub version: Option<crate::types::common::VersionNumber>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Pipeline {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Pipeline;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Pipeline object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_processors: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_processors = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("_meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "processors" => {
                            if seen_processors {
                                return Err(serde::de::Error::duplicate_field("processors"));
                            }
                            seen_processors = true;
                            f_processors = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionNumber>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Pipeline {
                    meta: f_meta,
                    description: f_description,
                    on_failure: f_on_failure,
                    processors: f_processors,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Pipeline {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("_meta", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.processors {
            map.serialize_entry("processors", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PipelineProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    #[doc = "Whether to ignore missing pipelines instead of failing."]
    pub ignore_missing_pipeline: Option<bool>,
    pub name: crate::types::common::Name,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PipelineProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PipelineProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PipelineProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_ignore_missing_pipeline: Option<bool> = None;
                let mut seen_ignore_missing_pipeline = false;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "ignore_missing_pipeline" => {
                            if seen_ignore_missing_pipeline {
                                return Err(serde::de::Error::duplicate_field(
                                    "ignore_missing_pipeline",
                                ));
                            }
                            seen_ignore_missing_pipeline = true;
                            f_ignore_missing_pipeline = map.next_value::<Option<bool>>()?;
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PipelineProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    ignore_missing_pipeline: f_ignore_missing_pipeline,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PipelineProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        if let Some(v) = &self.ignore_missing_pipeline {
            map.serialize_entry("ignore_missing_pipeline", v)?;
        }
        map.serialize_entry("name", &self.name)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ProcessorContainer {
    pub append: Option<crate::types::ingest_common::AppendProcessor>,
    pub attachment: Option<crate::types::ingest_common::AttachmentProcessor>,
    pub bytes: Option<crate::types::ingest_common::BytesProcessor>,
    pub circle: Option<crate::types::ingest_common::CircleProcessor>,
    pub convert: Option<crate::types::ingest_common::ConvertProcessor>,
    pub csv: Option<crate::types::ingest_common::CsvProcessor>,
    pub date: Option<crate::types::ingest_common::DateProcessor>,
    pub date_index_name: Option<crate::types::ingest_common::DateIndexNameProcessor>,
    pub dissect: Option<crate::types::ingest_common::DissectProcessor>,
    pub dot_expander: Option<crate::types::ingest_common::DotExpanderProcessor>,
    pub drop: Option<crate::types::ingest_common::DropProcessor>,
    pub fail: Option<crate::types::ingest_common::FailProcessor>,
    pub foreach: Option<Box<crate::types::ingest_common::ForeachProcessor>>,
    pub geoip: Option<crate::types::ingest_common::GeoIpProcessor>,
    pub grok: Option<crate::types::ingest_common::GrokProcessor>,
    pub gsub: Option<crate::types::ingest_common::GsubProcessor>,
    pub join: Option<crate::types::ingest_common::JoinProcessor>,
    pub json: Option<crate::types::ingest_common::JsonProcessor>,
    pub kv: Option<crate::types::ingest_common::KeyValueProcessor>,
    pub lowercase: Option<crate::types::ingest_common::LowercaseProcessor>,
    pub pipeline: Option<crate::types::ingest_common::PipelineProcessor>,
    pub remove: Option<crate::types::ingest_common::RemoveProcessor>,
    pub rename: Option<crate::types::ingest_common::RenameProcessor>,
    pub script: Option<crate::types::common::Script>,
    pub set: Option<crate::types::ingest_common::SetProcessor>,
    pub set_security_user: Option<crate::types::ingest_common::SetSecurityUserProcessor>,
    pub sort: Option<crate::types::ingest_common::SortProcessor>,
    pub split: Option<crate::types::ingest_common::SplitProcessor>,
    pub text_embedding: Option<crate::types::ingest_common::TextEmbeddingProcessor>,
    pub trim: Option<crate::types::ingest_common::TrimProcessor>,
    pub uppercase: Option<crate::types::ingest_common::UppercaseProcessor>,
    pub urldecode: Option<crate::types::ingest_common::UrlDecodeProcessor>,
    pub user_agent: Option<crate::types::ingest_common::UserAgentProcessor>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ProcessorContainer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ProcessorContainer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ProcessorContainer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_append: Option<crate::types::ingest_common::AppendProcessor> = None;
                let mut seen_append = false;
                let mut f_attachment: Option<crate::types::ingest_common::AttachmentProcessor> =
                    None;
                let mut seen_attachment = false;
                let mut f_bytes: Option<crate::types::ingest_common::BytesProcessor> = None;
                let mut seen_bytes = false;
                let mut f_circle: Option<crate::types::ingest_common::CircleProcessor> = None;
                let mut seen_circle = false;
                let mut f_convert: Option<crate::types::ingest_common::ConvertProcessor> = None;
                let mut seen_convert = false;
                let mut f_csv: Option<crate::types::ingest_common::CsvProcessor> = None;
                let mut seen_csv = false;
                let mut f_date: Option<crate::types::ingest_common::DateProcessor> = None;
                let mut seen_date = false;
                let mut f_date_index_name: Option<
                    crate::types::ingest_common::DateIndexNameProcessor,
                > = None;
                let mut seen_date_index_name = false;
                let mut f_dissect: Option<crate::types::ingest_common::DissectProcessor> = None;
                let mut seen_dissect = false;
                let mut f_dot_expander: Option<crate::types::ingest_common::DotExpanderProcessor> =
                    None;
                let mut seen_dot_expander = false;
                let mut f_drop: Option<crate::types::ingest_common::DropProcessor> = None;
                let mut seen_drop = false;
                let mut f_fail: Option<crate::types::ingest_common::FailProcessor> = None;
                let mut seen_fail = false;
                let mut f_foreach: Option<Box<crate::types::ingest_common::ForeachProcessor>> =
                    None;
                let mut seen_foreach = false;
                let mut f_geoip: Option<crate::types::ingest_common::GeoIpProcessor> = None;
                let mut seen_geoip = false;
                let mut f_grok: Option<crate::types::ingest_common::GrokProcessor> = None;
                let mut seen_grok = false;
                let mut f_gsub: Option<crate::types::ingest_common::GsubProcessor> = None;
                let mut seen_gsub = false;
                let mut f_join: Option<crate::types::ingest_common::JoinProcessor> = None;
                let mut seen_join = false;
                let mut f_json: Option<crate::types::ingest_common::JsonProcessor> = None;
                let mut seen_json = false;
                let mut f_kv: Option<crate::types::ingest_common::KeyValueProcessor> = None;
                let mut seen_kv = false;
                let mut f_lowercase: Option<crate::types::ingest_common::LowercaseProcessor> = None;
                let mut seen_lowercase = false;
                let mut f_pipeline: Option<crate::types::ingest_common::PipelineProcessor> = None;
                let mut seen_pipeline = false;
                let mut f_remove: Option<crate::types::ingest_common::RemoveProcessor> = None;
                let mut seen_remove = false;
                let mut f_rename: Option<crate::types::ingest_common::RenameProcessor> = None;
                let mut seen_rename = false;
                let mut f_script: Option<crate::types::common::Script> = None;
                let mut seen_script = false;
                let mut f_set: Option<crate::types::ingest_common::SetProcessor> = None;
                let mut seen_set = false;
                let mut f_set_security_user: Option<
                    crate::types::ingest_common::SetSecurityUserProcessor,
                > = None;
                let mut seen_set_security_user = false;
                let mut f_sort: Option<crate::types::ingest_common::SortProcessor> = None;
                let mut seen_sort = false;
                let mut f_split: Option<crate::types::ingest_common::SplitProcessor> = None;
                let mut seen_split = false;
                let mut f_text_embedding: Option<
                    crate::types::ingest_common::TextEmbeddingProcessor,
                > = None;
                let mut seen_text_embedding = false;
                let mut f_trim: Option<crate::types::ingest_common::TrimProcessor> = None;
                let mut seen_trim = false;
                let mut f_uppercase: Option<crate::types::ingest_common::UppercaseProcessor> = None;
                let mut seen_uppercase = false;
                let mut f_urldecode: Option<crate::types::ingest_common::UrlDecodeProcessor> = None;
                let mut seen_urldecode = false;
                let mut f_user_agent: Option<crate::types::ingest_common::UserAgentProcessor> =
                    None;
                let mut seen_user_agent = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "append" => {
                            if seen_append {
                                return Err(serde::de::Error::duplicate_field("append"));
                            }
                            seen_append = true;
                            f_append = map
                                .next_value::<Option<crate::types::ingest_common::AppendProcessor>>(
                                )?;
                        }
                        "attachment" => {
                            if seen_attachment {
                                return Err(serde::de::Error::duplicate_field("attachment"));
                            }
                            seen_attachment = true;
                            f_attachment = map.next_value::<Option<crate::types::ingest_common::AttachmentProcessor>>()?;
                        }
                        "bytes" => {
                            if seen_bytes {
                                return Err(serde::de::Error::duplicate_field("bytes"));
                            }
                            seen_bytes = true;
                            f_bytes = map
                                .next_value::<Option<crate::types::ingest_common::BytesProcessor>>(
                                )?;
                        }
                        "circle" => {
                            if seen_circle {
                                return Err(serde::de::Error::duplicate_field("circle"));
                            }
                            seen_circle = true;
                            f_circle = map
                                .next_value::<Option<crate::types::ingest_common::CircleProcessor>>(
                                )?;
                        }
                        "convert" => {
                            if seen_convert {
                                return Err(serde::de::Error::duplicate_field("convert"));
                            }
                            seen_convert = true;
                            f_convert = map.next_value::<Option<crate::types::ingest_common::ConvertProcessor>>()?;
                        }
                        "csv" => {
                            if seen_csv {
                                return Err(serde::de::Error::duplicate_field("csv"));
                            }
                            seen_csv = true;
                            f_csv = map
                                .next_value::<Option<crate::types::ingest_common::CsvProcessor>>(
                                )?;
                        }
                        "date" => {
                            if seen_date {
                                return Err(serde::de::Error::duplicate_field("date"));
                            }
                            seen_date = true;
                            f_date = map
                                .next_value::<Option<crate::types::ingest_common::DateProcessor>>(
                                )?;
                        }
                        "date_index_name" => {
                            if seen_date_index_name {
                                return Err(serde::de::Error::duplicate_field("date_index_name"));
                            }
                            seen_date_index_name = true;
                            f_date_index_name = map.next_value::<Option<crate::types::ingest_common::DateIndexNameProcessor>>()?;
                        }
                        "dissect" => {
                            if seen_dissect {
                                return Err(serde::de::Error::duplicate_field("dissect"));
                            }
                            seen_dissect = true;
                            f_dissect = map.next_value::<Option<crate::types::ingest_common::DissectProcessor>>()?;
                        }
                        "dot_expander" => {
                            if seen_dot_expander {
                                return Err(serde::de::Error::duplicate_field("dot_expander"));
                            }
                            seen_dot_expander = true;
                            f_dot_expander = map.next_value::<Option<crate::types::ingest_common::DotExpanderProcessor>>()?;
                        }
                        "drop" => {
                            if seen_drop {
                                return Err(serde::de::Error::duplicate_field("drop"));
                            }
                            seen_drop = true;
                            f_drop = map
                                .next_value::<Option<crate::types::ingest_common::DropProcessor>>(
                                )?;
                        }
                        "fail" => {
                            if seen_fail {
                                return Err(serde::de::Error::duplicate_field("fail"));
                            }
                            seen_fail = true;
                            f_fail = map
                                .next_value::<Option<crate::types::ingest_common::FailProcessor>>(
                                )?;
                        }
                        "foreach" => {
                            if seen_foreach {
                                return Err(serde::de::Error::duplicate_field("foreach"));
                            }
                            seen_foreach = true;
                            f_foreach = map.next_value::<Option<Box<crate::types::ingest_common::ForeachProcessor>>>()?;
                        }
                        "geoip" => {
                            if seen_geoip {
                                return Err(serde::de::Error::duplicate_field("geoip"));
                            }
                            seen_geoip = true;
                            f_geoip = map
                                .next_value::<Option<crate::types::ingest_common::GeoIpProcessor>>(
                                )?;
                        }
                        "grok" => {
                            if seen_grok {
                                return Err(serde::de::Error::duplicate_field("grok"));
                            }
                            seen_grok = true;
                            f_grok = map
                                .next_value::<Option<crate::types::ingest_common::GrokProcessor>>(
                                )?;
                        }
                        "gsub" => {
                            if seen_gsub {
                                return Err(serde::de::Error::duplicate_field("gsub"));
                            }
                            seen_gsub = true;
                            f_gsub = map
                                .next_value::<Option<crate::types::ingest_common::GsubProcessor>>(
                                )?;
                        }
                        "join" => {
                            if seen_join {
                                return Err(serde::de::Error::duplicate_field("join"));
                            }
                            seen_join = true;
                            f_join = map
                                .next_value::<Option<crate::types::ingest_common::JoinProcessor>>(
                                )?;
                        }
                        "json" => {
                            if seen_json {
                                return Err(serde::de::Error::duplicate_field("json"));
                            }
                            seen_json = true;
                            f_json = map
                                .next_value::<Option<crate::types::ingest_common::JsonProcessor>>(
                                )?;
                        }
                        "kv" => {
                            if seen_kv {
                                return Err(serde::de::Error::duplicate_field("kv"));
                            }
                            seen_kv = true;
                            f_kv = map.next_value::<Option<crate::types::ingest_common::KeyValueProcessor>>()?;
                        }
                        "lowercase" => {
                            if seen_lowercase {
                                return Err(serde::de::Error::duplicate_field("lowercase"));
                            }
                            seen_lowercase = true;
                            f_lowercase = map.next_value::<Option<crate::types::ingest_common::LowercaseProcessor>>()?;
                        }
                        "pipeline" => {
                            if seen_pipeline {
                                return Err(serde::de::Error::duplicate_field("pipeline"));
                            }
                            seen_pipeline = true;
                            f_pipeline = map.next_value::<Option<crate::types::ingest_common::PipelineProcessor>>()?;
                        }
                        "remove" => {
                            if seen_remove {
                                return Err(serde::de::Error::duplicate_field("remove"));
                            }
                            seen_remove = true;
                            f_remove = map
                                .next_value::<Option<crate::types::ingest_common::RemoveProcessor>>(
                                )?;
                        }
                        "rename" => {
                            if seen_rename {
                                return Err(serde::de::Error::duplicate_field("rename"));
                            }
                            seen_rename = true;
                            f_rename = map
                                .next_value::<Option<crate::types::ingest_common::RenameProcessor>>(
                                )?;
                        }
                        "script" => {
                            if seen_script {
                                return Err(serde::de::Error::duplicate_field("script"));
                            }
                            seen_script = true;
                            f_script = map.next_value::<Option<crate::types::common::Script>>()?;
                        }
                        "set" => {
                            if seen_set {
                                return Err(serde::de::Error::duplicate_field("set"));
                            }
                            seen_set = true;
                            f_set = map
                                .next_value::<Option<crate::types::ingest_common::SetProcessor>>(
                                )?;
                        }
                        "set_security_user" => {
                            if seen_set_security_user {
                                return Err(serde::de::Error::duplicate_field("set_security_user"));
                            }
                            seen_set_security_user = true;
                            f_set_security_user = map.next_value::<Option<crate::types::ingest_common::SetSecurityUserProcessor>>()?;
                        }
                        "sort" => {
                            if seen_sort {
                                return Err(serde::de::Error::duplicate_field("sort"));
                            }
                            seen_sort = true;
                            f_sort = map
                                .next_value::<Option<crate::types::ingest_common::SortProcessor>>(
                                )?;
                        }
                        "split" => {
                            if seen_split {
                                return Err(serde::de::Error::duplicate_field("split"));
                            }
                            seen_split = true;
                            f_split = map
                                .next_value::<Option<crate::types::ingest_common::SplitProcessor>>(
                                )?;
                        }
                        "text_embedding" => {
                            if seen_text_embedding {
                                return Err(serde::de::Error::duplicate_field("text_embedding"));
                            }
                            seen_text_embedding = true;
                            f_text_embedding = map.next_value::<Option<crate::types::ingest_common::TextEmbeddingProcessor>>()?;
                        }
                        "trim" => {
                            if seen_trim {
                                return Err(serde::de::Error::duplicate_field("trim"));
                            }
                            seen_trim = true;
                            f_trim = map
                                .next_value::<Option<crate::types::ingest_common::TrimProcessor>>(
                                )?;
                        }
                        "uppercase" => {
                            if seen_uppercase {
                                return Err(serde::de::Error::duplicate_field("uppercase"));
                            }
                            seen_uppercase = true;
                            f_uppercase = map.next_value::<Option<crate::types::ingest_common::UppercaseProcessor>>()?;
                        }
                        "urldecode" => {
                            if seen_urldecode {
                                return Err(serde::de::Error::duplicate_field("urldecode"));
                            }
                            seen_urldecode = true;
                            f_urldecode = map.next_value::<Option<crate::types::ingest_common::UrlDecodeProcessor>>()?;
                        }
                        "user_agent" => {
                            if seen_user_agent {
                                return Err(serde::de::Error::duplicate_field("user_agent"));
                            }
                            seen_user_agent = true;
                            f_user_agent = map.next_value::<Option<crate::types::ingest_common::UserAgentProcessor>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ProcessorContainer {
                    append: f_append,
                    attachment: f_attachment,
                    bytes: f_bytes,
                    circle: f_circle,
                    convert: f_convert,
                    csv: f_csv,
                    date: f_date,
                    date_index_name: f_date_index_name,
                    dissect: f_dissect,
                    dot_expander: f_dot_expander,
                    drop: f_drop,
                    fail: f_fail,
                    foreach: f_foreach,
                    geoip: f_geoip,
                    grok: f_grok,
                    gsub: f_gsub,
                    join: f_join,
                    json: f_json,
                    kv: f_kv,
                    lowercase: f_lowercase,
                    pipeline: f_pipeline,
                    remove: f_remove,
                    rename: f_rename,
                    script: f_script,
                    set: f_set,
                    set_security_user: f_set_security_user,
                    sort: f_sort,
                    split: f_split,
                    text_embedding: f_text_embedding,
                    trim: f_trim,
                    uppercase: f_uppercase,
                    urldecode: f_urldecode,
                    user_agent: f_user_agent,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ProcessorContainer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.append {
            map.serialize_entry("append", v)?;
        }
        if let Some(v) = &self.attachment {
            map.serialize_entry("attachment", v)?;
        }
        if let Some(v) = &self.bytes {
            map.serialize_entry("bytes", v)?;
        }
        if let Some(v) = &self.circle {
            map.serialize_entry("circle", v)?;
        }
        if let Some(v) = &self.convert {
            map.serialize_entry("convert", v)?;
        }
        if let Some(v) = &self.csv {
            map.serialize_entry("csv", v)?;
        }
        if let Some(v) = &self.date {
            map.serialize_entry("date", v)?;
        }
        if let Some(v) = &self.date_index_name {
            map.serialize_entry("date_index_name", v)?;
        }
        if let Some(v) = &self.dissect {
            map.serialize_entry("dissect", v)?;
        }
        if let Some(v) = &self.dot_expander {
            map.serialize_entry("dot_expander", v)?;
        }
        if let Some(v) = &self.drop {
            map.serialize_entry("drop", v)?;
        }
        if let Some(v) = &self.fail {
            map.serialize_entry("fail", v)?;
        }
        if let Some(v) = &self.foreach {
            map.serialize_entry("foreach", v)?;
        }
        if let Some(v) = &self.geoip {
            map.serialize_entry("geoip", v)?;
        }
        if let Some(v) = &self.grok {
            map.serialize_entry("grok", v)?;
        }
        if let Some(v) = &self.gsub {
            map.serialize_entry("gsub", v)?;
        }
        if let Some(v) = &self.join {
            map.serialize_entry("join", v)?;
        }
        if let Some(v) = &self.json {
            map.serialize_entry("json", v)?;
        }
        if let Some(v) = &self.kv {
            map.serialize_entry("kv", v)?;
        }
        if let Some(v) = &self.lowercase {
            map.serialize_entry("lowercase", v)?;
        }
        if let Some(v) = &self.pipeline {
            map.serialize_entry("pipeline", v)?;
        }
        if let Some(v) = &self.remove {
            map.serialize_entry("remove", v)?;
        }
        if let Some(v) = &self.rename {
            map.serialize_entry("rename", v)?;
        }
        if let Some(v) = &self.script {
            map.serialize_entry("script", v)?;
        }
        if let Some(v) = &self.set {
            map.serialize_entry("set", v)?;
        }
        if let Some(v) = &self.set_security_user {
            map.serialize_entry("set_security_user", v)?;
        }
        if let Some(v) = &self.sort {
            map.serialize_entry("sort", v)?;
        }
        if let Some(v) = &self.split {
            map.serialize_entry("split", v)?;
        }
        if let Some(v) = &self.text_embedding {
            map.serialize_entry("text_embedding", v)?;
        }
        if let Some(v) = &self.trim {
            map.serialize_entry("trim", v)?;
        }
        if let Some(v) = &self.uppercase {
            map.serialize_entry("uppercase", v)?;
        }
        if let Some(v) = &self.urldecode {
            map.serialize_entry("urldecode", v)?;
        }
        if let Some(v) = &self.user_agent {
            map.serialize_entry("user_agent", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RemoveProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    pub field: crate::types::common::Fields,
    #[doc = "If `true` and `field` does not exist or is `null`, the processor quietly exits without modifying the document."]
    pub ignore_missing: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RemoveProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RemoveProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RemoveProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_field: Option<crate::types::common::Fields> = None;
                let mut f_ignore_missing: Option<bool> = None;
                let mut seen_ignore_missing = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Fields>()?);
                        }
                        "ignore_missing" => {
                            if seen_ignore_missing {
                                return Err(serde::de::Error::duplicate_field("ignore_missing"));
                            }
                            seen_ignore_missing = true;
                            f_ignore_missing = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RemoveProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    ignore_missing: f_ignore_missing,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RemoveProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.ignore_missing {
            map.serialize_entry("ignore_missing", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RenameProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist, the processor quietly exits without modifying the document."]
    pub ignore_missing: Option<bool>,
    pub target_field: crate::types::common::Field,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RenameProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RenameProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RenameProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_ignore_missing: Option<bool> = None;
                let mut seen_ignore_missing = false;
                let mut f_target_field: Option<crate::types::common::Field> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "ignore_missing" => {
                            if seen_ignore_missing {
                                return Err(serde::de::Error::duplicate_field("ignore_missing"));
                            }
                            seen_ignore_missing = true;
                            f_ignore_missing = map.next_value::<Option<bool>>()?;
                        }
                        "target_field" => {
                            if f_target_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            f_target_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RenameProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    ignore_missing: f_ignore_missing,
                    target_field: f_target_field
                        .ok_or_else(|| serde::de::Error::missing_field("target_field"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RenameProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.ignore_missing {
            map.serialize_entry("ignore_missing", v)?;
        }
        map.serialize_entry("target_field", &self.target_field)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SetProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    pub copy_from: Option<crate::types::common::Field>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `value` is a template snippet that evaluates to `null` or the empty string, the processor quietly exits without modifying the document."]
    pub ignore_empty_value: Option<bool>,
    #[doc = "The media type for encoding `value`. Applies only when value is a template snippet. Must be one of `application/json`, `text/plain`, or `application/x-www-form-urlencoded`."]
    pub media_type: Option<String>,
    #[doc = "If `true` processor will update fields with pre-existing non-null-valued field. When set to `false`, such fields will not be touched."]
    pub r#override: Option<bool>,
    #[doc = "The value to be set for the field. Supports template snippets. May specify only one of `value` or `copy_from`."]
    pub value: Option<Box<serde_json::value::RawValue>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SetProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SetProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SetProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_copy_from: Option<crate::types::common::Field> = None;
                let mut seen_copy_from = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_ignore_empty_value: Option<bool> = None;
                let mut seen_ignore_empty_value = false;
                let mut f_media_type: Option<String> = None;
                let mut seen_media_type = false;
                let mut f_override: Option<bool> = None;
                let mut seen_override = false;
                let mut f_value: Option<Box<serde_json::value::RawValue>> = None;
                let mut seen_value = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "copy_from" => {
                            if seen_copy_from {
                                return Err(serde::de::Error::duplicate_field("copy_from"));
                            }
                            seen_copy_from = true;
                            f_copy_from =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "ignore_empty_value" => {
                            if seen_ignore_empty_value {
                                return Err(serde::de::Error::duplicate_field(
                                    "ignore_empty_value",
                                ));
                            }
                            seen_ignore_empty_value = true;
                            f_ignore_empty_value = map.next_value::<Option<bool>>()?;
                        }
                        "media_type" => {
                            if seen_media_type {
                                return Err(serde::de::Error::duplicate_field("media_type"));
                            }
                            seen_media_type = true;
                            f_media_type = map.next_value::<Option<String>>()?;
                        }
                        "override" => {
                            if seen_override {
                                return Err(serde::de::Error::duplicate_field("override"));
                            }
                            seen_override = true;
                            f_override = map.next_value::<Option<bool>>()?;
                        }
                        "value" => {
                            if seen_value {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            seen_value = true;
                            f_value =
                                map.next_value::<Option<Box<serde_json::value::RawValue>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SetProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    copy_from: f_copy_from,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    ignore_empty_value: f_ignore_empty_value,
                    media_type: f_media_type,
                    r#override: f_override,
                    value: f_value,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SetProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        if let Some(v) = &self.copy_from {
            map.serialize_entry("copy_from", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.ignore_empty_value {
            map.serialize_entry("ignore_empty_value", v)?;
        }
        if let Some(v) = &self.media_type {
            map.serialize_entry("media_type", v)?;
        }
        if let Some(v) = &self.r#override {
            map.serialize_entry("override", v)?;
        }
        if let Some(v) = &self.value {
            map.serialize_entry("value", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SetSecurityUserProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "Controls what user related properties are added to the field."]
    pub properties: Option<Vec<String>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SetSecurityUserProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SetSecurityUserProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SetSecurityUserProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_properties: Option<Vec<String>> = None;
                let mut seen_properties = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<Vec<String>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SetSecurityUserProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    properties: f_properties,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SetSecurityUserProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShapeType {
    #[doc = "`geo_shape`"]
    GeoShape,
    #[doc = "`xy_shape`"]
    XyShape,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ShapeType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ShapeType::GeoShape => "geo_shape",
            ShapeType::XyShape => "xy_shape",
            ShapeType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ShapeType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShapeType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "geo_shape" => ShapeType::GeoShape,
            "xy_shape" => ShapeType::XyShape,
            _ => ShapeType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SortProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    pub order: Option<crate::types::common::SortOrder>,
    pub target_field: Option<crate::types::common::Field>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SortProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SortProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SortProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_order: Option<crate::types::common::SortOrder> = None;
                let mut seen_order = false;
                let mut f_target_field: Option<crate::types::common::Field> = None;
                let mut seen_target_field = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "order" => {
                            if seen_order {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            seen_order = true;
                            f_order =
                                map.next_value::<Option<crate::types::common::SortOrder>>()?;
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SortProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    order: f_order,
                    target_field: f_target_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SortProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.order {
            map.serialize_entry("order", v)?;
        }
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SplitProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist, the processor quietly exits without modifying the document."]
    pub ignore_missing: Option<bool>,
    #[doc = "Preserves empty trailing fields, if any."]
    pub preserve_trailing: Option<bool>,
    #[doc = "A regex which matches the separator, for example, `,` or `\\s+`."]
    pub separator: String,
    pub target_field: Option<crate::types::common::Field>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SplitProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SplitProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SplitProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_ignore_missing: Option<bool> = None;
                let mut seen_ignore_missing = false;
                let mut f_preserve_trailing: Option<bool> = None;
                let mut seen_preserve_trailing = false;
                let mut f_separator: Option<String> = None;
                let mut f_target_field: Option<crate::types::common::Field> = None;
                let mut seen_target_field = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "ignore_missing" => {
                            if seen_ignore_missing {
                                return Err(serde::de::Error::duplicate_field("ignore_missing"));
                            }
                            seen_ignore_missing = true;
                            f_ignore_missing = map.next_value::<Option<bool>>()?;
                        }
                        "preserve_trailing" => {
                            if seen_preserve_trailing {
                                return Err(serde::de::Error::duplicate_field("preserve_trailing"));
                            }
                            seen_preserve_trailing = true;
                            f_preserve_trailing = map.next_value::<Option<bool>>()?;
                        }
                        "separator" => {
                            if f_separator.is_some() {
                                return Err(serde::de::Error::duplicate_field("separator"));
                            }
                            f_separator = Some(map.next_value::<String>()?);
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SplitProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    ignore_missing: f_ignore_missing,
                    preserve_trailing: f_preserve_trailing,
                    separator: f_separator
                        .ok_or_else(|| serde::de::Error::missing_field("separator"))?,
                    target_field: f_target_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SplitProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.ignore_missing {
            map.serialize_entry("ignore_missing", v)?;
        }
        if let Some(v) = &self.preserve_trailing {
            map.serialize_entry("preserve_trailing", v)?;
        }
        map.serialize_entry("separator", &self.separator)?;
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TextEmbeddingProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    pub batch_size: Option<i32>,
    #[doc = "Contains key-value pairs that specify the mapping of a text field to a vector field."]
    pub field_map: std::collections::BTreeMap<String, String>,
    pub model_id: crate::types::common::Id,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TextEmbeddingProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TextEmbeddingProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TextEmbeddingProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_batch_size: Option<i32> = None;
                let mut seen_batch_size = false;
                let mut f_field_map: Option<std::collections::BTreeMap<String, String>> = None;
                let mut f_model_id: Option<crate::types::common::Id> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "batch_size" => {
                            if seen_batch_size {
                                return Err(serde::de::Error::duplicate_field("batch_size"));
                            }
                            seen_batch_size = true;
                            f_batch_size = map.next_value::<Option<i32>>()?;
                        }
                        "field_map" => {
                            if f_field_map.is_some() {
                                return Err(serde::de::Error::duplicate_field("field_map"));
                            }
                            f_field_map = Some(
                                map.next_value::<std::collections::BTreeMap<String, String>>()?,
                            );
                        }
                        "model_id" => {
                            if f_model_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("model_id"));
                            }
                            f_model_id = Some(map.next_value::<crate::types::common::Id>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TextEmbeddingProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    batch_size: f_batch_size,
                    field_map: f_field_map
                        .ok_or_else(|| serde::de::Error::missing_field("field_map"))?,
                    model_id: f_model_id
                        .ok_or_else(|| serde::de::Error::missing_field("model_id"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TextEmbeddingProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        if let Some(v) = &self.batch_size {
            map.serialize_entry("batch_size", v)?;
        }
        map.serialize_entry("field_map", &self.field_map)?;
        map.serialize_entry("model_id", &self.model_id)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TrimProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist, the processor quietly exits without modifying the document."]
    pub ignore_missing: Option<bool>,
    pub target_field: Option<crate::types::common::Field>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TrimProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TrimProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TrimProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_ignore_missing: Option<bool> = None;
                let mut seen_ignore_missing = false;
                let mut f_target_field: Option<crate::types::common::Field> = None;
                let mut seen_target_field = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "ignore_missing" => {
                            if seen_ignore_missing {
                                return Err(serde::de::Error::duplicate_field("ignore_missing"));
                            }
                            seen_ignore_missing = true;
                            f_ignore_missing = map.next_value::<Option<bool>>()?;
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TrimProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    ignore_missing: f_ignore_missing,
                    target_field: f_target_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TrimProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.ignore_missing {
            map.serialize_entry("ignore_missing", v)?;
        }
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct UppercaseProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist or is `null`, the processor quietly exits without modifying the document."]
    pub ignore_missing: Option<bool>,
    pub target_field: Option<crate::types::common::Field>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UppercaseProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UppercaseProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UppercaseProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_ignore_missing: Option<bool> = None;
                let mut seen_ignore_missing = false;
                let mut f_target_field: Option<crate::types::common::Field> = None;
                let mut seen_target_field = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "ignore_missing" => {
                            if seen_ignore_missing {
                                return Err(serde::de::Error::duplicate_field("ignore_missing"));
                            }
                            seen_ignore_missing = true;
                            f_ignore_missing = map.next_value::<Option<bool>>()?;
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UppercaseProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    ignore_missing: f_ignore_missing,
                    target_field: f_target_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UppercaseProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.ignore_missing {
            map.serialize_entry("ignore_missing", v)?;
        }
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct UrlDecodeProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist or is `null`, the processor quietly exits without modifying the document."]
    pub ignore_missing: Option<bool>,
    pub target_field: Option<crate::types::common::Field>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UrlDecodeProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UrlDecodeProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UrlDecodeProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_ignore_missing: Option<bool> = None;
                let mut seen_ignore_missing = false;
                let mut f_target_field: Option<crate::types::common::Field> = None;
                let mut seen_target_field = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "ignore_missing" => {
                            if seen_ignore_missing {
                                return Err(serde::de::Error::duplicate_field("ignore_missing"));
                            }
                            seen_ignore_missing = true;
                            f_ignore_missing = map.next_value::<Option<bool>>()?;
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UrlDecodeProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    ignore_missing: f_ignore_missing,
                    target_field: f_target_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UrlDecodeProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.ignore_missing {
            map.serialize_entry("ignore_missing", v)?;
        }
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct UserAgentProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist, the processor quietly exits without modifying the document."]
    pub ignore_missing: Option<bool>,
    pub options: Option<Vec<crate::types::ingest_common::UserAgentProperty>>,
    #[doc = "The name of the file in the `config/ingest-user-agent` directory containing the regular expressions for parsing the user agent string. Both the directory and the file have to be created before starting OpenSearch. If not specified, ingest-user-agent will use the `regexes.yaml` from uap-core it ships with."]
    pub regex_file: Option<String>,
    pub target_field: Option<crate::types::common::Field>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UserAgentProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UserAgentProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UserAgentProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_if: Option<String> = None;
                let mut seen_if = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>> =
                    None;
                let mut seen_on_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_ignore_missing: Option<bool> = None;
                let mut seen_ignore_missing = false;
                let mut f_options: Option<Vec<crate::types::ingest_common::UserAgentProperty>> =
                    None;
                let mut seen_options = false;
                let mut f_regex_file: Option<String> = None;
                let mut seen_regex_file = false;
                let mut f_target_field: Option<crate::types::common::Field> = None;
                let mut seen_target_field = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "if" => {
                            if seen_if {
                                return Err(serde::de::Error::duplicate_field("if"));
                            }
                            seen_if = true;
                            f_if = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "on_failure" => {
                            if seen_on_failure {
                                return Err(serde::de::Error::duplicate_field("on_failure"));
                            }
                            seen_on_failure = true;
                            f_on_failure = map.next_value::<Option<Vec<crate::types::ingest_common::ProcessorContainer>>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "ignore_missing" => {
                            if seen_ignore_missing {
                                return Err(serde::de::Error::duplicate_field("ignore_missing"));
                            }
                            seen_ignore_missing = true;
                            f_ignore_missing = map.next_value::<Option<bool>>()?;
                        }
                        "options" => {
                            if seen_options {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            seen_options = true;
                            f_options = map.next_value::<Option<Vec<crate::types::ingest_common::UserAgentProperty>>>()?;
                        }
                        "regex_file" => {
                            if seen_regex_file {
                                return Err(serde::de::Error::duplicate_field("regex_file"));
                            }
                            seen_regex_file = true;
                            f_regex_file = map.next_value::<Option<String>>()?;
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UserAgentProcessor {
                    description: f_description,
                    r#if: f_if,
                    ignore_failure: f_ignore_failure,
                    on_failure: f_on_failure,
                    tag: f_tag,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    ignore_missing: f_ignore_missing,
                    options: f_options,
                    regex_file: f_regex_file,
                    target_field: f_target_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UserAgentProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.r#if {
            map.serialize_entry("if", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.on_failure {
            map.serialize_entry("on_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.ignore_missing {
            map.serialize_entry("ignore_missing", v)?;
        }
        if let Some(v) = &self.options {
            map.serialize_entry("options", v)?;
        }
        if let Some(v) = &self.regex_file {
            map.serialize_entry("regex_file", v)?;
        }
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserAgentProperty {
    #[doc = "`BUILD`"]
    Build,
    #[doc = "`DEVICE`"]
    Device,
    #[doc = "`MAJOR`"]
    Major,
    #[doc = "`MINOR`"]
    Minor,
    #[doc = "`NAME`"]
    Name,
    #[doc = "`OS`"]
    Os,
    #[doc = "`OS_MAJOR`"]
    OsMajor,
    #[doc = "`OS_MINOR`"]
    OsMinor,
    #[doc = "`OS_NAME`"]
    OsName,
    #[doc = "`PATCH`"]
    Patch,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl UserAgentProperty {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            UserAgentProperty::Build => "BUILD",
            UserAgentProperty::Device => "DEVICE",
            UserAgentProperty::Major => "MAJOR",
            UserAgentProperty::Minor => "MINOR",
            UserAgentProperty::Name => "NAME",
            UserAgentProperty::Os => "OS",
            UserAgentProperty::OsMajor => "OS_MAJOR",
            UserAgentProperty::OsMinor => "OS_MINOR",
            UserAgentProperty::OsName => "OS_NAME",
            UserAgentProperty::Patch => "PATCH",
            UserAgentProperty::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for UserAgentProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UserAgentProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "BUILD" => UserAgentProperty::Build,
            "DEVICE" => UserAgentProperty::Device,
            "MAJOR" => UserAgentProperty::Major,
            "MINOR" => UserAgentProperty::Minor,
            "NAME" => UserAgentProperty::Name,
            "OS" => UserAgentProperty::Os,
            "OS_MAJOR" => UserAgentProperty::OsMajor,
            "OS_MINOR" => UserAgentProperty::OsMinor,
            "OS_NAME" => UserAgentProperty::OsName,
            "PATCH" => UserAgentProperty::Patch,
            _ => UserAgentProperty::Unknown(s),
        })
    }
}
