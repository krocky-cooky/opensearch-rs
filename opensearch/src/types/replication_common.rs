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

//! Response types of the `replication_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct AutoFollowStats {
    #[doc = "The list of indices that failed to replicate for this pattern."]
    pub failed_indices: Option<Vec<String>>,
    #[doc = "When the last execution of this auto-follow pattern occurred."]
    pub last_execution_time: Option<f64>,
    #[doc = "The name of the auto-follow pattern."]
    pub name: Option<String>,
    #[doc = "The number of failed calls to the leader cluster for this pattern."]
    pub num_failed_leader_calls: Option<f64>,
    #[doc = "The number of failed replication starts for this pattern."]
    pub num_failed_start_replication: Option<f64>,
    #[doc = "The number of successful replication starts for this pattern."]
    pub num_success_start_replication: Option<f64>,
    #[doc = "The pattern used for auto-following indices."]
    pub pattern: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AutoFollowStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AutoFollowStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AutoFollowStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_failed_indices: Option<Vec<String>> = None;
                let mut seen_failed_indices = false;
                let mut f_last_execution_time: Option<f64> = None;
                let mut seen_last_execution_time = false;
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_num_failed_leader_calls: Option<f64> = None;
                let mut seen_num_failed_leader_calls = false;
                let mut f_num_failed_start_replication: Option<f64> = None;
                let mut seen_num_failed_start_replication = false;
                let mut f_num_success_start_replication: Option<f64> = None;
                let mut seen_num_success_start_replication = false;
                let mut f_pattern: Option<String> = None;
                let mut seen_pattern = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "failed_indices" => {
                            if seen_failed_indices {
                                return Err(serde::de::Error::duplicate_field("failed_indices"));
                            }
                            seen_failed_indices = true;
                            f_failed_indices = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "last_execution_time" => {
                            if seen_last_execution_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "last_execution_time",
                                ));
                            }
                            seen_last_execution_time = true;
                            f_last_execution_time = map.next_value::<Option<f64>>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "num_failed_leader_calls" => {
                            if seen_num_failed_leader_calls {
                                return Err(serde::de::Error::duplicate_field(
                                    "num_failed_leader_calls",
                                ));
                            }
                            seen_num_failed_leader_calls = true;
                            f_num_failed_leader_calls = map.next_value::<Option<f64>>()?;
                        }
                        "num_failed_start_replication" => {
                            if seen_num_failed_start_replication {
                                return Err(serde::de::Error::duplicate_field(
                                    "num_failed_start_replication",
                                ));
                            }
                            seen_num_failed_start_replication = true;
                            f_num_failed_start_replication = map.next_value::<Option<f64>>()?;
                        }
                        "num_success_start_replication" => {
                            if seen_num_success_start_replication {
                                return Err(serde::de::Error::duplicate_field(
                                    "num_success_start_replication",
                                ));
                            }
                            seen_num_success_start_replication = true;
                            f_num_success_start_replication = map.next_value::<Option<f64>>()?;
                        }
                        "pattern" => {
                            if seen_pattern {
                                return Err(serde::de::Error::duplicate_field("pattern"));
                            }
                            seen_pattern = true;
                            f_pattern = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AutoFollowStats {
                    failed_indices: f_failed_indices,
                    last_execution_time: f_last_execution_time,
                    name: f_name,
                    num_failed_leader_calls: f_num_failed_leader_calls,
                    num_failed_start_replication: f_num_failed_start_replication,
                    num_success_start_replication: f_num_success_start_replication,
                    pattern: f_pattern,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AutoFollowStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.failed_indices {
            map.serialize_entry("failed_indices", v)?;
        }
        if let Some(v) = &self.last_execution_time {
            map.serialize_entry("last_execution_time", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.num_failed_leader_calls {
            map.serialize_entry("num_failed_leader_calls", v)?;
        }
        if let Some(v) = &self.num_failed_start_replication {
            map.serialize_entry("num_failed_start_replication", v)?;
        }
        if let Some(v) = &self.num_success_start_replication {
            map.serialize_entry("num_success_start_replication", v)?;
        }
        if let Some(v) = &self.pattern {
            map.serialize_entry("pattern", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct AutoFollowStatus {
    pub autofollow_stats: Option<Vec<crate::types::replication_common::AutoFollowStats>>,
    #[doc = "The list of indices that failed to replicate."]
    pub failed_indices: Option<Vec<String>>,
    #[doc = "The number of failed calls to the leader cluster."]
    pub num_failed_leader_calls: Option<f64>,
    #[doc = "The number of failed replication starts."]
    pub num_failed_start_replication: Option<f64>,
    #[doc = "The number of successful replication starts."]
    pub num_success_start_replication: Option<f64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AutoFollowStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AutoFollowStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AutoFollowStatus object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_autofollow_stats: Option<
                    Vec<crate::types::replication_common::AutoFollowStats>,
                > = None;
                let mut seen_autofollow_stats = false;
                let mut f_failed_indices: Option<Vec<String>> = None;
                let mut seen_failed_indices = false;
                let mut f_num_failed_leader_calls: Option<f64> = None;
                let mut seen_num_failed_leader_calls = false;
                let mut f_num_failed_start_replication: Option<f64> = None;
                let mut seen_num_failed_start_replication = false;
                let mut f_num_success_start_replication: Option<f64> = None;
                let mut seen_num_success_start_replication = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "autofollow_stats" => {
                            if seen_autofollow_stats {
                                return Err(serde::de::Error::duplicate_field("autofollow_stats"));
                            }
                            seen_autofollow_stats = true;
                            f_autofollow_stats = map.next_value::<Option<Vec<crate::types::replication_common::AutoFollowStats>>>()?;
                        }
                        "failed_indices" => {
                            if seen_failed_indices {
                                return Err(serde::de::Error::duplicate_field("failed_indices"));
                            }
                            seen_failed_indices = true;
                            f_failed_indices = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "num_failed_leader_calls" => {
                            if seen_num_failed_leader_calls {
                                return Err(serde::de::Error::duplicate_field(
                                    "num_failed_leader_calls",
                                ));
                            }
                            seen_num_failed_leader_calls = true;
                            f_num_failed_leader_calls = map.next_value::<Option<f64>>()?;
                        }
                        "num_failed_start_replication" => {
                            if seen_num_failed_start_replication {
                                return Err(serde::de::Error::duplicate_field(
                                    "num_failed_start_replication",
                                ));
                            }
                            seen_num_failed_start_replication = true;
                            f_num_failed_start_replication = map.next_value::<Option<f64>>()?;
                        }
                        "num_success_start_replication" => {
                            if seen_num_success_start_replication {
                                return Err(serde::de::Error::duplicate_field(
                                    "num_success_start_replication",
                                ));
                            }
                            seen_num_success_start_replication = true;
                            f_num_success_start_replication = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AutoFollowStatus {
                    autofollow_stats: f_autofollow_stats,
                    failed_indices: f_failed_indices,
                    num_failed_leader_calls: f_num_failed_leader_calls,
                    num_failed_start_replication: f_num_failed_start_replication,
                    num_success_start_replication: f_num_success_start_replication,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AutoFollowStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.autofollow_stats {
            map.serialize_entry("autofollow_stats", v)?;
        }
        if let Some(v) = &self.failed_indices {
            map.serialize_entry("failed_indices", v)?;
        }
        if let Some(v) = &self.num_failed_leader_calls {
            map.serialize_entry("num_failed_leader_calls", v)?;
        }
        if let Some(v) = &self.num_failed_start_replication {
            map.serialize_entry("num_failed_start_replication", v)?;
        }
        if let Some(v) = &self.num_success_start_replication {
            map.serialize_entry("num_success_start_replication", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct FollowerStatus {
    #[doc = "The number of failed read requests during replication."]
    pub failed_read_requests: Option<f64>,
    #[doc = "The number of failed write requests during replication."]
    pub failed_write_requests: Option<f64>,
    #[doc = "The current checkpoint of the follower index."]
    pub follower_checkpoint: Option<f64>,
    pub index_stats: Option<
        std::collections::BTreeMap<String, crate::types::replication_common::IndexFollowerStatus>,
    >,
    #[doc = "The current checkpoint of the leader index."]
    pub leader_checkpoint: Option<f64>,
    #[doc = "The number of indexes currently bootstrapping."]
    pub num_bootstrapping_indices: Option<f64>,
    #[doc = "The number of indexes that have failed replication."]
    pub num_failed_indices: Option<f64>,
    #[doc = "The number of active index-level replication tasks."]
    pub num_index_tasks: Option<f64>,
    #[doc = "The number of indexes currently paused."]
    pub num_paused_indices: Option<f64>,
    #[doc = "The number of active shard-level replication tasks."]
    pub num_shard_tasks: Option<f64>,
    #[doc = "The number of indexes currently syncing."]
    pub num_syncing_indices: Option<f64>,
    #[doc = "The total number of operations read during replication."]
    pub operations_read: Option<f64>,
    #[doc = "The total number of operations written during replication."]
    pub operations_written: Option<f64>,
    #[doc = "The number of throttled read requests during replication."]
    pub throttled_read_requests: Option<f64>,
    #[doc = "The number of throttled write requests during replication."]
    pub throttled_write_requests: Option<f64>,
    pub total_write_time_millis: Option<crate::types::common::StringifiedEpochTimeUnitMillis>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FollowerStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FollowerStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FollowerStatus object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_failed_read_requests: Option<f64> = None;
                let mut seen_failed_read_requests = false;
                let mut f_failed_write_requests: Option<f64> = None;
                let mut seen_failed_write_requests = false;
                let mut f_follower_checkpoint: Option<f64> = None;
                let mut seen_follower_checkpoint = false;
                let mut f_index_stats: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::replication_common::IndexFollowerStatus,
                    >,
                > = None;
                let mut seen_index_stats = false;
                let mut f_leader_checkpoint: Option<f64> = None;
                let mut seen_leader_checkpoint = false;
                let mut f_num_bootstrapping_indices: Option<f64> = None;
                let mut seen_num_bootstrapping_indices = false;
                let mut f_num_failed_indices: Option<f64> = None;
                let mut seen_num_failed_indices = false;
                let mut f_num_index_tasks: Option<f64> = None;
                let mut seen_num_index_tasks = false;
                let mut f_num_paused_indices: Option<f64> = None;
                let mut seen_num_paused_indices = false;
                let mut f_num_shard_tasks: Option<f64> = None;
                let mut seen_num_shard_tasks = false;
                let mut f_num_syncing_indices: Option<f64> = None;
                let mut seen_num_syncing_indices = false;
                let mut f_operations_read: Option<f64> = None;
                let mut seen_operations_read = false;
                let mut f_operations_written: Option<f64> = None;
                let mut seen_operations_written = false;
                let mut f_throttled_read_requests: Option<f64> = None;
                let mut seen_throttled_read_requests = false;
                let mut f_throttled_write_requests: Option<f64> = None;
                let mut seen_throttled_write_requests = false;
                let mut f_total_write_time_millis: Option<
                    crate::types::common::StringifiedEpochTimeUnitMillis,
                > = None;
                let mut seen_total_write_time_millis = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "failed_read_requests" => {
                            if seen_failed_read_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "failed_read_requests",
                                ));
                            }
                            seen_failed_read_requests = true;
                            f_failed_read_requests = map.next_value::<Option<f64>>()?;
                        }
                        "failed_write_requests" => {
                            if seen_failed_write_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "failed_write_requests",
                                ));
                            }
                            seen_failed_write_requests = true;
                            f_failed_write_requests = map.next_value::<Option<f64>>()?;
                        }
                        "follower_checkpoint" => {
                            if seen_follower_checkpoint {
                                return Err(serde::de::Error::duplicate_field(
                                    "follower_checkpoint",
                                ));
                            }
                            seen_follower_checkpoint = true;
                            f_follower_checkpoint = map.next_value::<Option<f64>>()?;
                        }
                        "index_stats" => {
                            if seen_index_stats {
                                return Err(serde::de::Error::duplicate_field("index_stats"));
                            }
                            seen_index_stats = true;
                            f_index_stats = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::replication_common::IndexFollowerStatus,
                                >,
                            >>()?;
                        }
                        "leader_checkpoint" => {
                            if seen_leader_checkpoint {
                                return Err(serde::de::Error::duplicate_field("leader_checkpoint"));
                            }
                            seen_leader_checkpoint = true;
                            f_leader_checkpoint = map.next_value::<Option<f64>>()?;
                        }
                        "num_bootstrapping_indices" => {
                            if seen_num_bootstrapping_indices {
                                return Err(serde::de::Error::duplicate_field(
                                    "num_bootstrapping_indices",
                                ));
                            }
                            seen_num_bootstrapping_indices = true;
                            f_num_bootstrapping_indices = map.next_value::<Option<f64>>()?;
                        }
                        "num_failed_indices" => {
                            if seen_num_failed_indices {
                                return Err(serde::de::Error::duplicate_field(
                                    "num_failed_indices",
                                ));
                            }
                            seen_num_failed_indices = true;
                            f_num_failed_indices = map.next_value::<Option<f64>>()?;
                        }
                        "num_index_tasks" => {
                            if seen_num_index_tasks {
                                return Err(serde::de::Error::duplicate_field("num_index_tasks"));
                            }
                            seen_num_index_tasks = true;
                            f_num_index_tasks = map.next_value::<Option<f64>>()?;
                        }
                        "num_paused_indices" => {
                            if seen_num_paused_indices {
                                return Err(serde::de::Error::duplicate_field(
                                    "num_paused_indices",
                                ));
                            }
                            seen_num_paused_indices = true;
                            f_num_paused_indices = map.next_value::<Option<f64>>()?;
                        }
                        "num_shard_tasks" => {
                            if seen_num_shard_tasks {
                                return Err(serde::de::Error::duplicate_field("num_shard_tasks"));
                            }
                            seen_num_shard_tasks = true;
                            f_num_shard_tasks = map.next_value::<Option<f64>>()?;
                        }
                        "num_syncing_indices" => {
                            if seen_num_syncing_indices {
                                return Err(serde::de::Error::duplicate_field(
                                    "num_syncing_indices",
                                ));
                            }
                            seen_num_syncing_indices = true;
                            f_num_syncing_indices = map.next_value::<Option<f64>>()?;
                        }
                        "operations_read" => {
                            if seen_operations_read {
                                return Err(serde::de::Error::duplicate_field("operations_read"));
                            }
                            seen_operations_read = true;
                            f_operations_read = map.next_value::<Option<f64>>()?;
                        }
                        "operations_written" => {
                            if seen_operations_written {
                                return Err(serde::de::Error::duplicate_field(
                                    "operations_written",
                                ));
                            }
                            seen_operations_written = true;
                            f_operations_written = map.next_value::<Option<f64>>()?;
                        }
                        "throttled_read_requests" => {
                            if seen_throttled_read_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "throttled_read_requests",
                                ));
                            }
                            seen_throttled_read_requests = true;
                            f_throttled_read_requests = map.next_value::<Option<f64>>()?;
                        }
                        "throttled_write_requests" => {
                            if seen_throttled_write_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "throttled_write_requests",
                                ));
                            }
                            seen_throttled_write_requests = true;
                            f_throttled_write_requests = map.next_value::<Option<f64>>()?;
                        }
                        "total_write_time_millis" => {
                            if seen_total_write_time_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_write_time_millis",
                                ));
                            }
                            seen_total_write_time_millis = true;
                            f_total_write_time_millis = map.next_value::<Option<crate::types::common::StringifiedEpochTimeUnitMillis>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FollowerStatus {
                    failed_read_requests: f_failed_read_requests,
                    failed_write_requests: f_failed_write_requests,
                    follower_checkpoint: f_follower_checkpoint,
                    index_stats: f_index_stats,
                    leader_checkpoint: f_leader_checkpoint,
                    num_bootstrapping_indices: f_num_bootstrapping_indices,
                    num_failed_indices: f_num_failed_indices,
                    num_index_tasks: f_num_index_tasks,
                    num_paused_indices: f_num_paused_indices,
                    num_shard_tasks: f_num_shard_tasks,
                    num_syncing_indices: f_num_syncing_indices,
                    operations_read: f_operations_read,
                    operations_written: f_operations_written,
                    throttled_read_requests: f_throttled_read_requests,
                    throttled_write_requests: f_throttled_write_requests,
                    total_write_time_millis: f_total_write_time_millis,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FollowerStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.failed_read_requests {
            map.serialize_entry("failed_read_requests", v)?;
        }
        if let Some(v) = &self.failed_write_requests {
            map.serialize_entry("failed_write_requests", v)?;
        }
        if let Some(v) = &self.follower_checkpoint {
            map.serialize_entry("follower_checkpoint", v)?;
        }
        if let Some(v) = &self.index_stats {
            map.serialize_entry("index_stats", v)?;
        }
        if let Some(v) = &self.leader_checkpoint {
            map.serialize_entry("leader_checkpoint", v)?;
        }
        if let Some(v) = &self.num_bootstrapping_indices {
            map.serialize_entry("num_bootstrapping_indices", v)?;
        }
        if let Some(v) = &self.num_failed_indices {
            map.serialize_entry("num_failed_indices", v)?;
        }
        if let Some(v) = &self.num_index_tasks {
            map.serialize_entry("num_index_tasks", v)?;
        }
        if let Some(v) = &self.num_paused_indices {
            map.serialize_entry("num_paused_indices", v)?;
        }
        if let Some(v) = &self.num_shard_tasks {
            map.serialize_entry("num_shard_tasks", v)?;
        }
        if let Some(v) = &self.num_syncing_indices {
            map.serialize_entry("num_syncing_indices", v)?;
        }
        if let Some(v) = &self.operations_read {
            map.serialize_entry("operations_read", v)?;
        }
        if let Some(v) = &self.operations_written {
            map.serialize_entry("operations_written", v)?;
        }
        if let Some(v) = &self.throttled_read_requests {
            map.serialize_entry("throttled_read_requests", v)?;
        }
        if let Some(v) = &self.throttled_write_requests {
            map.serialize_entry("throttled_write_requests", v)?;
        }
        if let Some(v) = &self.total_write_time_millis {
            map.serialize_entry("total_write_time_millis", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IndexFollowerStatus {
    #[doc = "The number of failed read requests for this index during replication."]
    pub failed_read_requests: Option<f64>,
    #[doc = "The number of failed write requests for this index during replication."]
    pub failed_write_requests: Option<f64>,
    #[doc = "The current checkpoint of the follower index."]
    pub follower_checkpoint: Option<f64>,
    #[doc = "The current checkpoint of the leader index."]
    pub leader_checkpoint: Option<f64>,
    #[doc = "The number of operations read for this index during replication."]
    pub operations_read: Option<f64>,
    #[doc = "The number of operations written for this index during replication."]
    pub operations_written: Option<f64>,
    #[doc = "The number of throttled read requests for this index during replication."]
    pub throttled_read_requests: Option<f64>,
    #[doc = "The number of throttled write requests for this index during replication."]
    pub throttled_write_requests: Option<f64>,
    pub total_write_time_millis: Option<crate::types::common::StringifiedEpochTimeUnitMillis>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexFollowerStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexFollowerStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexFollowerStatus object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_failed_read_requests: Option<f64> = None;
                let mut seen_failed_read_requests = false;
                let mut f_failed_write_requests: Option<f64> = None;
                let mut seen_failed_write_requests = false;
                let mut f_follower_checkpoint: Option<f64> = None;
                let mut seen_follower_checkpoint = false;
                let mut f_leader_checkpoint: Option<f64> = None;
                let mut seen_leader_checkpoint = false;
                let mut f_operations_read: Option<f64> = None;
                let mut seen_operations_read = false;
                let mut f_operations_written: Option<f64> = None;
                let mut seen_operations_written = false;
                let mut f_throttled_read_requests: Option<f64> = None;
                let mut seen_throttled_read_requests = false;
                let mut f_throttled_write_requests: Option<f64> = None;
                let mut seen_throttled_write_requests = false;
                let mut f_total_write_time_millis: Option<
                    crate::types::common::StringifiedEpochTimeUnitMillis,
                > = None;
                let mut seen_total_write_time_millis = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "failed_read_requests" => {
                            if seen_failed_read_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "failed_read_requests",
                                ));
                            }
                            seen_failed_read_requests = true;
                            f_failed_read_requests = map.next_value::<Option<f64>>()?;
                        }
                        "failed_write_requests" => {
                            if seen_failed_write_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "failed_write_requests",
                                ));
                            }
                            seen_failed_write_requests = true;
                            f_failed_write_requests = map.next_value::<Option<f64>>()?;
                        }
                        "follower_checkpoint" => {
                            if seen_follower_checkpoint {
                                return Err(serde::de::Error::duplicate_field(
                                    "follower_checkpoint",
                                ));
                            }
                            seen_follower_checkpoint = true;
                            f_follower_checkpoint = map.next_value::<Option<f64>>()?;
                        }
                        "leader_checkpoint" => {
                            if seen_leader_checkpoint {
                                return Err(serde::de::Error::duplicate_field("leader_checkpoint"));
                            }
                            seen_leader_checkpoint = true;
                            f_leader_checkpoint = map.next_value::<Option<f64>>()?;
                        }
                        "operations_read" => {
                            if seen_operations_read {
                                return Err(serde::de::Error::duplicate_field("operations_read"));
                            }
                            seen_operations_read = true;
                            f_operations_read = map.next_value::<Option<f64>>()?;
                        }
                        "operations_written" => {
                            if seen_operations_written {
                                return Err(serde::de::Error::duplicate_field(
                                    "operations_written",
                                ));
                            }
                            seen_operations_written = true;
                            f_operations_written = map.next_value::<Option<f64>>()?;
                        }
                        "throttled_read_requests" => {
                            if seen_throttled_read_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "throttled_read_requests",
                                ));
                            }
                            seen_throttled_read_requests = true;
                            f_throttled_read_requests = map.next_value::<Option<f64>>()?;
                        }
                        "throttled_write_requests" => {
                            if seen_throttled_write_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "throttled_write_requests",
                                ));
                            }
                            seen_throttled_write_requests = true;
                            f_throttled_write_requests = map.next_value::<Option<f64>>()?;
                        }
                        "total_write_time_millis" => {
                            if seen_total_write_time_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_write_time_millis",
                                ));
                            }
                            seen_total_write_time_millis = true;
                            f_total_write_time_millis = map.next_value::<Option<crate::types::common::StringifiedEpochTimeUnitMillis>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexFollowerStatus {
                    failed_read_requests: f_failed_read_requests,
                    failed_write_requests: f_failed_write_requests,
                    follower_checkpoint: f_follower_checkpoint,
                    leader_checkpoint: f_leader_checkpoint,
                    operations_read: f_operations_read,
                    operations_written: f_operations_written,
                    throttled_read_requests: f_throttled_read_requests,
                    throttled_write_requests: f_throttled_write_requests,
                    total_write_time_millis: f_total_write_time_millis,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexFollowerStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.failed_read_requests {
            map.serialize_entry("failed_read_requests", v)?;
        }
        if let Some(v) = &self.failed_write_requests {
            map.serialize_entry("failed_write_requests", v)?;
        }
        if let Some(v) = &self.follower_checkpoint {
            map.serialize_entry("follower_checkpoint", v)?;
        }
        if let Some(v) = &self.leader_checkpoint {
            map.serialize_entry("leader_checkpoint", v)?;
        }
        if let Some(v) = &self.operations_read {
            map.serialize_entry("operations_read", v)?;
        }
        if let Some(v) = &self.operations_written {
            map.serialize_entry("operations_written", v)?;
        }
        if let Some(v) = &self.throttled_read_requests {
            map.serialize_entry("throttled_read_requests", v)?;
        }
        if let Some(v) = &self.throttled_write_requests {
            map.serialize_entry("throttled_write_requests", v)?;
        }
        if let Some(v) = &self.total_write_time_millis {
            map.serialize_entry("total_write_time_millis", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IndexStatus {
    pub bytes_read: Option<crate::types::common::ByteCount>,
    #[doc = "The number of operations read for this index during replication."]
    pub operations_read: Option<f64>,
    #[doc = "The number of operations read from Lucene for this index during replication."]
    pub operations_read_lucene: Option<f64>,
    #[doc = "The number of operations read from the translog for this index during replication."]
    pub operations_read_translog: Option<f64>,
    pub total_read_time_lucene_millis: Option<crate::types::common::StringifiedEpochTimeUnitMillis>,
    pub total_read_time_translog_millis:
        Option<crate::types::common::StringifiedEpochTimeUnitMillis>,
    pub translog_size_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexStatus object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_bytes_read: Option<crate::types::common::ByteCount> = None;
                let mut seen_bytes_read = false;
                let mut f_operations_read: Option<f64> = None;
                let mut seen_operations_read = false;
                let mut f_operations_read_lucene: Option<f64> = None;
                let mut seen_operations_read_lucene = false;
                let mut f_operations_read_translog: Option<f64> = None;
                let mut seen_operations_read_translog = false;
                let mut f_total_read_time_lucene_millis: Option<
                    crate::types::common::StringifiedEpochTimeUnitMillis,
                > = None;
                let mut seen_total_read_time_lucene_millis = false;
                let mut f_total_read_time_translog_millis: Option<
                    crate::types::common::StringifiedEpochTimeUnitMillis,
                > = None;
                let mut seen_total_read_time_translog_millis = false;
                let mut f_translog_size_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_translog_size_bytes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "bytes_read" => {
                            if seen_bytes_read {
                                return Err(serde::de::Error::duplicate_field("bytes_read"));
                            }
                            seen_bytes_read = true;
                            f_bytes_read =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "operations_read" => {
                            if seen_operations_read {
                                return Err(serde::de::Error::duplicate_field("operations_read"));
                            }
                            seen_operations_read = true;
                            f_operations_read = map.next_value::<Option<f64>>()?;
                        }
                        "operations_read_lucene" => {
                            if seen_operations_read_lucene {
                                return Err(serde::de::Error::duplicate_field(
                                    "operations_read_lucene",
                                ));
                            }
                            seen_operations_read_lucene = true;
                            f_operations_read_lucene = map.next_value::<Option<f64>>()?;
                        }
                        "operations_read_translog" => {
                            if seen_operations_read_translog {
                                return Err(serde::de::Error::duplicate_field(
                                    "operations_read_translog",
                                ));
                            }
                            seen_operations_read_translog = true;
                            f_operations_read_translog = map.next_value::<Option<f64>>()?;
                        }
                        "total_read_time_lucene_millis" => {
                            if seen_total_read_time_lucene_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_read_time_lucene_millis",
                                ));
                            }
                            seen_total_read_time_lucene_millis = true;
                            f_total_read_time_lucene_millis = map.next_value::<Option<crate::types::common::StringifiedEpochTimeUnitMillis>>()?;
                        }
                        "total_read_time_translog_millis" => {
                            if seen_total_read_time_translog_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_read_time_translog_millis",
                                ));
                            }
                            seen_total_read_time_translog_millis = true;
                            f_total_read_time_translog_millis = map.next_value::<Option<crate::types::common::StringifiedEpochTimeUnitMillis>>()?;
                        }
                        "translog_size_bytes" => {
                            if seen_translog_size_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "translog_size_bytes",
                                ));
                            }
                            seen_translog_size_bytes = true;
                            f_translog_size_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexStatus {
                    bytes_read: f_bytes_read,
                    operations_read: f_operations_read,
                    operations_read_lucene: f_operations_read_lucene,
                    operations_read_translog: f_operations_read_translog,
                    total_read_time_lucene_millis: f_total_read_time_lucene_millis,
                    total_read_time_translog_millis: f_total_read_time_translog_millis,
                    translog_size_bytes: f_translog_size_bytes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.bytes_read {
            map.serialize_entry("bytes_read", v)?;
        }
        if let Some(v) = &self.operations_read {
            map.serialize_entry("operations_read", v)?;
        }
        if let Some(v) = &self.operations_read_lucene {
            map.serialize_entry("operations_read_lucene", v)?;
        }
        if let Some(v) = &self.operations_read_translog {
            map.serialize_entry("operations_read_translog", v)?;
        }
        if let Some(v) = &self.total_read_time_lucene_millis {
            map.serialize_entry("total_read_time_lucene_millis", v)?;
        }
        if let Some(v) = &self.total_read_time_translog_millis {
            map.serialize_entry("total_read_time_translog_millis", v)?;
        }
        if let Some(v) = &self.translog_size_bytes {
            map.serialize_entry("translog_size_bytes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct LeaderStatus {
    pub bytes_read: Option<crate::types::common::ByteCount>,
    pub index_stats:
        Option<std::collections::BTreeMap<String, crate::types::replication_common::IndexStatus>>,
    #[doc = "The number of indexes being replicated."]
    pub num_replicated_indices: Option<f64>,
    #[doc = "The total number of operations read during replication."]
    pub operations_read: Option<f64>,
    #[doc = "The number of operations read from Lucene during replication."]
    pub operations_read_lucene: Option<f64>,
    #[doc = "The number of operations read from the translog during replication."]
    pub operations_read_translog: Option<f64>,
    pub total_read_time_lucene_millis: Option<crate::types::common::StringifiedEpochTimeUnitMillis>,
    pub total_read_time_translog_millis:
        Option<crate::types::common::StringifiedEpochTimeUnitMillis>,
    pub translog_size_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for LeaderStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = LeaderStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a LeaderStatus object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_bytes_read: Option<crate::types::common::ByteCount> = None;
                let mut seen_bytes_read = false;
                let mut f_index_stats: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::replication_common::IndexStatus,
                    >,
                > = None;
                let mut seen_index_stats = false;
                let mut f_num_replicated_indices: Option<f64> = None;
                let mut seen_num_replicated_indices = false;
                let mut f_operations_read: Option<f64> = None;
                let mut seen_operations_read = false;
                let mut f_operations_read_lucene: Option<f64> = None;
                let mut seen_operations_read_lucene = false;
                let mut f_operations_read_translog: Option<f64> = None;
                let mut seen_operations_read_translog = false;
                let mut f_total_read_time_lucene_millis: Option<
                    crate::types::common::StringifiedEpochTimeUnitMillis,
                > = None;
                let mut seen_total_read_time_lucene_millis = false;
                let mut f_total_read_time_translog_millis: Option<
                    crate::types::common::StringifiedEpochTimeUnitMillis,
                > = None;
                let mut seen_total_read_time_translog_millis = false;
                let mut f_translog_size_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_translog_size_bytes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "bytes_read" => {
                            if seen_bytes_read {
                                return Err(serde::de::Error::duplicate_field("bytes_read"));
                            }
                            seen_bytes_read = true;
                            f_bytes_read =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "index_stats" => {
                            if seen_index_stats {
                                return Err(serde::de::Error::duplicate_field("index_stats"));
                            }
                            seen_index_stats = true;
                            f_index_stats = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::replication_common::IndexStatus,
                                >,
                            >>()?;
                        }
                        "num_replicated_indices" => {
                            if seen_num_replicated_indices {
                                return Err(serde::de::Error::duplicate_field(
                                    "num_replicated_indices",
                                ));
                            }
                            seen_num_replicated_indices = true;
                            f_num_replicated_indices = map.next_value::<Option<f64>>()?;
                        }
                        "operations_read" => {
                            if seen_operations_read {
                                return Err(serde::de::Error::duplicate_field("operations_read"));
                            }
                            seen_operations_read = true;
                            f_operations_read = map.next_value::<Option<f64>>()?;
                        }
                        "operations_read_lucene" => {
                            if seen_operations_read_lucene {
                                return Err(serde::de::Error::duplicate_field(
                                    "operations_read_lucene",
                                ));
                            }
                            seen_operations_read_lucene = true;
                            f_operations_read_lucene = map.next_value::<Option<f64>>()?;
                        }
                        "operations_read_translog" => {
                            if seen_operations_read_translog {
                                return Err(serde::de::Error::duplicate_field(
                                    "operations_read_translog",
                                ));
                            }
                            seen_operations_read_translog = true;
                            f_operations_read_translog = map.next_value::<Option<f64>>()?;
                        }
                        "total_read_time_lucene_millis" => {
                            if seen_total_read_time_lucene_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_read_time_lucene_millis",
                                ));
                            }
                            seen_total_read_time_lucene_millis = true;
                            f_total_read_time_lucene_millis = map.next_value::<Option<crate::types::common::StringifiedEpochTimeUnitMillis>>()?;
                        }
                        "total_read_time_translog_millis" => {
                            if seen_total_read_time_translog_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_read_time_translog_millis",
                                ));
                            }
                            seen_total_read_time_translog_millis = true;
                            f_total_read_time_translog_millis = map.next_value::<Option<crate::types::common::StringifiedEpochTimeUnitMillis>>()?;
                        }
                        "translog_size_bytes" => {
                            if seen_translog_size_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "translog_size_bytes",
                                ));
                            }
                            seen_translog_size_bytes = true;
                            f_translog_size_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(LeaderStatus {
                    bytes_read: f_bytes_read,
                    index_stats: f_index_stats,
                    num_replicated_indices: f_num_replicated_indices,
                    operations_read: f_operations_read,
                    operations_read_lucene: f_operations_read_lucene,
                    operations_read_translog: f_operations_read_translog,
                    total_read_time_lucene_millis: f_total_read_time_lucene_millis,
                    total_read_time_translog_millis: f_total_read_time_translog_millis,
                    translog_size_bytes: f_translog_size_bytes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for LeaderStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.bytes_read {
            map.serialize_entry("bytes_read", v)?;
        }
        if let Some(v) = &self.index_stats {
            map.serialize_entry("index_stats", v)?;
        }
        if let Some(v) = &self.num_replicated_indices {
            map.serialize_entry("num_replicated_indices", v)?;
        }
        if let Some(v) = &self.operations_read {
            map.serialize_entry("operations_read", v)?;
        }
        if let Some(v) = &self.operations_read_lucene {
            map.serialize_entry("operations_read_lucene", v)?;
        }
        if let Some(v) = &self.operations_read_translog {
            map.serialize_entry("operations_read_translog", v)?;
        }
        if let Some(v) = &self.total_read_time_lucene_millis {
            map.serialize_entry("total_read_time_lucene_millis", v)?;
        }
        if let Some(v) = &self.total_read_time_translog_millis {
            map.serialize_entry("total_read_time_translog_millis", v)?;
        }
        if let Some(v) = &self.translog_size_bytes {
            map.serialize_entry("translog_size_bytes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReplicationStatus {
    #[doc = "`BOOTSTRAPPING`"]
    Bootstrapping,
    #[doc = "`PAUSED`"]
    Paused,
    #[doc = "`REPLICATION NOT IN PROGRESS`"]
    ReplicationNotInProgress,
    #[doc = "`RUNNING`"]
    Running,
    #[doc = "`SYNCING`"]
    Syncing,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ReplicationStatus {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ReplicationStatus::Bootstrapping => "BOOTSTRAPPING",
            ReplicationStatus::Paused => "PAUSED",
            ReplicationStatus::ReplicationNotInProgress => "REPLICATION NOT IN PROGRESS",
            ReplicationStatus::Running => "RUNNING",
            ReplicationStatus::Syncing => "SYNCING",
            ReplicationStatus::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ReplicationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReplicationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "BOOTSTRAPPING" => ReplicationStatus::Bootstrapping,
            "PAUSED" => ReplicationStatus::Paused,
            "REPLICATION NOT IN PROGRESS" => ReplicationStatus::ReplicationNotInProgress,
            "RUNNING" => ReplicationStatus::Running,
            "SYNCING" => ReplicationStatus::Syncing,
            _ => ReplicationStatus::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Status {
    pub follower_index: Option<crate::types::common::IndexName>,
    pub leader_alias: Option<crate::types::common::IndexAlias>,
    pub leader_index: Option<crate::types::common::IndexName>,
    #[doc = "The reason for the current replication status."]
    pub reason: Option<String>,
    #[doc = "The current status of the replication process."]
    pub status: Option<crate::types::replication_common::ReplicationStatus>,
    pub syncing_details: Option<crate::types::replication_common::SyncingDetails>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Status;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Status object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_follower_index: Option<crate::types::common::IndexName> = None;
                let mut seen_follower_index = false;
                let mut f_leader_alias: Option<crate::types::common::IndexAlias> = None;
                let mut seen_leader_alias = false;
                let mut f_leader_index: Option<crate::types::common::IndexName> = None;
                let mut seen_leader_index = false;
                let mut f_reason: Option<String> = None;
                let mut seen_reason = false;
                let mut f_status: Option<crate::types::replication_common::ReplicationStatus> =
                    None;
                let mut seen_status = false;
                let mut f_syncing_details: Option<
                    crate::types::replication_common::SyncingDetails,
                > = None;
                let mut seen_syncing_details = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "follower_index" => {
                            if seen_follower_index {
                                return Err(serde::de::Error::duplicate_field("follower_index"));
                            }
                            seen_follower_index = true;
                            f_follower_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "leader_alias" => {
                            if seen_leader_alias {
                                return Err(serde::de::Error::duplicate_field("leader_alias"));
                            }
                            seen_leader_alias = true;
                            f_leader_alias =
                                map.next_value::<Option<crate::types::common::IndexAlias>>()?;
                        }
                        "leader_index" => {
                            if seen_leader_index {
                                return Err(serde::de::Error::duplicate_field("leader_index"));
                            }
                            seen_leader_index = true;
                            f_leader_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "reason" => {
                            if seen_reason {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            seen_reason = true;
                            f_reason = map.next_value::<Option<String>>()?;
                        }
                        "status" => {
                            if seen_status {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            seen_status = true;
                            f_status = map.next_value::<Option<crate::types::replication_common::ReplicationStatus>>()?;
                        }
                        "syncing_details" => {
                            if seen_syncing_details {
                                return Err(serde::de::Error::duplicate_field("syncing_details"));
                            }
                            seen_syncing_details = true;
                            f_syncing_details = map.next_value::<Option<crate::types::replication_common::SyncingDetails>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Status {
                    follower_index: f_follower_index,
                    leader_alias: f_leader_alias,
                    leader_index: f_leader_index,
                    reason: f_reason,
                    status: f_status,
                    syncing_details: f_syncing_details,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.follower_index {
            map.serialize_entry("follower_index", v)?;
        }
        if let Some(v) = &self.leader_alias {
            map.serialize_entry("leader_alias", v)?;
        }
        if let Some(v) = &self.leader_index {
            map.serialize_entry("leader_index", v)?;
        }
        if let Some(v) = &self.reason {
            map.serialize_entry("reason", v)?;
        }
        if let Some(v) = &self.status {
            map.serialize_entry("status", v)?;
        }
        if let Some(v) = &self.syncing_details {
            map.serialize_entry("syncing_details", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SyncingDetails {
    #[doc = "The checkpoint of the follower index in the replication process."]
    pub follower_checkpoint: Option<i64>,
    #[doc = "The checkpoint of the leader index in the replication process."]
    pub leader_checkpoint: Option<i64>,
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SyncingDetails {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SyncingDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SyncingDetails object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_follower_checkpoint: Option<i64> = None;
                let mut seen_follower_checkpoint = false;
                let mut f_leader_checkpoint: Option<i64> = None;
                let mut seen_leader_checkpoint = false;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut seen_seq_no = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "follower_checkpoint" => {
                            if seen_follower_checkpoint {
                                return Err(serde::de::Error::duplicate_field(
                                    "follower_checkpoint",
                                ));
                            }
                            seen_follower_checkpoint = true;
                            f_follower_checkpoint = map.next_value::<Option<i64>>()?;
                        }
                        "leader_checkpoint" => {
                            if seen_leader_checkpoint {
                                return Err(serde::de::Error::duplicate_field("leader_checkpoint"));
                            }
                            seen_leader_checkpoint = true;
                            f_leader_checkpoint = map.next_value::<Option<i64>>()?;
                        }
                        "seq_no" => {
                            if seen_seq_no {
                                return Err(serde::de::Error::duplicate_field("seq_no"));
                            }
                            seen_seq_no = true;
                            f_seq_no =
                                map.next_value::<Option<crate::types::common::SequenceNumber>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SyncingDetails {
                    follower_checkpoint: f_follower_checkpoint,
                    leader_checkpoint: f_leader_checkpoint,
                    seq_no: f_seq_no,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SyncingDetails {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.follower_checkpoint {
            map.serialize_entry("follower_checkpoint", v)?;
        }
        if let Some(v) = &self.leader_checkpoint {
            map.serialize_entry("leader_checkpoint", v)?;
        }
        if let Some(v) = &self.seq_no {
            map.serialize_entry("seq_no", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
