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

//! Response types of the `snapshot_restore` specification namespace

#![allow(missing_docs)]

#[doc = "The response of the `snapshot.restore` API"]
pub type RestoreResponse = crate::types::snapshot::RestoreResponse;

#[derive(Debug, Clone)]
pub struct SnapshotRestore {
    #[doc = "The list of indexes that were restored."]
    pub indices: Vec<crate::types::common::IndexName>,
    #[doc = "Any statistics about the restored shards."]
    pub shards: crate::types::common::ShardStatistics,
    #[doc = "The name of the snapshot that was restored."]
    pub snapshot: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SnapshotRestore {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SnapshotRestore;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SnapshotRestore object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_indices: Option<Vec<crate::types::common::IndexName>> = None;
                let mut f_shards: Option<crate::types::common::ShardStatistics> = None;
                let mut f_snapshot: Option<String> = None;
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
                            f_indices =
                                Some(map.next_value::<Vec<crate::types::common::IndexName>>()?);
                        }
                        "shards" => {
                            if f_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("shards"));
                            }
                            f_shards =
                                Some(map.next_value::<crate::types::common::ShardStatistics>()?);
                        }
                        "snapshot" => {
                            if f_snapshot.is_some() {
                                return Err(serde::de::Error::duplicate_field("snapshot"));
                            }
                            f_snapshot = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SnapshotRestore {
                    indices: f_indices.ok_or_else(|| serde::de::Error::missing_field("indices"))?,
                    shards: f_shards.ok_or_else(|| serde::de::Error::missing_field("shards"))?,
                    snapshot: f_snapshot
                        .ok_or_else(|| serde::de::Error::missing_field("snapshot"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SnapshotRestore {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("indices", &self.indices)?;
        map.serialize_entry("shards", &self.shards)?;
        map.serialize_entry("snapshot", &self.snapshot)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
