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

//! Response types of the `search_shards` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct SearchShardsResponse {
    pub indices: std::collections::BTreeMap<String, crate::types::search_shards::ShardStoreIndex>,
    pub nodes: std::collections::BTreeMap<String, crate::types::common::NodeAttributes>,
    pub shards: Vec<Vec<crate::types::common::NodeShard>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SearchShardsResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SearchShardsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchShardsResponse object")
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
                    std::collections::BTreeMap<
                        String,
                        crate::types::search_shards::ShardStoreIndex,
                    >,
                > = None;
                let mut f_nodes: Option<
                    std::collections::BTreeMap<String, crate::types::common::NodeAttributes>,
                > = None;
                let mut f_shards: Option<Vec<Vec<crate::types::common::NodeShard>>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "indices" => {
                            if f_indices.is_some() {
                                return Err(serde::de::Error::duplicate_field("indices"));
                            }
                            f_indices = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                crate::types::search_shards::ShardStoreIndex,
                            >>()?);
                        }
                        "nodes" => {
                            if f_nodes.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            f_nodes = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                crate::types::common::NodeAttributes,
                            >>()?);
                        }
                        "shards" => {
                            if f_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("shards"));
                            }
                            f_shards = Some(
                                map.next_value::<Vec<Vec<crate::types::common::NodeShard>>>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SearchShardsResponse {
                    indices: f_indices.ok_or_else(|| serde::de::Error::missing_field("indices"))?,
                    nodes: f_nodes.ok_or_else(|| serde::de::Error::missing_field("nodes"))?,
                    shards: f_shards.ok_or_else(|| serde::de::Error::missing_field("shards"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SearchShardsResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("indices", &self.indices)?;
        map.serialize_entry("nodes", &self.nodes)?;
        map.serialize_entry("shards", &self.shards)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardStoreIndex {
    pub aliases: Option<Vec<crate::types::common::Name>>,
    pub filter: Option<crate::types::common_query_dsl::QueryContainer>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardStoreIndex {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardStoreIndex;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardStoreIndex object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_aliases: Option<Vec<crate::types::common::Name>> = None;
                let mut seen_aliases = false;
                let mut f_filter: Option<crate::types::common_query_dsl::QueryContainer> = None;
                let mut seen_filter = false;
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
                            f_aliases =
                                map.next_value::<Option<Vec<crate::types::common::Name>>>()?;
                        }
                        "filter" => {
                            if seen_filter {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            seen_filter = true;
                            f_filter = map.next_value::<Option<crate::types::common_query_dsl::QueryContainer>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardStoreIndex {
                    aliases: f_aliases,
                    filter: f_filter,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardStoreIndex {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.aliases {
            map.serialize_entry("aliases", v)?;
        }
        if let Some(v) = &self.filter {
            map.serialize_entry("filter", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
