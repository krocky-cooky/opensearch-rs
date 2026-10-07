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

//! Response types of the `cluster_health` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct AwarenessAttributeStats {
    pub active_shards: Option<i32>,
    pub data_nodes: Option<i32>,
    pub initializing_shards: Option<i32>,
    pub relocating_shards: Option<i32>,
    pub unassigned_shards: Option<i32>,
    pub weight: Option<i32>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AwarenessAttributeStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AwarenessAttributeStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AwarenessAttributeStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_active_shards: Option<i32> = None;
                let mut seen_active_shards = false;
                let mut f_data_nodes: Option<i32> = None;
                let mut seen_data_nodes = false;
                let mut f_initializing_shards: Option<i32> = None;
                let mut seen_initializing_shards = false;
                let mut f_relocating_shards: Option<i32> = None;
                let mut seen_relocating_shards = false;
                let mut f_unassigned_shards: Option<i32> = None;
                let mut seen_unassigned_shards = false;
                let mut f_weight: Option<i32> = None;
                let mut seen_weight = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "active_shards" => {
                            if seen_active_shards {
                                return Err(serde::de::Error::duplicate_field("active_shards"));
                            }
                            seen_active_shards = true;
                            f_active_shards = map.next_value::<Option<i32>>()?;
                        }
                        "data_nodes" => {
                            if seen_data_nodes {
                                return Err(serde::de::Error::duplicate_field("data_nodes"));
                            }
                            seen_data_nodes = true;
                            f_data_nodes = map.next_value::<Option<i32>>()?;
                        }
                        "initializing_shards" => {
                            if seen_initializing_shards {
                                return Err(serde::de::Error::duplicate_field(
                                    "initializing_shards",
                                ));
                            }
                            seen_initializing_shards = true;
                            f_initializing_shards = map.next_value::<Option<i32>>()?;
                        }
                        "relocating_shards" => {
                            if seen_relocating_shards {
                                return Err(serde::de::Error::duplicate_field("relocating_shards"));
                            }
                            seen_relocating_shards = true;
                            f_relocating_shards = map.next_value::<Option<i32>>()?;
                        }
                        "unassigned_shards" => {
                            if seen_unassigned_shards {
                                return Err(serde::de::Error::duplicate_field("unassigned_shards"));
                            }
                            seen_unassigned_shards = true;
                            f_unassigned_shards = map.next_value::<Option<i32>>()?;
                        }
                        "weight" => {
                            if seen_weight {
                                return Err(serde::de::Error::duplicate_field("weight"));
                            }
                            seen_weight = true;
                            f_weight = map.next_value::<Option<i32>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AwarenessAttributeStats {
                    active_shards: f_active_shards,
                    data_nodes: f_data_nodes,
                    initializing_shards: f_initializing_shards,
                    relocating_shards: f_relocating_shards,
                    unassigned_shards: f_unassigned_shards,
                    weight: f_weight,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AwarenessAttributeStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.active_shards {
            map.serialize_entry("active_shards", v)?;
        }
        if let Some(v) = &self.data_nodes {
            map.serialize_entry("data_nodes", v)?;
        }
        if let Some(v) = &self.initializing_shards {
            map.serialize_entry("initializing_shards", v)?;
        }
        if let Some(v) = &self.relocating_shards {
            map.serialize_entry("relocating_shards", v)?;
        }
        if let Some(v) = &self.unassigned_shards {
            map.serialize_entry("unassigned_shards", v)?;
        }
        if let Some(v) = &self.weight {
            map.serialize_entry("weight", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `cluster.health` API"]
pub type HealthResponse = crate::types::cluster_health::HealthResponseBody;

#[derive(Debug, Clone)]
pub struct HealthResponseBody {
    #[doc = "The number of active primary shards."]
    pub active_primary_shards: i32,
    #[doc = "The total number of active primary and replica shards."]
    pub active_shards: i32,
    pub active_shards_percent: Option<crate::types::common::PercentageString>,
    pub active_shards_percent_as_number: crate::types::common::PercentageNumber,
    #[doc = "Cluster health information for each awareness attribute."]
    pub awareness_attributes: Option<
        std::collections::BTreeMap<String, crate::types::cluster_health::AwarenessAttributeStats>,
    >,
    pub cluster_name: crate::types::common::Name,
    #[doc = "The number of shards whose allocation has been delayed by the timeout settings."]
    pub delayed_unassigned_shards: i32,
    #[doc = "True if the cluster-manager node has been discovered."]
    pub discovered_cluster_manager: Option<bool>,
    #[doc = "True if the cluster-manager node has been discovered."]
    pub discovered_master: Option<bool>,
    pub indices:
        Option<std::collections::BTreeMap<String, crate::types::cluster_health::IndexHealthStats>>,
    #[doc = "The number of shards that are under initialization."]
    pub initializing_shards: i32,
    #[doc = "The number of nodes that are dedicated data nodes."]
    pub number_of_data_nodes: i32,
    #[doc = "The number of unfinished fetches."]
    pub number_of_in_flight_fetch: i32,
    #[doc = "The number of nodes within the cluster."]
    pub number_of_nodes: i32,
    #[doc = "The number of cluster-level changes that have not yet been executed."]
    pub number_of_pending_tasks: i32,
    #[doc = "The number of shards that are under relocation."]
    pub relocating_shards: i32,
    pub status: crate::types::common::HealthStatus,
    pub task_max_waiting_in_queue: Option<crate::types::common::Duration>,
    pub task_max_waiting_in_queue_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "If `false` the response returned within the period of time that is specified by the timeout parameter (`30s` by default)."]
    pub timed_out: bool,
    #[doc = "The number of shards that are not allocated."]
    pub unassigned_shards: i32,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HealthResponseBody {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HealthResponseBody;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HealthResponseBody object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_active_primary_shards: Option<i32> = None;
                let mut f_active_shards: Option<i32> = None;
                let mut f_active_shards_percent: Option<crate::types::common::PercentageString> =
                    None;
                let mut seen_active_shards_percent = false;
                let mut f_active_shards_percent_as_number: Option<
                    crate::types::common::PercentageNumber,
                > = None;
                let mut f_awareness_attributes: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::cluster_health::AwarenessAttributeStats,
                    >,
                > = None;
                let mut seen_awareness_attributes = false;
                let mut f_cluster_name: Option<crate::types::common::Name> = None;
                let mut f_delayed_unassigned_shards: Option<i32> = None;
                let mut f_discovered_cluster_manager: Option<bool> = None;
                let mut seen_discovered_cluster_manager = false;
                let mut f_discovered_master: Option<bool> = None;
                let mut seen_discovered_master = false;
                let mut f_indices: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::cluster_health::IndexHealthStats,
                    >,
                > = None;
                let mut seen_indices = false;
                let mut f_initializing_shards: Option<i32> = None;
                let mut f_number_of_data_nodes: Option<i32> = None;
                let mut f_number_of_in_flight_fetch: Option<i32> = None;
                let mut f_number_of_nodes: Option<i32> = None;
                let mut f_number_of_pending_tasks: Option<i32> = None;
                let mut f_relocating_shards: Option<i32> = None;
                let mut f_status: Option<crate::types::common::HealthStatus> = None;
                let mut f_task_max_waiting_in_queue: Option<crate::types::common::Duration> = None;
                let mut seen_task_max_waiting_in_queue = false;
                let mut f_task_max_waiting_in_queue_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_timed_out: Option<bool> = None;
                let mut f_unassigned_shards: Option<i32> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "active_primary_shards" => {
                            if f_active_primary_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "active_primary_shards",
                                ));
                            }
                            f_active_primary_shards = Some(map.next_value::<i32>()?);
                        }
                        "active_shards" => {
                            if f_active_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("active_shards"));
                            }
                            f_active_shards = Some(map.next_value::<i32>()?);
                        }
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
                        "active_shards_percent_as_number" => {
                            if f_active_shards_percent_as_number.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "active_shards_percent_as_number",
                                ));
                            }
                            f_active_shards_percent_as_number =
                                Some(map.next_value::<crate::types::common::PercentageNumber>()?);
                        }
                        "awareness_attributes" => {
                            if seen_awareness_attributes {
                                return Err(serde::de::Error::duplicate_field(
                                    "awareness_attributes",
                                ));
                            }
                            seen_awareness_attributes = true;
                            f_awareness_attributes = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::cluster_health::AwarenessAttributeStats,
                                >,
                            >>()?;
                        }
                        "cluster_name" => {
                            if f_cluster_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster_name"));
                            }
                            f_cluster_name = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        "delayed_unassigned_shards" => {
                            if f_delayed_unassigned_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "delayed_unassigned_shards",
                                ));
                            }
                            f_delayed_unassigned_shards = Some(map.next_value::<i32>()?);
                        }
                        "discovered_cluster_manager" => {
                            if seen_discovered_cluster_manager {
                                return Err(serde::de::Error::duplicate_field(
                                    "discovered_cluster_manager",
                                ));
                            }
                            seen_discovered_cluster_manager = true;
                            f_discovered_cluster_manager = map.next_value::<Option<bool>>()?;
                        }
                        "discovered_master" => {
                            if seen_discovered_master {
                                return Err(serde::de::Error::duplicate_field("discovered_master"));
                            }
                            seen_discovered_master = true;
                            f_discovered_master = map.next_value::<Option<bool>>()?;
                        }
                        "indices" => {
                            if seen_indices {
                                return Err(serde::de::Error::duplicate_field("indices"));
                            }
                            seen_indices = true;
                            f_indices = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::cluster_health::IndexHealthStats,
                                >,
                            >>()?;
                        }
                        "initializing_shards" => {
                            if f_initializing_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "initializing_shards",
                                ));
                            }
                            f_initializing_shards = Some(map.next_value::<i32>()?);
                        }
                        "number_of_data_nodes" => {
                            if f_number_of_data_nodes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "number_of_data_nodes",
                                ));
                            }
                            f_number_of_data_nodes = Some(map.next_value::<i32>()?);
                        }
                        "number_of_in_flight_fetch" => {
                            if f_number_of_in_flight_fetch.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "number_of_in_flight_fetch",
                                ));
                            }
                            f_number_of_in_flight_fetch = Some(map.next_value::<i32>()?);
                        }
                        "number_of_nodes" => {
                            if f_number_of_nodes.is_some() {
                                return Err(serde::de::Error::duplicate_field("number_of_nodes"));
                            }
                            f_number_of_nodes = Some(map.next_value::<i32>()?);
                        }
                        "number_of_pending_tasks" => {
                            if f_number_of_pending_tasks.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "number_of_pending_tasks",
                                ));
                            }
                            f_number_of_pending_tasks = Some(map.next_value::<i32>()?);
                        }
                        "relocating_shards" => {
                            if f_relocating_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("relocating_shards"));
                            }
                            f_relocating_shards = Some(map.next_value::<i32>()?);
                        }
                        "status" => {
                            if f_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            f_status =
                                Some(map.next_value::<crate::types::common::HealthStatus>()?);
                        }
                        "task_max_waiting_in_queue" => {
                            if seen_task_max_waiting_in_queue {
                                return Err(serde::de::Error::duplicate_field(
                                    "task_max_waiting_in_queue",
                                ));
                            }
                            seen_task_max_waiting_in_queue = true;
                            f_task_max_waiting_in_queue =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "task_max_waiting_in_queue_millis" => {
                            if f_task_max_waiting_in_queue_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "task_max_waiting_in_queue_millis",
                                ));
                            }
                            f_task_max_waiting_in_queue_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "timed_out" => {
                            if f_timed_out.is_some() {
                                return Err(serde::de::Error::duplicate_field("timed_out"));
                            }
                            f_timed_out = Some(map.next_value::<bool>()?);
                        }
                        "unassigned_shards" => {
                            if f_unassigned_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("unassigned_shards"));
                            }
                            f_unassigned_shards = Some(map.next_value::<i32>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HealthResponseBody {
                    active_primary_shards: f_active_primary_shards
                        .ok_or_else(|| serde::de::Error::missing_field("active_primary_shards"))?,
                    active_shards: f_active_shards
                        .ok_or_else(|| serde::de::Error::missing_field("active_shards"))?,
                    active_shards_percent: f_active_shards_percent,
                    active_shards_percent_as_number: f_active_shards_percent_as_number.ok_or_else(
                        || serde::de::Error::missing_field("active_shards_percent_as_number"),
                    )?,
                    awareness_attributes: f_awareness_attributes,
                    cluster_name: f_cluster_name
                        .ok_or_else(|| serde::de::Error::missing_field("cluster_name"))?,
                    delayed_unassigned_shards: f_delayed_unassigned_shards.ok_or_else(|| {
                        serde::de::Error::missing_field("delayed_unassigned_shards")
                    })?,
                    discovered_cluster_manager: f_discovered_cluster_manager,
                    discovered_master: f_discovered_master,
                    indices: f_indices,
                    initializing_shards: f_initializing_shards
                        .ok_or_else(|| serde::de::Error::missing_field("initializing_shards"))?,
                    number_of_data_nodes: f_number_of_data_nodes
                        .ok_or_else(|| serde::de::Error::missing_field("number_of_data_nodes"))?,
                    number_of_in_flight_fetch: f_number_of_in_flight_fetch.ok_or_else(|| {
                        serde::de::Error::missing_field("number_of_in_flight_fetch")
                    })?,
                    number_of_nodes: f_number_of_nodes
                        .ok_or_else(|| serde::de::Error::missing_field("number_of_nodes"))?,
                    number_of_pending_tasks: f_number_of_pending_tasks.ok_or_else(|| {
                        serde::de::Error::missing_field("number_of_pending_tasks")
                    })?,
                    relocating_shards: f_relocating_shards
                        .ok_or_else(|| serde::de::Error::missing_field("relocating_shards"))?,
                    status: f_status.ok_or_else(|| serde::de::Error::missing_field("status"))?,
                    task_max_waiting_in_queue: f_task_max_waiting_in_queue,
                    task_max_waiting_in_queue_millis: f_task_max_waiting_in_queue_millis
                        .ok_or_else(|| {
                            serde::de::Error::missing_field("task_max_waiting_in_queue_millis")
                        })?,
                    timed_out: f_timed_out
                        .ok_or_else(|| serde::de::Error::missing_field("timed_out"))?,
                    unassigned_shards: f_unassigned_shards
                        .ok_or_else(|| serde::de::Error::missing_field("unassigned_shards"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HealthResponseBody {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("active_primary_shards", &self.active_primary_shards)?;
        map.serialize_entry("active_shards", &self.active_shards)?;
        if let Some(v) = &self.active_shards_percent {
            map.serialize_entry("active_shards_percent", v)?;
        }
        map.serialize_entry(
            "active_shards_percent_as_number",
            &self.active_shards_percent_as_number,
        )?;
        if let Some(v) = &self.awareness_attributes {
            map.serialize_entry("awareness_attributes", v)?;
        }
        map.serialize_entry("cluster_name", &self.cluster_name)?;
        map.serialize_entry("delayed_unassigned_shards", &self.delayed_unassigned_shards)?;
        if let Some(v) = &self.discovered_cluster_manager {
            map.serialize_entry("discovered_cluster_manager", v)?;
        }
        if let Some(v) = &self.discovered_master {
            map.serialize_entry("discovered_master", v)?;
        }
        if let Some(v) = &self.indices {
            map.serialize_entry("indices", v)?;
        }
        map.serialize_entry("initializing_shards", &self.initializing_shards)?;
        map.serialize_entry("number_of_data_nodes", &self.number_of_data_nodes)?;
        map.serialize_entry("number_of_in_flight_fetch", &self.number_of_in_flight_fetch)?;
        map.serialize_entry("number_of_nodes", &self.number_of_nodes)?;
        map.serialize_entry("number_of_pending_tasks", &self.number_of_pending_tasks)?;
        map.serialize_entry("relocating_shards", &self.relocating_shards)?;
        map.serialize_entry("status", &self.status)?;
        if let Some(v) = &self.task_max_waiting_in_queue {
            map.serialize_entry("task_max_waiting_in_queue", v)?;
        }
        map.serialize_entry(
            "task_max_waiting_in_queue_millis",
            &self.task_max_waiting_in_queue_millis,
        )?;
        map.serialize_entry("timed_out", &self.timed_out)?;
        map.serialize_entry("unassigned_shards", &self.unassigned_shards)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IndexHealthStats {
    pub active_primary_shards: i32,
    pub active_shards: i32,
    pub initializing_shards: i32,
    pub number_of_replicas: i32,
    pub number_of_shards: i32,
    pub relocating_shards: i32,
    pub shards:
        Option<std::collections::BTreeMap<String, crate::types::cluster_health::ShardHealthStats>>,
    pub status: crate::types::common::HealthStatus,
    pub unassigned_shards: i32,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexHealthStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexHealthStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexHealthStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_active_primary_shards: Option<i32> = None;
                let mut f_active_shards: Option<i32> = None;
                let mut f_initializing_shards: Option<i32> = None;
                let mut f_number_of_replicas: Option<i32> = None;
                let mut f_number_of_shards: Option<i32> = None;
                let mut f_relocating_shards: Option<i32> = None;
                let mut f_shards: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::cluster_health::ShardHealthStats,
                    >,
                > = None;
                let mut seen_shards = false;
                let mut f_status: Option<crate::types::common::HealthStatus> = None;
                let mut f_unassigned_shards: Option<i32> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "active_primary_shards" => {
                            if f_active_primary_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "active_primary_shards",
                                ));
                            }
                            f_active_primary_shards = Some(map.next_value::<i32>()?);
                        }
                        "active_shards" => {
                            if f_active_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("active_shards"));
                            }
                            f_active_shards = Some(map.next_value::<i32>()?);
                        }
                        "initializing_shards" => {
                            if f_initializing_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "initializing_shards",
                                ));
                            }
                            f_initializing_shards = Some(map.next_value::<i32>()?);
                        }
                        "number_of_replicas" => {
                            if f_number_of_replicas.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "number_of_replicas",
                                ));
                            }
                            f_number_of_replicas = Some(map.next_value::<i32>()?);
                        }
                        "number_of_shards" => {
                            if f_number_of_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("number_of_shards"));
                            }
                            f_number_of_shards = Some(map.next_value::<i32>()?);
                        }
                        "relocating_shards" => {
                            if f_relocating_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("relocating_shards"));
                            }
                            f_relocating_shards = Some(map.next_value::<i32>()?);
                        }
                        "shards" => {
                            if seen_shards {
                                return Err(serde::de::Error::duplicate_field("shards"));
                            }
                            seen_shards = true;
                            f_shards = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::cluster_health::ShardHealthStats,
                                >,
                            >>()?;
                        }
                        "status" => {
                            if f_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            f_status =
                                Some(map.next_value::<crate::types::common::HealthStatus>()?);
                        }
                        "unassigned_shards" => {
                            if f_unassigned_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("unassigned_shards"));
                            }
                            f_unassigned_shards = Some(map.next_value::<i32>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexHealthStats {
                    active_primary_shards: f_active_primary_shards
                        .ok_or_else(|| serde::de::Error::missing_field("active_primary_shards"))?,
                    active_shards: f_active_shards
                        .ok_or_else(|| serde::de::Error::missing_field("active_shards"))?,
                    initializing_shards: f_initializing_shards
                        .ok_or_else(|| serde::de::Error::missing_field("initializing_shards"))?,
                    number_of_replicas: f_number_of_replicas
                        .ok_or_else(|| serde::de::Error::missing_field("number_of_replicas"))?,
                    number_of_shards: f_number_of_shards
                        .ok_or_else(|| serde::de::Error::missing_field("number_of_shards"))?,
                    relocating_shards: f_relocating_shards
                        .ok_or_else(|| serde::de::Error::missing_field("relocating_shards"))?,
                    shards: f_shards,
                    status: f_status.ok_or_else(|| serde::de::Error::missing_field("status"))?,
                    unassigned_shards: f_unassigned_shards
                        .ok_or_else(|| serde::de::Error::missing_field("unassigned_shards"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexHealthStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("active_primary_shards", &self.active_primary_shards)?;
        map.serialize_entry("active_shards", &self.active_shards)?;
        map.serialize_entry("initializing_shards", &self.initializing_shards)?;
        map.serialize_entry("number_of_replicas", &self.number_of_replicas)?;
        map.serialize_entry("number_of_shards", &self.number_of_shards)?;
        map.serialize_entry("relocating_shards", &self.relocating_shards)?;
        if let Some(v) = &self.shards {
            map.serialize_entry("shards", v)?;
        }
        map.serialize_entry("status", &self.status)?;
        map.serialize_entry("unassigned_shards", &self.unassigned_shards)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardHealthStats {
    pub active_shards: i32,
    pub initializing_shards: i32,
    pub primary_active: bool,
    pub relocating_shards: i32,
    pub status: crate::types::common::HealthStatus,
    pub unassigned_shards: i32,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardHealthStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardHealthStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardHealthStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_active_shards: Option<i32> = None;
                let mut f_initializing_shards: Option<i32> = None;
                let mut f_primary_active: Option<bool> = None;
                let mut f_relocating_shards: Option<i32> = None;
                let mut f_status: Option<crate::types::common::HealthStatus> = None;
                let mut f_unassigned_shards: Option<i32> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "active_shards" => {
                            if f_active_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("active_shards"));
                            }
                            f_active_shards = Some(map.next_value::<i32>()?);
                        }
                        "initializing_shards" => {
                            if f_initializing_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "initializing_shards",
                                ));
                            }
                            f_initializing_shards = Some(map.next_value::<i32>()?);
                        }
                        "primary_active" => {
                            if f_primary_active.is_some() {
                                return Err(serde::de::Error::duplicate_field("primary_active"));
                            }
                            f_primary_active = Some(map.next_value::<bool>()?);
                        }
                        "relocating_shards" => {
                            if f_relocating_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("relocating_shards"));
                            }
                            f_relocating_shards = Some(map.next_value::<i32>()?);
                        }
                        "status" => {
                            if f_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            f_status =
                                Some(map.next_value::<crate::types::common::HealthStatus>()?);
                        }
                        "unassigned_shards" => {
                            if f_unassigned_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("unassigned_shards"));
                            }
                            f_unassigned_shards = Some(map.next_value::<i32>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardHealthStats {
                    active_shards: f_active_shards
                        .ok_or_else(|| serde::de::Error::missing_field("active_shards"))?,
                    initializing_shards: f_initializing_shards
                        .ok_or_else(|| serde::de::Error::missing_field("initializing_shards"))?,
                    primary_active: f_primary_active
                        .ok_or_else(|| serde::de::Error::missing_field("primary_active"))?,
                    relocating_shards: f_relocating_shards
                        .ok_or_else(|| serde::de::Error::missing_field("relocating_shards"))?,
                    status: f_status.ok_or_else(|| serde::de::Error::missing_field("status"))?,
                    unassigned_shards: f_unassigned_shards
                        .ok_or_else(|| serde::de::Error::missing_field("unassigned_shards"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardHealthStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("active_shards", &self.active_shards)?;
        map.serialize_entry("initializing_shards", &self.initializing_shards)?;
        map.serialize_entry("primary_active", &self.primary_active)?;
        map.serialize_entry("relocating_shards", &self.relocating_shards)?;
        map.serialize_entry("status", &self.status)?;
        map.serialize_entry("unassigned_shards", &self.unassigned_shards)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
