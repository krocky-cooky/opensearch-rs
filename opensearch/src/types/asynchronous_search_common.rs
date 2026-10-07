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

//! Response types of the `asynchronous_search_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct AsynchronousSearchStats {
    pub cancelled: Option<i64>,
    pub initialized: Option<i64>,
    pub persist_failed: Option<i64>,
    pub persisted: Option<i64>,
    pub rejected: Option<i64>,
    pub running_current: Option<i64>,
    pub search_completed: Option<i64>,
    pub search_failed: Option<i64>,
    pub submitted: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AsynchronousSearchStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AsynchronousSearchStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AsynchronousSearchStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cancelled: Option<i64> = None;
                let mut seen_cancelled = false;
                let mut f_initialized: Option<i64> = None;
                let mut seen_initialized = false;
                let mut f_persist_failed: Option<i64> = None;
                let mut seen_persist_failed = false;
                let mut f_persisted: Option<i64> = None;
                let mut seen_persisted = false;
                let mut f_rejected: Option<i64> = None;
                let mut seen_rejected = false;
                let mut f_running_current: Option<i64> = None;
                let mut seen_running_current = false;
                let mut f_search_completed: Option<i64> = None;
                let mut seen_search_completed = false;
                let mut f_search_failed: Option<i64> = None;
                let mut seen_search_failed = false;
                let mut f_submitted: Option<i64> = None;
                let mut seen_submitted = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cancelled" => {
                            if seen_cancelled {
                                return Err(serde::de::Error::duplicate_field("cancelled"));
                            }
                            seen_cancelled = true;
                            f_cancelled = map.next_value::<Option<i64>>()?;
                        }
                        "initialized" => {
                            if seen_initialized {
                                return Err(serde::de::Error::duplicate_field("initialized"));
                            }
                            seen_initialized = true;
                            f_initialized = map.next_value::<Option<i64>>()?;
                        }
                        "persist_failed" => {
                            if seen_persist_failed {
                                return Err(serde::de::Error::duplicate_field("persist_failed"));
                            }
                            seen_persist_failed = true;
                            f_persist_failed = map.next_value::<Option<i64>>()?;
                        }
                        "persisted" => {
                            if seen_persisted {
                                return Err(serde::de::Error::duplicate_field("persisted"));
                            }
                            seen_persisted = true;
                            f_persisted = map.next_value::<Option<i64>>()?;
                        }
                        "rejected" => {
                            if seen_rejected {
                                return Err(serde::de::Error::duplicate_field("rejected"));
                            }
                            seen_rejected = true;
                            f_rejected = map.next_value::<Option<i64>>()?;
                        }
                        "running_current" => {
                            if seen_running_current {
                                return Err(serde::de::Error::duplicate_field("running_current"));
                            }
                            seen_running_current = true;
                            f_running_current = map.next_value::<Option<i64>>()?;
                        }
                        "search_completed" => {
                            if seen_search_completed {
                                return Err(serde::de::Error::duplicate_field("search_completed"));
                            }
                            seen_search_completed = true;
                            f_search_completed = map.next_value::<Option<i64>>()?;
                        }
                        "search_failed" => {
                            if seen_search_failed {
                                return Err(serde::de::Error::duplicate_field("search_failed"));
                            }
                            seen_search_failed = true;
                            f_search_failed = map.next_value::<Option<i64>>()?;
                        }
                        "submitted" => {
                            if seen_submitted {
                                return Err(serde::de::Error::duplicate_field("submitted"));
                            }
                            seen_submitted = true;
                            f_submitted = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AsynchronousSearchStats {
                    cancelled: f_cancelled,
                    initialized: f_initialized,
                    persist_failed: f_persist_failed,
                    persisted: f_persisted,
                    rejected: f_rejected,
                    running_current: f_running_current,
                    search_completed: f_search_completed,
                    search_failed: f_search_failed,
                    submitted: f_submitted,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AsynchronousSearchStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cancelled {
            map.serialize_entry("cancelled", v)?;
        }
        if let Some(v) = &self.initialized {
            map.serialize_entry("initialized", v)?;
        }
        if let Some(v) = &self.persist_failed {
            map.serialize_entry("persist_failed", v)?;
        }
        if let Some(v) = &self.persisted {
            map.serialize_entry("persisted", v)?;
        }
        if let Some(v) = &self.rejected {
            map.serialize_entry("rejected", v)?;
        }
        if let Some(v) = &self.running_current {
            map.serialize_entry("running_current", v)?;
        }
        if let Some(v) = &self.search_completed {
            map.serialize_entry("search_completed", v)?;
        }
        if let Some(v) = &self.search_failed {
            map.serialize_entry("search_failed", v)?;
        }
        if let Some(v) = &self.submitted {
            map.serialize_entry("submitted", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NodesStats {
    pub asynchronous_search_stats:
        Option<crate::types::asynchronous_search_common::AsynchronousSearchStats>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodesStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodesStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodesStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_asynchronous_search_stats: Option<
                    crate::types::asynchronous_search_common::AsynchronousSearchStats,
                > = None;
                let mut seen_asynchronous_search_stats = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "asynchronous_search_stats" => {
                            if seen_asynchronous_search_stats {
                                return Err(serde::de::Error::duplicate_field(
                                    "asynchronous_search_stats",
                                ));
                            }
                            seen_asynchronous_search_stats = true;
                            f_asynchronous_search_stats = map.next_value::<Option<
                                crate::types::asynchronous_search_common::AsynchronousSearchStats,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodesStats {
                    asynchronous_search_stats: f_asynchronous_search_stats,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodesStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.asynchronous_search_stats {
            map.serialize_entry("asynchronous_search_stats", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ResponseBody {
    pub expiration_time_in_millis: Option<f64>,
    pub id: Option<String>,
    pub response: Option<crate::types::search::SearchResponse>,
    pub start_time_in_millis: Option<f64>,
    pub state: Option<String>,
    pub took: Option<f64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ResponseBody {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ResponseBody;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ResponseBody object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_expiration_time_in_millis: Option<f64> = None;
                let mut seen_expiration_time_in_millis = false;
                let mut f_id: Option<String> = None;
                let mut seen_id = false;
                let mut f_response: Option<crate::types::search::SearchResponse> = None;
                let mut seen_response = false;
                let mut f_start_time_in_millis: Option<f64> = None;
                let mut seen_start_time_in_millis = false;
                let mut f_state: Option<String> = None;
                let mut seen_state = false;
                let mut f_took: Option<f64> = None;
                let mut seen_took = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "expiration_time_in_millis" => {
                            if seen_expiration_time_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "expiration_time_in_millis",
                                ));
                            }
                            seen_expiration_time_in_millis = true;
                            f_expiration_time_in_millis = map.next_value::<Option<f64>>()?;
                        }
                        "id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<String>>()?;
                        }
                        "response" => {
                            if seen_response {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            seen_response = true;
                            f_response =
                                map.next_value::<Option<crate::types::search::SearchResponse>>()?;
                        }
                        "start_time_in_millis" => {
                            if seen_start_time_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "start_time_in_millis",
                                ));
                            }
                            seen_start_time_in_millis = true;
                            f_start_time_in_millis = map.next_value::<Option<f64>>()?;
                        }
                        "state" => {
                            if seen_state {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            seen_state = true;
                            f_state = map.next_value::<Option<String>>()?;
                        }
                        "took" => {
                            if seen_took {
                                return Err(serde::de::Error::duplicate_field("took"));
                            }
                            seen_took = true;
                            f_took = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ResponseBody {
                    expiration_time_in_millis: f_expiration_time_in_millis,
                    id: f_id,
                    response: f_response,
                    start_time_in_millis: f_start_time_in_millis,
                    state: f_state,
                    took: f_took,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ResponseBody {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.expiration_time_in_millis {
            map.serialize_entry("expiration_time_in_millis", v)?;
        }
        if let Some(v) = &self.id {
            map.serialize_entry("id", v)?;
        }
        if let Some(v) = &self.response {
            map.serialize_entry("response", v)?;
        }
        if let Some(v) = &self.start_time_in_millis {
            map.serialize_entry("start_time_in_millis", v)?;
        }
        if let Some(v) = &self.state {
            map.serialize_entry("state", v)?;
        }
        if let Some(v) = &self.took {
            map.serialize_entry("took", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct StatsResponse {
    #[doc = "Contains statistics about the number of nodes selected by the request's node filters."]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    pub cluster_name: Option<String>,
    pub _nodes: Option<
        std::collections::BTreeMap<String, crate::types::asynchronous_search_common::NodesStats>,
    >,
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
                let mut f_nodes: Option<crate::types::common::NodeStatistics> = None;
                let mut seen_nodes = false;
                let mut f_cluster_name: Option<String> = None;
                let mut seen_cluster_name = false;
                let mut f__nodes: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::asynchronous_search_common::NodesStats,
                    >,
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
                        "cluster_name" => {
                            if seen_cluster_name {
                                return Err(serde::de::Error::duplicate_field("cluster_name"));
                            }
                            seen_cluster_name = true;
                            f_cluster_name = map.next_value::<Option<String>>()?;
                        }
                        "nodes" => {
                            if seen__nodes {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            seen__nodes = true;
                            f__nodes = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::asynchronous_search_common::NodesStats,
                                >,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StatsResponse {
                    nodes: f_nodes,
                    cluster_name: f_cluster_name,
                    _nodes: f__nodes,
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
        if let Some(v) = &self.nodes {
            map.serialize_entry("_nodes", v)?;
        }
        if let Some(v) = &self.cluster_name {
            map.serialize_entry("cluster_name", v)?;
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
