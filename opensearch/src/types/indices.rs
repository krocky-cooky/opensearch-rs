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

//! Response types of the `indices` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct AddBlockResponse {
    #[doc = "For a successful response, this value is always true. On failure, an exception is returned instead."]
    pub acknowledged: bool,
    pub indices: Vec<crate::types::indices_add_block::IndicesBlockStatus>,
    pub shards_acknowledged: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AddBlockResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AddBlockResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AddBlockResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_acknowledged: Option<bool> = None;
                let mut f_indices: Option<
                    Vec<crate::types::indices_add_block::IndicesBlockStatus>,
                > = None;
                let mut f_shards_acknowledged: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "acknowledged" => {
                            if f_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field("acknowledged"));
                            }
                            f_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        "indices" => {
                            if f_indices.is_some() {
                                return Err(serde::de::Error::duplicate_field("indices"));
                            }
                            f_indices = Some(map.next_value::<Vec<crate::types::indices_add_block::IndicesBlockStatus>>()?);
                        }
                        "shards_acknowledged" => {
                            if f_shards_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "shards_acknowledged",
                                ));
                            }
                            f_shards_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AddBlockResponse {
                    acknowledged: f_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("acknowledged"))?,
                    indices: f_indices.ok_or_else(|| serde::de::Error::missing_field("indices"))?,
                    shards_acknowledged: f_shards_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("shards_acknowledged"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AddBlockResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("acknowledged", &self.acknowledged)?;
        map.serialize_entry("indices", &self.indices)?;
        map.serialize_entry("shards_acknowledged", &self.shards_acknowledged)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct AnalyzeResponse {
    pub detail: Option<crate::types::indices_analyze::AnalyzeDetail>,
    pub tokens: Option<Vec<crate::types::indices_analyze::AnalyzeToken>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AnalyzeResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AnalyzeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AnalyzeResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_detail: Option<crate::types::indices_analyze::AnalyzeDetail> = None;
                let mut seen_detail = false;
                let mut f_tokens: Option<Vec<crate::types::indices_analyze::AnalyzeToken>> = None;
                let mut seen_tokens = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "detail" => {
                            if seen_detail {
                                return Err(serde::de::Error::duplicate_field("detail"));
                            }
                            seen_detail = true;
                            f_detail = map
                                .next_value::<Option<crate::types::indices_analyze::AnalyzeDetail>>(
                                )?;
                        }
                        "tokens" => {
                            if seen_tokens {
                                return Err(serde::de::Error::duplicate_field("tokens"));
                            }
                            seen_tokens = true;
                            f_tokens = map.next_value::<Option<Vec<crate::types::indices_analyze::AnalyzeToken>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AnalyzeResponse {
                    detail: f_detail,
                    tokens: f_tokens,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AnalyzeResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.detail {
            map.serialize_entry("detail", v)?;
        }
        if let Some(v) = &self.tokens {
            map.serialize_entry("tokens", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct CloneResponse {
    #[doc = "For a successful response, this value is always true. On failure, an exception is returned instead."]
    pub acknowledged: bool,
    pub index: crate::types::common::IndexName,
    pub shards_acknowledged: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CloneResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CloneResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CloneResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_acknowledged: Option<bool> = None;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut f_shards_acknowledged: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "acknowledged" => {
                            if f_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field("acknowledged"));
                            }
                            f_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        "index" => {
                            if f_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            f_index = Some(map.next_value::<crate::types::common::IndexName>()?);
                        }
                        "shards_acknowledged" => {
                            if f_shards_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "shards_acknowledged",
                                ));
                            }
                            f_shards_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CloneResponse {
                    acknowledged: f_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("acknowledged"))?,
                    index: f_index.ok_or_else(|| serde::de::Error::missing_field("index"))?,
                    shards_acknowledged: f_shards_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("shards_acknowledged"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CloneResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("acknowledged", &self.acknowledged)?;
        map.serialize_entry("index", &self.index)?;
        map.serialize_entry("shards_acknowledged", &self.shards_acknowledged)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct CloseResponse {
    #[doc = "For a successful response, this value is always true. On failure, an exception is returned instead."]
    pub acknowledged: bool,
    pub indices: std::collections::BTreeMap<String, crate::types::indices_close::CloseIndexResult>,
    pub shards_acknowledged: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CloseResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CloseResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CloseResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_acknowledged: Option<bool> = None;
                let mut f_indices: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::indices_close::CloseIndexResult,
                    >,
                > = None;
                let mut f_shards_acknowledged: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "acknowledged" => {
                            if f_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field("acknowledged"));
                            }
                            f_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        "indices" => {
                            if f_indices.is_some() {
                                return Err(serde::de::Error::duplicate_field("indices"));
                            }
                            f_indices = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                crate::types::indices_close::CloseIndexResult,
                            >>()?);
                        }
                        "shards_acknowledged" => {
                            if f_shards_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "shards_acknowledged",
                                ));
                            }
                            f_shards_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CloseResponse {
                    acknowledged: f_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("acknowledged"))?,
                    indices: f_indices.ok_or_else(|| serde::de::Error::missing_field("indices"))?,
                    shards_acknowledged: f_shards_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("shards_acknowledged"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CloseResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("acknowledged", &self.acknowledged)?;
        map.serialize_entry("indices", &self.indices)?;
        map.serialize_entry("shards_acknowledged", &self.shards_acknowledged)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct CreateResponse {
    pub acknowledged: bool,
    pub index: crate::types::common::IndexName,
    pub shards_acknowledged: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CreateResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CreateResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CreateResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_acknowledged: Option<bool> = None;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut f_shards_acknowledged: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "acknowledged" => {
                            if f_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field("acknowledged"));
                            }
                            f_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        "index" => {
                            if f_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            f_index = Some(map.next_value::<crate::types::common::IndexName>()?);
                        }
                        "shards_acknowledged" => {
                            if f_shards_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "shards_acknowledged",
                                ));
                            }
                            f_shards_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CreateResponse {
                    acknowledged: f_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("acknowledged"))?,
                    index: f_index.ok_or_else(|| serde::de::Error::missing_field("index"))?,
                    shards_acknowledged: f_shards_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("shards_acknowledged"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CreateResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("acknowledged", &self.acknowledged)?;
        map.serialize_entry("index", &self.index)?;
        map.serialize_entry("shards_acknowledged", &self.shards_acknowledged)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct DataStreamsStatsResponse {
    pub shards: crate::types::common::ShardStatistics,
    #[doc = "Total number of backing indexes for the selected data streams."]
    pub backing_indices: i32,
    #[doc = "Total number of selected data streams."]
    pub data_stream_count: i32,
    #[doc = "Contains statistics for the selected data streams."]
    pub data_streams: Vec<crate::types::indices_common::DataStreamStats>,
    pub total_store_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total size, in bytes, of all shards for the selected data streams."]
    pub total_store_size_bytes: crate::types::common::ByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DataStreamsStatsResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DataStreamsStatsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DataStreamsStatsResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_shards: Option<crate::types::common::ShardStatistics> = None;
                let mut f_backing_indices: Option<i32> = None;
                let mut f_data_stream_count: Option<i32> = None;
                let mut f_data_streams: Option<Vec<crate::types::indices_common::DataStreamStats>> =
                    None;
                let mut f_total_store_size: Option<crate::types::common::HumanReadableByteCount> =
                    None;
                let mut seen_total_store_size = false;
                let mut f_total_store_size_bytes: Option<crate::types::common::ByteCount> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_shards" => {
                            if f_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("_shards"));
                            }
                            f_shards =
                                Some(map.next_value::<crate::types::common::ShardStatistics>()?);
                        }
                        "backing_indices" => {
                            if f_backing_indices.is_some() {
                                return Err(serde::de::Error::duplicate_field("backing_indices"));
                            }
                            f_backing_indices = Some(map.next_value::<i32>()?);
                        }
                        "data_stream_count" => {
                            if f_data_stream_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("data_stream_count"));
                            }
                            f_data_stream_count = Some(map.next_value::<i32>()?);
                        }
                        "data_streams" => {
                            if f_data_streams.is_some() {
                                return Err(serde::de::Error::duplicate_field("data_streams"));
                            }
                            f_data_streams = Some(map.next_value::<Vec<crate::types::indices_common::DataStreamStats>>()?);
                        }
                        "total_store_size" => {
                            if seen_total_store_size {
                                return Err(serde::de::Error::duplicate_field("total_store_size"));
                            }
                            seen_total_store_size = true;
                            f_total_store_size = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "total_store_size_bytes" => {
                            if f_total_store_size_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_store_size_bytes",
                                ));
                            }
                            f_total_store_size_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DataStreamsStatsResponse {
                    shards: f_shards.ok_or_else(|| serde::de::Error::missing_field("_shards"))?,
                    backing_indices: f_backing_indices
                        .ok_or_else(|| serde::de::Error::missing_field("backing_indices"))?,
                    data_stream_count: f_data_stream_count
                        .ok_or_else(|| serde::de::Error::missing_field("data_stream_count"))?,
                    data_streams: f_data_streams
                        .ok_or_else(|| serde::de::Error::missing_field("data_streams"))?,
                    total_store_size: f_total_store_size,
                    total_store_size_bytes: f_total_store_size_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("total_store_size_bytes"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DataStreamsStatsResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("_shards", &self.shards)?;
        map.serialize_entry("backing_indices", &self.backing_indices)?;
        map.serialize_entry("data_stream_count", &self.data_stream_count)?;
        map.serialize_entry("data_streams", &self.data_streams)?;
        if let Some(v) = &self.total_store_size {
            map.serialize_entry("total_store_size", v)?;
        }
        map.serialize_entry("total_store_size_bytes", &self.total_store_size_bytes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ForcemergeResponse {
    pub shards: crate::types::common::ShardStatistics,
    #[doc = "Task contains a task id returned when `wait_for_completion=false`, you can use the `task_id` to get the status of the task at _tasks/<task_id>."]
    pub task: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ForcemergeResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ForcemergeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ForcemergeResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_shards: Option<crate::types::common::ShardStatistics> = None;
                let mut f_task: Option<String> = None;
                let mut seen_task = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_shards" => {
                            if f_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("_shards"));
                            }
                            f_shards =
                                Some(map.next_value::<crate::types::common::ShardStatistics>()?);
                        }
                        "task" => {
                            if seen_task {
                                return Err(serde::de::Error::duplicate_field("task"));
                            }
                            seen_task = true;
                            f_task = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ForcemergeResponse {
                    shards: f_shards.ok_or_else(|| serde::de::Error::missing_field("_shards"))?,
                    task: f_task,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ForcemergeResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("_shards", &self.shards)?;
        if let Some(v) = &self.task {
            map.serialize_entry("task", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type GetAliasResponse =
    std::collections::BTreeMap<String, crate::types::indices_get_alias::IndexAliases>;

#[derive(Debug, Clone)]
pub struct GetDataStreamResponse {
    pub data_streams: Vec<crate::types::indices_common::DataStream>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GetDataStreamResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GetDataStreamResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GetDataStreamResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_data_streams: Option<Vec<crate::types::indices_common::DataStream>> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "data_streams" => {
                            if f_data_streams.is_some() {
                                return Err(serde::de::Error::duplicate_field("data_streams"));
                            }
                            f_data_streams = Some(
                                map.next_value::<Vec<crate::types::indices_common::DataStream>>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GetDataStreamResponse {
                    data_streams: f_data_streams
                        .ok_or_else(|| serde::de::Error::missing_field("data_streams"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GetDataStreamResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("data_streams", &self.data_streams)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type GetFieldMappingResponse =
    std::collections::BTreeMap<String, crate::types::indices_get_field_mapping::TypeFieldMappings>;

#[derive(Debug, Clone)]
pub struct GetIndexTemplateResponse {
    pub index_templates: Vec<crate::types::indices_get_index_template::IndexTemplateItem>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GetIndexTemplateResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GetIndexTemplateResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GetIndexTemplateResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_index_templates: Option<
                    Vec<crate::types::indices_get_index_template::IndexTemplateItem>,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "index_templates" => {
                            if f_index_templates.is_some() {
                                return Err(serde::de::Error::duplicate_field("index_templates"));
                            }
                            f_index_templates =
                                Some(map.next_value::<Vec<
                                    crate::types::indices_get_index_template::IndexTemplateItem,
                                >>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GetIndexTemplateResponse {
                    index_templates: f_index_templates
                        .ok_or_else(|| serde::de::Error::missing_field("index_templates"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GetIndexTemplateResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("index_templates", &self.index_templates)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type GetMappingResponse =
    std::collections::BTreeMap<String, crate::types::indices_get_mapping::IndexMappingRecord>;

pub type GetResponse = std::collections::BTreeMap<String, crate::types::indices_common::IndexState>;

pub type GetSettingsResponse =
    std::collections::BTreeMap<String, crate::types::indices_common::IndexState>;

pub type GetTemplateResponse =
    std::collections::BTreeMap<String, crate::types::indices_common::TemplateMapping>;

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct OpenResponse {
    raw: Box<serde_json::value::RawValue>,
}
impl OpenResponse {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `object` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_object(
        &self,
    ) -> std::result::Result<
        crate::types::indices::OpenResponseObject,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "object")
    }
    #[doc = "Parses the value as the `object_object` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_object_object(
        &self,
    ) -> std::result::Result<
        crate::types::indices::OpenResponseObjectObject,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "object_object")
    }
}

#[derive(Debug, Clone)]
pub struct OpenResponseObject {
    pub task: Option<crate::types::common::TaskId>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for OpenResponseObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = OpenResponseObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a OpenResponseObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_task: Option<crate::types::common::TaskId> = None;
                let mut seen_task = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "task" => {
                            if seen_task {
                                return Err(serde::de::Error::duplicate_field("task"));
                            }
                            seen_task = true;
                            f_task = map.next_value::<Option<crate::types::common::TaskId>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(OpenResponseObject {
                    task: f_task,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for OpenResponseObject {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.task {
            map.serialize_entry("task", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct OpenResponseObjectObject {
    pub acknowledged: bool,
    pub shards_acknowledged: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for OpenResponseObjectObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = OpenResponseObjectObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a OpenResponseObjectObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_acknowledged: Option<bool> = None;
                let mut f_shards_acknowledged: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "acknowledged" => {
                            if f_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field("acknowledged"));
                            }
                            f_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        "shards_acknowledged" => {
                            if f_shards_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "shards_acknowledged",
                                ));
                            }
                            f_shards_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(OpenResponseObjectObject {
                    acknowledged: f_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("acknowledged"))?,
                    shards_acknowledged: f_shards_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("shards_acknowledged"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for OpenResponseObjectObject {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("acknowledged", &self.acknowledged)?;
        map.serialize_entry("shards_acknowledged", &self.shards_acknowledged)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type RecoveryResponse =
    std::collections::BTreeMap<String, crate::types::indices_recovery::RecoveryStatus>;

#[derive(Debug, Clone)]
pub struct ResolveIndexResponse {
    pub aliases: Vec<crate::types::indices_resolve_index::ResolveIndexAliasItem>,
    pub data_streams: Vec<crate::types::indices_resolve_index::ResolveIndexDataStreamsItem>,
    pub indices: Vec<crate::types::indices_resolve_index::ResolveIndexItem>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ResolveIndexResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ResolveIndexResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ResolveIndexResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_aliases: Option<
                    Vec<crate::types::indices_resolve_index::ResolveIndexAliasItem>,
                > = None;
                let mut f_data_streams: Option<
                    Vec<crate::types::indices_resolve_index::ResolveIndexDataStreamsItem>,
                > = None;
                let mut f_indices: Option<
                    Vec<crate::types::indices_resolve_index::ResolveIndexItem>,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "aliases" => {
                            if f_aliases.is_some() {
                                return Err(serde::de::Error::duplicate_field("aliases"));
                            }
                            f_aliases =
                                Some(map.next_value::<Vec<
                                    crate::types::indices_resolve_index::ResolveIndexAliasItem,
                                >>()?);
                        }
                        "data_streams" => {
                            if f_data_streams.is_some() {
                                return Err(serde::de::Error::duplicate_field("data_streams"));
                            }
                            f_data_streams = Some(map.next_value::<Vec<
                                crate::types::indices_resolve_index::ResolveIndexDataStreamsItem,
                            >>()?);
                        }
                        "indices" => {
                            if f_indices.is_some() {
                                return Err(serde::de::Error::duplicate_field("indices"));
                            }
                            f_indices = Some(map.next_value::<Vec<crate::types::indices_resolve_index::ResolveIndexItem>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ResolveIndexResponse {
                    aliases: f_aliases.ok_or_else(|| serde::de::Error::missing_field("aliases"))?,
                    data_streams: f_data_streams
                        .ok_or_else(|| serde::de::Error::missing_field("data_streams"))?,
                    indices: f_indices.ok_or_else(|| serde::de::Error::missing_field("indices"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ResolveIndexResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("aliases", &self.aliases)?;
        map.serialize_entry("data_streams", &self.data_streams)?;
        map.serialize_entry("indices", &self.indices)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RolloverResponse {
    #[doc = "For a successful response, this value is always true. On failure, an exception is returned instead."]
    pub acknowledged: bool,
    pub conditions: std::collections::BTreeMap<String, bool>,
    pub dry_run: bool,
    pub new_index: String,
    pub old_index: String,
    pub rolled_over: bool,
    pub shards_acknowledged: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RolloverResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RolloverResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RolloverResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_acknowledged: Option<bool> = None;
                let mut f_conditions: Option<std::collections::BTreeMap<String, bool>> = None;
                let mut f_dry_run: Option<bool> = None;
                let mut f_new_index: Option<String> = None;
                let mut f_old_index: Option<String> = None;
                let mut f_rolled_over: Option<bool> = None;
                let mut f_shards_acknowledged: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "acknowledged" => {
                            if f_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field("acknowledged"));
                            }
                            f_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        "conditions" => {
                            if f_conditions.is_some() {
                                return Err(serde::de::Error::duplicate_field("conditions"));
                            }
                            f_conditions =
                                Some(map.next_value::<std::collections::BTreeMap<String, bool>>()?);
                        }
                        "dry_run" => {
                            if f_dry_run.is_some() {
                                return Err(serde::de::Error::duplicate_field("dry_run"));
                            }
                            f_dry_run = Some(map.next_value::<bool>()?);
                        }
                        "new_index" => {
                            if f_new_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("new_index"));
                            }
                            f_new_index = Some(map.next_value::<String>()?);
                        }
                        "old_index" => {
                            if f_old_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("old_index"));
                            }
                            f_old_index = Some(map.next_value::<String>()?);
                        }
                        "rolled_over" => {
                            if f_rolled_over.is_some() {
                                return Err(serde::de::Error::duplicate_field("rolled_over"));
                            }
                            f_rolled_over = Some(map.next_value::<bool>()?);
                        }
                        "shards_acknowledged" => {
                            if f_shards_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "shards_acknowledged",
                                ));
                            }
                            f_shards_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RolloverResponse {
                    acknowledged: f_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("acknowledged"))?,
                    conditions: f_conditions
                        .ok_or_else(|| serde::de::Error::missing_field("conditions"))?,
                    dry_run: f_dry_run.ok_or_else(|| serde::de::Error::missing_field("dry_run"))?,
                    new_index: f_new_index
                        .ok_or_else(|| serde::de::Error::missing_field("new_index"))?,
                    old_index: f_old_index
                        .ok_or_else(|| serde::de::Error::missing_field("old_index"))?,
                    rolled_over: f_rolled_over
                        .ok_or_else(|| serde::de::Error::missing_field("rolled_over"))?,
                    shards_acknowledged: f_shards_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("shards_acknowledged"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RolloverResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("acknowledged", &self.acknowledged)?;
        map.serialize_entry("conditions", &self.conditions)?;
        map.serialize_entry("dry_run", &self.dry_run)?;
        map.serialize_entry("new_index", &self.new_index)?;
        map.serialize_entry("old_index", &self.old_index)?;
        map.serialize_entry("rolled_over", &self.rolled_over)?;
        map.serialize_entry("shards_acknowledged", &self.shards_acknowledged)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SegmentsResponse {
    pub shards: crate::types::common::ShardStatistics,
    pub indices: std::collections::BTreeMap<String, crate::types::indices_segments::IndexSegment>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SegmentsResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SegmentsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SegmentsResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_shards: Option<crate::types::common::ShardStatistics> = None;
                let mut f_indices: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::indices_segments::IndexSegment,
                    >,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_shards" => {
                            if f_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("_shards"));
                            }
                            f_shards =
                                Some(map.next_value::<crate::types::common::ShardStatistics>()?);
                        }
                        "indices" => {
                            if f_indices.is_some() {
                                return Err(serde::de::Error::duplicate_field("indices"));
                            }
                            f_indices = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                crate::types::indices_segments::IndexSegment,
                            >>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SegmentsResponse {
                    shards: f_shards.ok_or_else(|| serde::de::Error::missing_field("_shards"))?,
                    indices: f_indices.ok_or_else(|| serde::de::Error::missing_field("indices"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SegmentsResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("_shards", &self.shards)?;
        map.serialize_entry("indices", &self.indices)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardStoresResponse {
    pub indices:
        std::collections::BTreeMap<String, crate::types::indices_shard_stores::IndicesShardStores>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardStoresResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardStoresResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardStoresResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_indices: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::indices_shard_stores::IndicesShardStores,
                    >,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "indices" => {
                            if f_indices.is_some() {
                                return Err(serde::de::Error::duplicate_field("indices"));
                            }
                            f_indices = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                crate::types::indices_shard_stores::IndicesShardStores,
                            >>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardStoresResponse {
                    indices: f_indices.ok_or_else(|| serde::de::Error::missing_field("indices"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardStoresResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("indices", &self.indices)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShrinkResponse {
    #[doc = "For a successful response, this value is always true. On failure, an exception is returned instead."]
    pub acknowledged: bool,
    pub index: crate::types::common::IndexName,
    pub shards_acknowledged: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShrinkResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShrinkResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShrinkResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_acknowledged: Option<bool> = None;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut f_shards_acknowledged: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "acknowledged" => {
                            if f_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field("acknowledged"));
                            }
                            f_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        "index" => {
                            if f_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            f_index = Some(map.next_value::<crate::types::common::IndexName>()?);
                        }
                        "shards_acknowledged" => {
                            if f_shards_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "shards_acknowledged",
                                ));
                            }
                            f_shards_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShrinkResponse {
                    acknowledged: f_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("acknowledged"))?,
                    index: f_index.ok_or_else(|| serde::de::Error::missing_field("index"))?,
                    shards_acknowledged: f_shards_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("shards_acknowledged"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShrinkResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("acknowledged", &self.acknowledged)?;
        map.serialize_entry("index", &self.index)?;
        map.serialize_entry("shards_acknowledged", &self.shards_acknowledged)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SimulateIndexTemplateResponse {
    pub overlapping: Option<Vec<crate::types::indices_simulate_template::Overlapping>>,
    pub template: Option<crate::types::indices_simulate_template::Template>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SimulateIndexTemplateResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SimulateIndexTemplateResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SimulateIndexTemplateResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_overlapping: Option<
                    Vec<crate::types::indices_simulate_template::Overlapping>,
                > = None;
                let mut seen_overlapping = false;
                let mut f_template: Option<crate::types::indices_simulate_template::Template> =
                    None;
                let mut seen_template = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "overlapping" => {
                            if seen_overlapping {
                                return Err(serde::de::Error::duplicate_field("overlapping"));
                            }
                            seen_overlapping = true;
                            f_overlapping =
                                map.next_value::<Option<
                                    Vec<crate::types::indices_simulate_template::Overlapping>,
                                >>()?;
                        }
                        "template" => {
                            if seen_template {
                                return Err(serde::de::Error::duplicate_field("template"));
                            }
                            seen_template = true;
                            f_template = map.next_value::<Option<crate::types::indices_simulate_template::Template>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SimulateIndexTemplateResponse {
                    overlapping: f_overlapping,
                    template: f_template,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SimulateIndexTemplateResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.overlapping {
            map.serialize_entry("overlapping", v)?;
        }
        if let Some(v) = &self.template {
            map.serialize_entry("template", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SimulateTemplateResponse {
    pub overlapping: Option<Vec<crate::types::indices_simulate_template::Overlapping>>,
    pub template: crate::types::indices_simulate_template::Template,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SimulateTemplateResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SimulateTemplateResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SimulateTemplateResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_overlapping: Option<
                    Vec<crate::types::indices_simulate_template::Overlapping>,
                > = None;
                let mut seen_overlapping = false;
                let mut f_template: Option<crate::types::indices_simulate_template::Template> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "overlapping" => {
                            if seen_overlapping {
                                return Err(serde::de::Error::duplicate_field("overlapping"));
                            }
                            seen_overlapping = true;
                            f_overlapping =
                                map.next_value::<Option<
                                    Vec<crate::types::indices_simulate_template::Overlapping>,
                                >>()?;
                        }
                        "template" => {
                            if f_template.is_some() {
                                return Err(serde::de::Error::duplicate_field("template"));
                            }
                            f_template = Some(map.next_value::<crate::types::indices_simulate_template::Template>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SimulateTemplateResponse {
                    overlapping: f_overlapping,
                    template: f_template
                        .ok_or_else(|| serde::de::Error::missing_field("template"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SimulateTemplateResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.overlapping {
            map.serialize_entry("overlapping", v)?;
        }
        map.serialize_entry("template", &self.template)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SplitResponse {
    #[doc = "For a successful response, this value is always true. On failure, an exception is returned instead."]
    pub acknowledged: bool,
    pub index: crate::types::common::IndexName,
    pub shards_acknowledged: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SplitResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SplitResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SplitResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_acknowledged: Option<bool> = None;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut f_shards_acknowledged: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "acknowledged" => {
                            if f_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field("acknowledged"));
                            }
                            f_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        "index" => {
                            if f_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            f_index = Some(map.next_value::<crate::types::common::IndexName>()?);
                        }
                        "shards_acknowledged" => {
                            if f_shards_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "shards_acknowledged",
                                ));
                            }
                            f_shards_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SplitResponse {
                    acknowledged: f_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("acknowledged"))?,
                    index: f_index.ok_or_else(|| serde::de::Error::missing_field("index"))?,
                    shards_acknowledged: f_shards_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("shards_acknowledged"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SplitResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("acknowledged", &self.acknowledged)?;
        map.serialize_entry("index", &self.index)?;
        map.serialize_entry("shards_acknowledged", &self.shards_acknowledged)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct StatsResponse {
    pub all: crate::types::indices_stats::AllIndicesStats,
    pub shards: crate::types::common::ShardStatistics,
    pub indices:
        Option<std::collections::BTreeMap<String, crate::types::indices_stats::IndicesStats>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StatsResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StatsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StatsResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_all: Option<crate::types::indices_stats::AllIndicesStats> = None;
                let mut f_shards: Option<crate::types::common::ShardStatistics> = None;
                let mut f_indices: Option<
                    std::collections::BTreeMap<String, crate::types::indices_stats::IndicesStats>,
                > = None;
                let mut seen_indices = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_all" => {
                            if f_all.is_some() {
                                return Err(serde::de::Error::duplicate_field("_all"));
                            }
                            f_all = Some(
                                map.next_value::<crate::types::indices_stats::AllIndicesStats>()?,
                            );
                        }
                        "_shards" => {
                            if f_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("_shards"));
                            }
                            f_shards =
                                Some(map.next_value::<crate::types::common::ShardStatistics>()?);
                        }
                        "indices" => {
                            if seen_indices {
                                return Err(serde::de::Error::duplicate_field("indices"));
                            }
                            seen_indices = true;
                            f_indices = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::indices_stats::IndicesStats,
                                >,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StatsResponse {
                    all: f_all.ok_or_else(|| serde::de::Error::missing_field("_all"))?,
                    shards: f_shards.ok_or_else(|| serde::de::Error::missing_field("_shards"))?,
                    indices: f_indices,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StatsResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("_all", &self.all)?;
        map.serialize_entry("_shards", &self.shards)?;
        if let Some(v) = &self.indices {
            map.serialize_entry("indices", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct UpgradeResponse {
    pub shards: crate::types::common::ShardStatistics,
    pub upgraded_indices: Option<
        std::collections::BTreeMap<String, crate::types::indices_common::UpgradeVersionStatus>,
    >,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UpgradeResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UpgradeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UpgradeResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_shards: Option<crate::types::common::ShardStatistics> = None;
                let mut f_upgraded_indices: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::indices_common::UpgradeVersionStatus,
                    >,
                > = None;
                let mut seen_upgraded_indices = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_shards" => {
                            if f_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("_shards"));
                            }
                            f_shards =
                                Some(map.next_value::<crate::types::common::ShardStatistics>()?);
                        }
                        "upgraded_indices" => {
                            if seen_upgraded_indices {
                                return Err(serde::de::Error::duplicate_field("upgraded_indices"));
                            }
                            seen_upgraded_indices = true;
                            f_upgraded_indices = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::indices_common::UpgradeVersionStatus,
                                >,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UpgradeResponse {
                    shards: f_shards.ok_or_else(|| serde::de::Error::missing_field("_shards"))?,
                    upgraded_indices: f_upgraded_indices,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UpgradeResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("_shards", &self.shards)?;
        if let Some(v) = &self.upgraded_indices {
            map.serialize_entry("upgraded_indices", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ValidateQueryResponse {
    pub shards: Option<crate::types::common::ShardStatistics>,
    pub error: Option<String>,
    pub explanations:
        Option<Vec<crate::types::indices_validate_query::IndicesValidationExplanation>>,
    pub valid: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ValidateQueryResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ValidateQueryResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ValidateQueryResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_shards: Option<crate::types::common::ShardStatistics> = None;
                let mut seen_shards = false;
                let mut f_error: Option<String> = None;
                let mut seen_error = false;
                let mut f_explanations: Option<
                    Vec<crate::types::indices_validate_query::IndicesValidationExplanation>,
                > = None;
                let mut seen_explanations = false;
                let mut f_valid: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_shards" => {
                            if seen_shards {
                                return Err(serde::de::Error::duplicate_field("_shards"));
                            }
                            seen_shards = true;
                            f_shards =
                                map.next_value::<Option<crate::types::common::ShardStatistics>>()?;
                        }
                        "error" => {
                            if seen_error {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            seen_error = true;
                            f_error = map.next_value::<Option<String>>()?;
                        }
                        "explanations" => {
                            if seen_explanations {
                                return Err(serde::de::Error::duplicate_field("explanations"));
                            }
                            seen_explanations = true;
                            f_explanations = map.next_value::<Option<Vec<crate::types::indices_validate_query::IndicesValidationExplanation>>>()?;
                        }
                        "valid" => {
                            if f_valid.is_some() {
                                return Err(serde::de::Error::duplicate_field("valid"));
                            }
                            f_valid = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ValidateQueryResponse {
                    shards: f_shards,
                    error: f_error,
                    explanations: f_explanations,
                    valid: f_valid.ok_or_else(|| serde::de::Error::missing_field("valid"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ValidateQueryResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.shards {
            map.serialize_entry("_shards", v)?;
        }
        if let Some(v) = &self.error {
            map.serialize_entry("error", v)?;
        }
        if let Some(v) = &self.explanations {
            map.serialize_entry("explanations", v)?;
        }
        map.serialize_entry("valid", &self.valid)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
