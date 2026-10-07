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

//! Response types of the `cat_health` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct HealthRecord {
    #[doc = "active number of shards in percent"]
    pub active_shards_percent: Option<crate::types::common::PercentageString>,
    #[doc = "cluster name"]
    pub cluster: Option<String>,
    #[doc = "cluster manager is discovered or not"]
    pub discovered_cluster_manager: Option<String>,
    #[doc = "cluster manager is discovered or not"]
    pub discovered_master: Option<String>,
    pub epoch: Option<crate::types::common::StringifiedEpochTimeUnitSeconds>,
    #[doc = "number of initializing nodes"]
    pub init: Option<String>,
    #[doc = "wait time of longest task pending"]
    pub max_task_wait_time: Option<String>,
    #[doc = "number of nodes that can store data"]
    pub node_data: Option<String>,
    #[doc = "total number of nodes"]
    pub node_total: Option<String>,
    #[doc = "number of pending tasks"]
    pub pending_tasks: Option<String>,
    #[doc = "number of primary shards"]
    pub pri: Option<String>,
    #[doc = "number of relocating nodes"]
    pub relo: Option<String>,
    #[doc = "total number of shards"]
    pub shards: Option<String>,
    #[doc = "health status"]
    pub status: Option<String>,
    pub timestamp: Option<crate::types::common::TimeOfDay>,
    #[doc = "number of unassigned shards"]
    pub unassign: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HealthRecord {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HealthRecord;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HealthRecord object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_active_shards_percent: Option<crate::types::common::PercentageString> =
                    None;
                let mut seen_active_shards_percent = false;
                let mut f_cluster: Option<String> = None;
                let mut seen_cluster = false;
                let mut f_discovered_cluster_manager: Option<String> = None;
                let mut seen_discovered_cluster_manager = false;
                let mut f_discovered_master: Option<String> = None;
                let mut seen_discovered_master = false;
                let mut f_epoch: Option<crate::types::common::StringifiedEpochTimeUnitSeconds> =
                    None;
                let mut seen_epoch = false;
                let mut f_init: Option<String> = None;
                let mut seen_init = false;
                let mut f_max_task_wait_time: Option<String> = None;
                let mut seen_max_task_wait_time = false;
                let mut f_node_data: Option<String> = None;
                let mut seen_node_data = false;
                let mut f_node_total: Option<String> = None;
                let mut seen_node_total = false;
                let mut f_pending_tasks: Option<String> = None;
                let mut seen_pending_tasks = false;
                let mut f_pri: Option<String> = None;
                let mut seen_pri = false;
                let mut f_relo: Option<String> = None;
                let mut seen_relo = false;
                let mut f_shards: Option<String> = None;
                let mut seen_shards = false;
                let mut f_status: Option<String> = None;
                let mut seen_status = false;
                let mut f_timestamp: Option<crate::types::common::TimeOfDay> = None;
                let mut seen_timestamp = false;
                let mut f_unassign: Option<String> = None;
                let mut seen_unassign = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "active_shards_percent" => {
                            if seen_active_shards_percent {
                                return Err(serde::de::Error::duplicate_field(
                                    "active_shards_percent",
                                ));
                            }
                            seen_active_shards_percent = true;
                            f_active_shards_percent =
                                map.next_value::<Option<crate::types::common::PercentageString>>()?;
                        }
                        "cluster" => {
                            if seen_cluster {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            seen_cluster = true;
                            f_cluster = map.next_value::<Option<String>>()?;
                        }
                        "discovered_cluster_manager" => {
                            if seen_discovered_cluster_manager {
                                return Err(serde::de::Error::duplicate_field(
                                    "discovered_cluster_manager",
                                ));
                            }
                            seen_discovered_cluster_manager = true;
                            f_discovered_cluster_manager = map.next_value::<Option<String>>()?;
                        }
                        "discovered_master" => {
                            if seen_discovered_master {
                                return Err(serde::de::Error::duplicate_field("discovered_master"));
                            }
                            seen_discovered_master = true;
                            f_discovered_master = map.next_value::<Option<String>>()?;
                        }
                        "epoch" => {
                            if seen_epoch {
                                return Err(serde::de::Error::duplicate_field("epoch"));
                            }
                            seen_epoch = true;
                            f_epoch = map.next_value::<Option<crate::types::common::StringifiedEpochTimeUnitSeconds>>()?;
                        }
                        "init" => {
                            if seen_init {
                                return Err(serde::de::Error::duplicate_field("init"));
                            }
                            seen_init = true;
                            f_init = map.next_value::<Option<String>>()?;
                        }
                        "max_task_wait_time" => {
                            if seen_max_task_wait_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_task_wait_time",
                                ));
                            }
                            seen_max_task_wait_time = true;
                            f_max_task_wait_time = map.next_value::<Option<String>>()?;
                        }
                        "node.data" => {
                            if seen_node_data {
                                return Err(serde::de::Error::duplicate_field("node.data"));
                            }
                            seen_node_data = true;
                            f_node_data = map.next_value::<Option<String>>()?;
                        }
                        "node.total" => {
                            if seen_node_total {
                                return Err(serde::de::Error::duplicate_field("node.total"));
                            }
                            seen_node_total = true;
                            f_node_total = map.next_value::<Option<String>>()?;
                        }
                        "pending_tasks" => {
                            if seen_pending_tasks {
                                return Err(serde::de::Error::duplicate_field("pending_tasks"));
                            }
                            seen_pending_tasks = true;
                            f_pending_tasks = map.next_value::<Option<String>>()?;
                        }
                        "pri" => {
                            if seen_pri {
                                return Err(serde::de::Error::duplicate_field("pri"));
                            }
                            seen_pri = true;
                            f_pri = map.next_value::<Option<String>>()?;
                        }
                        "relo" => {
                            if seen_relo {
                                return Err(serde::de::Error::duplicate_field("relo"));
                            }
                            seen_relo = true;
                            f_relo = map.next_value::<Option<String>>()?;
                        }
                        "shards" => {
                            if seen_shards {
                                return Err(serde::de::Error::duplicate_field("shards"));
                            }
                            seen_shards = true;
                            f_shards = map.next_value::<Option<String>>()?;
                        }
                        "status" => {
                            if seen_status {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            seen_status = true;
                            f_status = map.next_value::<Option<String>>()?;
                        }
                        "timestamp" => {
                            if seen_timestamp {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            seen_timestamp = true;
                            f_timestamp =
                                map.next_value::<Option<crate::types::common::TimeOfDay>>()?;
                        }
                        "unassign" => {
                            if seen_unassign {
                                return Err(serde::de::Error::duplicate_field("unassign"));
                            }
                            seen_unassign = true;
                            f_unassign = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HealthRecord {
                    active_shards_percent: f_active_shards_percent,
                    cluster: f_cluster,
                    discovered_cluster_manager: f_discovered_cluster_manager,
                    discovered_master: f_discovered_master,
                    epoch: f_epoch,
                    init: f_init,
                    max_task_wait_time: f_max_task_wait_time,
                    node_data: f_node_data,
                    node_total: f_node_total,
                    pending_tasks: f_pending_tasks,
                    pri: f_pri,
                    relo: f_relo,
                    shards: f_shards,
                    status: f_status,
                    timestamp: f_timestamp,
                    unassign: f_unassign,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HealthRecord {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.active_shards_percent {
            map.serialize_entry("active_shards_percent", v)?;
        }
        if let Some(v) = &self.cluster {
            map.serialize_entry("cluster", v)?;
        }
        if let Some(v) = &self.discovered_cluster_manager {
            map.serialize_entry("discovered_cluster_manager", v)?;
        }
        if let Some(v) = &self.discovered_master {
            map.serialize_entry("discovered_master", v)?;
        }
        if let Some(v) = &self.epoch {
            map.serialize_entry("epoch", v)?;
        }
        if let Some(v) = &self.init {
            map.serialize_entry("init", v)?;
        }
        if let Some(v) = &self.max_task_wait_time {
            map.serialize_entry("max_task_wait_time", v)?;
        }
        if let Some(v) = &self.node_data {
            map.serialize_entry("node.data", v)?;
        }
        if let Some(v) = &self.node_total {
            map.serialize_entry("node.total", v)?;
        }
        if let Some(v) = &self.pending_tasks {
            map.serialize_entry("pending_tasks", v)?;
        }
        if let Some(v) = &self.pri {
            map.serialize_entry("pri", v)?;
        }
        if let Some(v) = &self.relo {
            map.serialize_entry("relo", v)?;
        }
        if let Some(v) = &self.shards {
            map.serialize_entry("shards", v)?;
        }
        if let Some(v) = &self.status {
            map.serialize_entry("status", v)?;
        }
        if let Some(v) = &self.timestamp {
            map.serialize_entry("timestamp", v)?;
        }
        if let Some(v) = &self.unassign {
            map.serialize_entry("unassign", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `cat.health` API"]
pub type HealthResponse = crate::types::cat::HealthResponse;
