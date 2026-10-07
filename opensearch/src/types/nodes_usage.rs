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

//! Response types of the `nodes_usage` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct NodeUsage {
    pub aggregations: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub rest_actions: Option<std::collections::BTreeMap<String, i64>>,
    pub since: crate::types::common::EpochTimeUnitMillis,
    pub timestamp: crate::types::common::EpochTimeUnitMillis,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeUsage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeUsage;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeUsage object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_aggregations: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_aggregations = false;
                let mut f_rest_actions: Option<std::collections::BTreeMap<String, i64>> = None;
                let mut seen_rest_actions = false;
                let mut f_since: Option<crate::types::common::EpochTimeUnitMillis> = None;
                let mut f_timestamp: Option<crate::types::common::EpochTimeUnitMillis> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "aggregations" => {
                            if seen_aggregations {
                                return Err(serde::de::Error::duplicate_field("aggregations"));
                            }
                            seen_aggregations = true;
                            f_aggregations = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "rest_actions" => {
                            if seen_rest_actions {
                                return Err(serde::de::Error::duplicate_field("rest_actions"));
                            }
                            seen_rest_actions = true;
                            f_rest_actions = map
                                .next_value::<Option<std::collections::BTreeMap<String, i64>>>()?;
                        }
                        "since" => {
                            if f_since.is_some() {
                                return Err(serde::de::Error::duplicate_field("since"));
                            }
                            f_since = Some(
                                map.next_value::<crate::types::common::EpochTimeUnitMillis>()?,
                            );
                        }
                        "timestamp" => {
                            if f_timestamp.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            f_timestamp = Some(
                                map.next_value::<crate::types::common::EpochTimeUnitMillis>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeUsage {
                    aggregations: f_aggregations,
                    rest_actions: f_rest_actions,
                    since: f_since.ok_or_else(|| serde::de::Error::missing_field("since"))?,
                    timestamp: f_timestamp
                        .ok_or_else(|| serde::de::Error::missing_field("timestamp"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeUsage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.aggregations {
            map.serialize_entry("aggregations", v)?;
        }
        if let Some(v) = &self.rest_actions {
            map.serialize_entry("rest_actions", v)?;
        }
        map.serialize_entry("since", &self.since)?;
        map.serialize_entry("timestamp", &self.timestamp)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `nodes.usage` API"]
pub type UsageResponse = crate::types::nodes::UsageResponse;
