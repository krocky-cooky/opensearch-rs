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

//! Response types of the `cat_thread_pool` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct ThreadPoolRecord {
    #[doc = "The number of active threads in the current thread pool."]
    pub active: Option<String>,
    #[doc = "The number of completed tasks."]
    pub completed: Option<String>,
    #[doc = "The core number of active threads allowed in a scaling thread pool."]
    pub core: Option<String>,
    #[doc = "The ephemeral node identifier."]
    pub ephemeral_node_id: Option<String>,
    #[doc = "The hostname for the current node."]
    pub host: Option<String>,
    #[doc = "The IP address for the current node."]
    pub ip: Option<String>,
    #[doc = "The thread keep alive time."]
    pub keep_alive: Option<String>,
    #[doc = "The highest number of active threads in the current thread pool."]
    pub largest: Option<String>,
    #[doc = "The maximum number of active threads allowed in a scaling thread pool."]
    pub max: Option<String>,
    #[doc = "The thread pool name."]
    pub name: Option<String>,
    pub node_id: Option<crate::types::common::NodeId>,
    #[doc = "The node name."]
    pub node_name: Option<String>,
    #[doc = "The process identifier."]
    pub pid: Option<String>,
    #[doc = "The number of threads in the current thread pool."]
    pub pool_size: Option<String>,
    #[doc = "The bound transport port for the current node."]
    pub port: Option<String>,
    #[doc = "The number of tasks currently in queue."]
    pub queue: Option<String>,
    #[doc = "The maximum number of tasks permitted in the queue."]
    pub queue_size: Option<String>,
    #[doc = "The number of rejected tasks."]
    pub rejected: Option<String>,
    #[doc = "The number of active threads allowed in a fixed thread pool."]
    pub size: Option<String>,
    pub total_wait_time: Option<String>,
    #[doc = "The thread pool type. Returned values include `fixed`, `fixed_auto_queue_size`, `direct`, and `scaling`."]
    pub r#type: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ThreadPoolRecord {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ThreadPoolRecord;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ThreadPoolRecord object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_active: Option<String> = None;
                let mut seen_active = false;
                let mut f_completed: Option<String> = None;
                let mut seen_completed = false;
                let mut f_core: Option<String> = None;
                let mut seen_core = false;
                let mut f_ephemeral_node_id: Option<String> = None;
                let mut seen_ephemeral_node_id = false;
                let mut f_host: Option<String> = None;
                let mut seen_host = false;
                let mut f_ip: Option<String> = None;
                let mut seen_ip = false;
                let mut f_keep_alive: Option<String> = None;
                let mut seen_keep_alive = false;
                let mut f_largest: Option<String> = None;
                let mut seen_largest = false;
                let mut f_max: Option<String> = None;
                let mut seen_max = false;
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_node_id: Option<crate::types::common::NodeId> = None;
                let mut seen_node_id = false;
                let mut f_node_name: Option<String> = None;
                let mut seen_node_name = false;
                let mut f_pid: Option<String> = None;
                let mut seen_pid = false;
                let mut f_pool_size: Option<String> = None;
                let mut seen_pool_size = false;
                let mut f_port: Option<String> = None;
                let mut seen_port = false;
                let mut f_queue: Option<String> = None;
                let mut seen_queue = false;
                let mut f_queue_size: Option<String> = None;
                let mut seen_queue_size = false;
                let mut f_rejected: Option<String> = None;
                let mut seen_rejected = false;
                let mut f_size: Option<String> = None;
                let mut seen_size = false;
                let mut f_total_wait_time: Option<String> = None;
                let mut seen_total_wait_time = false;
                let mut f_type: Option<String> = None;
                let mut seen_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "active" => {
                            if seen_active {
                                return Err(serde::de::Error::duplicate_field("active"));
                            }
                            seen_active = true;
                            f_active = map.next_value::<Option<String>>()?;
                        }
                        "completed" => {
                            if seen_completed {
                                return Err(serde::de::Error::duplicate_field("completed"));
                            }
                            seen_completed = true;
                            f_completed = map.next_value::<Option<String>>()?;
                        }
                        "core" => {
                            if seen_core {
                                return Err(serde::de::Error::duplicate_field("core"));
                            }
                            seen_core = true;
                            f_core = map.next_value::<Option<String>>()?;
                        }
                        "ephemeral_node_id" => {
                            if seen_ephemeral_node_id {
                                return Err(serde::de::Error::duplicate_field("ephemeral_node_id"));
                            }
                            seen_ephemeral_node_id = true;
                            f_ephemeral_node_id = map.next_value::<Option<String>>()?;
                        }
                        "host" => {
                            if seen_host {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            seen_host = true;
                            f_host = map.next_value::<Option<String>>()?;
                        }
                        "ip" => {
                            if seen_ip {
                                return Err(serde::de::Error::duplicate_field("ip"));
                            }
                            seen_ip = true;
                            f_ip = map.next_value::<Option<String>>()?;
                        }
                        "keep_alive" => {
                            if seen_keep_alive {
                                return Err(serde::de::Error::duplicate_field("keep_alive"));
                            }
                            seen_keep_alive = true;
                            f_keep_alive = map.next_value::<Option<String>>()?;
                        }
                        "largest" => {
                            if seen_largest {
                                return Err(serde::de::Error::duplicate_field("largest"));
                            }
                            seen_largest = true;
                            f_largest = map.next_value::<Option<String>>()?;
                        }
                        "max" => {
                            if seen_max {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            seen_max = true;
                            f_max = map.next_value::<Option<String>>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "node_id" => {
                            if seen_node_id {
                                return Err(serde::de::Error::duplicate_field("node_id"));
                            }
                            seen_node_id = true;
                            f_node_id = map.next_value::<Option<crate::types::common::NodeId>>()?;
                        }
                        "node_name" => {
                            if seen_node_name {
                                return Err(serde::de::Error::duplicate_field("node_name"));
                            }
                            seen_node_name = true;
                            f_node_name = map.next_value::<Option<String>>()?;
                        }
                        "pid" => {
                            if seen_pid {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            seen_pid = true;
                            f_pid = map.next_value::<Option<String>>()?;
                        }
                        "pool_size" => {
                            if seen_pool_size {
                                return Err(serde::de::Error::duplicate_field("pool_size"));
                            }
                            seen_pool_size = true;
                            f_pool_size = map.next_value::<Option<String>>()?;
                        }
                        "port" => {
                            if seen_port {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            seen_port = true;
                            f_port = map.next_value::<Option<String>>()?;
                        }
                        "queue" => {
                            if seen_queue {
                                return Err(serde::de::Error::duplicate_field("queue"));
                            }
                            seen_queue = true;
                            f_queue = map.next_value::<Option<String>>()?;
                        }
                        "queue_size" => {
                            if seen_queue_size {
                                return Err(serde::de::Error::duplicate_field("queue_size"));
                            }
                            seen_queue_size = true;
                            f_queue_size = map.next_value::<Option<String>>()?;
                        }
                        "rejected" => {
                            if seen_rejected {
                                return Err(serde::de::Error::duplicate_field("rejected"));
                            }
                            seen_rejected = true;
                            f_rejected = map.next_value::<Option<String>>()?;
                        }
                        "size" => {
                            if seen_size {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            seen_size = true;
                            f_size = map.next_value::<Option<String>>()?;
                        }
                        "total_wait_time" => {
                            if seen_total_wait_time {
                                return Err(serde::de::Error::duplicate_field("total_wait_time"));
                            }
                            seen_total_wait_time = true;
                            f_total_wait_time = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ThreadPoolRecord {
                    active: f_active,
                    completed: f_completed,
                    core: f_core,
                    ephemeral_node_id: f_ephemeral_node_id,
                    host: f_host,
                    ip: f_ip,
                    keep_alive: f_keep_alive,
                    largest: f_largest,
                    max: f_max,
                    name: f_name,
                    node_id: f_node_id,
                    node_name: f_node_name,
                    pid: f_pid,
                    pool_size: f_pool_size,
                    port: f_port,
                    queue: f_queue,
                    queue_size: f_queue_size,
                    rejected: f_rejected,
                    size: f_size,
                    total_wait_time: f_total_wait_time,
                    r#type: f_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ThreadPoolRecord {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.active {
            map.serialize_entry("active", v)?;
        }
        if let Some(v) = &self.completed {
            map.serialize_entry("completed", v)?;
        }
        if let Some(v) = &self.core {
            map.serialize_entry("core", v)?;
        }
        if let Some(v) = &self.ephemeral_node_id {
            map.serialize_entry("ephemeral_node_id", v)?;
        }
        if let Some(v) = &self.host {
            map.serialize_entry("host", v)?;
        }
        if let Some(v) = &self.ip {
            map.serialize_entry("ip", v)?;
        }
        if let Some(v) = &self.keep_alive {
            map.serialize_entry("keep_alive", v)?;
        }
        if let Some(v) = &self.largest {
            map.serialize_entry("largest", v)?;
        }
        if let Some(v) = &self.max {
            map.serialize_entry("max", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.node_id {
            map.serialize_entry("node_id", v)?;
        }
        if let Some(v) = &self.node_name {
            map.serialize_entry("node_name", v)?;
        }
        if let Some(v) = &self.pid {
            map.serialize_entry("pid", v)?;
        }
        if let Some(v) = &self.pool_size {
            map.serialize_entry("pool_size", v)?;
        }
        if let Some(v) = &self.port {
            map.serialize_entry("port", v)?;
        }
        if let Some(v) = &self.queue {
            map.serialize_entry("queue", v)?;
        }
        if let Some(v) = &self.queue_size {
            map.serialize_entry("queue_size", v)?;
        }
        if let Some(v) = &self.rejected {
            map.serialize_entry("rejected", v)?;
        }
        if let Some(v) = &self.size {
            map.serialize_entry("size", v)?;
        }
        if let Some(v) = &self.total_wait_time {
            map.serialize_entry("total_wait_time", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("type", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `cat.thread_pool` API"]
pub type ThreadPoolResponse = crate::types::cat::ThreadPoolResponse;
