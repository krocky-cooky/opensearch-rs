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

//! Response types of the `count` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct CountResponse {
    pub shards: crate::types::common::ShardStatistics,
    pub count: i64,
    pub terminated_early: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CountResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CountResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CountResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_shards: Option<crate::types::common::ShardStatistics> = None;
                let mut f_count: Option<i64> = None;
                let mut f_terminated_early: Option<bool> = None;
                let mut seen_terminated_early = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_shards" => {
                            if f_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("_shards"));
                            }
                            f_shards =
                                Some(map.next_value::<crate::types::common::ShardStatistics>()?);
                        }
                        "count" => {
                            if f_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            f_count = Some(map.next_value::<i64>()?);
                        }
                        "terminated_early" => {
                            if seen_terminated_early {
                                return Err(serde::de::Error::duplicate_field("terminated_early"));
                            }
                            seen_terminated_early = true;
                            f_terminated_early = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CountResponse {
                    shards: f_shards.ok_or_else(|| serde::de::Error::missing_field("_shards"))?,
                    count: f_count.ok_or_else(|| serde::de::Error::missing_field("count"))?,
                    terminated_early: f_terminated_early,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CountResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("_shards", &self.shards)?;
        map.serialize_entry("count", &self.count)?;
        if let Some(v) = &self.terminated_early {
            map.serialize_entry("terminated_early", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
