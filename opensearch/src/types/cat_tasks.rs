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

//! Response types of the `cat_tasks` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct TasksRecord {
    #[doc = "The task action."]
    pub action: Option<String>,
    #[doc = "The task action description."]
    pub description: Option<String>,
    pub id: Option<crate::types::common::Id>,
    #[doc = "The IP address for the node."]
    pub ip: Option<String>,
    #[doc = "The node name."]
    pub node: Option<String>,
    pub node_id: Option<crate::types::common::NodeId>,
    #[doc = "The parent task identifier."]
    pub parent_task_id: Option<String>,
    #[doc = "The bound transport port for the node."]
    pub port: Option<String>,
    #[doc = "The running time."]
    pub running_time: Option<String>,
    #[doc = "The running time in nanoseconds."]
    pub running_time_ns: Option<String>,
    #[doc = "The start time in milliseconds."]
    pub start_time: Option<String>,
    pub task_id: Option<crate::types::common::Id>,
    #[doc = "The start time in `HH:MM:SS` format."]
    pub timestamp: Option<String>,
    #[doc = "The task type."]
    pub r#type: Option<String>,
    pub version: Option<crate::types::common::VersionString>,
    #[doc = "The X-Opaque-ID header."]
    pub x_opaque_id: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TasksRecord {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TasksRecord;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TasksRecord object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_action: Option<String> = None;
                let mut seen_action = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut seen_id = false;
                let mut f_ip: Option<String> = None;
                let mut seen_ip = false;
                let mut f_node: Option<String> = None;
                let mut seen_node = false;
                let mut f_node_id: Option<crate::types::common::NodeId> = None;
                let mut seen_node_id = false;
                let mut f_parent_task_id: Option<String> = None;
                let mut seen_parent_task_id = false;
                let mut f_port: Option<String> = None;
                let mut seen_port = false;
                let mut f_running_time: Option<String> = None;
                let mut seen_running_time = false;
                let mut f_running_time_ns: Option<String> = None;
                let mut seen_running_time_ns = false;
                let mut f_start_time: Option<String> = None;
                let mut seen_start_time = false;
                let mut f_task_id: Option<crate::types::common::Id> = None;
                let mut seen_task_id = false;
                let mut f_timestamp: Option<String> = None;
                let mut seen_timestamp = false;
                let mut f_type: Option<String> = None;
                let mut seen_type = false;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_x_opaque_id: Option<String> = None;
                let mut seen_x_opaque_id = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "action" => {
                            if seen_action {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            seen_action = true;
                            f_action = map.next_value::<Option<String>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<crate::types::common::Id>>()?;
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
                        "node_id" => {
                            if seen_node_id {
                                return Err(serde::de::Error::duplicate_field("node_id"));
                            }
                            seen_node_id = true;
                            f_node_id = map.next_value::<Option<crate::types::common::NodeId>>()?;
                        }
                        "parent_task_id" => {
                            if seen_parent_task_id {
                                return Err(serde::de::Error::duplicate_field("parent_task_id"));
                            }
                            seen_parent_task_id = true;
                            f_parent_task_id = map.next_value::<Option<String>>()?;
                        }
                        "port" => {
                            if seen_port {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            seen_port = true;
                            f_port = map.next_value::<Option<String>>()?;
                        }
                        "running_time" => {
                            if seen_running_time {
                                return Err(serde::de::Error::duplicate_field("running_time"));
                            }
                            seen_running_time = true;
                            f_running_time = map.next_value::<Option<String>>()?;
                        }
                        "running_time_ns" => {
                            if seen_running_time_ns {
                                return Err(serde::de::Error::duplicate_field("running_time_ns"));
                            }
                            seen_running_time_ns = true;
                            f_running_time_ns = map.next_value::<Option<String>>()?;
                        }
                        "start_time" => {
                            if seen_start_time {
                                return Err(serde::de::Error::duplicate_field("start_time"));
                            }
                            seen_start_time = true;
                            f_start_time = map.next_value::<Option<String>>()?;
                        }
                        "task_id" => {
                            if seen_task_id {
                                return Err(serde::de::Error::duplicate_field("task_id"));
                            }
                            seen_task_id = true;
                            f_task_id = map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        "timestamp" => {
                            if seen_timestamp {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            seen_timestamp = true;
                            f_timestamp = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<String>>()?;
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "x_opaque_id" => {
                            if seen_x_opaque_id {
                                return Err(serde::de::Error::duplicate_field("x_opaque_id"));
                            }
                            seen_x_opaque_id = true;
                            f_x_opaque_id = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TasksRecord {
                    action: f_action,
                    description: f_description,
                    id: f_id,
                    ip: f_ip,
                    node: f_node,
                    node_id: f_node_id,
                    parent_task_id: f_parent_task_id,
                    port: f_port,
                    running_time: f_running_time,
                    running_time_ns: f_running_time_ns,
                    start_time: f_start_time,
                    task_id: f_task_id,
                    timestamp: f_timestamp,
                    r#type: f_type,
                    version: f_version,
                    x_opaque_id: f_x_opaque_id,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TasksRecord {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.action {
            map.serialize_entry("action", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
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
        if let Some(v) = &self.node_id {
            map.serialize_entry("node_id", v)?;
        }
        if let Some(v) = &self.parent_task_id {
            map.serialize_entry("parent_task_id", v)?;
        }
        if let Some(v) = &self.port {
            map.serialize_entry("port", v)?;
        }
        if let Some(v) = &self.running_time {
            map.serialize_entry("running_time", v)?;
        }
        if let Some(v) = &self.running_time_ns {
            map.serialize_entry("running_time_ns", v)?;
        }
        if let Some(v) = &self.start_time {
            map.serialize_entry("start_time", v)?;
        }
        if let Some(v) = &self.task_id {
            map.serialize_entry("task_id", v)?;
        }
        if let Some(v) = &self.timestamp {
            map.serialize_entry("timestamp", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("type", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.x_opaque_id {
            map.serialize_entry("x_opaque_id", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `cat.tasks` API"]
pub type TasksResponse = crate::types::cat::TasksResponse;
