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

//! Response types of the `snapshot_cleanup_repository` specification namespace

#![allow(missing_docs)]

#[doc = "The response of the `snapshot.cleanup_repository` API"]
pub type CleanupRepositoryResponse = crate::types::snapshot::CleanupRepositoryResponse;

#[derive(Debug, Clone)]
pub struct CleanupRepositoryResults {
    #[doc = "The number of binary large objects (blobs) removed during cleanup."]
    pub deleted_blobs: i64,
    #[doc = "The number of bytes freed by cleanup operations."]
    pub deleted_bytes: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CleanupRepositoryResults {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CleanupRepositoryResults;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CleanupRepositoryResults object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_deleted_blobs: Option<i64> = None;
                let mut f_deleted_bytes: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "deleted_blobs" => {
                            if f_deleted_blobs.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleted_blobs"));
                            }
                            f_deleted_blobs = Some(map.next_value::<i64>()?);
                        }
                        "deleted_bytes" => {
                            if f_deleted_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleted_bytes"));
                            }
                            f_deleted_bytes = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CleanupRepositoryResults {
                    deleted_blobs: f_deleted_blobs
                        .ok_or_else(|| serde::de::Error::missing_field("deleted_blobs"))?,
                    deleted_bytes: f_deleted_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("deleted_bytes"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CleanupRepositoryResults {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("deleted_blobs", &self.deleted_blobs)?;
        map.serialize_entry("deleted_bytes", &self.deleted_bytes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
