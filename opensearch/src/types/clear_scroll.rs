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

//! Response types of the `clear_scroll` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct ClearScrollResponse {
    pub num_freed: i32,
    pub succeeded: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClearScrollResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClearScrollResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClearScrollResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_num_freed: Option<i32> = None;
                let mut f_succeeded: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "num_freed" => {
                            if f_num_freed.is_some() {
                                return Err(serde::de::Error::duplicate_field("num_freed"));
                            }
                            f_num_freed = Some(map.next_value::<i32>()?);
                        }
                        "succeeded" => {
                            if f_succeeded.is_some() {
                                return Err(serde::de::Error::duplicate_field("succeeded"));
                            }
                            f_succeeded = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClearScrollResponse {
                    num_freed: f_num_freed
                        .ok_or_else(|| serde::de::Error::missing_field("num_freed"))?,
                    succeeded: f_succeeded
                        .ok_or_else(|| serde::de::Error::missing_field("succeeded"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClearScrollResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("num_freed", &self.num_freed)?;
        map.serialize_entry("succeeded", &self.succeeded)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
