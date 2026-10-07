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

//! Response types of the `cat_aliases` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct AliasesRecord {
    #[doc = "alias name"]
    pub alias: Option<String>,
    #[doc = "filter"]
    pub filter: Option<String>,
    pub index: Option<crate::types::common::IndexName>,
    #[doc = "write index"]
    pub is_write_index: Option<String>,
    #[doc = "index routing"]
    pub routing_index: Option<String>,
    #[doc = "search routing"]
    pub routing_search: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AliasesRecord {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AliasesRecord;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AliasesRecord object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_alias: Option<String> = None;
                let mut seen_alias = false;
                let mut f_filter: Option<String> = None;
                let mut seen_filter = false;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut seen_index = false;
                let mut f_is_write_index: Option<String> = None;
                let mut seen_is_write_index = false;
                let mut f_routing_index: Option<String> = None;
                let mut seen_routing_index = false;
                let mut f_routing_search: Option<String> = None;
                let mut seen_routing_search = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "alias" => {
                            if seen_alias {
                                return Err(serde::de::Error::duplicate_field("alias"));
                            }
                            seen_alias = true;
                            f_alias = map.next_value::<Option<String>>()?;
                        }
                        "filter" => {
                            if seen_filter {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            seen_filter = true;
                            f_filter = map.next_value::<Option<String>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "is_write_index" => {
                            if seen_is_write_index {
                                return Err(serde::de::Error::duplicate_field("is_write_index"));
                            }
                            seen_is_write_index = true;
                            f_is_write_index = map.next_value::<Option<String>>()?;
                        }
                        "routing.index" => {
                            if seen_routing_index {
                                return Err(serde::de::Error::duplicate_field("routing.index"));
                            }
                            seen_routing_index = true;
                            f_routing_index = map.next_value::<Option<String>>()?;
                        }
                        "routing.search" => {
                            if seen_routing_search {
                                return Err(serde::de::Error::duplicate_field("routing.search"));
                            }
                            seen_routing_search = true;
                            f_routing_search = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AliasesRecord {
                    alias: f_alias,
                    filter: f_filter,
                    index: f_index,
                    is_write_index: f_is_write_index,
                    routing_index: f_routing_index,
                    routing_search: f_routing_search,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AliasesRecord {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.alias {
            map.serialize_entry("alias", v)?;
        }
        if let Some(v) = &self.filter {
            map.serialize_entry("filter", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.is_write_index {
            map.serialize_entry("is_write_index", v)?;
        }
        if let Some(v) = &self.routing_index {
            map.serialize_entry("routing.index", v)?;
        }
        if let Some(v) = &self.routing_search {
            map.serialize_entry("routing.search", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `cat.aliases` API"]
pub type AliasesResponse = crate::types::cat::AliasesResponse;
