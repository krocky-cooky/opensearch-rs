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

//! Response types of the `dangling_indices_list_dangling_indices` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct DanglingIndex {
    pub creation_date: Option<crate::types::common::DateTime>,
    pub creation_date_millis: crate::types::common::EpochTimeUnitMillis,
    pub index_name: String,
    pub index_uuid: String,
    pub node_ids: crate::types::common::Ids,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DanglingIndex {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DanglingIndex;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DanglingIndex object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_creation_date: Option<crate::types::common::DateTime> = None;
                let mut seen_creation_date = false;
                let mut f_creation_date_millis: Option<crate::types::common::EpochTimeUnitMillis> =
                    None;
                let mut f_index_name: Option<String> = None;
                let mut f_index_uuid: Option<String> = None;
                let mut f_node_ids: Option<crate::types::common::Ids> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "creation_date" => {
                            if seen_creation_date {
                                return Err(serde::de::Error::duplicate_field("creation_date"));
                            }
                            seen_creation_date = true;
                            f_creation_date =
                                map.next_value::<Option<crate::types::common::DateTime>>()?;
                        }
                        "creation_date_millis" => {
                            if f_creation_date_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "creation_date_millis",
                                ));
                            }
                            f_creation_date_millis = Some(
                                map.next_value::<crate::types::common::EpochTimeUnitMillis>()?,
                            );
                        }
                        "index_name" => {
                            if f_index_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("index_name"));
                            }
                            f_index_name = Some(map.next_value::<String>()?);
                        }
                        "index_uuid" => {
                            if f_index_uuid.is_some() {
                                return Err(serde::de::Error::duplicate_field("index_uuid"));
                            }
                            f_index_uuid = Some(map.next_value::<String>()?);
                        }
                        "node_ids" => {
                            if f_node_ids.is_some() {
                                return Err(serde::de::Error::duplicate_field("node_ids"));
                            }
                            f_node_ids = Some(map.next_value::<crate::types::common::Ids>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DanglingIndex {
                    creation_date: f_creation_date,
                    creation_date_millis: f_creation_date_millis
                        .ok_or_else(|| serde::de::Error::missing_field("creation_date_millis"))?,
                    index_name: f_index_name
                        .ok_or_else(|| serde::de::Error::missing_field("index_name"))?,
                    index_uuid: f_index_uuid
                        .ok_or_else(|| serde::de::Error::missing_field("index_uuid"))?,
                    node_ids: f_node_ids
                        .ok_or_else(|| serde::de::Error::missing_field("node_ids"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DanglingIndex {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.creation_date {
            map.serialize_entry("creation_date", v)?;
        }
        map.serialize_entry("creation_date_millis", &self.creation_date_millis)?;
        map.serialize_entry("index_name", &self.index_name)?;
        map.serialize_entry("index_uuid", &self.index_uuid)?;
        map.serialize_entry("node_ids", &self.node_ids)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `dangling_indices.list_dangling_indices` API"]
pub type ListDanglingIndicesResponse = crate::types::dangling_indices::ListDanglingIndicesResponse;
