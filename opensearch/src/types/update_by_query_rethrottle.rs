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

//! Response types of the `update_by_query_rethrottle` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct UpdateByQueryRethrottleNode {
    #[doc = "The attributes of the node."]
    pub attributes: Option<std::collections::BTreeMap<String, String>>,
    pub host: Option<crate::types::common::Host>,
    pub ip: Option<crate::types::common::Ip>,
    pub name: crate::types::common::Name,
    pub roles: Option<crate::types::common::NodeRoles>,
    pub transport_address: Option<crate::types::common::TransportAddress>,
    pub tasks: std::collections::BTreeMap<String, crate::types::tasks_common::TaskInfo>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UpdateByQueryRethrottleNode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UpdateByQueryRethrottleNode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UpdateByQueryRethrottleNode object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_attributes: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_attributes = false;
                let mut f_host: Option<crate::types::common::Host> = None;
                let mut seen_host = false;
                let mut f_ip: Option<crate::types::common::Ip> = None;
                let mut seen_ip = false;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut f_roles: Option<crate::types::common::NodeRoles> = None;
                let mut seen_roles = false;
                let mut f_transport_address: Option<crate::types::common::TransportAddress> = None;
                let mut seen_transport_address = false;
                let mut f_tasks: Option<
                    std::collections::BTreeMap<String, crate::types::tasks_common::TaskInfo>,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "attributes" => {
                            if seen_attributes {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            seen_attributes = true;
                            f_attributes = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "host" => {
                            if seen_host {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            seen_host = true;
                            f_host = map.next_value::<Option<crate::types::common::Host>>()?;
                        }
                        "ip" => {
                            if seen_ip {
                                return Err(serde::de::Error::duplicate_field("ip"));
                            }
                            seen_ip = true;
                            f_ip = map.next_value::<Option<crate::types::common::Ip>>()?;
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        "roles" => {
                            if seen_roles {
                                return Err(serde::de::Error::duplicate_field("roles"));
                            }
                            seen_roles = true;
                            f_roles =
                                map.next_value::<Option<crate::types::common::NodeRoles>>()?;
                        }
                        "transport_address" => {
                            if seen_transport_address {
                                return Err(serde::de::Error::duplicate_field("transport_address"));
                            }
                            seen_transport_address = true;
                            f_transport_address =
                                map.next_value::<Option<crate::types::common::TransportAddress>>()?;
                        }
                        "tasks" => {
                            if f_tasks.is_some() {
                                return Err(serde::de::Error::duplicate_field("tasks"));
                            }
                            f_tasks = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                crate::types::tasks_common::TaskInfo,
                            >>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UpdateByQueryRethrottleNode {
                    attributes: f_attributes,
                    host: f_host,
                    ip: f_ip,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    roles: f_roles,
                    transport_address: f_transport_address,
                    tasks: f_tasks.ok_or_else(|| serde::de::Error::missing_field("tasks"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UpdateByQueryRethrottleNode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.attributes {
            map.serialize_entry("attributes", v)?;
        }
        if let Some(v) = &self.host {
            map.serialize_entry("host", v)?;
        }
        if let Some(v) = &self.ip {
            map.serialize_entry("ip", v)?;
        }
        map.serialize_entry("name", &self.name)?;
        if let Some(v) = &self.roles {
            map.serialize_entry("roles", v)?;
        }
        if let Some(v) = &self.transport_address {
            map.serialize_entry("transport_address", v)?;
        }
        map.serialize_entry("tasks", &self.tasks)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct UpdateByQueryRethrottleResponse {
    pub node_failures: Option<Vec<crate::types::common::ErrorCause>>,
    pub nodes: std::collections::BTreeMap<
        String,
        crate::types::update_by_query_rethrottle::UpdateByQueryRethrottleNode,
    >,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UpdateByQueryRethrottleResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UpdateByQueryRethrottleResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UpdateByQueryRethrottleResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_node_failures: Option<Vec<crate::types::common::ErrorCause>> = None;
                let mut seen_node_failures = false;
                let mut f_nodes: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::update_by_query_rethrottle::UpdateByQueryRethrottleNode,
                    >,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "node_failures" => {
                            if seen_node_failures {
                                return Err(serde::de::Error::duplicate_field("node_failures"));
                            }
                            seen_node_failures = true;
                            f_node_failures =
                                map.next_value::<Option<Vec<crate::types::common::ErrorCause>>>()?;
                        }
                        "nodes" => {
                            if f_nodes.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            f_nodes = Some(map.next_value::<std::collections::BTreeMap<String, crate::types::update_by_query_rethrottle::UpdateByQueryRethrottleNode>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UpdateByQueryRethrottleResponse {
                    node_failures: f_node_failures,
                    nodes: f_nodes.ok_or_else(|| serde::de::Error::missing_field("nodes"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UpdateByQueryRethrottleResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.node_failures {
            map.serialize_entry("node_failures", v)?;
        }
        map.serialize_entry("nodes", &self.nodes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
