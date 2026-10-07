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

//! Response types of the `cat_segment_replication` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct SegmentReplicationRecord {
    pub bytes: Option<String>,
    pub bytes_behind: Option<String>,
    pub bytes_fetched: Option<String>,
    pub bytes_percent: Option<crate::types::common::PercentageString>,
    pub bytes_total: Option<String>,
    pub checkpoints_behind: Option<String>,
    pub current_lag: Option<String>,
    pub file_diff_stage_time_taken: Option<String>,
    pub files: Option<String>,
    pub files_fetched: Option<String>,
    pub files_percent: Option<crate::types::common::PercentageString>,
    pub files_total: Option<String>,
    pub finalize_replication_stage_time_taken: Option<String>,
    pub get_checkpoint_info_stage_time_taken: Option<String>,
    pub get_files_stage_time_taken: Option<String>,
    pub last_completed_lag: Option<String>,
    pub rejected_requests: Option<String>,
    pub replicating_stage_time_taken: Option<String>,
    pub shard_id: Option<String>,
    pub stage: Option<String>,
    pub start_time: Option<String>,
    pub stop_time: Option<String>,
    pub target_host: Option<String>,
    pub target_node: Option<String>,
    pub time: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SegmentReplicationRecord {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SegmentReplicationRecord;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SegmentReplicationRecord object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_bytes: Option<String> = None;
                let mut seen_bytes = false;
                let mut f_bytes_behind: Option<String> = None;
                let mut seen_bytes_behind = false;
                let mut f_bytes_fetched: Option<String> = None;
                let mut seen_bytes_fetched = false;
                let mut f_bytes_percent: Option<crate::types::common::PercentageString> = None;
                let mut seen_bytes_percent = false;
                let mut f_bytes_total: Option<String> = None;
                let mut seen_bytes_total = false;
                let mut f_checkpoints_behind: Option<String> = None;
                let mut seen_checkpoints_behind = false;
                let mut f_current_lag: Option<String> = None;
                let mut seen_current_lag = false;
                let mut f_file_diff_stage_time_taken: Option<String> = None;
                let mut seen_file_diff_stage_time_taken = false;
                let mut f_files: Option<String> = None;
                let mut seen_files = false;
                let mut f_files_fetched: Option<String> = None;
                let mut seen_files_fetched = false;
                let mut f_files_percent: Option<crate::types::common::PercentageString> = None;
                let mut seen_files_percent = false;
                let mut f_files_total: Option<String> = None;
                let mut seen_files_total = false;
                let mut f_finalize_replication_stage_time_taken: Option<String> = None;
                let mut seen_finalize_replication_stage_time_taken = false;
                let mut f_get_checkpoint_info_stage_time_taken: Option<String> = None;
                let mut seen_get_checkpoint_info_stage_time_taken = false;
                let mut f_get_files_stage_time_taken: Option<String> = None;
                let mut seen_get_files_stage_time_taken = false;
                let mut f_last_completed_lag: Option<String> = None;
                let mut seen_last_completed_lag = false;
                let mut f_rejected_requests: Option<String> = None;
                let mut seen_rejected_requests = false;
                let mut f_replicating_stage_time_taken: Option<String> = None;
                let mut seen_replicating_stage_time_taken = false;
                let mut f_shard_id: Option<String> = None;
                let mut seen_shard_id = false;
                let mut f_stage: Option<String> = None;
                let mut seen_stage = false;
                let mut f_start_time: Option<String> = None;
                let mut seen_start_time = false;
                let mut f_stop_time: Option<String> = None;
                let mut seen_stop_time = false;
                let mut f_target_host: Option<String> = None;
                let mut seen_target_host = false;
                let mut f_target_node: Option<String> = None;
                let mut seen_target_node = false;
                let mut f_time: Option<String> = None;
                let mut seen_time = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "bytes" => {
                            if seen_bytes {
                                return Err(serde::de::Error::duplicate_field("bytes"));
                            }
                            seen_bytes = true;
                            f_bytes = map.next_value::<Option<String>>()?;
                        }
                        "bytes_behind" => {
                            if seen_bytes_behind {
                                return Err(serde::de::Error::duplicate_field("bytes_behind"));
                            }
                            seen_bytes_behind = true;
                            f_bytes_behind = map.next_value::<Option<String>>()?;
                        }
                        "bytes_fetched" => {
                            if seen_bytes_fetched {
                                return Err(serde::de::Error::duplicate_field("bytes_fetched"));
                            }
                            seen_bytes_fetched = true;
                            f_bytes_fetched = map.next_value::<Option<String>>()?;
                        }
                        "bytes_percent" => {
                            if seen_bytes_percent {
                                return Err(serde::de::Error::duplicate_field("bytes_percent"));
                            }
                            seen_bytes_percent = true;
                            f_bytes_percent =
                                map.next_value::<Option<crate::types::common::PercentageString>>()?;
                        }
                        "bytes_total" => {
                            if seen_bytes_total {
                                return Err(serde::de::Error::duplicate_field("bytes_total"));
                            }
                            seen_bytes_total = true;
                            f_bytes_total = map.next_value::<Option<String>>()?;
                        }
                        "checkpoints_behind" => {
                            if seen_checkpoints_behind {
                                return Err(serde::de::Error::duplicate_field(
                                    "checkpoints_behind",
                                ));
                            }
                            seen_checkpoints_behind = true;
                            f_checkpoints_behind = map.next_value::<Option<String>>()?;
                        }
                        "current_lag" => {
                            if seen_current_lag {
                                return Err(serde::de::Error::duplicate_field("current_lag"));
                            }
                            seen_current_lag = true;
                            f_current_lag = map.next_value::<Option<String>>()?;
                        }
                        "file_diff_stage_time_taken" => {
                            if seen_file_diff_stage_time_taken {
                                return Err(serde::de::Error::duplicate_field(
                                    "file_diff_stage_time_taken",
                                ));
                            }
                            seen_file_diff_stage_time_taken = true;
                            f_file_diff_stage_time_taken = map.next_value::<Option<String>>()?;
                        }
                        "files" => {
                            if seen_files {
                                return Err(serde::de::Error::duplicate_field("files"));
                            }
                            seen_files = true;
                            f_files = map.next_value::<Option<String>>()?;
                        }
                        "files_fetched" => {
                            if seen_files_fetched {
                                return Err(serde::de::Error::duplicate_field("files_fetched"));
                            }
                            seen_files_fetched = true;
                            f_files_fetched = map.next_value::<Option<String>>()?;
                        }
                        "files_percent" => {
                            if seen_files_percent {
                                return Err(serde::de::Error::duplicate_field("files_percent"));
                            }
                            seen_files_percent = true;
                            f_files_percent =
                                map.next_value::<Option<crate::types::common::PercentageString>>()?;
                        }
                        "files_total" => {
                            if seen_files_total {
                                return Err(serde::de::Error::duplicate_field("files_total"));
                            }
                            seen_files_total = true;
                            f_files_total = map.next_value::<Option<String>>()?;
                        }
                        "finalize_replication_stage_time_taken" => {
                            if seen_finalize_replication_stage_time_taken {
                                return Err(serde::de::Error::duplicate_field(
                                    "finalize_replication_stage_time_taken",
                                ));
                            }
                            seen_finalize_replication_stage_time_taken = true;
                            f_finalize_replication_stage_time_taken =
                                map.next_value::<Option<String>>()?;
                        }
                        "get_checkpoint_info_stage_time_taken" => {
                            if seen_get_checkpoint_info_stage_time_taken {
                                return Err(serde::de::Error::duplicate_field(
                                    "get_checkpoint_info_stage_time_taken",
                                ));
                            }
                            seen_get_checkpoint_info_stage_time_taken = true;
                            f_get_checkpoint_info_stage_time_taken =
                                map.next_value::<Option<String>>()?;
                        }
                        "get_files_stage_time_taken" => {
                            if seen_get_files_stage_time_taken {
                                return Err(serde::de::Error::duplicate_field(
                                    "get_files_stage_time_taken",
                                ));
                            }
                            seen_get_files_stage_time_taken = true;
                            f_get_files_stage_time_taken = map.next_value::<Option<String>>()?;
                        }
                        "last_completed_lag" => {
                            if seen_last_completed_lag {
                                return Err(serde::de::Error::duplicate_field(
                                    "last_completed_lag",
                                ));
                            }
                            seen_last_completed_lag = true;
                            f_last_completed_lag = map.next_value::<Option<String>>()?;
                        }
                        "rejected_requests" => {
                            if seen_rejected_requests {
                                return Err(serde::de::Error::duplicate_field("rejected_requests"));
                            }
                            seen_rejected_requests = true;
                            f_rejected_requests = map.next_value::<Option<String>>()?;
                        }
                        "replicating_stage_time_taken" => {
                            if seen_replicating_stage_time_taken {
                                return Err(serde::de::Error::duplicate_field(
                                    "replicating_stage_time_taken",
                                ));
                            }
                            seen_replicating_stage_time_taken = true;
                            f_replicating_stage_time_taken = map.next_value::<Option<String>>()?;
                        }
                        "shardId" => {
                            if seen_shard_id {
                                return Err(serde::de::Error::duplicate_field("shardId"));
                            }
                            seen_shard_id = true;
                            f_shard_id = map.next_value::<Option<String>>()?;
                        }
                        "stage" => {
                            if seen_stage {
                                return Err(serde::de::Error::duplicate_field("stage"));
                            }
                            seen_stage = true;
                            f_stage = map.next_value::<Option<String>>()?;
                        }
                        "start_time" => {
                            if seen_start_time {
                                return Err(serde::de::Error::duplicate_field("start_time"));
                            }
                            seen_start_time = true;
                            f_start_time = map.next_value::<Option<String>>()?;
                        }
                        "stop_time" => {
                            if seen_stop_time {
                                return Err(serde::de::Error::duplicate_field("stop_time"));
                            }
                            seen_stop_time = true;
                            f_stop_time = map.next_value::<Option<String>>()?;
                        }
                        "target_host" => {
                            if seen_target_host {
                                return Err(serde::de::Error::duplicate_field("target_host"));
                            }
                            seen_target_host = true;
                            f_target_host = map.next_value::<Option<String>>()?;
                        }
                        "target_node" => {
                            if seen_target_node {
                                return Err(serde::de::Error::duplicate_field("target_node"));
                            }
                            seen_target_node = true;
                            f_target_node = map.next_value::<Option<String>>()?;
                        }
                        "time" => {
                            if seen_time {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            seen_time = true;
                            f_time = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SegmentReplicationRecord {
                    bytes: f_bytes,
                    bytes_behind: f_bytes_behind,
                    bytes_fetched: f_bytes_fetched,
                    bytes_percent: f_bytes_percent,
                    bytes_total: f_bytes_total,
                    checkpoints_behind: f_checkpoints_behind,
                    current_lag: f_current_lag,
                    file_diff_stage_time_taken: f_file_diff_stage_time_taken,
                    files: f_files,
                    files_fetched: f_files_fetched,
                    files_percent: f_files_percent,
                    files_total: f_files_total,
                    finalize_replication_stage_time_taken: f_finalize_replication_stage_time_taken,
                    get_checkpoint_info_stage_time_taken: f_get_checkpoint_info_stage_time_taken,
                    get_files_stage_time_taken: f_get_files_stage_time_taken,
                    last_completed_lag: f_last_completed_lag,
                    rejected_requests: f_rejected_requests,
                    replicating_stage_time_taken: f_replicating_stage_time_taken,
                    shard_id: f_shard_id,
                    stage: f_stage,
                    start_time: f_start_time,
                    stop_time: f_stop_time,
                    target_host: f_target_host,
                    target_node: f_target_node,
                    time: f_time,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SegmentReplicationRecord {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.bytes {
            map.serialize_entry("bytes", v)?;
        }
        if let Some(v) = &self.bytes_behind {
            map.serialize_entry("bytes_behind", v)?;
        }
        if let Some(v) = &self.bytes_fetched {
            map.serialize_entry("bytes_fetched", v)?;
        }
        if let Some(v) = &self.bytes_percent {
            map.serialize_entry("bytes_percent", v)?;
        }
        if let Some(v) = &self.bytes_total {
            map.serialize_entry("bytes_total", v)?;
        }
        if let Some(v) = &self.checkpoints_behind {
            map.serialize_entry("checkpoints_behind", v)?;
        }
        if let Some(v) = &self.current_lag {
            map.serialize_entry("current_lag", v)?;
        }
        if let Some(v) = &self.file_diff_stage_time_taken {
            map.serialize_entry("file_diff_stage_time_taken", v)?;
        }
        if let Some(v) = &self.files {
            map.serialize_entry("files", v)?;
        }
        if let Some(v) = &self.files_fetched {
            map.serialize_entry("files_fetched", v)?;
        }
        if let Some(v) = &self.files_percent {
            map.serialize_entry("files_percent", v)?;
        }
        if let Some(v) = &self.files_total {
            map.serialize_entry("files_total", v)?;
        }
        if let Some(v) = &self.finalize_replication_stage_time_taken {
            map.serialize_entry("finalize_replication_stage_time_taken", v)?;
        }
        if let Some(v) = &self.get_checkpoint_info_stage_time_taken {
            map.serialize_entry("get_checkpoint_info_stage_time_taken", v)?;
        }
        if let Some(v) = &self.get_files_stage_time_taken {
            map.serialize_entry("get_files_stage_time_taken", v)?;
        }
        if let Some(v) = &self.last_completed_lag {
            map.serialize_entry("last_completed_lag", v)?;
        }
        if let Some(v) = &self.rejected_requests {
            map.serialize_entry("rejected_requests", v)?;
        }
        if let Some(v) = &self.replicating_stage_time_taken {
            map.serialize_entry("replicating_stage_time_taken", v)?;
        }
        if let Some(v) = &self.shard_id {
            map.serialize_entry("shardId", v)?;
        }
        if let Some(v) = &self.stage {
            map.serialize_entry("stage", v)?;
        }
        if let Some(v) = &self.start_time {
            map.serialize_entry("start_time", v)?;
        }
        if let Some(v) = &self.stop_time {
            map.serialize_entry("stop_time", v)?;
        }
        if let Some(v) = &self.target_host {
            map.serialize_entry("target_host", v)?;
        }
        if let Some(v) = &self.target_node {
            map.serialize_entry("target_node", v)?;
        }
        if let Some(v) = &self.time {
            map.serialize_entry("time", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `cat.segment_replication` API"]
pub type SegmentReplicationResponse = crate::types::cat::SegmentReplicationResponse;
