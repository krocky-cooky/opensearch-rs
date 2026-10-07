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

//! Response types of the `sql_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct Cursor {
    #[doc = "The time to keep cursors alive."]
    pub keep_alive: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Cursor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Cursor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Cursor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_keep_alive: Option<String> = None;
                let mut seen_keep_alive = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "keep_alive" => {
                            if seen_keep_alive {
                                return Err(serde::de::Error::duplicate_field("keep_alive"));
                            }
                            seen_keep_alive = true;
                            f_keep_alive = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Cursor {
                    keep_alive: f_keep_alive,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Cursor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.keep_alive {
            map.serialize_entry("keep_alive", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ExplainBody {
    #[doc = "The child steps in the execution plan."]
    pub children: Option<Vec<crate::types::sql_common::ExplainBody>>,
    #[doc = "The details about the execution step."]
    pub description: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The name of the execution step."]
    pub name: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ExplainBody {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ExplainBody;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ExplainBody object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_children: Option<Vec<crate::types::sql_common::ExplainBody>> = None;
                let mut seen_children = false;
                let mut f_description: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_description = false;
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "children" => {
                            if seen_children {
                                return Err(serde::de::Error::duplicate_field("children"));
                            }
                            seen_children = true;
                            f_children = map
                                .next_value::<Option<Vec<crate::types::sql_common::ExplainBody>>>(
                                )?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ExplainBody {
                    children: f_children,
                    description: f_description,
                    name: f_name,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ExplainBody {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.children {
            map.serialize_entry("children", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ExplainResponse {
    pub root: Option<crate::types::sql_common::ExplainBody>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ExplainResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ExplainResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ExplainResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_root: Option<crate::types::sql_common::ExplainBody> = None;
                let mut seen_root = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "root" => {
                            if seen_root {
                                return Err(serde::de::Error::duplicate_field("root"));
                            }
                            seen_root = true;
                            f_root =
                                map.next_value::<Option<crate::types::sql_common::ExplainBody>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ExplainResponse {
                    root: f_root,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ExplainResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.root {
            map.serialize_entry("root", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Plugins {
    pub ppl: Option<crate::types::sql_common::Ppl>,
    pub query: Option<crate::types::sql_common::PluginsQuery>,
    pub sql: Option<crate::types::sql_common::Sql>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Plugins {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Plugins;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Plugins object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_ppl: Option<crate::types::sql_common::Ppl> = None;
                let mut seen_ppl = false;
                let mut f_query: Option<crate::types::sql_common::PluginsQuery> = None;
                let mut seen_query = false;
                let mut f_sql: Option<crate::types::sql_common::Sql> = None;
                let mut seen_sql = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "ppl" => {
                            if seen_ppl {
                                return Err(serde::de::Error::duplicate_field("ppl"));
                            }
                            seen_ppl = true;
                            f_ppl = map.next_value::<Option<crate::types::sql_common::Ppl>>()?;
                        }
                        "query" => {
                            if seen_query {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            seen_query = true;
                            f_query =
                                map.next_value::<Option<crate::types::sql_common::PluginsQuery>>()?;
                        }
                        "sql" => {
                            if seen_sql {
                                return Err(serde::de::Error::duplicate_field("sql"));
                            }
                            seen_sql = true;
                            f_sql = map.next_value::<Option<crate::types::sql_common::Sql>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Plugins {
                    ppl: f_ppl,
                    query: f_query,
                    sql: f_sql,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Plugins {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.ppl {
            map.serialize_entry("ppl", v)?;
        }
        if let Some(v) = &self.query {
            map.serialize_entry("query", v)?;
        }
        if let Some(v) = &self.sql {
            map.serialize_entry("sql", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PluginsQuery {
    #[doc = "The maximum memory allowed for queries."]
    pub memory_limit: Option<String>,
    #[doc = "The maximum result set size."]
    pub size_limit: Option<crate::types::sql_common::PluginsQuerySizeLimit>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PluginsQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PluginsQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PluginsQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_memory_limit: Option<String> = None;
                let mut seen_memory_limit = false;
                let mut f_size_limit: Option<crate::types::sql_common::PluginsQuerySizeLimit> =
                    None;
                let mut seen_size_limit = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "memory_limit" => {
                            if seen_memory_limit {
                                return Err(serde::de::Error::duplicate_field("memory_limit"));
                            }
                            seen_memory_limit = true;
                            f_memory_limit = map.next_value::<Option<String>>()?;
                        }
                        "size_limit" => {
                            if seen_size_limit {
                                return Err(serde::de::Error::duplicate_field("size_limit"));
                            }
                            seen_size_limit = true;
                            f_size_limit = map.next_value::<Option<crate::types::sql_common::PluginsQuerySizeLimit>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PluginsQuery {
                    memory_limit: f_memory_limit,
                    size_limit: f_size_limit,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PluginsQuery {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.memory_limit {
            map.serialize_entry("memory_limit", v)?;
        }
        if let Some(v) = &self.size_limit {
            map.serialize_entry("size_limit", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The maximum result set size."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum PluginsQuerySizeLimit {
    String(String),
    Integer(i64),
}
impl<'de> serde::Deserialize<'de> for PluginsQuerySizeLimit {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'"') => serde_json::from_str(text).map(PluginsQuerySizeLimit::String),
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(PluginsQuerySizeLimit::Integer)
            }
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a PluginsQuerySizeLimit value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct Ppl {
    #[doc = "Whether the PPL plugin is enabled."]
    pub enabled: Option<crate::types::sql_common::PplEnabled>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Ppl {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Ppl;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Ppl object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_enabled: Option<crate::types::sql_common::PplEnabled> = None;
                let mut seen_enabled = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "enabled" => {
                            if seen_enabled {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            seen_enabled = true;
                            f_enabled =
                                map.next_value::<Option<crate::types::sql_common::PplEnabled>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Ppl {
                    enabled: f_enabled,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Ppl {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.enabled {
            map.serialize_entry("enabled", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Whether the PPL plugin is enabled."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum PplEnabled {
    Boolean(bool),
    String(String),
}
impl<'de> serde::Deserialize<'de> for PplEnabled {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b't') | Some(b'f') => serde_json::from_str(text).map(PplEnabled::Boolean),
            Some(b'"') => serde_json::from_str(text).map(PplEnabled::String),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a PplEnabled value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct QueryResponse {
    #[doc = "The cursor identifier for pagination."]
    pub cursor: Option<String>,
    #[doc = "The rows of data returned by the query."]
    pub datarows: Option<Vec<Vec<Box<serde_json::value::RawValue>>>>,
    #[doc = "The schema of the query result."]
    pub schema: Option<Vec<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>>,
    #[doc = "The number of rows in the current response."]
    pub size: Option<i64>,
    #[doc = "The status code of the response."]
    pub status: Option<i64>,
    #[doc = "The total number of hits."]
    pub total: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for QueryResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = QueryResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a QueryResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cursor: Option<String> = None;
                let mut seen_cursor = false;
                let mut f_datarows: Option<Vec<Vec<Box<serde_json::value::RawValue>>>> = None;
                let mut seen_datarows = false;
                let mut f_schema: Option<
                    Vec<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
                > = None;
                let mut seen_schema = false;
                let mut f_size: Option<i64> = None;
                let mut seen_size = false;
                let mut f_status: Option<i64> = None;
                let mut seen_status = false;
                let mut f_total: Option<i64> = None;
                let mut seen_total = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cursor" => {
                            if seen_cursor {
                                return Err(serde::de::Error::duplicate_field("cursor"));
                            }
                            seen_cursor = true;
                            f_cursor = map.next_value::<Option<String>>()?;
                        }
                        "datarows" => {
                            if seen_datarows {
                                return Err(serde::de::Error::duplicate_field("datarows"));
                            }
                            seen_datarows = true;
                            f_datarows = map
                                .next_value::<Option<Vec<Vec<Box<serde_json::value::RawValue>>>>>(
                                )?;
                        }
                        "schema" => {
                            if seen_schema {
                                return Err(serde::de::Error::duplicate_field("schema"));
                            }
                            seen_schema = true;
                            f_schema = map.next_value::<Option<
                                Vec<
                                    std::collections::BTreeMap<
                                        String,
                                        Box<serde_json::value::RawValue>,
                                    >,
                                >,
                            >>()?;
                        }
                        "size" => {
                            if seen_size {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            seen_size = true;
                            f_size = map.next_value::<Option<i64>>()?;
                        }
                        "status" => {
                            if seen_status {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            seen_status = true;
                            f_status = map.next_value::<Option<i64>>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(QueryResponse {
                    cursor: f_cursor,
                    datarows: f_datarows,
                    schema: f_schema,
                    size: f_size,
                    status: f_status,
                    total: f_total,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for QueryResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cursor {
            map.serialize_entry("cursor", v)?;
        }
        if let Some(v) = &self.datarows {
            map.serialize_entry("datarows", v)?;
        }
        if let Some(v) = &self.schema {
            map.serialize_entry("schema", v)?;
        }
        if let Some(v) = &self.size {
            map.serialize_entry("size", v)?;
        }
        if let Some(v) = &self.status {
            map.serialize_entry("status", v)?;
        }
        if let Some(v) = &self.total {
            map.serialize_entry("total", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Sql {
    pub cursor: Option<crate::types::sql_common::Cursor>,
    #[doc = "Whether the SQL plugin is enabled."]
    pub enabled: Option<crate::types::sql_common::SqlEnabled>,
    #[doc = "The threshold in milliseconds for logging slow queries."]
    pub slowlog: Option<crate::types::sql_common::SqlSlowlog>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Sql {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Sql;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Sql object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cursor: Option<crate::types::sql_common::Cursor> = None;
                let mut seen_cursor = false;
                let mut f_enabled: Option<crate::types::sql_common::SqlEnabled> = None;
                let mut seen_enabled = false;
                let mut f_slowlog: Option<crate::types::sql_common::SqlSlowlog> = None;
                let mut seen_slowlog = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cursor" => {
                            if seen_cursor {
                                return Err(serde::de::Error::duplicate_field("cursor"));
                            }
                            seen_cursor = true;
                            f_cursor =
                                map.next_value::<Option<crate::types::sql_common::Cursor>>()?;
                        }
                        "enabled" => {
                            if seen_enabled {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            seen_enabled = true;
                            f_enabled =
                                map.next_value::<Option<crate::types::sql_common::SqlEnabled>>()?;
                        }
                        "slowlog" => {
                            if seen_slowlog {
                                return Err(serde::de::Error::duplicate_field("slowlog"));
                            }
                            seen_slowlog = true;
                            f_slowlog =
                                map.next_value::<Option<crate::types::sql_common::SqlSlowlog>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Sql {
                    cursor: f_cursor,
                    enabled: f_enabled,
                    slowlog: f_slowlog,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Sql {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cursor {
            map.serialize_entry("cursor", v)?;
        }
        if let Some(v) = &self.enabled {
            map.serialize_entry("enabled", v)?;
        }
        if let Some(v) = &self.slowlog {
            map.serialize_entry("slowlog", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SqlCloseResponse {
    #[doc = "Whether the cursor was closed successfully."]
    pub succeeded: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SqlCloseResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SqlCloseResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SqlCloseResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_succeeded: Option<bool> = None;
                let mut seen_succeeded = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "succeeded" => {
                            if seen_succeeded {
                                return Err(serde::de::Error::duplicate_field("succeeded"));
                            }
                            seen_succeeded = true;
                            f_succeeded = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SqlCloseResponse {
                    succeeded: f_succeeded,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SqlCloseResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.succeeded {
            map.serialize_entry("succeeded", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Whether the SQL plugin is enabled."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SqlEnabled {
    Boolean(bool),
    String(String),
}
impl<'de> serde::Deserialize<'de> for SqlEnabled {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b't') | Some(b'f') => serde_json::from_str(text).map(SqlEnabled::Boolean),
            Some(b'"') => serde_json::from_str(text).map(SqlEnabled::String),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a SqlEnabled value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct SqlSettingsResponse {
    #[doc = "Whether the settings were acknowledged."]
    pub acknowledged: Option<bool>,
    #[doc = "The permanent settings that persist through restarts."]
    pub persistent: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub transient: Option<crate::types::sql_common::Transient>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SqlSettingsResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SqlSettingsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SqlSettingsResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_acknowledged: Option<bool> = None;
                let mut seen_acknowledged = false;
                let mut f_persistent: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_persistent = false;
                let mut f_transient: Option<crate::types::sql_common::Transient> = None;
                let mut seen_transient = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "acknowledged" => {
                            if seen_acknowledged {
                                return Err(serde::de::Error::duplicate_field("acknowledged"));
                            }
                            seen_acknowledged = true;
                            f_acknowledged = map.next_value::<Option<bool>>()?;
                        }
                        "persistent" => {
                            if seen_persistent {
                                return Err(serde::de::Error::duplicate_field("persistent"));
                            }
                            seen_persistent = true;
                            f_persistent = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "transient" => {
                            if seen_transient {
                                return Err(serde::de::Error::duplicate_field("transient"));
                            }
                            seen_transient = true;
                            f_transient =
                                map.next_value::<Option<crate::types::sql_common::Transient>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SqlSettingsResponse {
                    acknowledged: f_acknowledged,
                    persistent: f_persistent,
                    transient: f_transient,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SqlSettingsResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.acknowledged {
            map.serialize_entry("acknowledged", v)?;
        }
        if let Some(v) = &self.persistent {
            map.serialize_entry("persistent", v)?;
        }
        if let Some(v) = &self.transient {
            map.serialize_entry("transient", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The threshold in milliseconds for logging slow queries."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SqlSlowlog {
    Integer(i64),
    String(String),
}
impl<'de> serde::Deserialize<'de> for SqlSlowlog {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'-') | Some(b'0'..=b'9') => serde_json::from_str(text).map(SqlSlowlog::Integer),
            Some(b'"') => serde_json::from_str(text).map(SqlSlowlog::String),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a SqlSlowlog value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct Transient {
    pub plugins: Option<crate::types::sql_common::Plugins>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Transient {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Transient;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Transient object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_plugins: Option<crate::types::sql_common::Plugins> = None;
                let mut seen_plugins = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "plugins" => {
                            if seen_plugins {
                                return Err(serde::de::Error::duplicate_field("plugins"));
                            }
                            seen_plugins = true;
                            f_plugins =
                                map.next_value::<Option<crate::types::sql_common::Plugins>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Transient {
                    plugins: f_plugins,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Transient {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.plugins {
            map.serialize_entry("plugins", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
