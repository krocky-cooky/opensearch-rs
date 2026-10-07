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

//! Response types of the `indices_common` specification namespace

#![allow(missing_docs)]

#[doc = "The index alias configuration."]
#[derive(Debug, Clone)]
pub struct Alias {
    pub filter: Option<crate::types::common_query_dsl::QueryContainer>,
    pub index_routing: Option<crate::types::common::Routing>,
    #[doc = "When `true`, the alias is hidden. All indexes for the alias must have the same `is_hidden` value."]
    pub is_hidden: Option<bool>,
    #[doc = "When `true`, the index is the write index for the alias."]
    pub is_write_index: Option<bool>,
    pub routing: Option<crate::types::common::Routing>,
    pub search_routing: Option<crate::types::common::Routing>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Alias {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Alias;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Alias object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_filter: Option<crate::types::common_query_dsl::QueryContainer> = None;
                let mut seen_filter = false;
                let mut f_index_routing: Option<crate::types::common::Routing> = None;
                let mut seen_index_routing = false;
                let mut f_is_hidden: Option<bool> = None;
                let mut seen_is_hidden = false;
                let mut f_is_write_index: Option<bool> = None;
                let mut seen_is_write_index = false;
                let mut f_routing: Option<crate::types::common::Routing> = None;
                let mut seen_routing = false;
                let mut f_search_routing: Option<crate::types::common::Routing> = None;
                let mut seen_search_routing = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "filter" => {
                            if seen_filter {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            seen_filter = true;
                            f_filter = map.next_value::<Option<crate::types::common_query_dsl::QueryContainer>>()?;
                        }
                        "index_routing" => {
                            if seen_index_routing {
                                return Err(serde::de::Error::duplicate_field("index_routing"));
                            }
                            seen_index_routing = true;
                            f_index_routing =
                                map.next_value::<Option<crate::types::common::Routing>>()?;
                        }
                        "is_hidden" => {
                            if seen_is_hidden {
                                return Err(serde::de::Error::duplicate_field("is_hidden"));
                            }
                            seen_is_hidden = true;
                            f_is_hidden = map.next_value::<Option<bool>>()?;
                        }
                        "is_write_index" => {
                            if seen_is_write_index {
                                return Err(serde::de::Error::duplicate_field("is_write_index"));
                            }
                            seen_is_write_index = true;
                            f_is_write_index = map.next_value::<Option<bool>>()?;
                        }
                        "routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("routing"));
                            }
                            seen_routing = true;
                            f_routing =
                                map.next_value::<Option<crate::types::common::Routing>>()?;
                        }
                        "search_routing" => {
                            if seen_search_routing {
                                return Err(serde::de::Error::duplicate_field("search_routing"));
                            }
                            seen_search_routing = true;
                            f_search_routing =
                                map.next_value::<Option<crate::types::common::Routing>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Alias {
                    filter: f_filter,
                    index_routing: f_index_routing,
                    is_hidden: f_is_hidden,
                    is_write_index: f_is_write_index,
                    routing: f_routing,
                    search_routing: f_search_routing,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Alias {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.filter {
            map.serialize_entry("filter", v)?;
        }
        if let Some(v) = &self.index_routing {
            map.serialize_entry("index_routing", v)?;
        }
        if let Some(v) = &self.is_hidden {
            map.serialize_entry("is_hidden", v)?;
        }
        if let Some(v) = &self.is_write_index {
            map.serialize_entry("is_write_index", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("routing", v)?;
        }
        if let Some(v) = &self.search_routing {
            map.serialize_entry("search_routing", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for an index alias."]
#[derive(Debug, Clone)]
pub struct AliasDefinition {
    pub filter: Option<crate::types::common_query_dsl::QueryContainer>,
    #[doc = "The value used to route indexing operations to a specific shard. When specified, this overwrites the `routing` value for indexing operations."]
    pub index_routing: Option<String>,
    #[doc = "When `true`, the alias is hidden. All indexes for the alias must have the same `is_hidden` value."]
    pub is_hidden: Option<bool>,
    #[doc = "When `true`, the index is the write index for the alias."]
    pub is_write_index: Option<bool>,
    #[doc = "The value used to route indexing and search operations to a specific shard."]
    pub routing: Option<String>,
    #[doc = "The value used to route search operations to a specific shard. When specified, this overwrites the `routing` value for search operations."]
    pub search_routing: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AliasDefinition {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AliasDefinition;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AliasDefinition object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_filter: Option<crate::types::common_query_dsl::QueryContainer> = None;
                let mut seen_filter = false;
                let mut f_index_routing: Option<String> = None;
                let mut seen_index_routing = false;
                let mut f_is_hidden: Option<bool> = None;
                let mut seen_is_hidden = false;
                let mut f_is_write_index: Option<bool> = None;
                let mut seen_is_write_index = false;
                let mut f_routing: Option<String> = None;
                let mut seen_routing = false;
                let mut f_search_routing: Option<String> = None;
                let mut seen_search_routing = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "filter" => {
                            if seen_filter {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            seen_filter = true;
                            f_filter = map.next_value::<Option<crate::types::common_query_dsl::QueryContainer>>()?;
                        }
                        "index_routing" => {
                            if seen_index_routing {
                                return Err(serde::de::Error::duplicate_field("index_routing"));
                            }
                            seen_index_routing = true;
                            f_index_routing = map.next_value::<Option<String>>()?;
                        }
                        "is_hidden" => {
                            if seen_is_hidden {
                                return Err(serde::de::Error::duplicate_field("is_hidden"));
                            }
                            seen_is_hidden = true;
                            f_is_hidden = map.next_value::<Option<bool>>()?;
                        }
                        "is_write_index" => {
                            if seen_is_write_index {
                                return Err(serde::de::Error::duplicate_field("is_write_index"));
                            }
                            seen_is_write_index = true;
                            f_is_write_index = map.next_value::<Option<bool>>()?;
                        }
                        "routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("routing"));
                            }
                            seen_routing = true;
                            f_routing = map.next_value::<Option<String>>()?;
                        }
                        "search_routing" => {
                            if seen_search_routing {
                                return Err(serde::de::Error::duplicate_field("search_routing"));
                            }
                            seen_search_routing = true;
                            f_search_routing = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AliasDefinition {
                    filter: f_filter,
                    index_routing: f_index_routing,
                    is_hidden: f_is_hidden,
                    is_write_index: f_is_write_index,
                    routing: f_routing,
                    search_routing: f_search_routing,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AliasDefinition {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.filter {
            map.serialize_entry("filter", v)?;
        }
        if let Some(v) = &self.index_routing {
            map.serialize_entry("index_routing", v)?;
        }
        if let Some(v) = &self.is_hidden {
            map.serialize_entry("is_hidden", v)?;
        }
        if let Some(v) = &self.is_write_index {
            map.serialize_entry("is_write_index", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("routing", v)?;
        }
        if let Some(v) = &self.search_routing {
            map.serialize_entry("search_routing", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The type of storage implementation."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuiltinStorageType {
    #[doc = "`fs`"]
    Fs,
    #[doc = "`hybridfs`"]
    Hybridfs,
    #[doc = "`mmapfs`"]
    Mmapfs,
    #[doc = "`niofs`"]
    Niofs,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl BuiltinStorageType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            BuiltinStorageType::Fs => "fs",
            BuiltinStorageType::Hybridfs => "hybridfs",
            BuiltinStorageType::Mmapfs => "mmapfs",
            BuiltinStorageType::Niofs => "niofs",
            BuiltinStorageType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for BuiltinStorageType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BuiltinStorageType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "fs" => BuiltinStorageType::Fs,
            "hybridfs" => BuiltinStorageType::Hybridfs,
            "mmapfs" => BuiltinStorageType::Mmapfs,
            "niofs" => BuiltinStorageType::Niofs,
            _ => BuiltinStorageType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct DataStream {
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "When `true`, the data stream allows custom routing on write request."]
    pub allow_custom_routing: Option<bool>,
    #[doc = "The current generation number for the data stream. This number acts as a cumulative count of the stream's rollovers, starting at 1."]
    pub generation: i64,
    #[doc = "When `true`, the data stream is hidden."]
    pub hidden: Option<bool>,
    pub ilm_policy: Option<crate::types::common::Name>,
    #[doc = "An array of objects containing information about the data stream's backing indexes. The last item in this array contains information about the stream's current write index."]
    pub indices: Vec<crate::types::indices_common::DataStreamIndex>,
    pub name: crate::types::common::DataStreamName,
    pub next_generation_managed_by: Option<crate::types::indices_common::ManagedBy>,
    #[doc = "Indicates if ILM should take precedence over DSL in case both are configured to managed this data stream."]
    pub prefer_ilm: Option<bool>,
    #[doc = "When `true`, the data stream is created and managed by cross-cluster replication and the local cluster can not write into this data stream or change its mappings."]
    pub replicated: Option<bool>,
    pub status: crate::types::common::HealthStatus,
    #[doc = "When `true`, the data stream is created and managed by an OpenSearch stack component and cannot be modified through normal user interaction."]
    pub system: Option<bool>,
    pub template: crate::types::common::Name,
    pub timestamp_field: crate::types::indices_common::DataStreamTimestampField,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DataStream {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DataStream;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DataStream object")
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
                let mut f_allow_custom_routing: Option<bool> = None;
                let mut seen_allow_custom_routing = false;
                let mut f_generation: Option<i64> = None;
                let mut f_hidden: Option<bool> = None;
                let mut seen_hidden = false;
                let mut f_ilm_policy: Option<crate::types::common::Name> = None;
                let mut seen_ilm_policy = false;
                let mut f_indices: Option<Vec<crate::types::indices_common::DataStreamIndex>> =
                    None;
                let mut f_name: Option<crate::types::common::DataStreamName> = None;
                let mut f_next_generation_managed_by: Option<
                    crate::types::indices_common::ManagedBy,
                > = None;
                let mut seen_next_generation_managed_by = false;
                let mut f_prefer_ilm: Option<bool> = None;
                let mut seen_prefer_ilm = false;
                let mut f_replicated: Option<bool> = None;
                let mut seen_replicated = false;
                let mut f_status: Option<crate::types::common::HealthStatus> = None;
                let mut f_system: Option<bool> = None;
                let mut seen_system = false;
                let mut f_template: Option<crate::types::common::Name> = None;
                let mut f_timestamp_field: Option<
                    crate::types::indices_common::DataStreamTimestampField,
                > = None;
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
                        "allow_custom_routing" => {
                            if seen_allow_custom_routing {
                                return Err(serde::de::Error::duplicate_field(
                                    "allow_custom_routing",
                                ));
                            }
                            seen_allow_custom_routing = true;
                            f_allow_custom_routing = map.next_value::<Option<bool>>()?;
                        }
                        "generation" => {
                            if f_generation.is_some() {
                                return Err(serde::de::Error::duplicate_field("generation"));
                            }
                            f_generation = Some(map.next_value::<i64>()?);
                        }
                        "hidden" => {
                            if seen_hidden {
                                return Err(serde::de::Error::duplicate_field("hidden"));
                            }
                            seen_hidden = true;
                            f_hidden = map.next_value::<Option<bool>>()?;
                        }
                        "ilm_policy" => {
                            if seen_ilm_policy {
                                return Err(serde::de::Error::duplicate_field("ilm_policy"));
                            }
                            seen_ilm_policy = true;
                            f_ilm_policy =
                                map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "indices" => {
                            if f_indices.is_some() {
                                return Err(serde::de::Error::duplicate_field("indices"));
                            }
                            f_indices = Some(map.next_value::<Vec<crate::types::indices_common::DataStreamIndex>>()?);
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name =
                                Some(map.next_value::<crate::types::common::DataStreamName>()?);
                        }
                        "next_generation_managed_by" => {
                            if seen_next_generation_managed_by {
                                return Err(serde::de::Error::duplicate_field(
                                    "next_generation_managed_by",
                                ));
                            }
                            seen_next_generation_managed_by = true;
                            f_next_generation_managed_by = map
                                .next_value::<Option<crate::types::indices_common::ManagedBy>>()?;
                        }
                        "prefer_ilm" => {
                            if seen_prefer_ilm {
                                return Err(serde::de::Error::duplicate_field("prefer_ilm"));
                            }
                            seen_prefer_ilm = true;
                            f_prefer_ilm = map.next_value::<Option<bool>>()?;
                        }
                        "replicated" => {
                            if seen_replicated {
                                return Err(serde::de::Error::duplicate_field("replicated"));
                            }
                            seen_replicated = true;
                            f_replicated = map.next_value::<Option<bool>>()?;
                        }
                        "status" => {
                            if f_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            f_status =
                                Some(map.next_value::<crate::types::common::HealthStatus>()?);
                        }
                        "system" => {
                            if seen_system {
                                return Err(serde::de::Error::duplicate_field("system"));
                            }
                            seen_system = true;
                            f_system = map.next_value::<Option<bool>>()?;
                        }
                        "template" => {
                            if f_template.is_some() {
                                return Err(serde::de::Error::duplicate_field("template"));
                            }
                            f_template = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        "timestamp_field" => {
                            if f_timestamp_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp_field"));
                            }
                            f_timestamp_field = Some(map.next_value::<crate::types::indices_common::DataStreamTimestampField>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DataStream {
                    meta: f_meta,
                    allow_custom_routing: f_allow_custom_routing,
                    generation: f_generation
                        .ok_or_else(|| serde::de::Error::missing_field("generation"))?,
                    hidden: f_hidden,
                    ilm_policy: f_ilm_policy,
                    indices: f_indices.ok_or_else(|| serde::de::Error::missing_field("indices"))?,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    next_generation_managed_by: f_next_generation_managed_by,
                    prefer_ilm: f_prefer_ilm,
                    replicated: f_replicated,
                    status: f_status.ok_or_else(|| serde::de::Error::missing_field("status"))?,
                    system: f_system,
                    template: f_template
                        .ok_or_else(|| serde::de::Error::missing_field("template"))?,
                    timestamp_field: f_timestamp_field
                        .ok_or_else(|| serde::de::Error::missing_field("timestamp_field"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DataStream {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("_meta", v)?;
        }
        if let Some(v) = &self.allow_custom_routing {
            map.serialize_entry("allow_custom_routing", v)?;
        }
        map.serialize_entry("generation", &self.generation)?;
        if let Some(v) = &self.hidden {
            map.serialize_entry("hidden", v)?;
        }
        if let Some(v) = &self.ilm_policy {
            map.serialize_entry("ilm_policy", v)?;
        }
        map.serialize_entry("indices", &self.indices)?;
        map.serialize_entry("name", &self.name)?;
        if let Some(v) = &self.next_generation_managed_by {
            map.serialize_entry("next_generation_managed_by", v)?;
        }
        if let Some(v) = &self.prefer_ilm {
            map.serialize_entry("prefer_ilm", v)?;
        }
        if let Some(v) = &self.replicated {
            map.serialize_entry("replicated", v)?;
        }
        map.serialize_entry("status", &self.status)?;
        if let Some(v) = &self.system {
            map.serialize_entry("system", v)?;
        }
        map.serialize_entry("template", &self.template)?;
        map.serialize_entry("timestamp_field", &self.timestamp_field)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for a data stream's backing index."]
#[derive(Debug, Clone)]
pub struct DataStreamIndex {
    pub ilm_policy: Option<crate::types::common::Name>,
    pub index_name: crate::types::common::IndexName,
    pub index_uuid: crate::types::common::Uuid,
    pub managed_by: Option<crate::types::indices_common::ManagedBy>,
    #[doc = "Indicates if ILM should take precedence over DSL in case both are configured to manage this index."]
    pub prefer_ilm: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DataStreamIndex {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DataStreamIndex;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DataStreamIndex object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_ilm_policy: Option<crate::types::common::Name> = None;
                let mut seen_ilm_policy = false;
                let mut f_index_name: Option<crate::types::common::IndexName> = None;
                let mut f_index_uuid: Option<crate::types::common::Uuid> = None;
                let mut f_managed_by: Option<crate::types::indices_common::ManagedBy> = None;
                let mut seen_managed_by = false;
                let mut f_prefer_ilm: Option<bool> = None;
                let mut seen_prefer_ilm = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "ilm_policy" => {
                            if seen_ilm_policy {
                                return Err(serde::de::Error::duplicate_field("ilm_policy"));
                            }
                            seen_ilm_policy = true;
                            f_ilm_policy =
                                map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "index_name" => {
                            if f_index_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("index_name"));
                            }
                            f_index_name =
                                Some(map.next_value::<crate::types::common::IndexName>()?);
                        }
                        "index_uuid" => {
                            if f_index_uuid.is_some() {
                                return Err(serde::de::Error::duplicate_field("index_uuid"));
                            }
                            f_index_uuid = Some(map.next_value::<crate::types::common::Uuid>()?);
                        }
                        "managed_by" => {
                            if seen_managed_by {
                                return Err(serde::de::Error::duplicate_field("managed_by"));
                            }
                            seen_managed_by = true;
                            f_managed_by = map
                                .next_value::<Option<crate::types::indices_common::ManagedBy>>()?;
                        }
                        "prefer_ilm" => {
                            if seen_prefer_ilm {
                                return Err(serde::de::Error::duplicate_field("prefer_ilm"));
                            }
                            seen_prefer_ilm = true;
                            f_prefer_ilm = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DataStreamIndex {
                    ilm_policy: f_ilm_policy,
                    index_name: f_index_name
                        .ok_or_else(|| serde::de::Error::missing_field("index_name"))?,
                    index_uuid: f_index_uuid
                        .ok_or_else(|| serde::de::Error::missing_field("index_uuid"))?,
                    managed_by: f_managed_by,
                    prefer_ilm: f_prefer_ilm,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DataStreamIndex {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.ilm_policy {
            map.serialize_entry("ilm_policy", v)?;
        }
        map.serialize_entry("index_name", &self.index_name)?;
        map.serialize_entry("index_uuid", &self.index_uuid)?;
        if let Some(v) = &self.managed_by {
            map.serialize_entry("managed_by", v)?;
        }
        if let Some(v) = &self.prefer_ilm {
            map.serialize_entry("prefer_ilm", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The statistics for a data stream."]
#[derive(Debug, Clone)]
pub struct DataStreamStats {
    #[doc = "Current number of backing indexes for the data stream."]
    pub backing_indices: i32,
    pub data_stream: crate::types::common::Name,
    pub maximum_timestamp: crate::types::common::EpochTimeUnitMillis,
    pub store_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total size, in bytes, of all shards for the data stream's backing indexes."]
    pub store_size_bytes: crate::types::common::ByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DataStreamStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DataStreamStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DataStreamStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_backing_indices: Option<i32> = None;
                let mut f_data_stream: Option<crate::types::common::Name> = None;
                let mut f_maximum_timestamp: Option<crate::types::common::EpochTimeUnitMillis> =
                    None;
                let mut f_store_size: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_store_size = false;
                let mut f_store_size_bytes: Option<crate::types::common::ByteCount> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "backing_indices" => {
                            if f_backing_indices.is_some() {
                                return Err(serde::de::Error::duplicate_field("backing_indices"));
                            }
                            f_backing_indices = Some(map.next_value::<i32>()?);
                        }
                        "data_stream" => {
                            if f_data_stream.is_some() {
                                return Err(serde::de::Error::duplicate_field("data_stream"));
                            }
                            f_data_stream = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        "maximum_timestamp" => {
                            if f_maximum_timestamp.is_some() {
                                return Err(serde::de::Error::duplicate_field("maximum_timestamp"));
                            }
                            f_maximum_timestamp = Some(
                                map.next_value::<crate::types::common::EpochTimeUnitMillis>()?,
                            );
                        }
                        "store_size" => {
                            if seen_store_size {
                                return Err(serde::de::Error::duplicate_field("store_size"));
                            }
                            seen_store_size = true;
                            f_store_size = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "store_size_bytes" => {
                            if f_store_size_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("store_size_bytes"));
                            }
                            f_store_size_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DataStreamStats {
                    backing_indices: f_backing_indices
                        .ok_or_else(|| serde::de::Error::missing_field("backing_indices"))?,
                    data_stream: f_data_stream
                        .ok_or_else(|| serde::de::Error::missing_field("data_stream"))?,
                    maximum_timestamp: f_maximum_timestamp
                        .ok_or_else(|| serde::de::Error::missing_field("maximum_timestamp"))?,
                    store_size: f_store_size,
                    store_size_bytes: f_store_size_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("store_size_bytes"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DataStreamStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("backing_indices", &self.backing_indices)?;
        map.serialize_entry("data_stream", &self.data_stream)?;
        map.serialize_entry("maximum_timestamp", &self.maximum_timestamp)?;
        if let Some(v) = &self.store_size {
            map.serialize_entry("store_size", v)?;
        }
        map.serialize_entry("store_size_bytes", &self.store_size_bytes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The timestamp field configuration for a data stream."]
#[derive(Debug, Clone)]
pub struct DataStreamTimestampField {
    pub name: crate::types::common::Field,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DataStreamTimestampField {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DataStreamTimestampField;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DataStreamTimestampField object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<crate::types::common::Field> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DataStreamTimestampField {
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DataStreamTimestampField {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("name", &self.name)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The frequency filter configuration for field data."]
#[derive(Debug, Clone)]
pub struct FielddataFrequencyFilter {
    #[doc = "The maximum frequency threshold."]
    pub max: f64,
    #[doc = "The minimum frequency threshold."]
    pub min: f64,
    #[doc = "The minimum segment size to apply the filter."]
    pub min_segment_size: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FielddataFrequencyFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FielddataFrequencyFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FielddataFrequencyFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_max: Option<f64> = None;
                let mut f_min: Option<f64> = None;
                let mut f_min_segment_size: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "max" => {
                            if f_max.is_some() {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            f_max = Some(map.next_value::<f64>()?);
                        }
                        "min" => {
                            if f_min.is_some() {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            f_min = Some(map.next_value::<f64>()?);
                        }
                        "min_segment_size" => {
                            if f_min_segment_size.is_some() {
                                return Err(serde::de::Error::duplicate_field("min_segment_size"));
                            }
                            f_min_segment_size = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FielddataFrequencyFilter {
                    max: f_max.ok_or_else(|| serde::de::Error::missing_field("max"))?,
                    min: f_min.ok_or_else(|| serde::de::Error::missing_field("min"))?,
                    min_segment_size: f_min_segment_size
                        .ok_or_else(|| serde::de::Error::missing_field("min_segment_size"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FielddataFrequencyFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("max", &self.max)?;
        map.serialize_entry("min", &self.min)?;
        map.serialize_entry("min_segment_size", &self.min_segment_size)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The type of check to perform when starting an index."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexCheckOnStartup {
    #[doc = "`checksum`"]
    Checksum,
    #[doc = "`false`"]
    False,
    #[doc = "`true`"]
    True,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexCheckOnStartup {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexCheckOnStartup::Checksum => "checksum",
            IndexCheckOnStartup::False => "false",
            IndexCheckOnStartup::True => "true",
            IndexCheckOnStartup::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexCheckOnStartup {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexCheckOnStartup {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "checksum" => IndexCheckOnStartup::Checksum,
            "false" => IndexCheckOnStartup::False,
            "true" => IndexCheckOnStartup::True,
            _ => IndexCheckOnStartup::Unknown(s),
        })
    }
}

#[doc = "The upgrade status information for indexes."]
#[derive(Debug, Clone)]
pub struct IndexGetUpgradeStatus {
    #[doc = "The upgrade status for individual indexes."]
    pub indices:
        Option<std::collections::BTreeMap<String, crate::types::indices_common::UpgradeStatus>>,
    pub size_in_bytes: Option<crate::types::common::ByteCount>,
    pub size_to_upgrade_ancient_in_bytes: Option<crate::types::common::ByteCount>,
    pub size_to_upgrade_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexGetUpgradeStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexGetUpgradeStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexGetUpgradeStatus object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_indices: Option<
                    std::collections::BTreeMap<String, crate::types::indices_common::UpgradeStatus>,
                > = None;
                let mut seen_indices = false;
                let mut f_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_size_in_bytes = false;
                let mut f_size_to_upgrade_ancient_in_bytes: Option<
                    crate::types::common::ByteCount,
                > = None;
                let mut seen_size_to_upgrade_ancient_in_bytes = false;
                let mut f_size_to_upgrade_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_size_to_upgrade_in_bytes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "indices" => {
                            if seen_indices {
                                return Err(serde::de::Error::duplicate_field("indices"));
                            }
                            seen_indices = true;
                            f_indices = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::indices_common::UpgradeStatus,
                                >,
                            >>()?;
                        }
                        "size_in_bytes" => {
                            if seen_size_in_bytes {
                                return Err(serde::de::Error::duplicate_field("size_in_bytes"));
                            }
                            seen_size_in_bytes = true;
                            f_size_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "size_to_upgrade_ancient_in_bytes" => {
                            if seen_size_to_upgrade_ancient_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "size_to_upgrade_ancient_in_bytes",
                                ));
                            }
                            seen_size_to_upgrade_ancient_in_bytes = true;
                            f_size_to_upgrade_ancient_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "size_to_upgrade_in_bytes" => {
                            if seen_size_to_upgrade_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "size_to_upgrade_in_bytes",
                                ));
                            }
                            seen_size_to_upgrade_in_bytes = true;
                            f_size_to_upgrade_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexGetUpgradeStatus {
                    indices: f_indices,
                    size_in_bytes: f_size_in_bytes,
                    size_to_upgrade_ancient_in_bytes: f_size_to_upgrade_ancient_in_bytes,
                    size_to_upgrade_in_bytes: f_size_to_upgrade_in_bytes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexGetUpgradeStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.indices {
            map.serialize_entry("indices", v)?;
        }
        if let Some(v) = &self.size_in_bytes {
            map.serialize_entry("size_in_bytes", v)?;
        }
        if let Some(v) = &self.size_to_upgrade_ancient_in_bytes {
            map.serialize_entry("size_to_upgrade_ancient_in_bytes", v)?;
        }
        if let Some(v) = &self.size_to_upgrade_in_bytes {
            map.serialize_entry("size_to_upgrade_in_bytes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The routing configuration for index operations."]
#[derive(Debug, Clone)]
pub struct IndexRouting {
    pub allocation: Option<crate::types::indices_common::IndexRoutingAllocation>,
    pub rebalance: Option<crate::types::indices_common::IndexRoutingRebalance>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexRouting {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexRouting;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexRouting object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_allocation: Option<crate::types::indices_common::IndexRoutingAllocation> =
                    None;
                let mut seen_allocation = false;
                let mut f_rebalance: Option<crate::types::indices_common::IndexRoutingRebalance> =
                    None;
                let mut seen_rebalance = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "allocation" => {
                            if seen_allocation {
                                return Err(serde::de::Error::duplicate_field("allocation"));
                            }
                            seen_allocation = true;
                            f_allocation = map.next_value::<Option<crate::types::indices_common::IndexRoutingAllocation>>()?;
                        }
                        "rebalance" => {
                            if seen_rebalance {
                                return Err(serde::de::Error::duplicate_field("rebalance"));
                            }
                            seen_rebalance = true;
                            f_rebalance = map.next_value::<Option<crate::types::indices_common::IndexRoutingRebalance>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexRouting {
                    allocation: f_allocation,
                    rebalance: f_rebalance,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexRouting {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.allocation {
            map.serialize_entry("allocation", v)?;
        }
        if let Some(v) = &self.rebalance {
            map.serialize_entry("rebalance", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for shard allocation."]
#[derive(Debug, Clone)]
pub struct IndexRoutingAllocation {
    pub disk: Option<crate::types::indices_common::IndexRoutingAllocationDisk>,
    pub enable: Option<crate::types::indices_common::IndexRoutingAllocationOptions>,
    pub include: Option<crate::types::indices_common::IndexRoutingAllocationInclude>,
    pub initial_recovery:
        Option<crate::types::indices_common::IndexRoutingAllocationInitialRecovery>,
    pub total_primary_shards_per_node: Option<crate::types::common::StringifiedInteger>,
    pub total_shards_per_node: Option<crate::types::common::StringifiedInteger>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexRoutingAllocation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexRoutingAllocation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexRoutingAllocation object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_disk: Option<crate::types::indices_common::IndexRoutingAllocationDisk> =
                    None;
                let mut seen_disk = false;
                let mut f_enable: Option<
                    crate::types::indices_common::IndexRoutingAllocationOptions,
                > = None;
                let mut seen_enable = false;
                let mut f_include: Option<
                    crate::types::indices_common::IndexRoutingAllocationInclude,
                > = None;
                let mut seen_include = false;
                let mut f_initial_recovery: Option<
                    crate::types::indices_common::IndexRoutingAllocationInitialRecovery,
                > = None;
                let mut seen_initial_recovery = false;
                let mut f_total_primary_shards_per_node: Option<
                    crate::types::common::StringifiedInteger,
                > = None;
                let mut seen_total_primary_shards_per_node = false;
                let mut f_total_shards_per_node: Option<crate::types::common::StringifiedInteger> =
                    None;
                let mut seen_total_shards_per_node = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "disk" => {
                            if seen_disk {
                                return Err(serde::de::Error::duplicate_field("disk"));
                            }
                            seen_disk = true;
                            f_disk =
                                map.next_value::<Option<
                                    crate::types::indices_common::IndexRoutingAllocationDisk,
                                >>()?;
                        }
                        "enable" => {
                            if seen_enable {
                                return Err(serde::de::Error::duplicate_field("enable"));
                            }
                            seen_enable = true;
                            f_enable = map.next_value::<Option<
                                crate::types::indices_common::IndexRoutingAllocationOptions,
                            >>()?;
                        }
                        "include" => {
                            if seen_include {
                                return Err(serde::de::Error::duplicate_field("include"));
                            }
                            seen_include = true;
                            f_include = map.next_value::<Option<
                                crate::types::indices_common::IndexRoutingAllocationInclude,
                            >>()?;
                        }
                        "initial_recovery" => {
                            if seen_initial_recovery {
                                return Err(serde::de::Error::duplicate_field("initial_recovery"));
                            }
                            seen_initial_recovery = true;
                            f_initial_recovery = map.next_value::<Option<
                                crate::types::indices_common::IndexRoutingAllocationInitialRecovery,
                            >>()?;
                        }
                        "total_primary_shards_per_node" => {
                            if seen_total_primary_shards_per_node {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_primary_shards_per_node",
                                ));
                            }
                            seen_total_primary_shards_per_node = true;
                            f_total_primary_shards_per_node = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "total_shards_per_node" => {
                            if seen_total_shards_per_node {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_shards_per_node",
                                ));
                            }
                            seen_total_shards_per_node = true;
                            f_total_shards_per_node = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexRoutingAllocation {
                    disk: f_disk,
                    enable: f_enable,
                    include: f_include,
                    initial_recovery: f_initial_recovery,
                    total_primary_shards_per_node: f_total_primary_shards_per_node,
                    total_shards_per_node: f_total_shards_per_node,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexRoutingAllocation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.disk {
            map.serialize_entry("disk", v)?;
        }
        if let Some(v) = &self.enable {
            map.serialize_entry("enable", v)?;
        }
        if let Some(v) = &self.include {
            map.serialize_entry("include", v)?;
        }
        if let Some(v) = &self.initial_recovery {
            map.serialize_entry("initial_recovery", v)?;
        }
        if let Some(v) = &self.total_primary_shards_per_node {
            map.serialize_entry("total_primary_shards_per_node", v)?;
        }
        if let Some(v) = &self.total_shards_per_node {
            map.serialize_entry("total_shards_per_node", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The disk-based allocation settings."]
#[derive(Debug, Clone)]
pub struct IndexRoutingAllocationDisk {
    pub threshold_enabled: Option<crate::types::common::StringifiedBoolean>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexRoutingAllocationDisk {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexRoutingAllocationDisk;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexRoutingAllocationDisk object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_threshold_enabled: Option<crate::types::common::StringifiedBoolean> =
                    None;
                let mut seen_threshold_enabled = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "threshold_enabled" => {
                            if seen_threshold_enabled {
                                return Err(serde::de::Error::duplicate_field("threshold_enabled"));
                            }
                            seen_threshold_enabled = true;
                            f_threshold_enabled = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexRoutingAllocationDisk {
                    threshold_enabled: f_threshold_enabled,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexRoutingAllocationDisk {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.threshold_enabled {
            map.serialize_entry("threshold_enabled", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The inclusion rules for shard allocation."]
#[derive(Debug, Clone)]
pub struct IndexRoutingAllocationInclude {
    pub id: Option<crate::types::common::Id>,
    #[doc = "The data tier preference for the index."]
    pub tier_preference: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexRoutingAllocationInclude {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexRoutingAllocationInclude;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexRoutingAllocationInclude object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut seen_id = false;
                let mut f_tier_preference: Option<String> = None;
                let mut seen_tier_preference = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("_id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        "_tier_preference" => {
                            if seen_tier_preference {
                                return Err(serde::de::Error::duplicate_field("_tier_preference"));
                            }
                            seen_tier_preference = true;
                            f_tier_preference = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexRoutingAllocationInclude {
                    id: f_id,
                    tier_preference: f_tier_preference,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexRoutingAllocationInclude {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.id {
            map.serialize_entry("_id", v)?;
        }
        if let Some(v) = &self.tier_preference {
            map.serialize_entry("_tier_preference", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for initial shard recovery."]
#[derive(Debug, Clone)]
pub struct IndexRoutingAllocationInitialRecovery {
    pub id: Option<crate::types::common::Id>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexRoutingAllocationInitialRecovery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexRoutingAllocationInitialRecovery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexRoutingAllocationInitialRecovery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut seen_id = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("_id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexRoutingAllocationInitialRecovery {
                    id: f_id,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexRoutingAllocationInitialRecovery {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.id {
            map.serialize_entry("_id", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The options for shard allocation control."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexRoutingAllocationOptions {
    #[doc = "`all`"]
    All,
    #[doc = "`new_primaries`"]
    NewPrimaries,
    #[doc = "`none`"]
    None,
    #[doc = "`primaries`"]
    Primaries,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexRoutingAllocationOptions {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexRoutingAllocationOptions::All => "all",
            IndexRoutingAllocationOptions::NewPrimaries => "new_primaries",
            IndexRoutingAllocationOptions::None => "none",
            IndexRoutingAllocationOptions::Primaries => "primaries",
            IndexRoutingAllocationOptions::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexRoutingAllocationOptions {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexRoutingAllocationOptions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "all" => IndexRoutingAllocationOptions::All,
            "new_primaries" => IndexRoutingAllocationOptions::NewPrimaries,
            "none" => IndexRoutingAllocationOptions::None,
            "primaries" => IndexRoutingAllocationOptions::Primaries,
            _ => IndexRoutingAllocationOptions::Unknown(s),
        })
    }
}

#[doc = "The configuration for shard rebalancing."]
#[derive(Debug, Clone)]
pub struct IndexRoutingRebalance {
    pub enable: crate::types::indices_common::IndexRoutingRebalanceOptions,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexRoutingRebalance {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexRoutingRebalance;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexRoutingRebalance object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_enable: Option<
                    crate::types::indices_common::IndexRoutingRebalanceOptions,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "enable" => {
                            if f_enable.is_some() {
                                return Err(serde::de::Error::duplicate_field("enable"));
                            }
                            f_enable = Some(map.next_value::<crate::types::indices_common::IndexRoutingRebalanceOptions>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexRoutingRebalance {
                    enable: f_enable.ok_or_else(|| serde::de::Error::missing_field("enable"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexRoutingRebalance {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("enable", &self.enable)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The options for shard rebalancing control."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexRoutingRebalanceOptions {
    #[doc = "`all`"]
    All,
    #[doc = "`none`"]
    None,
    #[doc = "`primaries`"]
    Primaries,
    #[doc = "`replicas`"]
    Replicas,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexRoutingRebalanceOptions {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexRoutingRebalanceOptions::All => "all",
            IndexRoutingRebalanceOptions::None => "none",
            IndexRoutingRebalanceOptions::Primaries => "primaries",
            IndexRoutingRebalanceOptions::Replicas => "replicas",
            IndexRoutingRebalanceOptions::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexRoutingRebalanceOptions {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexRoutingRebalanceOptions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "all" => IndexRoutingRebalanceOptions::All,
            "none" => IndexRoutingRebalanceOptions::None,
            "primaries" => IndexRoutingRebalanceOptions::Primaries,
            "replicas" => IndexRoutingRebalanceOptions::Replicas,
            _ => IndexRoutingRebalanceOptions::Unknown(s),
        })
    }
}

#[doc = "The configuration for segment sorting."]
#[derive(Debug, Clone)]
pub struct IndexSegmentSort {
    pub field: Option<crate::types::common::Fields>,
    pub missing: Option<crate::types::indices_common::IndexSegmentSortMissing>,
    pub mode: Option<crate::types::indices_common::IndexSegmentSortMode>,
    pub order: Option<crate::types::indices_common::IndexSegmentSortOrder>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSegmentSort {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSegmentSort;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSegmentSort object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_field: Option<crate::types::common::Fields> = None;
                let mut seen_field = false;
                let mut f_missing: Option<crate::types::indices_common::IndexSegmentSortMissing> =
                    None;
                let mut seen_missing = false;
                let mut f_mode: Option<crate::types::indices_common::IndexSegmentSortMode> = None;
                let mut seen_mode = false;
                let mut f_order: Option<crate::types::indices_common::IndexSegmentSortOrder> = None;
                let mut seen_order = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "field" => {
                            if seen_field {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            seen_field = true;
                            f_field = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "missing" => {
                            if seen_missing {
                                return Err(serde::de::Error::duplicate_field("missing"));
                            }
                            seen_missing = true;
                            f_missing = map.next_value::<Option<crate::types::indices_common::IndexSegmentSortMissing>>()?;
                        }
                        "mode" => {
                            if seen_mode {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            seen_mode = true;
                            f_mode = map.next_value::<Option<crate::types::indices_common::IndexSegmentSortMode>>()?;
                        }
                        "order" => {
                            if seen_order {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            seen_order = true;
                            f_order = map.next_value::<Option<crate::types::indices_common::IndexSegmentSortOrder>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSegmentSort {
                    field: f_field,
                    missing: f_missing,
                    mode: f_mode,
                    order: f_order,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSegmentSort {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.field {
            map.serialize_entry("field", v)?;
        }
        if let Some(v) = &self.missing {
            map.serialize_entry("missing", v)?;
        }
        if let Some(v) = &self.mode {
            map.serialize_entry("mode", v)?;
        }
        if let Some(v) = &self.order {
            map.serialize_entry("order", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum IndexSegmentSortMissing {
    SegmentSortMissing(crate::types::indices_common::SegmentSortMissing),
    Array(Vec<crate::types::indices_common::SegmentSortMissing>),
}
impl<'de> serde::Deserialize<'de> for IndexSegmentSortMissing {
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
            Some(b'"') => {
                serde_json::from_str(text).map(IndexSegmentSortMissing::SegmentSortMissing)
            }
            Some(b'[') => serde_json::from_str(text).map(IndexSegmentSortMissing::Array),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a IndexSegmentSortMissing value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum IndexSegmentSortMode {
    SegmentSortMode(crate::types::indices_common::SegmentSortMode),
    Array(Vec<crate::types::indices_common::SegmentSortMode>),
}
impl<'de> serde::Deserialize<'de> for IndexSegmentSortMode {
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
            Some(b'"') => serde_json::from_str(text).map(IndexSegmentSortMode::SegmentSortMode),
            Some(b'[') => serde_json::from_str(text).map(IndexSegmentSortMode::Array),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a IndexSegmentSortMode value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum IndexSegmentSortOrder {
    SegmentSortOrder(crate::types::indices_common::SegmentSortOrder),
    Array(Vec<crate::types::indices_common::SegmentSortOrder>),
}
impl<'de> serde::Deserialize<'de> for IndexSegmentSortOrder {
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
            Some(b'"') => serde_json::from_str(text).map(IndexSegmentSortOrder::SegmentSortOrder),
            Some(b'[') => serde_json::from_str(text).map(IndexSegmentSortOrder::Array),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a IndexSegmentSortOrder value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The block settings that control index operations."]
#[derive(Debug, Clone)]
pub struct IndexSettingBlocks {
    pub metadata: Option<crate::types::common::StringifiedBoolean>,
    pub read: Option<crate::types::common::StringifiedBoolean>,
    pub read_only: Option<crate::types::common::StringifiedBoolean>,
    pub read_only_allow_delete: Option<crate::types::common::StringifiedBoolean>,
    #[doc = "When true, the index is in search-only mode, allowing only read operations"]
    pub search_only: Option<crate::types::common::StringifiedBoolean>,
    pub write: Option<crate::types::common::StringifiedBoolean>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingBlocks {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingBlocks;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingBlocks object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_metadata: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_metadata = false;
                let mut f_read: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_read = false;
                let mut f_read_only: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_read_only = false;
                let mut f_read_only_allow_delete: Option<crate::types::common::StringifiedBoolean> =
                    None;
                let mut seen_read_only_allow_delete = false;
                let mut f_search_only: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_search_only = false;
                let mut f_write: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_write = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "metadata" => {
                            if seen_metadata {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            seen_metadata = true;
                            f_metadata = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "read" => {
                            if seen_read {
                                return Err(serde::de::Error::duplicate_field("read"));
                            }
                            seen_read = true;
                            f_read = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "read_only" => {
                            if seen_read_only {
                                return Err(serde::de::Error::duplicate_field("read_only"));
                            }
                            seen_read_only = true;
                            f_read_only = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "read_only_allow_delete" => {
                            if seen_read_only_allow_delete {
                                return Err(serde::de::Error::duplicate_field(
                                    "read_only_allow_delete",
                                ));
                            }
                            seen_read_only_allow_delete = true;
                            f_read_only_allow_delete = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "search_only" => {
                            if seen_search_only {
                                return Err(serde::de::Error::duplicate_field("search_only"));
                            }
                            seen_search_only = true;
                            f_search_only = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "write" => {
                            if seen_write {
                                return Err(serde::de::Error::duplicate_field("write"));
                            }
                            seen_write = true;
                            f_write = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingBlocks {
                    metadata: f_metadata,
                    read: f_read,
                    read_only: f_read_only,
                    read_only_allow_delete: f_read_only_allow_delete,
                    search_only: f_search_only,
                    write: f_write,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingBlocks {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.metadata {
            map.serialize_entry("metadata", v)?;
        }
        if let Some(v) = &self.read {
            map.serialize_entry("read", v)?;
        }
        if let Some(v) = &self.read_only {
            map.serialize_entry("read_only", v)?;
        }
        if let Some(v) = &self.read_only_allow_delete {
            map.serialize_entry("read_only_allow_delete", v)?;
        }
        if let Some(v) = &self.search_only {
            map.serialize_entry("search_only", v)?;
        }
        if let Some(v) = &self.write {
            map.serialize_entry("write", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration settings for an index."]
#[derive(Debug, Clone)]
pub struct IndexSettings {
    pub analysis: Option<crate::types::indices_common::IndexSettingsAnalysis>,
    pub analyze: Option<crate::types::indices_common::IndexSettingsAnalyze>,
    pub analyze_max_token_count: Option<crate::types::common::StringifiedInteger>,
    #[doc = "The range of replicas to maintain."]
    pub auto_expand_replicas: Option<String>,
    pub blocks: Option<crate::types::indices_common::IndexSettingBlocks>,
    pub blocks_metadata: Option<crate::types::common::StringifiedBoolean>,
    pub blocks_read: Option<crate::types::common::StringifiedBoolean>,
    pub blocks_read_only: Option<crate::types::common::StringifiedBoolean>,
    pub blocks_read_only_allow_delete: Option<crate::types::common::StringifiedBoolean>,
    pub blocks_write: Option<crate::types::common::StringifiedBoolean>,
    pub check_on_startup: Option<crate::types::indices_common::IndexCheckOnStartup>,
    #[doc = "The compression type for stored data."]
    pub codec: Option<String>,
    pub composite_index_star_tree: Option<crate::types::indices_common::IndexSettingsStarTree>,
    pub creation_date: Option<crate::types::common::StringifiedEpochTimeUnitMillis>,
    pub creation_date_string: Option<crate::types::common::DateTime>,
    pub default_pipeline: Option<crate::types::common::PipelineName>,
    pub final_pipeline: Option<crate::types::common::PipelineName>,
    pub format: Option<crate::types::common::StringifiedInteger>,
    pub gc_deletes: Option<crate::types::common::Duration>,
    pub hidden: Option<crate::types::common::StringifiedBoolean>,
    pub highlight: Option<crate::types::indices_common::IndexSettingsHighlight>,
    pub highlight_max_analyzed_offset: Option<crate::types::common::StringifiedInteger>,
    pub index: Option<Box<crate::types::indices_common::IndexSettings>>,
    pub indexing: Option<crate::types::indices_common::IndexSettingsIndexing>,
    pub indexing_pressure: Option<crate::types::indices_common::IndexingPressure>,
    pub ingestion_source: Option<crate::types::indices_common::IngestionSource>,
    pub knn: Option<crate::types::common::StringifiedBoolean>,
    pub knn_algo_param_ef_search: Option<crate::types::common::StringifiedInteger>,
    pub lifecycle: Option<crate::types::indices_common::IndexSettingsLifecycle>,
    pub lifecycle_name: Option<crate::types::common::Name>,
    pub load_fixed_bitset_filters_eagerly: Option<crate::types::common::StringifiedBoolean>,
    pub mapping: Option<crate::types::indices_common::IndexSettingsMapping>,
    pub max_docvalue_fields_search: Option<crate::types::common::StringifiedInteger>,
    pub max_inner_result_window: Option<crate::types::common::StringifiedInteger>,
    pub max_ngram_diff: Option<crate::types::common::StringifiedInteger>,
    pub max_refresh_listeners: Option<crate::types::common::StringifiedInteger>,
    pub max_regex_length: Option<crate::types::common::StringifiedInteger>,
    pub max_rescore_window: Option<crate::types::common::StringifiedInteger>,
    pub max_result_window: Option<crate::types::common::StringifiedInteger>,
    pub max_script_fields: Option<crate::types::common::StringifiedInteger>,
    pub max_shingle_diff: Option<crate::types::common::StringifiedInteger>,
    pub max_slices_per_scroll: Option<crate::types::common::StringifiedInteger>,
    pub max_terms_count: Option<crate::types::common::StringifiedInteger>,
    pub merge: Option<crate::types::indices_common::IndexSettingsMerge>,
    pub merge_scheduler_max_thread_count: Option<crate::types::common::StringifiedInteger>,
    pub mode: Option<String>,
    pub number_of_replicas: Option<crate::types::common::StringifiedInteger>,
    pub number_of_routing_shards: Option<crate::types::common::StringifiedInteger>,
    pub number_of_shards: Option<crate::types::common::StringifiedInteger>,
    pub priority: Option<crate::types::common::StringifiedInteger>,
    pub provided_name: Option<crate::types::common::Name>,
    pub queries: Option<crate::types::indices_common::IndexSettingsQueries>,
    pub query_string: Option<crate::types::indices_common::IndexSettingsQueryString>,
    pub query_string_lenient: Option<crate::types::common::StringifiedBoolean>,
    pub refresh_interval: Option<crate::types::common::Duration>,
    #[doc = "Defines the replication type."]
    pub replication_type: Option<crate::types::indices_common::ReplicationType>,
    pub routing: Option<crate::types::indices_common::IndexRouting>,
    pub routing_partition_size: Option<crate::types::common::StringifiedInteger>,
    pub routing_path: Option<crate::types::common::StringOrStringArray>,
    pub search: Option<crate::types::indices_common::IndexSettingsSearch>,
    pub search_idle_after: Option<crate::types::common::Duration>,
    pub settings: Option<Box<crate::types::indices_common::IndexSettings>>,
    pub similarity: Option<crate::types::indices_common::IndexSettingsSimilarity>,
    pub soft_deletes: Option<crate::types::indices_common::SoftDeletes>,
    pub soft_deletes_retention_lease_period: Option<crate::types::common::Duration>,
    pub sort: Option<crate::types::indices_common::IndexSegmentSort>,
    pub store: Option<crate::types::indices_common::IndexSettingsStore>,
    pub top_metrics_max_size: Option<crate::types::common::StringifiedInteger>,
    pub translog: Option<crate::types::indices_common::Translog>,
    pub translog_durability: Option<crate::types::indices_common::TranslogDurability>,
    pub translog_flush_threshold_size: Option<crate::types::common::HumanReadableByteCount>,
    pub uuid: Option<crate::types::common::Uuid>,
    pub verified_before_close: Option<crate::types::common::StringifiedBoolean>,
    pub version: Option<crate::types::indices_common::IndexVersioning>,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettings {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettings;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettings object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_analysis: Option<crate::types::indices_common::IndexSettingsAnalysis> =
                    None;
                let mut seen_analysis = false;
                let mut f_analyze: Option<crate::types::indices_common::IndexSettingsAnalyze> =
                    None;
                let mut seen_analyze = false;
                let mut f_analyze_max_token_count: Option<
                    crate::types::common::StringifiedInteger,
                > = None;
                let mut seen_analyze_max_token_count = false;
                let mut f_auto_expand_replicas: Option<String> = None;
                let mut seen_auto_expand_replicas = false;
                let mut f_blocks: Option<crate::types::indices_common::IndexSettingBlocks> = None;
                let mut seen_blocks = false;
                let mut f_blocks_metadata: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_blocks_metadata = false;
                let mut f_blocks_read: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_blocks_read = false;
                let mut f_blocks_read_only: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_blocks_read_only = false;
                let mut f_blocks_read_only_allow_delete: Option<
                    crate::types::common::StringifiedBoolean,
                > = None;
                let mut seen_blocks_read_only_allow_delete = false;
                let mut f_blocks_write: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_blocks_write = false;
                let mut f_check_on_startup: Option<
                    crate::types::indices_common::IndexCheckOnStartup,
                > = None;
                let mut seen_check_on_startup = false;
                let mut f_codec: Option<String> = None;
                let mut seen_codec = false;
                let mut f_composite_index_star_tree: Option<
                    crate::types::indices_common::IndexSettingsStarTree,
                > = None;
                let mut seen_composite_index_star_tree = false;
                let mut f_creation_date: Option<
                    crate::types::common::StringifiedEpochTimeUnitMillis,
                > = None;
                let mut seen_creation_date = false;
                let mut f_creation_date_string: Option<crate::types::common::DateTime> = None;
                let mut seen_creation_date_string = false;
                let mut f_default_pipeline: Option<crate::types::common::PipelineName> = None;
                let mut seen_default_pipeline = false;
                let mut f_final_pipeline: Option<crate::types::common::PipelineName> = None;
                let mut seen_final_pipeline = false;
                let mut f_format: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_format = false;
                let mut f_gc_deletes: Option<crate::types::common::Duration> = None;
                let mut seen_gc_deletes = false;
                let mut f_hidden: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_hidden = false;
                let mut f_highlight: Option<crate::types::indices_common::IndexSettingsHighlight> =
                    None;
                let mut seen_highlight = false;
                let mut f_highlight_max_analyzed_offset: Option<
                    crate::types::common::StringifiedInteger,
                > = None;
                let mut seen_highlight_max_analyzed_offset = false;
                let mut f_index: Option<Box<crate::types::indices_common::IndexSettings>> = None;
                let mut seen_index = false;
                let mut f_indexing: Option<crate::types::indices_common::IndexSettingsIndexing> =
                    None;
                let mut seen_indexing = false;
                let mut f_indexing_pressure: Option<
                    crate::types::indices_common::IndexingPressure,
                > = None;
                let mut seen_indexing_pressure = false;
                let mut f_ingestion_source: Option<crate::types::indices_common::IngestionSource> =
                    None;
                let mut seen_ingestion_source = false;
                let mut f_knn: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_knn = false;
                let mut f_knn_algo_param_ef_search: Option<
                    crate::types::common::StringifiedInteger,
                > = None;
                let mut seen_knn_algo_param_ef_search = false;
                let mut f_lifecycle: Option<crate::types::indices_common::IndexSettingsLifecycle> =
                    None;
                let mut seen_lifecycle = false;
                let mut f_lifecycle_name: Option<crate::types::common::Name> = None;
                let mut seen_lifecycle_name = false;
                let mut f_load_fixed_bitset_filters_eagerly: Option<
                    crate::types::common::StringifiedBoolean,
                > = None;
                let mut seen_load_fixed_bitset_filters_eagerly = false;
                let mut f_mapping: Option<crate::types::indices_common::IndexSettingsMapping> =
                    None;
                let mut seen_mapping = false;
                let mut f_max_docvalue_fields_search: Option<
                    crate::types::common::StringifiedInteger,
                > = None;
                let mut seen_max_docvalue_fields_search = false;
                let mut f_max_inner_result_window: Option<
                    crate::types::common::StringifiedInteger,
                > = None;
                let mut seen_max_inner_result_window = false;
                let mut f_max_ngram_diff: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_max_ngram_diff = false;
                let mut f_max_refresh_listeners: Option<crate::types::common::StringifiedInteger> =
                    None;
                let mut seen_max_refresh_listeners = false;
                let mut f_max_regex_length: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_max_regex_length = false;
                let mut f_max_rescore_window: Option<crate::types::common::StringifiedInteger> =
                    None;
                let mut seen_max_rescore_window = false;
                let mut f_max_result_window: Option<crate::types::common::StringifiedInteger> =
                    None;
                let mut seen_max_result_window = false;
                let mut f_max_script_fields: Option<crate::types::common::StringifiedInteger> =
                    None;
                let mut seen_max_script_fields = false;
                let mut f_max_shingle_diff: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_max_shingle_diff = false;
                let mut f_max_slices_per_scroll: Option<crate::types::common::StringifiedInteger> =
                    None;
                let mut seen_max_slices_per_scroll = false;
                let mut f_max_terms_count: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_max_terms_count = false;
                let mut f_merge: Option<crate::types::indices_common::IndexSettingsMerge> = None;
                let mut seen_merge = false;
                let mut f_merge_scheduler_max_thread_count: Option<
                    crate::types::common::StringifiedInteger,
                > = None;
                let mut seen_merge_scheduler_max_thread_count = false;
                let mut f_mode: Option<String> = None;
                let mut seen_mode = false;
                let mut f_number_of_replicas: Option<crate::types::common::StringifiedInteger> =
                    None;
                let mut seen_number_of_replicas = false;
                let mut f_number_of_routing_shards: Option<
                    crate::types::common::StringifiedInteger,
                > = None;
                let mut seen_number_of_routing_shards = false;
                let mut f_number_of_shards: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_number_of_shards = false;
                let mut f_priority: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_priority = false;
                let mut f_provided_name: Option<crate::types::common::Name> = None;
                let mut seen_provided_name = false;
                let mut f_queries: Option<crate::types::indices_common::IndexSettingsQueries> =
                    None;
                let mut seen_queries = false;
                let mut f_query_string: Option<
                    crate::types::indices_common::IndexSettingsQueryString,
                > = None;
                let mut seen_query_string = false;
                let mut f_query_string_lenient: Option<crate::types::common::StringifiedBoolean> =
                    None;
                let mut seen_query_string_lenient = false;
                let mut f_refresh_interval: Option<crate::types::common::Duration> = None;
                let mut seen_refresh_interval = false;
                let mut f_replication_type: Option<crate::types::indices_common::ReplicationType> =
                    None;
                let mut seen_replication_type = false;
                let mut f_routing: Option<crate::types::indices_common::IndexRouting> = None;
                let mut seen_routing = false;
                let mut f_routing_partition_size: Option<crate::types::common::StringifiedInteger> =
                    None;
                let mut seen_routing_partition_size = false;
                let mut f_routing_path: Option<crate::types::common::StringOrStringArray> = None;
                let mut seen_routing_path = false;
                let mut f_search: Option<crate::types::indices_common::IndexSettingsSearch> = None;
                let mut seen_search = false;
                let mut f_search_idle_after: Option<crate::types::common::Duration> = None;
                let mut seen_search_idle_after = false;
                let mut f_settings: Option<Box<crate::types::indices_common::IndexSettings>> = None;
                let mut seen_settings = false;
                let mut f_similarity: Option<
                    crate::types::indices_common::IndexSettingsSimilarity,
                > = None;
                let mut seen_similarity = false;
                let mut f_soft_deletes: Option<crate::types::indices_common::SoftDeletes> = None;
                let mut seen_soft_deletes = false;
                let mut f_soft_deletes_retention_lease_period: Option<
                    crate::types::common::Duration,
                > = None;
                let mut seen_soft_deletes_retention_lease_period = false;
                let mut f_sort: Option<crate::types::indices_common::IndexSegmentSort> = None;
                let mut seen_sort = false;
                let mut f_store: Option<crate::types::indices_common::IndexSettingsStore> = None;
                let mut seen_store = false;
                let mut f_top_metrics_max_size: Option<crate::types::common::StringifiedInteger> =
                    None;
                let mut seen_top_metrics_max_size = false;
                let mut f_translog: Option<crate::types::indices_common::Translog> = None;
                let mut seen_translog = false;
                let mut f_translog_durability: Option<
                    crate::types::indices_common::TranslogDurability,
                > = None;
                let mut seen_translog_durability = false;
                let mut f_translog_flush_threshold_size: Option<
                    crate::types::common::HumanReadableByteCount,
                > = None;
                let mut seen_translog_flush_threshold_size = false;
                let mut f_uuid: Option<crate::types::common::Uuid> = None;
                let mut seen_uuid = false;
                let mut f_verified_before_close: Option<crate::types::common::StringifiedBoolean> =
                    None;
                let mut seen_verified_before_close = false;
                let mut f_version: Option<crate::types::indices_common::IndexVersioning> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "analysis" => {
                            if seen_analysis {
                                return Err(serde::de::Error::duplicate_field("analysis"));
                            }
                            seen_analysis = true;
                            f_analysis = map.next_value::<Option<crate::types::indices_common::IndexSettingsAnalysis>>()?;
                        }
                        "analyze" => {
                            if seen_analyze {
                                return Err(serde::de::Error::duplicate_field("analyze"));
                            }
                            seen_analyze = true;
                            f_analyze = map.next_value::<Option<crate::types::indices_common::IndexSettingsAnalyze>>()?;
                        }
                        "analyze.max_token_count" => {
                            if seen_analyze_max_token_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "analyze.max_token_count",
                                ));
                            }
                            seen_analyze_max_token_count = true;
                            f_analyze_max_token_count = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "auto_expand_replicas" => {
                            if seen_auto_expand_replicas {
                                return Err(serde::de::Error::duplicate_field(
                                    "auto_expand_replicas",
                                ));
                            }
                            seen_auto_expand_replicas = true;
                            f_auto_expand_replicas = map.next_value::<Option<String>>()?;
                        }
                        "blocks" => {
                            if seen_blocks {
                                return Err(serde::de::Error::duplicate_field("blocks"));
                            }
                            seen_blocks = true;
                            f_blocks = map.next_value::<Option<crate::types::indices_common::IndexSettingBlocks>>()?;
                        }
                        "blocks.metadata" => {
                            if seen_blocks_metadata {
                                return Err(serde::de::Error::duplicate_field("blocks.metadata"));
                            }
                            seen_blocks_metadata = true;
                            f_blocks_metadata = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "blocks.read" => {
                            if seen_blocks_read {
                                return Err(serde::de::Error::duplicate_field("blocks.read"));
                            }
                            seen_blocks_read = true;
                            f_blocks_read = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "blocks.read_only" => {
                            if seen_blocks_read_only {
                                return Err(serde::de::Error::duplicate_field("blocks.read_only"));
                            }
                            seen_blocks_read_only = true;
                            f_blocks_read_only = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "blocks.read_only_allow_delete" => {
                            if seen_blocks_read_only_allow_delete {
                                return Err(serde::de::Error::duplicate_field(
                                    "blocks.read_only_allow_delete",
                                ));
                            }
                            seen_blocks_read_only_allow_delete = true;
                            f_blocks_read_only_allow_delete = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "blocks.write" => {
                            if seen_blocks_write {
                                return Err(serde::de::Error::duplicate_field("blocks.write"));
                            }
                            seen_blocks_write = true;
                            f_blocks_write = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "check_on_startup" => {
                            if seen_check_on_startup {
                                return Err(serde::de::Error::duplicate_field("check_on_startup"));
                            }
                            seen_check_on_startup = true;
                            f_check_on_startup = map.next_value::<Option<crate::types::indices_common::IndexCheckOnStartup>>()?;
                        }
                        "codec" => {
                            if seen_codec {
                                return Err(serde::de::Error::duplicate_field("codec"));
                            }
                            seen_codec = true;
                            f_codec = map.next_value::<Option<String>>()?;
                        }
                        "composite_index.star_tree" => {
                            if seen_composite_index_star_tree {
                                return Err(serde::de::Error::duplicate_field(
                                    "composite_index.star_tree",
                                ));
                            }
                            seen_composite_index_star_tree = true;
                            f_composite_index_star_tree = map.next_value::<Option<crate::types::indices_common::IndexSettingsStarTree>>()?;
                        }
                        "creation_date" => {
                            if seen_creation_date {
                                return Err(serde::de::Error::duplicate_field("creation_date"));
                            }
                            seen_creation_date = true;
                            f_creation_date = map.next_value::<Option<crate::types::common::StringifiedEpochTimeUnitMillis>>()?;
                        }
                        "creation_date_string" => {
                            if seen_creation_date_string {
                                return Err(serde::de::Error::duplicate_field(
                                    "creation_date_string",
                                ));
                            }
                            seen_creation_date_string = true;
                            f_creation_date_string =
                                map.next_value::<Option<crate::types::common::DateTime>>()?;
                        }
                        "default_pipeline" => {
                            if seen_default_pipeline {
                                return Err(serde::de::Error::duplicate_field("default_pipeline"));
                            }
                            seen_default_pipeline = true;
                            f_default_pipeline =
                                map.next_value::<Option<crate::types::common::PipelineName>>()?;
                        }
                        "final_pipeline" => {
                            if seen_final_pipeline {
                                return Err(serde::de::Error::duplicate_field("final_pipeline"));
                            }
                            seen_final_pipeline = true;
                            f_final_pipeline =
                                map.next_value::<Option<crate::types::common::PipelineName>>()?;
                        }
                        "format" => {
                            if seen_format {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            seen_format = true;
                            f_format = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "gc_deletes" => {
                            if seen_gc_deletes {
                                return Err(serde::de::Error::duplicate_field("gc_deletes"));
                            }
                            seen_gc_deletes = true;
                            f_gc_deletes =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "hidden" => {
                            if seen_hidden {
                                return Err(serde::de::Error::duplicate_field("hidden"));
                            }
                            seen_hidden = true;
                            f_hidden = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "highlight" => {
                            if seen_highlight {
                                return Err(serde::de::Error::duplicate_field("highlight"));
                            }
                            seen_highlight = true;
                            f_highlight = map.next_value::<Option<crate::types::indices_common::IndexSettingsHighlight>>()?;
                        }
                        "highlight.max_analyzed_offset" => {
                            if seen_highlight_max_analyzed_offset {
                                return Err(serde::de::Error::duplicate_field(
                                    "highlight.max_analyzed_offset",
                                ));
                            }
                            seen_highlight_max_analyzed_offset = true;
                            f_highlight_max_analyzed_offset = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<Box<crate::types::indices_common::IndexSettings>>>()?;
                        }
                        "indexing" => {
                            if seen_indexing {
                                return Err(serde::de::Error::duplicate_field("indexing"));
                            }
                            seen_indexing = true;
                            f_indexing = map.next_value::<Option<crate::types::indices_common::IndexSettingsIndexing>>()?;
                        }
                        "indexing_pressure" => {
                            if seen_indexing_pressure {
                                return Err(serde::de::Error::duplicate_field("indexing_pressure"));
                            }
                            seen_indexing_pressure = true;
                            f_indexing_pressure = map.next_value::<Option<crate::types::indices_common::IndexingPressure>>()?;
                        }
                        "ingestion_source" => {
                            if seen_ingestion_source {
                                return Err(serde::de::Error::duplicate_field("ingestion_source"));
                            }
                            seen_ingestion_source = true;
                            f_ingestion_source = map.next_value::<Option<crate::types::indices_common::IngestionSource>>()?;
                        }
                        "knn" => {
                            if seen_knn {
                                return Err(serde::de::Error::duplicate_field("knn"));
                            }
                            seen_knn = true;
                            f_knn = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "knn.algo_param.ef_search" => {
                            if seen_knn_algo_param_ef_search {
                                return Err(serde::de::Error::duplicate_field(
                                    "knn.algo_param.ef_search",
                                ));
                            }
                            seen_knn_algo_param_ef_search = true;
                            f_knn_algo_param_ef_search = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "lifecycle" => {
                            if seen_lifecycle {
                                return Err(serde::de::Error::duplicate_field("lifecycle"));
                            }
                            seen_lifecycle = true;
                            f_lifecycle = map.next_value::<Option<crate::types::indices_common::IndexSettingsLifecycle>>()?;
                        }
                        "lifecycle.name" => {
                            if seen_lifecycle_name {
                                return Err(serde::de::Error::duplicate_field("lifecycle.name"));
                            }
                            seen_lifecycle_name = true;
                            f_lifecycle_name =
                                map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "load_fixed_bitset_filters_eagerly" => {
                            if seen_load_fixed_bitset_filters_eagerly {
                                return Err(serde::de::Error::duplicate_field(
                                    "load_fixed_bitset_filters_eagerly",
                                ));
                            }
                            seen_load_fixed_bitset_filters_eagerly = true;
                            f_load_fixed_bitset_filters_eagerly = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "mapping" => {
                            if seen_mapping {
                                return Err(serde::de::Error::duplicate_field("mapping"));
                            }
                            seen_mapping = true;
                            f_mapping = map.next_value::<Option<crate::types::indices_common::IndexSettingsMapping>>()?;
                        }
                        "max_docvalue_fields_search" => {
                            if seen_max_docvalue_fields_search {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_docvalue_fields_search",
                                ));
                            }
                            seen_max_docvalue_fields_search = true;
                            f_max_docvalue_fields_search = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_inner_result_window" => {
                            if seen_max_inner_result_window {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_inner_result_window",
                                ));
                            }
                            seen_max_inner_result_window = true;
                            f_max_inner_result_window = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_ngram_diff" => {
                            if seen_max_ngram_diff {
                                return Err(serde::de::Error::duplicate_field("max_ngram_diff"));
                            }
                            seen_max_ngram_diff = true;
                            f_max_ngram_diff = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_refresh_listeners" => {
                            if seen_max_refresh_listeners {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_refresh_listeners",
                                ));
                            }
                            seen_max_refresh_listeners = true;
                            f_max_refresh_listeners = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_regex_length" => {
                            if seen_max_regex_length {
                                return Err(serde::de::Error::duplicate_field("max_regex_length"));
                            }
                            seen_max_regex_length = true;
                            f_max_regex_length = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_rescore_window" => {
                            if seen_max_rescore_window {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_rescore_window",
                                ));
                            }
                            seen_max_rescore_window = true;
                            f_max_rescore_window = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_result_window" => {
                            if seen_max_result_window {
                                return Err(serde::de::Error::duplicate_field("max_result_window"));
                            }
                            seen_max_result_window = true;
                            f_max_result_window = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_script_fields" => {
                            if seen_max_script_fields {
                                return Err(serde::de::Error::duplicate_field("max_script_fields"));
                            }
                            seen_max_script_fields = true;
                            f_max_script_fields = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_shingle_diff" => {
                            if seen_max_shingle_diff {
                                return Err(serde::de::Error::duplicate_field("max_shingle_diff"));
                            }
                            seen_max_shingle_diff = true;
                            f_max_shingle_diff = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_slices_per_scroll" => {
                            if seen_max_slices_per_scroll {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_slices_per_scroll",
                                ));
                            }
                            seen_max_slices_per_scroll = true;
                            f_max_slices_per_scroll = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_terms_count" => {
                            if seen_max_terms_count {
                                return Err(serde::de::Error::duplicate_field("max_terms_count"));
                            }
                            seen_max_terms_count = true;
                            f_max_terms_count = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "merge" => {
                            if seen_merge {
                                return Err(serde::de::Error::duplicate_field("merge"));
                            }
                            seen_merge = true;
                            f_merge = map.next_value::<Option<crate::types::indices_common::IndexSettingsMerge>>()?;
                        }
                        "merge.scheduler.max_thread_count" => {
                            if seen_merge_scheduler_max_thread_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "merge.scheduler.max_thread_count",
                                ));
                            }
                            seen_merge_scheduler_max_thread_count = true;
                            f_merge_scheduler_max_thread_count = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "mode" => {
                            if seen_mode {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            seen_mode = true;
                            f_mode = map.next_value::<Option<String>>()?;
                        }
                        "number_of_replicas" => {
                            if seen_number_of_replicas {
                                return Err(serde::de::Error::duplicate_field(
                                    "number_of_replicas",
                                ));
                            }
                            seen_number_of_replicas = true;
                            f_number_of_replicas = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "number_of_routing_shards" => {
                            if seen_number_of_routing_shards {
                                return Err(serde::de::Error::duplicate_field(
                                    "number_of_routing_shards",
                                ));
                            }
                            seen_number_of_routing_shards = true;
                            f_number_of_routing_shards = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "number_of_shards" => {
                            if seen_number_of_shards {
                                return Err(serde::de::Error::duplicate_field("number_of_shards"));
                            }
                            seen_number_of_shards = true;
                            f_number_of_shards = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "priority" => {
                            if seen_priority {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            seen_priority = true;
                            f_priority = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "provided_name" => {
                            if seen_provided_name {
                                return Err(serde::de::Error::duplicate_field("provided_name"));
                            }
                            seen_provided_name = true;
                            f_provided_name =
                                map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "queries" => {
                            if seen_queries {
                                return Err(serde::de::Error::duplicate_field("queries"));
                            }
                            seen_queries = true;
                            f_queries = map.next_value::<Option<crate::types::indices_common::IndexSettingsQueries>>()?;
                        }
                        "query_string" => {
                            if seen_query_string {
                                return Err(serde::de::Error::duplicate_field("query_string"));
                            }
                            seen_query_string = true;
                            f_query_string = map.next_value::<Option<crate::types::indices_common::IndexSettingsQueryString>>()?;
                        }
                        "query_string.lenient" => {
                            if seen_query_string_lenient {
                                return Err(serde::de::Error::duplicate_field(
                                    "query_string.lenient",
                                ));
                            }
                            seen_query_string_lenient = true;
                            f_query_string_lenient = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "refresh_interval" => {
                            if seen_refresh_interval {
                                return Err(serde::de::Error::duplicate_field("refresh_interval"));
                            }
                            seen_refresh_interval = true;
                            f_refresh_interval =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "replication.type" => {
                            if seen_replication_type {
                                return Err(serde::de::Error::duplicate_field("replication.type"));
                            }
                            seen_replication_type = true;
                            f_replication_type = map.next_value::<Option<crate::types::indices_common::ReplicationType>>()?;
                        }
                        "routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("routing"));
                            }
                            seen_routing = true;
                            f_routing = map
                                .next_value::<Option<crate::types::indices_common::IndexRouting>>(
                                )?;
                        }
                        "routing_partition_size" => {
                            if seen_routing_partition_size {
                                return Err(serde::de::Error::duplicate_field(
                                    "routing_partition_size",
                                ));
                            }
                            seen_routing_partition_size = true;
                            f_routing_partition_size = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "routing_path" => {
                            if seen_routing_path {
                                return Err(serde::de::Error::duplicate_field("routing_path"));
                            }
                            seen_routing_path = true;
                            f_routing_path = map
                                .next_value::<Option<crate::types::common::StringOrStringArray>>(
                                )?;
                        }
                        "search" => {
                            if seen_search {
                                return Err(serde::de::Error::duplicate_field("search"));
                            }
                            seen_search = true;
                            f_search = map.next_value::<Option<crate::types::indices_common::IndexSettingsSearch>>()?;
                        }
                        "search.idle.after" => {
                            if seen_search_idle_after {
                                return Err(serde::de::Error::duplicate_field("search.idle.after"));
                            }
                            seen_search_idle_after = true;
                            f_search_idle_after =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "settings" => {
                            if seen_settings {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            seen_settings = true;
                            f_settings = map.next_value::<Option<Box<crate::types::indices_common::IndexSettings>>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<crate::types::indices_common::IndexSettingsSimilarity>>()?;
                        }
                        "soft_deletes" => {
                            if seen_soft_deletes {
                                return Err(serde::de::Error::duplicate_field("soft_deletes"));
                            }
                            seen_soft_deletes = true;
                            f_soft_deletes = map
                                .next_value::<Option<crate::types::indices_common::SoftDeletes>>(
                                )?;
                        }
                        "soft_deletes.retention_lease.period" => {
                            if seen_soft_deletes_retention_lease_period {
                                return Err(serde::de::Error::duplicate_field(
                                    "soft_deletes.retention_lease.period",
                                ));
                            }
                            seen_soft_deletes_retention_lease_period = true;
                            f_soft_deletes_retention_lease_period =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "sort" => {
                            if seen_sort {
                                return Err(serde::de::Error::duplicate_field("sort"));
                            }
                            seen_sort = true;
                            f_sort = map.next_value::<Option<crate::types::indices_common::IndexSegmentSort>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<crate::types::indices_common::IndexSettingsStore>>()?;
                        }
                        "top_metrics_max_size" => {
                            if seen_top_metrics_max_size {
                                return Err(serde::de::Error::duplicate_field(
                                    "top_metrics_max_size",
                                ));
                            }
                            seen_top_metrics_max_size = true;
                            f_top_metrics_max_size = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "translog" => {
                            if seen_translog {
                                return Err(serde::de::Error::duplicate_field("translog"));
                            }
                            seen_translog = true;
                            f_translog =
                                map.next_value::<Option<crate::types::indices_common::Translog>>()?;
                        }
                        "translog.durability" => {
                            if seen_translog_durability {
                                return Err(serde::de::Error::duplicate_field(
                                    "translog.durability",
                                ));
                            }
                            seen_translog_durability = true;
                            f_translog_durability = map.next_value::<Option<crate::types::indices_common::TranslogDurability>>()?;
                        }
                        "translog.flush_threshold_size" => {
                            if seen_translog_flush_threshold_size {
                                return Err(serde::de::Error::duplicate_field(
                                    "translog.flush_threshold_size",
                                ));
                            }
                            seen_translog_flush_threshold_size = true;
                            f_translog_flush_threshold_size = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "uuid" => {
                            if seen_uuid {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            seen_uuid = true;
                            f_uuid = map.next_value::<Option<crate::types::common::Uuid>>()?;
                        }
                        "verified_before_close" => {
                            if seen_verified_before_close {
                                return Err(serde::de::Error::duplicate_field(
                                    "verified_before_close",
                                ));
                            }
                            seen_verified_before_close = true;
                            f_verified_before_close = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version = map.next_value::<Option<crate::types::indices_common::IndexVersioning>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettings {
                    analysis: f_analysis,
                    analyze: f_analyze,
                    analyze_max_token_count: f_analyze_max_token_count,
                    auto_expand_replicas: f_auto_expand_replicas,
                    blocks: f_blocks,
                    blocks_metadata: f_blocks_metadata,
                    blocks_read: f_blocks_read,
                    blocks_read_only: f_blocks_read_only,
                    blocks_read_only_allow_delete: f_blocks_read_only_allow_delete,
                    blocks_write: f_blocks_write,
                    check_on_startup: f_check_on_startup,
                    codec: f_codec,
                    composite_index_star_tree: f_composite_index_star_tree,
                    creation_date: f_creation_date,
                    creation_date_string: f_creation_date_string,
                    default_pipeline: f_default_pipeline,
                    final_pipeline: f_final_pipeline,
                    format: f_format,
                    gc_deletes: f_gc_deletes,
                    hidden: f_hidden,
                    highlight: f_highlight,
                    highlight_max_analyzed_offset: f_highlight_max_analyzed_offset,
                    index: f_index,
                    indexing: f_indexing,
                    indexing_pressure: f_indexing_pressure,
                    ingestion_source: f_ingestion_source,
                    knn: f_knn,
                    knn_algo_param_ef_search: f_knn_algo_param_ef_search,
                    lifecycle: f_lifecycle,
                    lifecycle_name: f_lifecycle_name,
                    load_fixed_bitset_filters_eagerly: f_load_fixed_bitset_filters_eagerly,
                    mapping: f_mapping,
                    max_docvalue_fields_search: f_max_docvalue_fields_search,
                    max_inner_result_window: f_max_inner_result_window,
                    max_ngram_diff: f_max_ngram_diff,
                    max_refresh_listeners: f_max_refresh_listeners,
                    max_regex_length: f_max_regex_length,
                    max_rescore_window: f_max_rescore_window,
                    max_result_window: f_max_result_window,
                    max_script_fields: f_max_script_fields,
                    max_shingle_diff: f_max_shingle_diff,
                    max_slices_per_scroll: f_max_slices_per_scroll,
                    max_terms_count: f_max_terms_count,
                    merge: f_merge,
                    merge_scheduler_max_thread_count: f_merge_scheduler_max_thread_count,
                    mode: f_mode,
                    number_of_replicas: f_number_of_replicas,
                    number_of_routing_shards: f_number_of_routing_shards,
                    number_of_shards: f_number_of_shards,
                    priority: f_priority,
                    provided_name: f_provided_name,
                    queries: f_queries,
                    query_string: f_query_string,
                    query_string_lenient: f_query_string_lenient,
                    refresh_interval: f_refresh_interval,
                    replication_type: f_replication_type,
                    routing: f_routing,
                    routing_partition_size: f_routing_partition_size,
                    routing_path: f_routing_path,
                    search: f_search,
                    search_idle_after: f_search_idle_after,
                    settings: f_settings,
                    similarity: f_similarity,
                    soft_deletes: f_soft_deletes,
                    soft_deletes_retention_lease_period: f_soft_deletes_retention_lease_period,
                    sort: f_sort,
                    store: f_store,
                    top_metrics_max_size: f_top_metrics_max_size,
                    translog: f_translog,
                    translog_durability: f_translog_durability,
                    translog_flush_threshold_size: f_translog_flush_threshold_size,
                    uuid: f_uuid,
                    verified_before_close: f_verified_before_close,
                    version: f_version,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettings {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.analysis {
            map.serialize_entry("analysis", v)?;
        }
        if let Some(v) = &self.analyze {
            map.serialize_entry("analyze", v)?;
        }
        if let Some(v) = &self.analyze_max_token_count {
            map.serialize_entry("analyze.max_token_count", v)?;
        }
        if let Some(v) = &self.auto_expand_replicas {
            map.serialize_entry("auto_expand_replicas", v)?;
        }
        if let Some(v) = &self.blocks {
            map.serialize_entry("blocks", v)?;
        }
        if let Some(v) = &self.blocks_metadata {
            map.serialize_entry("blocks.metadata", v)?;
        }
        if let Some(v) = &self.blocks_read {
            map.serialize_entry("blocks.read", v)?;
        }
        if let Some(v) = &self.blocks_read_only {
            map.serialize_entry("blocks.read_only", v)?;
        }
        if let Some(v) = &self.blocks_read_only_allow_delete {
            map.serialize_entry("blocks.read_only_allow_delete", v)?;
        }
        if let Some(v) = &self.blocks_write {
            map.serialize_entry("blocks.write", v)?;
        }
        if let Some(v) = &self.check_on_startup {
            map.serialize_entry("check_on_startup", v)?;
        }
        if let Some(v) = &self.codec {
            map.serialize_entry("codec", v)?;
        }
        if let Some(v) = &self.composite_index_star_tree {
            map.serialize_entry("composite_index.star_tree", v)?;
        }
        if let Some(v) = &self.creation_date {
            map.serialize_entry("creation_date", v)?;
        }
        if let Some(v) = &self.creation_date_string {
            map.serialize_entry("creation_date_string", v)?;
        }
        if let Some(v) = &self.default_pipeline {
            map.serialize_entry("default_pipeline", v)?;
        }
        if let Some(v) = &self.final_pipeline {
            map.serialize_entry("final_pipeline", v)?;
        }
        if let Some(v) = &self.format {
            map.serialize_entry("format", v)?;
        }
        if let Some(v) = &self.gc_deletes {
            map.serialize_entry("gc_deletes", v)?;
        }
        if let Some(v) = &self.hidden {
            map.serialize_entry("hidden", v)?;
        }
        if let Some(v) = &self.highlight {
            map.serialize_entry("highlight", v)?;
        }
        if let Some(v) = &self.highlight_max_analyzed_offset {
            map.serialize_entry("highlight.max_analyzed_offset", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.indexing {
            map.serialize_entry("indexing", v)?;
        }
        if let Some(v) = &self.indexing_pressure {
            map.serialize_entry("indexing_pressure", v)?;
        }
        if let Some(v) = &self.ingestion_source {
            map.serialize_entry("ingestion_source", v)?;
        }
        if let Some(v) = &self.knn {
            map.serialize_entry("knn", v)?;
        }
        if let Some(v) = &self.knn_algo_param_ef_search {
            map.serialize_entry("knn.algo_param.ef_search", v)?;
        }
        if let Some(v) = &self.lifecycle {
            map.serialize_entry("lifecycle", v)?;
        }
        if let Some(v) = &self.lifecycle_name {
            map.serialize_entry("lifecycle.name", v)?;
        }
        if let Some(v) = &self.load_fixed_bitset_filters_eagerly {
            map.serialize_entry("load_fixed_bitset_filters_eagerly", v)?;
        }
        if let Some(v) = &self.mapping {
            map.serialize_entry("mapping", v)?;
        }
        if let Some(v) = &self.max_docvalue_fields_search {
            map.serialize_entry("max_docvalue_fields_search", v)?;
        }
        if let Some(v) = &self.max_inner_result_window {
            map.serialize_entry("max_inner_result_window", v)?;
        }
        if let Some(v) = &self.max_ngram_diff {
            map.serialize_entry("max_ngram_diff", v)?;
        }
        if let Some(v) = &self.max_refresh_listeners {
            map.serialize_entry("max_refresh_listeners", v)?;
        }
        if let Some(v) = &self.max_regex_length {
            map.serialize_entry("max_regex_length", v)?;
        }
        if let Some(v) = &self.max_rescore_window {
            map.serialize_entry("max_rescore_window", v)?;
        }
        if let Some(v) = &self.max_result_window {
            map.serialize_entry("max_result_window", v)?;
        }
        if let Some(v) = &self.max_script_fields {
            map.serialize_entry("max_script_fields", v)?;
        }
        if let Some(v) = &self.max_shingle_diff {
            map.serialize_entry("max_shingle_diff", v)?;
        }
        if let Some(v) = &self.max_slices_per_scroll {
            map.serialize_entry("max_slices_per_scroll", v)?;
        }
        if let Some(v) = &self.max_terms_count {
            map.serialize_entry("max_terms_count", v)?;
        }
        if let Some(v) = &self.merge {
            map.serialize_entry("merge", v)?;
        }
        if let Some(v) = &self.merge_scheduler_max_thread_count {
            map.serialize_entry("merge.scheduler.max_thread_count", v)?;
        }
        if let Some(v) = &self.mode {
            map.serialize_entry("mode", v)?;
        }
        if let Some(v) = &self.number_of_replicas {
            map.serialize_entry("number_of_replicas", v)?;
        }
        if let Some(v) = &self.number_of_routing_shards {
            map.serialize_entry("number_of_routing_shards", v)?;
        }
        if let Some(v) = &self.number_of_shards {
            map.serialize_entry("number_of_shards", v)?;
        }
        if let Some(v) = &self.priority {
            map.serialize_entry("priority", v)?;
        }
        if let Some(v) = &self.provided_name {
            map.serialize_entry("provided_name", v)?;
        }
        if let Some(v) = &self.queries {
            map.serialize_entry("queries", v)?;
        }
        if let Some(v) = &self.query_string {
            map.serialize_entry("query_string", v)?;
        }
        if let Some(v) = &self.query_string_lenient {
            map.serialize_entry("query_string.lenient", v)?;
        }
        if let Some(v) = &self.refresh_interval {
            map.serialize_entry("refresh_interval", v)?;
        }
        if let Some(v) = &self.replication_type {
            map.serialize_entry("replication.type", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("routing", v)?;
        }
        if let Some(v) = &self.routing_partition_size {
            map.serialize_entry("routing_partition_size", v)?;
        }
        if let Some(v) = &self.routing_path {
            map.serialize_entry("routing_path", v)?;
        }
        if let Some(v) = &self.search {
            map.serialize_entry("search", v)?;
        }
        if let Some(v) = &self.search_idle_after {
            map.serialize_entry("search.idle.after", v)?;
        }
        if let Some(v) = &self.settings {
            map.serialize_entry("settings", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.soft_deletes {
            map.serialize_entry("soft_deletes", v)?;
        }
        if let Some(v) = &self.soft_deletes_retention_lease_period {
            map.serialize_entry("soft_deletes.retention_lease.period", v)?;
        }
        if let Some(v) = &self.sort {
            map.serialize_entry("sort", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.top_metrics_max_size {
            map.serialize_entry("top_metrics_max_size", v)?;
        }
        if let Some(v) = &self.translog {
            map.serialize_entry("translog", v)?;
        }
        if let Some(v) = &self.translog_durability {
            map.serialize_entry("translog.durability", v)?;
        }
        if let Some(v) = &self.translog_flush_threshold_size {
            map.serialize_entry("translog.flush_threshold_size", v)?;
        }
        if let Some(v) = &self.uuid {
            map.serialize_entry("uuid", v)?;
        }
        if let Some(v) = &self.verified_before_close {
            map.serialize_entry("verified_before_close", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The text analysis configuration."]
#[derive(Debug, Clone)]
pub struct IndexSettingsAnalysis {
    #[doc = "The custom analyzer configurations."]
    pub analyzer:
        Option<std::collections::BTreeMap<String, crate::types::common_analysis::Analyzer>>,
    #[doc = "The custom character filter configurations."]
    pub char_filter:
        Option<std::collections::BTreeMap<String, crate::types::common_analysis::CharFilter>>,
    #[doc = "The custom token filter configurations."]
    pub filter:
        Option<std::collections::BTreeMap<String, crate::types::common_analysis::TokenFilter>>,
    #[doc = "The custom normalizer configurations."]
    pub normalizer:
        Option<std::collections::BTreeMap<String, crate::types::common_analysis::Normalizer>>,
    #[doc = "The custom tokenizer configurations."]
    pub tokenizer:
        Option<std::collections::BTreeMap<String, crate::types::common_analysis::Tokenizer>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsAnalysis {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsAnalysis;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsAnalysis object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_analyzer: Option<
                    std::collections::BTreeMap<String, crate::types::common_analysis::Analyzer>,
                > = None;
                let mut seen_analyzer = false;
                let mut f_char_filter: Option<
                    std::collections::BTreeMap<String, crate::types::common_analysis::CharFilter>,
                > = None;
                let mut seen_char_filter = false;
                let mut f_filter: Option<
                    std::collections::BTreeMap<String, crate::types::common_analysis::TokenFilter>,
                > = None;
                let mut seen_filter = false;
                let mut f_normalizer: Option<
                    std::collections::BTreeMap<String, crate::types::common_analysis::Normalizer>,
                > = None;
                let mut seen_normalizer = false;
                let mut f_tokenizer: Option<
                    std::collections::BTreeMap<String, crate::types::common_analysis::Tokenizer>,
                > = None;
                let mut seen_tokenizer = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "analyzer" => {
                            if seen_analyzer {
                                return Err(serde::de::Error::duplicate_field("analyzer"));
                            }
                            seen_analyzer = true;
                            f_analyzer = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_analysis::Analyzer,
                                >,
                            >>()?;
                        }
                        "char_filter" => {
                            if seen_char_filter {
                                return Err(serde::de::Error::duplicate_field("char_filter"));
                            }
                            seen_char_filter = true;
                            f_char_filter = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_analysis::CharFilter,
                                >,
                            >>()?;
                        }
                        "filter" => {
                            if seen_filter {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            seen_filter = true;
                            f_filter = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_analysis::TokenFilter,
                                >,
                            >>()?;
                        }
                        "normalizer" => {
                            if seen_normalizer {
                                return Err(serde::de::Error::duplicate_field("normalizer"));
                            }
                            seen_normalizer = true;
                            f_normalizer = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_analysis::Normalizer,
                                >,
                            >>()?;
                        }
                        "tokenizer" => {
                            if seen_tokenizer {
                                return Err(serde::de::Error::duplicate_field("tokenizer"));
                            }
                            seen_tokenizer = true;
                            f_tokenizer = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_analysis::Tokenizer,
                                >,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsAnalysis {
                    analyzer: f_analyzer,
                    char_filter: f_char_filter,
                    filter: f_filter,
                    normalizer: f_normalizer,
                    tokenizer: f_tokenizer,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsAnalysis {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.analyzer {
            map.serialize_entry("analyzer", v)?;
        }
        if let Some(v) = &self.char_filter {
            map.serialize_entry("char_filter", v)?;
        }
        if let Some(v) = &self.filter {
            map.serialize_entry("filter", v)?;
        }
        if let Some(v) = &self.normalizer {
            map.serialize_entry("normalizer", v)?;
        }
        if let Some(v) = &self.tokenizer {
            map.serialize_entry("tokenizer", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for text analysis."]
#[derive(Debug, Clone)]
pub struct IndexSettingsAnalyze {
    pub max_token_count: Option<crate::types::common::StringifiedInteger>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsAnalyze {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsAnalyze;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsAnalyze object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_max_token_count: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_max_token_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "max_token_count" => {
                            if seen_max_token_count {
                                return Err(serde::de::Error::duplicate_field("max_token_count"));
                            }
                            seen_max_token_count = true;
                            f_max_token_count = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsAnalyze {
                    max_token_count: f_max_token_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsAnalyze {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.max_token_count {
            map.serialize_entry("max_token_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for highlighting."]
#[derive(Debug, Clone)]
pub struct IndexSettingsHighlight {
    pub max_analyzed_offset: Option<crate::types::common::StringifiedInteger>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsHighlight {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsHighlight;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsHighlight object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_max_analyzed_offset: Option<crate::types::common::StringifiedInteger> =
                    None;
                let mut seen_max_analyzed_offset = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "max_analyzed_offset" => {
                            if seen_max_analyzed_offset {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_analyzed_offset",
                                ));
                            }
                            seen_max_analyzed_offset = true;
                            f_max_analyzed_offset = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsHighlight {
                    max_analyzed_offset: f_max_analyzed_offset,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsHighlight {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.max_analyzed_offset {
            map.serialize_entry("max_analyzed_offset", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The indexing-related settings for an index."]
#[derive(Debug, Clone)]
pub struct IndexSettingsIndexing {
    pub slowlog: Option<crate::types::indices_common::IndexingSlowlog>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsIndexing {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsIndexing;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsIndexing object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_slowlog: Option<crate::types::indices_common::IndexingSlowlog> = None;
                let mut seen_slowlog = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "slowlog" => {
                            if seen_slowlog {
                                return Err(serde::de::Error::duplicate_field("slowlog"));
                            }
                            seen_slowlog = true;
                            f_slowlog = map.next_value::<Option<crate::types::indices_common::IndexingSlowlog>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsIndexing {
                    slowlog: f_slowlog,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsIndexing {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.slowlog {
            map.serialize_entry("slowlog", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The index lifecycle management configuration."]
#[derive(Debug, Clone)]
pub struct IndexSettingsLifecycle {
    pub indexing_complete: Option<crate::types::common::StringifiedBoolean>,
    pub name: crate::types::common::Name,
    #[doc = "The timestamp used to calculate the index age for its phase transitions. Use this setting if you create a new index that contains old data and want to use the original creation date to calculate the index age. Specified as a Unix epoch value in milliseconds."]
    pub origination_date: Option<crate::types::common::StringifiedEpochTimeUnitMillis>,
    #[doc = "When `true`, parses the origination date from the index name. This origination date is used to calculate the index age for its phase transitions. The index name must match the pattern `^.*-{date_format}-\\\\d+`, where the `date_format` is `yyyy.MM.dd` and the trailing digits are optional. An index that was rolled over would normally match the full format, for example `logs-2016.10.31-000002`). If the index name doesn't match the pattern, index creation fails."]
    pub parse_origination_date: Option<bool>,
    #[doc = "The index alias to update when the index rolls over. Specify when using a policy that contains a rollover action. When the index rolls over, the alias is updated to reflect that the index is no longer the write index. For more information about rolling indexes, see Rollover."]
    pub rollover_alias: Option<String>,
    pub step: Option<crate::types::indices_common::IndexSettingsLifecycleStep>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsLifecycle {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsLifecycle;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsLifecycle object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_indexing_complete: Option<crate::types::common::StringifiedBoolean> =
                    None;
                let mut seen_indexing_complete = false;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut f_origination_date: Option<
                    crate::types::common::StringifiedEpochTimeUnitMillis,
                > = None;
                let mut seen_origination_date = false;
                let mut f_parse_origination_date: Option<bool> = None;
                let mut seen_parse_origination_date = false;
                let mut f_rollover_alias: Option<String> = None;
                let mut seen_rollover_alias = false;
                let mut f_step: Option<crate::types::indices_common::IndexSettingsLifecycleStep> =
                    None;
                let mut seen_step = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "indexing_complete" => {
                            if seen_indexing_complete {
                                return Err(serde::de::Error::duplicate_field("indexing_complete"));
                            }
                            seen_indexing_complete = true;
                            f_indexing_complete = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        "origination_date" => {
                            if seen_origination_date {
                                return Err(serde::de::Error::duplicate_field("origination_date"));
                            }
                            seen_origination_date = true;
                            f_origination_date = map.next_value::<Option<crate::types::common::StringifiedEpochTimeUnitMillis>>()?;
                        }
                        "parse_origination_date" => {
                            if seen_parse_origination_date {
                                return Err(serde::de::Error::duplicate_field(
                                    "parse_origination_date",
                                ));
                            }
                            seen_parse_origination_date = true;
                            f_parse_origination_date = map.next_value::<Option<bool>>()?;
                        }
                        "rollover_alias" => {
                            if seen_rollover_alias {
                                return Err(serde::de::Error::duplicate_field("rollover_alias"));
                            }
                            seen_rollover_alias = true;
                            f_rollover_alias = map.next_value::<Option<String>>()?;
                        }
                        "step" => {
                            if seen_step {
                                return Err(serde::de::Error::duplicate_field("step"));
                            }
                            seen_step = true;
                            f_step =
                                map.next_value::<Option<
                                    crate::types::indices_common::IndexSettingsLifecycleStep,
                                >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsLifecycle {
                    indexing_complete: f_indexing_complete,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    origination_date: f_origination_date,
                    parse_origination_date: f_parse_origination_date,
                    rollover_alias: f_rollover_alias,
                    step: f_step,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsLifecycle {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.indexing_complete {
            map.serialize_entry("indexing_complete", v)?;
        }
        map.serialize_entry("name", &self.name)?;
        if let Some(v) = &self.origination_date {
            map.serialize_entry("origination_date", v)?;
        }
        if let Some(v) = &self.parse_origination_date {
            map.serialize_entry("parse_origination_date", v)?;
        }
        if let Some(v) = &self.rollover_alias {
            map.serialize_entry("rollover_alias", v)?;
        }
        if let Some(v) = &self.step {
            map.serialize_entry("step", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for the lifecycle step execution."]
#[derive(Debug, Clone)]
pub struct IndexSettingsLifecycleStep {
    pub wait_time_threshold: Option<crate::types::common::Duration>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsLifecycleStep {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsLifecycleStep;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsLifecycleStep object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_wait_time_threshold: Option<crate::types::common::Duration> = None;
                let mut seen_wait_time_threshold = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "wait_time_threshold" => {
                            if seen_wait_time_threshold {
                                return Err(serde::de::Error::duplicate_field(
                                    "wait_time_threshold",
                                ));
                            }
                            seen_wait_time_threshold = true;
                            f_wait_time_threshold =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsLifecycleStep {
                    wait_time_threshold: f_wait_time_threshold,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsLifecycleStep {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.wait_time_threshold {
            map.serialize_entry("wait_time_threshold", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The mapping configuration for the index."]
#[derive(Debug, Clone)]
pub struct IndexSettingsMapping {
    pub coerce: Option<crate::types::common::StringifiedBoolean>,
    pub depth: Option<crate::types::indices_common::IndexSettingsMappingLimitDepth>,
    pub dimension_fields:
        Option<crate::types::indices_common::IndexSettingsMappingLimitDimensionFields>,
    pub field_name_length:
        Option<crate::types::indices_common::IndexSettingsMappingLimitFieldNameLength>,
    pub ignore_malformed: Option<crate::types::common::StringifiedBoolean>,
    pub nested_fields: Option<crate::types::indices_common::IndexSettingsMappingLimitNestedFields>,
    pub nested_objects:
        Option<crate::types::indices_common::IndexSettingsMappingLimitNestedObjects>,
    pub total_fields: Option<crate::types::indices_common::IndexSettingsMappingLimitTotalFields>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsMapping {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsMapping;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsMapping object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_coerce: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_coerce = false;
                let mut f_depth: Option<
                    crate::types::indices_common::IndexSettingsMappingLimitDepth,
                > = None;
                let mut seen_depth = false;
                let mut f_dimension_fields: Option<
                    crate::types::indices_common::IndexSettingsMappingLimitDimensionFields,
                > = None;
                let mut seen_dimension_fields = false;
                let mut f_field_name_length: Option<
                    crate::types::indices_common::IndexSettingsMappingLimitFieldNameLength,
                > = None;
                let mut seen_field_name_length = false;
                let mut f_ignore_malformed: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_ignore_malformed = false;
                let mut f_nested_fields: Option<
                    crate::types::indices_common::IndexSettingsMappingLimitNestedFields,
                > = None;
                let mut seen_nested_fields = false;
                let mut f_nested_objects: Option<
                    crate::types::indices_common::IndexSettingsMappingLimitNestedObjects,
                > = None;
                let mut seen_nested_objects = false;
                let mut f_total_fields: Option<
                    crate::types::indices_common::IndexSettingsMappingLimitTotalFields,
                > = None;
                let mut seen_total_fields = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "coerce" => {
                            if seen_coerce {
                                return Err(serde::de::Error::duplicate_field("coerce"));
                            }
                            seen_coerce = true;
                            f_coerce = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "depth" => {
                            if seen_depth {
                                return Err(serde::de::Error::duplicate_field("depth"));
                            }
                            seen_depth = true;
                            f_depth = map.next_value::<Option<
                                crate::types::indices_common::IndexSettingsMappingLimitDepth,
                            >>()?;
                        }
                        "dimension_fields" => {
                            if seen_dimension_fields {
                                return Err(serde::de::Error::duplicate_field("dimension_fields"));
                            }
                            seen_dimension_fields = true;
                            f_dimension_fields = map.next_value::<Option<crate::types::indices_common::IndexSettingsMappingLimitDimensionFields>>()?;
                        }
                        "field_name_length" => {
                            if seen_field_name_length {
                                return Err(serde::de::Error::duplicate_field("field_name_length"));
                            }
                            seen_field_name_length = true;
                            f_field_name_length = map.next_value::<Option<crate::types::indices_common::IndexSettingsMappingLimitFieldNameLength>>()?;
                        }
                        "ignore_malformed" => {
                            if seen_ignore_malformed {
                                return Err(serde::de::Error::duplicate_field("ignore_malformed"));
                            }
                            seen_ignore_malformed = true;
                            f_ignore_malformed = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "nested_fields" => {
                            if seen_nested_fields {
                                return Err(serde::de::Error::duplicate_field("nested_fields"));
                            }
                            seen_nested_fields = true;
                            f_nested_fields = map.next_value::<Option<
                                crate::types::indices_common::IndexSettingsMappingLimitNestedFields,
                            >>()?;
                        }
                        "nested_objects" => {
                            if seen_nested_objects {
                                return Err(serde::de::Error::duplicate_field("nested_objects"));
                            }
                            seen_nested_objects = true;
                            f_nested_objects = map.next_value::<Option<crate::types::indices_common::IndexSettingsMappingLimitNestedObjects>>()?;
                        }
                        "total_fields" => {
                            if seen_total_fields {
                                return Err(serde::de::Error::duplicate_field("total_fields"));
                            }
                            seen_total_fields = true;
                            f_total_fields = map.next_value::<Option<
                                crate::types::indices_common::IndexSettingsMappingLimitTotalFields,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsMapping {
                    coerce: f_coerce,
                    depth: f_depth,
                    dimension_fields: f_dimension_fields,
                    field_name_length: f_field_name_length,
                    ignore_malformed: f_ignore_malformed,
                    nested_fields: f_nested_fields,
                    nested_objects: f_nested_objects,
                    total_fields: f_total_fields,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsMapping {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.coerce {
            map.serialize_entry("coerce", v)?;
        }
        if let Some(v) = &self.depth {
            map.serialize_entry("depth", v)?;
        }
        if let Some(v) = &self.dimension_fields {
            map.serialize_entry("dimension_fields", v)?;
        }
        if let Some(v) = &self.field_name_length {
            map.serialize_entry("field_name_length", v)?;
        }
        if let Some(v) = &self.ignore_malformed {
            map.serialize_entry("ignore_malformed", v)?;
        }
        if let Some(v) = &self.nested_fields {
            map.serialize_entry("nested_fields", v)?;
        }
        if let Some(v) = &self.nested_objects {
            map.serialize_entry("nested_objects", v)?;
        }
        if let Some(v) = &self.total_fields {
            map.serialize_entry("total_fields", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IndexSettingsMappingLimitDepth {
    #[doc = "The maximum depth for a field, which is measured as the number of inner objects. For instance, if all fields are defined at the root object level, then the depth is `1`. If there is one object mapping, then the depth is `2`."]
    pub limit: Option<crate::types::common::StringifiedLong>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsMappingLimitDepth {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsMappingLimitDepth;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsMappingLimitDepth object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_limit: Option<crate::types::common::StringifiedLong> = None;
                let mut seen_limit = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "limit" => {
                            if seen_limit {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            seen_limit = true;
                            f_limit =
                                map.next_value::<Option<crate::types::common::StringifiedLong>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsMappingLimitDepth {
                    limit: f_limit,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsMappingLimitDepth {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.limit {
            map.serialize_entry("limit", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IndexSettingsMappingLimitDimensionFields {
    #[doc = "[preview] This functionality is in technical preview and may be changed or removed in a future release. OpenSearch will work to fix any issues, but features in technical preview are not subject to the support SLA of official GA features."]
    pub limit: Option<crate::types::common::StringifiedLong>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsMappingLimitDimensionFields {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsMappingLimitDimensionFields;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsMappingLimitDimensionFields object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_limit: Option<crate::types::common::StringifiedLong> = None;
                let mut seen_limit = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "limit" => {
                            if seen_limit {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            seen_limit = true;
                            f_limit =
                                map.next_value::<Option<crate::types::common::StringifiedLong>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsMappingLimitDimensionFields {
                    limit: f_limit,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsMappingLimitDimensionFields {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.limit {
            map.serialize_entry("limit", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IndexSettingsMappingLimitFieldNameLength {
    #[doc = "Setting for the maximum length of a field name. This setting isn't really something that addresses mappings explosion but might still be useful if you want to limit the field length. It usually shouldn't be necessary to set this setting. The default is okay unless a user starts to add a huge number of fields with really long names. Default is `Long.MAX_VALUE` (no limit)."]
    pub limit: Option<crate::types::common::StringifiedLong>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsMappingLimitFieldNameLength {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsMappingLimitFieldNameLength;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsMappingLimitFieldNameLength object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_limit: Option<crate::types::common::StringifiedLong> = None;
                let mut seen_limit = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "limit" => {
                            if seen_limit {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            seen_limit = true;
                            f_limit =
                                map.next_value::<Option<crate::types::common::StringifiedLong>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsMappingLimitFieldNameLength {
                    limit: f_limit,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsMappingLimitFieldNameLength {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.limit {
            map.serialize_entry("limit", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IndexSettingsMappingLimitNestedFields {
    #[doc = "The maximum number of distinct nested mappings in an index. The nested type should only be used in special cases, when arrays of objects need to be queried independently of each other. To safeguard against poorly designed mappings, this setting limits the number of unique nested types per index."]
    pub limit: Option<crate::types::common::StringifiedLong>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsMappingLimitNestedFields {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsMappingLimitNestedFields;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsMappingLimitNestedFields object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_limit: Option<crate::types::common::StringifiedLong> = None;
                let mut seen_limit = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "limit" => {
                            if seen_limit {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            seen_limit = true;
                            f_limit =
                                map.next_value::<Option<crate::types::common::StringifiedLong>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsMappingLimitNestedFields {
                    limit: f_limit,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsMappingLimitNestedFields {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.limit {
            map.serialize_entry("limit", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IndexSettingsMappingLimitNestedObjects {
    #[doc = "The maximum number of nested JSON objects that a single document can contain across all nested types. This limit helps to prevent out of memory errors when a document contains too many nested objects."]
    pub limit: Option<crate::types::common::StringifiedLong>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsMappingLimitNestedObjects {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsMappingLimitNestedObjects;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsMappingLimitNestedObjects object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_limit: Option<crate::types::common::StringifiedLong> = None;
                let mut seen_limit = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "limit" => {
                            if seen_limit {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            seen_limit = true;
                            f_limit =
                                map.next_value::<Option<crate::types::common::StringifiedLong>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsMappingLimitNestedObjects {
                    limit: f_limit,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsMappingLimitNestedObjects {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.limit {
            map.serialize_entry("limit", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IndexSettingsMappingLimitTotalFields {
    #[doc = "The maximum number of fields in an index. Field and object mappings, as well as field aliases count towards this limit. The limit is in place to prevent mappings and searches from becoming too large. Higher values can lead to performance degradations and memory issues, especially in clusters with a high load or few resources."]
    pub limit: Option<crate::types::common::StringifiedLong>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsMappingLimitTotalFields {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsMappingLimitTotalFields;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsMappingLimitTotalFields object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_limit: Option<crate::types::common::StringifiedLong> = None;
                let mut seen_limit = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "limit" => {
                            if seen_limit {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            seen_limit = true;
                            f_limit =
                                map.next_value::<Option<crate::types::common::StringifiedLong>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsMappingLimitTotalFields {
                    limit: f_limit,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsMappingLimitTotalFields {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.limit {
            map.serialize_entry("limit", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for segment merging."]
#[derive(Debug, Clone)]
pub struct IndexSettingsMerge {
    pub log_byte_size_policy:
        Option<crate::types::indices_common::IndexSettingsMergeLogByteSizePolicy>,
    pub policy: Option<crate::types::indices_common::IndexSettingsMergePolicy>,
    pub policy_deletes_pct_allowed: Option<crate::types::common::StringifiedDouble>,
    pub policy_expunge_deletes_allowed: Option<crate::types::common::StringifiedDouble>,
    pub policy_floor_segment: Option<crate::types::common::HumanReadableByteCount>,
    pub policy_max_merge_at_once: Option<crate::types::common::StringifiedInteger>,
    pub policy_max_merged_segment: Option<crate::types::common::HumanReadableByteCount>,
    pub policy_reclaim_deletes_weight: Option<crate::types::common::StringifiedDouble>,
    pub policy_segments_per_tier: Option<crate::types::common::StringifiedDouble>,
    pub scheduler: Option<crate::types::indices_common::IndexSettingsMergeScheduler>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsMerge {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsMerge;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsMerge object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_log_byte_size_policy: Option<
                    crate::types::indices_common::IndexSettingsMergeLogByteSizePolicy,
                > = None;
                let mut seen_log_byte_size_policy = false;
                let mut f_policy: Option<crate::types::indices_common::IndexSettingsMergePolicy> =
                    None;
                let mut seen_policy = false;
                let mut f_policy_deletes_pct_allowed: Option<
                    crate::types::common::StringifiedDouble,
                > = None;
                let mut seen_policy_deletes_pct_allowed = false;
                let mut f_policy_expunge_deletes_allowed: Option<
                    crate::types::common::StringifiedDouble,
                > = None;
                let mut seen_policy_expunge_deletes_allowed = false;
                let mut f_policy_floor_segment: Option<
                    crate::types::common::HumanReadableByteCount,
                > = None;
                let mut seen_policy_floor_segment = false;
                let mut f_policy_max_merge_at_once: Option<
                    crate::types::common::StringifiedInteger,
                > = None;
                let mut seen_policy_max_merge_at_once = false;
                let mut f_policy_max_merged_segment: Option<
                    crate::types::common::HumanReadableByteCount,
                > = None;
                let mut seen_policy_max_merged_segment = false;
                let mut f_policy_reclaim_deletes_weight: Option<
                    crate::types::common::StringifiedDouble,
                > = None;
                let mut seen_policy_reclaim_deletes_weight = false;
                let mut f_policy_segments_per_tier: Option<
                    crate::types::common::StringifiedDouble,
                > = None;
                let mut seen_policy_segments_per_tier = false;
                let mut f_scheduler: Option<
                    crate::types::indices_common::IndexSettingsMergeScheduler,
                > = None;
                let mut seen_scheduler = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "log_byte_size_policy" => {
                            if seen_log_byte_size_policy {
                                return Err(serde::de::Error::duplicate_field(
                                    "log_byte_size_policy",
                                ));
                            }
                            seen_log_byte_size_policy = true;
                            f_log_byte_size_policy = map.next_value::<Option<
                                crate::types::indices_common::IndexSettingsMergeLogByteSizePolicy,
                            >>()?;
                        }
                        "policy" => {
                            if seen_policy {
                                return Err(serde::de::Error::duplicate_field("policy"));
                            }
                            seen_policy = true;
                            f_policy = map.next_value::<Option<crate::types::indices_common::IndexSettingsMergePolicy>>()?;
                        }
                        "policy.deletes_pct_allowed" => {
                            if seen_policy_deletes_pct_allowed {
                                return Err(serde::de::Error::duplicate_field(
                                    "policy.deletes_pct_allowed",
                                ));
                            }
                            seen_policy_deletes_pct_allowed = true;
                            f_policy_deletes_pct_allowed = map
                                .next_value::<Option<crate::types::common::StringifiedDouble>>()?;
                        }
                        "policy.expunge_deletes_allowed" => {
                            if seen_policy_expunge_deletes_allowed {
                                return Err(serde::de::Error::duplicate_field(
                                    "policy.expunge_deletes_allowed",
                                ));
                            }
                            seen_policy_expunge_deletes_allowed = true;
                            f_policy_expunge_deletes_allowed = map
                                .next_value::<Option<crate::types::common::StringifiedDouble>>()?;
                        }
                        "policy.floor_segment" => {
                            if seen_policy_floor_segment {
                                return Err(serde::de::Error::duplicate_field(
                                    "policy.floor_segment",
                                ));
                            }
                            seen_policy_floor_segment = true;
                            f_policy_floor_segment = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "policy.max_merge_at_once" => {
                            if seen_policy_max_merge_at_once {
                                return Err(serde::de::Error::duplicate_field(
                                    "policy.max_merge_at_once",
                                ));
                            }
                            seen_policy_max_merge_at_once = true;
                            f_policy_max_merge_at_once = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "policy.max_merged_segment" => {
                            if seen_policy_max_merged_segment {
                                return Err(serde::de::Error::duplicate_field(
                                    "policy.max_merged_segment",
                                ));
                            }
                            seen_policy_max_merged_segment = true;
                            f_policy_max_merged_segment = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "policy.reclaim_deletes_weight" => {
                            if seen_policy_reclaim_deletes_weight {
                                return Err(serde::de::Error::duplicate_field(
                                    "policy.reclaim_deletes_weight",
                                ));
                            }
                            seen_policy_reclaim_deletes_weight = true;
                            f_policy_reclaim_deletes_weight = map
                                .next_value::<Option<crate::types::common::StringifiedDouble>>()?;
                        }
                        "policy.segments_per_tier" => {
                            if seen_policy_segments_per_tier {
                                return Err(serde::de::Error::duplicate_field(
                                    "policy.segments_per_tier",
                                ));
                            }
                            seen_policy_segments_per_tier = true;
                            f_policy_segments_per_tier = map
                                .next_value::<Option<crate::types::common::StringifiedDouble>>()?;
                        }
                        "scheduler" => {
                            if seen_scheduler {
                                return Err(serde::de::Error::duplicate_field("scheduler"));
                            }
                            seen_scheduler = true;
                            f_scheduler =
                                map.next_value::<Option<
                                    crate::types::indices_common::IndexSettingsMergeScheduler,
                                >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsMerge {
                    log_byte_size_policy: f_log_byte_size_policy,
                    policy: f_policy,
                    policy_deletes_pct_allowed: f_policy_deletes_pct_allowed,
                    policy_expunge_deletes_allowed: f_policy_expunge_deletes_allowed,
                    policy_floor_segment: f_policy_floor_segment,
                    policy_max_merge_at_once: f_policy_max_merge_at_once,
                    policy_max_merged_segment: f_policy_max_merged_segment,
                    policy_reclaim_deletes_weight: f_policy_reclaim_deletes_weight,
                    policy_segments_per_tier: f_policy_segments_per_tier,
                    scheduler: f_scheduler,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsMerge {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.log_byte_size_policy {
            map.serialize_entry("log_byte_size_policy", v)?;
        }
        if let Some(v) = &self.policy {
            map.serialize_entry("policy", v)?;
        }
        if let Some(v) = &self.policy_deletes_pct_allowed {
            map.serialize_entry("policy.deletes_pct_allowed", v)?;
        }
        if let Some(v) = &self.policy_expunge_deletes_allowed {
            map.serialize_entry("policy.expunge_deletes_allowed", v)?;
        }
        if let Some(v) = &self.policy_floor_segment {
            map.serialize_entry("policy.floor_segment", v)?;
        }
        if let Some(v) = &self.policy_max_merge_at_once {
            map.serialize_entry("policy.max_merge_at_once", v)?;
        }
        if let Some(v) = &self.policy_max_merged_segment {
            map.serialize_entry("policy.max_merged_segment", v)?;
        }
        if let Some(v) = &self.policy_reclaim_deletes_weight {
            map.serialize_entry("policy.reclaim_deletes_weight", v)?;
        }
        if let Some(v) = &self.policy_segments_per_tier {
            map.serialize_entry("policy.segments_per_tier", v)?;
        }
        if let Some(v) = &self.scheduler {
            map.serialize_entry("scheduler", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for log byte size merge policy."]
#[derive(Debug, Clone)]
pub struct IndexSettingsMergeLogByteSizePolicy {
    pub max_merge_segment: Option<crate::types::common::HumanReadableByteCount>,
    pub max_merge_segment_forced_merge: Option<crate::types::common::HumanReadableByteCount>,
    pub max_merged_docs: Option<crate::types::common::StringifiedInteger>,
    pub merge_factor: Option<crate::types::common::StringifiedInteger>,
    pub min_merge: Option<crate::types::common::HumanReadableByteCount>,
    pub no_cfs_ratio: Option<crate::types::common::StringifiedDouble>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsMergeLogByteSizePolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsMergeLogByteSizePolicy;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsMergeLogByteSizePolicy object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_max_merge_segment: Option<crate::types::common::HumanReadableByteCount> =
                    None;
                let mut seen_max_merge_segment = false;
                let mut f_max_merge_segment_forced_merge: Option<
                    crate::types::common::HumanReadableByteCount,
                > = None;
                let mut seen_max_merge_segment_forced_merge = false;
                let mut f_max_merged_docs: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_max_merged_docs = false;
                let mut f_merge_factor: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_merge_factor = false;
                let mut f_min_merge: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_min_merge = false;
                let mut f_no_cfs_ratio: Option<crate::types::common::StringifiedDouble> = None;
                let mut seen_no_cfs_ratio = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "max_merge_segment" => {
                            if seen_max_merge_segment {
                                return Err(serde::de::Error::duplicate_field("max_merge_segment"));
                            }
                            seen_max_merge_segment = true;
                            f_max_merge_segment = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "max_merge_segment_forced_merge" => {
                            if seen_max_merge_segment_forced_merge {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_merge_segment_forced_merge",
                                ));
                            }
                            seen_max_merge_segment_forced_merge = true;
                            f_max_merge_segment_forced_merge = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "max_merged_docs" => {
                            if seen_max_merged_docs {
                                return Err(serde::de::Error::duplicate_field("max_merged_docs"));
                            }
                            seen_max_merged_docs = true;
                            f_max_merged_docs = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "merge_factor" => {
                            if seen_merge_factor {
                                return Err(serde::de::Error::duplicate_field("merge_factor"));
                            }
                            seen_merge_factor = true;
                            f_merge_factor = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "min_merge" => {
                            if seen_min_merge {
                                return Err(serde::de::Error::duplicate_field("min_merge"));
                            }
                            seen_min_merge = true;
                            f_min_merge = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "no_cfs_ratio" => {
                            if seen_no_cfs_ratio {
                                return Err(serde::de::Error::duplicate_field("no_cfs_ratio"));
                            }
                            seen_no_cfs_ratio = true;
                            f_no_cfs_ratio = map
                                .next_value::<Option<crate::types::common::StringifiedDouble>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsMergeLogByteSizePolicy {
                    max_merge_segment: f_max_merge_segment,
                    max_merge_segment_forced_merge: f_max_merge_segment_forced_merge,
                    max_merged_docs: f_max_merged_docs,
                    merge_factor: f_merge_factor,
                    min_merge: f_min_merge,
                    no_cfs_ratio: f_no_cfs_ratio,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsMergeLogByteSizePolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.max_merge_segment {
            map.serialize_entry("max_merge_segment", v)?;
        }
        if let Some(v) = &self.max_merge_segment_forced_merge {
            map.serialize_entry("max_merge_segment_forced_merge", v)?;
        }
        if let Some(v) = &self.max_merged_docs {
            map.serialize_entry("max_merged_docs", v)?;
        }
        if let Some(v) = &self.merge_factor {
            map.serialize_entry("merge_factor", v)?;
        }
        if let Some(v) = &self.min_merge {
            map.serialize_entry("min_merge", v)?;
        }
        if let Some(v) = &self.no_cfs_ratio {
            map.serialize_entry("no_cfs_ratio", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The policy configuration for segment merging."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum IndexSettingsMergePolicy {
    Name(crate::types::indices_common::IndexSettingsMergePolicyName),
    TieredPolicyConfig(crate::types::indices_common::IndexSettingsMergeTieredPolicy),
}
impl<'de> serde::Deserialize<'de> for IndexSettingsMergePolicy {
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
            Some(b'"') => serde_json::from_str(text).map(IndexSettingsMergePolicy::Name),
            Some(b'{') => {
                serde_json::from_str(text).map(IndexSettingsMergePolicy::TieredPolicyConfig)
            }
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a IndexSettingsMergePolicy value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The type of merge policy."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexSettingsMergePolicyName {
    #[doc = "`default`"]
    Default,
    #[doc = "`log_byte_size`"]
    LogByteSize,
    #[doc = "`tiered`"]
    Tiered,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexSettingsMergePolicyName {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexSettingsMergePolicyName::Default => "default",
            IndexSettingsMergePolicyName::LogByteSize => "log_byte_size",
            IndexSettingsMergePolicyName::Tiered => "tiered",
            IndexSettingsMergePolicyName::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexSettingsMergePolicyName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexSettingsMergePolicyName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "default" => IndexSettingsMergePolicyName::Default,
            "log_byte_size" => IndexSettingsMergePolicyName::LogByteSize,
            "tiered" => IndexSettingsMergePolicyName::Tiered,
            _ => IndexSettingsMergePolicyName::Unknown(s),
        })
    }
}

#[doc = "The configuration for merge scheduling."]
#[derive(Debug, Clone)]
pub struct IndexSettingsMergeScheduler {
    pub auto_throttle: Option<crate::types::common::StringifiedBoolean>,
    pub max_merge_count: Option<crate::types::common::StringifiedInteger>,
    pub max_thread_count: Option<crate::types::common::StringifiedInteger>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsMergeScheduler {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsMergeScheduler;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsMergeScheduler object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_auto_throttle: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_auto_throttle = false;
                let mut f_max_merge_count: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_max_merge_count = false;
                let mut f_max_thread_count: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_max_thread_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "auto_throttle" => {
                            if seen_auto_throttle {
                                return Err(serde::de::Error::duplicate_field("auto_throttle"));
                            }
                            seen_auto_throttle = true;
                            f_auto_throttle = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "max_merge_count" => {
                            if seen_max_merge_count {
                                return Err(serde::de::Error::duplicate_field("max_merge_count"));
                            }
                            seen_max_merge_count = true;
                            f_max_merge_count = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_thread_count" => {
                            if seen_max_thread_count {
                                return Err(serde::de::Error::duplicate_field("max_thread_count"));
                            }
                            seen_max_thread_count = true;
                            f_max_thread_count = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsMergeScheduler {
                    auto_throttle: f_auto_throttle,
                    max_merge_count: f_max_merge_count,
                    max_thread_count: f_max_thread_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsMergeScheduler {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.auto_throttle {
            map.serialize_entry("auto_throttle", v)?;
        }
        if let Some(v) = &self.max_merge_count {
            map.serialize_entry("max_merge_count", v)?;
        }
        if let Some(v) = &self.max_thread_count {
            map.serialize_entry("max_thread_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for tiered merge policy."]
#[derive(Debug, Clone)]
pub struct IndexSettingsMergeTieredPolicy {
    pub deletes_pct_allowed: Option<crate::types::common::StringifiedDouble>,
    pub expunge_deletes_allowed: Option<crate::types::common::StringifiedDouble>,
    pub floor_segment: Option<crate::types::common::HumanReadableByteCount>,
    pub max_merge_at_once: Option<crate::types::common::StringifiedInteger>,
    pub max_merge_at_once_explicit: Option<crate::types::common::StringifiedInteger>,
    pub max_merged_segment: Option<crate::types::common::HumanReadableByteCount>,
    pub reclaim_deletes_weight: Option<crate::types::common::StringifiedDouble>,
    pub segments_per_tier: Option<crate::types::common::StringifiedDouble>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsMergeTieredPolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsMergeTieredPolicy;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsMergeTieredPolicy object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_deletes_pct_allowed: Option<crate::types::common::StringifiedDouble> =
                    None;
                let mut seen_deletes_pct_allowed = false;
                let mut f_expunge_deletes_allowed: Option<crate::types::common::StringifiedDouble> =
                    None;
                let mut seen_expunge_deletes_allowed = false;
                let mut f_floor_segment: Option<crate::types::common::HumanReadableByteCount> =
                    None;
                let mut seen_floor_segment = false;
                let mut f_max_merge_at_once: Option<crate::types::common::StringifiedInteger> =
                    None;
                let mut seen_max_merge_at_once = false;
                let mut f_max_merge_at_once_explicit: Option<
                    crate::types::common::StringifiedInteger,
                > = None;
                let mut seen_max_merge_at_once_explicit = false;
                let mut f_max_merged_segment: Option<crate::types::common::HumanReadableByteCount> =
                    None;
                let mut seen_max_merged_segment = false;
                let mut f_reclaim_deletes_weight: Option<crate::types::common::StringifiedDouble> =
                    None;
                let mut seen_reclaim_deletes_weight = false;
                let mut f_segments_per_tier: Option<crate::types::common::StringifiedDouble> = None;
                let mut seen_segments_per_tier = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "deletes_pct_allowed" => {
                            if seen_deletes_pct_allowed {
                                return Err(serde::de::Error::duplicate_field(
                                    "deletes_pct_allowed",
                                ));
                            }
                            seen_deletes_pct_allowed = true;
                            f_deletes_pct_allowed = map
                                .next_value::<Option<crate::types::common::StringifiedDouble>>()?;
                        }
                        "expunge_deletes_allowed" => {
                            if seen_expunge_deletes_allowed {
                                return Err(serde::de::Error::duplicate_field(
                                    "expunge_deletes_allowed",
                                ));
                            }
                            seen_expunge_deletes_allowed = true;
                            f_expunge_deletes_allowed = map
                                .next_value::<Option<crate::types::common::StringifiedDouble>>()?;
                        }
                        "floor_segment" => {
                            if seen_floor_segment {
                                return Err(serde::de::Error::duplicate_field("floor_segment"));
                            }
                            seen_floor_segment = true;
                            f_floor_segment = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "max_merge_at_once" => {
                            if seen_max_merge_at_once {
                                return Err(serde::de::Error::duplicate_field("max_merge_at_once"));
                            }
                            seen_max_merge_at_once = true;
                            f_max_merge_at_once = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_merge_at_once_explicit" => {
                            if seen_max_merge_at_once_explicit {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_merge_at_once_explicit",
                                ));
                            }
                            seen_max_merge_at_once_explicit = true;
                            f_max_merge_at_once_explicit = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_merged_segment" => {
                            if seen_max_merged_segment {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_merged_segment",
                                ));
                            }
                            seen_max_merged_segment = true;
                            f_max_merged_segment = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "reclaim_deletes_weight" => {
                            if seen_reclaim_deletes_weight {
                                return Err(serde::de::Error::duplicate_field(
                                    "reclaim_deletes_weight",
                                ));
                            }
                            seen_reclaim_deletes_weight = true;
                            f_reclaim_deletes_weight = map
                                .next_value::<Option<crate::types::common::StringifiedDouble>>()?;
                        }
                        "segments_per_tier" => {
                            if seen_segments_per_tier {
                                return Err(serde::de::Error::duplicate_field("segments_per_tier"));
                            }
                            seen_segments_per_tier = true;
                            f_segments_per_tier = map
                                .next_value::<Option<crate::types::common::StringifiedDouble>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsMergeTieredPolicy {
                    deletes_pct_allowed: f_deletes_pct_allowed,
                    expunge_deletes_allowed: f_expunge_deletes_allowed,
                    floor_segment: f_floor_segment,
                    max_merge_at_once: f_max_merge_at_once,
                    max_merge_at_once_explicit: f_max_merge_at_once_explicit,
                    max_merged_segment: f_max_merged_segment,
                    reclaim_deletes_weight: f_reclaim_deletes_weight,
                    segments_per_tier: f_segments_per_tier,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsMergeTieredPolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.deletes_pct_allowed {
            map.serialize_entry("deletes_pct_allowed", v)?;
        }
        if let Some(v) = &self.expunge_deletes_allowed {
            map.serialize_entry("expunge_deletes_allowed", v)?;
        }
        if let Some(v) = &self.floor_segment {
            map.serialize_entry("floor_segment", v)?;
        }
        if let Some(v) = &self.max_merge_at_once {
            map.serialize_entry("max_merge_at_once", v)?;
        }
        if let Some(v) = &self.max_merge_at_once_explicit {
            map.serialize_entry("max_merge_at_once_explicit", v)?;
        }
        if let Some(v) = &self.max_merged_segment {
            map.serialize_entry("max_merged_segment", v)?;
        }
        if let Some(v) = &self.reclaim_deletes_weight {
            map.serialize_entry("reclaim_deletes_weight", v)?;
        }
        if let Some(v) = &self.segments_per_tier {
            map.serialize_entry("segments_per_tier", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for query caching."]
#[derive(Debug, Clone)]
pub struct IndexSettingsQueries {
    pub cache: Option<crate::types::indices_common::IndexSettingsQueriesCache>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsQueries {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsQueries;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsQueries object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cache: Option<crate::types::indices_common::IndexSettingsQueriesCache> =
                    None;
                let mut seen_cache = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cache" => {
                            if seen_cache {
                                return Err(serde::de::Error::duplicate_field("cache"));
                            }
                            seen_cache = true;
                            f_cache = map.next_value::<Option<crate::types::indices_common::IndexSettingsQueriesCache>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsQueries {
                    cache: f_cache,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsQueries {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cache {
            map.serialize_entry("cache", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The query cache settings."]
#[derive(Debug, Clone)]
pub struct IndexSettingsQueriesCache {
    pub enabled: crate::types::common::StringifiedBoolean,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsQueriesCache {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsQueriesCache;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsQueriesCache object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_enabled: Option<crate::types::common::StringifiedBoolean> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "enabled" => {
                            if f_enabled.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            f_enabled =
                                Some(map.next_value::<crate::types::common::StringifiedBoolean>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsQueriesCache {
                    enabled: f_enabled.ok_or_else(|| serde::de::Error::missing_field("enabled"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsQueriesCache {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("enabled", &self.enabled)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for query string parsing."]
#[derive(Debug, Clone)]
pub struct IndexSettingsQueryString {
    pub lenient: Option<crate::types::common::StringifiedBoolean>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsQueryString {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsQueryString;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsQueryString object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_lenient: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_lenient = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "lenient" => {
                            if seen_lenient {
                                return Err(serde::de::Error::duplicate_field("lenient"));
                            }
                            seen_lenient = true;
                            f_lenient = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsQueryString {
                    lenient: f_lenient,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsQueryString {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.lenient {
            map.serialize_entry("lenient", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for search operations."]
#[derive(Debug, Clone)]
pub struct IndexSettingsSearch {
    pub concurrent: Option<crate::types::indices_common::IndexSettingsSearchConcurrent>,
    pub concurrent_segment_search:
        Option<crate::types::indices_common::IndexSettingsSearchConcurrentSegmentSearch>,
    #[doc = "The default search pipeline to use."]
    pub default_pipeline: Option<String>,
    pub idle: Option<crate::types::indices_common::SearchIdle>,
    pub slowlog: Option<crate::types::indices_common::SearchSlowlog>,
    pub star_tree_index: Option<crate::types::indices_common::IndexSettingsSearchStarTreeIndex>,
    pub throttled: Option<crate::types::common::StringifiedBoolean>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSearch {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsSearch;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsSearch object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_concurrent: Option<
                    crate::types::indices_common::IndexSettingsSearchConcurrent,
                > = None;
                let mut seen_concurrent = false;
                let mut f_concurrent_segment_search: Option<
                    crate::types::indices_common::IndexSettingsSearchConcurrentSegmentSearch,
                > = None;
                let mut seen_concurrent_segment_search = false;
                let mut f_default_pipeline: Option<String> = None;
                let mut seen_default_pipeline = false;
                let mut f_idle: Option<crate::types::indices_common::SearchIdle> = None;
                let mut seen_idle = false;
                let mut f_slowlog: Option<crate::types::indices_common::SearchSlowlog> = None;
                let mut seen_slowlog = false;
                let mut f_star_tree_index: Option<
                    crate::types::indices_common::IndexSettingsSearchStarTreeIndex,
                > = None;
                let mut seen_star_tree_index = false;
                let mut f_throttled: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_throttled = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "concurrent" => {
                            if seen_concurrent {
                                return Err(serde::de::Error::duplicate_field("concurrent"));
                            }
                            seen_concurrent = true;
                            f_concurrent = map.next_value::<Option<
                                crate::types::indices_common::IndexSettingsSearchConcurrent,
                            >>()?;
                        }
                        "concurrent_segment_search" => {
                            if seen_concurrent_segment_search {
                                return Err(serde::de::Error::duplicate_field(
                                    "concurrent_segment_search",
                                ));
                            }
                            seen_concurrent_segment_search = true;
                            f_concurrent_segment_search = map.next_value::<Option<crate::types::indices_common::IndexSettingsSearchConcurrentSegmentSearch>>()?;
                        }
                        "default_pipeline" => {
                            if seen_default_pipeline {
                                return Err(serde::de::Error::duplicate_field("default_pipeline"));
                            }
                            seen_default_pipeline = true;
                            f_default_pipeline = map.next_value::<Option<String>>()?;
                        }
                        "idle" => {
                            if seen_idle {
                                return Err(serde::de::Error::duplicate_field("idle"));
                            }
                            seen_idle = true;
                            f_idle = map
                                .next_value::<Option<crate::types::indices_common::SearchIdle>>()?;
                        }
                        "slowlog" => {
                            if seen_slowlog {
                                return Err(serde::de::Error::duplicate_field("slowlog"));
                            }
                            seen_slowlog = true;
                            f_slowlog = map
                                .next_value::<Option<crate::types::indices_common::SearchSlowlog>>(
                                )?;
                        }
                        "star_tree_index" => {
                            if seen_star_tree_index {
                                return Err(serde::de::Error::duplicate_field("star_tree_index"));
                            }
                            seen_star_tree_index = true;
                            f_star_tree_index = map.next_value::<Option<
                                crate::types::indices_common::IndexSettingsSearchStarTreeIndex,
                            >>()?;
                        }
                        "throttled" => {
                            if seen_throttled {
                                return Err(serde::de::Error::duplicate_field("throttled"));
                            }
                            seen_throttled = true;
                            f_throttled = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsSearch {
                    concurrent: f_concurrent,
                    concurrent_segment_search: f_concurrent_segment_search,
                    default_pipeline: f_default_pipeline,
                    idle: f_idle,
                    slowlog: f_slowlog,
                    star_tree_index: f_star_tree_index,
                    throttled: f_throttled,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsSearch {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.concurrent {
            map.serialize_entry("concurrent", v)?;
        }
        if let Some(v) = &self.concurrent_segment_search {
            map.serialize_entry("concurrent_segment_search", v)?;
        }
        if let Some(v) = &self.default_pipeline {
            map.serialize_entry("default_pipeline", v)?;
        }
        if let Some(v) = &self.idle {
            map.serialize_entry("idle", v)?;
        }
        if let Some(v) = &self.slowlog {
            map.serialize_entry("slowlog", v)?;
        }
        if let Some(v) = &self.star_tree_index {
            map.serialize_entry("star_tree_index", v)?;
        }
        if let Some(v) = &self.throttled {
            map.serialize_entry("throttled", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for concurrent search operations."]
#[derive(Debug, Clone)]
pub struct IndexSettingsSearchConcurrent {
    pub max_slice_count: Option<crate::types::common::StringifiedInteger>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSearchConcurrent {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsSearchConcurrent;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsSearchConcurrent object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_max_slice_count: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_max_slice_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "max_slice_count" => {
                            if seen_max_slice_count {
                                return Err(serde::de::Error::duplicate_field("max_slice_count"));
                            }
                            seen_max_slice_count = true;
                            f_max_slice_count = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsSearchConcurrent {
                    max_slice_count: f_max_slice_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsSearchConcurrent {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.max_slice_count {
            map.serialize_entry("max_slice_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for concurrent segment search."]
#[derive(Debug, Clone)]
pub struct IndexSettingsSearchConcurrentSegmentSearch {
    pub enabled: Option<crate::types::common::StringifiedBoolean>,
    #[doc = "The mode of concurrent segment search."]
    pub mode: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSearchConcurrentSegmentSearch {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsSearchConcurrentSegmentSearch;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsSearchConcurrentSegmentSearch object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_enabled: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_enabled = false;
                let mut f_mode: Option<String> = None;
                let mut seen_mode = false;
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
                            f_enabled = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "mode" => {
                            if seen_mode {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            seen_mode = true;
                            f_mode = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsSearchConcurrentSegmentSearch {
                    enabled: f_enabled,
                    mode: f_mode,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsSearchConcurrentSegmentSearch {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.enabled {
            map.serialize_entry("enabled", v)?;
        }
        if let Some(v) = &self.mode {
            map.serialize_entry("mode", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IndexSettingsSearchStarTreeIndex {
    pub enabled: Option<crate::types::common::StringifiedBoolean>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSearchStarTreeIndex {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsSearchStarTreeIndex;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsSearchStarTreeIndex object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_enabled: Option<crate::types::common::StringifiedBoolean> = None;
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
                            f_enabled = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsSearchStarTreeIndex {
                    enabled: f_enabled,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsSearchStarTreeIndex {
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

#[doc = "The configuration for similarity algorithms."]
#[derive(Debug, Clone)]
pub struct IndexSettingsSimilarity {
    pub bm_25: Option<crate::types::indices_common::IndexSettingsSimilarityBm25>,
    pub dfi: Option<crate::types::indices_common::IndexSettingsSimilarityDfi>,
    pub dfr: Option<crate::types::indices_common::IndexSettingsSimilarityDfr>,
    pub ib: Option<crate::types::indices_common::IndexSettingsSimilarityIb>,
    pub lmd: Option<crate::types::indices_common::IndexSettingsSimilarityLmd>,
    pub lmj: Option<crate::types::indices_common::IndexSettingsSimilarityLmj>,
    pub scripted_tfidf: Option<crate::types::indices_common::IndexSettingsSimilarityScriptedTfidf>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSimilarity {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsSimilarity;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsSimilarity object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_bm_25: Option<crate::types::indices_common::IndexSettingsSimilarityBm25> =
                    None;
                let mut seen_bm_25 = false;
                let mut f_dfi: Option<crate::types::indices_common::IndexSettingsSimilarityDfi> =
                    None;
                let mut seen_dfi = false;
                let mut f_dfr: Option<crate::types::indices_common::IndexSettingsSimilarityDfr> =
                    None;
                let mut seen_dfr = false;
                let mut f_ib: Option<crate::types::indices_common::IndexSettingsSimilarityIb> =
                    None;
                let mut seen_ib = false;
                let mut f_lmd: Option<crate::types::indices_common::IndexSettingsSimilarityLmd> =
                    None;
                let mut seen_lmd = false;
                let mut f_lmj: Option<crate::types::indices_common::IndexSettingsSimilarityLmj> =
                    None;
                let mut seen_lmj = false;
                let mut f_scripted_tfidf: Option<
                    crate::types::indices_common::IndexSettingsSimilarityScriptedTfidf,
                > = None;
                let mut seen_scripted_tfidf = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "bm25" => {
                            if seen_bm_25 {
                                return Err(serde::de::Error::duplicate_field("bm25"));
                            }
                            seen_bm_25 = true;
                            f_bm_25 =
                                map.next_value::<Option<
                                    crate::types::indices_common::IndexSettingsSimilarityBm25,
                                >>()?;
                        }
                        "dfi" => {
                            if seen_dfi {
                                return Err(serde::de::Error::duplicate_field("dfi"));
                            }
                            seen_dfi = true;
                            f_dfi =
                                map.next_value::<Option<
                                    crate::types::indices_common::IndexSettingsSimilarityDfi,
                                >>()?;
                        }
                        "dfr" => {
                            if seen_dfr {
                                return Err(serde::de::Error::duplicate_field("dfr"));
                            }
                            seen_dfr = true;
                            f_dfr =
                                map.next_value::<Option<
                                    crate::types::indices_common::IndexSettingsSimilarityDfr,
                                >>()?;
                        }
                        "ib" => {
                            if seen_ib {
                                return Err(serde::de::Error::duplicate_field("ib"));
                            }
                            seen_ib = true;
                            f_ib = map.next_value::<Option<crate::types::indices_common::IndexSettingsSimilarityIb>>()?;
                        }
                        "lmd" => {
                            if seen_lmd {
                                return Err(serde::de::Error::duplicate_field("lmd"));
                            }
                            seen_lmd = true;
                            f_lmd =
                                map.next_value::<Option<
                                    crate::types::indices_common::IndexSettingsSimilarityLmd,
                                >>()?;
                        }
                        "lmj" => {
                            if seen_lmj {
                                return Err(serde::de::Error::duplicate_field("lmj"));
                            }
                            seen_lmj = true;
                            f_lmj =
                                map.next_value::<Option<
                                    crate::types::indices_common::IndexSettingsSimilarityLmj,
                                >>()?;
                        }
                        "scripted_tfidf" => {
                            if seen_scripted_tfidf {
                                return Err(serde::de::Error::duplicate_field("scripted_tfidf"));
                            }
                            seen_scripted_tfidf = true;
                            f_scripted_tfidf = map.next_value::<Option<
                                crate::types::indices_common::IndexSettingsSimilarityScriptedTfidf,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsSimilarity {
                    bm_25: f_bm_25,
                    dfi: f_dfi,
                    dfr: f_dfr,
                    ib: f_ib,
                    lmd: f_lmd,
                    lmj: f_lmj,
                    scripted_tfidf: f_scripted_tfidf,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsSimilarity {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.bm_25 {
            map.serialize_entry("bm25", v)?;
        }
        if let Some(v) = &self.dfi {
            map.serialize_entry("dfi", v)?;
        }
        if let Some(v) = &self.dfr {
            map.serialize_entry("dfr", v)?;
        }
        if let Some(v) = &self.ib {
            map.serialize_entry("ib", v)?;
        }
        if let Some(v) = &self.lmd {
            map.serialize_entry("lmd", v)?;
        }
        if let Some(v) = &self.lmj {
            map.serialize_entry("lmj", v)?;
        }
        if let Some(v) = &self.scripted_tfidf {
            map.serialize_entry("scripted_tfidf", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The BM25 similarity algorithm configuration."]
#[derive(Debug, Clone)]
pub struct IndexSettingsSimilarityBm25 {
    #[doc = "The length normalization parameter."]
    pub b: f64,
    #[doc = "Whether to discount overlapping tokens."]
    pub discount_overlaps: bool,
    #[doc = "The term frequency normalization parameter."]
    pub k_1: f64,
    #[doc = "The BM25 similarity algorithm."]
    pub r#type: crate::types::indices_common::IndexSettingsSimilarityBm25Type,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSimilarityBm25 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsSimilarityBm25;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsSimilarityBm25 object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_b: Option<f64> = None;
                let mut f_discount_overlaps: Option<bool> = None;
                let mut f_k_1: Option<f64> = None;
                let mut f_type: Option<
                    crate::types::indices_common::IndexSettingsSimilarityBm25Type,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "b" => {
                            if f_b.is_some() {
                                return Err(serde::de::Error::duplicate_field("b"));
                            }
                            f_b = Some(map.next_value::<f64>()?);
                        }
                        "discount_overlaps" => {
                            if f_discount_overlaps.is_some() {
                                return Err(serde::de::Error::duplicate_field("discount_overlaps"));
                            }
                            f_discount_overlaps = Some(map.next_value::<bool>()?);
                        }
                        "k1" => {
                            if f_k_1.is_some() {
                                return Err(serde::de::Error::duplicate_field("k1"));
                            }
                            f_k_1 = Some(map.next_value::<f64>()?);
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::indices_common::IndexSettingsSimilarityBm25Type>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsSimilarityBm25 {
                    b: f_b.ok_or_else(|| serde::de::Error::missing_field("b"))?,
                    discount_overlaps: f_discount_overlaps
                        .ok_or_else(|| serde::de::Error::missing_field("discount_overlaps"))?,
                    k_1: f_k_1.ok_or_else(|| serde::de::Error::missing_field("k1"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsSimilarityBm25 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("b", &self.b)?;
        map.serialize_entry("discount_overlaps", &self.discount_overlaps)?;
        map.serialize_entry("k1", &self.k_1)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The BM25 similarity algorithm."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexSettingsSimilarityBm25Type {
    #[doc = "`BM25`"]
    Bm25,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexSettingsSimilarityBm25Type {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexSettingsSimilarityBm25Type::Bm25 => "BM25",
            IndexSettingsSimilarityBm25Type::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexSettingsSimilarityBm25Type {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSimilarityBm25Type {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "BM25" => IndexSettingsSimilarityBm25Type::Bm25,
            _ => IndexSettingsSimilarityBm25Type::Unknown(s),
        })
    }
}

#[doc = "The divergence from independence (DFI) similarity algorithm configuration."]
#[derive(Debug, Clone)]
pub struct IndexSettingsSimilarityDfi {
    pub independence_measure: crate::types::common::DFIIndependenceMeasure,
    pub r#type: crate::types::indices_common::IndexSettingsSimilarityDfiType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSimilarityDfi {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsSimilarityDfi;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsSimilarityDfi object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_independence_measure: Option<
                    crate::types::common::DFIIndependenceMeasure,
                > = None;
                let mut f_type: Option<
                    crate::types::indices_common::IndexSettingsSimilarityDfiType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "independence_measure" => {
                            if f_independence_measure.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "independence_measure",
                                ));
                            }
                            f_independence_measure = Some(
                                map.next_value::<crate::types::common::DFIIndependenceMeasure>()?,
                            );
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::indices_common::IndexSettingsSimilarityDfiType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsSimilarityDfi {
                    independence_measure: f_independence_measure
                        .ok_or_else(|| serde::de::Error::missing_field("independence_measure"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsSimilarityDfi {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("independence_measure", &self.independence_measure)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexSettingsSimilarityDfiType {
    #[doc = "`DFI`"]
    Dfi,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexSettingsSimilarityDfiType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexSettingsSimilarityDfiType::Dfi => "DFI",
            IndexSettingsSimilarityDfiType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexSettingsSimilarityDfiType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSimilarityDfiType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "DFI" => IndexSettingsSimilarityDfiType::Dfi,
            _ => IndexSettingsSimilarityDfiType::Unknown(s),
        })
    }
}

#[doc = "The divergence from randomness (DFR) similarity algorithm configuration."]
#[derive(Debug, Clone)]
pub struct IndexSettingsSimilarityDfr {
    pub after_effect: crate::types::common::DFRAfterEffect,
    pub basic_model: crate::types::common::DFRBasicModel,
    pub normalization: crate::types::common::TermFrequencyNormalization,
    pub r#type: crate::types::indices_common::IndexSettingsSimilarityDfrType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSimilarityDfr {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsSimilarityDfr;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsSimilarityDfr object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_after_effect: Option<crate::types::common::DFRAfterEffect> = None;
                let mut f_basic_model: Option<crate::types::common::DFRBasicModel> = None;
                let mut f_normalization: Option<crate::types::common::TermFrequencyNormalization> =
                    None;
                let mut f_type: Option<
                    crate::types::indices_common::IndexSettingsSimilarityDfrType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "after_effect" => {
                            if f_after_effect.is_some() {
                                return Err(serde::de::Error::duplicate_field("after_effect"));
                            }
                            f_after_effect =
                                Some(map.next_value::<crate::types::common::DFRAfterEffect>()?);
                        }
                        "basic_model" => {
                            if f_basic_model.is_some() {
                                return Err(serde::de::Error::duplicate_field("basic_model"));
                            }
                            f_basic_model =
                                Some(map.next_value::<crate::types::common::DFRBasicModel>()?);
                        }
                        "normalization" => {
                            if f_normalization.is_some() {
                                return Err(serde::de::Error::duplicate_field("normalization"));
                            }
                            f_normalization = Some(
                                map.next_value::<crate::types::common::TermFrequencyNormalization>(
                                )?,
                            );
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::indices_common::IndexSettingsSimilarityDfrType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsSimilarityDfr {
                    after_effect: f_after_effect
                        .ok_or_else(|| serde::de::Error::missing_field("after_effect"))?,
                    basic_model: f_basic_model
                        .ok_or_else(|| serde::de::Error::missing_field("basic_model"))?,
                    normalization: f_normalization
                        .ok_or_else(|| serde::de::Error::missing_field("normalization"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsSimilarityDfr {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("after_effect", &self.after_effect)?;
        map.serialize_entry("basic_model", &self.basic_model)?;
        map.serialize_entry("normalization", &self.normalization)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexSettingsSimilarityDfrType {
    #[doc = "`DFR`"]
    Dfr,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexSettingsSimilarityDfrType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexSettingsSimilarityDfrType::Dfr => "DFR",
            IndexSettingsSimilarityDfrType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexSettingsSimilarityDfrType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSimilarityDfrType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "DFR" => IndexSettingsSimilarityDfrType::Dfr,
            _ => IndexSettingsSimilarityDfrType::Unknown(s),
        })
    }
}

#[doc = "The information-based similarity algorithm configuration."]
#[derive(Debug, Clone)]
pub struct IndexSettingsSimilarityIb {
    pub distribution: crate::types::common::IBDistribution,
    pub lambda: crate::types::common::IBLambda,
    pub normalization: crate::types::common::TermFrequencyNormalization,
    pub r#type: crate::types::indices_common::IndexSettingsSimilarityIbType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSimilarityIb {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsSimilarityIb;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsSimilarityIb object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_distribution: Option<crate::types::common::IBDistribution> = None;
                let mut f_lambda: Option<crate::types::common::IBLambda> = None;
                let mut f_normalization: Option<crate::types::common::TermFrequencyNormalization> =
                    None;
                let mut f_type: Option<
                    crate::types::indices_common::IndexSettingsSimilarityIbType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "distribution" => {
                            if f_distribution.is_some() {
                                return Err(serde::de::Error::duplicate_field("distribution"));
                            }
                            f_distribution =
                                Some(map.next_value::<crate::types::common::IBDistribution>()?);
                        }
                        "lambda" => {
                            if f_lambda.is_some() {
                                return Err(serde::de::Error::duplicate_field("lambda"));
                            }
                            f_lambda = Some(map.next_value::<crate::types::common::IBLambda>()?);
                        }
                        "normalization" => {
                            if f_normalization.is_some() {
                                return Err(serde::de::Error::duplicate_field("normalization"));
                            }
                            f_normalization = Some(
                                map.next_value::<crate::types::common::TermFrequencyNormalization>(
                                )?,
                            );
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::indices_common::IndexSettingsSimilarityIbType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsSimilarityIb {
                    distribution: f_distribution
                        .ok_or_else(|| serde::de::Error::missing_field("distribution"))?,
                    lambda: f_lambda.ok_or_else(|| serde::de::Error::missing_field("lambda"))?,
                    normalization: f_normalization
                        .ok_or_else(|| serde::de::Error::missing_field("normalization"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsSimilarityIb {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("distribution", &self.distribution)?;
        map.serialize_entry("lambda", &self.lambda)?;
        map.serialize_entry("normalization", &self.normalization)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexSettingsSimilarityIbType {
    #[doc = "`IB`"]
    Ib,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexSettingsSimilarityIbType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexSettingsSimilarityIbType::Ib => "IB",
            IndexSettingsSimilarityIbType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexSettingsSimilarityIbType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSimilarityIbType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "IB" => IndexSettingsSimilarityIbType::Ib,
            _ => IndexSettingsSimilarityIbType::Unknown(s),
        })
    }
}

#[doc = "The LM Dirichlet similarity algorithm configuration."]
#[derive(Debug, Clone)]
pub struct IndexSettingsSimilarityLmd {
    #[doc = "The smoothing parameter."]
    pub mu: f64,
    pub r#type: crate::types::indices_common::IndexSettingsSimilarityLmdType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSimilarityLmd {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsSimilarityLmd;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsSimilarityLmd object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_mu: Option<f64> = None;
                let mut f_type: Option<
                    crate::types::indices_common::IndexSettingsSimilarityLmdType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "mu" => {
                            if f_mu.is_some() {
                                return Err(serde::de::Error::duplicate_field("mu"));
                            }
                            f_mu = Some(map.next_value::<f64>()?);
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::indices_common::IndexSettingsSimilarityLmdType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsSimilarityLmd {
                    mu: f_mu.ok_or_else(|| serde::de::Error::missing_field("mu"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsSimilarityLmd {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("mu", &self.mu)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexSettingsSimilarityLmdType {
    #[doc = "`LMDirichlet`"]
    Lmdirichlet,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexSettingsSimilarityLmdType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexSettingsSimilarityLmdType::Lmdirichlet => "LMDirichlet",
            IndexSettingsSimilarityLmdType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexSettingsSimilarityLmdType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSimilarityLmdType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "LMDirichlet" => IndexSettingsSimilarityLmdType::Lmdirichlet,
            _ => IndexSettingsSimilarityLmdType::Unknown(s),
        })
    }
}

#[doc = "The LM Jelinek-Mercer similarity algorithm configuration."]
#[derive(Debug, Clone)]
pub struct IndexSettingsSimilarityLmj {
    #[doc = "The optimal mixture parameter."]
    pub lambda: f64,
    pub r#type: crate::types::indices_common::IndexSettingsSimilarityLmjType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSimilarityLmj {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsSimilarityLmj;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsSimilarityLmj object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_lambda: Option<f64> = None;
                let mut f_type: Option<
                    crate::types::indices_common::IndexSettingsSimilarityLmjType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "lambda" => {
                            if f_lambda.is_some() {
                                return Err(serde::de::Error::duplicate_field("lambda"));
                            }
                            f_lambda = Some(map.next_value::<f64>()?);
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::indices_common::IndexSettingsSimilarityLmjType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsSimilarityLmj {
                    lambda: f_lambda.ok_or_else(|| serde::de::Error::missing_field("lambda"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsSimilarityLmj {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("lambda", &self.lambda)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexSettingsSimilarityLmjType {
    #[doc = "`LMJelinekMercer`"]
    LmjelinekMercer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexSettingsSimilarityLmjType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexSettingsSimilarityLmjType::LmjelinekMercer => "LMJelinekMercer",
            IndexSettingsSimilarityLmjType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexSettingsSimilarityLmjType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSimilarityLmjType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "LMJelinekMercer" => IndexSettingsSimilarityLmjType::LmjelinekMercer,
            _ => IndexSettingsSimilarityLmjType::Unknown(s),
        })
    }
}

#[doc = "The scripted TF/IDF similarity algorithm configuration."]
#[derive(Debug, Clone)]
pub struct IndexSettingsSimilarityScriptedTfidf {
    pub script: crate::types::common::Script,
    pub r#type: crate::types::indices_common::IndexSettingsSimilarityScriptedTfidfType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSimilarityScriptedTfidf {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsSimilarityScriptedTfidf;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsSimilarityScriptedTfidf object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_script: Option<crate::types::common::Script> = None;
                let mut f_type: Option<
                    crate::types::indices_common::IndexSettingsSimilarityScriptedTfidfType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "script" => {
                            if f_script.is_some() {
                                return Err(serde::de::Error::duplicate_field("script"));
                            }
                            f_script = Some(map.next_value::<crate::types::common::Script>()?);
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::indices_common::IndexSettingsSimilarityScriptedTfidfType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsSimilarityScriptedTfidf {
                    script: f_script.ok_or_else(|| serde::de::Error::missing_field("script"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsSimilarityScriptedTfidf {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("script", &self.script)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexSettingsSimilarityScriptedTfidfType {
    #[doc = "`scripted`"]
    Scripted,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexSettingsSimilarityScriptedTfidfType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexSettingsSimilarityScriptedTfidfType::Scripted => "scripted",
            IndexSettingsSimilarityScriptedTfidfType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexSettingsSimilarityScriptedTfidfType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSimilarityScriptedTfidfType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "scripted" => IndexSettingsSimilarityScriptedTfidfType::Scripted,
            _ => IndexSettingsSimilarityScriptedTfidfType::Unknown(s),
        })
    }
}

#[doc = "The star tree indexing configuration."]
#[derive(Debug, Clone)]
pub struct IndexSettingsStarTree {
    pub default: Option<crate::types::indices_common::IndexSettingsStarTreeDefault>,
    pub field: Option<crate::types::indices_common::IndexSettingsStarTreeField>,
    pub max_fields: Option<crate::types::common::StringifiedInteger>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsStarTree {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsStarTree;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsStarTree object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_default: Option<
                    crate::types::indices_common::IndexSettingsStarTreeDefault,
                > = None;
                let mut seen_default = false;
                let mut f_field: Option<crate::types::indices_common::IndexSettingsStarTreeField> =
                    None;
                let mut seen_field = false;
                let mut f_max_fields: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_max_fields = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "default" => {
                            if seen_default {
                                return Err(serde::de::Error::duplicate_field("default"));
                            }
                            seen_default = true;
                            f_default = map.next_value::<Option<
                                crate::types::indices_common::IndexSettingsStarTreeDefault,
                            >>()?;
                        }
                        "field" => {
                            if seen_field {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            seen_field = true;
                            f_field =
                                map.next_value::<Option<
                                    crate::types::indices_common::IndexSettingsStarTreeField,
                                >>()?;
                        }
                        "max_fields" => {
                            if seen_max_fields {
                                return Err(serde::de::Error::duplicate_field("max_fields"));
                            }
                            seen_max_fields = true;
                            f_max_fields = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsStarTree {
                    default: f_default,
                    field: f_field,
                    max_fields: f_max_fields,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsStarTree {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.default {
            map.serialize_entry("default", v)?;
        }
        if let Some(v) = &self.field {
            map.serialize_entry("field", v)?;
        }
        if let Some(v) = &self.max_fields {
            map.serialize_entry("max_fields", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The default settings for star tree indexing."]
#[derive(Debug, Clone)]
pub struct IndexSettingsStarTreeDefault {
    pub max_leaf_docs: Option<crate::types::common::StringifiedInteger>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsStarTreeDefault {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsStarTreeDefault;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsStarTreeDefault object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_max_leaf_docs: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_max_leaf_docs = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "max_leaf_docs" => {
                            if seen_max_leaf_docs {
                                return Err(serde::de::Error::duplicate_field("max_leaf_docs"));
                            }
                            seen_max_leaf_docs = true;
                            f_max_leaf_docs = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsStarTreeDefault {
                    max_leaf_docs: f_max_leaf_docs,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsStarTreeDefault {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.max_leaf_docs {
            map.serialize_entry("max_leaf_docs", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The field-specific settings for star tree indexing."]
#[derive(Debug, Clone)]
pub struct IndexSettingsStarTreeField {
    pub default: Option<crate::types::indices_common::IndexSettingsStarTreeFieldDefault>,
    pub max_base_metrics: Option<crate::types::common::StringifiedInteger>,
    pub max_date_intervals: Option<crate::types::common::StringifiedInteger>,
    pub max_dimensions: Option<crate::types::common::StringifiedInteger>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsStarTreeField {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsStarTreeField;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsStarTreeField object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_default: Option<
                    crate::types::indices_common::IndexSettingsStarTreeFieldDefault,
                > = None;
                let mut seen_default = false;
                let mut f_max_base_metrics: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_max_base_metrics = false;
                let mut f_max_date_intervals: Option<crate::types::common::StringifiedInteger> =
                    None;
                let mut seen_max_date_intervals = false;
                let mut f_max_dimensions: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_max_dimensions = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "default" => {
                            if seen_default {
                                return Err(serde::de::Error::duplicate_field("default"));
                            }
                            seen_default = true;
                            f_default = map.next_value::<Option<
                                crate::types::indices_common::IndexSettingsStarTreeFieldDefault,
                            >>()?;
                        }
                        "max_base_metrics" => {
                            if seen_max_base_metrics {
                                return Err(serde::de::Error::duplicate_field("max_base_metrics"));
                            }
                            seen_max_base_metrics = true;
                            f_max_base_metrics = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_date_intervals" => {
                            if seen_max_date_intervals {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_date_intervals",
                                ));
                            }
                            seen_max_date_intervals = true;
                            f_max_date_intervals = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_dimensions" => {
                            if seen_max_dimensions {
                                return Err(serde::de::Error::duplicate_field("max_dimensions"));
                            }
                            seen_max_dimensions = true;
                            f_max_dimensions = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsStarTreeField {
                    default: f_default,
                    max_base_metrics: f_max_base_metrics,
                    max_date_intervals: f_max_date_intervals,
                    max_dimensions: f_max_dimensions,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsStarTreeField {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.default {
            map.serialize_entry("default", v)?;
        }
        if let Some(v) = &self.max_base_metrics {
            map.serialize_entry("max_base_metrics", v)?;
        }
        if let Some(v) = &self.max_date_intervals {
            map.serialize_entry("max_date_intervals", v)?;
        }
        if let Some(v) = &self.max_dimensions {
            map.serialize_entry("max_dimensions", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The field-specific settings for star tree indexing."]
#[derive(Debug, Clone)]
pub struct IndexSettingsStarTreeFieldDefault {
    #[doc = "The list of date intervals to use."]
    pub date_intervals: Option<Vec<String>>,
    #[doc = "The list of metrics to calculate."]
    pub metrics: Option<Vec<String>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsStarTreeFieldDefault {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsStarTreeFieldDefault;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsStarTreeFieldDefault object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_date_intervals: Option<Vec<String>> = None;
                let mut seen_date_intervals = false;
                let mut f_metrics: Option<Vec<String>> = None;
                let mut seen_metrics = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "date_intervals" => {
                            if seen_date_intervals {
                                return Err(serde::de::Error::duplicate_field("date_intervals"));
                            }
                            seen_date_intervals = true;
                            f_date_intervals = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "metrics" => {
                            if seen_metrics {
                                return Err(serde::de::Error::duplicate_field("metrics"));
                            }
                            seen_metrics = true;
                            f_metrics = map.next_value::<Option<Vec<String>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsStarTreeFieldDefault {
                    date_intervals: f_date_intervals,
                    metrics: f_metrics,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsStarTreeFieldDefault {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.date_intervals {
            map.serialize_entry("date_intervals", v)?;
        }
        if let Some(v) = &self.metrics {
            map.serialize_entry("metrics", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IndexSettingsStore {
    #[doc = "Whether memory-mapping is allowed. You can restrict the use of the `mmapfs` and the related `hybridfs` store types with the setting `node.store.allow_mmap`. This setting is useful, for example, if you are in an environment where you can not control the ability to create a lot of memory maps so you need disable the ability to use memory-mapping."]
    pub allow_mmap: Option<crate::types::common::StringifiedBoolean>,
    pub fs: Option<crate::types::indices_common::IndexSettingsStoreFs>,
    pub hybrid: Option<crate::types::indices_common::IndexSettingsStoreHybrid>,
    #[doc = "The list of files to preload into memory."]
    pub preload: Option<Vec<String>>,
    pub stats_refresh_interval: Option<crate::types::common::Duration>,
    pub r#type: crate::types::indices_common::StorageType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsStore {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsStore;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsStore object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_allow_mmap: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_allow_mmap = false;
                let mut f_fs: Option<crate::types::indices_common::IndexSettingsStoreFs> = None;
                let mut seen_fs = false;
                let mut f_hybrid: Option<crate::types::indices_common::IndexSettingsStoreHybrid> =
                    None;
                let mut seen_hybrid = false;
                let mut f_preload: Option<Vec<String>> = None;
                let mut seen_preload = false;
                let mut f_stats_refresh_interval: Option<crate::types::common::Duration> = None;
                let mut seen_stats_refresh_interval = false;
                let mut f_type: Option<crate::types::indices_common::StorageType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "allow_mmap" => {
                            if seen_allow_mmap {
                                return Err(serde::de::Error::duplicate_field("allow_mmap"));
                            }
                            seen_allow_mmap = true;
                            f_allow_mmap = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "fs" => {
                            if seen_fs {
                                return Err(serde::de::Error::duplicate_field("fs"));
                            }
                            seen_fs = true;
                            f_fs = map.next_value::<Option<crate::types::indices_common::IndexSettingsStoreFs>>()?;
                        }
                        "hybrid" => {
                            if seen_hybrid {
                                return Err(serde::de::Error::duplicate_field("hybrid"));
                            }
                            seen_hybrid = true;
                            f_hybrid = map.next_value::<Option<crate::types::indices_common::IndexSettingsStoreHybrid>>()?;
                        }
                        "preload" => {
                            if seen_preload {
                                return Err(serde::de::Error::duplicate_field("preload"));
                            }
                            seen_preload = true;
                            f_preload = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "stats_refresh_interval" => {
                            if seen_stats_refresh_interval {
                                return Err(serde::de::Error::duplicate_field(
                                    "stats_refresh_interval",
                                ));
                            }
                            seen_stats_refresh_interval = true;
                            f_stats_refresh_interval =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(
                                map.next_value::<crate::types::indices_common::StorageType>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsStore {
                    allow_mmap: f_allow_mmap,
                    fs: f_fs,
                    hybrid: f_hybrid,
                    preload: f_preload,
                    stats_refresh_interval: f_stats_refresh_interval,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsStore {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.allow_mmap {
            map.serialize_entry("allow_mmap", v)?;
        }
        if let Some(v) = &self.fs {
            map.serialize_entry("fs", v)?;
        }
        if let Some(v) = &self.hybrid {
            map.serialize_entry("hybrid", v)?;
        }
        if let Some(v) = &self.preload {
            map.serialize_entry("preload", v)?;
        }
        if let Some(v) = &self.stats_refresh_interval {
            map.serialize_entry("stats_refresh_interval", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The filesystem storage configuration."]
#[derive(Debug, Clone)]
pub struct IndexSettingsStoreFs {
    pub fs_lock: Option<crate::types::indices_common::IndexSettingsStoreFsLock>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsStoreFs {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsStoreFs;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsStoreFs object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_fs_lock: Option<crate::types::indices_common::IndexSettingsStoreFsLock> =
                    None;
                let mut seen_fs_lock = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "fs_lock" => {
                            if seen_fs_lock {
                                return Err(serde::de::Error::duplicate_field("fs_lock"));
                            }
                            seen_fs_lock = true;
                            f_fs_lock = map.next_value::<Option<crate::types::indices_common::IndexSettingsStoreFsLock>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsStoreFs {
                    fs_lock: f_fs_lock,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsStoreFs {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.fs_lock {
            map.serialize_entry("fs_lock", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The type of file system lock."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexSettingsStoreFsLock {
    #[doc = "`native`"]
    Native,
    #[doc = "`simple`"]
    Simple,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexSettingsStoreFsLock {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexSettingsStoreFsLock::Native => "native",
            IndexSettingsStoreFsLock::Simple => "simple",
            IndexSettingsStoreFsLock::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexSettingsStoreFsLock {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexSettingsStoreFsLock {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "native" => IndexSettingsStoreFsLock::Native,
            "simple" => IndexSettingsStoreFsLock::Simple,
            _ => IndexSettingsStoreFsLock::Unknown(s),
        })
    }
}

#[doc = "The hybrid storage type configuration."]
#[derive(Debug, Clone)]
pub struct IndexSettingsStoreHybrid {
    pub mmap: Option<crate::types::indices_common::IndexSettingsStoreHybridMmap>,
    pub nio: Option<crate::types::indices_common::IndexSettingsStoreHybridNio>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsStoreHybrid {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsStoreHybrid;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsStoreHybrid object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_mmap: Option<crate::types::indices_common::IndexSettingsStoreHybridMmap> =
                    None;
                let mut seen_mmap = false;
                let mut f_nio: Option<crate::types::indices_common::IndexSettingsStoreHybridNio> =
                    None;
                let mut seen_nio = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "mmap" => {
                            if seen_mmap {
                                return Err(serde::de::Error::duplicate_field("mmap"));
                            }
                            seen_mmap = true;
                            f_mmap = map.next_value::<Option<
                                crate::types::indices_common::IndexSettingsStoreHybridMmap,
                            >>()?;
                        }
                        "nio" => {
                            if seen_nio {
                                return Err(serde::de::Error::duplicate_field("nio"));
                            }
                            seen_nio = true;
                            f_nio =
                                map.next_value::<Option<
                                    crate::types::indices_common::IndexSettingsStoreHybridNio,
                                >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsStoreHybrid {
                    mmap: f_mmap,
                    nio: f_nio,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsStoreHybrid {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.mmap {
            map.serialize_entry("mmap", v)?;
        }
        if let Some(v) = &self.nio {
            map.serialize_entry("nio", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The memory-mapped storage configuration."]
#[derive(Debug, Clone)]
pub struct IndexSettingsStoreHybridMmap {
    #[doc = "The file extensions to use memory-mapping for."]
    pub extensions: Option<Vec<String>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsStoreHybridMmap {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsStoreHybridMmap;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsStoreHybridMmap object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_extensions: Option<Vec<String>> = None;
                let mut seen_extensions = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "extensions" => {
                            if seen_extensions {
                                return Err(serde::de::Error::duplicate_field("extensions"));
                            }
                            seen_extensions = true;
                            f_extensions = map.next_value::<Option<Vec<String>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsStoreHybridMmap {
                    extensions: f_extensions,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsStoreHybridMmap {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.extensions {
            map.serialize_entry("extensions", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The NIO storage configuration."]
#[derive(Debug, Clone)]
pub struct IndexSettingsStoreHybridNio {
    #[doc = "The file extensions to use NIO for."]
    pub extensions: Option<Vec<String>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettingsStoreHybridNio {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettingsStoreHybridNio;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettingsStoreHybridNio object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_extensions: Option<Vec<String>> = None;
                let mut seen_extensions = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "extensions" => {
                            if seen_extensions {
                                return Err(serde::de::Error::duplicate_field("extensions"));
                            }
                            seen_extensions = true;
                            f_extensions = map.next_value::<Option<Vec<String>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettingsStoreHybridNio {
                    extensions: f_extensions,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettingsStoreHybridNio {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.extensions {
            map.serialize_entry("extensions", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The state configuration of an index."]
#[derive(Debug, Clone)]
pub struct IndexState {
    #[doc = "The aliases associated with the index."]
    pub aliases: Option<std::collections::BTreeMap<String, crate::types::indices_common::Alias>>,
    pub data_stream: Option<crate::types::common::DataStreamName>,
    pub defaults: Option<crate::types::indices_common::IndexSettings>,
    pub mappings: Option<crate::types::common_mapping::TypeMapping>,
    pub settings: Option<crate::types::indices_common::IndexSettings>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexState;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexState object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_aliases: Option<
                    std::collections::BTreeMap<String, crate::types::indices_common::Alias>,
                > = None;
                let mut seen_aliases = false;
                let mut f_data_stream: Option<crate::types::common::DataStreamName> = None;
                let mut seen_data_stream = false;
                let mut f_defaults: Option<crate::types::indices_common::IndexSettings> = None;
                let mut seen_defaults = false;
                let mut f_mappings: Option<crate::types::common_mapping::TypeMapping> = None;
                let mut seen_mappings = false;
                let mut f_settings: Option<crate::types::indices_common::IndexSettings> = None;
                let mut seen_settings = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "aliases" => {
                            if seen_aliases {
                                return Err(serde::de::Error::duplicate_field("aliases"));
                            }
                            seen_aliases = true;
                            f_aliases = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::indices_common::Alias,
                                >,
                            >>()?;
                        }
                        "data_stream" => {
                            if seen_data_stream {
                                return Err(serde::de::Error::duplicate_field("data_stream"));
                            }
                            seen_data_stream = true;
                            f_data_stream =
                                map.next_value::<Option<crate::types::common::DataStreamName>>()?;
                        }
                        "defaults" => {
                            if seen_defaults {
                                return Err(serde::de::Error::duplicate_field("defaults"));
                            }
                            seen_defaults = true;
                            f_defaults = map
                                .next_value::<Option<crate::types::indices_common::IndexSettings>>(
                                )?;
                        }
                        "mappings" => {
                            if seen_mappings {
                                return Err(serde::de::Error::duplicate_field("mappings"));
                            }
                            seen_mappings = true;
                            f_mappings = map
                                .next_value::<Option<crate::types::common_mapping::TypeMapping>>(
                                )?;
                        }
                        "settings" => {
                            if seen_settings {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            seen_settings = true;
                            f_settings = map
                                .next_value::<Option<crate::types::indices_common::IndexSettings>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexState {
                    aliases: f_aliases,
                    data_stream: f_data_stream,
                    defaults: f_defaults,
                    mappings: f_mappings,
                    settings: f_settings,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.aliases {
            map.serialize_entry("aliases", v)?;
        }
        if let Some(v) = &self.data_stream {
            map.serialize_entry("data_stream", v)?;
        }
        if let Some(v) = &self.defaults {
            map.serialize_entry("defaults", v)?;
        }
        if let Some(v) = &self.mappings {
            map.serialize_entry("mappings", v)?;
        }
        if let Some(v) = &self.settings {
            map.serialize_entry("settings", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for an index template."]
#[derive(Debug, Clone)]
pub struct IndexTemplate {
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "Whether the template allows automatic index creation."]
    pub allow_auto_create: Option<bool>,
    #[doc = "An ordered list of component template names. Component templates are merged in the order specified, meaning that the last component template specified has the highest precedence."]
    pub composed_of: Option<Vec<crate::types::common::Name>>,
    pub data_stream: Option<crate::types::indices_common::IndexTemplateDataStreamConfiguration>,
    pub index_patterns: crate::types::common::Names,
    #[doc = "The priority to determine index template precedence when a new data stream or index is created. The index template with the highest priority is chosen. If no priority is specified the template is treated as though it is of priority 0 (lowest priority). This number is not automatically generated by OpenSearch."]
    pub priority: Option<i64>,
    pub template: Option<crate::types::indices_common::IndexTemplateSummary>,
    pub version: Option<crate::types::common::VersionNumber>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexTemplate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexTemplate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexTemplate object")
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
                let mut f_allow_auto_create: Option<bool> = None;
                let mut seen_allow_auto_create = false;
                let mut f_composed_of: Option<Vec<crate::types::common::Name>> = None;
                let mut seen_composed_of = false;
                let mut f_data_stream: Option<
                    crate::types::indices_common::IndexTemplateDataStreamConfiguration,
                > = None;
                let mut seen_data_stream = false;
                let mut f_index_patterns: Option<crate::types::common::Names> = None;
                let mut f_priority: Option<i64> = None;
                let mut seen_priority = false;
                let mut f_template: Option<crate::types::indices_common::IndexTemplateSummary> =
                    None;
                let mut seen_template = false;
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
                        "allow_auto_create" => {
                            if seen_allow_auto_create {
                                return Err(serde::de::Error::duplicate_field("allow_auto_create"));
                            }
                            seen_allow_auto_create = true;
                            f_allow_auto_create = map.next_value::<Option<bool>>()?;
                        }
                        "composed_of" => {
                            if seen_composed_of {
                                return Err(serde::de::Error::duplicate_field("composed_of"));
                            }
                            seen_composed_of = true;
                            f_composed_of =
                                map.next_value::<Option<Vec<crate::types::common::Name>>>()?;
                        }
                        "data_stream" => {
                            if seen_data_stream {
                                return Err(serde::de::Error::duplicate_field("data_stream"));
                            }
                            seen_data_stream = true;
                            f_data_stream = map.next_value::<Option<
                                crate::types::indices_common::IndexTemplateDataStreamConfiguration,
                            >>()?;
                        }
                        "index_patterns" => {
                            if f_index_patterns.is_some() {
                                return Err(serde::de::Error::duplicate_field("index_patterns"));
                            }
                            f_index_patterns =
                                Some(map.next_value::<crate::types::common::Names>()?);
                        }
                        "priority" => {
                            if seen_priority {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            seen_priority = true;
                            f_priority = map.next_value::<Option<i64>>()?;
                        }
                        "template" => {
                            if seen_template {
                                return Err(serde::de::Error::duplicate_field("template"));
                            }
                            seen_template = true;
                            f_template = map.next_value::<Option<crate::types::indices_common::IndexTemplateSummary>>()?;
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
                Ok(IndexTemplate {
                    meta: f_meta,
                    allow_auto_create: f_allow_auto_create,
                    composed_of: f_composed_of,
                    data_stream: f_data_stream,
                    index_patterns: f_index_patterns
                        .ok_or_else(|| serde::de::Error::missing_field("index_patterns"))?,
                    priority: f_priority,
                    template: f_template,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexTemplate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("_meta", v)?;
        }
        if let Some(v) = &self.allow_auto_create {
            map.serialize_entry("allow_auto_create", v)?;
        }
        if let Some(v) = &self.composed_of {
            map.serialize_entry("composed_of", v)?;
        }
        if let Some(v) = &self.data_stream {
            map.serialize_entry("data_stream", v)?;
        }
        map.serialize_entry("index_patterns", &self.index_patterns)?;
        if let Some(v) = &self.priority {
            map.serialize_entry("priority", v)?;
        }
        if let Some(v) = &self.template {
            map.serialize_entry("template", v)?;
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

#[doc = "The data stream configuration for an index template."]
#[derive(Debug, Clone)]
pub struct IndexTemplateDataStreamConfiguration {
    #[doc = "When `true`, the data stream supports custom routing."]
    pub allow_custom_routing: Option<bool>,
    #[doc = "When `true`, the data stream is hidden."]
    pub hidden: Option<bool>,
    pub timestamp_field: Option<crate::types::indices_common::DataStreamTimestampField>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexTemplateDataStreamConfiguration {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexTemplateDataStreamConfiguration;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexTemplateDataStreamConfiguration object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_allow_custom_routing: Option<bool> = None;
                let mut seen_allow_custom_routing = false;
                let mut f_hidden: Option<bool> = None;
                let mut seen_hidden = false;
                let mut f_timestamp_field: Option<
                    crate::types::indices_common::DataStreamTimestampField,
                > = None;
                let mut seen_timestamp_field = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "allow_custom_routing" => {
                            if seen_allow_custom_routing {
                                return Err(serde::de::Error::duplicate_field(
                                    "allow_custom_routing",
                                ));
                            }
                            seen_allow_custom_routing = true;
                            f_allow_custom_routing = map.next_value::<Option<bool>>()?;
                        }
                        "hidden" => {
                            if seen_hidden {
                                return Err(serde::de::Error::duplicate_field("hidden"));
                            }
                            seen_hidden = true;
                            f_hidden = map.next_value::<Option<bool>>()?;
                        }
                        "timestamp_field" => {
                            if seen_timestamp_field {
                                return Err(serde::de::Error::duplicate_field("timestamp_field"));
                            }
                            seen_timestamp_field = true;
                            f_timestamp_field = map.next_value::<Option<crate::types::indices_common::DataStreamTimestampField>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexTemplateDataStreamConfiguration {
                    allow_custom_routing: f_allow_custom_routing,
                    hidden: f_hidden,
                    timestamp_field: f_timestamp_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexTemplateDataStreamConfiguration {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.allow_custom_routing {
            map.serialize_entry("allow_custom_routing", v)?;
        }
        if let Some(v) = &self.hidden {
            map.serialize_entry("hidden", v)?;
        }
        if let Some(v) = &self.timestamp_field {
            map.serialize_entry("timestamp_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The summary configuration for an index template."]
#[derive(Debug, Clone)]
pub struct IndexTemplateSummary {
    #[doc = "The aliases to add. If the index template includes a `data_stream` object, these are data stream aliases. Otherwise, these are index aliases. Data stream aliases ignore the `index_routing`, `routing`, and `search_routing` options."]
    pub aliases: Option<std::collections::BTreeMap<String, crate::types::indices_common::Alias>>,
    pub mappings: Option<crate::types::common_mapping::TypeMapping>,
    pub settings: Option<crate::types::indices_common::IndexSettings>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexTemplateSummary {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexTemplateSummary;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexTemplateSummary object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_aliases: Option<
                    std::collections::BTreeMap<String, crate::types::indices_common::Alias>,
                > = None;
                let mut seen_aliases = false;
                let mut f_mappings: Option<crate::types::common_mapping::TypeMapping> = None;
                let mut seen_mappings = false;
                let mut f_settings: Option<crate::types::indices_common::IndexSettings> = None;
                let mut seen_settings = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "aliases" => {
                            if seen_aliases {
                                return Err(serde::de::Error::duplicate_field("aliases"));
                            }
                            seen_aliases = true;
                            f_aliases = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::indices_common::Alias,
                                >,
                            >>()?;
                        }
                        "mappings" => {
                            if seen_mappings {
                                return Err(serde::de::Error::duplicate_field("mappings"));
                            }
                            seen_mappings = true;
                            f_mappings = map
                                .next_value::<Option<crate::types::common_mapping::TypeMapping>>(
                                )?;
                        }
                        "settings" => {
                            if seen_settings {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            seen_settings = true;
                            f_settings = map
                                .next_value::<Option<crate::types::indices_common::IndexSettings>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexTemplateSummary {
                    aliases: f_aliases,
                    mappings: f_mappings,
                    settings: f_settings,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexTemplateSummary {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.aliases {
            map.serialize_entry("aliases", v)?;
        }
        if let Some(v) = &self.mappings {
            map.serialize_entry("mappings", v)?;
        }
        if let Some(v) = &self.settings {
            map.serialize_entry("settings", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The versioning information for the index."]
#[derive(Debug, Clone)]
pub struct IndexVersioning {
    pub created: Option<crate::types::common::VersionString>,
    #[doc = "The string representation of the version when the index was created."]
    pub created_string: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexVersioning {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexVersioning;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexVersioning object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_created: Option<crate::types::common::VersionString> = None;
                let mut seen_created = false;
                let mut f_created_string: Option<String> = None;
                let mut seen_created_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "created" => {
                            if seen_created {
                                return Err(serde::de::Error::duplicate_field("created"));
                            }
                            seen_created = true;
                            f_created =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "created_string" => {
                            if seen_created_string {
                                return Err(serde::de::Error::duplicate_field("created_string"));
                            }
                            seen_created_string = true;
                            f_created_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexVersioning {
                    created: f_created,
                    created_string: f_created_string,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexVersioning {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.created {
            map.serialize_entry("created", v)?;
        }
        if let Some(v) = &self.created_string {
            map.serialize_entry("created_string", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for indexing backpressure."]
#[derive(Debug, Clone)]
pub struct IndexingPressure {
    pub memory: crate::types::indices_common::IndexingPressureMemory,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexingPressure {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexingPressure;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexingPressure object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_memory: Option<crate::types::indices_common::IndexingPressureMemory> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "memory" => {
                            if f_memory.is_some() {
                                return Err(serde::de::Error::duplicate_field("memory"));
                            }
                            f_memory = Some(map.next_value::<crate::types::indices_common::IndexingPressureMemory>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexingPressure {
                    memory: f_memory.ok_or_else(|| serde::de::Error::missing_field("memory"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexingPressure {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("memory", &self.memory)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The memory-related settings for indexing backpressure."]
#[derive(Debug, Clone)]
pub struct IndexingPressureMemory {
    #[doc = "The number of outstanding bytes that may be consumed by indexing requests. When this limit is reached or exceeded, the node will reject new coordinating and primary operations. When replica operations consume 1.5x this limit, the node will reject new replica operations. Defaults to 10% of the heap."]
    pub limit: Option<crate::types::indices_common::IndexingPressureMemoryLimit>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexingPressureMemory {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexingPressureMemory;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexingPressureMemory object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_limit: Option<crate::types::indices_common::IndexingPressureMemoryLimit> =
                    None;
                let mut seen_limit = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "limit" => {
                            if seen_limit {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            seen_limit = true;
                            f_limit =
                                map.next_value::<Option<
                                    crate::types::indices_common::IndexingPressureMemoryLimit,
                                >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexingPressureMemory {
                    limit: f_limit,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexingPressureMemory {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.limit {
            map.serialize_entry("limit", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The number of outstanding bytes that may be consumed by indexing requests. When this limit is reached or exceeded, the node will reject new coordinating and primary operations. When replica operations consume 1.5x this limit, the node will reject new replica operations. Defaults to 10% of the heap."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum IndexingPressureMemoryLimit {
    String(String),
    Integer(i64),
}
impl<'de> serde::Deserialize<'de> for IndexingPressureMemoryLimit {
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
            Some(b'"') => serde_json::from_str(text).map(IndexingPressureMemoryLimit::String),
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(IndexingPressureMemoryLimit::Integer)
            }
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a IndexingPressureMemoryLimit value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The configuration for indexing slow log."]
#[derive(Debug, Clone)]
pub struct IndexingSlowlog {
    #[doc = "The log level for slow indexing operations."]
    pub level: Option<String>,
    pub reformat: Option<crate::types::common::StringifiedBoolean>,
    pub source: Option<crate::types::common::StringifiedInteger>,
    pub threshold: Option<crate::types::indices_common::IndexingSlowlogThresholds>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexingSlowlog {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexingSlowlog;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexingSlowlog object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_level: Option<String> = None;
                let mut seen_level = false;
                let mut f_reformat: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_reformat = false;
                let mut f_source: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_source = false;
                let mut f_threshold: Option<
                    crate::types::indices_common::IndexingSlowlogThresholds,
                > = None;
                let mut seen_threshold = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "level" => {
                            if seen_level {
                                return Err(serde::de::Error::duplicate_field("level"));
                            }
                            seen_level = true;
                            f_level = map.next_value::<Option<String>>()?;
                        }
                        "reformat" => {
                            if seen_reformat {
                                return Err(serde::de::Error::duplicate_field("reformat"));
                            }
                            seen_reformat = true;
                            f_reformat = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "source" => {
                            if seen_source {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            seen_source = true;
                            f_source = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "threshold" => {
                            if seen_threshold {
                                return Err(serde::de::Error::duplicate_field("threshold"));
                            }
                            seen_threshold = true;
                            f_threshold = map.next_value::<Option<crate::types::indices_common::IndexingSlowlogThresholds>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexingSlowlog {
                    level: f_level,
                    reformat: f_reformat,
                    source: f_source,
                    threshold: f_threshold,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexingSlowlog {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.level {
            map.serialize_entry("level", v)?;
        }
        if let Some(v) = &self.reformat {
            map.serialize_entry("reformat", v)?;
        }
        if let Some(v) = &self.source {
            map.serialize_entry("source", v)?;
        }
        if let Some(v) = &self.threshold {
            map.serialize_entry("threshold", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The threshold configuration for indexing slow log."]
#[derive(Debug, Clone)]
pub struct IndexingSlowlogThresholds {
    pub index: Option<crate::types::indices_common::SlowlogThresholdLevels>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexingSlowlogThresholds {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexingSlowlogThresholds;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexingSlowlogThresholds object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_index: Option<crate::types::indices_common::SlowlogThresholdLevels> =
                    None;
                let mut seen_index = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<crate::types::indices_common::SlowlogThresholdLevels>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexingSlowlogThresholds {
                    index: f_index,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexingSlowlogThresholds {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The pull-based ingestion settings controlling how data is read from streaming sources."]
#[derive(Debug, Clone)]
pub struct IngestionSource {
    #[doc = "Indicates if all-active ingestion is enabled for this index."]
    pub all_active: Option<crate::types::common::StringifiedBoolean>,
    #[doc = "Defines the policy for error handling."]
    pub error_strategy: Option<crate::types::ingestion_common::ErrorPolicy>,
    #[doc = "Defines the size of the internal blocking queue in pull-based ingestion."]
    pub internal_queue_size: Option<crate::types::common::StringifiedInteger>,
    #[doc = "Defines the number of processor or writer threads."]
    pub num_processor_threads: Option<crate::types::common::StringifiedInteger>,
    #[doc = "Custom parameters for the ingestion source."]
    pub param: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub pointer: Option<crate::types::indices_common::IngestionSourcePointer>,
    pub pointer_init_reset: Option<crate::types::indices_common::IngestionSourcePointerInitReset>,
    #[doc = "Specifies the offset value or timestamp in milliseconds."]
    pub pointer_init_reset_value: Option<String>,
    pub poll: Option<crate::types::indices_common::IngestionSourcePoll>,
    #[doc = "The maximum number of records to retrieve in each poll operation."]
    pub poll_max_batch_size: Option<crate::types::common::StringifiedLong>,
    #[doc = "Determines the Poll timeout value."]
    pub poll_timeout: Option<crate::types::common::StringifiedInteger>,
    #[doc = "The streaming source type."]
    pub r#type: Option<crate::types::indices_common::IngestionSourceType>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IngestionSource {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IngestionSource;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IngestionSource object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_all_active: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_all_active = false;
                let mut f_error_strategy: Option<crate::types::ingestion_common::ErrorPolicy> =
                    None;
                let mut seen_error_strategy = false;
                let mut f_internal_queue_size: Option<crate::types::common::StringifiedInteger> =
                    None;
                let mut seen_internal_queue_size = false;
                let mut f_num_processor_threads: Option<crate::types::common::StringifiedInteger> =
                    None;
                let mut seen_num_processor_threads = false;
                let mut f_param: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_param = false;
                let mut f_pointer: Option<crate::types::indices_common::IngestionSourcePointer> =
                    None;
                let mut seen_pointer = false;
                let mut f_pointer_init_reset: Option<
                    crate::types::indices_common::IngestionSourcePointerInitReset,
                > = None;
                let mut seen_pointer_init_reset = false;
                let mut f_pointer_init_reset_value: Option<String> = None;
                let mut seen_pointer_init_reset_value = false;
                let mut f_poll: Option<crate::types::indices_common::IngestionSourcePoll> = None;
                let mut seen_poll = false;
                let mut f_poll_max_batch_size: Option<crate::types::common::StringifiedLong> = None;
                let mut seen_poll_max_batch_size = false;
                let mut f_poll_timeout: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_poll_timeout = false;
                let mut f_type: Option<crate::types::indices_common::IngestionSourceType> = None;
                let mut seen_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "all_active" => {
                            if seen_all_active {
                                return Err(serde::de::Error::duplicate_field("all_active"));
                            }
                            seen_all_active = true;
                            f_all_active = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "error_strategy" => {
                            if seen_error_strategy {
                                return Err(serde::de::Error::duplicate_field("error_strategy"));
                            }
                            seen_error_strategy = true;
                            f_error_strategy = map
                                .next_value::<Option<crate::types::ingestion_common::ErrorPolicy>>(
                                )?;
                        }
                        "internal_queue_size" => {
                            if seen_internal_queue_size {
                                return Err(serde::de::Error::duplicate_field(
                                    "internal_queue_size",
                                ));
                            }
                            seen_internal_queue_size = true;
                            f_internal_queue_size = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "num_processor_threads" => {
                            if seen_num_processor_threads {
                                return Err(serde::de::Error::duplicate_field(
                                    "num_processor_threads",
                                ));
                            }
                            seen_num_processor_threads = true;
                            f_num_processor_threads = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "param" => {
                            if seen_param {
                                return Err(serde::de::Error::duplicate_field("param"));
                            }
                            seen_param = true;
                            f_param = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "pointer" => {
                            if seen_pointer {
                                return Err(serde::de::Error::duplicate_field("pointer"));
                            }
                            seen_pointer = true;
                            f_pointer = map.next_value::<Option<crate::types::indices_common::IngestionSourcePointer>>()?;
                        }
                        "pointer.init.reset" => {
                            if seen_pointer_init_reset {
                                return Err(serde::de::Error::duplicate_field(
                                    "pointer.init.reset",
                                ));
                            }
                            seen_pointer_init_reset = true;
                            f_pointer_init_reset = map.next_value::<Option<
                                crate::types::indices_common::IngestionSourcePointerInitReset,
                            >>()?;
                        }
                        "pointer.init.reset.value" => {
                            if seen_pointer_init_reset_value {
                                return Err(serde::de::Error::duplicate_field(
                                    "pointer.init.reset.value",
                                ));
                            }
                            seen_pointer_init_reset_value = true;
                            f_pointer_init_reset_value = map.next_value::<Option<String>>()?;
                        }
                        "poll" => {
                            if seen_poll {
                                return Err(serde::de::Error::duplicate_field("poll"));
                            }
                            seen_poll = true;
                            f_poll = map.next_value::<Option<crate::types::indices_common::IngestionSourcePoll>>()?;
                        }
                        "poll.max_batch_size" => {
                            if seen_poll_max_batch_size {
                                return Err(serde::de::Error::duplicate_field(
                                    "poll.max_batch_size",
                                ));
                            }
                            seen_poll_max_batch_size = true;
                            f_poll_max_batch_size =
                                map.next_value::<Option<crate::types::common::StringifiedLong>>()?;
                        }
                        "poll.timeout" => {
                            if seen_poll_timeout {
                                return Err(serde::de::Error::duplicate_field("poll.timeout"));
                            }
                            seen_poll_timeout = true;
                            f_poll_timeout = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<crate::types::indices_common::IngestionSourceType>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IngestionSource {
                    all_active: f_all_active,
                    error_strategy: f_error_strategy,
                    internal_queue_size: f_internal_queue_size,
                    num_processor_threads: f_num_processor_threads,
                    param: f_param,
                    pointer: f_pointer,
                    pointer_init_reset: f_pointer_init_reset,
                    pointer_init_reset_value: f_pointer_init_reset_value,
                    poll: f_poll,
                    poll_max_batch_size: f_poll_max_batch_size,
                    poll_timeout: f_poll_timeout,
                    r#type: f_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IngestionSource {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.all_active {
            map.serialize_entry("all_active", v)?;
        }
        if let Some(v) = &self.error_strategy {
            map.serialize_entry("error_strategy", v)?;
        }
        if let Some(v) = &self.internal_queue_size {
            map.serialize_entry("internal_queue_size", v)?;
        }
        if let Some(v) = &self.num_processor_threads {
            map.serialize_entry("num_processor_threads", v)?;
        }
        if let Some(v) = &self.param {
            map.serialize_entry("param", v)?;
        }
        if let Some(v) = &self.pointer {
            map.serialize_entry("pointer", v)?;
        }
        if let Some(v) = &self.pointer_init_reset {
            map.serialize_entry("pointer.init.reset", v)?;
        }
        if let Some(v) = &self.pointer_init_reset_value {
            map.serialize_entry("pointer.init.reset.value", v)?;
        }
        if let Some(v) = &self.poll {
            map.serialize_entry("poll", v)?;
        }
        if let Some(v) = &self.poll_max_batch_size {
            map.serialize_entry("poll.max_batch_size", v)?;
        }
        if let Some(v) = &self.poll_timeout {
            map.serialize_entry("poll.timeout", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("type", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IngestionSourcePointer {
    pub init: Option<crate::types::indices_common::IngestionSourcePointerInit>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IngestionSourcePointer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IngestionSourcePointer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IngestionSourcePointer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_init: Option<crate::types::indices_common::IngestionSourcePointerInit> =
                    None;
                let mut seen_init = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "init" => {
                            if seen_init {
                                return Err(serde::de::Error::duplicate_field("init"));
                            }
                            seen_init = true;
                            f_init =
                                map.next_value::<Option<
                                    crate::types::indices_common::IngestionSourcePointerInit,
                                >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IngestionSourcePointer {
                    init: f_init,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IngestionSourcePointer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.init {
            map.serialize_entry("init", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IngestionSourcePointerInit {
    pub reset: Option<crate::types::indices_common::IngestionSourcePointerInitReset>,
    #[doc = "Specifies the offset value or timestamp in milliseconds."]
    pub reset_value: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IngestionSourcePointerInit {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IngestionSourcePointerInit;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IngestionSourcePointerInit object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_reset: Option<
                    crate::types::indices_common::IngestionSourcePointerInitReset,
                > = None;
                let mut seen_reset = false;
                let mut f_reset_value: Option<String> = None;
                let mut seen_reset_value = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "reset" => {
                            if seen_reset {
                                return Err(serde::de::Error::duplicate_field("reset"));
                            }
                            seen_reset = true;
                            f_reset = map.next_value::<Option<
                                crate::types::indices_common::IngestionSourcePointerInitReset,
                            >>()?;
                        }
                        "reset.value" => {
                            if seen_reset_value {
                                return Err(serde::de::Error::duplicate_field("reset.value"));
                            }
                            seen_reset_value = true;
                            f_reset_value = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IngestionSourcePointerInit {
                    reset: f_reset,
                    reset_value: f_reset_value,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IngestionSourcePointerInit {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.reset {
            map.serialize_entry("reset", v)?;
        }
        if let Some(v) = &self.reset_value {
            map.serialize_entry("reset.value", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Determines where to start reading from the stream."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IngestionSourcePointerInitReset {
    #[doc = "`EARLIEST` (also accepted: `earliest`)"]
    Earliest,
    #[doc = "`LATEST` (also accepted: `latest`)"]
    Latest,
    #[doc = "`NONE` (also accepted: `none`)"]
    None,
    #[doc = "`RESET_BY_OFFSET` (also accepted: `reset_by_offset`)"]
    ResetByOffset,
    #[doc = "`RESET_BY_TIMESTAMP` (also accepted: `reset_by_timestamp`)"]
    ResetByTimestamp,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IngestionSourcePointerInitReset {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IngestionSourcePointerInitReset::Earliest => "EARLIEST",
            IngestionSourcePointerInitReset::Latest => "LATEST",
            IngestionSourcePointerInitReset::None => "NONE",
            IngestionSourcePointerInitReset::ResetByOffset => "RESET_BY_OFFSET",
            IngestionSourcePointerInitReset::ResetByTimestamp => "RESET_BY_TIMESTAMP",
            IngestionSourcePointerInitReset::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IngestionSourcePointerInitReset {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IngestionSourcePointerInitReset {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "EARLIEST" | "earliest" => IngestionSourcePointerInitReset::Earliest,
            "LATEST" | "latest" => IngestionSourcePointerInitReset::Latest,
            "NONE" | "none" => IngestionSourcePointerInitReset::None,
            "RESET_BY_OFFSET" | "reset_by_offset" => IngestionSourcePointerInitReset::ResetByOffset,
            "RESET_BY_TIMESTAMP" | "reset_by_timestamp" => {
                IngestionSourcePointerInitReset::ResetByTimestamp
            }
            _ => IngestionSourcePointerInitReset::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct IngestionSourcePoll {
    #[doc = "The maximum number of records to retrieve in each poll operation."]
    pub max_batch_size: Option<crate::types::common::StringifiedLong>,
    #[doc = "Determines the Poll timeout value."]
    pub timeout: Option<crate::types::common::StringifiedInteger>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IngestionSourcePoll {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IngestionSourcePoll;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IngestionSourcePoll object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_max_batch_size: Option<crate::types::common::StringifiedLong> = None;
                let mut seen_max_batch_size = false;
                let mut f_timeout: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_timeout = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "max_batch_size" => {
                            if seen_max_batch_size {
                                return Err(serde::de::Error::duplicate_field("max_batch_size"));
                            }
                            seen_max_batch_size = true;
                            f_max_batch_size =
                                map.next_value::<Option<crate::types::common::StringifiedLong>>()?;
                        }
                        "timeout" => {
                            if seen_timeout {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            seen_timeout = true;
                            f_timeout = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IngestionSourcePoll {
                    max_batch_size: f_max_batch_size,
                    timeout: f_timeout,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IngestionSourcePoll {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.max_batch_size {
            map.serialize_entry("max_batch_size", v)?;
        }
        if let Some(v) = &self.timeout {
            map.serialize_entry("timeout", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IngestionSourceType {
    #[doc = "`file`"]
    File,
    #[doc = "`kafka`"]
    Kafka,
    #[doc = "`kinesis`"]
    Kinesis,
    #[doc = "`none`"]
    None,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IngestionSourceType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IngestionSourceType::File => "file",
            IngestionSourceType::Kafka => "kafka",
            IngestionSourceType::Kinesis => "kinesis",
            IngestionSourceType::None => "none",
            IngestionSourceType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IngestionSourceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IngestionSourceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "file" => IngestionSourceType::File,
            "kafka" => IngestionSourceType::Kafka,
            "kinesis" => IngestionSourceType::Kinesis,
            "none" => IngestionSourceType::None,
            _ => IngestionSourceType::Unknown(s),
        })
    }
}

#[doc = "The component managing the data stream or index."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ManagedBy {
    #[doc = "`Data stream lifecycle`"]
    DataStreamLifecycle,
    #[doc = "`Index Lifecycle Management`"]
    IndexLifecycleManagement,
    #[doc = "`Unmanaged`"]
    Unmanaged,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ManagedBy {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ManagedBy::DataStreamLifecycle => "Data stream lifecycle",
            ManagedBy::IndexLifecycleManagement => "Index Lifecycle Management",
            ManagedBy::Unmanaged => "Unmanaged",
            ManagedBy::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ManagedBy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ManagedBy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "Data stream lifecycle" => ManagedBy::DataStreamLifecycle,
            "Index Lifecycle Management" => ManagedBy::IndexLifecycleManagement,
            "Unmanaged" => ManagedBy::Unmanaged,
            _ => ManagedBy::Unknown(s),
        })
    }
}

#[doc = "The configuration for numeric field data."]
#[derive(Debug, Clone)]
pub struct NumericFielddata {
    pub format: crate::types::indices_common::NumericFielddataFormat,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NumericFielddata {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NumericFielddata;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NumericFielddata object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_format: Option<crate::types::indices_common::NumericFielddataFormat> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "format" => {
                            if f_format.is_some() {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            f_format = Some(map.next_value::<crate::types::indices_common::NumericFielddataFormat>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NumericFielddata {
                    format: f_format.ok_or_else(|| serde::de::Error::missing_field("format"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NumericFielddata {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("format", &self.format)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The format for numeric field data."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NumericFielddataFormat {
    #[doc = "`array`"]
    Array,
    #[doc = "`disabled`"]
    Disabled,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl NumericFielddataFormat {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            NumericFielddataFormat::Array => "array",
            NumericFielddataFormat::Disabled => "disabled",
            NumericFielddataFormat::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for NumericFielddataFormat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NumericFielddataFormat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "array" => NumericFielddataFormat::Array,
            "disabled" => NumericFielddataFormat::Disabled,
            _ => NumericFielddataFormat::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReplicationType {
    #[doc = "`DOCUMENT`"]
    Document,
    #[doc = "`SEGMENT`"]
    Segment,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ReplicationType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ReplicationType::Document => "DOCUMENT",
            ReplicationType::Segment => "SEGMENT",
            ReplicationType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ReplicationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReplicationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "DOCUMENT" => ReplicationType::Document,
            "SEGMENT" => ReplicationType::Segment,
            _ => ReplicationType::Unknown(s),
        })
    }
}

#[doc = "The retention lease configuration."]
#[derive(Debug, Clone)]
pub struct RetentionLease {
    pub period: crate::types::common::Duration,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RetentionLease {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RetentionLease;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RetentionLease object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_period: Option<crate::types::common::Duration> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "period" => {
                            if f_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            f_period = Some(map.next_value::<crate::types::common::Duration>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RetentionLease {
                    period: f_period.ok_or_else(|| serde::de::Error::missing_field("period"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RetentionLease {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("period", &self.period)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for search idle behavior."]
#[derive(Debug, Clone)]
pub struct SearchIdle {
    pub after: Option<crate::types::common::Duration>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SearchIdle {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SearchIdle;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchIdle object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_after: Option<crate::types::common::Duration> = None;
                let mut seen_after = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "after" => {
                            if seen_after {
                                return Err(serde::de::Error::duplicate_field("after"));
                            }
                            seen_after = true;
                            f_after = map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SearchIdle {
                    after: f_after,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SearchIdle {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.after {
            map.serialize_entry("after", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for search slow log."]
#[derive(Debug, Clone)]
pub struct SearchSlowlog {
    #[doc = "The log level for slow searches."]
    pub level: Option<String>,
    #[doc = "Whether to reformat the logged search source."]
    pub reformat: Option<bool>,
    pub threshold: Option<crate::types::indices_common::SearchSlowlogThresholds>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SearchSlowlog {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SearchSlowlog;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchSlowlog object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_level: Option<String> = None;
                let mut seen_level = false;
                let mut f_reformat: Option<bool> = None;
                let mut seen_reformat = false;
                let mut f_threshold: Option<crate::types::indices_common::SearchSlowlogThresholds> =
                    None;
                let mut seen_threshold = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "level" => {
                            if seen_level {
                                return Err(serde::de::Error::duplicate_field("level"));
                            }
                            seen_level = true;
                            f_level = map.next_value::<Option<String>>()?;
                        }
                        "reformat" => {
                            if seen_reformat {
                                return Err(serde::de::Error::duplicate_field("reformat"));
                            }
                            seen_reformat = true;
                            f_reformat = map.next_value::<Option<bool>>()?;
                        }
                        "threshold" => {
                            if seen_threshold {
                                return Err(serde::de::Error::duplicate_field("threshold"));
                            }
                            seen_threshold = true;
                            f_threshold = map.next_value::<Option<crate::types::indices_common::SearchSlowlogThresholds>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SearchSlowlog {
                    level: f_level,
                    reformat: f_reformat,
                    threshold: f_threshold,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SearchSlowlog {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.level {
            map.serialize_entry("level", v)?;
        }
        if let Some(v) = &self.reformat {
            map.serialize_entry("reformat", v)?;
        }
        if let Some(v) = &self.threshold {
            map.serialize_entry("threshold", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The threshold configuration for search slow log."]
#[derive(Debug, Clone)]
pub struct SearchSlowlogThresholds {
    pub fetch: Option<crate::types::indices_common::SlowlogThresholdLevels>,
    pub query: Option<crate::types::indices_common::SlowlogThresholdLevels>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SearchSlowlogThresholds {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SearchSlowlogThresholds;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchSlowlogThresholds object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_fetch: Option<crate::types::indices_common::SlowlogThresholdLevels> =
                    None;
                let mut seen_fetch = false;
                let mut f_query: Option<crate::types::indices_common::SlowlogThresholdLevels> =
                    None;
                let mut seen_query = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "fetch" => {
                            if seen_fetch {
                                return Err(serde::de::Error::duplicate_field("fetch"));
                            }
                            seen_fetch = true;
                            f_fetch = map.next_value::<Option<crate::types::indices_common::SlowlogThresholdLevels>>()?;
                        }
                        "query" => {
                            if seen_query {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            seen_query = true;
                            f_query = map.next_value::<Option<crate::types::indices_common::SlowlogThresholdLevels>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SearchSlowlogThresholds {
                    fetch: f_fetch,
                    query: f_query,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SearchSlowlogThresholds {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.fetch {
            map.serialize_entry("fetch", v)?;
        }
        if let Some(v) = &self.query {
            map.serialize_entry("query", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The handling of missing values in segment sorting."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentSortMissing {
    #[doc = "`_first`"]
    First,
    #[doc = "`_last`"]
    Last,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SegmentSortMissing {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SegmentSortMissing::First => "_first",
            SegmentSortMissing::Last => "_last",
            SegmentSortMissing::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SegmentSortMissing {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SegmentSortMissing {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "_first" => SegmentSortMissing::First,
            "_last" => SegmentSortMissing::Last,
            _ => SegmentSortMissing::Unknown(s),
        })
    }
}

#[doc = "The sort mode for segments."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentSortMode {
    #[doc = "`max`"]
    Max,
    #[doc = "`min`"]
    Min,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SegmentSortMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SegmentSortMode::Max => "max",
            SegmentSortMode::Min => "min",
            SegmentSortMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SegmentSortMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SegmentSortMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "max" => SegmentSortMode::Max,
            "min" => SegmentSortMode::Min,
            _ => SegmentSortMode::Unknown(s),
        })
    }
}

#[doc = "The sort order for segments."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentSortOrder {
    #[doc = "`asc`"]
    Asc,
    #[doc = "`desc`"]
    Desc,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SegmentSortOrder {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SegmentSortOrder::Asc => "asc",
            SegmentSortOrder::Desc => "desc",
            SegmentSortOrder::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SegmentSortOrder {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SegmentSortOrder {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "asc" => SegmentSortOrder::Asc,
            "desc" => SegmentSortOrder::Desc,
            _ => SegmentSortOrder::Unknown(s),
        })
    }
}

#[doc = "The threshold levels for slow log."]
#[derive(Debug, Clone)]
pub struct SlowlogThresholdLevels {
    pub debug: Option<crate::types::common::Duration>,
    pub info: Option<crate::types::common::Duration>,
    pub trace: Option<crate::types::common::Duration>,
    pub warn: Option<crate::types::common::Duration>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SlowlogThresholdLevels {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SlowlogThresholdLevels;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SlowlogThresholdLevels object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_debug: Option<crate::types::common::Duration> = None;
                let mut seen_debug = false;
                let mut f_info: Option<crate::types::common::Duration> = None;
                let mut seen_info = false;
                let mut f_trace: Option<crate::types::common::Duration> = None;
                let mut seen_trace = false;
                let mut f_warn: Option<crate::types::common::Duration> = None;
                let mut seen_warn = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "debug" => {
                            if seen_debug {
                                return Err(serde::de::Error::duplicate_field("debug"));
                            }
                            seen_debug = true;
                            f_debug = map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "info" => {
                            if seen_info {
                                return Err(serde::de::Error::duplicate_field("info"));
                            }
                            seen_info = true;
                            f_info = map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "trace" => {
                            if seen_trace {
                                return Err(serde::de::Error::duplicate_field("trace"));
                            }
                            seen_trace = true;
                            f_trace = map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "warn" => {
                            if seen_warn {
                                return Err(serde::de::Error::duplicate_field("warn"));
                            }
                            seen_warn = true;
                            f_warn = map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SlowlogThresholdLevels {
                    debug: f_debug,
                    info: f_info,
                    trace: f_trace,
                    warn: f_warn,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SlowlogThresholdLevels {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.debug {
            map.serialize_entry("debug", v)?;
        }
        if let Some(v) = &self.info {
            map.serialize_entry("info", v)?;
        }
        if let Some(v) = &self.trace {
            map.serialize_entry("trace", v)?;
        }
        if let Some(v) = &self.warn {
            map.serialize_entry("warn", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for soft deletes."]
#[derive(Debug, Clone)]
pub struct SoftDeletes {
    #[doc = "Whether soft deletes are enabled on the index."]
    pub enabled: Option<crate::types::common::StringifiedBoolean>,
    pub retention: Option<crate::types::indices_common::SoftDeletesRetention>,
    pub retention_lease: Option<crate::types::indices_common::RetentionLease>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SoftDeletes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SoftDeletes;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SoftDeletes object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_enabled: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_enabled = false;
                let mut f_retention: Option<crate::types::indices_common::SoftDeletesRetention> =
                    None;
                let mut seen_retention = false;
                let mut f_retention_lease: Option<crate::types::indices_common::RetentionLease> =
                    None;
                let mut seen_retention_lease = false;
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
                            f_enabled = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "retention" => {
                            if seen_retention {
                                return Err(serde::de::Error::duplicate_field("retention"));
                            }
                            seen_retention = true;
                            f_retention = map.next_value::<Option<crate::types::indices_common::SoftDeletesRetention>>()?;
                        }
                        "retention_lease" => {
                            if seen_retention_lease {
                                return Err(serde::de::Error::duplicate_field("retention_lease"));
                            }
                            seen_retention_lease = true;
                            f_retention_lease = map
                                .next_value::<Option<crate::types::indices_common::RetentionLease>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SoftDeletes {
                    enabled: f_enabled,
                    retention: f_retention,
                    retention_lease: f_retention_lease,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SoftDeletes {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.enabled {
            map.serialize_entry("enabled", v)?;
        }
        if let Some(v) = &self.retention {
            map.serialize_entry("retention", v)?;
        }
        if let Some(v) = &self.retention_lease {
            map.serialize_entry("retention_lease", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The retention configuration for soft deletes."]
#[derive(Debug, Clone)]
pub struct SoftDeletesRetention {
    pub operations: Option<crate::types::common::StringifiedLong>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SoftDeletesRetention {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SoftDeletesRetention;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SoftDeletesRetention object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_operations: Option<crate::types::common::StringifiedLong> = None;
                let mut seen_operations = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "operations" => {
                            if seen_operations {
                                return Err(serde::de::Error::duplicate_field("operations"));
                            }
                            seen_operations = true;
                            f_operations =
                                map.next_value::<Option<crate::types::common::StringifiedLong>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SoftDeletesRetention {
                    operations: f_operations,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SoftDeletesRetention {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.operations {
            map.serialize_entry("operations", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct StorageType {
    raw: Box<serde_json::value::RawValue>,
}
impl StorageType {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `builtin` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_builtin(
        &self,
    ) -> std::result::Result<
        crate::types::indices_common::BuiltinStorageType,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "builtin")
    }
    #[doc = "Parses the value as the `custom` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_custom(&self) -> std::result::Result<String, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "custom")
    }
}

#[doc = "The mapping configuration for a template."]
#[derive(Debug, Clone)]
pub struct TemplateMapping {
    pub aliases: std::collections::BTreeMap<String, crate::types::indices_common::Alias>,
    pub index_patterns: Vec<crate::types::common::Name>,
    pub mappings: crate::types::common_mapping::TypeMapping,
    #[doc = "The order in which the template should be applied."]
    pub order: i32,
    #[doc = "The index settings to apply."]
    pub settings: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
    pub version: Option<crate::types::common::VersionNumber>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TemplateMapping {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TemplateMapping;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TemplateMapping object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_aliases: Option<
                    std::collections::BTreeMap<String, crate::types::indices_common::Alias>,
                > = None;
                let mut f_index_patterns: Option<Vec<crate::types::common::Name>> = None;
                let mut f_mappings: Option<crate::types::common_mapping::TypeMapping> = None;
                let mut f_order: Option<i32> = None;
                let mut f_settings: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "aliases" => {
                            if f_aliases.is_some() {
                                return Err(serde::de::Error::duplicate_field("aliases"));
                            }
                            f_aliases = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                crate::types::indices_common::Alias,
                            >>()?);
                        }
                        "index_patterns" => {
                            if f_index_patterns.is_some() {
                                return Err(serde::de::Error::duplicate_field("index_patterns"));
                            }
                            f_index_patterns =
                                Some(map.next_value::<Vec<crate::types::common::Name>>()?);
                        }
                        "mappings" => {
                            if f_mappings.is_some() {
                                return Err(serde::de::Error::duplicate_field("mappings"));
                            }
                            f_mappings = Some(
                                map.next_value::<crate::types::common_mapping::TypeMapping>()?,
                            );
                        }
                        "order" => {
                            if f_order.is_some() {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            f_order = Some(map.next_value::<i32>()?);
                        }
                        "settings" => {
                            if f_settings.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            f_settings = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                Box<serde_json::value::RawValue>,
                            >>()?);
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
                Ok(TemplateMapping {
                    aliases: f_aliases.ok_or_else(|| serde::de::Error::missing_field("aliases"))?,
                    index_patterns: f_index_patterns
                        .ok_or_else(|| serde::de::Error::missing_field("index_patterns"))?,
                    mappings: f_mappings
                        .ok_or_else(|| serde::de::Error::missing_field("mappings"))?,
                    order: f_order.ok_or_else(|| serde::de::Error::missing_field("order"))?,
                    settings: f_settings
                        .ok_or_else(|| serde::de::Error::missing_field("settings"))?,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TemplateMapping {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("aliases", &self.aliases)?;
        map.serialize_entry("index_patterns", &self.index_patterns)?;
        map.serialize_entry("mappings", &self.mappings)?;
        map.serialize_entry("order", &self.order)?;
        map.serialize_entry("settings", &self.settings)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The translog configuration."]
#[derive(Debug, Clone)]
pub struct Translog {
    pub durability: Option<crate::types::indices_common::TranslogDurability>,
    pub flush_threshold_size: Option<crate::types::common::HumanReadableByteCount>,
    pub generation_threshold_size: Option<crate::types::common::HumanReadableByteCount>,
    pub retention: Option<crate::types::indices_common::TranslogRetention>,
    pub sync_interval: Option<crate::types::common::Duration>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Translog {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Translog;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Translog object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_durability: Option<crate::types::indices_common::TranslogDurability> =
                    None;
                let mut seen_durability = false;
                let mut f_flush_threshold_size: Option<
                    crate::types::common::HumanReadableByteCount,
                > = None;
                let mut seen_flush_threshold_size = false;
                let mut f_generation_threshold_size: Option<
                    crate::types::common::HumanReadableByteCount,
                > = None;
                let mut seen_generation_threshold_size = false;
                let mut f_retention: Option<crate::types::indices_common::TranslogRetention> = None;
                let mut seen_retention = false;
                let mut f_sync_interval: Option<crate::types::common::Duration> = None;
                let mut seen_sync_interval = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "durability" => {
                            if seen_durability {
                                return Err(serde::de::Error::duplicate_field("durability"));
                            }
                            seen_durability = true;
                            f_durability = map.next_value::<Option<crate::types::indices_common::TranslogDurability>>()?;
                        }
                        "flush_threshold_size" => {
                            if seen_flush_threshold_size {
                                return Err(serde::de::Error::duplicate_field(
                                    "flush_threshold_size",
                                ));
                            }
                            seen_flush_threshold_size = true;
                            f_flush_threshold_size = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "generation_threshold_size" => {
                            if seen_generation_threshold_size {
                                return Err(serde::de::Error::duplicate_field(
                                    "generation_threshold_size",
                                ));
                            }
                            seen_generation_threshold_size = true;
                            f_generation_threshold_size = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "retention" => {
                            if seen_retention {
                                return Err(serde::de::Error::duplicate_field("retention"));
                            }
                            seen_retention = true;
                            f_retention = map.next_value::<Option<crate::types::indices_common::TranslogRetention>>()?;
                        }
                        "sync_interval" => {
                            if seen_sync_interval {
                                return Err(serde::de::Error::duplicate_field("sync_interval"));
                            }
                            seen_sync_interval = true;
                            f_sync_interval =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Translog {
                    durability: f_durability,
                    flush_threshold_size: f_flush_threshold_size,
                    generation_threshold_size: f_generation_threshold_size,
                    retention: f_retention,
                    sync_interval: f_sync_interval,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Translog {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.durability {
            map.serialize_entry("durability", v)?;
        }
        if let Some(v) = &self.flush_threshold_size {
            map.serialize_entry("flush_threshold_size", v)?;
        }
        if let Some(v) = &self.generation_threshold_size {
            map.serialize_entry("generation_threshold_size", v)?;
        }
        if let Some(v) = &self.retention {
            map.serialize_entry("retention", v)?;
        }
        if let Some(v) = &self.sync_interval {
            map.serialize_entry("sync_interval", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The durability settings for the translog."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TranslogDurability {
    #[doc = "`ASYNC` (also accepted: `async`)"]
    Async,
    #[doc = "`REQUEST` (also accepted: `request`)"]
    Request,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TranslogDurability {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TranslogDurability::Async => "ASYNC",
            TranslogDurability::Request => "REQUEST",
            TranslogDurability::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TranslogDurability {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TranslogDurability {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ASYNC" | "async" => TranslogDurability::Async,
            "REQUEST" | "request" => TranslogDurability::Request,
            _ => TranslogDurability::Unknown(s),
        })
    }
}

#[doc = "The retention configuration for the translog."]
#[derive(Debug, Clone)]
pub struct TranslogRetention {
    pub age: Option<crate::types::common::Duration>,
    pub size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TranslogRetention {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TranslogRetention;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TranslogRetention object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_age: Option<crate::types::common::Duration> = None;
                let mut seen_age = false;
                let mut f_size: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_size = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "age" => {
                            if seen_age {
                                return Err(serde::de::Error::duplicate_field("age"));
                            }
                            seen_age = true;
                            f_age = map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "size" => {
                            if seen_size {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            seen_size = true;
                            f_size = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TranslogRetention {
                    age: f_age,
                    size: f_size,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TranslogRetention {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.age {
            map.serialize_entry("age", v)?;
        }
        if let Some(v) = &self.size {
            map.serialize_entry("size", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The upgrade status details for an index."]
#[derive(Debug, Clone)]
pub struct UpgradeStatus {
    pub size_in_bytes: Option<crate::types::common::ByteCount>,
    pub size_to_upgrade_ancient_in_bytes: Option<crate::types::common::ByteCount>,
    pub size_to_upgrade_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UpgradeStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UpgradeStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UpgradeStatus object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_size_in_bytes = false;
                let mut f_size_to_upgrade_ancient_in_bytes: Option<
                    crate::types::common::ByteCount,
                > = None;
                let mut seen_size_to_upgrade_ancient_in_bytes = false;
                let mut f_size_to_upgrade_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_size_to_upgrade_in_bytes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "size_in_bytes" => {
                            if seen_size_in_bytes {
                                return Err(serde::de::Error::duplicate_field("size_in_bytes"));
                            }
                            seen_size_in_bytes = true;
                            f_size_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "size_to_upgrade_ancient_in_bytes" => {
                            if seen_size_to_upgrade_ancient_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "size_to_upgrade_ancient_in_bytes",
                                ));
                            }
                            seen_size_to_upgrade_ancient_in_bytes = true;
                            f_size_to_upgrade_ancient_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "size_to_upgrade_in_bytes" => {
                            if seen_size_to_upgrade_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "size_to_upgrade_in_bytes",
                                ));
                            }
                            seen_size_to_upgrade_in_bytes = true;
                            f_size_to_upgrade_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UpgradeStatus {
                    size_in_bytes: f_size_in_bytes,
                    size_to_upgrade_ancient_in_bytes: f_size_to_upgrade_ancient_in_bytes,
                    size_to_upgrade_in_bytes: f_size_to_upgrade_in_bytes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UpgradeStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.size_in_bytes {
            map.serialize_entry("size_in_bytes", v)?;
        }
        if let Some(v) = &self.size_to_upgrade_ancient_in_bytes {
            map.serialize_entry("size_to_upgrade_ancient_in_bytes", v)?;
        }
        if let Some(v) = &self.size_to_upgrade_in_bytes {
            map.serialize_entry("size_to_upgrade_in_bytes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The version status for an upgrade operation."]
#[derive(Debug, Clone)]
pub struct UpgradeVersionStatus {
    pub oldest_lucene_segment_version: Option<crate::types::common::VersionString>,
    pub upgrade_version: Option<crate::types::common::VersionString>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UpgradeVersionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UpgradeVersionStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UpgradeVersionStatus object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_oldest_lucene_segment_version: Option<
                    crate::types::common::VersionString,
                > = None;
                let mut seen_oldest_lucene_segment_version = false;
                let mut f_upgrade_version: Option<crate::types::common::VersionString> = None;
                let mut seen_upgrade_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "oldest_lucene_segment_version" => {
                            if seen_oldest_lucene_segment_version {
                                return Err(serde::de::Error::duplicate_field(
                                    "oldest_lucene_segment_version",
                                ));
                            }
                            seen_oldest_lucene_segment_version = true;
                            f_oldest_lucene_segment_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "upgrade_version" => {
                            if seen_upgrade_version {
                                return Err(serde::de::Error::duplicate_field("upgrade_version"));
                            }
                            seen_upgrade_version = true;
                            f_upgrade_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UpgradeVersionStatus {
                    oldest_lucene_segment_version: f_oldest_lucene_segment_version,
                    upgrade_version: f_upgrade_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UpgradeVersionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.oldest_lucene_segment_version {
            map.serialize_entry("oldest_lucene_segment_version", v)?;
        }
        if let Some(v) = &self.upgrade_version {
            map.serialize_entry("upgrade_version", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
