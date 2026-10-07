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

//! Response types of the `cat_nodeattrs` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct NodeAttributesRecord {
    #[doc = "The attribute name."]
    pub attr: Option<String>,
    #[doc = "The hostname."]
    pub host: Option<String>,
    #[doc = "The unique node identifier."]
    pub id: Option<String>,
    #[doc = "The IP address."]
    pub ip: Option<String>,
    #[doc = "The node name."]
    pub node: Option<String>,
    #[doc = "The process identifier."]
    pub pid: Option<String>,
    #[doc = "The bound transport port."]
    pub port: Option<String>,
    #[doc = "The attribute value."]
    pub value: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeAttributesRecord {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeAttributesRecord;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeAttributesRecord object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_attr: Option<String> = None;
                let mut seen_attr = false;
                let mut f_host: Option<String> = None;
                let mut seen_host = false;
                let mut f_id: Option<String> = None;
                let mut seen_id = false;
                let mut f_ip: Option<String> = None;
                let mut seen_ip = false;
                let mut f_node: Option<String> = None;
                let mut seen_node = false;
                let mut f_pid: Option<String> = None;
                let mut seen_pid = false;
                let mut f_port: Option<String> = None;
                let mut seen_port = false;
                let mut f_value: Option<String> = None;
                let mut seen_value = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "attr" => {
                            if seen_attr {
                                return Err(serde::de::Error::duplicate_field("attr"));
                            }
                            seen_attr = true;
                            f_attr = map.next_value::<Option<String>>()?;
                        }
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
                        "pid" => {
                            if seen_pid {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            seen_pid = true;
                            f_pid = map.next_value::<Option<String>>()?;
                        }
                        "port" => {
                            if seen_port {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            seen_port = true;
                            f_port = map.next_value::<Option<String>>()?;
                        }
                        "value" => {
                            if seen_value {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            seen_value = true;
                            f_value = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeAttributesRecord {
                    attr: f_attr,
                    host: f_host,
                    id: f_id,
                    ip: f_ip,
                    node: f_node,
                    pid: f_pid,
                    port: f_port,
                    value: f_value,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeAttributesRecord {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.attr {
            map.serialize_entry("attr", v)?;
        }
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
        if let Some(v) = &self.pid {
            map.serialize_entry("pid", v)?;
        }
        if let Some(v) = &self.port {
            map.serialize_entry("port", v)?;
        }
        if let Some(v) = &self.value {
            map.serialize_entry("value", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `cat.nodeattrs` API"]
pub type NodeattrsResponse = crate::types::cat::NodeattrsResponse;
