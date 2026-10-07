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

//! Response types of the `knn_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct DeletedModel {
    pub model_id: String,
    pub result: crate::types::knn_common::DeletionResult,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DeletedModel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DeletedModel;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DeletedModel object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_model_id: Option<String> = None;
                let mut f_result: Option<crate::types::knn_common::DeletionResult> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "model_id" => {
                            if f_model_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("model_id"));
                            }
                            f_model_id = Some(map.next_value::<String>()?);
                        }
                        "result" => {
                            if f_result.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            f_result =
                                Some(map.next_value::<crate::types::knn_common::DeletionResult>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DeletedModel {
                    model_id: f_model_id
                        .ok_or_else(|| serde::de::Error::missing_field("model_id"))?,
                    result: f_result.ok_or_else(|| serde::de::Error::missing_field("result"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DeletedModel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("model_id", &self.model_id)?;
        map.serialize_entry("result", &self.result)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeletionResult {
    #[doc = "`deleted`"]
    Deleted,
    #[doc = "`error`"]
    Error,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DeletionResult {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DeletionResult::Deleted => "deleted",
            DeletionResult::Error => "error",
            DeletionResult::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DeletionResult {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletionResult {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "deleted" => DeletionResult::Deleted,
            "error" => DeletionResult::Error,
            _ => DeletionResult::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct GraphMergeStats {
    pub current: Option<i64>,
    pub current_docs: Option<i64>,
    pub current_size_in_bytes: Option<crate::types::common::ByteCount>,
    pub total: Option<i64>,
    pub total_docs: Option<i64>,
    pub total_size_in_bytes: Option<crate::types::common::ByteCount>,
    pub total_time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GraphMergeStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GraphMergeStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GraphMergeStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_current: Option<i64> = None;
                let mut seen_current = false;
                let mut f_current_docs: Option<i64> = None;
                let mut seen_current_docs = false;
                let mut f_current_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_current_size_in_bytes = false;
                let mut f_total: Option<i64> = None;
                let mut seen_total = false;
                let mut f_total_docs: Option<i64> = None;
                let mut seen_total_docs = false;
                let mut f_total_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_total_size_in_bytes = false;
                let mut f_total_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut seen_total_time_in_millis = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "current" => {
                            if seen_current {
                                return Err(serde::de::Error::duplicate_field("current"));
                            }
                            seen_current = true;
                            f_current = map.next_value::<Option<i64>>()?;
                        }
                        "current_docs" => {
                            if seen_current_docs {
                                return Err(serde::de::Error::duplicate_field("current_docs"));
                            }
                            seen_current_docs = true;
                            f_current_docs = map.next_value::<Option<i64>>()?;
                        }
                        "current_size_in_bytes" => {
                            if seen_current_size_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "current_size_in_bytes",
                                ));
                            }
                            seen_current_size_in_bytes = true;
                            f_current_size_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map.next_value::<Option<i64>>()?;
                        }
                        "total_docs" => {
                            if seen_total_docs {
                                return Err(serde::de::Error::duplicate_field("total_docs"));
                            }
                            seen_total_docs = true;
                            f_total_docs = map.next_value::<Option<i64>>()?;
                        }
                        "total_size_in_bytes" => {
                            if seen_total_size_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_size_in_bytes",
                                ));
                            }
                            seen_total_size_in_bytes = true;
                            f_total_size_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "total_time_in_millis" => {
                            if seen_total_time_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_time_in_millis",
                                ));
                            }
                            seen_total_time_in_millis = true;
                            f_total_time_in_millis = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GraphMergeStats {
                    current: f_current,
                    current_docs: f_current_docs,
                    current_size_in_bytes: f_current_size_in_bytes,
                    total: f_total,
                    total_docs: f_total_docs,
                    total_size_in_bytes: f_total_size_in_bytes,
                    total_time_in_millis: f_total_time_in_millis,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GraphMergeStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.current {
            map.serialize_entry("current", v)?;
        }
        if let Some(v) = &self.current_docs {
            map.serialize_entry("current_docs", v)?;
        }
        if let Some(v) = &self.current_size_in_bytes {
            map.serialize_entry("current_size_in_bytes", v)?;
        }
        if let Some(v) = &self.total {
            map.serialize_entry("total", v)?;
        }
        if let Some(v) = &self.total_docs {
            map.serialize_entry("total_docs", v)?;
        }
        if let Some(v) = &self.total_size_in_bytes {
            map.serialize_entry("total_size_in_bytes", v)?;
        }
        if let Some(v) = &self.total_time_in_millis {
            map.serialize_entry("total_time_in_millis", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct GraphRefreshStats {
    pub total: Option<i64>,
    pub total_time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GraphRefreshStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GraphRefreshStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GraphRefreshStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_total: Option<i64> = None;
                let mut seen_total = false;
                let mut f_total_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut seen_total_time_in_millis = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map.next_value::<Option<i64>>()?;
                        }
                        "total_time_in_millis" => {
                            if seen_total_time_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_time_in_millis",
                                ));
                            }
                            seen_total_time_in_millis = true;
                            f_total_time_in_millis = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GraphRefreshStats {
                    total: f_total,
                    total_time_in_millis: f_total_time_in_millis,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GraphRefreshStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.total {
            map.serialize_entry("total", v)?;
        }
        if let Some(v) = &self.total_time_in_millis {
            map.serialize_entry("total_time_in_millis", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct GraphStats {
    pub merge: Option<crate::types::knn_common::GraphMergeStats>,
    pub refresh: Option<crate::types::knn_common::GraphRefreshStats>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GraphStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GraphStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GraphStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_merge: Option<crate::types::knn_common::GraphMergeStats> = None;
                let mut seen_merge = false;
                let mut f_refresh: Option<crate::types::knn_common::GraphRefreshStats> = None;
                let mut seen_refresh = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "merge" => {
                            if seen_merge {
                                return Err(serde::de::Error::duplicate_field("merge"));
                            }
                            seen_merge = true;
                            f_merge = map
                                .next_value::<Option<crate::types::knn_common::GraphMergeStats>>(
                                )?;
                        }
                        "refresh" => {
                            if seen_refresh {
                                return Err(serde::de::Error::duplicate_field("refresh"));
                            }
                            seen_refresh = true;
                            f_refresh = map
                                .next_value::<Option<crate::types::knn_common::GraphRefreshStats>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GraphStats {
                    merge: f_merge,
                    refresh: f_refresh,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GraphStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.merge {
            map.serialize_entry("merge", v)?;
        }
        if let Some(v) = &self.refresh {
            map.serialize_entry("refresh", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NodeStats {
    pub cache_capacity_reached: Option<bool>,
    pub eviction_count: Option<i64>,
    pub faiss_initialized: Option<bool>,
    pub graph_index_errors: Option<i64>,
    pub graph_index_requests: Option<i64>,
    pub graph_memory_usage: Option<crate::types::common::ByteCount>,
    pub graph_memory_usage_percentage: Option<crate::types::common::PercentageNumber>,
    pub graph_query_errors: Option<i64>,
    pub graph_query_requests: Option<i64>,
    pub graph_stats: Option<crate::types::knn_common::GraphStats>,
    pub hit_count: Option<i64>,
    pub indexing_from_model_degraded: Option<bool>,
    pub indices_in_cache:
        Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub knn_query_requests: Option<i64>,
    pub knn_query_with_filter_requests: Option<i64>,
    pub load_exception_count: Option<i64>,
    pub load_success_count: Option<i64>,
    pub lucene_initialized: Option<bool>,
    pub max_distance_query_requests: Option<i64>,
    pub max_distance_query_with_filter_requests: Option<i64>,
    pub min_score_query_requests: Option<i64>,
    pub min_score_query_with_filter_requests: Option<i64>,
    pub miss_count: Option<i64>,
    pub nmslib_initialized: Option<bool>,
    pub remote_vector_index_build_stats:
        Option<crate::types::knn_common::RemoteVectorIndexBuildStats>,
    pub script_compilation_errors: Option<i64>,
    pub script_compilations: Option<i64>,
    pub script_query_errors: Option<i64>,
    pub script_query_requests: Option<i64>,
    pub total_load_time: Option<crate::types::common::DurationValueUnitNanos>,
    pub training_errors: Option<i64>,
    pub training_memory_usage: Option<crate::types::common::ByteCount>,
    pub training_memory_usage_percentage: Option<crate::types::common::PercentageNumber>,
    pub training_requests: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cache_capacity_reached: Option<bool> = None;
                let mut seen_cache_capacity_reached = false;
                let mut f_eviction_count: Option<i64> = None;
                let mut seen_eviction_count = false;
                let mut f_faiss_initialized: Option<bool> = None;
                let mut seen_faiss_initialized = false;
                let mut f_graph_index_errors: Option<i64> = None;
                let mut seen_graph_index_errors = false;
                let mut f_graph_index_requests: Option<i64> = None;
                let mut seen_graph_index_requests = false;
                let mut f_graph_memory_usage: Option<crate::types::common::ByteCount> = None;
                let mut seen_graph_memory_usage = false;
                let mut f_graph_memory_usage_percentage: Option<
                    crate::types::common::PercentageNumber,
                > = None;
                let mut seen_graph_memory_usage_percentage = false;
                let mut f_graph_query_errors: Option<i64> = None;
                let mut seen_graph_query_errors = false;
                let mut f_graph_query_requests: Option<i64> = None;
                let mut seen_graph_query_requests = false;
                let mut f_graph_stats: Option<crate::types::knn_common::GraphStats> = None;
                let mut seen_graph_stats = false;
                let mut f_hit_count: Option<i64> = None;
                let mut seen_hit_count = false;
                let mut f_indexing_from_model_degraded: Option<bool> = None;
                let mut seen_indexing_from_model_degraded = false;
                let mut f_indices_in_cache: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_indices_in_cache = false;
                let mut f_knn_query_requests: Option<i64> = None;
                let mut seen_knn_query_requests = false;
                let mut f_knn_query_with_filter_requests: Option<i64> = None;
                let mut seen_knn_query_with_filter_requests = false;
                let mut f_load_exception_count: Option<i64> = None;
                let mut seen_load_exception_count = false;
                let mut f_load_success_count: Option<i64> = None;
                let mut seen_load_success_count = false;
                let mut f_lucene_initialized: Option<bool> = None;
                let mut seen_lucene_initialized = false;
                let mut f_max_distance_query_requests: Option<i64> = None;
                let mut seen_max_distance_query_requests = false;
                let mut f_max_distance_query_with_filter_requests: Option<i64> = None;
                let mut seen_max_distance_query_with_filter_requests = false;
                let mut f_min_score_query_requests: Option<i64> = None;
                let mut seen_min_score_query_requests = false;
                let mut f_min_score_query_with_filter_requests: Option<i64> = None;
                let mut seen_min_score_query_with_filter_requests = false;
                let mut f_miss_count: Option<i64> = None;
                let mut seen_miss_count = false;
                let mut f_nmslib_initialized: Option<bool> = None;
                let mut seen_nmslib_initialized = false;
                let mut f_remote_vector_index_build_stats: Option<
                    crate::types::knn_common::RemoteVectorIndexBuildStats,
                > = None;
                let mut seen_remote_vector_index_build_stats = false;
                let mut f_script_compilation_errors: Option<i64> = None;
                let mut seen_script_compilation_errors = false;
                let mut f_script_compilations: Option<i64> = None;
                let mut seen_script_compilations = false;
                let mut f_script_query_errors: Option<i64> = None;
                let mut seen_script_query_errors = false;
                let mut f_script_query_requests: Option<i64> = None;
                let mut seen_script_query_requests = false;
                let mut f_total_load_time: Option<crate::types::common::DurationValueUnitNanos> =
                    None;
                let mut seen_total_load_time = false;
                let mut f_training_errors: Option<i64> = None;
                let mut seen_training_errors = false;
                let mut f_training_memory_usage: Option<crate::types::common::ByteCount> = None;
                let mut seen_training_memory_usage = false;
                let mut f_training_memory_usage_percentage: Option<
                    crate::types::common::PercentageNumber,
                > = None;
                let mut seen_training_memory_usage_percentage = false;
                let mut f_training_requests: Option<i64> = None;
                let mut seen_training_requests = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cache_capacity_reached" => {
                            if seen_cache_capacity_reached {
                                return Err(serde::de::Error::duplicate_field(
                                    "cache_capacity_reached",
                                ));
                            }
                            seen_cache_capacity_reached = true;
                            f_cache_capacity_reached = map.next_value::<Option<bool>>()?;
                        }
                        "eviction_count" => {
                            if seen_eviction_count {
                                return Err(serde::de::Error::duplicate_field("eviction_count"));
                            }
                            seen_eviction_count = true;
                            f_eviction_count = map.next_value::<Option<i64>>()?;
                        }
                        "faiss_initialized" => {
                            if seen_faiss_initialized {
                                return Err(serde::de::Error::duplicate_field("faiss_initialized"));
                            }
                            seen_faiss_initialized = true;
                            f_faiss_initialized = map.next_value::<Option<bool>>()?;
                        }
                        "graph_index_errors" => {
                            if seen_graph_index_errors {
                                return Err(serde::de::Error::duplicate_field(
                                    "graph_index_errors",
                                ));
                            }
                            seen_graph_index_errors = true;
                            f_graph_index_errors = map.next_value::<Option<i64>>()?;
                        }
                        "graph_index_requests" => {
                            if seen_graph_index_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "graph_index_requests",
                                ));
                            }
                            seen_graph_index_requests = true;
                            f_graph_index_requests = map.next_value::<Option<i64>>()?;
                        }
                        "graph_memory_usage" => {
                            if seen_graph_memory_usage {
                                return Err(serde::de::Error::duplicate_field(
                                    "graph_memory_usage",
                                ));
                            }
                            seen_graph_memory_usage = true;
                            f_graph_memory_usage =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "graph_memory_usage_percentage" => {
                            if seen_graph_memory_usage_percentage {
                                return Err(serde::de::Error::duplicate_field(
                                    "graph_memory_usage_percentage",
                                ));
                            }
                            seen_graph_memory_usage_percentage = true;
                            f_graph_memory_usage_percentage =
                                map.next_value::<Option<crate::types::common::PercentageNumber>>()?;
                        }
                        "graph_query_errors" => {
                            if seen_graph_query_errors {
                                return Err(serde::de::Error::duplicate_field(
                                    "graph_query_errors",
                                ));
                            }
                            seen_graph_query_errors = true;
                            f_graph_query_errors = map.next_value::<Option<i64>>()?;
                        }
                        "graph_query_requests" => {
                            if seen_graph_query_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "graph_query_requests",
                                ));
                            }
                            seen_graph_query_requests = true;
                            f_graph_query_requests = map.next_value::<Option<i64>>()?;
                        }
                        "graph_stats" => {
                            if seen_graph_stats {
                                return Err(serde::de::Error::duplicate_field("graph_stats"));
                            }
                            seen_graph_stats = true;
                            f_graph_stats =
                                map.next_value::<Option<crate::types::knn_common::GraphStats>>()?;
                        }
                        "hit_count" => {
                            if seen_hit_count {
                                return Err(serde::de::Error::duplicate_field("hit_count"));
                            }
                            seen_hit_count = true;
                            f_hit_count = map.next_value::<Option<i64>>()?;
                        }
                        "indexing_from_model_degraded" => {
                            if seen_indexing_from_model_degraded {
                                return Err(serde::de::Error::duplicate_field(
                                    "indexing_from_model_degraded",
                                ));
                            }
                            seen_indexing_from_model_degraded = true;
                            f_indexing_from_model_degraded = map.next_value::<Option<bool>>()?;
                        }
                        "indices_in_cache" => {
                            if seen_indices_in_cache {
                                return Err(serde::de::Error::duplicate_field("indices_in_cache"));
                            }
                            seen_indices_in_cache = true;
                            f_indices_in_cache = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "knn_query_requests" => {
                            if seen_knn_query_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "knn_query_requests",
                                ));
                            }
                            seen_knn_query_requests = true;
                            f_knn_query_requests = map.next_value::<Option<i64>>()?;
                        }
                        "knn_query_with_filter_requests" => {
                            if seen_knn_query_with_filter_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "knn_query_with_filter_requests",
                                ));
                            }
                            seen_knn_query_with_filter_requests = true;
                            f_knn_query_with_filter_requests = map.next_value::<Option<i64>>()?;
                        }
                        "load_exception_count" => {
                            if seen_load_exception_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "load_exception_count",
                                ));
                            }
                            seen_load_exception_count = true;
                            f_load_exception_count = map.next_value::<Option<i64>>()?;
                        }
                        "load_success_count" => {
                            if seen_load_success_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "load_success_count",
                                ));
                            }
                            seen_load_success_count = true;
                            f_load_success_count = map.next_value::<Option<i64>>()?;
                        }
                        "lucene_initialized" => {
                            if seen_lucene_initialized {
                                return Err(serde::de::Error::duplicate_field(
                                    "lucene_initialized",
                                ));
                            }
                            seen_lucene_initialized = true;
                            f_lucene_initialized = map.next_value::<Option<bool>>()?;
                        }
                        "max_distance_query_requests" => {
                            if seen_max_distance_query_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_distance_query_requests",
                                ));
                            }
                            seen_max_distance_query_requests = true;
                            f_max_distance_query_requests = map.next_value::<Option<i64>>()?;
                        }
                        "max_distance_query_with_filter_requests" => {
                            if seen_max_distance_query_with_filter_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_distance_query_with_filter_requests",
                                ));
                            }
                            seen_max_distance_query_with_filter_requests = true;
                            f_max_distance_query_with_filter_requests =
                                map.next_value::<Option<i64>>()?;
                        }
                        "min_score_query_requests" => {
                            if seen_min_score_query_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "min_score_query_requests",
                                ));
                            }
                            seen_min_score_query_requests = true;
                            f_min_score_query_requests = map.next_value::<Option<i64>>()?;
                        }
                        "min_score_query_with_filter_requests" => {
                            if seen_min_score_query_with_filter_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "min_score_query_with_filter_requests",
                                ));
                            }
                            seen_min_score_query_with_filter_requests = true;
                            f_min_score_query_with_filter_requests =
                                map.next_value::<Option<i64>>()?;
                        }
                        "miss_count" => {
                            if seen_miss_count {
                                return Err(serde::de::Error::duplicate_field("miss_count"));
                            }
                            seen_miss_count = true;
                            f_miss_count = map.next_value::<Option<i64>>()?;
                        }
                        "nmslib_initialized" => {
                            if seen_nmslib_initialized {
                                return Err(serde::de::Error::duplicate_field(
                                    "nmslib_initialized",
                                ));
                            }
                            seen_nmslib_initialized = true;
                            f_nmslib_initialized = map.next_value::<Option<bool>>()?;
                        }
                        "remote_vector_index_build_stats" => {
                            if seen_remote_vector_index_build_stats {
                                return Err(serde::de::Error::duplicate_field(
                                    "remote_vector_index_build_stats",
                                ));
                            }
                            seen_remote_vector_index_build_stats = true;
                            f_remote_vector_index_build_stats = map.next_value::<Option<crate::types::knn_common::RemoteVectorIndexBuildStats>>()?;
                        }
                        "script_compilation_errors" => {
                            if seen_script_compilation_errors {
                                return Err(serde::de::Error::duplicate_field(
                                    "script_compilation_errors",
                                ));
                            }
                            seen_script_compilation_errors = true;
                            f_script_compilation_errors = map.next_value::<Option<i64>>()?;
                        }
                        "script_compilations" => {
                            if seen_script_compilations {
                                return Err(serde::de::Error::duplicate_field(
                                    "script_compilations",
                                ));
                            }
                            seen_script_compilations = true;
                            f_script_compilations = map.next_value::<Option<i64>>()?;
                        }
                        "script_query_errors" => {
                            if seen_script_query_errors {
                                return Err(serde::de::Error::duplicate_field(
                                    "script_query_errors",
                                ));
                            }
                            seen_script_query_errors = true;
                            f_script_query_errors = map.next_value::<Option<i64>>()?;
                        }
                        "script_query_requests" => {
                            if seen_script_query_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "script_query_requests",
                                ));
                            }
                            seen_script_query_requests = true;
                            f_script_query_requests = map.next_value::<Option<i64>>()?;
                        }
                        "total_load_time" => {
                            if seen_total_load_time {
                                return Err(serde::de::Error::duplicate_field("total_load_time"));
                            }
                            seen_total_load_time = true;
                            f_total_load_time = map
                                .next_value::<Option<crate::types::common::DurationValueUnitNanos>>(
                                )?;
                        }
                        "training_errors" => {
                            if seen_training_errors {
                                return Err(serde::de::Error::duplicate_field("training_errors"));
                            }
                            seen_training_errors = true;
                            f_training_errors = map.next_value::<Option<i64>>()?;
                        }
                        "training_memory_usage" => {
                            if seen_training_memory_usage {
                                return Err(serde::de::Error::duplicate_field(
                                    "training_memory_usage",
                                ));
                            }
                            seen_training_memory_usage = true;
                            f_training_memory_usage =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "training_memory_usage_percentage" => {
                            if seen_training_memory_usage_percentage {
                                return Err(serde::de::Error::duplicate_field(
                                    "training_memory_usage_percentage",
                                ));
                            }
                            seen_training_memory_usage_percentage = true;
                            f_training_memory_usage_percentage =
                                map.next_value::<Option<crate::types::common::PercentageNumber>>()?;
                        }
                        "training_requests" => {
                            if seen_training_requests {
                                return Err(serde::de::Error::duplicate_field("training_requests"));
                            }
                            seen_training_requests = true;
                            f_training_requests = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeStats {
                    cache_capacity_reached: f_cache_capacity_reached,
                    eviction_count: f_eviction_count,
                    faiss_initialized: f_faiss_initialized,
                    graph_index_errors: f_graph_index_errors,
                    graph_index_requests: f_graph_index_requests,
                    graph_memory_usage: f_graph_memory_usage,
                    graph_memory_usage_percentage: f_graph_memory_usage_percentage,
                    graph_query_errors: f_graph_query_errors,
                    graph_query_requests: f_graph_query_requests,
                    graph_stats: f_graph_stats,
                    hit_count: f_hit_count,
                    indexing_from_model_degraded: f_indexing_from_model_degraded,
                    indices_in_cache: f_indices_in_cache,
                    knn_query_requests: f_knn_query_requests,
                    knn_query_with_filter_requests: f_knn_query_with_filter_requests,
                    load_exception_count: f_load_exception_count,
                    load_success_count: f_load_success_count,
                    lucene_initialized: f_lucene_initialized,
                    max_distance_query_requests: f_max_distance_query_requests,
                    max_distance_query_with_filter_requests:
                        f_max_distance_query_with_filter_requests,
                    min_score_query_requests: f_min_score_query_requests,
                    min_score_query_with_filter_requests: f_min_score_query_with_filter_requests,
                    miss_count: f_miss_count,
                    nmslib_initialized: f_nmslib_initialized,
                    remote_vector_index_build_stats: f_remote_vector_index_build_stats,
                    script_compilation_errors: f_script_compilation_errors,
                    script_compilations: f_script_compilations,
                    script_query_errors: f_script_query_errors,
                    script_query_requests: f_script_query_requests,
                    total_load_time: f_total_load_time,
                    training_errors: f_training_errors,
                    training_memory_usage: f_training_memory_usage,
                    training_memory_usage_percentage: f_training_memory_usage_percentage,
                    training_requests: f_training_requests,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cache_capacity_reached {
            map.serialize_entry("cache_capacity_reached", v)?;
        }
        if let Some(v) = &self.eviction_count {
            map.serialize_entry("eviction_count", v)?;
        }
        if let Some(v) = &self.faiss_initialized {
            map.serialize_entry("faiss_initialized", v)?;
        }
        if let Some(v) = &self.graph_index_errors {
            map.serialize_entry("graph_index_errors", v)?;
        }
        if let Some(v) = &self.graph_index_requests {
            map.serialize_entry("graph_index_requests", v)?;
        }
        if let Some(v) = &self.graph_memory_usage {
            map.serialize_entry("graph_memory_usage", v)?;
        }
        if let Some(v) = &self.graph_memory_usage_percentage {
            map.serialize_entry("graph_memory_usage_percentage", v)?;
        }
        if let Some(v) = &self.graph_query_errors {
            map.serialize_entry("graph_query_errors", v)?;
        }
        if let Some(v) = &self.graph_query_requests {
            map.serialize_entry("graph_query_requests", v)?;
        }
        if let Some(v) = &self.graph_stats {
            map.serialize_entry("graph_stats", v)?;
        }
        if let Some(v) = &self.hit_count {
            map.serialize_entry("hit_count", v)?;
        }
        if let Some(v) = &self.indexing_from_model_degraded {
            map.serialize_entry("indexing_from_model_degraded", v)?;
        }
        if let Some(v) = &self.indices_in_cache {
            map.serialize_entry("indices_in_cache", v)?;
        }
        if let Some(v) = &self.knn_query_requests {
            map.serialize_entry("knn_query_requests", v)?;
        }
        if let Some(v) = &self.knn_query_with_filter_requests {
            map.serialize_entry("knn_query_with_filter_requests", v)?;
        }
        if let Some(v) = &self.load_exception_count {
            map.serialize_entry("load_exception_count", v)?;
        }
        if let Some(v) = &self.load_success_count {
            map.serialize_entry("load_success_count", v)?;
        }
        if let Some(v) = &self.lucene_initialized {
            map.serialize_entry("lucene_initialized", v)?;
        }
        if let Some(v) = &self.max_distance_query_requests {
            map.serialize_entry("max_distance_query_requests", v)?;
        }
        if let Some(v) = &self.max_distance_query_with_filter_requests {
            map.serialize_entry("max_distance_query_with_filter_requests", v)?;
        }
        if let Some(v) = &self.min_score_query_requests {
            map.serialize_entry("min_score_query_requests", v)?;
        }
        if let Some(v) = &self.min_score_query_with_filter_requests {
            map.serialize_entry("min_score_query_with_filter_requests", v)?;
        }
        if let Some(v) = &self.miss_count {
            map.serialize_entry("miss_count", v)?;
        }
        if let Some(v) = &self.nmslib_initialized {
            map.serialize_entry("nmslib_initialized", v)?;
        }
        if let Some(v) = &self.remote_vector_index_build_stats {
            map.serialize_entry("remote_vector_index_build_stats", v)?;
        }
        if let Some(v) = &self.script_compilation_errors {
            map.serialize_entry("script_compilation_errors", v)?;
        }
        if let Some(v) = &self.script_compilations {
            map.serialize_entry("script_compilations", v)?;
        }
        if let Some(v) = &self.script_query_errors {
            map.serialize_entry("script_query_errors", v)?;
        }
        if let Some(v) = &self.script_query_requests {
            map.serialize_entry("script_query_requests", v)?;
        }
        if let Some(v) = &self.total_load_time {
            map.serialize_entry("total_load_time", v)?;
        }
        if let Some(v) = &self.training_errors {
            map.serialize_entry("training_errors", v)?;
        }
        if let Some(v) = &self.training_memory_usage {
            map.serialize_entry("training_memory_usage", v)?;
        }
        if let Some(v) = &self.training_memory_usage_percentage {
            map.serialize_entry("training_memory_usage_percentage", v)?;
        }
        if let Some(v) = &self.training_requests {
            map.serialize_entry("training_requests", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RemoteVectorIndexBuildStats {
    pub build_stats: Option<crate::types::knn_common::RemoteVectorIndexBuildStatsDetails>,
    pub client_stats: Option<crate::types::knn_common::RemoteVectorIndexClientStats>,
    pub repository_stats: Option<crate::types::knn_common::RemoteVectorIndexRepositoryStats>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RemoteVectorIndexBuildStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RemoteVectorIndexBuildStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RemoteVectorIndexBuildStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_build_stats: Option<
                    crate::types::knn_common::RemoteVectorIndexBuildStatsDetails,
                > = None;
                let mut seen_build_stats = false;
                let mut f_client_stats: Option<
                    crate::types::knn_common::RemoteVectorIndexClientStats,
                > = None;
                let mut seen_client_stats = false;
                let mut f_repository_stats: Option<
                    crate::types::knn_common::RemoteVectorIndexRepositoryStats,
                > = None;
                let mut seen_repository_stats = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "build_stats" => {
                            if seen_build_stats {
                                return Err(serde::de::Error::duplicate_field("build_stats"));
                            }
                            seen_build_stats = true;
                            f_build_stats = map.next_value::<Option<
                                crate::types::knn_common::RemoteVectorIndexBuildStatsDetails,
                            >>()?;
                        }
                        "client_stats" => {
                            if seen_client_stats {
                                return Err(serde::de::Error::duplicate_field("client_stats"));
                            }
                            seen_client_stats = true;
                            f_client_stats = map.next_value::<Option<crate::types::knn_common::RemoteVectorIndexClientStats>>()?;
                        }
                        "repository_stats" => {
                            if seen_repository_stats {
                                return Err(serde::de::Error::duplicate_field("repository_stats"));
                            }
                            seen_repository_stats = true;
                            f_repository_stats = map.next_value::<Option<
                                crate::types::knn_common::RemoteVectorIndexRepositoryStats,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RemoteVectorIndexBuildStats {
                    build_stats: f_build_stats,
                    client_stats: f_client_stats,
                    repository_stats: f_repository_stats,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RemoteVectorIndexBuildStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.build_stats {
            map.serialize_entry("build_stats", v)?;
        }
        if let Some(v) = &self.client_stats {
            map.serialize_entry("client_stats", v)?;
        }
        if let Some(v) = &self.repository_stats {
            map.serialize_entry("repository_stats", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RemoteVectorIndexBuildStatsDetails {
    pub remote_index_build_current_flush_operations: Option<i64>,
    pub remote_index_build_current_flush_size: Option<crate::types::common::ByteCount>,
    pub remote_index_build_current_merge_operations: Option<i64>,
    pub remote_index_build_current_merge_size: Option<crate::types::common::ByteCount>,
    pub remote_index_build_flush_time_in_millis:
        Option<crate::types::common::DurationValueUnitMillis>,
    pub remote_index_build_merge_time_in_millis:
        Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RemoteVectorIndexBuildStatsDetails {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RemoteVectorIndexBuildStatsDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RemoteVectorIndexBuildStatsDetails object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_remote_index_build_current_flush_operations: Option<i64> = None;
                let mut seen_remote_index_build_current_flush_operations = false;
                let mut f_remote_index_build_current_flush_size: Option<
                    crate::types::common::ByteCount,
                > = None;
                let mut seen_remote_index_build_current_flush_size = false;
                let mut f_remote_index_build_current_merge_operations: Option<i64> = None;
                let mut seen_remote_index_build_current_merge_operations = false;
                let mut f_remote_index_build_current_merge_size: Option<
                    crate::types::common::ByteCount,
                > = None;
                let mut seen_remote_index_build_current_merge_size = false;
                let mut f_remote_index_build_flush_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut seen_remote_index_build_flush_time_in_millis = false;
                let mut f_remote_index_build_merge_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut seen_remote_index_build_merge_time_in_millis = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "remote_index_build_current_flush_operations" => {
                            if seen_remote_index_build_current_flush_operations {
                                return Err(serde::de::Error::duplicate_field(
                                    "remote_index_build_current_flush_operations",
                                ));
                            }
                            seen_remote_index_build_current_flush_operations = true;
                            f_remote_index_build_current_flush_operations =
                                map.next_value::<Option<i64>>()?;
                        }
                        "remote_index_build_current_flush_size" => {
                            if seen_remote_index_build_current_flush_size {
                                return Err(serde::de::Error::duplicate_field(
                                    "remote_index_build_current_flush_size",
                                ));
                            }
                            seen_remote_index_build_current_flush_size = true;
                            f_remote_index_build_current_flush_size =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "remote_index_build_current_merge_operations" => {
                            if seen_remote_index_build_current_merge_operations {
                                return Err(serde::de::Error::duplicate_field(
                                    "remote_index_build_current_merge_operations",
                                ));
                            }
                            seen_remote_index_build_current_merge_operations = true;
                            f_remote_index_build_current_merge_operations =
                                map.next_value::<Option<i64>>()?;
                        }
                        "remote_index_build_current_merge_size" => {
                            if seen_remote_index_build_current_merge_size {
                                return Err(serde::de::Error::duplicate_field(
                                    "remote_index_build_current_merge_size",
                                ));
                            }
                            seen_remote_index_build_current_merge_size = true;
                            f_remote_index_build_current_merge_size =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "remote_index_build_flush_time_in_millis" => {
                            if seen_remote_index_build_flush_time_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "remote_index_build_flush_time_in_millis",
                                ));
                            }
                            seen_remote_index_build_flush_time_in_millis = true;
                            f_remote_index_build_flush_time_in_millis = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        "remote_index_build_merge_time_in_millis" => {
                            if seen_remote_index_build_merge_time_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "remote_index_build_merge_time_in_millis",
                                ));
                            }
                            seen_remote_index_build_merge_time_in_millis = true;
                            f_remote_index_build_merge_time_in_millis = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RemoteVectorIndexBuildStatsDetails {
                    remote_index_build_current_flush_operations:
                        f_remote_index_build_current_flush_operations,
                    remote_index_build_current_flush_size: f_remote_index_build_current_flush_size,
                    remote_index_build_current_merge_operations:
                        f_remote_index_build_current_merge_operations,
                    remote_index_build_current_merge_size: f_remote_index_build_current_merge_size,
                    remote_index_build_flush_time_in_millis:
                        f_remote_index_build_flush_time_in_millis,
                    remote_index_build_merge_time_in_millis:
                        f_remote_index_build_merge_time_in_millis,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RemoteVectorIndexBuildStatsDetails {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.remote_index_build_current_flush_operations {
            map.serialize_entry("remote_index_build_current_flush_operations", v)?;
        }
        if let Some(v) = &self.remote_index_build_current_flush_size {
            map.serialize_entry("remote_index_build_current_flush_size", v)?;
        }
        if let Some(v) = &self.remote_index_build_current_merge_operations {
            map.serialize_entry("remote_index_build_current_merge_operations", v)?;
        }
        if let Some(v) = &self.remote_index_build_current_merge_size {
            map.serialize_entry("remote_index_build_current_merge_size", v)?;
        }
        if let Some(v) = &self.remote_index_build_flush_time_in_millis {
            map.serialize_entry("remote_index_build_flush_time_in_millis", v)?;
        }
        if let Some(v) = &self.remote_index_build_merge_time_in_millis {
            map.serialize_entry("remote_index_build_merge_time_in_millis", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RemoteVectorIndexClientStats {
    pub build_request_failure_count: Option<i64>,
    pub build_request_success_count: Option<i64>,
    pub index_build_failure_count: Option<i64>,
    pub index_build_success_count: Option<i64>,
    pub status_request_failure_count: Option<i64>,
    pub status_request_success_count: Option<i64>,
    pub waiting_time_in_ms: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RemoteVectorIndexClientStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RemoteVectorIndexClientStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RemoteVectorIndexClientStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_build_request_failure_count: Option<i64> = None;
                let mut seen_build_request_failure_count = false;
                let mut f_build_request_success_count: Option<i64> = None;
                let mut seen_build_request_success_count = false;
                let mut f_index_build_failure_count: Option<i64> = None;
                let mut seen_index_build_failure_count = false;
                let mut f_index_build_success_count: Option<i64> = None;
                let mut seen_index_build_success_count = false;
                let mut f_status_request_failure_count: Option<i64> = None;
                let mut seen_status_request_failure_count = false;
                let mut f_status_request_success_count: Option<i64> = None;
                let mut seen_status_request_success_count = false;
                let mut f_waiting_time_in_ms: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut seen_waiting_time_in_ms = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "build_request_failure_count" => {
                            if seen_build_request_failure_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "build_request_failure_count",
                                ));
                            }
                            seen_build_request_failure_count = true;
                            f_build_request_failure_count = map.next_value::<Option<i64>>()?;
                        }
                        "build_request_success_count" => {
                            if seen_build_request_success_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "build_request_success_count",
                                ));
                            }
                            seen_build_request_success_count = true;
                            f_build_request_success_count = map.next_value::<Option<i64>>()?;
                        }
                        "index_build_failure_count" => {
                            if seen_index_build_failure_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "index_build_failure_count",
                                ));
                            }
                            seen_index_build_failure_count = true;
                            f_index_build_failure_count = map.next_value::<Option<i64>>()?;
                        }
                        "index_build_success_count" => {
                            if seen_index_build_success_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "index_build_success_count",
                                ));
                            }
                            seen_index_build_success_count = true;
                            f_index_build_success_count = map.next_value::<Option<i64>>()?;
                        }
                        "status_request_failure_count" => {
                            if seen_status_request_failure_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "status_request_failure_count",
                                ));
                            }
                            seen_status_request_failure_count = true;
                            f_status_request_failure_count = map.next_value::<Option<i64>>()?;
                        }
                        "status_request_success_count" => {
                            if seen_status_request_success_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "status_request_success_count",
                                ));
                            }
                            seen_status_request_success_count = true;
                            f_status_request_success_count = map.next_value::<Option<i64>>()?;
                        }
                        "waiting_time_in_ms" => {
                            if seen_waiting_time_in_ms {
                                return Err(serde::de::Error::duplicate_field(
                                    "waiting_time_in_ms",
                                ));
                            }
                            seen_waiting_time_in_ms = true;
                            f_waiting_time_in_ms = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RemoteVectorIndexClientStats {
                    build_request_failure_count: f_build_request_failure_count,
                    build_request_success_count: f_build_request_success_count,
                    index_build_failure_count: f_index_build_failure_count,
                    index_build_success_count: f_index_build_success_count,
                    status_request_failure_count: f_status_request_failure_count,
                    status_request_success_count: f_status_request_success_count,
                    waiting_time_in_ms: f_waiting_time_in_ms,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RemoteVectorIndexClientStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.build_request_failure_count {
            map.serialize_entry("build_request_failure_count", v)?;
        }
        if let Some(v) = &self.build_request_success_count {
            map.serialize_entry("build_request_success_count", v)?;
        }
        if let Some(v) = &self.index_build_failure_count {
            map.serialize_entry("index_build_failure_count", v)?;
        }
        if let Some(v) = &self.index_build_success_count {
            map.serialize_entry("index_build_success_count", v)?;
        }
        if let Some(v) = &self.status_request_failure_count {
            map.serialize_entry("status_request_failure_count", v)?;
        }
        if let Some(v) = &self.status_request_success_count {
            map.serialize_entry("status_request_success_count", v)?;
        }
        if let Some(v) = &self.waiting_time_in_ms {
            map.serialize_entry("waiting_time_in_ms", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RemoteVectorIndexRepositoryStats {
    pub read_failure_count: Option<i64>,
    pub read_success_count: Option<i64>,
    pub successful_read_time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    pub successful_write_time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    pub write_failure_count: Option<i64>,
    pub write_success_count: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RemoteVectorIndexRepositoryStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RemoteVectorIndexRepositoryStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RemoteVectorIndexRepositoryStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_read_failure_count: Option<i64> = None;
                let mut seen_read_failure_count = false;
                let mut f_read_success_count: Option<i64> = None;
                let mut seen_read_success_count = false;
                let mut f_successful_read_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut seen_successful_read_time_in_millis = false;
                let mut f_successful_write_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut seen_successful_write_time_in_millis = false;
                let mut f_write_failure_count: Option<i64> = None;
                let mut seen_write_failure_count = false;
                let mut f_write_success_count: Option<i64> = None;
                let mut seen_write_success_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "read_failure_count" => {
                            if seen_read_failure_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "read_failure_count",
                                ));
                            }
                            seen_read_failure_count = true;
                            f_read_failure_count = map.next_value::<Option<i64>>()?;
                        }
                        "read_success_count" => {
                            if seen_read_success_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "read_success_count",
                                ));
                            }
                            seen_read_success_count = true;
                            f_read_success_count = map.next_value::<Option<i64>>()?;
                        }
                        "successful_read_time_in_millis" => {
                            if seen_successful_read_time_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "successful_read_time_in_millis",
                                ));
                            }
                            seen_successful_read_time_in_millis = true;
                            f_successful_read_time_in_millis = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        "successful_write_time_in_millis" => {
                            if seen_successful_write_time_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "successful_write_time_in_millis",
                                ));
                            }
                            seen_successful_write_time_in_millis = true;
                            f_successful_write_time_in_millis = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        "write_failure_count" => {
                            if seen_write_failure_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "write_failure_count",
                                ));
                            }
                            seen_write_failure_count = true;
                            f_write_failure_count = map.next_value::<Option<i64>>()?;
                        }
                        "write_success_count" => {
                            if seen_write_success_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "write_success_count",
                                ));
                            }
                            seen_write_success_count = true;
                            f_write_success_count = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RemoteVectorIndexRepositoryStats {
                    read_failure_count: f_read_failure_count,
                    read_success_count: f_read_success_count,
                    successful_read_time_in_millis: f_successful_read_time_in_millis,
                    successful_write_time_in_millis: f_successful_write_time_in_millis,
                    write_failure_count: f_write_failure_count,
                    write_success_count: f_write_success_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RemoteVectorIndexRepositoryStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.read_failure_count {
            map.serialize_entry("read_failure_count", v)?;
        }
        if let Some(v) = &self.read_success_count {
            map.serialize_entry("read_success_count", v)?;
        }
        if let Some(v) = &self.successful_read_time_in_millis {
            map.serialize_entry("successful_read_time_in_millis", v)?;
        }
        if let Some(v) = &self.successful_write_time_in_millis {
            map.serialize_entry("successful_write_time_in_millis", v)?;
        }
        if let Some(v) = &self.write_failure_count {
            map.serialize_entry("write_failure_count", v)?;
        }
        if let Some(v) = &self.write_success_count {
            map.serialize_entry("write_success_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Stats {
    #[doc = "Contains statistics about the number of nodes selected by the request's node filters."]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    pub circuit_breaker_triggered: Option<bool>,
    pub cluster_name: Option<crate::types::common::Name>,
    pub model_index_status: Option<crate::types::common::HealthStatus>,
    pub _nodes: Option<std::collections::BTreeMap<String, crate::types::knn_common::NodeStats>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Stats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Stats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Stats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_nodes: Option<crate::types::common::NodeStatistics> = None;
                let mut seen_nodes = false;
                let mut f_circuit_breaker_triggered: Option<bool> = None;
                let mut seen_circuit_breaker_triggered = false;
                let mut f_cluster_name: Option<crate::types::common::Name> = None;
                let mut seen_cluster_name = false;
                let mut f_model_index_status: Option<crate::types::common::HealthStatus> = None;
                let mut seen_model_index_status = false;
                let mut f__nodes: Option<
                    std::collections::BTreeMap<String, crate::types::knn_common::NodeStats>,
                > = None;
                let mut seen__nodes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_nodes" => {
                            if seen_nodes {
                                return Err(serde::de::Error::duplicate_field("_nodes"));
                            }
                            seen_nodes = true;
                            f_nodes =
                                map.next_value::<Option<crate::types::common::NodeStatistics>>()?;
                        }
                        "circuit_breaker_triggered" => {
                            if seen_circuit_breaker_triggered {
                                return Err(serde::de::Error::duplicate_field(
                                    "circuit_breaker_triggered",
                                ));
                            }
                            seen_circuit_breaker_triggered = true;
                            f_circuit_breaker_triggered = map.next_value::<Option<bool>>()?;
                        }
                        "cluster_name" => {
                            if seen_cluster_name {
                                return Err(serde::de::Error::duplicate_field("cluster_name"));
                            }
                            seen_cluster_name = true;
                            f_cluster_name =
                                map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "model_index_status" => {
                            if seen_model_index_status {
                                return Err(serde::de::Error::duplicate_field(
                                    "model_index_status",
                                ));
                            }
                            seen_model_index_status = true;
                            f_model_index_status =
                                map.next_value::<Option<crate::types::common::HealthStatus>>()?;
                        }
                        "nodes" => {
                            if seen__nodes {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            seen__nodes = true;
                            f__nodes = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::knn_common::NodeStats,
                                >,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Stats {
                    nodes: f_nodes,
                    circuit_breaker_triggered: f_circuit_breaker_triggered,
                    cluster_name: f_cluster_name,
                    model_index_status: f_model_index_status,
                    _nodes: f__nodes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.nodes {
            map.serialize_entry("_nodes", v)?;
        }
        if let Some(v) = &self.circuit_breaker_triggered {
            map.serialize_entry("circuit_breaker_triggered", v)?;
        }
        if let Some(v) = &self.cluster_name {
            map.serialize_entry("cluster_name", v)?;
        }
        if let Some(v) = &self.model_index_status {
            map.serialize_entry("model_index_status", v)?;
        }
        if let Some(v) = &self._nodes {
            map.serialize_entry("nodes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
