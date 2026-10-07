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

//! Response types of the `cat_cluster_manager` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct ClusterManagerRecord {
    #[doc = "hostname"]
    pub host: Option<String>,
    #[doc = "node id"]
    pub id: Option<String>,
    #[doc = "IP address"]
    pub ip: Option<String>,
    #[doc = "node name"]
    pub node: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterManagerRecord {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterManagerRecord;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterManagerRecord object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_host: Option<String> = None;
                let mut seen_host = false;
                let mut f_id: Option<String> = None;
                let mut seen_id = false;
                let mut f_ip: Option<String> = None;
                let mut seen_ip = false;
                let mut f_node: Option<String> = None;
                let mut seen_node = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "host" => {
                            if seen_host {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            seen_host = true;
                            f_host = map.next_value::<Option<String>>()?;
                        }
                        "id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<String>>()?;
                        }
                        "ip" => {
                            if seen_ip {
                                return Err(serde::de::Error::duplicate_field("ip"));
                            }
                            seen_ip = true;
                            f_ip = map.next_value::<Option<String>>()?;
                        }
                        "node" => {
                            if seen_node {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            seen_node = true;
                            f_node = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterManagerRecord {
                    host: f_host,
                    id: f_id,
                    ip: f_ip,
                    node: f_node,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterManagerRecord {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.host {
            map.serialize_entry("host", v)?;
        }
        if let Some(v) = &self.id {
            map.serialize_entry("id", v)?;
        }
        if let Some(v) = &self.ip {
            map.serialize_entry("ip", v)?;
        }
        if let Some(v) = &self.node {
            map.serialize_entry("node", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `cat.cluster_manager` API"]
pub type ClusterManagerResponse = crate::types::cat::ClusterManagerResponse;
