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

//! Response types of the `cat_recovery` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct RecoveryRecord {
    #[doc = "The number of bytes to recover."]
    pub bytes: Option<String>,
    pub bytes_percent: Option<crate::types::common::PercentageString>,
    #[doc = "The bytes recovered."]
    pub bytes_recovered: Option<String>,
    #[doc = "The total number of bytes."]
    pub bytes_total: Option<String>,
    #[doc = "The number of files to recover."]
    pub files: Option<String>,
    pub files_percent: Option<crate::types::common::PercentageString>,
    #[doc = "The files recovered."]
    pub files_recovered: Option<String>,
    #[doc = "The total number of files."]
    pub files_total: Option<String>,
    pub index: Option<crate::types::common::IndexName>,
    #[doc = "The repository name."]
    pub repository: Option<String>,
    #[doc = "The shard name."]
    pub shard: Option<String>,
    #[doc = "The snapshot name."]
    pub snapshot: Option<String>,
    #[doc = "The source host."]
    pub source_host: Option<String>,
    #[doc = "The source node name."]
    pub source_node: Option<String>,
    #[doc = "The recovery stage."]
    pub stage: Option<String>,
    pub start_time: Option<crate::types::common::DateTime>,
    pub start_time_millis: Option<crate::types::common::StringifiedEpochTimeUnitMillis>,
    pub stop_time: Option<crate::types::common::DateTime>,
    pub stop_time_millis: Option<crate::types::common::StringifiedEpochTimeUnitMillis>,
    #[doc = "The target host."]
    pub target_host: Option<String>,
    #[doc = "The target node name."]
    pub target_node: Option<String>,
    pub time: Option<crate::types::common::Duration>,
    #[doc = "The number of translog operations to recover."]
    pub translog_ops: Option<String>,
    pub translog_ops_percent: Option<crate::types::common::PercentageString>,
    #[doc = "The translog operations recovered."]
    pub translog_ops_recovered: Option<String>,
    #[doc = "The recovery type."]
    pub r#type: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RecoveryRecord {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RecoveryRecord;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RecoveryRecord object")
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
                let mut f_bytes_percent: Option<crate::types::common::PercentageString> = None;
                let mut seen_bytes_percent = false;
                let mut f_bytes_recovered: Option<String> = None;
                let mut seen_bytes_recovered = false;
                let mut f_bytes_total: Option<String> = None;
                let mut seen_bytes_total = false;
                let mut f_files: Option<String> = None;
                let mut seen_files = false;
                let mut f_files_percent: Option<crate::types::common::PercentageString> = None;
                let mut seen_files_percent = false;
                let mut f_files_recovered: Option<String> = None;
                let mut seen_files_recovered = false;
                let mut f_files_total: Option<String> = None;
                let mut seen_files_total = false;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut seen_index = false;
                let mut f_repository: Option<String> = None;
                let mut seen_repository = false;
                let mut f_shard: Option<String> = None;
                let mut seen_shard = false;
                let mut f_snapshot: Option<String> = None;
                let mut seen_snapshot = false;
                let mut f_source_host: Option<String> = None;
                let mut seen_source_host = false;
                let mut f_source_node: Option<String> = None;
                let mut seen_source_node = false;
                let mut f_stage: Option<String> = None;
                let mut seen_stage = false;
                let mut f_start_time: Option<crate::types::common::DateTime> = None;
                let mut seen_start_time = false;
                let mut f_start_time_millis: Option<
                    crate::types::common::StringifiedEpochTimeUnitMillis,
                > = None;
                let mut seen_start_time_millis = false;
                let mut f_stop_time: Option<crate::types::common::DateTime> = None;
                let mut seen_stop_time = false;
                let mut f_stop_time_millis: Option<
                    crate::types::common::StringifiedEpochTimeUnitMillis,
                > = None;
                let mut seen_stop_time_millis = false;
                let mut f_target_host: Option<String> = None;
                let mut seen_target_host = false;
                let mut f_target_node: Option<String> = None;
                let mut seen_target_node = false;
                let mut f_time: Option<crate::types::common::Duration> = None;
                let mut seen_time = false;
                let mut f_translog_ops: Option<String> = None;
                let mut seen_translog_ops = false;
                let mut f_translog_ops_percent: Option<crate::types::common::PercentageString> =
                    None;
                let mut seen_translog_ops_percent = false;
                let mut f_translog_ops_recovered: Option<String> = None;
                let mut seen_translog_ops_recovered = false;
                let mut f_type: Option<String> = None;
                let mut seen_type = false;
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
                        "bytes_percent" => {
                            if seen_bytes_percent {
                                return Err(serde::de::Error::duplicate_field("bytes_percent"));
                            }
                            seen_bytes_percent = true;
                            f_bytes_percent =
                                map.next_value::<Option<crate::types::common::PercentageString>>()?;
                        }
                        "bytes_recovered" => {
                            if seen_bytes_recovered {
                                return Err(serde::de::Error::duplicate_field("bytes_recovered"));
                            }
                            seen_bytes_recovered = true;
                            f_bytes_recovered = map.next_value::<Option<String>>()?;
                        }
                        "bytes_total" => {
                            if seen_bytes_total {
                                return Err(serde::de::Error::duplicate_field("bytes_total"));
                            }
                            seen_bytes_total = true;
                            f_bytes_total = map.next_value::<Option<String>>()?;
                        }
                        "files" => {
                            if seen_files {
                                return Err(serde::de::Error::duplicate_field("files"));
                            }
                            seen_files = true;
                            f_files = map.next_value::<Option<String>>()?;
                        }
                        "files_percent" => {
                            if seen_files_percent {
                                return Err(serde::de::Error::duplicate_field("files_percent"));
                            }
                            seen_files_percent = true;
                            f_files_percent =
                                map.next_value::<Option<crate::types::common::PercentageString>>()?;
                        }
                        "files_recovered" => {
                            if seen_files_recovered {
                                return Err(serde::de::Error::duplicate_field("files_recovered"));
                            }
                            seen_files_recovered = true;
                            f_files_recovered = map.next_value::<Option<String>>()?;
                        }
                        "files_total" => {
                            if seen_files_total {
                                return Err(serde::de::Error::duplicate_field("files_total"));
                            }
                            seen_files_total = true;
                            f_files_total = map.next_value::<Option<String>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "repository" => {
                            if seen_repository {
                                return Err(serde::de::Error::duplicate_field("repository"));
                            }
                            seen_repository = true;
                            f_repository = map.next_value::<Option<String>>()?;
                        }
                        "shard" => {
                            if seen_shard {
                                return Err(serde::de::Error::duplicate_field("shard"));
                            }
                            seen_shard = true;
                            f_shard = map.next_value::<Option<String>>()?;
                        }
                        "snapshot" => {
                            if seen_snapshot {
                                return Err(serde::de::Error::duplicate_field("snapshot"));
                            }
                            seen_snapshot = true;
                            f_snapshot = map.next_value::<Option<String>>()?;
                        }
                        "source_host" => {
                            if seen_source_host {
                                return Err(serde::de::Error::duplicate_field("source_host"));
                            }
                            seen_source_host = true;
                            f_source_host = map.next_value::<Option<String>>()?;
                        }
                        "source_node" => {
                            if seen_source_node {
                                return Err(serde::de::Error::duplicate_field("source_node"));
                            }
                            seen_source_node = true;
                            f_source_node = map.next_value::<Option<String>>()?;
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
                            f_start_time =
                                map.next_value::<Option<crate::types::common::DateTime>>()?;
                        }
                        "start_time_millis" => {
                            if seen_start_time_millis {
                                return Err(serde::de::Error::duplicate_field("start_time_millis"));
                            }
                            seen_start_time_millis = true;
                            f_start_time_millis = map.next_value::<Option<crate::types::common::StringifiedEpochTimeUnitMillis>>()?;
                        }
                        "stop_time" => {
                            if seen_stop_time {
                                return Err(serde::de::Error::duplicate_field("stop_time"));
                            }
                            seen_stop_time = true;
                            f_stop_time =
                                map.next_value::<Option<crate::types::common::DateTime>>()?;
                        }
                        "stop_time_millis" => {
                            if seen_stop_time_millis {
                                return Err(serde::de::Error::duplicate_field("stop_time_millis"));
                            }
                            seen_stop_time_millis = true;
                            f_stop_time_millis = map.next_value::<Option<crate::types::common::StringifiedEpochTimeUnitMillis>>()?;
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
                            f_time = map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "translog_ops" => {
                            if seen_translog_ops {
                                return Err(serde::de::Error::duplicate_field("translog_ops"));
                            }
                            seen_translog_ops = true;
                            f_translog_ops = map.next_value::<Option<String>>()?;
                        }
                        "translog_ops_percent" => {
                            if seen_translog_ops_percent {
                                return Err(serde::de::Error::duplicate_field(
                                    "translog_ops_percent",
                                ));
                            }
                            seen_translog_ops_percent = true;
                            f_translog_ops_percent =
                                map.next_value::<Option<crate::types::common::PercentageString>>()?;
                        }
                        "translog_ops_recovered" => {
                            if seen_translog_ops_recovered {
                                return Err(serde::de::Error::duplicate_field(
                                    "translog_ops_recovered",
                                ));
                            }
                            seen_translog_ops_recovered = true;
                            f_translog_ops_recovered = map.next_value::<Option<String>>()?;
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
                Ok(RecoveryRecord {
                    bytes: f_bytes,
                    bytes_percent: f_bytes_percent,
                    bytes_recovered: f_bytes_recovered,
                    bytes_total: f_bytes_total,
                    files: f_files,
                    files_percent: f_files_percent,
                    files_recovered: f_files_recovered,
                    files_total: f_files_total,
                    index: f_index,
                    repository: f_repository,
                    shard: f_shard,
                    snapshot: f_snapshot,
                    source_host: f_source_host,
                    source_node: f_source_node,
                    stage: f_stage,
                    start_time: f_start_time,
                    start_time_millis: f_start_time_millis,
                    stop_time: f_stop_time,
                    stop_time_millis: f_stop_time_millis,
                    target_host: f_target_host,
                    target_node: f_target_node,
                    time: f_time,
                    translog_ops: f_translog_ops,
                    translog_ops_percent: f_translog_ops_percent,
                    translog_ops_recovered: f_translog_ops_recovered,
                    r#type: f_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RecoveryRecord {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.bytes {
            map.serialize_entry("bytes", v)?;
        }
        if let Some(v) = &self.bytes_percent {
            map.serialize_entry("bytes_percent", v)?;
        }
        if let Some(v) = &self.bytes_recovered {
            map.serialize_entry("bytes_recovered", v)?;
        }
        if let Some(v) = &self.bytes_total {
            map.serialize_entry("bytes_total", v)?;
        }
        if let Some(v) = &self.files {
            map.serialize_entry("files", v)?;
        }
        if let Some(v) = &self.files_percent {
            map.serialize_entry("files_percent", v)?;
        }
        if let Some(v) = &self.files_recovered {
            map.serialize_entry("files_recovered", v)?;
        }
        if let Some(v) = &self.files_total {
            map.serialize_entry("files_total", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.repository {
            map.serialize_entry("repository", v)?;
        }
        if let Some(v) = &self.shard {
            map.serialize_entry("shard", v)?;
        }
        if let Some(v) = &self.snapshot {
            map.serialize_entry("snapshot", v)?;
        }
        if let Some(v) = &self.source_host {
            map.serialize_entry("source_host", v)?;
        }
        if let Some(v) = &self.source_node {
            map.serialize_entry("source_node", v)?;
        }
        if let Some(v) = &self.stage {
            map.serialize_entry("stage", v)?;
        }
        if let Some(v) = &self.start_time {
            map.serialize_entry("start_time", v)?;
        }
        if let Some(v) = &self.start_time_millis {
            map.serialize_entry("start_time_millis", v)?;
        }
        if let Some(v) = &self.stop_time {
            map.serialize_entry("stop_time", v)?;
        }
        if let Some(v) = &self.stop_time_millis {
            map.serialize_entry("stop_time_millis", v)?;
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
        if let Some(v) = &self.translog_ops {
            map.serialize_entry("translog_ops", v)?;
        }
        if let Some(v) = &self.translog_ops_percent {
            map.serialize_entry("translog_ops_percent", v)?;
        }
        if let Some(v) = &self.translog_ops_recovered {
            map.serialize_entry("translog_ops_recovered", v)?;
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

#[doc = "The response of the `cat.recovery` API"]
pub type RecoveryResponse = crate::types::cat::RecoveryResponse;
