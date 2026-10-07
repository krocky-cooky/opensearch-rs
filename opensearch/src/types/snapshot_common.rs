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

//! Response types of the `snapshot_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct FileCountSnapshotStats {
    #[doc = "The number of files in the snapshot."]
    pub file_count: Option<i32>,
    #[doc = "The total size of files in the snapshot."]
    pub size_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FileCountSnapshotStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FileCountSnapshotStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FileCountSnapshotStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_file_count: Option<i32> = None;
                let mut seen_file_count = false;
                let mut f_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_size_in_bytes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "file_count" => {
                            if seen_file_count {
                                return Err(serde::de::Error::duplicate_field("file_count"));
                            }
                            seen_file_count = true;
                            f_file_count = map.next_value::<Option<i32>>()?;
                        }
                        "size_in_bytes" => {
                            if seen_size_in_bytes {
                                return Err(serde::de::Error::duplicate_field("size_in_bytes"));
                            }
                            seen_size_in_bytes = true;
                            f_size_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FileCountSnapshotStats {
                    file_count: f_file_count,
                    size_in_bytes: f_size_in_bytes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FileCountSnapshotStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.file_count {
            map.serialize_entry("file_count", v)?;
        }
        if let Some(v) = &self.size_in_bytes {
            map.serialize_entry("size_in_bytes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The name of the repository to store the snapshot."]
#[derive(Debug, Clone)]
pub struct Repository {
    #[doc = "The configuration settings for the repository."]
    pub settings: Option<crate::types::snapshot_common::RepositorySettings>,
    #[doc = "The type of the snapshot repository."]
    pub r#type: Option<String>,
    #[doc = "The unique identifier for the repository."]
    pub uuid: Option<crate::types::common::Uuid>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Repository {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Repository;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Repository object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_settings: Option<crate::types::snapshot_common::RepositorySettings> =
                    None;
                let mut seen_settings = false;
                let mut f_type: Option<String> = None;
                let mut seen_type = false;
                let mut f_uuid: Option<crate::types::common::Uuid> = None;
                let mut seen_uuid = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "settings" => {
                            if seen_settings {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            seen_settings = true;
                            f_settings = map.next_value::<Option<crate::types::snapshot_common::RepositorySettings>>()?;
                        }
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<String>>()?;
                        }
                        "uuid" => {
                            if seen_uuid {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            seen_uuid = true;
                            f_uuid = map.next_value::<Option<crate::types::common::Uuid>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Repository {
                    settings: f_settings,
                    r#type: f_type,
                    uuid: f_uuid,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Repository {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.settings {
            map.serialize_entry("settings", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("type", v)?;
        }
        if let Some(v) = &self.uuid {
            map.serialize_entry("uuid", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The settings for the snapshot repository."]
#[derive(Debug, Clone)]
pub struct RepositorySettings {
    #[doc = "The chunk size for the repository."]
    pub chunk_size: Option<String>,
    #[doc = "Whether compression is enabled for the repository."]
    pub compress: Option<crate::types::common::StringifiedBoolean>,
    #[doc = "The number of concurrent streams for the repository."]
    pub concurrent_streams: Option<crate::types::common::StringifiedInteger>,
    #[doc = "The location where snapshots are stored."]
    pub location: Option<String>,
    #[doc = "Whether the repository is read-only."]
    pub read_only: Option<crate::types::common::StringifiedBoolean>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RepositorySettings {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RepositorySettings;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RepositorySettings object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_chunk_size: Option<String> = None;
                let mut seen_chunk_size = false;
                let mut f_compress: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_compress = false;
                let mut f_concurrent_streams: Option<crate::types::common::StringifiedInteger> =
                    None;
                let mut seen_concurrent_streams = false;
                let mut f_location: Option<String> = None;
                let mut seen_location = false;
                let mut f_read_only: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_read_only = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "chunk_size" => {
                            if seen_chunk_size {
                                return Err(serde::de::Error::duplicate_field("chunk_size"));
                            }
                            seen_chunk_size = true;
                            f_chunk_size = map.next_value::<Option<String>>()?;
                        }
                        "compress" => {
                            if seen_compress {
                                return Err(serde::de::Error::duplicate_field("compress"));
                            }
                            seen_compress = true;
                            f_compress = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "concurrent_streams" => {
                            if seen_concurrent_streams {
                                return Err(serde::de::Error::duplicate_field(
                                    "concurrent_streams",
                                ));
                            }
                            seen_concurrent_streams = true;
                            f_concurrent_streams = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "location" => {
                            if seen_location {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            seen_location = true;
                            f_location = map.next_value::<Option<String>>()?;
                        }
                        "read_only" => {
                            if seen_read_only {
                                return Err(serde::de::Error::duplicate_field("read_only"));
                            }
                            seen_read_only = true;
                            f_read_only = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RepositorySettings {
                    chunk_size: f_chunk_size,
                    compress: f_compress,
                    concurrent_streams: f_concurrent_streams,
                    location: f_location,
                    read_only: f_read_only,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RepositorySettings {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.chunk_size {
            map.serialize_entry("chunk_size", v)?;
        }
        if let Some(v) = &self.compress {
            map.serialize_entry("compress", v)?;
        }
        if let Some(v) = &self.concurrent_streams {
            map.serialize_entry("concurrent_streams", v)?;
        }
        if let Some(v) = &self.location {
            map.serialize_entry("location", v)?;
        }
        if let Some(v) = &self.read_only {
            map.serialize_entry("read_only", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SnapshotIndexStats {
    #[doc = "The status of individual shards in the snapshot."]
    pub shards: Option<
        std::collections::BTreeMap<String, crate::types::snapshot_common::SnapshotShardsStatus>,
    >,
    #[doc = "The statistics about snapshot shards."]
    pub shards_stats: Option<crate::types::snapshot_common::SnapshotShardsStats>,
    #[doc = "The detailed statistics about the snapshot."]
    pub stats: Option<crate::types::snapshot_common::SnapshotStats>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SnapshotIndexStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SnapshotIndexStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SnapshotIndexStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_shards: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::snapshot_common::SnapshotShardsStatus,
                    >,
                > = None;
                let mut seen_shards = false;
                let mut f_shards_stats: Option<crate::types::snapshot_common::SnapshotShardsStats> =
                    None;
                let mut seen_shards_stats = false;
                let mut f_stats: Option<crate::types::snapshot_common::SnapshotStats> = None;
                let mut seen_stats = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "shards" => {
                            if seen_shards {
                                return Err(serde::de::Error::duplicate_field("shards"));
                            }
                            seen_shards = true;
                            f_shards = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::snapshot_common::SnapshotShardsStatus,
                                >,
                            >>()?;
                        }
                        "shards_stats" => {
                            if seen_shards_stats {
                                return Err(serde::de::Error::duplicate_field("shards_stats"));
                            }
                            seen_shards_stats = true;
                            f_shards_stats = map.next_value::<Option<crate::types::snapshot_common::SnapshotShardsStats>>()?;
                        }
                        "stats" => {
                            if seen_stats {
                                return Err(serde::de::Error::duplicate_field("stats"));
                            }
                            seen_stats = true;
                            f_stats = map
                                .next_value::<Option<crate::types::snapshot_common::SnapshotStats>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SnapshotIndexStats {
                    shards: f_shards,
                    shards_stats: f_shards_stats,
                    stats: f_stats,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SnapshotIndexStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.shards {
            map.serialize_entry("shards", v)?;
        }
        if let Some(v) = &self.shards_stats {
            map.serialize_entry("shards_stats", v)?;
        }
        if let Some(v) = &self.stats {
            map.serialize_entry("stats", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SnapshotInfo {
    #[doc = "The list of data streams included in the snapshot."]
    pub data_streams: Option<Vec<String>>,
    #[doc = "The total time taken to complete the snapshot."]
    pub duration: Option<crate::types::common::Duration>,
    #[doc = "The total time taken to complete the snapshot in milliseconds."]
    pub duration_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "The time when the snapshot completed."]
    pub end_time: Option<crate::types::common::DateTime>,
    #[doc = "The time when the snapshot completed in milliseconds."]
    pub end_time_in_millis: Option<crate::types::common::EpochTimeUnitMillis>,
    #[doc = "The list of shard failures that occurred during the snapshot."]
    pub failures: Option<Vec<crate::types::snapshot_common::SnapshotShardFailure>>,
    #[doc = "Whether the snapshot includes the cluster state."]
    pub include_global_state: Option<bool>,
    #[doc = "The list of indexes included in the snapshot."]
    pub indices: Option<Vec<crate::types::common::IndexName>>,
    #[doc = "The custom metadata attached to the snapshot."]
    pub metadata: Option<crate::types::common::Metadata>,
    #[doc = "The timestamp when the snapshot was pinned."]
    pub pinned_timestamp: Option<crate::types::common::EpochTimeUnitMillis>,
    #[doc = "The reason for the snapshot creation."]
    pub reason: Option<String>,
    #[doc = "Whether the snapshot uses remote store index shallow copy."]
    pub remote_store_index_shallow_copy: Option<bool>,
    #[doc = "The statistics about the shards included in the snapshot."]
    pub shards: Option<crate::types::common::ShardStatistics>,
    #[doc = "The name of the snapshot."]
    pub snapshot: Option<crate::types::common::Name>,
    #[doc = "The time when the snapshot started."]
    pub start_time: Option<crate::types::common::DateTime>,
    #[doc = "The time when the snapshot started in milliseconds."]
    pub start_time_in_millis: Option<crate::types::common::EpochTimeUnitMillis>,
    #[doc = "The current state of the snapshot."]
    pub state: Option<String>,
    #[doc = "The unique identifier for the snapshot."]
    pub uuid: Option<crate::types::common::Uuid>,
    #[doc = "The version of OpenSearch when the snapshot was created."]
    pub version: Option<crate::types::common::VersionString>,
    #[doc = "The internal version number of OpenSearch when the snapshot was created."]
    pub version_id: Option<crate::types::common::VersionNumber>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SnapshotInfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SnapshotInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SnapshotInfo object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_data_streams: Option<Vec<String>> = None;
                let mut seen_data_streams = false;
                let mut f_duration: Option<crate::types::common::Duration> = None;
                let mut seen_duration = false;
                let mut f_duration_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut seen_duration_in_millis = false;
                let mut f_end_time: Option<crate::types::common::DateTime> = None;
                let mut seen_end_time = false;
                let mut f_end_time_in_millis: Option<crate::types::common::EpochTimeUnitMillis> =
                    None;
                let mut seen_end_time_in_millis = false;
                let mut f_failures: Option<
                    Vec<crate::types::snapshot_common::SnapshotShardFailure>,
                > = None;
                let mut seen_failures = false;
                let mut f_include_global_state: Option<bool> = None;
                let mut seen_include_global_state = false;
                let mut f_indices: Option<Vec<crate::types::common::IndexName>> = None;
                let mut seen_indices = false;
                let mut f_metadata: Option<crate::types::common::Metadata> = None;
                let mut seen_metadata = false;
                let mut f_pinned_timestamp: Option<crate::types::common::EpochTimeUnitMillis> =
                    None;
                let mut seen_pinned_timestamp = false;
                let mut f_reason: Option<String> = None;
                let mut seen_reason = false;
                let mut f_remote_store_index_shallow_copy: Option<bool> = None;
                let mut seen_remote_store_index_shallow_copy = false;
                let mut f_shards: Option<crate::types::common::ShardStatistics> = None;
                let mut seen_shards = false;
                let mut f_snapshot: Option<crate::types::common::Name> = None;
                let mut seen_snapshot = false;
                let mut f_start_time: Option<crate::types::common::DateTime> = None;
                let mut seen_start_time = false;
                let mut f_start_time_in_millis: Option<crate::types::common::EpochTimeUnitMillis> =
                    None;
                let mut seen_start_time_in_millis = false;
                let mut f_state: Option<String> = None;
                let mut seen_state = false;
                let mut f_uuid: Option<crate::types::common::Uuid> = None;
                let mut seen_uuid = false;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_version_id: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version_id = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "data_streams" => {
                            if seen_data_streams {
                                return Err(serde::de::Error::duplicate_field("data_streams"));
                            }
                            seen_data_streams = true;
                            f_data_streams = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "duration" => {
                            if seen_duration {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            seen_duration = true;
                            f_duration =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "duration_in_millis" => {
                            if seen_duration_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "duration_in_millis",
                                ));
                            }
                            seen_duration_in_millis = true;
                            f_duration_in_millis = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        "end_time" => {
                            if seen_end_time {
                                return Err(serde::de::Error::duplicate_field("end_time"));
                            }
                            seen_end_time = true;
                            f_end_time =
                                map.next_value::<Option<crate::types::common::DateTime>>()?;
                        }
                        "end_time_in_millis" => {
                            if seen_end_time_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "end_time_in_millis",
                                ));
                            }
                            seen_end_time_in_millis = true;
                            f_end_time_in_millis = map
                                .next_value::<Option<crate::types::common::EpochTimeUnitMillis>>(
                                )?;
                        }
                        "failures" => {
                            if seen_failures {
                                return Err(serde::de::Error::duplicate_field("failures"));
                            }
                            seen_failures = true;
                            f_failures =
                                map.next_value::<Option<
                                    Vec<crate::types::snapshot_common::SnapshotShardFailure>,
                                >>()?;
                        }
                        "include_global_state" => {
                            if seen_include_global_state {
                                return Err(serde::de::Error::duplicate_field(
                                    "include_global_state",
                                ));
                            }
                            seen_include_global_state = true;
                            f_include_global_state = map.next_value::<Option<bool>>()?;
                        }
                        "indices" => {
                            if seen_indices {
                                return Err(serde::de::Error::duplicate_field("indices"));
                            }
                            seen_indices = true;
                            f_indices =
                                map.next_value::<Option<Vec<crate::types::common::IndexName>>>()?;
                        }
                        "metadata" => {
                            if seen_metadata {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            seen_metadata = true;
                            f_metadata =
                                map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "pinned_timestamp" => {
                            if seen_pinned_timestamp {
                                return Err(serde::de::Error::duplicate_field("pinned_timestamp"));
                            }
                            seen_pinned_timestamp = true;
                            f_pinned_timestamp = map
                                .next_value::<Option<crate::types::common::EpochTimeUnitMillis>>(
                                )?;
                        }
                        "reason" => {
                            if seen_reason {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            seen_reason = true;
                            f_reason = map.next_value::<Option<String>>()?;
                        }
                        "remote_store_index_shallow_copy" => {
                            if seen_remote_store_index_shallow_copy {
                                return Err(serde::de::Error::duplicate_field(
                                    "remote_store_index_shallow_copy",
                                ));
                            }
                            seen_remote_store_index_shallow_copy = true;
                            f_remote_store_index_shallow_copy = map.next_value::<Option<bool>>()?;
                        }
                        "shards" => {
                            if seen_shards {
                                return Err(serde::de::Error::duplicate_field("shards"));
                            }
                            seen_shards = true;
                            f_shards =
                                map.next_value::<Option<crate::types::common::ShardStatistics>>()?;
                        }
                        "snapshot" => {
                            if seen_snapshot {
                                return Err(serde::de::Error::duplicate_field("snapshot"));
                            }
                            seen_snapshot = true;
                            f_snapshot = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "start_time" => {
                            if seen_start_time {
                                return Err(serde::de::Error::duplicate_field("start_time"));
                            }
                            seen_start_time = true;
                            f_start_time =
                                map.next_value::<Option<crate::types::common::DateTime>>()?;
                        }
                        "start_time_in_millis" => {
                            if seen_start_time_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "start_time_in_millis",
                                ));
                            }
                            seen_start_time_in_millis = true;
                            f_start_time_in_millis = map
                                .next_value::<Option<crate::types::common::EpochTimeUnitMillis>>(
                                )?;
                        }
                        "state" => {
                            if seen_state {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            seen_state = true;
                            f_state = map.next_value::<Option<String>>()?;
                        }
                        "uuid" => {
                            if seen_uuid {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            seen_uuid = true;
                            f_uuid = map.next_value::<Option<crate::types::common::Uuid>>()?;
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "version_id" => {
                            if seen_version_id {
                                return Err(serde::de::Error::duplicate_field("version_id"));
                            }
                            seen_version_id = true;
                            f_version_id =
                                map.next_value::<Option<crate::types::common::VersionNumber>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SnapshotInfo {
                    data_streams: f_data_streams,
                    duration: f_duration,
                    duration_in_millis: f_duration_in_millis,
                    end_time: f_end_time,
                    end_time_in_millis: f_end_time_in_millis,
                    failures: f_failures,
                    include_global_state: f_include_global_state,
                    indices: f_indices,
                    metadata: f_metadata,
                    pinned_timestamp: f_pinned_timestamp,
                    reason: f_reason,
                    remote_store_index_shallow_copy: f_remote_store_index_shallow_copy,
                    shards: f_shards,
                    snapshot: f_snapshot,
                    start_time: f_start_time,
                    start_time_in_millis: f_start_time_in_millis,
                    state: f_state,
                    uuid: f_uuid,
                    version: f_version,
                    version_id: f_version_id,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SnapshotInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.data_streams {
            map.serialize_entry("data_streams", v)?;
        }
        if let Some(v) = &self.duration {
            map.serialize_entry("duration", v)?;
        }
        if let Some(v) = &self.duration_in_millis {
            map.serialize_entry("duration_in_millis", v)?;
        }
        if let Some(v) = &self.end_time {
            map.serialize_entry("end_time", v)?;
        }
        if let Some(v) = &self.end_time_in_millis {
            map.serialize_entry("end_time_in_millis", v)?;
        }
        if let Some(v) = &self.failures {
            map.serialize_entry("failures", v)?;
        }
        if let Some(v) = &self.include_global_state {
            map.serialize_entry("include_global_state", v)?;
        }
        if let Some(v) = &self.indices {
            map.serialize_entry("indices", v)?;
        }
        if let Some(v) = &self.metadata {
            map.serialize_entry("metadata", v)?;
        }
        if let Some(v) = &self.pinned_timestamp {
            map.serialize_entry("pinned_timestamp", v)?;
        }
        if let Some(v) = &self.reason {
            map.serialize_entry("reason", v)?;
        }
        if let Some(v) = &self.remote_store_index_shallow_copy {
            map.serialize_entry("remote_store_index_shallow_copy", v)?;
        }
        if let Some(v) = &self.shards {
            map.serialize_entry("shards", v)?;
        }
        if let Some(v) = &self.snapshot {
            map.serialize_entry("snapshot", v)?;
        }
        if let Some(v) = &self.start_time {
            map.serialize_entry("start_time", v)?;
        }
        if let Some(v) = &self.start_time_in_millis {
            map.serialize_entry("start_time_in_millis", v)?;
        }
        if let Some(v) = &self.state {
            map.serialize_entry("state", v)?;
        }
        if let Some(v) = &self.uuid {
            map.serialize_entry("uuid", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.version_id {
            map.serialize_entry("version_id", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SnapshotShardFailure {
    #[doc = "The name of the index containing the failed shard."]
    pub index: Option<crate::types::common::IndexName>,
    #[doc = "The ID of the node where the shard failure occurred."]
    pub node_id: Option<crate::types::common::Id>,
    #[doc = "The reason for the shard failure."]
    pub reason: Option<String>,
    #[doc = "The ID of the failed shard."]
    pub shard_id: Option<crate::types::common::Id>,
    #[doc = "The status of the failed shard."]
    pub status: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SnapshotShardFailure {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SnapshotShardFailure;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SnapshotShardFailure object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut seen_index = false;
                let mut f_node_id: Option<crate::types::common::Id> = None;
                let mut seen_node_id = false;
                let mut f_reason: Option<String> = None;
                let mut seen_reason = false;
                let mut f_shard_id: Option<crate::types::common::Id> = None;
                let mut seen_shard_id = false;
                let mut f_status: Option<String> = None;
                let mut seen_status = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "node_id" => {
                            if seen_node_id {
                                return Err(serde::de::Error::duplicate_field("node_id"));
                            }
                            seen_node_id = true;
                            f_node_id = map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        "reason" => {
                            if seen_reason {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            seen_reason = true;
                            f_reason = map.next_value::<Option<String>>()?;
                        }
                        "shard_id" => {
                            if seen_shard_id {
                                return Err(serde::de::Error::duplicate_field("shard_id"));
                            }
                            seen_shard_id = true;
                            f_shard_id = map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        "status" => {
                            if seen_status {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            seen_status = true;
                            f_status = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SnapshotShardFailure {
                    index: f_index,
                    node_id: f_node_id,
                    reason: f_reason,
                    shard_id: f_shard_id,
                    status: f_status,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SnapshotShardFailure {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.node_id {
            map.serialize_entry("node_id", v)?;
        }
        if let Some(v) = &self.reason {
            map.serialize_entry("reason", v)?;
        }
        if let Some(v) = &self.shard_id {
            map.serialize_entry("shard_id", v)?;
        }
        if let Some(v) = &self.status {
            map.serialize_entry("status", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SnapshotShardsStats {
    #[doc = "The number of completed shard snapshots."]
    pub done: Option<i64>,
    #[doc = "The number of failed shard snapshots."]
    pub failed: Option<i64>,
    #[doc = "The number of finalizing shard snapshots."]
    pub finalizing: Option<i64>,
    #[doc = "The number of initializing shard snapshots."]
    pub initializing: Option<i64>,
    #[doc = "The number of started shard snapshots."]
    pub started: Option<i64>,
    #[doc = "The total number of shard snapshots."]
    pub total: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SnapshotShardsStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SnapshotShardsStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SnapshotShardsStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_done: Option<i64> = None;
                let mut seen_done = false;
                let mut f_failed: Option<i64> = None;
                let mut seen_failed = false;
                let mut f_finalizing: Option<i64> = None;
                let mut seen_finalizing = false;
                let mut f_initializing: Option<i64> = None;
                let mut seen_initializing = false;
                let mut f_started: Option<i64> = None;
                let mut seen_started = false;
                let mut f_total: Option<i64> = None;
                let mut seen_total = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "done" => {
                            if seen_done {
                                return Err(serde::de::Error::duplicate_field("done"));
                            }
                            seen_done = true;
                            f_done = map.next_value::<Option<i64>>()?;
                        }
                        "failed" => {
                            if seen_failed {
                                return Err(serde::de::Error::duplicate_field("failed"));
                            }
                            seen_failed = true;
                            f_failed = map.next_value::<Option<i64>>()?;
                        }
                        "finalizing" => {
                            if seen_finalizing {
                                return Err(serde::de::Error::duplicate_field("finalizing"));
                            }
                            seen_finalizing = true;
                            f_finalizing = map.next_value::<Option<i64>>()?;
                        }
                        "initializing" => {
                            if seen_initializing {
                                return Err(serde::de::Error::duplicate_field("initializing"));
                            }
                            seen_initializing = true;
                            f_initializing = map.next_value::<Option<i64>>()?;
                        }
                        "started" => {
                            if seen_started {
                                return Err(serde::de::Error::duplicate_field("started"));
                            }
                            seen_started = true;
                            f_started = map.next_value::<Option<i64>>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SnapshotShardsStats {
                    done: f_done,
                    failed: f_failed,
                    finalizing: f_finalizing,
                    initializing: f_initializing,
                    started: f_started,
                    total: f_total,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SnapshotShardsStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.done {
            map.serialize_entry("done", v)?;
        }
        if let Some(v) = &self.failed {
            map.serialize_entry("failed", v)?;
        }
        if let Some(v) = &self.finalizing {
            map.serialize_entry("finalizing", v)?;
        }
        if let Some(v) = &self.initializing {
            map.serialize_entry("initializing", v)?;
        }
        if let Some(v) = &self.started {
            map.serialize_entry("started", v)?;
        }
        if let Some(v) = &self.total {
            map.serialize_entry("total", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SnapshotShardsStatsStage {
    #[doc = "`DONE`"]
    Done,
    #[doc = "`FAILURE`"]
    Failure,
    #[doc = "`FINALIZE`"]
    Finalize,
    #[doc = "`INIT`"]
    Init,
    #[doc = "`STARTED`"]
    Started,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SnapshotShardsStatsStage {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SnapshotShardsStatsStage::Done => "DONE",
            SnapshotShardsStatsStage::Failure => "FAILURE",
            SnapshotShardsStatsStage::Finalize => "FINALIZE",
            SnapshotShardsStatsStage::Init => "INIT",
            SnapshotShardsStatsStage::Started => "STARTED",
            SnapshotShardsStatsStage::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SnapshotShardsStatsStage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SnapshotShardsStatsStage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "DONE" => SnapshotShardsStatsStage::Done,
            "FAILURE" => SnapshotShardsStatsStage::Failure,
            "FINALIZE" => SnapshotShardsStatsStage::Finalize,
            "INIT" => SnapshotShardsStatsStage::Init,
            "STARTED" => SnapshotShardsStatsStage::Started,
            _ => SnapshotShardsStatsStage::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SnapshotShardsStatsSummary {
    #[doc = "The incremental statistics for the shard snapshot."]
    pub incremental: Option<crate::types::snapshot_common::SnapshotShardsStatsSummaryItem>,
    #[doc = "The processed statistics for the shard snapshot."]
    pub processed: Option<crate::types::snapshot_common::SnapshotShardsStatsSummaryItem>,
    #[doc = "When the shard snapshot started in milliseconds."]
    pub start_time_in_millis: Option<crate::types::common::EpochTimeUnitMillis>,
    #[doc = "The total time taken for the shard snapshot."]
    pub time: Option<crate::types::common::Duration>,
    #[doc = "The total time taken for the shard snapshot in milliseconds."]
    pub time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "The total statistics for the shard snapshot."]
    pub total: Option<crate::types::snapshot_common::SnapshotShardsStatsSummaryItem>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SnapshotShardsStatsSummary {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SnapshotShardsStatsSummary;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SnapshotShardsStatsSummary object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_incremental: Option<
                    crate::types::snapshot_common::SnapshotShardsStatsSummaryItem,
                > = None;
                let mut seen_incremental = false;
                let mut f_processed: Option<
                    crate::types::snapshot_common::SnapshotShardsStatsSummaryItem,
                > = None;
                let mut seen_processed = false;
                let mut f_start_time_in_millis: Option<crate::types::common::EpochTimeUnitMillis> =
                    None;
                let mut seen_start_time_in_millis = false;
                let mut f_time: Option<crate::types::common::Duration> = None;
                let mut seen_time = false;
                let mut f_time_in_millis: Option<crate::types::common::DurationValueUnitMillis> =
                    None;
                let mut seen_time_in_millis = false;
                let mut f_total: Option<
                    crate::types::snapshot_common::SnapshotShardsStatsSummaryItem,
                > = None;
                let mut seen_total = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "incremental" => {
                            if seen_incremental {
                                return Err(serde::de::Error::duplicate_field("incremental"));
                            }
                            seen_incremental = true;
                            f_incremental = map.next_value::<Option<
                                crate::types::snapshot_common::SnapshotShardsStatsSummaryItem,
                            >>()?;
                        }
                        "processed" => {
                            if seen_processed {
                                return Err(serde::de::Error::duplicate_field("processed"));
                            }
                            seen_processed = true;
                            f_processed = map.next_value::<Option<
                                crate::types::snapshot_common::SnapshotShardsStatsSummaryItem,
                            >>()?;
                        }
                        "start_time_in_millis" => {
                            if seen_start_time_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "start_time_in_millis",
                                ));
                            }
                            seen_start_time_in_millis = true;
                            f_start_time_in_millis = map
                                .next_value::<Option<crate::types::common::EpochTimeUnitMillis>>(
                                )?;
                        }
                        "time" => {
                            if seen_time {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            seen_time = true;
                            f_time = map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "time_in_millis" => {
                            if seen_time_in_millis {
                                return Err(serde::de::Error::duplicate_field("time_in_millis"));
                            }
                            seen_time_in_millis = true;
                            f_time_in_millis = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map.next_value::<Option<
                                crate::types::snapshot_common::SnapshotShardsStatsSummaryItem,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SnapshotShardsStatsSummary {
                    incremental: f_incremental,
                    processed: f_processed,
                    start_time_in_millis: f_start_time_in_millis,
                    time: f_time,
                    time_in_millis: f_time_in_millis,
                    total: f_total,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SnapshotShardsStatsSummary {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.incremental {
            map.serialize_entry("incremental", v)?;
        }
        if let Some(v) = &self.processed {
            map.serialize_entry("processed", v)?;
        }
        if let Some(v) = &self.start_time_in_millis {
            map.serialize_entry("start_time_in_millis", v)?;
        }
        if let Some(v) = &self.time {
            map.serialize_entry("time", v)?;
        }
        if let Some(v) = &self.time_in_millis {
            map.serialize_entry("time_in_millis", v)?;
        }
        if let Some(v) = &self.total {
            map.serialize_entry("total", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SnapshotShardsStatsSummaryItem {
    #[doc = "The number of files in the shard snapshot."]
    pub file_count: Option<i64>,
    #[doc = "The total size of files in the shard snapshot."]
    pub size_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SnapshotShardsStatsSummaryItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SnapshotShardsStatsSummaryItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SnapshotShardsStatsSummaryItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_file_count: Option<i64> = None;
                let mut seen_file_count = false;
                let mut f_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_size_in_bytes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "file_count" => {
                            if seen_file_count {
                                return Err(serde::de::Error::duplicate_field("file_count"));
                            }
                            seen_file_count = true;
                            f_file_count = map.next_value::<Option<i64>>()?;
                        }
                        "size_in_bytes" => {
                            if seen_size_in_bytes {
                                return Err(serde::de::Error::duplicate_field("size_in_bytes"));
                            }
                            seen_size_in_bytes = true;
                            f_size_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SnapshotShardsStatsSummaryItem {
                    file_count: f_file_count,
                    size_in_bytes: f_size_in_bytes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SnapshotShardsStatsSummaryItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.file_count {
            map.serialize_entry("file_count", v)?;
        }
        if let Some(v) = &self.size_in_bytes {
            map.serialize_entry("size_in_bytes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SnapshotShardsStatus {
    #[doc = "The ID of the node containing the shard."]
    pub node: Option<crate::types::common::NodeId>,
    #[doc = "The reason for the current shard status."]
    pub reason: Option<String>,
    #[doc = "The current stage of the shard snapshot."]
    pub stage: crate::types::snapshot_common::SnapshotShardsStatsStage,
    #[doc = "The statistical summary of the shard snapshot."]
    pub stats: crate::types::snapshot_common::SnapshotShardsStatsSummary,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SnapshotShardsStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SnapshotShardsStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SnapshotShardsStatus object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_node: Option<crate::types::common::NodeId> = None;
                let mut seen_node = false;
                let mut f_reason: Option<String> = None;
                let mut seen_reason = false;
                let mut f_stage: Option<crate::types::snapshot_common::SnapshotShardsStatsStage> =
                    None;
                let mut f_stats: Option<crate::types::snapshot_common::SnapshotShardsStatsSummary> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "node" => {
                            if seen_node {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            seen_node = true;
                            f_node = map.next_value::<Option<crate::types::common::NodeId>>()?;
                        }
                        "reason" => {
                            if seen_reason {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            seen_reason = true;
                            f_reason = map.next_value::<Option<String>>()?;
                        }
                        "stage" => {
                            if f_stage.is_some() {
                                return Err(serde::de::Error::duplicate_field("stage"));
                            }
                            f_stage = Some(map.next_value::<crate::types::snapshot_common::SnapshotShardsStatsStage>()?);
                        }
                        "stats" => {
                            if f_stats.is_some() {
                                return Err(serde::de::Error::duplicate_field("stats"));
                            }
                            f_stats = Some(map.next_value::<crate::types::snapshot_common::SnapshotShardsStatsSummary>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SnapshotShardsStatus {
                    node: f_node,
                    reason: f_reason,
                    stage: f_stage.ok_or_else(|| serde::de::Error::missing_field("stage"))?,
                    stats: f_stats.ok_or_else(|| serde::de::Error::missing_field("stats"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SnapshotShardsStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.node {
            map.serialize_entry("node", v)?;
        }
        if let Some(v) = &self.reason {
            map.serialize_entry("reason", v)?;
        }
        map.serialize_entry("stage", &self.stage)?;
        map.serialize_entry("stats", &self.stats)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SnapshotStats {
    #[doc = "The incremental statistics for the snapshot."]
    pub incremental: Option<crate::types::snapshot_common::FileCountSnapshotStats>,
    #[doc = "The processed statistics for the snapshot."]
    pub processed: Option<crate::types::snapshot_common::FileCountSnapshotStats>,
    #[doc = "When the snapshot started in milliseconds."]
    pub start_time_in_millis: Option<crate::types::common::EpochTimeUnitMillis>,
    #[doc = "The total time taken for the snapshot."]
    pub time: Option<crate::types::common::Duration>,
    #[doc = "The total time taken for the snapshot in milliseconds."]
    pub time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "The total statistics for the snapshot."]
    pub total: Option<crate::types::snapshot_common::FileCountSnapshotStats>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SnapshotStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SnapshotStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SnapshotStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_incremental: Option<
                    crate::types::snapshot_common::FileCountSnapshotStats,
                > = None;
                let mut seen_incremental = false;
                let mut f_processed: Option<crate::types::snapshot_common::FileCountSnapshotStats> =
                    None;
                let mut seen_processed = false;
                let mut f_start_time_in_millis: Option<crate::types::common::EpochTimeUnitMillis> =
                    None;
                let mut seen_start_time_in_millis = false;
                let mut f_time: Option<crate::types::common::Duration> = None;
                let mut seen_time = false;
                let mut f_time_in_millis: Option<crate::types::common::DurationValueUnitMillis> =
                    None;
                let mut seen_time_in_millis = false;
                let mut f_total: Option<crate::types::snapshot_common::FileCountSnapshotStats> =
                    None;
                let mut seen_total = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "incremental" => {
                            if seen_incremental {
                                return Err(serde::de::Error::duplicate_field("incremental"));
                            }
                            seen_incremental = true;
                            f_incremental = map.next_value::<Option<crate::types::snapshot_common::FileCountSnapshotStats>>()?;
                        }
                        "processed" => {
                            if seen_processed {
                                return Err(serde::de::Error::duplicate_field("processed"));
                            }
                            seen_processed = true;
                            f_processed = map.next_value::<Option<crate::types::snapshot_common::FileCountSnapshotStats>>()?;
                        }
                        "start_time_in_millis" => {
                            if seen_start_time_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "start_time_in_millis",
                                ));
                            }
                            seen_start_time_in_millis = true;
                            f_start_time_in_millis = map
                                .next_value::<Option<crate::types::common::EpochTimeUnitMillis>>(
                                )?;
                        }
                        "time" => {
                            if seen_time {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            seen_time = true;
                            f_time = map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "time_in_millis" => {
                            if seen_time_in_millis {
                                return Err(serde::de::Error::duplicate_field("time_in_millis"));
                            }
                            seen_time_in_millis = true;
                            f_time_in_millis = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map.next_value::<Option<crate::types::snapshot_common::FileCountSnapshotStats>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SnapshotStats {
                    incremental: f_incremental,
                    processed: f_processed,
                    start_time_in_millis: f_start_time_in_millis,
                    time: f_time,
                    time_in_millis: f_time_in_millis,
                    total: f_total,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SnapshotStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.incremental {
            map.serialize_entry("incremental", v)?;
        }
        if let Some(v) = &self.processed {
            map.serialize_entry("processed", v)?;
        }
        if let Some(v) = &self.start_time_in_millis {
            map.serialize_entry("start_time_in_millis", v)?;
        }
        if let Some(v) = &self.time {
            map.serialize_entry("time", v)?;
        }
        if let Some(v) = &self.time_in_millis {
            map.serialize_entry("time_in_millis", v)?;
        }
        if let Some(v) = &self.total {
            map.serialize_entry("total", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SnapshotStatus {
    #[doc = "Whether the snapshot includes the cluster state."]
    pub include_global_state: Option<bool>,
    #[doc = "The status of indexes in the snapshot."]
    pub indices: Option<
        std::collections::BTreeMap<String, crate::types::snapshot_common::SnapshotIndexStats>,
    >,
    #[doc = "The name of the repository containing the snapshot."]
    pub repository: Option<String>,
    #[doc = "The statistics about snapshot shards."]
    pub shards_stats: Option<crate::types::snapshot_common::SnapshotShardsStats>,
    #[doc = "The name of the snapshot."]
    pub snapshot: Option<String>,
    #[doc = "The current state of the snapshot."]
    pub state: Option<String>,
    #[doc = "The detailed statistics about the snapshot."]
    pub stats: Option<crate::types::snapshot_common::SnapshotStats>,
    #[doc = "The unique identifier for the snapshot."]
    pub uuid: Option<crate::types::common::Uuid>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SnapshotStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SnapshotStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SnapshotStatus object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_include_global_state: Option<bool> = None;
                let mut seen_include_global_state = false;
                let mut f_indices: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::snapshot_common::SnapshotIndexStats,
                    >,
                > = None;
                let mut seen_indices = false;
                let mut f_repository: Option<String> = None;
                let mut seen_repository = false;
                let mut f_shards_stats: Option<crate::types::snapshot_common::SnapshotShardsStats> =
                    None;
                let mut seen_shards_stats = false;
                let mut f_snapshot: Option<String> = None;
                let mut seen_snapshot = false;
                let mut f_state: Option<String> = None;
                let mut seen_state = false;
                let mut f_stats: Option<crate::types::snapshot_common::SnapshotStats> = None;
                let mut seen_stats = false;
                let mut f_uuid: Option<crate::types::common::Uuid> = None;
                let mut seen_uuid = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "include_global_state" => {
                            if seen_include_global_state {
                                return Err(serde::de::Error::duplicate_field(
                                    "include_global_state",
                                ));
                            }
                            seen_include_global_state = true;
                            f_include_global_state = map.next_value::<Option<bool>>()?;
                        }
                        "indices" => {
                            if seen_indices {
                                return Err(serde::de::Error::duplicate_field("indices"));
                            }
                            seen_indices = true;
                            f_indices = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::snapshot_common::SnapshotIndexStats,
                                >,
                            >>()?;
                        }
                        "repository" => {
                            if seen_repository {
                                return Err(serde::de::Error::duplicate_field("repository"));
                            }
                            seen_repository = true;
                            f_repository = map.next_value::<Option<String>>()?;
                        }
                        "shards_stats" => {
                            if seen_shards_stats {
                                return Err(serde::de::Error::duplicate_field("shards_stats"));
                            }
                            seen_shards_stats = true;
                            f_shards_stats = map.next_value::<Option<crate::types::snapshot_common::SnapshotShardsStats>>()?;
                        }
                        "snapshot" => {
                            if seen_snapshot {
                                return Err(serde::de::Error::duplicate_field("snapshot"));
                            }
                            seen_snapshot = true;
                            f_snapshot = map.next_value::<Option<String>>()?;
                        }
                        "state" => {
                            if seen_state {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            seen_state = true;
                            f_state = map.next_value::<Option<String>>()?;
                        }
                        "stats" => {
                            if seen_stats {
                                return Err(serde::de::Error::duplicate_field("stats"));
                            }
                            seen_stats = true;
                            f_stats = map
                                .next_value::<Option<crate::types::snapshot_common::SnapshotStats>>(
                                )?;
                        }
                        "uuid" => {
                            if seen_uuid {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            seen_uuid = true;
                            f_uuid = map.next_value::<Option<crate::types::common::Uuid>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SnapshotStatus {
                    include_global_state: f_include_global_state,
                    indices: f_indices,
                    repository: f_repository,
                    shards_stats: f_shards_stats,
                    snapshot: f_snapshot,
                    state: f_state,
                    stats: f_stats,
                    uuid: f_uuid,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SnapshotStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.include_global_state {
            map.serialize_entry("include_global_state", v)?;
        }
        if let Some(v) = &self.indices {
            map.serialize_entry("indices", v)?;
        }
        if let Some(v) = &self.repository {
            map.serialize_entry("repository", v)?;
        }
        if let Some(v) = &self.shards_stats {
            map.serialize_entry("shards_stats", v)?;
        }
        if let Some(v) = &self.snapshot {
            map.serialize_entry("snapshot", v)?;
        }
        if let Some(v) = &self.state {
            map.serialize_entry("state", v)?;
        }
        if let Some(v) = &self.stats {
            map.serialize_entry("stats", v)?;
        }
        if let Some(v) = &self.uuid {
            map.serialize_entry("uuid", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
