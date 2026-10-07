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

//! Response types of the `cat_snapshots` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct SnapshotsRecord {
    pub duration: Option<crate::types::common::Duration>,
    pub end_epoch: Option<crate::types::common::StringifiedEpochTimeUnitSeconds>,
    pub end_time: Option<crate::types::common::TimeOfDay>,
    #[doc = "The number of failed shards in the snapshot."]
    pub failed_shards: Option<String>,
    #[doc = "The unique identifier for the snapshot."]
    pub id: Option<String>,
    #[doc = "The number of indexes in the snapshot."]
    pub indices: Option<String>,
    #[doc = "The reason for any snapshot failures."]
    pub reason: Option<String>,
    #[doc = "The repository name."]
    pub repository: Option<String>,
    pub start_epoch: Option<crate::types::common::StringifiedEpochTimeUnitSeconds>,
    pub start_time: Option<crate::types::common::TimeOfDay>,
    #[doc = "The state of the snapshot process. Returned values include: `FAILED`: The snapshot process failed. `INCOMPATIBLE`: The snapshot process is incompatible with the current cluster version. `IN_PROGRESS`: The snapshot process started but has not completed. `PARTIAL`: The snapshot process completed with a partial success. `SUCCESS`: The snapshot process completed with a full success."]
    pub status: Option<String>,
    #[doc = "The number of successful shards in the snapshot."]
    pub successful_shards: Option<String>,
    #[doc = "The total number of shards in the snapshot."]
    pub total_shards: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SnapshotsRecord {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SnapshotsRecord;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SnapshotsRecord object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_duration: Option<crate::types::common::Duration> = None;
                let mut seen_duration = false;
                let mut f_end_epoch: Option<crate::types::common::StringifiedEpochTimeUnitSeconds> =
                    None;
                let mut seen_end_epoch = false;
                let mut f_end_time: Option<crate::types::common::TimeOfDay> = None;
                let mut seen_end_time = false;
                let mut f_failed_shards: Option<String> = None;
                let mut seen_failed_shards = false;
                let mut f_id: Option<String> = None;
                let mut seen_id = false;
                let mut f_indices: Option<String> = None;
                let mut seen_indices = false;
                let mut f_reason: Option<String> = None;
                let mut seen_reason = false;
                let mut f_repository: Option<String> = None;
                let mut seen_repository = false;
                let mut f_start_epoch: Option<
                    crate::types::common::StringifiedEpochTimeUnitSeconds,
                > = None;
                let mut seen_start_epoch = false;
                let mut f_start_time: Option<crate::types::common::TimeOfDay> = None;
                let mut seen_start_time = false;
                let mut f_status: Option<String> = None;
                let mut seen_status = false;
                let mut f_successful_shards: Option<String> = None;
                let mut seen_successful_shards = false;
                let mut f_total_shards: Option<String> = None;
                let mut seen_total_shards = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "duration" => {
                            if seen_duration {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            seen_duration = true;
                            f_duration =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "end_epoch" => {
                            if seen_end_epoch {
                                return Err(serde::de::Error::duplicate_field("end_epoch"));
                            }
                            seen_end_epoch = true;
                            f_end_epoch = map.next_value::<Option<crate::types::common::StringifiedEpochTimeUnitSeconds>>()?;
                        }
                        "end_time" => {
                            if seen_end_time {
                                return Err(serde::de::Error::duplicate_field("end_time"));
                            }
                            seen_end_time = true;
                            f_end_time =
                                map.next_value::<Option<crate::types::common::TimeOfDay>>()?;
                        }
                        "failed_shards" => {
                            if seen_failed_shards {
                                return Err(serde::de::Error::duplicate_field("failed_shards"));
                            }
                            seen_failed_shards = true;
                            f_failed_shards = map.next_value::<Option<String>>()?;
                        }
                        "id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<String>>()?;
                        }
                        "indices" => {
                            if seen_indices {
                                return Err(serde::de::Error::duplicate_field("indices"));
                            }
                            seen_indices = true;
                            f_indices = map.next_value::<Option<String>>()?;
                        }
                        "reason" => {
                            if seen_reason {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            seen_reason = true;
                            f_reason = map.next_value::<Option<String>>()?;
                        }
                        "repository" => {
                            if seen_repository {
                                return Err(serde::de::Error::duplicate_field("repository"));
                            }
                            seen_repository = true;
                            f_repository = map.next_value::<Option<String>>()?;
                        }
                        "start_epoch" => {
                            if seen_start_epoch {
                                return Err(serde::de::Error::duplicate_field("start_epoch"));
                            }
                            seen_start_epoch = true;
                            f_start_epoch = map.next_value::<Option<crate::types::common::StringifiedEpochTimeUnitSeconds>>()?;
                        }
                        "start_time" => {
                            if seen_start_time {
                                return Err(serde::de::Error::duplicate_field("start_time"));
                            }
                            seen_start_time = true;
                            f_start_time =
                                map.next_value::<Option<crate::types::common::TimeOfDay>>()?;
                        }
                        "status" => {
                            if seen_status {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            seen_status = true;
                            f_status = map.next_value::<Option<String>>()?;
                        }
                        "successful_shards" => {
                            if seen_successful_shards {
                                return Err(serde::de::Error::duplicate_field("successful_shards"));
                            }
                            seen_successful_shards = true;
                            f_successful_shards = map.next_value::<Option<String>>()?;
                        }
                        "total_shards" => {
                            if seen_total_shards {
                                return Err(serde::de::Error::duplicate_field("total_shards"));
                            }
                            seen_total_shards = true;
                            f_total_shards = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SnapshotsRecord {
                    duration: f_duration,
                    end_epoch: f_end_epoch,
                    end_time: f_end_time,
                    failed_shards: f_failed_shards,
                    id: f_id,
                    indices: f_indices,
                    reason: f_reason,
                    repository: f_repository,
                    start_epoch: f_start_epoch,
                    start_time: f_start_time,
                    status: f_status,
                    successful_shards: f_successful_shards,
                    total_shards: f_total_shards,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SnapshotsRecord {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.duration {
            map.serialize_entry("duration", v)?;
        }
        if let Some(v) = &self.end_epoch {
            map.serialize_entry("end_epoch", v)?;
        }
        if let Some(v) = &self.end_time {
            map.serialize_entry("end_time", v)?;
        }
        if let Some(v) = &self.failed_shards {
            map.serialize_entry("failed_shards", v)?;
        }
        if let Some(v) = &self.id {
            map.serialize_entry("id", v)?;
        }
        if let Some(v) = &self.indices {
            map.serialize_entry("indices", v)?;
        }
        if let Some(v) = &self.reason {
            map.serialize_entry("reason", v)?;
        }
        if let Some(v) = &self.repository {
            map.serialize_entry("repository", v)?;
        }
        if let Some(v) = &self.start_epoch {
            map.serialize_entry("start_epoch", v)?;
        }
        if let Some(v) = &self.start_time {
            map.serialize_entry("start_time", v)?;
        }
        if let Some(v) = &self.status {
            map.serialize_entry("status", v)?;
        }
        if let Some(v) = &self.successful_shards {
            map.serialize_entry("successful_shards", v)?;
        }
        if let Some(v) = &self.total_shards {
            map.serialize_entry("total_shards", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `cat.snapshots` API"]
pub type SnapshotsResponse = crate::types::cat::SnapshotsResponse;
