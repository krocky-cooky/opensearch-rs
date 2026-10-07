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

//! Response types of the `nodes_stats` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct AdaptiveSelection {
    #[doc = "The exponentially weighted moving average queue size of search requests on the keyed node."]
    pub avg_queue_size: Option<i64>,
    pub avg_response_time: Option<crate::types::common::Duration>,
    #[doc = "The exponentially weighted moving average response time, in nanoseconds, of search requests on the keyed node."]
    pub avg_response_time_ns: Option<crate::types::common::DurationValueUnitNanos>,
    pub avg_service_time: Option<crate::types::common::Duration>,
    #[doc = "The exponentially weighted moving average service time, in nanoseconds, of search requests on the keyed node."]
    pub avg_service_time_ns: Option<crate::types::common::DurationValueUnitNanos>,
    #[doc = "The number of outstanding search requests to the keyed node from the node these stats are for."]
    pub outgoing_searches: Option<i64>,
    #[doc = "The rank of this node; used for shard selection when routing search requests."]
    pub rank: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AdaptiveSelection {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AdaptiveSelection;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AdaptiveSelection object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_avg_queue_size: Option<i64> = None;
                let mut seen_avg_queue_size = false;
                let mut f_avg_response_time: Option<crate::types::common::Duration> = None;
                let mut seen_avg_response_time = false;
                let mut f_avg_response_time_ns: Option<
                    crate::types::common::DurationValueUnitNanos,
                > = None;
                let mut seen_avg_response_time_ns = false;
                let mut f_avg_service_time: Option<crate::types::common::Duration> = None;
                let mut seen_avg_service_time = false;
                let mut f_avg_service_time_ns: Option<
                    crate::types::common::DurationValueUnitNanos,
                > = None;
                let mut seen_avg_service_time_ns = false;
                let mut f_outgoing_searches: Option<i64> = None;
                let mut seen_outgoing_searches = false;
                let mut f_rank: Option<String> = None;
                let mut seen_rank = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "avg_queue_size" => {
                            if seen_avg_queue_size {
                                return Err(serde::de::Error::duplicate_field("avg_queue_size"));
                            }
                            seen_avg_queue_size = true;
                            f_avg_queue_size = map.next_value::<Option<i64>>()?;
                        }
                        "avg_response_time" => {
                            if seen_avg_response_time {
                                return Err(serde::de::Error::duplicate_field("avg_response_time"));
                            }
                            seen_avg_response_time = true;
                            f_avg_response_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "avg_response_time_ns" => {
                            if seen_avg_response_time_ns {
                                return Err(serde::de::Error::duplicate_field(
                                    "avg_response_time_ns",
                                ));
                            }
                            seen_avg_response_time_ns = true;
                            f_avg_response_time_ns = map
                                .next_value::<Option<crate::types::common::DurationValueUnitNanos>>(
                                )?;
                        }
                        "avg_service_time" => {
                            if seen_avg_service_time {
                                return Err(serde::de::Error::duplicate_field("avg_service_time"));
                            }
                            seen_avg_service_time = true;
                            f_avg_service_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "avg_service_time_ns" => {
                            if seen_avg_service_time_ns {
                                return Err(serde::de::Error::duplicate_field(
                                    "avg_service_time_ns",
                                ));
                            }
                            seen_avg_service_time_ns = true;
                            f_avg_service_time_ns = map
                                .next_value::<Option<crate::types::common::DurationValueUnitNanos>>(
                                )?;
                        }
                        "outgoing_searches" => {
                            if seen_outgoing_searches {
                                return Err(serde::de::Error::duplicate_field("outgoing_searches"));
                            }
                            seen_outgoing_searches = true;
                            f_outgoing_searches = map.next_value::<Option<i64>>()?;
                        }
                        "rank" => {
                            if seen_rank {
                                return Err(serde::de::Error::duplicate_field("rank"));
                            }
                            seen_rank = true;
                            f_rank = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AdaptiveSelection {
                    avg_queue_size: f_avg_queue_size,
                    avg_response_time: f_avg_response_time,
                    avg_response_time_ns: f_avg_response_time_ns,
                    avg_service_time: f_avg_service_time,
                    avg_service_time_ns: f_avg_service_time_ns,
                    outgoing_searches: f_outgoing_searches,
                    rank: f_rank,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AdaptiveSelection {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.avg_queue_size {
            map.serialize_entry("avg_queue_size", v)?;
        }
        if let Some(v) = &self.avg_response_time {
            map.serialize_entry("avg_response_time", v)?;
        }
        if let Some(v) = &self.avg_response_time_ns {
            map.serialize_entry("avg_response_time_ns", v)?;
        }
        if let Some(v) = &self.avg_service_time {
            map.serialize_entry("avg_service_time", v)?;
        }
        if let Some(v) = &self.avg_service_time_ns {
            map.serialize_entry("avg_service_time_ns", v)?;
        }
        if let Some(v) = &self.outgoing_searches {
            map.serialize_entry("outgoing_searches", v)?;
        }
        if let Some(v) = &self.rank {
            map.serialize_entry("rank", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Breaker {
    #[doc = "Estimated memory used for the operation."]
    pub estimated_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Estimated memory used, in bytes, for the operation."]
    pub estimated_size_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Memory limit for the circuit breaker."]
    pub limit_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Memory limit, in bytes, for the circuit breaker."]
    pub limit_size_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "A constant that all estimates for the circuit breaker are multiplied with to calculate a final estimate."]
    pub overhead: Option<f64>,
    #[doc = "Total number of times the circuit breaker has been triggered and prevented an out of memory error."]
    pub tripped: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Breaker {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Breaker;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Breaker object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_estimated_size: Option<crate::types::common::HumanReadableByteCount> =
                    None;
                let mut seen_estimated_size = false;
                let mut f_estimated_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_estimated_size_in_bytes = false;
                let mut f_limit_size: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_limit_size = false;
                let mut f_limit_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_limit_size_in_bytes = false;
                let mut f_overhead: Option<f64> = None;
                let mut seen_overhead = false;
                let mut f_tripped: Option<i64> = None;
                let mut seen_tripped = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "estimated_size" => {
                            if seen_estimated_size {
                                return Err(serde::de::Error::duplicate_field("estimated_size"));
                            }
                            seen_estimated_size = true;
                            f_estimated_size = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "estimated_size_in_bytes" => {
                            if seen_estimated_size_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "estimated_size_in_bytes",
                                ));
                            }
                            seen_estimated_size_in_bytes = true;
                            f_estimated_size_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "limit_size" => {
                            if seen_limit_size {
                                return Err(serde::de::Error::duplicate_field("limit_size"));
                            }
                            seen_limit_size = true;
                            f_limit_size = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "limit_size_in_bytes" => {
                            if seen_limit_size_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "limit_size_in_bytes",
                                ));
                            }
                            seen_limit_size_in_bytes = true;
                            f_limit_size_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "overhead" => {
                            if seen_overhead {
                                return Err(serde::de::Error::duplicate_field("overhead"));
                            }
                            seen_overhead = true;
                            f_overhead = map.next_value::<Option<f64>>()?;
                        }
                        "tripped" => {
                            if seen_tripped {
                                return Err(serde::de::Error::duplicate_field("tripped"));
                            }
                            seen_tripped = true;
                            f_tripped = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Breaker {
                    estimated_size: f_estimated_size,
                    estimated_size_in_bytes: f_estimated_size_in_bytes,
                    limit_size: f_limit_size,
                    limit_size_in_bytes: f_limit_size_in_bytes,
                    overhead: f_overhead,
                    tripped: f_tripped,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Breaker {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.estimated_size {
            map.serialize_entry("estimated_size", v)?;
        }
        if let Some(v) = &self.estimated_size_in_bytes {
            map.serialize_entry("estimated_size_in_bytes", v)?;
        }
        if let Some(v) = &self.limit_size {
            map.serialize_entry("limit_size", v)?;
        }
        if let Some(v) = &self.limit_size_in_bytes {
            map.serialize_entry("limit_size_in_bytes", v)?;
        }
        if let Some(v) = &self.overhead {
            map.serialize_entry("overhead", v)?;
        }
        if let Some(v) = &self.tripped {
            map.serialize_entry("tripped", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct CacheIndicesStats {
    pub evictions: Option<i64>,
    pub hit_count: Option<i64>,
    pub item_count: Option<i64>,
    pub miss_count: Option<i64>,
    pub size: Option<crate::types::common::HumanReadableByteCount>,
    pub size_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CacheIndicesStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CacheIndicesStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CacheIndicesStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_evictions: Option<i64> = None;
                let mut seen_evictions = false;
                let mut f_hit_count: Option<i64> = None;
                let mut seen_hit_count = false;
                let mut f_item_count: Option<i64> = None;
                let mut seen_item_count = false;
                let mut f_miss_count: Option<i64> = None;
                let mut seen_miss_count = false;
                let mut f_size: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_size = false;
                let mut f_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_size_in_bytes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "evictions" => {
                            if seen_evictions {
                                return Err(serde::de::Error::duplicate_field("evictions"));
                            }
                            seen_evictions = true;
                            f_evictions = map.next_value::<Option<i64>>()?;
                        }
                        "hit_count" => {
                            if seen_hit_count {
                                return Err(serde::de::Error::duplicate_field("hit_count"));
                            }
                            seen_hit_count = true;
                            f_hit_count = map.next_value::<Option<i64>>()?;
                        }
                        "item_count" => {
                            if seen_item_count {
                                return Err(serde::de::Error::duplicate_field("item_count"));
                            }
                            seen_item_count = true;
                            f_item_count = map.next_value::<Option<i64>>()?;
                        }
                        "miss_count" => {
                            if seen_miss_count {
                                return Err(serde::de::Error::duplicate_field("miss_count"));
                            }
                            seen_miss_count = true;
                            f_miss_count = map.next_value::<Option<i64>>()?;
                        }
                        "size" => {
                            if seen_size {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            seen_size = true;
                            f_size = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
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
                Ok(CacheIndicesStats {
                    evictions: f_evictions,
                    hit_count: f_hit_count,
                    item_count: f_item_count,
                    miss_count: f_miss_count,
                    size: f_size,
                    size_in_bytes: f_size_in_bytes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CacheIndicesStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.evictions {
            map.serialize_entry("evictions", v)?;
        }
        if let Some(v) = &self.hit_count {
            map.serialize_entry("hit_count", v)?;
        }
        if let Some(v) = &self.item_count {
            map.serialize_entry("item_count", v)?;
        }
        if let Some(v) = &self.miss_count {
            map.serialize_entry("miss_count", v)?;
        }
        if let Some(v) = &self.size {
            map.serialize_entry("size", v)?;
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
pub struct CacheShardStats {
    pub evictions: Option<i64>,
    pub hit_count: Option<i64>,
    pub item_count: Option<i64>,
    pub miss_count: Option<i64>,
    pub size: Option<crate::types::common::HumanReadableByteCount>,
    pub size_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CacheShardStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CacheShardStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CacheShardStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_evictions: Option<i64> = None;
                let mut seen_evictions = false;
                let mut f_hit_count: Option<i64> = None;
                let mut seen_hit_count = false;
                let mut f_item_count: Option<i64> = None;
                let mut seen_item_count = false;
                let mut f_miss_count: Option<i64> = None;
                let mut seen_miss_count = false;
                let mut f_size: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_size = false;
                let mut f_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_size_in_bytes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "evictions" => {
                            if seen_evictions {
                                return Err(serde::de::Error::duplicate_field("evictions"));
                            }
                            seen_evictions = true;
                            f_evictions = map.next_value::<Option<i64>>()?;
                        }
                        "hit_count" => {
                            if seen_hit_count {
                                return Err(serde::de::Error::duplicate_field("hit_count"));
                            }
                            seen_hit_count = true;
                            f_hit_count = map.next_value::<Option<i64>>()?;
                        }
                        "item_count" => {
                            if seen_item_count {
                                return Err(serde::de::Error::duplicate_field("item_count"));
                            }
                            seen_item_count = true;
                            f_item_count = map.next_value::<Option<i64>>()?;
                        }
                        "miss_count" => {
                            if seen_miss_count {
                                return Err(serde::de::Error::duplicate_field("miss_count"));
                            }
                            seen_miss_count = true;
                            f_miss_count = map.next_value::<Option<i64>>()?;
                        }
                        "size" => {
                            if seen_size {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            seen_size = true;
                            f_size = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
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
                Ok(CacheShardStats {
                    evictions: f_evictions,
                    hit_count: f_hit_count,
                    item_count: f_item_count,
                    miss_count: f_miss_count,
                    size: f_size,
                    size_in_bytes: f_size_in_bytes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CacheShardStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.evictions {
            map.serialize_entry("evictions", v)?;
        }
        if let Some(v) = &self.hit_count {
            map.serialize_entry("hit_count", v)?;
        }
        if let Some(v) = &self.item_count {
            map.serialize_entry("item_count", v)?;
        }
        if let Some(v) = &self.miss_count {
            map.serialize_entry("miss_count", v)?;
        }
        if let Some(v) = &self.size {
            map.serialize_entry("size", v)?;
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
pub struct CacheStats {
    pub evictions: Option<i64>,
    pub hit_count: Option<i64>,
    pub item_count: Option<i64>,
    pub miss_count: Option<i64>,
    pub size: Option<crate::types::common::HumanReadableByteCount>,
    pub size_in_bytes: Option<crate::types::common::ByteCount>,
    pub indices:
        Option<std::collections::BTreeMap<String, crate::types::nodes_stats::CacheIndicesStats>>,
    pub shards:
        Option<std::collections::BTreeMap<String, crate::types::nodes_stats::CacheShardStats>>,
    pub store_name: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CacheStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CacheStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CacheStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_evictions: Option<i64> = None;
                let mut seen_evictions = false;
                let mut f_hit_count: Option<i64> = None;
                let mut seen_hit_count = false;
                let mut f_item_count: Option<i64> = None;
                let mut seen_item_count = false;
                let mut f_miss_count: Option<i64> = None;
                let mut seen_miss_count = false;
                let mut f_size: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_size = false;
                let mut f_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_size_in_bytes = false;
                let mut f_indices: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::nodes_stats::CacheIndicesStats,
                    >,
                > = None;
                let mut seen_indices = false;
                let mut f_shards: Option<
                    std::collections::BTreeMap<String, crate::types::nodes_stats::CacheShardStats>,
                > = None;
                let mut seen_shards = false;
                let mut f_store_name: Option<String> = None;
                let mut seen_store_name = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "evictions" => {
                            if seen_evictions {
                                return Err(serde::de::Error::duplicate_field("evictions"));
                            }
                            seen_evictions = true;
                            f_evictions = map.next_value::<Option<i64>>()?;
                        }
                        "hit_count" => {
                            if seen_hit_count {
                                return Err(serde::de::Error::duplicate_field("hit_count"));
                            }
                            seen_hit_count = true;
                            f_hit_count = map.next_value::<Option<i64>>()?;
                        }
                        "item_count" => {
                            if seen_item_count {
                                return Err(serde::de::Error::duplicate_field("item_count"));
                            }
                            seen_item_count = true;
                            f_item_count = map.next_value::<Option<i64>>()?;
                        }
                        "miss_count" => {
                            if seen_miss_count {
                                return Err(serde::de::Error::duplicate_field("miss_count"));
                            }
                            seen_miss_count = true;
                            f_miss_count = map.next_value::<Option<i64>>()?;
                        }
                        "size" => {
                            if seen_size {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            seen_size = true;
                            f_size = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "size_in_bytes" => {
                            if seen_size_in_bytes {
                                return Err(serde::de::Error::duplicate_field("size_in_bytes"));
                            }
                            seen_size_in_bytes = true;
                            f_size_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "indices" => {
                            if seen_indices {
                                return Err(serde::de::Error::duplicate_field("indices"));
                            }
                            seen_indices = true;
                            f_indices = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::nodes_stats::CacheIndicesStats,
                                >,
                            >>()?;
                        }
                        "shards" => {
                            if seen_shards {
                                return Err(serde::de::Error::duplicate_field("shards"));
                            }
                            seen_shards = true;
                            f_shards = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::nodes_stats::CacheShardStats,
                                >,
                            >>()?;
                        }
                        "store_name" => {
                            if seen_store_name {
                                return Err(serde::de::Error::duplicate_field("store_name"));
                            }
                            seen_store_name = true;
                            f_store_name = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CacheStats {
                    evictions: f_evictions,
                    hit_count: f_hit_count,
                    item_count: f_item_count,
                    miss_count: f_miss_count,
                    size: f_size,
                    size_in_bytes: f_size_in_bytes,
                    indices: f_indices,
                    shards: f_shards,
                    store_name: f_store_name,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CacheStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.evictions {
            map.serialize_entry("evictions", v)?;
        }
        if let Some(v) = &self.hit_count {
            map.serialize_entry("hit_count", v)?;
        }
        if let Some(v) = &self.item_count {
            map.serialize_entry("item_count", v)?;
        }
        if let Some(v) = &self.miss_count {
            map.serialize_entry("miss_count", v)?;
        }
        if let Some(v) = &self.size {
            map.serialize_entry("size", v)?;
        }
        if let Some(v) = &self.size_in_bytes {
            map.serialize_entry("size_in_bytes", v)?;
        }
        if let Some(v) = &self.indices {
            map.serialize_entry("indices", v)?;
        }
        if let Some(v) = &self.shards {
            map.serialize_entry("shards", v)?;
        }
        if let Some(v) = &self.store_name {
            map.serialize_entry("store_name", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type CachesStats = std::collections::BTreeMap<String, crate::types::nodes_stats::CacheStats>;

#[derive(Debug, Clone)]
pub struct CgroupCpuAcctStats {
    #[doc = "The `cpuacct` control group to which the OpenSearch process belongs."]
    pub control_group: Option<String>,
    pub usage_nanos: Option<crate::types::common::DurationValueUnitNanos>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CgroupCpuAcctStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CgroupCpuAcctStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CgroupCpuAcctStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_control_group: Option<String> = None;
                let mut seen_control_group = false;
                let mut f_usage_nanos: Option<crate::types::common::DurationValueUnitNanos> = None;
                let mut seen_usage_nanos = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "control_group" => {
                            if seen_control_group {
                                return Err(serde::de::Error::duplicate_field("control_group"));
                            }
                            seen_control_group = true;
                            f_control_group = map.next_value::<Option<String>>()?;
                        }
                        "usage_nanos" => {
                            if seen_usage_nanos {
                                return Err(serde::de::Error::duplicate_field("usage_nanos"));
                            }
                            seen_usage_nanos = true;
                            f_usage_nanos = map
                                .next_value::<Option<crate::types::common::DurationValueUnitNanos>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CgroupCpuAcctStats {
                    control_group: f_control_group,
                    usage_nanos: f_usage_nanos,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CgroupCpuAcctStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.control_group {
            map.serialize_entry("control_group", v)?;
        }
        if let Some(v) = &self.usage_nanos {
            map.serialize_entry("usage_nanos", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct CgroupCpuStat {
    #[doc = "The number of reporting periods (as specified by `cfs_period_micros`) that have elapsed."]
    pub number_of_elapsed_periods: Option<i64>,
    #[doc = "The number of times all tasks in the same cgroup as the OpenSearch process have been throttled."]
    pub number_of_times_throttled: Option<i64>,
    pub time_throttled_nanos: Option<crate::types::common::DurationValueUnitNanos>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CgroupCpuStat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CgroupCpuStat;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CgroupCpuStat object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_number_of_elapsed_periods: Option<i64> = None;
                let mut seen_number_of_elapsed_periods = false;
                let mut f_number_of_times_throttled: Option<i64> = None;
                let mut seen_number_of_times_throttled = false;
                let mut f_time_throttled_nanos: Option<
                    crate::types::common::DurationValueUnitNanos,
                > = None;
                let mut seen_time_throttled_nanos = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "number_of_elapsed_periods" => {
                            if seen_number_of_elapsed_periods {
                                return Err(serde::de::Error::duplicate_field(
                                    "number_of_elapsed_periods",
                                ));
                            }
                            seen_number_of_elapsed_periods = true;
                            f_number_of_elapsed_periods = map.next_value::<Option<i64>>()?;
                        }
                        "number_of_times_throttled" => {
                            if seen_number_of_times_throttled {
                                return Err(serde::de::Error::duplicate_field(
                                    "number_of_times_throttled",
                                ));
                            }
                            seen_number_of_times_throttled = true;
                            f_number_of_times_throttled = map.next_value::<Option<i64>>()?;
                        }
                        "time_throttled_nanos" => {
                            if seen_time_throttled_nanos {
                                return Err(serde::de::Error::duplicate_field(
                                    "time_throttled_nanos",
                                ));
                            }
                            seen_time_throttled_nanos = true;
                            f_time_throttled_nanos = map
                                .next_value::<Option<crate::types::common::DurationValueUnitNanos>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CgroupCpuStat {
                    number_of_elapsed_periods: f_number_of_elapsed_periods,
                    number_of_times_throttled: f_number_of_times_throttled,
                    time_throttled_nanos: f_time_throttled_nanos,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CgroupCpuStat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.number_of_elapsed_periods {
            map.serialize_entry("number_of_elapsed_periods", v)?;
        }
        if let Some(v) = &self.number_of_times_throttled {
            map.serialize_entry("number_of_times_throttled", v)?;
        }
        if let Some(v) = &self.time_throttled_nanos {
            map.serialize_entry("time_throttled_nanos", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct CgroupCpuStats {
    #[doc = "The period of time, in microseconds, for how regularly all tasks in the same cgroup as the OpenSearch process should have their access to CPU resources reallocated."]
    pub cfs_period_micros: Option<crate::types::common::DurationValueUnitMicros>,
    #[doc = "The total amount of time, in microseconds, for which all tasks in the same cgroup as the OpenSearch process can run during one period `cfs_period_micros`."]
    pub cfs_quota_micros: Option<crate::types::common::DurationValueUnitMicros>,
    #[doc = "The `cpu` control group to which the OpenSearch process belongs."]
    pub control_group: Option<String>,
    pub stat: Option<crate::types::nodes_stats::CgroupCpuStat>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CgroupCpuStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CgroupCpuStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CgroupCpuStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cfs_period_micros: Option<crate::types::common::DurationValueUnitMicros> =
                    None;
                let mut seen_cfs_period_micros = false;
                let mut f_cfs_quota_micros: Option<crate::types::common::DurationValueUnitMicros> =
                    None;
                let mut seen_cfs_quota_micros = false;
                let mut f_control_group: Option<String> = None;
                let mut seen_control_group = false;
                let mut f_stat: Option<crate::types::nodes_stats::CgroupCpuStat> = None;
                let mut seen_stat = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cfs_period_micros" => {
                            if seen_cfs_period_micros {
                                return Err(serde::de::Error::duplicate_field("cfs_period_micros"));
                            }
                            seen_cfs_period_micros = true;
                            f_cfs_period_micros = map.next_value::<Option<crate::types::common::DurationValueUnitMicros>>()?;
                        }
                        "cfs_quota_micros" => {
                            if seen_cfs_quota_micros {
                                return Err(serde::de::Error::duplicate_field("cfs_quota_micros"));
                            }
                            seen_cfs_quota_micros = true;
                            f_cfs_quota_micros = map.next_value::<Option<crate::types::common::DurationValueUnitMicros>>()?;
                        }
                        "control_group" => {
                            if seen_control_group {
                                return Err(serde::de::Error::duplicate_field("control_group"));
                            }
                            seen_control_group = true;
                            f_control_group = map.next_value::<Option<String>>()?;
                        }
                        "stat" => {
                            if seen_stat {
                                return Err(serde::de::Error::duplicate_field("stat"));
                            }
                            seen_stat = true;
                            f_stat = map
                                .next_value::<Option<crate::types::nodes_stats::CgroupCpuStat>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CgroupCpuStats {
                    cfs_period_micros: f_cfs_period_micros,
                    cfs_quota_micros: f_cfs_quota_micros,
                    control_group: f_control_group,
                    stat: f_stat,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CgroupCpuStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cfs_period_micros {
            map.serialize_entry("cfs_period_micros", v)?;
        }
        if let Some(v) = &self.cfs_quota_micros {
            map.serialize_entry("cfs_quota_micros", v)?;
        }
        if let Some(v) = &self.control_group {
            map.serialize_entry("control_group", v)?;
        }
        if let Some(v) = &self.stat {
            map.serialize_entry("stat", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct CgroupMemoryStats {
    #[doc = "The `memory` control group to which the OpenSearch process belongs."]
    pub control_group: Option<String>,
    #[doc = "The maximum amount of user memory (including file cache) allowed for all tasks in the same cgroup as the OpenSearch process. This value can be too big to store in a `long`, so is returned as a string so that the value returned can exactly match what the underlying operating system interface returns. Any value that is too large to parse into a `long` almost certainly means no limit has been set for the cgroup."]
    pub limit_in_bytes: Option<String>,
    #[doc = "The total current memory usage by processes in the cgroup, in bytes, by all tasks in the same cgroup as the OpenSearch process. This value is stored as a string for consistency with `limit_in_bytes`."]
    pub usage_in_bytes: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CgroupMemoryStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CgroupMemoryStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CgroupMemoryStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_control_group: Option<String> = None;
                let mut seen_control_group = false;
                let mut f_limit_in_bytes: Option<String> = None;
                let mut seen_limit_in_bytes = false;
                let mut f_usage_in_bytes: Option<String> = None;
                let mut seen_usage_in_bytes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "control_group" => {
                            if seen_control_group {
                                return Err(serde::de::Error::duplicate_field("control_group"));
                            }
                            seen_control_group = true;
                            f_control_group = map.next_value::<Option<String>>()?;
                        }
                        "limit_in_bytes" => {
                            if seen_limit_in_bytes {
                                return Err(serde::de::Error::duplicate_field("limit_in_bytes"));
                            }
                            seen_limit_in_bytes = true;
                            f_limit_in_bytes = map.next_value::<Option<String>>()?;
                        }
                        "usage_in_bytes" => {
                            if seen_usage_in_bytes {
                                return Err(serde::de::Error::duplicate_field("usage_in_bytes"));
                            }
                            seen_usage_in_bytes = true;
                            f_usage_in_bytes = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CgroupMemoryStats {
                    control_group: f_control_group,
                    limit_in_bytes: f_limit_in_bytes,
                    usage_in_bytes: f_usage_in_bytes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CgroupMemoryStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.control_group {
            map.serialize_entry("control_group", v)?;
        }
        if let Some(v) = &self.limit_in_bytes {
            map.serialize_entry("limit_in_bytes", v)?;
        }
        if let Some(v) = &self.usage_in_bytes {
            map.serialize_entry("usage_in_bytes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct CgroupStats {
    pub cpu: Option<crate::types::nodes_stats::CgroupCpuStats>,
    pub cpuacct: Option<crate::types::nodes_stats::CgroupCpuAcctStats>,
    pub memory: Option<crate::types::nodes_stats::CgroupMemoryStats>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CgroupStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CgroupStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CgroupStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cpu: Option<crate::types::nodes_stats::CgroupCpuStats> = None;
                let mut seen_cpu = false;
                let mut f_cpuacct: Option<crate::types::nodes_stats::CgroupCpuAcctStats> = None;
                let mut seen_cpuacct = false;
                let mut f_memory: Option<crate::types::nodes_stats::CgroupMemoryStats> = None;
                let mut seen_memory = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cpu" => {
                            if seen_cpu {
                                return Err(serde::de::Error::duplicate_field("cpu"));
                            }
                            seen_cpu = true;
                            f_cpu = map
                                .next_value::<Option<crate::types::nodes_stats::CgroupCpuStats>>(
                                )?;
                        }
                        "cpuacct" => {
                            if seen_cpuacct {
                                return Err(serde::de::Error::duplicate_field("cpuacct"));
                            }
                            seen_cpuacct = true;
                            f_cpuacct = map.next_value::<Option<crate::types::nodes_stats::CgroupCpuAcctStats>>()?;
                        }
                        "memory" => {
                            if seen_memory {
                                return Err(serde::de::Error::duplicate_field("memory"));
                            }
                            seen_memory = true;
                            f_memory = map
                                .next_value::<Option<crate::types::nodes_stats::CgroupMemoryStats>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CgroupStats {
                    cpu: f_cpu,
                    cpuacct: f_cpuacct,
                    memory: f_memory,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CgroupStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cpu {
            map.serialize_entry("cpu", v)?;
        }
        if let Some(v) = &self.cpuacct {
            map.serialize_entry("cpuacct", v)?;
        }
        if let Some(v) = &self.memory {
            map.serialize_entry("memory", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ClusterStateOverallStats {
    pub failed_count: Option<i64>,
    pub total_time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    pub update_count: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterStateOverallStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterStateOverallStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterStateOverallStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_failed_count: Option<i64> = None;
                let mut seen_failed_count = false;
                let mut f_total_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut seen_total_time_in_millis = false;
                let mut f_update_count: Option<i64> = None;
                let mut seen_update_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "failed_count" => {
                            if seen_failed_count {
                                return Err(serde::de::Error::duplicate_field("failed_count"));
                            }
                            seen_failed_count = true;
                            f_failed_count = map.next_value::<Option<i64>>()?;
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
                        "update_count" => {
                            if seen_update_count {
                                return Err(serde::de::Error::duplicate_field("update_count"));
                            }
                            seen_update_count = true;
                            f_update_count = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterStateOverallStats {
                    failed_count: f_failed_count,
                    total_time_in_millis: f_total_time_in_millis,
                    update_count: f_update_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterStateOverallStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.failed_count {
            map.serialize_entry("failed_count", v)?;
        }
        if let Some(v) = &self.total_time_in_millis {
            map.serialize_entry("total_time_in_millis", v)?;
        }
        if let Some(v) = &self.update_count {
            map.serialize_entry("update_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ClusterStateQueue {
    #[doc = "Number of committed cluster states in queue."]
    pub committed: Option<i32>,
    #[doc = "Number of pending cluster states in queue."]
    pub pending: Option<i32>,
    #[doc = "Total number of cluster states in queue."]
    pub total: Option<i32>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterStateQueue {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterStateQueue;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterStateQueue object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_committed: Option<i32> = None;
                let mut seen_committed = false;
                let mut f_pending: Option<i32> = None;
                let mut seen_pending = false;
                let mut f_total: Option<i32> = None;
                let mut seen_total = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "committed" => {
                            if seen_committed {
                                return Err(serde::de::Error::duplicate_field("committed"));
                            }
                            seen_committed = true;
                            f_committed = map.next_value::<Option<i32>>()?;
                        }
                        "pending" => {
                            if seen_pending {
                                return Err(serde::de::Error::duplicate_field("pending"));
                            }
                            seen_pending = true;
                            f_pending = map.next_value::<Option<i32>>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map.next_value::<Option<i32>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterStateQueue {
                    committed: f_committed,
                    pending: f_pending,
                    total: f_total,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterStateQueue {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.committed {
            map.serialize_entry("committed", v)?;
        }
        if let Some(v) = &self.pending {
            map.serialize_entry("pending", v)?;
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
pub struct ClusterStateStats {
    pub overall: Option<crate::types::nodes_stats::ClusterStateOverallStats>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterStateStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterStateStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterStateStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_overall: Option<crate::types::nodes_stats::ClusterStateOverallStats> =
                    None;
                let mut seen_overall = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "overall" => {
                            if seen_overall {
                                return Err(serde::de::Error::duplicate_field("overall"));
                            }
                            seen_overall = true;
                            f_overall = map.next_value::<Option<crate::types::nodes_stats::ClusterStateOverallStats>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterStateStats {
                    overall: f_overall,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterStateStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.overall {
            map.serialize_entry("overall", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct DataPathStats {
    #[doc = "Total amount of disk space available to this Java virtual machine on this file store."]
    pub available: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total number of bytes available to this Java virtual machine on this file store."]
    pub available_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Total number of cache bytes reserved available to this Java virtual machine on this file store."]
    pub cache_reserved: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total number of cache bytes reserved available to this Java virtual machine on this file store."]
    pub cache_reserved_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Total amount of unallocated disk space in the file store."]
    pub free: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total number of unallocated bytes in the file store."]
    pub free_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Mount point of the file store (for example: `/dev/sda2`)."]
    pub mount: Option<String>,
    #[doc = "Path to the file store."]
    pub path: Option<String>,
    #[doc = "Total size of the file store."]
    pub total: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total size of the file store in bytes."]
    pub total_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Type of the file store (ex: ext4)."]
    pub r#type: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DataPathStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DataPathStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DataPathStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_available: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_available = false;
                let mut f_available_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_available_in_bytes = false;
                let mut f_cache_reserved: Option<crate::types::common::HumanReadableByteCount> =
                    None;
                let mut seen_cache_reserved = false;
                let mut f_cache_reserved_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_cache_reserved_in_bytes = false;
                let mut f_free: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_free = false;
                let mut f_free_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_free_in_bytes = false;
                let mut f_mount: Option<String> = None;
                let mut seen_mount = false;
                let mut f_path: Option<String> = None;
                let mut seen_path = false;
                let mut f_total: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_total = false;
                let mut f_total_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_total_in_bytes = false;
                let mut f_type: Option<String> = None;
                let mut seen_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "available" => {
                            if seen_available {
                                return Err(serde::de::Error::duplicate_field("available"));
                            }
                            seen_available = true;
                            f_available = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "available_in_bytes" => {
                            if seen_available_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "available_in_bytes",
                                ));
                            }
                            seen_available_in_bytes = true;
                            f_available_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "cache_reserved" => {
                            if seen_cache_reserved {
                                return Err(serde::de::Error::duplicate_field("cache_reserved"));
                            }
                            seen_cache_reserved = true;
                            f_cache_reserved = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "cache_reserved_in_bytes" => {
                            if seen_cache_reserved_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "cache_reserved_in_bytes",
                                ));
                            }
                            seen_cache_reserved_in_bytes = true;
                            f_cache_reserved_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "free" => {
                            if seen_free {
                                return Err(serde::de::Error::duplicate_field("free"));
                            }
                            seen_free = true;
                            f_free = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "free_in_bytes" => {
                            if seen_free_in_bytes {
                                return Err(serde::de::Error::duplicate_field("free_in_bytes"));
                            }
                            seen_free_in_bytes = true;
                            f_free_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "mount" => {
                            if seen_mount {
                                return Err(serde::de::Error::duplicate_field("mount"));
                            }
                            seen_mount = true;
                            f_mount = map.next_value::<Option<String>>()?;
                        }
                        "path" => {
                            if seen_path {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            seen_path = true;
                            f_path = map.next_value::<Option<String>>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "total_in_bytes" => {
                            if seen_total_in_bytes {
                                return Err(serde::de::Error::duplicate_field("total_in_bytes"));
                            }
                            seen_total_in_bytes = true;
                            f_total_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
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
                Ok(DataPathStats {
                    available: f_available,
                    available_in_bytes: f_available_in_bytes,
                    cache_reserved: f_cache_reserved,
                    cache_reserved_in_bytes: f_cache_reserved_in_bytes,
                    free: f_free,
                    free_in_bytes: f_free_in_bytes,
                    mount: f_mount,
                    path: f_path,
                    total: f_total,
                    total_in_bytes: f_total_in_bytes,
                    r#type: f_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DataPathStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.available {
            map.serialize_entry("available", v)?;
        }
        if let Some(v) = &self.available_in_bytes {
            map.serialize_entry("available_in_bytes", v)?;
        }
        if let Some(v) = &self.cache_reserved {
            map.serialize_entry("cache_reserved", v)?;
        }
        if let Some(v) = &self.cache_reserved_in_bytes {
            map.serialize_entry("cache_reserved_in_bytes", v)?;
        }
        if let Some(v) = &self.free {
            map.serialize_entry("free", v)?;
        }
        if let Some(v) = &self.free_in_bytes {
            map.serialize_entry("free_in_bytes", v)?;
        }
        if let Some(v) = &self.mount {
            map.serialize_entry("mount", v)?;
        }
        if let Some(v) = &self.path {
            map.serialize_entry("path", v)?;
        }
        if let Some(v) = &self.total {
            map.serialize_entry("total", v)?;
        }
        if let Some(v) = &self.total_in_bytes {
            map.serialize_entry("total_in_bytes", v)?;
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

#[derive(Debug, Clone)]
pub struct Discovery {
    pub cluster_state_queue: Option<crate::types::nodes_stats::ClusterStateQueue>,
    pub cluster_state_stats: Option<crate::types::nodes_stats::ClusterStateStats>,
    pub published_cluster_states: Option<crate::types::nodes_stats::PublishedClusterStates>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Discovery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Discovery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Discovery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cluster_state_queue: Option<
                    crate::types::nodes_stats::ClusterStateQueue,
                > = None;
                let mut seen_cluster_state_queue = false;
                let mut f_cluster_state_stats: Option<
                    crate::types::nodes_stats::ClusterStateStats,
                > = None;
                let mut seen_cluster_state_stats = false;
                let mut f_published_cluster_states: Option<
                    crate::types::nodes_stats::PublishedClusterStates,
                > = None;
                let mut seen_published_cluster_states = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cluster_state_queue" => {
                            if seen_cluster_state_queue {
                                return Err(serde::de::Error::duplicate_field(
                                    "cluster_state_queue",
                                ));
                            }
                            seen_cluster_state_queue = true;
                            f_cluster_state_queue = map
                                .next_value::<Option<crate::types::nodes_stats::ClusterStateQueue>>(
                                )?;
                        }
                        "cluster_state_stats" => {
                            if seen_cluster_state_stats {
                                return Err(serde::de::Error::duplicate_field(
                                    "cluster_state_stats",
                                ));
                            }
                            seen_cluster_state_stats = true;
                            f_cluster_state_stats = map
                                .next_value::<Option<crate::types::nodes_stats::ClusterStateStats>>(
                                )?;
                        }
                        "published_cluster_states" => {
                            if seen_published_cluster_states {
                                return Err(serde::de::Error::duplicate_field(
                                    "published_cluster_states",
                                ));
                            }
                            seen_published_cluster_states = true;
                            f_published_cluster_states = map.next_value::<Option<crate::types::nodes_stats::PublishedClusterStates>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Discovery {
                    cluster_state_queue: f_cluster_state_queue,
                    cluster_state_stats: f_cluster_state_stats,
                    published_cluster_states: f_published_cluster_states,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Discovery {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cluster_state_queue {
            map.serialize_entry("cluster_state_queue", v)?;
        }
        if let Some(v) = &self.cluster_state_stats {
            map.serialize_entry("cluster_state_stats", v)?;
        }
        if let Some(v) = &self.published_cluster_states {
            map.serialize_entry("published_cluster_states", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ExtendedMemoryStats {
    #[doc = "Amount of free physical memory."]
    pub free: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Amount of free physical memory in bytes."]
    pub free_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Total amount of physical memory."]
    pub total: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total amount of physical memory in bytes."]
    pub total_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Amount of used physical memory."]
    pub used: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Amount of used physical memory in bytes."]
    pub used_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Percentage of free memory."]
    pub free_percent: Option<crate::types::common::PercentageNumber>,
    #[doc = "Percentage of used memory."]
    pub used_percent: Option<crate::types::common::PercentageNumber>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ExtendedMemoryStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ExtendedMemoryStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ExtendedMemoryStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_free: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_free = false;
                let mut f_free_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_free_in_bytes = false;
                let mut f_total: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_total = false;
                let mut f_total_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_total_in_bytes = false;
                let mut f_used: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_used = false;
                let mut f_used_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_used_in_bytes = false;
                let mut f_free_percent: Option<crate::types::common::PercentageNumber> = None;
                let mut seen_free_percent = false;
                let mut f_used_percent: Option<crate::types::common::PercentageNumber> = None;
                let mut seen_used_percent = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "free" => {
                            if seen_free {
                                return Err(serde::de::Error::duplicate_field("free"));
                            }
                            seen_free = true;
                            f_free = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "free_in_bytes" => {
                            if seen_free_in_bytes {
                                return Err(serde::de::Error::duplicate_field("free_in_bytes"));
                            }
                            seen_free_in_bytes = true;
                            f_free_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "total_in_bytes" => {
                            if seen_total_in_bytes {
                                return Err(serde::de::Error::duplicate_field("total_in_bytes"));
                            }
                            seen_total_in_bytes = true;
                            f_total_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "used" => {
                            if seen_used {
                                return Err(serde::de::Error::duplicate_field("used"));
                            }
                            seen_used = true;
                            f_used = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "used_in_bytes" => {
                            if seen_used_in_bytes {
                                return Err(serde::de::Error::duplicate_field("used_in_bytes"));
                            }
                            seen_used_in_bytes = true;
                            f_used_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "free_percent" => {
                            if seen_free_percent {
                                return Err(serde::de::Error::duplicate_field("free_percent"));
                            }
                            seen_free_percent = true;
                            f_free_percent =
                                map.next_value::<Option<crate::types::common::PercentageNumber>>()?;
                        }
                        "used_percent" => {
                            if seen_used_percent {
                                return Err(serde::de::Error::duplicate_field("used_percent"));
                            }
                            seen_used_percent = true;
                            f_used_percent =
                                map.next_value::<Option<crate::types::common::PercentageNumber>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ExtendedMemoryStats {
                    free: f_free,
                    free_in_bytes: f_free_in_bytes,
                    total: f_total,
                    total_in_bytes: f_total_in_bytes,
                    used: f_used,
                    used_in_bytes: f_used_in_bytes,
                    free_percent: f_free_percent,
                    used_percent: f_used_percent,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ExtendedMemoryStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.free {
            map.serialize_entry("free", v)?;
        }
        if let Some(v) = &self.free_in_bytes {
            map.serialize_entry("free_in_bytes", v)?;
        }
        if let Some(v) = &self.total {
            map.serialize_entry("total", v)?;
        }
        if let Some(v) = &self.total_in_bytes {
            map.serialize_entry("total_in_bytes", v)?;
        }
        if let Some(v) = &self.used {
            map.serialize_entry("used", v)?;
        }
        if let Some(v) = &self.used_in_bytes {
            map.serialize_entry("used_in_bytes", v)?;
        }
        if let Some(v) = &self.free_percent {
            map.serialize_entry("free_percent", v)?;
        }
        if let Some(v) = &self.used_percent {
            map.serialize_entry("used_percent", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct FileSystem {
    #[doc = "List of all file stores."]
    pub data: Option<Vec<crate::types::nodes_stats::DataPathStats>>,
    pub io_stats: Option<crate::types::nodes_stats::IoStats>,
    #[doc = "Last time the file stores statistics were refreshed. Recorded in milliseconds since the Unix Epoch."]
    pub timestamp: Option<crate::types::common::EpochTimeUnitMillis>,
    pub total: Option<crate::types::nodes_stats::FileSystemTotal>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FileSystem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FileSystem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FileSystem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_data: Option<Vec<crate::types::nodes_stats::DataPathStats>> = None;
                let mut seen_data = false;
                let mut f_io_stats: Option<crate::types::nodes_stats::IoStats> = None;
                let mut seen_io_stats = false;
                let mut f_timestamp: Option<crate::types::common::EpochTimeUnitMillis> = None;
                let mut seen_timestamp = false;
                let mut f_total: Option<crate::types::nodes_stats::FileSystemTotal> = None;
                let mut seen_total = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "data" => {
                            if seen_data {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            seen_data = true;
                            f_data = map.next_value::<Option<Vec<crate::types::nodes_stats::DataPathStats>>>()?;
                        }
                        "io_stats" => {
                            if seen_io_stats {
                                return Err(serde::de::Error::duplicate_field("io_stats"));
                            }
                            seen_io_stats = true;
                            f_io_stats =
                                map.next_value::<Option<crate::types::nodes_stats::IoStats>>()?;
                        }
                        "timestamp" => {
                            if seen_timestamp {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            seen_timestamp = true;
                            f_timestamp = map
                                .next_value::<Option<crate::types::common::EpochTimeUnitMillis>>(
                                )?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map
                                .next_value::<Option<crate::types::nodes_stats::FileSystemTotal>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FileSystem {
                    data: f_data,
                    io_stats: f_io_stats,
                    timestamp: f_timestamp,
                    total: f_total,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FileSystem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.data {
            map.serialize_entry("data", v)?;
        }
        if let Some(v) = &self.io_stats {
            map.serialize_entry("io_stats", v)?;
        }
        if let Some(v) = &self.timestamp {
            map.serialize_entry("timestamp", v)?;
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
pub struct FileSystemTotal {
    #[doc = "Total disk space available to this Java virtual machine on all file stores. Depending on OS or process level restrictions, this might appear less than `free`. This is the actual amount of free disk space the OpenSearch node can utilize."]
    pub available: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total number of bytes available to this Java virtual machine on all file stores. Depending on OS or process level restrictions, this might appear less than `free_in_bytes`. This is the actual amount of free disk space the OpenSearch node can utilize."]
    pub available_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Total size of cache bytes reserved in all file stores."]
    pub cache_reserved: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total size of cache bytes reserved in all file stores."]
    pub cache_reserved_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Total unallocated disk space in all file stores."]
    pub free: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total number of unallocated bytes in all file stores."]
    pub free_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Total size of all file stores."]
    pub total: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total size of all file stores in bytes."]
    pub total_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FileSystemTotal {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FileSystemTotal;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FileSystemTotal object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_available: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_available = false;
                let mut f_available_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_available_in_bytes = false;
                let mut f_cache_reserved: Option<crate::types::common::HumanReadableByteCount> =
                    None;
                let mut seen_cache_reserved = false;
                let mut f_cache_reserved_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_cache_reserved_in_bytes = false;
                let mut f_free: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_free = false;
                let mut f_free_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_free_in_bytes = false;
                let mut f_total: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_total = false;
                let mut f_total_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_total_in_bytes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "available" => {
                            if seen_available {
                                return Err(serde::de::Error::duplicate_field("available"));
                            }
                            seen_available = true;
                            f_available = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "available_in_bytes" => {
                            if seen_available_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "available_in_bytes",
                                ));
                            }
                            seen_available_in_bytes = true;
                            f_available_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "cache_reserved" => {
                            if seen_cache_reserved {
                                return Err(serde::de::Error::duplicate_field("cache_reserved"));
                            }
                            seen_cache_reserved = true;
                            f_cache_reserved = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "cache_reserved_in_bytes" => {
                            if seen_cache_reserved_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "cache_reserved_in_bytes",
                                ));
                            }
                            seen_cache_reserved_in_bytes = true;
                            f_cache_reserved_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "free" => {
                            if seen_free {
                                return Err(serde::de::Error::duplicate_field("free"));
                            }
                            seen_free = true;
                            f_free = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "free_in_bytes" => {
                            if seen_free_in_bytes {
                                return Err(serde::de::Error::duplicate_field("free_in_bytes"));
                            }
                            seen_free_in_bytes = true;
                            f_free_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "total_in_bytes" => {
                            if seen_total_in_bytes {
                                return Err(serde::de::Error::duplicate_field("total_in_bytes"));
                            }
                            seen_total_in_bytes = true;
                            f_total_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FileSystemTotal {
                    available: f_available,
                    available_in_bytes: f_available_in_bytes,
                    cache_reserved: f_cache_reserved,
                    cache_reserved_in_bytes: f_cache_reserved_in_bytes,
                    free: f_free,
                    free_in_bytes: f_free_in_bytes,
                    total: f_total,
                    total_in_bytes: f_total_in_bytes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FileSystemTotal {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.available {
            map.serialize_entry("available", v)?;
        }
        if let Some(v) = &self.available_in_bytes {
            map.serialize_entry("available_in_bytes", v)?;
        }
        if let Some(v) = &self.cache_reserved {
            map.serialize_entry("cache_reserved", v)?;
        }
        if let Some(v) = &self.cache_reserved_in_bytes {
            map.serialize_entry("cache_reserved_in_bytes", v)?;
        }
        if let Some(v) = &self.free {
            map.serialize_entry("free", v)?;
        }
        if let Some(v) = &self.free_in_bytes {
            map.serialize_entry("free_in_bytes", v)?;
        }
        if let Some(v) = &self.total {
            map.serialize_entry("total", v)?;
        }
        if let Some(v) = &self.total_in_bytes {
            map.serialize_entry("total_in_bytes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct GarbageCollector {
    #[doc = "Contains statistics about JVM garbage collectors for the node."]
    pub collectors: Option<
        std::collections::BTreeMap<String, crate::types::nodes_stats::GarbageCollectorTotal>,
    >,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GarbageCollector {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GarbageCollector;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GarbageCollector object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_collectors: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::nodes_stats::GarbageCollectorTotal,
                    >,
                > = None;
                let mut seen_collectors = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "collectors" => {
                            if seen_collectors {
                                return Err(serde::de::Error::duplicate_field("collectors"));
                            }
                            seen_collectors = true;
                            f_collectors = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::nodes_stats::GarbageCollectorTotal,
                                >,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GarbageCollector {
                    collectors: f_collectors,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GarbageCollector {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.collectors {
            map.serialize_entry("collectors", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct GarbageCollectorTotal {
    #[doc = "Total number of JVM garbage collectors that collect objects."]
    pub collection_count: Option<i64>,
    #[doc = "Total time spent by JVM collecting objects."]
    pub collection_time: Option<crate::types::common::Duration>,
    #[doc = "Total time, in milliseconds, spent by JVM collecting objects."]
    pub collection_time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GarbageCollectorTotal {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GarbageCollectorTotal;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GarbageCollectorTotal object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_collection_count: Option<i64> = None;
                let mut seen_collection_count = false;
                let mut f_collection_time: Option<crate::types::common::Duration> = None;
                let mut seen_collection_time = false;
                let mut f_collection_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut seen_collection_time_in_millis = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "collection_count" => {
                            if seen_collection_count {
                                return Err(serde::de::Error::duplicate_field("collection_count"));
                            }
                            seen_collection_count = true;
                            f_collection_count = map.next_value::<Option<i64>>()?;
                        }
                        "collection_time" => {
                            if seen_collection_time {
                                return Err(serde::de::Error::duplicate_field("collection_time"));
                            }
                            seen_collection_time = true;
                            f_collection_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "collection_time_in_millis" => {
                            if seen_collection_time_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "collection_time_in_millis",
                                ));
                            }
                            seen_collection_time_in_millis = true;
                            f_collection_time_in_millis = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GarbageCollectorTotal {
                    collection_count: f_collection_count,
                    collection_time: f_collection_time,
                    collection_time_in_millis: f_collection_time_in_millis,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GarbageCollectorTotal {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.collection_count {
            map.serialize_entry("collection_count", v)?;
        }
        if let Some(v) = &self.collection_time {
            map.serialize_entry("collection_time", v)?;
        }
        if let Some(v) = &self.collection_time_in_millis {
            map.serialize_entry("collection_time_in_millis", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Http {
    #[doc = "Current number of open HTTP connections for the node."]
    pub current_open: Option<i64>,
    #[doc = "Total number of HTTP connections opened for the node."]
    pub total_opened: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Http {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Http;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Http object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_current_open: Option<i64> = None;
                let mut seen_current_open = false;
                let mut f_total_opened: Option<i64> = None;
                let mut seen_total_opened = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "current_open" => {
                            if seen_current_open {
                                return Err(serde::de::Error::duplicate_field("current_open"));
                            }
                            seen_current_open = true;
                            f_current_open = map.next_value::<Option<i64>>()?;
                        }
                        "total_opened" => {
                            if seen_total_opened {
                                return Err(serde::de::Error::duplicate_field("total_opened"));
                            }
                            seen_total_opened = true;
                            f_total_opened = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Http {
                    current_open: f_current_open,
                    total_opened: f_total_opened,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Http {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.current_open {
            map.serialize_entry("current_open", v)?;
        }
        if let Some(v) = &self.total_opened {
            map.serialize_entry("total_opened", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IndexingPressure {
    pub memory: Option<crate::types::nodes_stats::IndexingPressureMemory>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexingPressure {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexingPressure;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexingPressure object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_memory: Option<crate::types::nodes_stats::IndexingPressureMemory> = None;
                let mut seen_memory = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "memory" => {
                            if seen_memory {
                                return Err(serde::de::Error::duplicate_field("memory"));
                            }
                            seen_memory = true;
                            f_memory = map.next_value::<Option<crate::types::nodes_stats::IndexingPressureMemory>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexingPressure {
                    memory: f_memory,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexingPressure {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.memory {
            map.serialize_entry("memory", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IndexingPressureMemory {
    pub current: Option<crate::types::nodes_stats::PressureMemory>,
    pub limit: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Configured memory limit, in bytes, for the indexing requests. Replica requests have an automatic limit that is 1.5x this value."]
    pub limit_in_bytes: Option<crate::types::common::ByteCount>,
    pub total: Option<crate::types::nodes_stats::PressureMemory>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexingPressureMemory {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexingPressureMemory;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexingPressureMemory object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_current: Option<crate::types::nodes_stats::PressureMemory> = None;
                let mut seen_current = false;
                let mut f_limit: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_limit = false;
                let mut f_limit_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_limit_in_bytes = false;
                let mut f_total: Option<crate::types::nodes_stats::PressureMemory> = None;
                let mut seen_total = false;
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
                            f_current = map
                                .next_value::<Option<crate::types::nodes_stats::PressureMemory>>(
                                )?;
                        }
                        "limit" => {
                            if seen_limit {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            seen_limit = true;
                            f_limit = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "limit_in_bytes" => {
                            if seen_limit_in_bytes {
                                return Err(serde::de::Error::duplicate_field("limit_in_bytes"));
                            }
                            seen_limit_in_bytes = true;
                            f_limit_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map
                                .next_value::<Option<crate::types::nodes_stats::PressureMemory>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexingPressureMemory {
                    current: f_current,
                    limit: f_limit,
                    limit_in_bytes: f_limit_in_bytes,
                    total: f_total,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexingPressureMemory {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.current {
            map.serialize_entry("current", v)?;
        }
        if let Some(v) = &self.limit {
            map.serialize_entry("limit", v)?;
        }
        if let Some(v) = &self.limit_in_bytes {
            map.serialize_entry("limit_in_bytes", v)?;
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
pub struct Ingest {
    #[doc = "Contains statistics about ingest pipelines for the node."]
    pub pipelines:
        Option<std::collections::BTreeMap<String, crate::types::nodes_stats::IngestTotal>>,
    pub total: Option<crate::types::nodes_stats::IngestTotal>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Ingest {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Ingest;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Ingest object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_pipelines: Option<
                    std::collections::BTreeMap<String, crate::types::nodes_stats::IngestTotal>,
                > = None;
                let mut seen_pipelines = false;
                let mut f_total: Option<crate::types::nodes_stats::IngestTotal> = None;
                let mut seen_total = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "pipelines" => {
                            if seen_pipelines {
                                return Err(serde::de::Error::duplicate_field("pipelines"));
                            }
                            seen_pipelines = true;
                            f_pipelines = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::nodes_stats::IngestTotal,
                                >,
                            >>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total =
                                map.next_value::<Option<crate::types::nodes_stats::IngestTotal>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Ingest {
                    pipelines: f_pipelines,
                    total: f_total,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Ingest {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.pipelines {
            map.serialize_entry("pipelines", v)?;
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
pub struct IngestTotal {
    #[doc = "Total number of documents ingested during the lifetime of this node."]
    pub count: Option<i64>,
    #[doc = "Total number of documents currently being ingested."]
    pub current: Option<i64>,
    #[doc = "Total number of failed ingest operations during the lifetime of this node."]
    pub failed: Option<i64>,
    #[doc = "Total number of ingest processors."]
    pub processors:
        Option<Vec<std::collections::BTreeMap<String, crate::types::nodes_stats::KeyedProcessor>>>,
    pub time: Option<crate::types::common::Duration>,
    pub time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IngestTotal {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IngestTotal;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IngestTotal object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_count: Option<i64> = None;
                let mut seen_count = false;
                let mut f_current: Option<i64> = None;
                let mut seen_current = false;
                let mut f_failed: Option<i64> = None;
                let mut seen_failed = false;
                let mut f_processors: Option<
                    Vec<
                        std::collections::BTreeMap<
                            String,
                            crate::types::nodes_stats::KeyedProcessor,
                        >,
                    >,
                > = None;
                let mut seen_processors = false;
                let mut f_time: Option<crate::types::common::Duration> = None;
                let mut seen_time = false;
                let mut f_time_in_millis: Option<crate::types::common::DurationValueUnitMillis> =
                    None;
                let mut seen_time_in_millis = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "count" => {
                            if seen_count {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            seen_count = true;
                            f_count = map.next_value::<Option<i64>>()?;
                        }
                        "current" => {
                            if seen_current {
                                return Err(serde::de::Error::duplicate_field("current"));
                            }
                            seen_current = true;
                            f_current = map.next_value::<Option<i64>>()?;
                        }
                        "failed" => {
                            if seen_failed {
                                return Err(serde::de::Error::duplicate_field("failed"));
                            }
                            seen_failed = true;
                            f_failed = map.next_value::<Option<i64>>()?;
                        }
                        "processors" => {
                            if seen_processors {
                                return Err(serde::de::Error::duplicate_field("processors"));
                            }
                            seen_processors = true;
                            f_processors = map.next_value::<Option<
                                Vec<
                                    std::collections::BTreeMap<
                                        String,
                                        crate::types::nodes_stats::KeyedProcessor,
                                    >,
                                >,
                            >>()?;
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
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IngestTotal {
                    count: f_count,
                    current: f_current,
                    failed: f_failed,
                    processors: f_processors,
                    time: f_time,
                    time_in_millis: f_time_in_millis,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IngestTotal {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.count {
            map.serialize_entry("count", v)?;
        }
        if let Some(v) = &self.current {
            map.serialize_entry("current", v)?;
        }
        if let Some(v) = &self.failed {
            map.serialize_entry("failed", v)?;
        }
        if let Some(v) = &self.processors {
            map.serialize_entry("processors", v)?;
        }
        if let Some(v) = &self.time {
            map.serialize_entry("time", v)?;
        }
        if let Some(v) = &self.time_in_millis {
            map.serialize_entry("time_in_millis", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IoStatDevice {
    #[doc = "The Linux device name."]
    pub device_name: Option<String>,
    pub io_time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "The total number of read and write operations for the device completed since starting OpenSearch."]
    pub operations: Option<i64>,
    pub queue_size: Option<i64>,
    #[doc = "The total number of kilobytes read for the device since starting OpenSearch."]
    pub read_kilobytes: Option<i64>,
    #[doc = "The total number of read operations for the device completed since starting OpenSearch."]
    pub read_operations: Option<i64>,
    pub read_time: Option<i64>,
    #[doc = "The total number of kilobytes written for the device since starting OpenSearch."]
    pub write_kilobytes: Option<i64>,
    #[doc = "The total number of write operations for the device completed since starting OpenSearch."]
    pub write_operations: Option<i64>,
    pub write_time: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IoStatDevice {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IoStatDevice;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IoStatDevice object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_device_name: Option<String> = None;
                let mut seen_device_name = false;
                let mut f_io_time_in_millis: Option<crate::types::common::DurationValueUnitMillis> =
                    None;
                let mut seen_io_time_in_millis = false;
                let mut f_operations: Option<i64> = None;
                let mut seen_operations = false;
                let mut f_queue_size: Option<i64> = None;
                let mut seen_queue_size = false;
                let mut f_read_kilobytes: Option<i64> = None;
                let mut seen_read_kilobytes = false;
                let mut f_read_operations: Option<i64> = None;
                let mut seen_read_operations = false;
                let mut f_read_time: Option<i64> = None;
                let mut seen_read_time = false;
                let mut f_write_kilobytes: Option<i64> = None;
                let mut seen_write_kilobytes = false;
                let mut f_write_operations: Option<i64> = None;
                let mut seen_write_operations = false;
                let mut f_write_time: Option<i64> = None;
                let mut seen_write_time = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "device_name" => {
                            if seen_device_name {
                                return Err(serde::de::Error::duplicate_field("device_name"));
                            }
                            seen_device_name = true;
                            f_device_name = map.next_value::<Option<String>>()?;
                        }
                        "io_time_in_millis" => {
                            if seen_io_time_in_millis {
                                return Err(serde::de::Error::duplicate_field("io_time_in_millis"));
                            }
                            seen_io_time_in_millis = true;
                            f_io_time_in_millis = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        "operations" => {
                            if seen_operations {
                                return Err(serde::de::Error::duplicate_field("operations"));
                            }
                            seen_operations = true;
                            f_operations = map.next_value::<Option<i64>>()?;
                        }
                        "queue_size" => {
                            if seen_queue_size {
                                return Err(serde::de::Error::duplicate_field("queue_size"));
                            }
                            seen_queue_size = true;
                            f_queue_size = map.next_value::<Option<i64>>()?;
                        }
                        "read_kilobytes" => {
                            if seen_read_kilobytes {
                                return Err(serde::de::Error::duplicate_field("read_kilobytes"));
                            }
                            seen_read_kilobytes = true;
                            f_read_kilobytes = map.next_value::<Option<i64>>()?;
                        }
                        "read_operations" => {
                            if seen_read_operations {
                                return Err(serde::de::Error::duplicate_field("read_operations"));
                            }
                            seen_read_operations = true;
                            f_read_operations = map.next_value::<Option<i64>>()?;
                        }
                        "read_time" => {
                            if seen_read_time {
                                return Err(serde::de::Error::duplicate_field("read_time"));
                            }
                            seen_read_time = true;
                            f_read_time = map.next_value::<Option<i64>>()?;
                        }
                        "write_kilobytes" => {
                            if seen_write_kilobytes {
                                return Err(serde::de::Error::duplicate_field("write_kilobytes"));
                            }
                            seen_write_kilobytes = true;
                            f_write_kilobytes = map.next_value::<Option<i64>>()?;
                        }
                        "write_operations" => {
                            if seen_write_operations {
                                return Err(serde::de::Error::duplicate_field("write_operations"));
                            }
                            seen_write_operations = true;
                            f_write_operations = map.next_value::<Option<i64>>()?;
                        }
                        "write_time" => {
                            if seen_write_time {
                                return Err(serde::de::Error::duplicate_field("write_time"));
                            }
                            seen_write_time = true;
                            f_write_time = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IoStatDevice {
                    device_name: f_device_name,
                    io_time_in_millis: f_io_time_in_millis,
                    operations: f_operations,
                    queue_size: f_queue_size,
                    read_kilobytes: f_read_kilobytes,
                    read_operations: f_read_operations,
                    read_time: f_read_time,
                    write_kilobytes: f_write_kilobytes,
                    write_operations: f_write_operations,
                    write_time: f_write_time,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IoStatDevice {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.device_name {
            map.serialize_entry("device_name", v)?;
        }
        if let Some(v) = &self.io_time_in_millis {
            map.serialize_entry("io_time_in_millis", v)?;
        }
        if let Some(v) = &self.operations {
            map.serialize_entry("operations", v)?;
        }
        if let Some(v) = &self.queue_size {
            map.serialize_entry("queue_size", v)?;
        }
        if let Some(v) = &self.read_kilobytes {
            map.serialize_entry("read_kilobytes", v)?;
        }
        if let Some(v) = &self.read_operations {
            map.serialize_entry("read_operations", v)?;
        }
        if let Some(v) = &self.read_time {
            map.serialize_entry("read_time", v)?;
        }
        if let Some(v) = &self.write_kilobytes {
            map.serialize_entry("write_kilobytes", v)?;
        }
        if let Some(v) = &self.write_operations {
            map.serialize_entry("write_operations", v)?;
        }
        if let Some(v) = &self.write_time {
            map.serialize_entry("write_time", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IoStats {
    #[doc = "Array of disk metrics for each device that is backing an OpenSearch data path. These disk metrics are probed periodically and averages between the last probe and the current probe are computed."]
    pub devices: Option<Vec<crate::types::nodes_stats::IoStatDevice>>,
    pub total: Option<crate::types::nodes_stats::IoStatDevice>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IoStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IoStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IoStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_devices: Option<Vec<crate::types::nodes_stats::IoStatDevice>> = None;
                let mut seen_devices = false;
                let mut f_total: Option<crate::types::nodes_stats::IoStatDevice> = None;
                let mut seen_total = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "devices" => {
                            if seen_devices {
                                return Err(serde::de::Error::duplicate_field("devices"));
                            }
                            seen_devices = true;
                            f_devices = map
                                .next_value::<Option<Vec<crate::types::nodes_stats::IoStatDevice>>>(
                                )?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map
                                .next_value::<Option<crate::types::nodes_stats::IoStatDevice>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IoStats {
                    devices: f_devices,
                    total: f_total,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IoStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.devices {
            map.serialize_entry("devices", v)?;
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
pub struct Jvm {
    #[doc = "Contains statistics about JVM buffer pools for the node."]
    pub buffer_pools:
        Option<std::collections::BTreeMap<String, crate::types::nodes_stats::NodeBufferPool>>,
    pub classes: Option<crate::types::nodes_stats::JvmClasses>,
    pub gc: Option<crate::types::nodes_stats::GarbageCollector>,
    pub mem: Option<crate::types::nodes_stats::JvmMemoryStats>,
    pub threads: Option<crate::types::nodes_stats::JvmThreads>,
    #[doc = "Last time JVM statistics were refreshed."]
    pub timestamp: Option<crate::types::common::EpochTimeUnitMillis>,
    #[doc = "Human-readable JVM uptime. Only returned if the `human` query parameter is `true`."]
    pub uptime: Option<crate::types::common::Duration>,
    #[doc = "JVM uptime in milliseconds."]
    pub uptime_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Jvm {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Jvm;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Jvm object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_buffer_pools: Option<
                    std::collections::BTreeMap<String, crate::types::nodes_stats::NodeBufferPool>,
                > = None;
                let mut seen_buffer_pools = false;
                let mut f_classes: Option<crate::types::nodes_stats::JvmClasses> = None;
                let mut seen_classes = false;
                let mut f_gc: Option<crate::types::nodes_stats::GarbageCollector> = None;
                let mut seen_gc = false;
                let mut f_mem: Option<crate::types::nodes_stats::JvmMemoryStats> = None;
                let mut seen_mem = false;
                let mut f_threads: Option<crate::types::nodes_stats::JvmThreads> = None;
                let mut seen_threads = false;
                let mut f_timestamp: Option<crate::types::common::EpochTimeUnitMillis> = None;
                let mut seen_timestamp = false;
                let mut f_uptime: Option<crate::types::common::Duration> = None;
                let mut seen_uptime = false;
                let mut f_uptime_in_millis: Option<crate::types::common::DurationValueUnitMillis> =
                    None;
                let mut seen_uptime_in_millis = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "buffer_pools" => {
                            if seen_buffer_pools {
                                return Err(serde::de::Error::duplicate_field("buffer_pools"));
                            }
                            seen_buffer_pools = true;
                            f_buffer_pools = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::nodes_stats::NodeBufferPool,
                                >,
                            >>()?;
                        }
                        "classes" => {
                            if seen_classes {
                                return Err(serde::de::Error::duplicate_field("classes"));
                            }
                            seen_classes = true;
                            f_classes =
                                map.next_value::<Option<crate::types::nodes_stats::JvmClasses>>()?;
                        }
                        "gc" => {
                            if seen_gc {
                                return Err(serde::de::Error::duplicate_field("gc"));
                            }
                            seen_gc = true;
                            f_gc = map
                                .next_value::<Option<crate::types::nodes_stats::GarbageCollector>>(
                                )?;
                        }
                        "mem" => {
                            if seen_mem {
                                return Err(serde::de::Error::duplicate_field("mem"));
                            }
                            seen_mem = true;
                            f_mem = map
                                .next_value::<Option<crate::types::nodes_stats::JvmMemoryStats>>(
                                )?;
                        }
                        "threads" => {
                            if seen_threads {
                                return Err(serde::de::Error::duplicate_field("threads"));
                            }
                            seen_threads = true;
                            f_threads =
                                map.next_value::<Option<crate::types::nodes_stats::JvmThreads>>()?;
                        }
                        "timestamp" => {
                            if seen_timestamp {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            seen_timestamp = true;
                            f_timestamp = map
                                .next_value::<Option<crate::types::common::EpochTimeUnitMillis>>(
                                )?;
                        }
                        "uptime" => {
                            if seen_uptime {
                                return Err(serde::de::Error::duplicate_field("uptime"));
                            }
                            seen_uptime = true;
                            f_uptime =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "uptime_in_millis" => {
                            if seen_uptime_in_millis {
                                return Err(serde::de::Error::duplicate_field("uptime_in_millis"));
                            }
                            seen_uptime_in_millis = true;
                            f_uptime_in_millis = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Jvm {
                    buffer_pools: f_buffer_pools,
                    classes: f_classes,
                    gc: f_gc,
                    mem: f_mem,
                    threads: f_threads,
                    timestamp: f_timestamp,
                    uptime: f_uptime,
                    uptime_in_millis: f_uptime_in_millis,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Jvm {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.buffer_pools {
            map.serialize_entry("buffer_pools", v)?;
        }
        if let Some(v) = &self.classes {
            map.serialize_entry("classes", v)?;
        }
        if let Some(v) = &self.gc {
            map.serialize_entry("gc", v)?;
        }
        if let Some(v) = &self.mem {
            map.serialize_entry("mem", v)?;
        }
        if let Some(v) = &self.threads {
            map.serialize_entry("threads", v)?;
        }
        if let Some(v) = &self.timestamp {
            map.serialize_entry("timestamp", v)?;
        }
        if let Some(v) = &self.uptime {
            map.serialize_entry("uptime", v)?;
        }
        if let Some(v) = &self.uptime_in_millis {
            map.serialize_entry("uptime_in_millis", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct JvmClasses {
    #[doc = "Number of classes currently loaded by JVM."]
    pub current_loaded_count: Option<i64>,
    #[doc = "Total number of classes loaded since the JVM started."]
    pub total_loaded_count: Option<i64>,
    #[doc = "Total number of classes unloaded since the JVM started."]
    pub total_unloaded_count: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for JvmClasses {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = JvmClasses;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a JvmClasses object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_current_loaded_count: Option<i64> = None;
                let mut seen_current_loaded_count = false;
                let mut f_total_loaded_count: Option<i64> = None;
                let mut seen_total_loaded_count = false;
                let mut f_total_unloaded_count: Option<i64> = None;
                let mut seen_total_unloaded_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "current_loaded_count" => {
                            if seen_current_loaded_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "current_loaded_count",
                                ));
                            }
                            seen_current_loaded_count = true;
                            f_current_loaded_count = map.next_value::<Option<i64>>()?;
                        }
                        "total_loaded_count" => {
                            if seen_total_loaded_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_loaded_count",
                                ));
                            }
                            seen_total_loaded_count = true;
                            f_total_loaded_count = map.next_value::<Option<i64>>()?;
                        }
                        "total_unloaded_count" => {
                            if seen_total_unloaded_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_unloaded_count",
                                ));
                            }
                            seen_total_unloaded_count = true;
                            f_total_unloaded_count = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(JvmClasses {
                    current_loaded_count: f_current_loaded_count,
                    total_loaded_count: f_total_loaded_count,
                    total_unloaded_count: f_total_unloaded_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for JvmClasses {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.current_loaded_count {
            map.serialize_entry("current_loaded_count", v)?;
        }
        if let Some(v) = &self.total_loaded_count {
            map.serialize_entry("total_loaded_count", v)?;
        }
        if let Some(v) = &self.total_unloaded_count {
            map.serialize_entry("total_unloaded_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct JvmMemoryStats {
    #[doc = "Amount of memory available for use by the heap."]
    pub heap_committed: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Amount of memory, in bytes, available for use by the heap."]
    pub heap_committed_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Maximum amount of memory available for use by the heap."]
    pub heap_max: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Maximum amount of memory, in bytes, available for use by the heap."]
    pub heap_max_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Memory currently in use by the heap."]
    pub heap_used: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Memory, in bytes, currently in use by the heap."]
    pub heap_used_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Percentage of memory currently in use by the heap."]
    pub heap_used_percent: Option<crate::types::common::PercentageNumber>,
    #[doc = "Amount of non-heap memory available."]
    pub non_heap_committed: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Amount of non-heap memory available, in bytes."]
    pub non_heap_committed_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Non-heap memory used."]
    pub non_heap_used: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Non-heap memory used, in bytes."]
    pub non_heap_used_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Contains statistics about heap memory usage for the node."]
    pub pools: Option<std::collections::BTreeMap<String, crate::types::nodes_stats::Pool>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for JvmMemoryStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = JvmMemoryStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a JvmMemoryStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_heap_committed: Option<crate::types::common::HumanReadableByteCount> =
                    None;
                let mut seen_heap_committed = false;
                let mut f_heap_committed_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_heap_committed_in_bytes = false;
                let mut f_heap_max: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_heap_max = false;
                let mut f_heap_max_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_heap_max_in_bytes = false;
                let mut f_heap_used: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_heap_used = false;
                let mut f_heap_used_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_heap_used_in_bytes = false;
                let mut f_heap_used_percent: Option<crate::types::common::PercentageNumber> = None;
                let mut seen_heap_used_percent = false;
                let mut f_non_heap_committed: Option<crate::types::common::HumanReadableByteCount> =
                    None;
                let mut seen_non_heap_committed = false;
                let mut f_non_heap_committed_in_bytes: Option<crate::types::common::ByteCount> =
                    None;
                let mut seen_non_heap_committed_in_bytes = false;
                let mut f_non_heap_used: Option<crate::types::common::HumanReadableByteCount> =
                    None;
                let mut seen_non_heap_used = false;
                let mut f_non_heap_used_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_non_heap_used_in_bytes = false;
                let mut f_pools: Option<
                    std::collections::BTreeMap<String, crate::types::nodes_stats::Pool>,
                > = None;
                let mut seen_pools = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "heap_committed" => {
                            if seen_heap_committed {
                                return Err(serde::de::Error::duplicate_field("heap_committed"));
                            }
                            seen_heap_committed = true;
                            f_heap_committed = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "heap_committed_in_bytes" => {
                            if seen_heap_committed_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "heap_committed_in_bytes",
                                ));
                            }
                            seen_heap_committed_in_bytes = true;
                            f_heap_committed_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "heap_max" => {
                            if seen_heap_max {
                                return Err(serde::de::Error::duplicate_field("heap_max"));
                            }
                            seen_heap_max = true;
                            f_heap_max = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "heap_max_in_bytes" => {
                            if seen_heap_max_in_bytes {
                                return Err(serde::de::Error::duplicate_field("heap_max_in_bytes"));
                            }
                            seen_heap_max_in_bytes = true;
                            f_heap_max_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "heap_used" => {
                            if seen_heap_used {
                                return Err(serde::de::Error::duplicate_field("heap_used"));
                            }
                            seen_heap_used = true;
                            f_heap_used = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "heap_used_in_bytes" => {
                            if seen_heap_used_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "heap_used_in_bytes",
                                ));
                            }
                            seen_heap_used_in_bytes = true;
                            f_heap_used_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "heap_used_percent" => {
                            if seen_heap_used_percent {
                                return Err(serde::de::Error::duplicate_field("heap_used_percent"));
                            }
                            seen_heap_used_percent = true;
                            f_heap_used_percent =
                                map.next_value::<Option<crate::types::common::PercentageNumber>>()?;
                        }
                        "non_heap_committed" => {
                            if seen_non_heap_committed {
                                return Err(serde::de::Error::duplicate_field(
                                    "non_heap_committed",
                                ));
                            }
                            seen_non_heap_committed = true;
                            f_non_heap_committed = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "non_heap_committed_in_bytes" => {
                            if seen_non_heap_committed_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "non_heap_committed_in_bytes",
                                ));
                            }
                            seen_non_heap_committed_in_bytes = true;
                            f_non_heap_committed_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "non_heap_used" => {
                            if seen_non_heap_used {
                                return Err(serde::de::Error::duplicate_field("non_heap_used"));
                            }
                            seen_non_heap_used = true;
                            f_non_heap_used = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "non_heap_used_in_bytes" => {
                            if seen_non_heap_used_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "non_heap_used_in_bytes",
                                ));
                            }
                            seen_non_heap_used_in_bytes = true;
                            f_non_heap_used_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "pools" => {
                            if seen_pools {
                                return Err(serde::de::Error::duplicate_field("pools"));
                            }
                            seen_pools = true;
                            f_pools = map.next_value::<Option<
                                std::collections::BTreeMap<String, crate::types::nodes_stats::Pool>,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(JvmMemoryStats {
                    heap_committed: f_heap_committed,
                    heap_committed_in_bytes: f_heap_committed_in_bytes,
                    heap_max: f_heap_max,
                    heap_max_in_bytes: f_heap_max_in_bytes,
                    heap_used: f_heap_used,
                    heap_used_in_bytes: f_heap_used_in_bytes,
                    heap_used_percent: f_heap_used_percent,
                    non_heap_committed: f_non_heap_committed,
                    non_heap_committed_in_bytes: f_non_heap_committed_in_bytes,
                    non_heap_used: f_non_heap_used,
                    non_heap_used_in_bytes: f_non_heap_used_in_bytes,
                    pools: f_pools,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for JvmMemoryStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.heap_committed {
            map.serialize_entry("heap_committed", v)?;
        }
        if let Some(v) = &self.heap_committed_in_bytes {
            map.serialize_entry("heap_committed_in_bytes", v)?;
        }
        if let Some(v) = &self.heap_max {
            map.serialize_entry("heap_max", v)?;
        }
        if let Some(v) = &self.heap_max_in_bytes {
            map.serialize_entry("heap_max_in_bytes", v)?;
        }
        if let Some(v) = &self.heap_used {
            map.serialize_entry("heap_used", v)?;
        }
        if let Some(v) = &self.heap_used_in_bytes {
            map.serialize_entry("heap_used_in_bytes", v)?;
        }
        if let Some(v) = &self.heap_used_percent {
            map.serialize_entry("heap_used_percent", v)?;
        }
        if let Some(v) = &self.non_heap_committed {
            map.serialize_entry("non_heap_committed", v)?;
        }
        if let Some(v) = &self.non_heap_committed_in_bytes {
            map.serialize_entry("non_heap_committed_in_bytes", v)?;
        }
        if let Some(v) = &self.non_heap_used {
            map.serialize_entry("non_heap_used", v)?;
        }
        if let Some(v) = &self.non_heap_used_in_bytes {
            map.serialize_entry("non_heap_used_in_bytes", v)?;
        }
        if let Some(v) = &self.pools {
            map.serialize_entry("pools", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct JvmThreads {
    #[doc = "Number of active threads in use by JVM."]
    pub count: Option<i64>,
    #[doc = "Highest number of threads used by JVM."]
    pub peak_count: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for JvmThreads {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = JvmThreads;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a JvmThreads object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_count: Option<i64> = None;
                let mut seen_count = false;
                let mut f_peak_count: Option<i64> = None;
                let mut seen_peak_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "count" => {
                            if seen_count {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            seen_count = true;
                            f_count = map.next_value::<Option<i64>>()?;
                        }
                        "peak_count" => {
                            if seen_peak_count {
                                return Err(serde::de::Error::duplicate_field("peak_count"));
                            }
                            seen_peak_count = true;
                            f_peak_count = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(JvmThreads {
                    count: f_count,
                    peak_count: f_peak_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for JvmThreads {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.count {
            map.serialize_entry("count", v)?;
        }
        if let Some(v) = &self.peak_count {
            map.serialize_entry("peak_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct KeyedProcessor {
    pub stats: Option<crate::types::nodes_stats::Processor>,
    pub r#type: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for KeyedProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = KeyedProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a KeyedProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_stats: Option<crate::types::nodes_stats::Processor> = None;
                let mut seen_stats = false;
                let mut f_type: Option<String> = None;
                let mut seen_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "stats" => {
                            if seen_stats {
                                return Err(serde::de::Error::duplicate_field("stats"));
                            }
                            seen_stats = true;
                            f_stats =
                                map.next_value::<Option<crate::types::nodes_stats::Processor>>()?;
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
                Ok(KeyedProcessor {
                    stats: f_stats,
                    r#type: f_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for KeyedProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.stats {
            map.serialize_entry("stats", v)?;
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

#[derive(Debug, Clone)]
pub struct LastGcStats {
    pub max: Option<crate::types::common::HumanReadableByteCount>,
    pub max_in_bytes: Option<crate::types::common::ByteCount>,
    pub usage_percent: Option<crate::types::common::PercentageNumber>,
    pub used: Option<crate::types::common::HumanReadableByteCount>,
    pub used_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for LastGcStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = LastGcStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a LastGcStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_max: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_max = false;
                let mut f_max_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_max_in_bytes = false;
                let mut f_usage_percent: Option<crate::types::common::PercentageNumber> = None;
                let mut seen_usage_percent = false;
                let mut f_used: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_used = false;
                let mut f_used_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_used_in_bytes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "max" => {
                            if seen_max {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            seen_max = true;
                            f_max = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "max_in_bytes" => {
                            if seen_max_in_bytes {
                                return Err(serde::de::Error::duplicate_field("max_in_bytes"));
                            }
                            seen_max_in_bytes = true;
                            f_max_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "usage_percent" => {
                            if seen_usage_percent {
                                return Err(serde::de::Error::duplicate_field("usage_percent"));
                            }
                            seen_usage_percent = true;
                            f_usage_percent =
                                map.next_value::<Option<crate::types::common::PercentageNumber>>()?;
                        }
                        "used" => {
                            if seen_used {
                                return Err(serde::de::Error::duplicate_field("used"));
                            }
                            seen_used = true;
                            f_used = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "used_in_bytes" => {
                            if seen_used_in_bytes {
                                return Err(serde::de::Error::duplicate_field("used_in_bytes"));
                            }
                            seen_used_in_bytes = true;
                            f_used_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(LastGcStats {
                    max: f_max,
                    max_in_bytes: f_max_in_bytes,
                    usage_percent: f_usage_percent,
                    used: f_used,
                    used_in_bytes: f_used_in_bytes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for LastGcStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.max {
            map.serialize_entry("max", v)?;
        }
        if let Some(v) = &self.max_in_bytes {
            map.serialize_entry("max_in_bytes", v)?;
        }
        if let Some(v) = &self.usage_percent {
            map.serialize_entry("usage_percent", v)?;
        }
        if let Some(v) = &self.used {
            map.serialize_entry("used", v)?;
        }
        if let Some(v) = &self.used_in_bytes {
            map.serialize_entry("used_in_bytes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct MemoryStats {
    #[doc = "Amount of free physical memory."]
    pub free: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Amount of free physical memory in bytes."]
    pub free_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Total amount of physical memory."]
    pub total: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total amount of physical memory in bytes."]
    pub total_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Amount of used physical memory."]
    pub used: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Amount of used physical memory in bytes."]
    pub used_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MemoryStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MemoryStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MemoryStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_free: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_free = false;
                let mut f_free_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_free_in_bytes = false;
                let mut f_total: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_total = false;
                let mut f_total_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_total_in_bytes = false;
                let mut f_used: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_used = false;
                let mut f_used_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_used_in_bytes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "free" => {
                            if seen_free {
                                return Err(serde::de::Error::duplicate_field("free"));
                            }
                            seen_free = true;
                            f_free = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "free_in_bytes" => {
                            if seen_free_in_bytes {
                                return Err(serde::de::Error::duplicate_field("free_in_bytes"));
                            }
                            seen_free_in_bytes = true;
                            f_free_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "total_in_bytes" => {
                            if seen_total_in_bytes {
                                return Err(serde::de::Error::duplicate_field("total_in_bytes"));
                            }
                            seen_total_in_bytes = true;
                            f_total_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "used" => {
                            if seen_used {
                                return Err(serde::de::Error::duplicate_field("used"));
                            }
                            seen_used = true;
                            f_used = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "used_in_bytes" => {
                            if seen_used_in_bytes {
                                return Err(serde::de::Error::duplicate_field("used_in_bytes"));
                            }
                            seen_used_in_bytes = true;
                            f_used_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MemoryStats {
                    free: f_free,
                    free_in_bytes: f_free_in_bytes,
                    total: f_total,
                    total_in_bytes: f_total_in_bytes,
                    used: f_used,
                    used_in_bytes: f_used_in_bytes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MemoryStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.free {
            map.serialize_entry("free", v)?;
        }
        if let Some(v) = &self.free_in_bytes {
            map.serialize_entry("free_in_bytes", v)?;
        }
        if let Some(v) = &self.total {
            map.serialize_entry("total", v)?;
        }
        if let Some(v) = &self.total_in_bytes {
            map.serialize_entry("total_in_bytes", v)?;
        }
        if let Some(v) = &self.used {
            map.serialize_entry("used", v)?;
        }
        if let Some(v) = &self.used_in_bytes {
            map.serialize_entry("used_in_bytes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NodeBufferPool {
    #[doc = "Number of buffer pools."]
    pub count: Option<i64>,
    #[doc = "Total capacity of buffer pools."]
    pub total_capacity: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total capacity of buffer pools in bytes."]
    pub total_capacity_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Size of buffer pools."]
    pub used: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Size of buffer pools in bytes."]
    pub used_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeBufferPool {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeBufferPool;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeBufferPool object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_count: Option<i64> = None;
                let mut seen_count = false;
                let mut f_total_capacity: Option<crate::types::common::HumanReadableByteCount> =
                    None;
                let mut seen_total_capacity = false;
                let mut f_total_capacity_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_total_capacity_in_bytes = false;
                let mut f_used: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_used = false;
                let mut f_used_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_used_in_bytes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "count" => {
                            if seen_count {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            seen_count = true;
                            f_count = map.next_value::<Option<i64>>()?;
                        }
                        "total_capacity" => {
                            if seen_total_capacity {
                                return Err(serde::de::Error::duplicate_field("total_capacity"));
                            }
                            seen_total_capacity = true;
                            f_total_capacity = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "total_capacity_in_bytes" => {
                            if seen_total_capacity_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_capacity_in_bytes",
                                ));
                            }
                            seen_total_capacity_in_bytes = true;
                            f_total_capacity_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "used" => {
                            if seen_used {
                                return Err(serde::de::Error::duplicate_field("used"));
                            }
                            seen_used = true;
                            f_used = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "used_in_bytes" => {
                            if seen_used_in_bytes {
                                return Err(serde::de::Error::duplicate_field("used_in_bytes"));
                            }
                            seen_used_in_bytes = true;
                            f_used_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeBufferPool {
                    count: f_count,
                    total_capacity: f_total_capacity,
                    total_capacity_in_bytes: f_total_capacity_in_bytes,
                    used: f_used,
                    used_in_bytes: f_used_in_bytes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeBufferPool {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.count {
            map.serialize_entry("count", v)?;
        }
        if let Some(v) = &self.total_capacity {
            map.serialize_entry("total_capacity", v)?;
        }
        if let Some(v) = &self.total_capacity_in_bytes {
            map.serialize_entry("total_capacity_in_bytes", v)?;
        }
        if let Some(v) = &self.used {
            map.serialize_entry("used", v)?;
        }
        if let Some(v) = &self.used_in_bytes {
            map.serialize_entry("used_in_bytes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NodeIndexShardStats {
    pub completion: Option<crate::types::common::CompletionStats>,
    pub docs: Option<crate::types::common::DocStats>,
    pub fielddata: Option<crate::types::common::FielddataStats>,
    pub flush: Option<crate::types::common::FlushStats>,
    pub get: Option<crate::types::common::GetStats>,
    pub indexing: Option<crate::types::common::IndexingStats>,
    pub merges: Option<crate::types::common::MergesStats>,
    pub query_cache: Option<crate::types::common::QueryCacheStats>,
    pub recovery: Option<crate::types::common::RecoveryStats>,
    pub refresh: Option<crate::types::common::RefreshStats>,
    pub request_cache: Option<crate::types::common::RequestCacheStats>,
    pub search: Option<crate::types::common::SearchStats>,
    pub segments: Option<crate::types::common::SegmentsStats>,
    pub store: Option<crate::types::common::StoreStats>,
    pub translog: Option<crate::types::common::TranslogStats>,
    pub warmer: Option<crate::types::common::WarmerStats>,
    pub commit: Option<crate::types::indices_stats::ShardCommit>,
    pub retention_leases: Option<crate::types::indices_stats::ShardRetentionLeases>,
    pub routing: Option<crate::types::indices_stats::ShardRouting>,
    pub seq_no: Option<crate::types::indices_stats::ShardSequenceNumber>,
    pub shard_path: Option<crate::types::indices_stats::ShardPath>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeIndexShardStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeIndexShardStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeIndexShardStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_completion: Option<crate::types::common::CompletionStats> = None;
                let mut seen_completion = false;
                let mut f_docs: Option<crate::types::common::DocStats> = None;
                let mut seen_docs = false;
                let mut f_fielddata: Option<crate::types::common::FielddataStats> = None;
                let mut seen_fielddata = false;
                let mut f_flush: Option<crate::types::common::FlushStats> = None;
                let mut seen_flush = false;
                let mut f_get: Option<crate::types::common::GetStats> = None;
                let mut seen_get = false;
                let mut f_indexing: Option<crate::types::common::IndexingStats> = None;
                let mut seen_indexing = false;
                let mut f_merges: Option<crate::types::common::MergesStats> = None;
                let mut seen_merges = false;
                let mut f_query_cache: Option<crate::types::common::QueryCacheStats> = None;
                let mut seen_query_cache = false;
                let mut f_recovery: Option<crate::types::common::RecoveryStats> = None;
                let mut seen_recovery = false;
                let mut f_refresh: Option<crate::types::common::RefreshStats> = None;
                let mut seen_refresh = false;
                let mut f_request_cache: Option<crate::types::common::RequestCacheStats> = None;
                let mut seen_request_cache = false;
                let mut f_search: Option<crate::types::common::SearchStats> = None;
                let mut seen_search = false;
                let mut f_segments: Option<crate::types::common::SegmentsStats> = None;
                let mut seen_segments = false;
                let mut f_store: Option<crate::types::common::StoreStats> = None;
                let mut seen_store = false;
                let mut f_translog: Option<crate::types::common::TranslogStats> = None;
                let mut seen_translog = false;
                let mut f_warmer: Option<crate::types::common::WarmerStats> = None;
                let mut seen_warmer = false;
                let mut f_commit: Option<crate::types::indices_stats::ShardCommit> = None;
                let mut seen_commit = false;
                let mut f_retention_leases: Option<
                    crate::types::indices_stats::ShardRetentionLeases,
                > = None;
                let mut seen_retention_leases = false;
                let mut f_routing: Option<crate::types::indices_stats::ShardRouting> = None;
                let mut seen_routing = false;
                let mut f_seq_no: Option<crate::types::indices_stats::ShardSequenceNumber> = None;
                let mut seen_seq_no = false;
                let mut f_shard_path: Option<crate::types::indices_stats::ShardPath> = None;
                let mut seen_shard_path = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "completion" => {
                            if seen_completion {
                                return Err(serde::de::Error::duplicate_field("completion"));
                            }
                            seen_completion = true;
                            f_completion =
                                map.next_value::<Option<crate::types::common::CompletionStats>>()?;
                        }
                        "docs" => {
                            if seen_docs {
                                return Err(serde::de::Error::duplicate_field("docs"));
                            }
                            seen_docs = true;
                            f_docs = map.next_value::<Option<crate::types::common::DocStats>>()?;
                        }
                        "fielddata" => {
                            if seen_fielddata {
                                return Err(serde::de::Error::duplicate_field("fielddata"));
                            }
                            seen_fielddata = true;
                            f_fielddata =
                                map.next_value::<Option<crate::types::common::FielddataStats>>()?;
                        }
                        "flush" => {
                            if seen_flush {
                                return Err(serde::de::Error::duplicate_field("flush"));
                            }
                            seen_flush = true;
                            f_flush =
                                map.next_value::<Option<crate::types::common::FlushStats>>()?;
                        }
                        "get" => {
                            if seen_get {
                                return Err(serde::de::Error::duplicate_field("get"));
                            }
                            seen_get = true;
                            f_get = map.next_value::<Option<crate::types::common::GetStats>>()?;
                        }
                        "indexing" => {
                            if seen_indexing {
                                return Err(serde::de::Error::duplicate_field("indexing"));
                            }
                            seen_indexing = true;
                            f_indexing =
                                map.next_value::<Option<crate::types::common::IndexingStats>>()?;
                        }
                        "merges" => {
                            if seen_merges {
                                return Err(serde::de::Error::duplicate_field("merges"));
                            }
                            seen_merges = true;
                            f_merges =
                                map.next_value::<Option<crate::types::common::MergesStats>>()?;
                        }
                        "query_cache" => {
                            if seen_query_cache {
                                return Err(serde::de::Error::duplicate_field("query_cache"));
                            }
                            seen_query_cache = true;
                            f_query_cache =
                                map.next_value::<Option<crate::types::common::QueryCacheStats>>()?;
                        }
                        "recovery" => {
                            if seen_recovery {
                                return Err(serde::de::Error::duplicate_field("recovery"));
                            }
                            seen_recovery = true;
                            f_recovery =
                                map.next_value::<Option<crate::types::common::RecoveryStats>>()?;
                        }
                        "refresh" => {
                            if seen_refresh {
                                return Err(serde::de::Error::duplicate_field("refresh"));
                            }
                            seen_refresh = true;
                            f_refresh =
                                map.next_value::<Option<crate::types::common::RefreshStats>>()?;
                        }
                        "request_cache" => {
                            if seen_request_cache {
                                return Err(serde::de::Error::duplicate_field("request_cache"));
                            }
                            seen_request_cache = true;
                            f_request_cache = map
                                .next_value::<Option<crate::types::common::RequestCacheStats>>()?;
                        }
                        "search" => {
                            if seen_search {
                                return Err(serde::de::Error::duplicate_field("search"));
                            }
                            seen_search = true;
                            f_search =
                                map.next_value::<Option<crate::types::common::SearchStats>>()?;
                        }
                        "segments" => {
                            if seen_segments {
                                return Err(serde::de::Error::duplicate_field("segments"));
                            }
                            seen_segments = true;
                            f_segments =
                                map.next_value::<Option<crate::types::common::SegmentsStats>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store =
                                map.next_value::<Option<crate::types::common::StoreStats>>()?;
                        }
                        "translog" => {
                            if seen_translog {
                                return Err(serde::de::Error::duplicate_field("translog"));
                            }
                            seen_translog = true;
                            f_translog =
                                map.next_value::<Option<crate::types::common::TranslogStats>>()?;
                        }
                        "warmer" => {
                            if seen_warmer {
                                return Err(serde::de::Error::duplicate_field("warmer"));
                            }
                            seen_warmer = true;
                            f_warmer =
                                map.next_value::<Option<crate::types::common::WarmerStats>>()?;
                        }
                        "commit" => {
                            if seen_commit {
                                return Err(serde::de::Error::duplicate_field("commit"));
                            }
                            seen_commit = true;
                            f_commit = map
                                .next_value::<Option<crate::types::indices_stats::ShardCommit>>()?;
                        }
                        "retention_leases" => {
                            if seen_retention_leases {
                                return Err(serde::de::Error::duplicate_field("retention_leases"));
                            }
                            seen_retention_leases = true;
                            f_retention_leases = map.next_value::<Option<crate::types::indices_stats::ShardRetentionLeases>>()?;
                        }
                        "routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("routing"));
                            }
                            seen_routing = true;
                            f_routing = map
                                .next_value::<Option<crate::types::indices_stats::ShardRouting>>(
                                )?;
                        }
                        "seq_no" => {
                            if seen_seq_no {
                                return Err(serde::de::Error::duplicate_field("seq_no"));
                            }
                            seen_seq_no = true;
                            f_seq_no = map.next_value::<Option<crate::types::indices_stats::ShardSequenceNumber>>()?;
                        }
                        "shard_path" => {
                            if seen_shard_path {
                                return Err(serde::de::Error::duplicate_field("shard_path"));
                            }
                            seen_shard_path = true;
                            f_shard_path =
                                map.next_value::<Option<crate::types::indices_stats::ShardPath>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeIndexShardStats {
                    completion: f_completion,
                    docs: f_docs,
                    fielddata: f_fielddata,
                    flush: f_flush,
                    get: f_get,
                    indexing: f_indexing,
                    merges: f_merges,
                    query_cache: f_query_cache,
                    recovery: f_recovery,
                    refresh: f_refresh,
                    request_cache: f_request_cache,
                    search: f_search,
                    segments: f_segments,
                    store: f_store,
                    translog: f_translog,
                    warmer: f_warmer,
                    commit: f_commit,
                    retention_leases: f_retention_leases,
                    routing: f_routing,
                    seq_no: f_seq_no,
                    shard_path: f_shard_path,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeIndexShardStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.completion {
            map.serialize_entry("completion", v)?;
        }
        if let Some(v) = &self.docs {
            map.serialize_entry("docs", v)?;
        }
        if let Some(v) = &self.fielddata {
            map.serialize_entry("fielddata", v)?;
        }
        if let Some(v) = &self.flush {
            map.serialize_entry("flush", v)?;
        }
        if let Some(v) = &self.get {
            map.serialize_entry("get", v)?;
        }
        if let Some(v) = &self.indexing {
            map.serialize_entry("indexing", v)?;
        }
        if let Some(v) = &self.merges {
            map.serialize_entry("merges", v)?;
        }
        if let Some(v) = &self.query_cache {
            map.serialize_entry("query_cache", v)?;
        }
        if let Some(v) = &self.recovery {
            map.serialize_entry("recovery", v)?;
        }
        if let Some(v) = &self.refresh {
            map.serialize_entry("refresh", v)?;
        }
        if let Some(v) = &self.request_cache {
            map.serialize_entry("request_cache", v)?;
        }
        if let Some(v) = &self.search {
            map.serialize_entry("search", v)?;
        }
        if let Some(v) = &self.segments {
            map.serialize_entry("segments", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.translog {
            map.serialize_entry("translog", v)?;
        }
        if let Some(v) = &self.warmer {
            map.serialize_entry("warmer", v)?;
        }
        if let Some(v) = &self.commit {
            map.serialize_entry("commit", v)?;
        }
        if let Some(v) = &self.retention_leases {
            map.serialize_entry("retention_leases", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("routing", v)?;
        }
        if let Some(v) = &self.seq_no {
            map.serialize_entry("seq_no", v)?;
        }
        if let Some(v) = &self.shard_path {
            map.serialize_entry("shard_path", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NodeIndicesStats {
    pub completion: Option<crate::types::common::CompletionStats>,
    pub docs: Option<crate::types::common::DocStats>,
    pub fielddata: Option<crate::types::common::FielddataStats>,
    pub flush: Option<crate::types::common::FlushStats>,
    pub get: Option<crate::types::common::GetStats>,
    pub indexing: Option<crate::types::common::IndexingStats>,
    pub merges: Option<crate::types::common::MergesStats>,
    pub query_cache: Option<crate::types::common::QueryCacheStats>,
    pub recovery: Option<crate::types::common::RecoveryStats>,
    pub refresh: Option<crate::types::common::RefreshStats>,
    pub request_cache: Option<crate::types::common::RequestCacheStats>,
    pub search: Option<crate::types::common::SearchStats>,
    pub segments: Option<crate::types::common::SegmentsStats>,
    pub store: Option<crate::types::common::StoreStats>,
    pub translog: Option<crate::types::common::TranslogStats>,
    pub warmer: Option<crate::types::common::WarmerStats>,
    pub indices:
        Option<std::collections::BTreeMap<String, crate::types::indices_stats::IndexStats>>,
    pub shards: Option<
        std::collections::BTreeMap<
            String,
            Vec<std::collections::BTreeMap<String, crate::types::nodes_stats::NodeIndexShardStats>>,
        >,
    >,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeIndicesStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeIndicesStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeIndicesStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_completion: Option<crate::types::common::CompletionStats> = None;
                let mut seen_completion = false;
                let mut f_docs: Option<crate::types::common::DocStats> = None;
                let mut seen_docs = false;
                let mut f_fielddata: Option<crate::types::common::FielddataStats> = None;
                let mut seen_fielddata = false;
                let mut f_flush: Option<crate::types::common::FlushStats> = None;
                let mut seen_flush = false;
                let mut f_get: Option<crate::types::common::GetStats> = None;
                let mut seen_get = false;
                let mut f_indexing: Option<crate::types::common::IndexingStats> = None;
                let mut seen_indexing = false;
                let mut f_merges: Option<crate::types::common::MergesStats> = None;
                let mut seen_merges = false;
                let mut f_query_cache: Option<crate::types::common::QueryCacheStats> = None;
                let mut seen_query_cache = false;
                let mut f_recovery: Option<crate::types::common::RecoveryStats> = None;
                let mut seen_recovery = false;
                let mut f_refresh: Option<crate::types::common::RefreshStats> = None;
                let mut seen_refresh = false;
                let mut f_request_cache: Option<crate::types::common::RequestCacheStats> = None;
                let mut seen_request_cache = false;
                let mut f_search: Option<crate::types::common::SearchStats> = None;
                let mut seen_search = false;
                let mut f_segments: Option<crate::types::common::SegmentsStats> = None;
                let mut seen_segments = false;
                let mut f_store: Option<crate::types::common::StoreStats> = None;
                let mut seen_store = false;
                let mut f_translog: Option<crate::types::common::TranslogStats> = None;
                let mut seen_translog = false;
                let mut f_warmer: Option<crate::types::common::WarmerStats> = None;
                let mut seen_warmer = false;
                let mut f_indices: Option<
                    std::collections::BTreeMap<String, crate::types::indices_stats::IndexStats>,
                > = None;
                let mut seen_indices = false;
                let mut f_shards: Option<
                    std::collections::BTreeMap<
                        String,
                        Vec<
                            std::collections::BTreeMap<
                                String,
                                crate::types::nodes_stats::NodeIndexShardStats,
                            >,
                        >,
                    >,
                > = None;
                let mut seen_shards = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "completion" => {
                            if seen_completion {
                                return Err(serde::de::Error::duplicate_field("completion"));
                            }
                            seen_completion = true;
                            f_completion =
                                map.next_value::<Option<crate::types::common::CompletionStats>>()?;
                        }
                        "docs" => {
                            if seen_docs {
                                return Err(serde::de::Error::duplicate_field("docs"));
                            }
                            seen_docs = true;
                            f_docs = map.next_value::<Option<crate::types::common::DocStats>>()?;
                        }
                        "fielddata" => {
                            if seen_fielddata {
                                return Err(serde::de::Error::duplicate_field("fielddata"));
                            }
                            seen_fielddata = true;
                            f_fielddata =
                                map.next_value::<Option<crate::types::common::FielddataStats>>()?;
                        }
                        "flush" => {
                            if seen_flush {
                                return Err(serde::de::Error::duplicate_field("flush"));
                            }
                            seen_flush = true;
                            f_flush =
                                map.next_value::<Option<crate::types::common::FlushStats>>()?;
                        }
                        "get" => {
                            if seen_get {
                                return Err(serde::de::Error::duplicate_field("get"));
                            }
                            seen_get = true;
                            f_get = map.next_value::<Option<crate::types::common::GetStats>>()?;
                        }
                        "indexing" => {
                            if seen_indexing {
                                return Err(serde::de::Error::duplicate_field("indexing"));
                            }
                            seen_indexing = true;
                            f_indexing =
                                map.next_value::<Option<crate::types::common::IndexingStats>>()?;
                        }
                        "merges" => {
                            if seen_merges {
                                return Err(serde::de::Error::duplicate_field("merges"));
                            }
                            seen_merges = true;
                            f_merges =
                                map.next_value::<Option<crate::types::common::MergesStats>>()?;
                        }
                        "query_cache" => {
                            if seen_query_cache {
                                return Err(serde::de::Error::duplicate_field("query_cache"));
                            }
                            seen_query_cache = true;
                            f_query_cache =
                                map.next_value::<Option<crate::types::common::QueryCacheStats>>()?;
                        }
                        "recovery" => {
                            if seen_recovery {
                                return Err(serde::de::Error::duplicate_field("recovery"));
                            }
                            seen_recovery = true;
                            f_recovery =
                                map.next_value::<Option<crate::types::common::RecoveryStats>>()?;
                        }
                        "refresh" => {
                            if seen_refresh {
                                return Err(serde::de::Error::duplicate_field("refresh"));
                            }
                            seen_refresh = true;
                            f_refresh =
                                map.next_value::<Option<crate::types::common::RefreshStats>>()?;
                        }
                        "request_cache" => {
                            if seen_request_cache {
                                return Err(serde::de::Error::duplicate_field("request_cache"));
                            }
                            seen_request_cache = true;
                            f_request_cache = map
                                .next_value::<Option<crate::types::common::RequestCacheStats>>()?;
                        }
                        "search" => {
                            if seen_search {
                                return Err(serde::de::Error::duplicate_field("search"));
                            }
                            seen_search = true;
                            f_search =
                                map.next_value::<Option<crate::types::common::SearchStats>>()?;
                        }
                        "segments" => {
                            if seen_segments {
                                return Err(serde::de::Error::duplicate_field("segments"));
                            }
                            seen_segments = true;
                            f_segments =
                                map.next_value::<Option<crate::types::common::SegmentsStats>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store =
                                map.next_value::<Option<crate::types::common::StoreStats>>()?;
                        }
                        "translog" => {
                            if seen_translog {
                                return Err(serde::de::Error::duplicate_field("translog"));
                            }
                            seen_translog = true;
                            f_translog =
                                map.next_value::<Option<crate::types::common::TranslogStats>>()?;
                        }
                        "warmer" => {
                            if seen_warmer {
                                return Err(serde::de::Error::duplicate_field("warmer"));
                            }
                            seen_warmer = true;
                            f_warmer =
                                map.next_value::<Option<crate::types::common::WarmerStats>>()?;
                        }
                        "indices" => {
                            if seen_indices {
                                return Err(serde::de::Error::duplicate_field("indices"));
                            }
                            seen_indices = true;
                            f_indices = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::indices_stats::IndexStats,
                                >,
                            >>()?;
                        }
                        "shards" => {
                            if seen_shards {
                                return Err(serde::de::Error::duplicate_field("shards"));
                            }
                            seen_shards = true;
                            f_shards = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Vec<
                                        std::collections::BTreeMap<
                                            String,
                                            crate::types::nodes_stats::NodeIndexShardStats,
                                        >,
                                    >,
                                >,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeIndicesStats {
                    completion: f_completion,
                    docs: f_docs,
                    fielddata: f_fielddata,
                    flush: f_flush,
                    get: f_get,
                    indexing: f_indexing,
                    merges: f_merges,
                    query_cache: f_query_cache,
                    recovery: f_recovery,
                    refresh: f_refresh,
                    request_cache: f_request_cache,
                    search: f_search,
                    segments: f_segments,
                    store: f_store,
                    translog: f_translog,
                    warmer: f_warmer,
                    indices: f_indices,
                    shards: f_shards,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeIndicesStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.completion {
            map.serialize_entry("completion", v)?;
        }
        if let Some(v) = &self.docs {
            map.serialize_entry("docs", v)?;
        }
        if let Some(v) = &self.fielddata {
            map.serialize_entry("fielddata", v)?;
        }
        if let Some(v) = &self.flush {
            map.serialize_entry("flush", v)?;
        }
        if let Some(v) = &self.get {
            map.serialize_entry("get", v)?;
        }
        if let Some(v) = &self.indexing {
            map.serialize_entry("indexing", v)?;
        }
        if let Some(v) = &self.merges {
            map.serialize_entry("merges", v)?;
        }
        if let Some(v) = &self.query_cache {
            map.serialize_entry("query_cache", v)?;
        }
        if let Some(v) = &self.recovery {
            map.serialize_entry("recovery", v)?;
        }
        if let Some(v) = &self.refresh {
            map.serialize_entry("refresh", v)?;
        }
        if let Some(v) = &self.request_cache {
            map.serialize_entry("request_cache", v)?;
        }
        if let Some(v) = &self.search {
            map.serialize_entry("search", v)?;
        }
        if let Some(v) = &self.segments {
            map.serialize_entry("segments", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.translog {
            map.serialize_entry("translog", v)?;
        }
        if let Some(v) = &self.warmer {
            map.serialize_entry("warmer", v)?;
        }
        if let Some(v) = &self.indices {
            map.serialize_entry("indices", v)?;
        }
        if let Some(v) = &self.shards {
            map.serialize_entry("shards", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct OperatingSystem {
    pub cgroup: Option<crate::types::nodes_stats::CgroupStats>,
    pub cpu: Option<crate::types::nodes_stats::OperatingSystemCpuStats>,
    pub mem: Option<crate::types::nodes_stats::ExtendedMemoryStats>,
    pub swap: Option<crate::types::nodes_stats::MemoryStats>,
    pub timestamp: Option<crate::types::common::EpochTimeUnitMillis>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for OperatingSystem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = OperatingSystem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a OperatingSystem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cgroup: Option<crate::types::nodes_stats::CgroupStats> = None;
                let mut seen_cgroup = false;
                let mut f_cpu: Option<crate::types::nodes_stats::OperatingSystemCpuStats> = None;
                let mut seen_cpu = false;
                let mut f_mem: Option<crate::types::nodes_stats::ExtendedMemoryStats> = None;
                let mut seen_mem = false;
                let mut f_swap: Option<crate::types::nodes_stats::MemoryStats> = None;
                let mut seen_swap = false;
                let mut f_timestamp: Option<crate::types::common::EpochTimeUnitMillis> = None;
                let mut seen_timestamp = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cgroup" => {
                            if seen_cgroup {
                                return Err(serde::de::Error::duplicate_field("cgroup"));
                            }
                            seen_cgroup = true;
                            f_cgroup =
                                map.next_value::<Option<crate::types::nodes_stats::CgroupStats>>()?;
                        }
                        "cpu" => {
                            if seen_cpu {
                                return Err(serde::de::Error::duplicate_field("cpu"));
                            }
                            seen_cpu = true;
                            f_cpu = map.next_value::<Option<crate::types::nodes_stats::OperatingSystemCpuStats>>()?;
                        }
                        "mem" => {
                            if seen_mem {
                                return Err(serde::de::Error::duplicate_field("mem"));
                            }
                            seen_mem = true;
                            f_mem = map.next_value::<Option<crate::types::nodes_stats::ExtendedMemoryStats>>()?;
                        }
                        "swap" => {
                            if seen_swap {
                                return Err(serde::de::Error::duplicate_field("swap"));
                            }
                            seen_swap = true;
                            f_swap =
                                map.next_value::<Option<crate::types::nodes_stats::MemoryStats>>()?;
                        }
                        "timestamp" => {
                            if seen_timestamp {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            seen_timestamp = true;
                            f_timestamp = map
                                .next_value::<Option<crate::types::common::EpochTimeUnitMillis>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(OperatingSystem {
                    cgroup: f_cgroup,
                    cpu: f_cpu,
                    mem: f_mem,
                    swap: f_swap,
                    timestamp: f_timestamp,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for OperatingSystem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cgroup {
            map.serialize_entry("cgroup", v)?;
        }
        if let Some(v) = &self.cpu {
            map.serialize_entry("cpu", v)?;
        }
        if let Some(v) = &self.mem {
            map.serialize_entry("mem", v)?;
        }
        if let Some(v) = &self.swap {
            map.serialize_entry("swap", v)?;
        }
        if let Some(v) = &self.timestamp {
            map.serialize_entry("timestamp", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct OperatingSystemCpuStats {
    pub load_average: Option<std::collections::BTreeMap<String, f64>>,
    pub percent: Option<crate::types::common::PercentageNumber>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for OperatingSystemCpuStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = OperatingSystemCpuStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a OperatingSystemCpuStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_load_average: Option<std::collections::BTreeMap<String, f64>> = None;
                let mut seen_load_average = false;
                let mut f_percent: Option<crate::types::common::PercentageNumber> = None;
                let mut seen_percent = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "load_average" => {
                            if seen_load_average {
                                return Err(serde::de::Error::duplicate_field("load_average"));
                            }
                            seen_load_average = true;
                            f_load_average = map
                                .next_value::<Option<std::collections::BTreeMap<String, f64>>>()?;
                        }
                        "percent" => {
                            if seen_percent {
                                return Err(serde::de::Error::duplicate_field("percent"));
                            }
                            seen_percent = true;
                            f_percent =
                                map.next_value::<Option<crate::types::common::PercentageNumber>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(OperatingSystemCpuStats {
                    load_average: f_load_average,
                    percent: f_percent,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for OperatingSystemCpuStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.load_average {
            map.serialize_entry("load_average", v)?;
        }
        if let Some(v) = &self.percent {
            map.serialize_entry("percent", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Pool {
    #[doc = "Most recent GC cycle stats for a particular memory pool."]
    pub last_gc_stats: Option<crate::types::nodes_stats::LastGcStats>,
    #[doc = "Maximum amount of memory available for use by the heap."]
    pub max: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Maximum amount of memory, in bytes, available for use by the heap."]
    pub max_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Largest amount of memory historically used by the heap."]
    pub peak_max: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Largest amount of memory, in bytes, historically used by the heap."]
    pub peak_max_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Largest amount of memory historically used by the heap."]
    pub peak_used: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Largest amount of memory, in bytes, historically used by the heap."]
    pub peak_used_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Memory used by the heap."]
    pub used: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Memory, in bytes, used by the heap."]
    pub used_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Pool {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Pool;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Pool object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_last_gc_stats: Option<crate::types::nodes_stats::LastGcStats> = None;
                let mut seen_last_gc_stats = false;
                let mut f_max: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_max = false;
                let mut f_max_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_max_in_bytes = false;
                let mut f_peak_max: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_peak_max = false;
                let mut f_peak_max_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_peak_max_in_bytes = false;
                let mut f_peak_used: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_peak_used = false;
                let mut f_peak_used_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_peak_used_in_bytes = false;
                let mut f_used: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_used = false;
                let mut f_used_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_used_in_bytes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "last_gc_stats" => {
                            if seen_last_gc_stats {
                                return Err(serde::de::Error::duplicate_field("last_gc_stats"));
                            }
                            seen_last_gc_stats = true;
                            f_last_gc_stats =
                                map.next_value::<Option<crate::types::nodes_stats::LastGcStats>>()?;
                        }
                        "max" => {
                            if seen_max {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            seen_max = true;
                            f_max = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "max_in_bytes" => {
                            if seen_max_in_bytes {
                                return Err(serde::de::Error::duplicate_field("max_in_bytes"));
                            }
                            seen_max_in_bytes = true;
                            f_max_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "peak_max" => {
                            if seen_peak_max {
                                return Err(serde::de::Error::duplicate_field("peak_max"));
                            }
                            seen_peak_max = true;
                            f_peak_max = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "peak_max_in_bytes" => {
                            if seen_peak_max_in_bytes {
                                return Err(serde::de::Error::duplicate_field("peak_max_in_bytes"));
                            }
                            seen_peak_max_in_bytes = true;
                            f_peak_max_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "peak_used" => {
                            if seen_peak_used {
                                return Err(serde::de::Error::duplicate_field("peak_used"));
                            }
                            seen_peak_used = true;
                            f_peak_used = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "peak_used_in_bytes" => {
                            if seen_peak_used_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "peak_used_in_bytes",
                                ));
                            }
                            seen_peak_used_in_bytes = true;
                            f_peak_used_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "used" => {
                            if seen_used {
                                return Err(serde::de::Error::duplicate_field("used"));
                            }
                            seen_used = true;
                            f_used = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "used_in_bytes" => {
                            if seen_used_in_bytes {
                                return Err(serde::de::Error::duplicate_field("used_in_bytes"));
                            }
                            seen_used_in_bytes = true;
                            f_used_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Pool {
                    last_gc_stats: f_last_gc_stats,
                    max: f_max,
                    max_in_bytes: f_max_in_bytes,
                    peak_max: f_peak_max,
                    peak_max_in_bytes: f_peak_max_in_bytes,
                    peak_used: f_peak_used,
                    peak_used_in_bytes: f_peak_used_in_bytes,
                    used: f_used,
                    used_in_bytes: f_used_in_bytes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Pool {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.last_gc_stats {
            map.serialize_entry("last_gc_stats", v)?;
        }
        if let Some(v) = &self.max {
            map.serialize_entry("max", v)?;
        }
        if let Some(v) = &self.max_in_bytes {
            map.serialize_entry("max_in_bytes", v)?;
        }
        if let Some(v) = &self.peak_max {
            map.serialize_entry("peak_max", v)?;
        }
        if let Some(v) = &self.peak_max_in_bytes {
            map.serialize_entry("peak_max_in_bytes", v)?;
        }
        if let Some(v) = &self.peak_used {
            map.serialize_entry("peak_used", v)?;
        }
        if let Some(v) = &self.peak_used_in_bytes {
            map.serialize_entry("peak_used_in_bytes", v)?;
        }
        if let Some(v) = &self.used {
            map.serialize_entry("used", v)?;
        }
        if let Some(v) = &self.used_in_bytes {
            map.serialize_entry("used_in_bytes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PressureMemory {
    pub all: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Memory consumed, in bytes, by indexing requests in the coordinating, primary, or replica stage."]
    pub all_in_bytes: Option<crate::types::common::ByteCount>,
    pub combined_coordinating_and_primary: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Memory consumed, in bytes, by indexing requests in the coordinating or primary stage. This value is not the sum of coordinating and primary as a node can reuse the coordinating memory if the primary stage is executed locally."]
    pub combined_coordinating_and_primary_in_bytes: Option<crate::types::common::ByteCount>,
    pub coordinating: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Memory consumed, in bytes, by indexing requests in the coordinating stage."]
    pub coordinating_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Number of indexing requests rejected in the coordinating stage."]
    pub coordinating_rejections: Option<i64>,
    pub primary: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Memory consumed, in bytes, by indexing requests in the primary stage."]
    pub primary_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Number of indexing requests rejected in the primary stage."]
    pub primary_rejections: Option<i64>,
    pub replica: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Memory consumed, in bytes, by indexing requests in the replica stage."]
    pub replica_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Number of indexing requests rejected in the replica stage."]
    pub replica_rejections: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PressureMemory {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PressureMemory;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PressureMemory object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_all: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_all = false;
                let mut f_all_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_all_in_bytes = false;
                let mut f_combined_coordinating_and_primary: Option<
                    crate::types::common::HumanReadableByteCount,
                > = None;
                let mut seen_combined_coordinating_and_primary = false;
                let mut f_combined_coordinating_and_primary_in_bytes: Option<
                    crate::types::common::ByteCount,
                > = None;
                let mut seen_combined_coordinating_and_primary_in_bytes = false;
                let mut f_coordinating: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_coordinating = false;
                let mut f_coordinating_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_coordinating_in_bytes = false;
                let mut f_coordinating_rejections: Option<i64> = None;
                let mut seen_coordinating_rejections = false;
                let mut f_primary: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_primary = false;
                let mut f_primary_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_primary_in_bytes = false;
                let mut f_primary_rejections: Option<i64> = None;
                let mut seen_primary_rejections = false;
                let mut f_replica: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_replica = false;
                let mut f_replica_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_replica_in_bytes = false;
                let mut f_replica_rejections: Option<i64> = None;
                let mut seen_replica_rejections = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "all" => {
                            if seen_all {
                                return Err(serde::de::Error::duplicate_field("all"));
                            }
                            seen_all = true;
                            f_all = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "all_in_bytes" => {
                            if seen_all_in_bytes {
                                return Err(serde::de::Error::duplicate_field("all_in_bytes"));
                            }
                            seen_all_in_bytes = true;
                            f_all_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "combined_coordinating_and_primary" => {
                            if seen_combined_coordinating_and_primary {
                                return Err(serde::de::Error::duplicate_field(
                                    "combined_coordinating_and_primary",
                                ));
                            }
                            seen_combined_coordinating_and_primary = true;
                            f_combined_coordinating_and_primary = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "combined_coordinating_and_primary_in_bytes" => {
                            if seen_combined_coordinating_and_primary_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "combined_coordinating_and_primary_in_bytes",
                                ));
                            }
                            seen_combined_coordinating_and_primary_in_bytes = true;
                            f_combined_coordinating_and_primary_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "coordinating" => {
                            if seen_coordinating {
                                return Err(serde::de::Error::duplicate_field("coordinating"));
                            }
                            seen_coordinating = true;
                            f_coordinating = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "coordinating_in_bytes" => {
                            if seen_coordinating_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "coordinating_in_bytes",
                                ));
                            }
                            seen_coordinating_in_bytes = true;
                            f_coordinating_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "coordinating_rejections" => {
                            if seen_coordinating_rejections {
                                return Err(serde::de::Error::duplicate_field(
                                    "coordinating_rejections",
                                ));
                            }
                            seen_coordinating_rejections = true;
                            f_coordinating_rejections = map.next_value::<Option<i64>>()?;
                        }
                        "primary" => {
                            if seen_primary {
                                return Err(serde::de::Error::duplicate_field("primary"));
                            }
                            seen_primary = true;
                            f_primary = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "primary_in_bytes" => {
                            if seen_primary_in_bytes {
                                return Err(serde::de::Error::duplicate_field("primary_in_bytes"));
                            }
                            seen_primary_in_bytes = true;
                            f_primary_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "primary_rejections" => {
                            if seen_primary_rejections {
                                return Err(serde::de::Error::duplicate_field(
                                    "primary_rejections",
                                ));
                            }
                            seen_primary_rejections = true;
                            f_primary_rejections = map.next_value::<Option<i64>>()?;
                        }
                        "replica" => {
                            if seen_replica {
                                return Err(serde::de::Error::duplicate_field("replica"));
                            }
                            seen_replica = true;
                            f_replica = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "replica_in_bytes" => {
                            if seen_replica_in_bytes {
                                return Err(serde::de::Error::duplicate_field("replica_in_bytes"));
                            }
                            seen_replica_in_bytes = true;
                            f_replica_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "replica_rejections" => {
                            if seen_replica_rejections {
                                return Err(serde::de::Error::duplicate_field(
                                    "replica_rejections",
                                ));
                            }
                            seen_replica_rejections = true;
                            f_replica_rejections = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PressureMemory {
                    all: f_all,
                    all_in_bytes: f_all_in_bytes,
                    combined_coordinating_and_primary: f_combined_coordinating_and_primary,
                    combined_coordinating_and_primary_in_bytes:
                        f_combined_coordinating_and_primary_in_bytes,
                    coordinating: f_coordinating,
                    coordinating_in_bytes: f_coordinating_in_bytes,
                    coordinating_rejections: f_coordinating_rejections,
                    primary: f_primary,
                    primary_in_bytes: f_primary_in_bytes,
                    primary_rejections: f_primary_rejections,
                    replica: f_replica,
                    replica_in_bytes: f_replica_in_bytes,
                    replica_rejections: f_replica_rejections,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PressureMemory {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.all {
            map.serialize_entry("all", v)?;
        }
        if let Some(v) = &self.all_in_bytes {
            map.serialize_entry("all_in_bytes", v)?;
        }
        if let Some(v) = &self.combined_coordinating_and_primary {
            map.serialize_entry("combined_coordinating_and_primary", v)?;
        }
        if let Some(v) = &self.combined_coordinating_and_primary_in_bytes {
            map.serialize_entry("combined_coordinating_and_primary_in_bytes", v)?;
        }
        if let Some(v) = &self.coordinating {
            map.serialize_entry("coordinating", v)?;
        }
        if let Some(v) = &self.coordinating_in_bytes {
            map.serialize_entry("coordinating_in_bytes", v)?;
        }
        if let Some(v) = &self.coordinating_rejections {
            map.serialize_entry("coordinating_rejections", v)?;
        }
        if let Some(v) = &self.primary {
            map.serialize_entry("primary", v)?;
        }
        if let Some(v) = &self.primary_in_bytes {
            map.serialize_entry("primary_in_bytes", v)?;
        }
        if let Some(v) = &self.primary_rejections {
            map.serialize_entry("primary_rejections", v)?;
        }
        if let Some(v) = &self.replica {
            map.serialize_entry("replica", v)?;
        }
        if let Some(v) = &self.replica_in_bytes {
            map.serialize_entry("replica_in_bytes", v)?;
        }
        if let Some(v) = &self.replica_rejections {
            map.serialize_entry("replica_rejections", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Process {
    pub cpu: Option<crate::types::nodes_stats::ProcessCpuStats>,
    #[doc = "Maximum number of file descriptors allowed on the system, or `-1` if not supported."]
    pub max_file_descriptors: Option<i64>,
    pub mem: Option<crate::types::nodes_stats::ProcessMemoryStats>,
    #[doc = "Number of opened file descriptors associated with the current or `-1` if not supported."]
    pub open_file_descriptors: Option<i64>,
    #[doc = "Last time the statistics were refreshed. Recorded in milliseconds since the Unix Epoch."]
    pub timestamp: Option<crate::types::common::EpochTimeUnitMillis>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Process {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Process;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Process object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cpu: Option<crate::types::nodes_stats::ProcessCpuStats> = None;
                let mut seen_cpu = false;
                let mut f_max_file_descriptors: Option<i64> = None;
                let mut seen_max_file_descriptors = false;
                let mut f_mem: Option<crate::types::nodes_stats::ProcessMemoryStats> = None;
                let mut seen_mem = false;
                let mut f_open_file_descriptors: Option<i64> = None;
                let mut seen_open_file_descriptors = false;
                let mut f_timestamp: Option<crate::types::common::EpochTimeUnitMillis> = None;
                let mut seen_timestamp = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cpu" => {
                            if seen_cpu {
                                return Err(serde::de::Error::duplicate_field("cpu"));
                            }
                            seen_cpu = true;
                            f_cpu = map
                                .next_value::<Option<crate::types::nodes_stats::ProcessCpuStats>>(
                                )?;
                        }
                        "max_file_descriptors" => {
                            if seen_max_file_descriptors {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_file_descriptors",
                                ));
                            }
                            seen_max_file_descriptors = true;
                            f_max_file_descriptors = map.next_value::<Option<i64>>()?;
                        }
                        "mem" => {
                            if seen_mem {
                                return Err(serde::de::Error::duplicate_field("mem"));
                            }
                            seen_mem = true;
                            f_mem = map.next_value::<Option<crate::types::nodes_stats::ProcessMemoryStats>>()?;
                        }
                        "open_file_descriptors" => {
                            if seen_open_file_descriptors {
                                return Err(serde::de::Error::duplicate_field(
                                    "open_file_descriptors",
                                ));
                            }
                            seen_open_file_descriptors = true;
                            f_open_file_descriptors = map.next_value::<Option<i64>>()?;
                        }
                        "timestamp" => {
                            if seen_timestamp {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            seen_timestamp = true;
                            f_timestamp = map
                                .next_value::<Option<crate::types::common::EpochTimeUnitMillis>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Process {
                    cpu: f_cpu,
                    max_file_descriptors: f_max_file_descriptors,
                    mem: f_mem,
                    open_file_descriptors: f_open_file_descriptors,
                    timestamp: f_timestamp,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Process {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cpu {
            map.serialize_entry("cpu", v)?;
        }
        if let Some(v) = &self.max_file_descriptors {
            map.serialize_entry("max_file_descriptors", v)?;
        }
        if let Some(v) = &self.mem {
            map.serialize_entry("mem", v)?;
        }
        if let Some(v) = &self.open_file_descriptors {
            map.serialize_entry("open_file_descriptors", v)?;
        }
        if let Some(v) = &self.timestamp {
            map.serialize_entry("timestamp", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ProcessCpuStats {
    pub percent: crate::types::common::PercentageNumber,
    pub total: Option<crate::types::common::Duration>,
    pub total_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ProcessCpuStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ProcessCpuStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ProcessCpuStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_percent: Option<crate::types::common::PercentageNumber> = None;
                let mut f_total: Option<crate::types::common::Duration> = None;
                let mut seen_total = false;
                let mut f_total_in_millis: Option<crate::types::common::DurationValueUnitMillis> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "percent" => {
                            if f_percent.is_some() {
                                return Err(serde::de::Error::duplicate_field("percent"));
                            }
                            f_percent =
                                Some(map.next_value::<crate::types::common::PercentageNumber>()?);
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "total_in_millis" => {
                            if f_total_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field("total_in_millis"));
                            }
                            f_total_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ProcessCpuStats {
                    percent: f_percent.ok_or_else(|| serde::de::Error::missing_field("percent"))?,
                    total: f_total,
                    total_in_millis: f_total_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("total_in_millis"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ProcessCpuStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("percent", &self.percent)?;
        if let Some(v) = &self.total {
            map.serialize_entry("total", v)?;
        }
        map.serialize_entry("total_in_millis", &self.total_in_millis)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ProcessMemoryStats {
    pub total_virtual: Option<crate::types::common::HumanReadableByteCount>,
    pub total_virtual_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ProcessMemoryStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ProcessMemoryStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ProcessMemoryStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_total_virtual: Option<crate::types::common::HumanReadableByteCount> =
                    None;
                let mut seen_total_virtual = false;
                let mut f_total_virtual_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_total_virtual_in_bytes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "total_virtual" => {
                            if seen_total_virtual {
                                return Err(serde::de::Error::duplicate_field("total_virtual"));
                            }
                            seen_total_virtual = true;
                            f_total_virtual = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "total_virtual_in_bytes" => {
                            if seen_total_virtual_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_virtual_in_bytes",
                                ));
                            }
                            seen_total_virtual_in_bytes = true;
                            f_total_virtual_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ProcessMemoryStats {
                    total_virtual: f_total_virtual,
                    total_virtual_in_bytes: f_total_virtual_in_bytes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ProcessMemoryStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.total_virtual {
            map.serialize_entry("total_virtual", v)?;
        }
        if let Some(v) = &self.total_virtual_in_bytes {
            map.serialize_entry("total_virtual_in_bytes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Processor {
    #[doc = "Number of documents transformed by the processor."]
    pub count: Option<i64>,
    #[doc = "Number of documents currently being transformed by the processor."]
    pub current: Option<i64>,
    #[doc = "Number of failed operations for the processor."]
    pub failed: Option<i64>,
    pub time: Option<crate::types::common::Duration>,
    pub time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Processor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Processor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Processor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_count: Option<i64> = None;
                let mut seen_count = false;
                let mut f_current: Option<i64> = None;
                let mut seen_current = false;
                let mut f_failed: Option<i64> = None;
                let mut seen_failed = false;
                let mut f_time: Option<crate::types::common::Duration> = None;
                let mut seen_time = false;
                let mut f_time_in_millis: Option<crate::types::common::DurationValueUnitMillis> =
                    None;
                let mut seen_time_in_millis = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "count" => {
                            if seen_count {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            seen_count = true;
                            f_count = map.next_value::<Option<i64>>()?;
                        }
                        "current" => {
                            if seen_current {
                                return Err(serde::de::Error::duplicate_field("current"));
                            }
                            seen_current = true;
                            f_current = map.next_value::<Option<i64>>()?;
                        }
                        "failed" => {
                            if seen_failed {
                                return Err(serde::de::Error::duplicate_field("failed"));
                            }
                            seen_failed = true;
                            f_failed = map.next_value::<Option<i64>>()?;
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
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Processor {
                    count: f_count,
                    current: f_current,
                    failed: f_failed,
                    time: f_time,
                    time_in_millis: f_time_in_millis,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Processor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.count {
            map.serialize_entry("count", v)?;
        }
        if let Some(v) = &self.current {
            map.serialize_entry("current", v)?;
        }
        if let Some(v) = &self.failed {
            map.serialize_entry("failed", v)?;
        }
        if let Some(v) = &self.time {
            map.serialize_entry("time", v)?;
        }
        if let Some(v) = &self.time_in_millis {
            map.serialize_entry("time_in_millis", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PublishedClusterStates {
    #[doc = "Number of compatible differences between published cluster states."]
    pub compatible_diffs: Option<i64>,
    #[doc = "Number of published cluster states."]
    pub full_states: Option<i64>,
    #[doc = "Number of incompatible differences between published cluster states."]
    pub incompatible_diffs: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PublishedClusterStates {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PublishedClusterStates;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PublishedClusterStates object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_compatible_diffs: Option<i64> = None;
                let mut seen_compatible_diffs = false;
                let mut f_full_states: Option<i64> = None;
                let mut seen_full_states = false;
                let mut f_incompatible_diffs: Option<i64> = None;
                let mut seen_incompatible_diffs = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "compatible_diffs" => {
                            if seen_compatible_diffs {
                                return Err(serde::de::Error::duplicate_field("compatible_diffs"));
                            }
                            seen_compatible_diffs = true;
                            f_compatible_diffs = map.next_value::<Option<i64>>()?;
                        }
                        "full_states" => {
                            if seen_full_states {
                                return Err(serde::de::Error::duplicate_field("full_states"));
                            }
                            seen_full_states = true;
                            f_full_states = map.next_value::<Option<i64>>()?;
                        }
                        "incompatible_diffs" => {
                            if seen_incompatible_diffs {
                                return Err(serde::de::Error::duplicate_field(
                                    "incompatible_diffs",
                                ));
                            }
                            seen_incompatible_diffs = true;
                            f_incompatible_diffs = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PublishedClusterStates {
                    compatible_diffs: f_compatible_diffs,
                    full_states: f_full_states,
                    incompatible_diffs: f_incompatible_diffs,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PublishedClusterStates {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.compatible_diffs {
            map.serialize_entry("compatible_diffs", v)?;
        }
        if let Some(v) = &self.full_states {
            map.serialize_entry("full_states", v)?;
        }
        if let Some(v) = &self.incompatible_diffs {
            map.serialize_entry("incompatible_diffs", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RemoteStoreStats {
    #[doc = "Timestamp for the last successful fetch of pinned timestamps."]
    pub last_successful_fetch_of_pinned_timestamps:
        Option<crate::types::common::EpochTimeUnitSeconds>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RemoteStoreStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RemoteStoreStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RemoteStoreStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_last_successful_fetch_of_pinned_timestamps: Option<
                    crate::types::common::EpochTimeUnitSeconds,
                > = None;
                let mut seen_last_successful_fetch_of_pinned_timestamps = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "last_successful_fetch_of_pinned_timestamps" => {
                            if seen_last_successful_fetch_of_pinned_timestamps {
                                return Err(serde::de::Error::duplicate_field(
                                    "last_successful_fetch_of_pinned_timestamps",
                                ));
                            }
                            seen_last_successful_fetch_of_pinned_timestamps = true;
                            f_last_successful_fetch_of_pinned_timestamps = map
                                .next_value::<Option<crate::types::common::EpochTimeUnitSeconds>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RemoteStoreStats {
                    last_successful_fetch_of_pinned_timestamps:
                        f_last_successful_fetch_of_pinned_timestamps,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RemoteStoreStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.last_successful_fetch_of_pinned_timestamps {
            map.serialize_entry("last_successful_fetch_of_pinned_timestamps", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Statistics snapshot about a repository."]
#[derive(Debug, Clone)]
pub struct RepositoryStatsSnapshot {
    #[doc = "The location configuration of the repository."]
    pub repository_location: Option<std::collections::BTreeMap<String, String>>,
    #[doc = "The name of the repository."]
    pub repository_name: Option<String>,
    #[doc = "The type of the repository."]
    pub repository_type: Option<String>,
    #[doc = "The count of requests by operation type (when detailed mode is disabled)."]
    pub request_counts: Option<std::collections::BTreeMap<String, i64>>,
    #[doc = "The count of failed requests by operation type (when detailed mode is enabled)."]
    pub request_failures_total: Option<std::collections::BTreeMap<String, i64>>,
    #[doc = "The total retry count by operation type (when detailed mode is enabled)."]
    pub request_retry_count_total: Option<std::collections::BTreeMap<String, i64>>,
    #[doc = "The count of successful requests by operation type (when detailed mode is enabled)."]
    pub request_success_total: Option<std::collections::BTreeMap<String, i64>>,
    #[doc = "The total request time in milliseconds by operation type (when detailed mode is enabled)."]
    pub request_time_in_millis: Option<std::collections::BTreeMap<String, i64>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RepositoryStatsSnapshot {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RepositoryStatsSnapshot;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RepositoryStatsSnapshot object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_repository_location: Option<std::collections::BTreeMap<String, String>> =
                    None;
                let mut seen_repository_location = false;
                let mut f_repository_name: Option<String> = None;
                let mut seen_repository_name = false;
                let mut f_repository_type: Option<String> = None;
                let mut seen_repository_type = false;
                let mut f_request_counts: Option<std::collections::BTreeMap<String, i64>> = None;
                let mut seen_request_counts = false;
                let mut f_request_failures_total: Option<std::collections::BTreeMap<String, i64>> =
                    None;
                let mut seen_request_failures_total = false;
                let mut f_request_retry_count_total: Option<
                    std::collections::BTreeMap<String, i64>,
                > = None;
                let mut seen_request_retry_count_total = false;
                let mut f_request_success_total: Option<std::collections::BTreeMap<String, i64>> =
                    None;
                let mut seen_request_success_total = false;
                let mut f_request_time_in_millis: Option<std::collections::BTreeMap<String, i64>> =
                    None;
                let mut seen_request_time_in_millis = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "repository_location" => {
                            if seen_repository_location {
                                return Err(serde::de::Error::duplicate_field(
                                    "repository_location",
                                ));
                            }
                            seen_repository_location = true;
                            f_repository_location = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "repository_name" => {
                            if seen_repository_name {
                                return Err(serde::de::Error::duplicate_field("repository_name"));
                            }
                            seen_repository_name = true;
                            f_repository_name = map.next_value::<Option<String>>()?;
                        }
                        "repository_type" => {
                            if seen_repository_type {
                                return Err(serde::de::Error::duplicate_field("repository_type"));
                            }
                            seen_repository_type = true;
                            f_repository_type = map.next_value::<Option<String>>()?;
                        }
                        "request_counts" => {
                            if seen_request_counts {
                                return Err(serde::de::Error::duplicate_field("request_counts"));
                            }
                            seen_request_counts = true;
                            f_request_counts = map
                                .next_value::<Option<std::collections::BTreeMap<String, i64>>>()?;
                        }
                        "request_failures_total" => {
                            if seen_request_failures_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "request_failures_total",
                                ));
                            }
                            seen_request_failures_total = true;
                            f_request_failures_total = map
                                .next_value::<Option<std::collections::BTreeMap<String, i64>>>()?;
                        }
                        "request_retry_count_total" => {
                            if seen_request_retry_count_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "request_retry_count_total",
                                ));
                            }
                            seen_request_retry_count_total = true;
                            f_request_retry_count_total = map
                                .next_value::<Option<std::collections::BTreeMap<String, i64>>>()?;
                        }
                        "request_success_total" => {
                            if seen_request_success_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "request_success_total",
                                ));
                            }
                            seen_request_success_total = true;
                            f_request_success_total = map
                                .next_value::<Option<std::collections::BTreeMap<String, i64>>>()?;
                        }
                        "request_time_in_millis" => {
                            if seen_request_time_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "request_time_in_millis",
                                ));
                            }
                            seen_request_time_in_millis = true;
                            f_request_time_in_millis = map
                                .next_value::<Option<std::collections::BTreeMap<String, i64>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RepositoryStatsSnapshot {
                    repository_location: f_repository_location,
                    repository_name: f_repository_name,
                    repository_type: f_repository_type,
                    request_counts: f_request_counts,
                    request_failures_total: f_request_failures_total,
                    request_retry_count_total: f_request_retry_count_total,
                    request_success_total: f_request_success_total,
                    request_time_in_millis: f_request_time_in_millis,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RepositoryStatsSnapshot {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.repository_location {
            map.serialize_entry("repository_location", v)?;
        }
        if let Some(v) = &self.repository_name {
            map.serialize_entry("repository_name", v)?;
        }
        if let Some(v) = &self.repository_type {
            map.serialize_entry("repository_type", v)?;
        }
        if let Some(v) = &self.request_counts {
            map.serialize_entry("request_counts", v)?;
        }
        if let Some(v) = &self.request_failures_total {
            map.serialize_entry("request_failures_total", v)?;
        }
        if let Some(v) = &self.request_retry_count_total {
            map.serialize_entry("request_retry_count_total", v)?;
        }
        if let Some(v) = &self.request_success_total {
            map.serialize_entry("request_success_total", v)?;
        }
        if let Some(v) = &self.request_time_in_millis {
            map.serialize_entry("request_time_in_millis", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ScriptCacheStats {
    pub contexts: Option<Vec<crate::types::nodes_stats::ScriptContextStats>>,
    pub sum: crate::types::nodes_stats::ScriptStats,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ScriptCacheStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ScriptCacheStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ScriptCacheStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_contexts: Option<Vec<crate::types::nodes_stats::ScriptContextStats>> =
                    None;
                let mut seen_contexts = false;
                let mut f_sum: Option<crate::types::nodes_stats::ScriptStats> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "contexts" => {
                            if seen_contexts {
                                return Err(serde::de::Error::duplicate_field("contexts"));
                            }
                            seen_contexts = true;
                            f_contexts = map.next_value::<Option<Vec<crate::types::nodes_stats::ScriptContextStats>>>()?;
                        }
                        "sum" => {
                            if f_sum.is_some() {
                                return Err(serde::de::Error::duplicate_field("sum"));
                            }
                            f_sum =
                                Some(map.next_value::<crate::types::nodes_stats::ScriptStats>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ScriptCacheStats {
                    contexts: f_contexts,
                    sum: f_sum.ok_or_else(|| serde::de::Error::missing_field("sum"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ScriptCacheStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.contexts {
            map.serialize_entry("contexts", v)?;
        }
        map.serialize_entry("sum", &self.sum)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ScriptContextStats {
    #[doc = "Total number of times the script cache has evicted old data."]
    pub cache_evictions: i64,
    #[doc = "Total number of times the script compilation circuit breaker has limited inline script compilations."]
    pub compilation_limit_triggered: i64,
    #[doc = "Total number of inline script compilations performed by the node."]
    pub compilations: i64,
    pub context: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ScriptContextStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ScriptContextStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ScriptContextStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cache_evictions: Option<i64> = None;
                let mut f_compilation_limit_triggered: Option<i64> = None;
                let mut f_compilations: Option<i64> = None;
                let mut f_context: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cache_evictions" => {
                            if f_cache_evictions.is_some() {
                                return Err(serde::de::Error::duplicate_field("cache_evictions"));
                            }
                            f_cache_evictions = Some(map.next_value::<i64>()?);
                        }
                        "compilation_limit_triggered" => {
                            if f_compilation_limit_triggered.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "compilation_limit_triggered",
                                ));
                            }
                            f_compilation_limit_triggered = Some(map.next_value::<i64>()?);
                        }
                        "compilations" => {
                            if f_compilations.is_some() {
                                return Err(serde::de::Error::duplicate_field("compilations"));
                            }
                            f_compilations = Some(map.next_value::<i64>()?);
                        }
                        "context" => {
                            if f_context.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            f_context = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ScriptContextStats {
                    cache_evictions: f_cache_evictions
                        .ok_or_else(|| serde::de::Error::missing_field("cache_evictions"))?,
                    compilation_limit_triggered: f_compilation_limit_triggered.ok_or_else(
                        || serde::de::Error::missing_field("compilation_limit_triggered"),
                    )?,
                    compilations: f_compilations
                        .ok_or_else(|| serde::de::Error::missing_field("compilations"))?,
                    context: f_context.ok_or_else(|| serde::de::Error::missing_field("context"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ScriptContextStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("cache_evictions", &self.cache_evictions)?;
        map.serialize_entry(
            "compilation_limit_triggered",
            &self.compilation_limit_triggered,
        )?;
        map.serialize_entry("compilations", &self.compilations)?;
        map.serialize_entry("context", &self.context)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ScriptStats {
    #[doc = "Total number of times the script cache has evicted old data."]
    pub cache_evictions: i64,
    #[doc = "Total number of times the script compilation circuit breaker has limited inline script compilations."]
    pub compilation_limit_triggered: i64,
    #[doc = "Total number of inline script compilations performed by the node."]
    pub compilations: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ScriptStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ScriptStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ScriptStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cache_evictions: Option<i64> = None;
                let mut f_compilation_limit_triggered: Option<i64> = None;
                let mut f_compilations: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cache_evictions" => {
                            if f_cache_evictions.is_some() {
                                return Err(serde::de::Error::duplicate_field("cache_evictions"));
                            }
                            f_cache_evictions = Some(map.next_value::<i64>()?);
                        }
                        "compilation_limit_triggered" => {
                            if f_compilation_limit_triggered.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "compilation_limit_triggered",
                                ));
                            }
                            f_compilation_limit_triggered = Some(map.next_value::<i64>()?);
                        }
                        "compilations" => {
                            if f_compilations.is_some() {
                                return Err(serde::de::Error::duplicate_field("compilations"));
                            }
                            f_compilations = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ScriptStats {
                    cache_evictions: f_cache_evictions
                        .ok_or_else(|| serde::de::Error::missing_field("cache_evictions"))?,
                    compilation_limit_triggered: f_compilation_limit_triggered.ok_or_else(
                        || serde::de::Error::missing_field("compilation_limit_triggered"),
                    )?,
                    compilations: f_compilations
                        .ok_or_else(|| serde::de::Error::missing_field("compilations"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ScriptStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("cache_evictions", &self.cache_evictions)?;
        map.serialize_entry(
            "compilation_limit_triggered",
            &self.compilation_limit_triggered,
        )?;
        map.serialize_entry("compilations", &self.compilations)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardAdmissionControlStats {
    pub global_cpu_usage: Option<crate::types::nodes_stats::UsageStats>,
    pub global_io_usage: Option<crate::types::nodes_stats::UsageStats>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardAdmissionControlStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardAdmissionControlStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardAdmissionControlStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_global_cpu_usage: Option<crate::types::nodes_stats::UsageStats> = None;
                let mut seen_global_cpu_usage = false;
                let mut f_global_io_usage: Option<crate::types::nodes_stats::UsageStats> = None;
                let mut seen_global_io_usage = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "global_cpu_usage" => {
                            if seen_global_cpu_usage {
                                return Err(serde::de::Error::duplicate_field("global_cpu_usage"));
                            }
                            seen_global_cpu_usage = true;
                            f_global_cpu_usage =
                                map.next_value::<Option<crate::types::nodes_stats::UsageStats>>()?;
                        }
                        "global_io_usage" => {
                            if seen_global_io_usage {
                                return Err(serde::de::Error::duplicate_field("global_io_usage"));
                            }
                            seen_global_io_usage = true;
                            f_global_io_usage =
                                map.next_value::<Option<crate::types::nodes_stats::UsageStats>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardAdmissionControlStats {
                    global_cpu_usage: f_global_cpu_usage,
                    global_io_usage: f_global_io_usage,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardAdmissionControlStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.global_cpu_usage {
            map.serialize_entry("global_cpu_usage", v)?;
        }
        if let Some(v) = &self.global_io_usage {
            map.serialize_entry("global_io_usage", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardClusterManagerThrottlingStats {
    pub stats: Option<crate::types::nodes_stats::ShardClusterManagerThrottlingStatsDetail>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardClusterManagerThrottlingStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardClusterManagerThrottlingStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardClusterManagerThrottlingStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_stats: Option<
                    crate::types::nodes_stats::ShardClusterManagerThrottlingStatsDetail,
                > = None;
                let mut seen_stats = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "stats" => {
                            if seen_stats {
                                return Err(serde::de::Error::duplicate_field("stats"));
                            }
                            seen_stats = true;
                            f_stats = map.next_value::<Option<
                                crate::types::nodes_stats::ShardClusterManagerThrottlingStatsDetail,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardClusterManagerThrottlingStats {
                    stats: f_stats,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardClusterManagerThrottlingStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
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
pub struct ShardClusterManagerThrottlingStatsDetail {
    pub throttled_tasks_per_task_type: std::collections::BTreeMap<String, i64>,
    pub total_throttled_tasks: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardClusterManagerThrottlingStatsDetail {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardClusterManagerThrottlingStatsDetail;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardClusterManagerThrottlingStatsDetail object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_throttled_tasks_per_task_type: Option<
                    std::collections::BTreeMap<String, i64>,
                > = None;
                let mut f_total_throttled_tasks: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "throttled_tasks_per_task_type" => {
                            if f_throttled_tasks_per_task_type.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "throttled_tasks_per_task_type",
                                ));
                            }
                            f_throttled_tasks_per_task_type =
                                Some(map.next_value::<std::collections::BTreeMap<String, i64>>()?);
                        }
                        "total_throttled_tasks" => {
                            if f_total_throttled_tasks.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_throttled_tasks",
                                ));
                            }
                            f_total_throttled_tasks = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardClusterManagerThrottlingStatsDetail {
                    throttled_tasks_per_task_type: f_throttled_tasks_per_task_type.ok_or_else(
                        || serde::de::Error::missing_field("throttled_tasks_per_task_type"),
                    )?,
                    total_throttled_tasks: f_total_throttled_tasks
                        .ok_or_else(|| serde::de::Error::missing_field("total_throttled_tasks"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardClusterManagerThrottlingStatsDetail {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry(
            "throttled_tasks_per_task_type",
            &self.throttled_tasks_per_task_type,
        )?;
        map.serialize_entry("total_throttled_tasks", &self.total_throttled_tasks)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardIndexingPressurePerShardIndexingStats {
    pub coordinating_count: i64,
    pub coordinating_time_in_millis: crate::types::common::DurationValueUnitMillis,
    pub primary_count: i64,
    pub primary_time_in_millis: crate::types::common::DurationValueUnitMillis,
    pub replica_count: i64,
    pub replica_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardIndexingPressurePerShardIndexingStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardIndexingPressurePerShardIndexingStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardIndexingPressurePerShardIndexingStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_coordinating_count: Option<i64> = None;
                let mut f_coordinating_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_primary_count: Option<i64> = None;
                let mut f_primary_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_replica_count: Option<i64> = None;
                let mut f_replica_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "coordinating_count" => {
                            if f_coordinating_count.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "coordinating_count",
                                ));
                            }
                            f_coordinating_count = Some(map.next_value::<i64>()?);
                        }
                        "coordinating_time_in_millis" => {
                            if f_coordinating_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "coordinating_time_in_millis",
                                ));
                            }
                            f_coordinating_time_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "primary_count" => {
                            if f_primary_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("primary_count"));
                            }
                            f_primary_count = Some(map.next_value::<i64>()?);
                        }
                        "primary_time_in_millis" => {
                            if f_primary_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "primary_time_in_millis",
                                ));
                            }
                            f_primary_time_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "replica_count" => {
                            if f_replica_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("replica_count"));
                            }
                            f_replica_count = Some(map.next_value::<i64>()?);
                        }
                        "replica_time_in_millis" => {
                            if f_replica_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "replica_time_in_millis",
                                ));
                            }
                            f_replica_time_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardIndexingPressurePerShardIndexingStats {
                    coordinating_count: f_coordinating_count
                        .ok_or_else(|| serde::de::Error::missing_field("coordinating_count"))?,
                    coordinating_time_in_millis: f_coordinating_time_in_millis.ok_or_else(
                        || serde::de::Error::missing_field("coordinating_time_in_millis"),
                    )?,
                    primary_count: f_primary_count
                        .ok_or_else(|| serde::de::Error::missing_field("primary_count"))?,
                    primary_time_in_millis: f_primary_time_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("primary_time_in_millis"))?,
                    replica_count: f_replica_count
                        .ok_or_else(|| serde::de::Error::missing_field("replica_count"))?,
                    replica_time_in_millis: f_replica_time_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("replica_time_in_millis"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardIndexingPressurePerShardIndexingStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("coordinating_count", &self.coordinating_count)?;
        map.serialize_entry(
            "coordinating_time_in_millis",
            &self.coordinating_time_in_millis,
        )?;
        map.serialize_entry("primary_count", &self.primary_count)?;
        map.serialize_entry("primary_time_in_millis", &self.primary_time_in_millis)?;
        map.serialize_entry("replica_count", &self.replica_count)?;
        map.serialize_entry("replica_time_in_millis", &self.replica_time_in_millis)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardIndexingPressurePerShardLastSuccessfulTimestamp {
    pub coordinating_last_successful_request_timestamp_in_millis:
        crate::types::common::EpochTimeUnitMillis,
    pub primary_last_successful_request_timestamp_in_millis:
        crate::types::common::EpochTimeUnitMillis,
    pub replica_last_successful_request_timestamp_in_millis:
        crate::types::common::EpochTimeUnitMillis,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardIndexingPressurePerShardLastSuccessfulTimestamp {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardIndexingPressurePerShardLastSuccessfulTimestamp;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardIndexingPressurePerShardLastSuccessfulTimestamp object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_coordinating_last_successful_request_timestamp_in_millis: Option<
                    crate::types::common::EpochTimeUnitMillis,
                > = None;
                let mut f_primary_last_successful_request_timestamp_in_millis: Option<
                    crate::types::common::EpochTimeUnitMillis,
                > = None;
                let mut f_replica_last_successful_request_timestamp_in_millis: Option<
                    crate::types::common::EpochTimeUnitMillis,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "coordinating_last_successful_request_timestamp_in_millis" => {
                            if f_coordinating_last_successful_request_timestamp_in_millis.is_some()
                            {
                                return Err(serde::de::Error::duplicate_field(
                                    "coordinating_last_successful_request_timestamp_in_millis",
                                ));
                            }
                            f_coordinating_last_successful_request_timestamp_in_millis = Some(
                                map.next_value::<crate::types::common::EpochTimeUnitMillis>()?,
                            );
                        }
                        "primary_last_successful_request_timestamp_in_millis" => {
                            if f_primary_last_successful_request_timestamp_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "primary_last_successful_request_timestamp_in_millis",
                                ));
                            }
                            f_primary_last_successful_request_timestamp_in_millis = Some(
                                map.next_value::<crate::types::common::EpochTimeUnitMillis>()?,
                            );
                        }
                        "replica_last_successful_request_timestamp_in_millis" => {
                            if f_replica_last_successful_request_timestamp_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "replica_last_successful_request_timestamp_in_millis",
                                ));
                            }
                            f_replica_last_successful_request_timestamp_in_millis = Some(
                                map.next_value::<crate::types::common::EpochTimeUnitMillis>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardIndexingPressurePerShardLastSuccessfulTimestamp {
                    coordinating_last_successful_request_timestamp_in_millis:
                        f_coordinating_last_successful_request_timestamp_in_millis.ok_or_else(
                            || {
                                serde::de::Error::missing_field(
                                    "coordinating_last_successful_request_timestamp_in_millis",
                                )
                            },
                        )?,
                    primary_last_successful_request_timestamp_in_millis:
                        f_primary_last_successful_request_timestamp_in_millis.ok_or_else(|| {
                            serde::de::Error::missing_field(
                                "primary_last_successful_request_timestamp_in_millis",
                            )
                        })?,
                    replica_last_successful_request_timestamp_in_millis:
                        f_replica_last_successful_request_timestamp_in_millis.ok_or_else(|| {
                            serde::de::Error::missing_field(
                                "replica_last_successful_request_timestamp_in_millis",
                            )
                        })?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardIndexingPressurePerShardLastSuccessfulTimestamp {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry(
            "coordinating_last_successful_request_timestamp_in_millis",
            &self.coordinating_last_successful_request_timestamp_in_millis,
        )?;
        map.serialize_entry(
            "primary_last_successful_request_timestamp_in_millis",
            &self.primary_last_successful_request_timestamp_in_millis,
        )?;
        map.serialize_entry(
            "replica_last_successful_request_timestamp_in_millis",
            &self.replica_last_successful_request_timestamp_in_millis,
        )?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardIndexingPressurePerShardMemoryAllocationCurrentStats {
    pub current_coordinating_and_primary_bytes: crate::types::common::ByteCount,
    pub current_replica_bytes: crate::types::common::ByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardIndexingPressurePerShardMemoryAllocationCurrentStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardIndexingPressurePerShardMemoryAllocationCurrentStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardIndexingPressurePerShardMemoryAllocationCurrentStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_current_coordinating_and_primary_bytes: Option<
                    crate::types::common::ByteCount,
                > = None;
                let mut f_current_replica_bytes: Option<crate::types::common::ByteCount> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "current_coordinating_and_primary_bytes" => {
                            if f_current_coordinating_and_primary_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "current_coordinating_and_primary_bytes",
                                ));
                            }
                            f_current_coordinating_and_primary_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "current_replica_bytes" => {
                            if f_current_replica_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "current_replica_bytes",
                                ));
                            }
                            f_current_replica_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardIndexingPressurePerShardMemoryAllocationCurrentStats {
                    current_coordinating_and_primary_bytes:
                        f_current_coordinating_and_primary_bytes.ok_or_else(|| {
                            serde::de::Error::missing_field(
                                "current_coordinating_and_primary_bytes",
                            )
                        })?,
                    current_replica_bytes: f_current_replica_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("current_replica_bytes"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardIndexingPressurePerShardMemoryAllocationCurrentStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry(
            "current_coordinating_and_primary_bytes",
            &self.current_coordinating_and_primary_bytes,
        )?;
        map.serialize_entry("current_replica_bytes", &self.current_replica_bytes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardIndexingPressurePerShardMemoryAllocationLimitStats {
    pub current_coordinating_and_primary_limits_in_bytes: crate::types::common::ByteCount,
    pub current_replica_limits_in_bytes: crate::types::common::ByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardIndexingPressurePerShardMemoryAllocationLimitStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardIndexingPressurePerShardMemoryAllocationLimitStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardIndexingPressurePerShardMemoryAllocationLimitStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_current_coordinating_and_primary_limits_in_bytes: Option<
                    crate::types::common::ByteCount,
                > = None;
                let mut f_current_replica_limits_in_bytes: Option<crate::types::common::ByteCount> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "current_coordinating_and_primary_limits_in_bytes" => {
                            if f_current_coordinating_and_primary_limits_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "current_coordinating_and_primary_limits_in_bytes",
                                ));
                            }
                            f_current_coordinating_and_primary_limits_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "current_replica_limits_in_bytes" => {
                            if f_current_replica_limits_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "current_replica_limits_in_bytes",
                                ));
                            }
                            f_current_replica_limits_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardIndexingPressurePerShardMemoryAllocationLimitStats {
                    current_coordinating_and_primary_limits_in_bytes:
                        f_current_coordinating_and_primary_limits_in_bytes.ok_or_else(|| {
                            serde::de::Error::missing_field(
                                "current_coordinating_and_primary_limits_in_bytes",
                            )
                        })?,
                    current_replica_limits_in_bytes: f_current_replica_limits_in_bytes.ok_or_else(
                        || serde::de::Error::missing_field("current_replica_limits_in_bytes"),
                    )?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardIndexingPressurePerShardMemoryAllocationLimitStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry(
            "current_coordinating_and_primary_limits_in_bytes",
            &self.current_coordinating_and_primary_limits_in_bytes,
        )?;
        map.serialize_entry(
            "current_replica_limits_in_bytes",
            &self.current_replica_limits_in_bytes,
        )?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardIndexingPressurePerShardMemoryAllocationStats {
    pub current:
        crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryAllocationCurrentStats,
    pub limit: crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryAllocationLimitStats,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardIndexingPressurePerShardMemoryAllocationStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardIndexingPressurePerShardMemoryAllocationStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardIndexingPressurePerShardMemoryAllocationStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_current: Option<crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryAllocationCurrentStats> = None;
                let mut f_limit: Option<crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryAllocationLimitStats> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "current" => {
                            if f_current.is_some() {
                                return Err(serde::de::Error::duplicate_field("current"));
                            }
                            f_current = Some(map.next_value::<crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryAllocationCurrentStats>()?);
                        }
                        "limit" => {
                            if f_limit.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            f_limit = Some(map.next_value::<crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryAllocationLimitStats>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardIndexingPressurePerShardMemoryAllocationStats {
                    current: f_current.ok_or_else(|| serde::de::Error::missing_field("current"))?,
                    limit: f_limit.ok_or_else(|| serde::de::Error::missing_field("limit"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardIndexingPressurePerShardMemoryAllocationStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("current", &self.current)?;
        map.serialize_entry("limit", &self.limit)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardIndexingPressurePerShardMemoryStats {
    pub current: crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryStatsDetails,
    pub total: crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryStatsDetails,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardIndexingPressurePerShardMemoryStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardIndexingPressurePerShardMemoryStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardIndexingPressurePerShardMemoryStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_current: Option<
                    crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryStatsDetails,
                > = None;
                let mut f_total: Option<
                    crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryStatsDetails,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "current" => {
                            if f_current.is_some() {
                                return Err(serde::de::Error::duplicate_field("current"));
                            }
                            f_current = Some(map.next_value::<crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryStatsDetails>()?);
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(map.next_value::<crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryStatsDetails>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardIndexingPressurePerShardMemoryStats {
                    current: f_current.ok_or_else(|| serde::de::Error::missing_field("current"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardIndexingPressurePerShardMemoryStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("current", &self.current)?;
        map.serialize_entry("total", &self.total)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardIndexingPressurePerShardMemoryStatsDetails {
    pub coordinating: Option<crate::types::common::HumanReadableByteCount>,
    pub coordinating_in_bytes: crate::types::common::ByteCount,
    pub primary: Option<crate::types::common::HumanReadableByteCount>,
    pub primary_in_bytes: crate::types::common::ByteCount,
    pub replica: Option<crate::types::common::HumanReadableByteCount>,
    pub replica_in_bytes: crate::types::common::ByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardIndexingPressurePerShardMemoryStatsDetails {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardIndexingPressurePerShardMemoryStatsDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardIndexingPressurePerShardMemoryStatsDetails object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_coordinating: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_coordinating = false;
                let mut f_coordinating_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_primary: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_primary = false;
                let mut f_primary_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_replica: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_replica = false;
                let mut f_replica_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "coordinating" => {
                            if seen_coordinating {
                                return Err(serde::de::Error::duplicate_field("coordinating"));
                            }
                            seen_coordinating = true;
                            f_coordinating = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "coordinating_in_bytes" => {
                            if f_coordinating_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "coordinating_in_bytes",
                                ));
                            }
                            f_coordinating_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "primary" => {
                            if seen_primary {
                                return Err(serde::de::Error::duplicate_field("primary"));
                            }
                            seen_primary = true;
                            f_primary = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "primary_in_bytes" => {
                            if f_primary_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("primary_in_bytes"));
                            }
                            f_primary_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "replica" => {
                            if seen_replica {
                                return Err(serde::de::Error::duplicate_field("replica"));
                            }
                            seen_replica = true;
                            f_replica = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "replica_in_bytes" => {
                            if f_replica_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("replica_in_bytes"));
                            }
                            f_replica_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardIndexingPressurePerShardMemoryStatsDetails {
                    coordinating: f_coordinating,
                    coordinating_in_bytes: f_coordinating_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("coordinating_in_bytes"))?,
                    primary: f_primary,
                    primary_in_bytes: f_primary_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("primary_in_bytes"))?,
                    replica: f_replica,
                    replica_in_bytes: f_replica_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("replica_in_bytes"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardIndexingPressurePerShardMemoryStatsDetails {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.coordinating {
            map.serialize_entry("coordinating", v)?;
        }
        map.serialize_entry("coordinating_in_bytes", &self.coordinating_in_bytes)?;
        if let Some(v) = &self.primary {
            map.serialize_entry("primary", v)?;
        }
        map.serialize_entry("primary_in_bytes", &self.primary_in_bytes)?;
        if let Some(v) = &self.replica {
            map.serialize_entry("replica", v)?;
        }
        map.serialize_entry("replica_in_bytes", &self.replica_in_bytes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardIndexingPressurePerShardRejectionCoordinatingStats {
    pub breakup: Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>,
    pub breakup_shadow_mode:
        Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>,
    pub coordinating_rejections: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardIndexingPressurePerShardRejectionCoordinatingStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardIndexingPressurePerShardRejectionCoordinatingStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardIndexingPressurePerShardRejectionCoordinatingStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_breakup: Option<
                    crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats,
                > = None;
                let mut seen_breakup = false;
                let mut f_breakup_shadow_mode: Option<
                    crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats,
                > = None;
                let mut seen_breakup_shadow_mode = false;
                let mut f_coordinating_rejections: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "breakup" => {
                            if seen_breakup {
                                return Err(serde::de::Error::duplicate_field("breakup"));
                            }
                            seen_breakup = true;
                            f_breakup = map.next_value::<Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>>()?;
                        }
                        "breakup_shadow_mode" => {
                            if seen_breakup_shadow_mode {
                                return Err(serde::de::Error::duplicate_field(
                                    "breakup_shadow_mode",
                                ));
                            }
                            seen_breakup_shadow_mode = true;
                            f_breakup_shadow_mode = map.next_value::<Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>>()?;
                        }
                        "coordinating_rejections" => {
                            if f_coordinating_rejections.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "coordinating_rejections",
                                ));
                            }
                            f_coordinating_rejections = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardIndexingPressurePerShardRejectionCoordinatingStats {
                    breakup: f_breakup,
                    breakup_shadow_mode: f_breakup_shadow_mode,
                    coordinating_rejections: f_coordinating_rejections.ok_or_else(|| {
                        serde::de::Error::missing_field("coordinating_rejections")
                    })?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardIndexingPressurePerShardRejectionCoordinatingStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.breakup {
            map.serialize_entry("breakup", v)?;
        }
        if let Some(v) = &self.breakup_shadow_mode {
            map.serialize_entry("breakup_shadow_mode", v)?;
        }
        map.serialize_entry("coordinating_rejections", &self.coordinating_rejections)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardIndexingPressurePerShardRejectionPrimaryStats {
    pub breakup: Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>,
    pub breakup_shadow_mode:
        Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>,
    pub primary_rejections: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardIndexingPressurePerShardRejectionPrimaryStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardIndexingPressurePerShardRejectionPrimaryStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardIndexingPressurePerShardRejectionPrimaryStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_breakup: Option<
                    crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats,
                > = None;
                let mut seen_breakup = false;
                let mut f_breakup_shadow_mode: Option<
                    crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats,
                > = None;
                let mut seen_breakup_shadow_mode = false;
                let mut f_primary_rejections: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "breakup" => {
                            if seen_breakup {
                                return Err(serde::de::Error::duplicate_field("breakup"));
                            }
                            seen_breakup = true;
                            f_breakup = map.next_value::<Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>>()?;
                        }
                        "breakup_shadow_mode" => {
                            if seen_breakup_shadow_mode {
                                return Err(serde::de::Error::duplicate_field(
                                    "breakup_shadow_mode",
                                ));
                            }
                            seen_breakup_shadow_mode = true;
                            f_breakup_shadow_mode = map.next_value::<Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>>()?;
                        }
                        "primary_rejections" => {
                            if f_primary_rejections.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "primary_rejections",
                                ));
                            }
                            f_primary_rejections = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardIndexingPressurePerShardRejectionPrimaryStats {
                    breakup: f_breakup,
                    breakup_shadow_mode: f_breakup_shadow_mode,
                    primary_rejections: f_primary_rejections
                        .ok_or_else(|| serde::de::Error::missing_field("primary_rejections"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardIndexingPressurePerShardRejectionPrimaryStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.breakup {
            map.serialize_entry("breakup", v)?;
        }
        if let Some(v) = &self.breakup_shadow_mode {
            map.serialize_entry("breakup_shadow_mode", v)?;
        }
        map.serialize_entry("primary_rejections", &self.primary_rejections)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardIndexingPressurePerShardRejectionReplicaStats {
    pub breakup: Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>,
    pub breakup_shadow_mode:
        Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>,
    pub replica_rejections: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardIndexingPressurePerShardRejectionReplicaStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardIndexingPressurePerShardRejectionReplicaStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardIndexingPressurePerShardRejectionReplicaStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_breakup: Option<
                    crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats,
                > = None;
                let mut seen_breakup = false;
                let mut f_breakup_shadow_mode: Option<
                    crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats,
                > = None;
                let mut seen_breakup_shadow_mode = false;
                let mut f_replica_rejections: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "breakup" => {
                            if seen_breakup {
                                return Err(serde::de::Error::duplicate_field("breakup"));
                            }
                            seen_breakup = true;
                            f_breakup = map.next_value::<Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>>()?;
                        }
                        "breakup_shadow_mode" => {
                            if seen_breakup_shadow_mode {
                                return Err(serde::de::Error::duplicate_field(
                                    "breakup_shadow_mode",
                                ));
                            }
                            seen_breakup_shadow_mode = true;
                            f_breakup_shadow_mode = map.next_value::<Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>>()?;
                        }
                        "replica_rejections" => {
                            if f_replica_rejections.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "replica_rejections",
                                ));
                            }
                            f_replica_rejections = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardIndexingPressurePerShardRejectionReplicaStats {
                    breakup: f_breakup,
                    breakup_shadow_mode: f_breakup_shadow_mode,
                    replica_rejections: f_replica_rejections
                        .ok_or_else(|| serde::de::Error::missing_field("replica_rejections"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardIndexingPressurePerShardRejectionReplicaStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.breakup {
            map.serialize_entry("breakup", v)?;
        }
        if let Some(v) = &self.breakup_shadow_mode {
            map.serialize_entry("breakup_shadow_mode", v)?;
        }
        map.serialize_entry("replica_rejections", &self.replica_rejections)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardIndexingPressurePerShardRejectionStats {
    pub coordinating:
        crate::types::nodes_stats::ShardIndexingPressurePerShardRejectionCoordinatingStats,
    pub primary: crate::types::nodes_stats::ShardIndexingPressurePerShardRejectionPrimaryStats,
    pub replica: crate::types::nodes_stats::ShardIndexingPressurePerShardRejectionReplicaStats,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardIndexingPressurePerShardRejectionStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardIndexingPressurePerShardRejectionStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardIndexingPressurePerShardRejectionStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_coordinating: Option<crate::types::nodes_stats::ShardIndexingPressurePerShardRejectionCoordinatingStats> = None;
                let mut f_primary: Option<
                    crate::types::nodes_stats::ShardIndexingPressurePerShardRejectionPrimaryStats,
                > = None;
                let mut f_replica: Option<
                    crate::types::nodes_stats::ShardIndexingPressurePerShardRejectionReplicaStats,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "coordinating" => {
                            if f_coordinating.is_some() {
                                return Err(serde::de::Error::duplicate_field("coordinating"));
                            }
                            f_coordinating = Some(map.next_value::<crate::types::nodes_stats::ShardIndexingPressurePerShardRejectionCoordinatingStats>()?);
                        }
                        "primary" => {
                            if f_primary.is_some() {
                                return Err(serde::de::Error::duplicate_field("primary"));
                            }
                            f_primary = Some(map.next_value::<crate::types::nodes_stats::ShardIndexingPressurePerShardRejectionPrimaryStats>()?);
                        }
                        "replica" => {
                            if f_replica.is_some() {
                                return Err(serde::de::Error::duplicate_field("replica"));
                            }
                            f_replica = Some(map.next_value::<crate::types::nodes_stats::ShardIndexingPressurePerShardRejectionReplicaStats>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardIndexingPressurePerShardRejectionStats {
                    coordinating: f_coordinating
                        .ok_or_else(|| serde::de::Error::missing_field("coordinating"))?,
                    primary: f_primary.ok_or_else(|| serde::de::Error::missing_field("primary"))?,
                    replica: f_replica.ok_or_else(|| serde::de::Error::missing_field("replica"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardIndexingPressurePerShardRejectionStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("coordinating", &self.coordinating)?;
        map.serialize_entry("primary", &self.primary)?;
        map.serialize_entry("replica", &self.replica)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardIndexingPressurePerShardStats {
    pub indexing: crate::types::nodes_stats::ShardIndexingPressurePerShardIndexingStats,
    pub last_successful_timestamp:
        crate::types::nodes_stats::ShardIndexingPressurePerShardLastSuccessfulTimestamp,
    pub memory: crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryStats,
    pub memory_allocation:
        crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryAllocationStats,
    pub rejection: crate::types::nodes_stats::ShardIndexingPressurePerShardRejectionStats,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardIndexingPressurePerShardStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardIndexingPressurePerShardStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardIndexingPressurePerShardStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_indexing: Option<
                    crate::types::nodes_stats::ShardIndexingPressurePerShardIndexingStats,
                > = None;
                let mut f_last_successful_timestamp: Option<
                    crate::types::nodes_stats::ShardIndexingPressurePerShardLastSuccessfulTimestamp,
                > = None;
                let mut f_memory: Option<
                    crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryStats,
                > = None;
                let mut f_memory_allocation: Option<
                    crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryAllocationStats,
                > = None;
                let mut f_rejection: Option<
                    crate::types::nodes_stats::ShardIndexingPressurePerShardRejectionStats,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "indexing" => {
                            if f_indexing.is_some() {
                                return Err(serde::de::Error::duplicate_field("indexing"));
                            }
                            f_indexing = Some(map.next_value::<crate::types::nodes_stats::ShardIndexingPressurePerShardIndexingStats>()?);
                        }
                        "last_successful_timestamp" => {
                            if f_last_successful_timestamp.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "last_successful_timestamp",
                                ));
                            }
                            f_last_successful_timestamp = Some(map.next_value::<crate::types::nodes_stats::ShardIndexingPressurePerShardLastSuccessfulTimestamp>()?);
                        }
                        "memory" => {
                            if f_memory.is_some() {
                                return Err(serde::de::Error::duplicate_field("memory"));
                            }
                            f_memory = Some(map.next_value::<crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryStats>()?);
                        }
                        "memory_allocation" => {
                            if f_memory_allocation.is_some() {
                                return Err(serde::de::Error::duplicate_field("memory_allocation"));
                            }
                            f_memory_allocation = Some(map.next_value::<crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryAllocationStats>()?);
                        }
                        "rejection" => {
                            if f_rejection.is_some() {
                                return Err(serde::de::Error::duplicate_field("rejection"));
                            }
                            f_rejection = Some(map.next_value::<crate::types::nodes_stats::ShardIndexingPressurePerShardRejectionStats>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardIndexingPressurePerShardStats {
                    indexing: f_indexing
                        .ok_or_else(|| serde::de::Error::missing_field("indexing"))?,
                    last_successful_timestamp: f_last_successful_timestamp.ok_or_else(|| {
                        serde::de::Error::missing_field("last_successful_timestamp")
                    })?,
                    memory: f_memory.ok_or_else(|| serde::de::Error::missing_field("memory"))?,
                    memory_allocation: f_memory_allocation
                        .ok_or_else(|| serde::de::Error::missing_field("memory_allocation"))?,
                    rejection: f_rejection
                        .ok_or_else(|| serde::de::Error::missing_field("rejection"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardIndexingPressurePerShardStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("indexing", &self.indexing)?;
        map.serialize_entry("last_successful_timestamp", &self.last_successful_timestamp)?;
        map.serialize_entry("memory", &self.memory)?;
        map.serialize_entry("memory_allocation", &self.memory_allocation)?;
        map.serialize_entry("rejection", &self.rejection)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardIndexingPressureRejectionsBreakupStats {
    pub no_successful_request_limits: Option<i64>,
    pub node_limits: Option<i64>,
    pub throughput_degradation_limits: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardIndexingPressureRejectionsBreakupStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardIndexingPressureRejectionsBreakupStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardIndexingPressureRejectionsBreakupStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_no_successful_request_limits: Option<i64> = None;
                let mut seen_no_successful_request_limits = false;
                let mut f_node_limits: Option<i64> = None;
                let mut seen_node_limits = false;
                let mut f_throughput_degradation_limits: Option<i64> = None;
                let mut seen_throughput_degradation_limits = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "no_successful_request_limits" => {
                            if seen_no_successful_request_limits {
                                return Err(serde::de::Error::duplicate_field(
                                    "no_successful_request_limits",
                                ));
                            }
                            seen_no_successful_request_limits = true;
                            f_no_successful_request_limits = map.next_value::<Option<i64>>()?;
                        }
                        "node_limits" => {
                            if seen_node_limits {
                                return Err(serde::de::Error::duplicate_field("node_limits"));
                            }
                            seen_node_limits = true;
                            f_node_limits = map.next_value::<Option<i64>>()?;
                        }
                        "throughput_degradation_limits" => {
                            if seen_throughput_degradation_limits {
                                return Err(serde::de::Error::duplicate_field(
                                    "throughput_degradation_limits",
                                ));
                            }
                            seen_throughput_degradation_limits = true;
                            f_throughput_degradation_limits = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardIndexingPressureRejectionsBreakupStats {
                    no_successful_request_limits: f_no_successful_request_limits,
                    node_limits: f_node_limits,
                    throughput_degradation_limits: f_throughput_degradation_limits,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardIndexingPressureRejectionsBreakupStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.no_successful_request_limits {
            map.serialize_entry("no_successful_request_limits", v)?;
        }
        if let Some(v) = &self.node_limits {
            map.serialize_entry("node_limits", v)?;
        }
        if let Some(v) = &self.throughput_degradation_limits {
            map.serialize_entry("throughput_degradation_limits", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardIndexingPressureStats {
    pub enabled: Option<bool>,
    pub enforced: Option<bool>,
    pub stats: Option<
        std::collections::BTreeMap<
            String,
            crate::types::nodes_stats::ShardIndexingPressurePerShardStats,
        >,
    >,
    pub total_rejections_breakup:
        Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>,
    pub total_rejections_breakup_shadow_mode:
        Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardIndexingPressureStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardIndexingPressureStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardIndexingPressureStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_enabled: Option<bool> = None;
                let mut seen_enabled = false;
                let mut f_enforced: Option<bool> = None;
                let mut seen_enforced = false;
                let mut f_stats: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::nodes_stats::ShardIndexingPressurePerShardStats,
                    >,
                > = None;
                let mut seen_stats = false;
                let mut f_total_rejections_breakup: Option<
                    crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats,
                > = None;
                let mut seen_total_rejections_breakup = false;
                let mut f_total_rejections_breakup_shadow_mode: Option<
                    crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats,
                > = None;
                let mut seen_total_rejections_breakup_shadow_mode = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "enabled" => {
                            if seen_enabled {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            seen_enabled = true;
                            f_enabled = map.next_value::<Option<bool>>()?;
                        }
                        "enforced" => {
                            if seen_enforced {
                                return Err(serde::de::Error::duplicate_field("enforced"));
                            }
                            seen_enforced = true;
                            f_enforced = map.next_value::<Option<bool>>()?;
                        }
                        "stats" => {
                            if seen_stats {
                                return Err(serde::de::Error::duplicate_field("stats"));
                            }
                            seen_stats = true;
                            f_stats = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::nodes_stats::ShardIndexingPressurePerShardStats,
                                >,
                            >>()?;
                        }
                        "total_rejections_breakup" => {
                            if seen_total_rejections_breakup {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_rejections_breakup",
                                ));
                            }
                            seen_total_rejections_breakup = true;
                            f_total_rejections_breakup = map.next_value::<Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>>()?;
                        }
                        "total_rejections_breakup_shadow_mode" => {
                            if seen_total_rejections_breakup_shadow_mode {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_rejections_breakup_shadow_mode",
                                ));
                            }
                            seen_total_rejections_breakup_shadow_mode = true;
                            f_total_rejections_breakup_shadow_mode = map.next_value::<Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardIndexingPressureStats {
                    enabled: f_enabled,
                    enforced: f_enforced,
                    stats: f_stats,
                    total_rejections_breakup: f_total_rejections_breakup,
                    total_rejections_breakup_shadow_mode: f_total_rejections_breakup_shadow_mode,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardIndexingPressureStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.enabled {
            map.serialize_entry("enabled", v)?;
        }
        if let Some(v) = &self.enforced {
            map.serialize_entry("enforced", v)?;
        }
        if let Some(v) = &self.stats {
            map.serialize_entry("stats", v)?;
        }
        if let Some(v) = &self.total_rejections_breakup {
            map.serialize_entry("total_rejections_breakup", v)?;
        }
        if let Some(v) = &self.total_rejections_breakup_shadow_mode {
            map.serialize_entry("total_rejections_breakup_shadow_mode", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type ShardRepositoriesStats = Vec<crate::types::nodes_stats::RepositoryStatsSnapshot>;

pub type ShardResourceUsageStats =
    std::collections::BTreeMap<String, crate::types::nodes_stats::ShardResourceUsageStatsDetail>;

#[derive(Debug, Clone)]
pub struct ShardResourceUsageStatsDetail {
    pub cpu_utilization_percent: Option<crate::types::common::PercentageString>,
    pub io_usage_stats: Option<crate::types::nodes_stats::ShardResourceUsageStatsIoUsageStats>,
    pub memory_utilization_percent: Option<crate::types::common::PercentageString>,
    pub timestamp: Option<crate::types::common::EpochTimeUnitMillis>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardResourceUsageStatsDetail {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardResourceUsageStatsDetail;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardResourceUsageStatsDetail object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cpu_utilization_percent: Option<crate::types::common::PercentageString> =
                    None;
                let mut seen_cpu_utilization_percent = false;
                let mut f_io_usage_stats: Option<
                    crate::types::nodes_stats::ShardResourceUsageStatsIoUsageStats,
                > = None;
                let mut seen_io_usage_stats = false;
                let mut f_memory_utilization_percent: Option<
                    crate::types::common::PercentageString,
                > = None;
                let mut seen_memory_utilization_percent = false;
                let mut f_timestamp: Option<crate::types::common::EpochTimeUnitMillis> = None;
                let mut seen_timestamp = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cpu_utilization_percent" => {
                            if seen_cpu_utilization_percent {
                                return Err(serde::de::Error::duplicate_field(
                                    "cpu_utilization_percent",
                                ));
                            }
                            seen_cpu_utilization_percent = true;
                            f_cpu_utilization_percent =
                                map.next_value::<Option<crate::types::common::PercentageString>>()?;
                        }
                        "io_usage_stats" => {
                            if seen_io_usage_stats {
                                return Err(serde::de::Error::duplicate_field("io_usage_stats"));
                            }
                            seen_io_usage_stats = true;
                            f_io_usage_stats = map.next_value::<Option<
                                crate::types::nodes_stats::ShardResourceUsageStatsIoUsageStats,
                            >>()?;
                        }
                        "memory_utilization_percent" => {
                            if seen_memory_utilization_percent {
                                return Err(serde::de::Error::duplicate_field(
                                    "memory_utilization_percent",
                                ));
                            }
                            seen_memory_utilization_percent = true;
                            f_memory_utilization_percent =
                                map.next_value::<Option<crate::types::common::PercentageString>>()?;
                        }
                        "timestamp" => {
                            if seen_timestamp {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            seen_timestamp = true;
                            f_timestamp = map
                                .next_value::<Option<crate::types::common::EpochTimeUnitMillis>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardResourceUsageStatsDetail {
                    cpu_utilization_percent: f_cpu_utilization_percent,
                    io_usage_stats: f_io_usage_stats,
                    memory_utilization_percent: f_memory_utilization_percent,
                    timestamp: f_timestamp,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardResourceUsageStatsDetail {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cpu_utilization_percent {
            map.serialize_entry("cpu_utilization_percent", v)?;
        }
        if let Some(v) = &self.io_usage_stats {
            map.serialize_entry("io_usage_stats", v)?;
        }
        if let Some(v) = &self.memory_utilization_percent {
            map.serialize_entry("memory_utilization_percent", v)?;
        }
        if let Some(v) = &self.timestamp {
            map.serialize_entry("timestamp", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardResourceUsageStatsIoUsageStats {
    pub max_io_utilization_percent: Option<crate::types::common::PercentageString>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardResourceUsageStatsIoUsageStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardResourceUsageStatsIoUsageStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardResourceUsageStatsIoUsageStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_max_io_utilization_percent: Option<
                    crate::types::common::PercentageString,
                > = None;
                let mut seen_max_io_utilization_percent = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "max_io_utilization_percent" => {
                            if seen_max_io_utilization_percent {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_io_utilization_percent",
                                ));
                            }
                            seen_max_io_utilization_percent = true;
                            f_max_io_utilization_percent =
                                map.next_value::<Option<crate::types::common::PercentageString>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardResourceUsageStatsIoUsageStats {
                    max_io_utilization_percent: f_max_io_utilization_percent,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardResourceUsageStatsIoUsageStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.max_io_utilization_percent {
            map.serialize_entry("max_io_utilization_percent", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShardSearchBackpressureMode {
    #[doc = "`disabled`"]
    Disabled,
    #[doc = "`enforced`"]
    Enforced,
    #[doc = "`monitor_only`"]
    MonitorOnly,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ShardSearchBackpressureMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ShardSearchBackpressureMode::Disabled => "disabled",
            ShardSearchBackpressureMode::Enforced => "enforced",
            ShardSearchBackpressureMode::MonitorOnly => "monitor_only",
            ShardSearchBackpressureMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ShardSearchBackpressureMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShardSearchBackpressureMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "disabled" => ShardSearchBackpressureMode::Disabled,
            "enforced" => ShardSearchBackpressureMode::Enforced,
            "monitor_only" => ShardSearchBackpressureMode::MonitorOnly,
            _ => ShardSearchBackpressureMode::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ShardSearchBackpressureStats {
    pub mode: Option<crate::types::nodes_stats::ShardSearchBackpressureMode>,
    pub search_shard_task: Option<crate::types::nodes_stats::ShardSearchBackpressureTaskStats>,
    pub search_task: Option<crate::types::nodes_stats::ShardSearchBackpressureTaskStats>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardSearchBackpressureStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardSearchBackpressureStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardSearchBackpressureStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_mode: Option<crate::types::nodes_stats::ShardSearchBackpressureMode> =
                    None;
                let mut seen_mode = false;
                let mut f_search_shard_task: Option<
                    crate::types::nodes_stats::ShardSearchBackpressureTaskStats,
                > = None;
                let mut seen_search_shard_task = false;
                let mut f_search_task: Option<
                    crate::types::nodes_stats::ShardSearchBackpressureTaskStats,
                > = None;
                let mut seen_search_task = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "mode" => {
                            if seen_mode {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            seen_mode = true;
                            f_mode = map.next_value::<Option<crate::types::nodes_stats::ShardSearchBackpressureMode>>()?;
                        }
                        "search_shard_task" => {
                            if seen_search_shard_task {
                                return Err(serde::de::Error::duplicate_field("search_shard_task"));
                            }
                            seen_search_shard_task = true;
                            f_search_shard_task = map.next_value::<Option<
                                crate::types::nodes_stats::ShardSearchBackpressureTaskStats,
                            >>()?;
                        }
                        "search_task" => {
                            if seen_search_task {
                                return Err(serde::de::Error::duplicate_field("search_task"));
                            }
                            seen_search_task = true;
                            f_search_task = map.next_value::<Option<
                                crate::types::nodes_stats::ShardSearchBackpressureTaskStats,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardSearchBackpressureStats {
                    mode: f_mode,
                    search_shard_task: f_search_shard_task,
                    search_task: f_search_task,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardSearchBackpressureStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.mode {
            map.serialize_entry("mode", v)?;
        }
        if let Some(v) = &self.search_shard_task {
            map.serialize_entry("search_shard_task", v)?;
        }
        if let Some(v) = &self.search_task {
            map.serialize_entry("search_task", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardSearchBackpressureTaskCancellationStats {
    pub cancellation_count: Option<i64>,
    pub cancellation_limit_reached_count: Option<i64>,
    pub cancelled_task_percentage: Option<crate::types::common::PercentageNumber>,
    pub current_cancellation_eligible_tasks_count: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardSearchBackpressureTaskCancellationStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardSearchBackpressureTaskCancellationStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardSearchBackpressureTaskCancellationStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cancellation_count: Option<i64> = None;
                let mut seen_cancellation_count = false;
                let mut f_cancellation_limit_reached_count: Option<i64> = None;
                let mut seen_cancellation_limit_reached_count = false;
                let mut f_cancelled_task_percentage: Option<
                    crate::types::common::PercentageNumber,
                > = None;
                let mut seen_cancelled_task_percentage = false;
                let mut f_current_cancellation_eligible_tasks_count: Option<i64> = None;
                let mut seen_current_cancellation_eligible_tasks_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cancellation_count" => {
                            if seen_cancellation_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "cancellation_count",
                                ));
                            }
                            seen_cancellation_count = true;
                            f_cancellation_count = map.next_value::<Option<i64>>()?;
                        }
                        "cancellation_limit_reached_count" => {
                            if seen_cancellation_limit_reached_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "cancellation_limit_reached_count",
                                ));
                            }
                            seen_cancellation_limit_reached_count = true;
                            f_cancellation_limit_reached_count = map.next_value::<Option<i64>>()?;
                        }
                        "cancelled_task_percentage" => {
                            if seen_cancelled_task_percentage {
                                return Err(serde::de::Error::duplicate_field(
                                    "cancelled_task_percentage",
                                ));
                            }
                            seen_cancelled_task_percentage = true;
                            f_cancelled_task_percentage =
                                map.next_value::<Option<crate::types::common::PercentageNumber>>()?;
                        }
                        "current_cancellation_eligible_tasks_count" => {
                            if seen_current_cancellation_eligible_tasks_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "current_cancellation_eligible_tasks_count",
                                ));
                            }
                            seen_current_cancellation_eligible_tasks_count = true;
                            f_current_cancellation_eligible_tasks_count =
                                map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardSearchBackpressureTaskCancellationStats {
                    cancellation_count: f_cancellation_count,
                    cancellation_limit_reached_count: f_cancellation_limit_reached_count,
                    cancelled_task_percentage: f_cancelled_task_percentage,
                    current_cancellation_eligible_tasks_count:
                        f_current_cancellation_eligible_tasks_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardSearchBackpressureTaskCancellationStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cancellation_count {
            map.serialize_entry("cancellation_count", v)?;
        }
        if let Some(v) = &self.cancellation_limit_reached_count {
            map.serialize_entry("cancellation_limit_reached_count", v)?;
        }
        if let Some(v) = &self.cancelled_task_percentage {
            map.serialize_entry("cancelled_task_percentage", v)?;
        }
        if let Some(v) = &self.current_cancellation_eligible_tasks_count {
            map.serialize_entry("current_cancellation_eligible_tasks_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardSearchBackpressureTaskResourceTrackerCpuUsageTrackerStats {
    pub cancellation_count: Option<i64>,
    pub current_avg: Option<crate::types::common::Duration>,
    pub current_avg_millis: Option<crate::types::common::DurationValueUnitMillis>,
    pub current_max: Option<crate::types::common::Duration>,
    pub current_max_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de>
    for ShardSearchBackpressureTaskResourceTrackerCpuUsageTrackerStats
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardSearchBackpressureTaskResourceTrackerCpuUsageTrackerStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(
                    "a ShardSearchBackpressureTaskResourceTrackerCpuUsageTrackerStats object",
                )
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cancellation_count: Option<i64> = None;
                let mut seen_cancellation_count = false;
                let mut f_current_avg: Option<crate::types::common::Duration> = None;
                let mut seen_current_avg = false;
                let mut f_current_avg_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut seen_current_avg_millis = false;
                let mut f_current_max: Option<crate::types::common::Duration> = None;
                let mut seen_current_max = false;
                let mut f_current_max_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut seen_current_max_millis = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cancellation_count" => {
                            if seen_cancellation_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "cancellation_count",
                                ));
                            }
                            seen_cancellation_count = true;
                            f_cancellation_count = map.next_value::<Option<i64>>()?;
                        }
                        "current_avg" => {
                            if seen_current_avg {
                                return Err(serde::de::Error::duplicate_field("current_avg"));
                            }
                            seen_current_avg = true;
                            f_current_avg =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "current_avg_millis" => {
                            if seen_current_avg_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "current_avg_millis",
                                ));
                            }
                            seen_current_avg_millis = true;
                            f_current_avg_millis = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        "current_max" => {
                            if seen_current_max {
                                return Err(serde::de::Error::duplicate_field("current_max"));
                            }
                            seen_current_max = true;
                            f_current_max =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "current_max_millis" => {
                            if seen_current_max_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "current_max_millis",
                                ));
                            }
                            seen_current_max_millis = true;
                            f_current_max_millis = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(
                    ShardSearchBackpressureTaskResourceTrackerCpuUsageTrackerStats {
                        cancellation_count: f_cancellation_count,
                        current_avg: f_current_avg,
                        current_avg_millis: f_current_avg_millis,
                        current_max: f_current_max,
                        current_max_millis: f_current_max_millis,
                        unknown_fields: residual,
                    },
                )
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardSearchBackpressureTaskResourceTrackerCpuUsageTrackerStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cancellation_count {
            map.serialize_entry("cancellation_count", v)?;
        }
        if let Some(v) = &self.current_avg {
            map.serialize_entry("current_avg", v)?;
        }
        if let Some(v) = &self.current_avg_millis {
            map.serialize_entry("current_avg_millis", v)?;
        }
        if let Some(v) = &self.current_max {
            map.serialize_entry("current_max", v)?;
        }
        if let Some(v) = &self.current_max_millis {
            map.serialize_entry("current_max_millis", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardSearchBackpressureTaskResourceTrackerElapsedTimeTrackerStats {
    pub cancellation_count: Option<i64>,
    pub current_avg: Option<crate::types::common::Duration>,
    pub current_avg_millis: Option<crate::types::common::DurationValueUnitMillis>,
    pub current_max: Option<crate::types::common::Duration>,
    pub current_max_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de>
    for ShardSearchBackpressureTaskResourceTrackerElapsedTimeTrackerStats
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardSearchBackpressureTaskResourceTrackerElapsedTimeTrackerStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(
                    "a ShardSearchBackpressureTaskResourceTrackerElapsedTimeTrackerStats object",
                )
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cancellation_count: Option<i64> = None;
                let mut seen_cancellation_count = false;
                let mut f_current_avg: Option<crate::types::common::Duration> = None;
                let mut seen_current_avg = false;
                let mut f_current_avg_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut seen_current_avg_millis = false;
                let mut f_current_max: Option<crate::types::common::Duration> = None;
                let mut seen_current_max = false;
                let mut f_current_max_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut seen_current_max_millis = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cancellation_count" => {
                            if seen_cancellation_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "cancellation_count",
                                ));
                            }
                            seen_cancellation_count = true;
                            f_cancellation_count = map.next_value::<Option<i64>>()?;
                        }
                        "current_avg" => {
                            if seen_current_avg {
                                return Err(serde::de::Error::duplicate_field("current_avg"));
                            }
                            seen_current_avg = true;
                            f_current_avg =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "current_avg_millis" => {
                            if seen_current_avg_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "current_avg_millis",
                                ));
                            }
                            seen_current_avg_millis = true;
                            f_current_avg_millis = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        "current_max" => {
                            if seen_current_max {
                                return Err(serde::de::Error::duplicate_field("current_max"));
                            }
                            seen_current_max = true;
                            f_current_max =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "current_max_millis" => {
                            if seen_current_max_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "current_max_millis",
                                ));
                            }
                            seen_current_max_millis = true;
                            f_current_max_millis = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(
                    ShardSearchBackpressureTaskResourceTrackerElapsedTimeTrackerStats {
                        cancellation_count: f_cancellation_count,
                        current_avg: f_current_avg,
                        current_avg_millis: f_current_avg_millis,
                        current_max: f_current_max,
                        current_max_millis: f_current_max_millis,
                        unknown_fields: residual,
                    },
                )
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardSearchBackpressureTaskResourceTrackerElapsedTimeTrackerStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cancellation_count {
            map.serialize_entry("cancellation_count", v)?;
        }
        if let Some(v) = &self.current_avg {
            map.serialize_entry("current_avg", v)?;
        }
        if let Some(v) = &self.current_avg_millis {
            map.serialize_entry("current_avg_millis", v)?;
        }
        if let Some(v) = &self.current_max {
            map.serialize_entry("current_max", v)?;
        }
        if let Some(v) = &self.current_max_millis {
            map.serialize_entry("current_max_millis", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardSearchBackpressureTaskResourceTrackerHeapUsageTrackerStats {
    pub cancellation_count: Option<i64>,
    pub current_avg: Option<crate::types::common::HumanReadableByteCount>,
    pub current_avg_bytes: Option<crate::types::common::ByteCount>,
    pub current_max: Option<crate::types::common::HumanReadableByteCount>,
    pub current_max_bytes: Option<crate::types::common::ByteCount>,
    pub rolling_avg: Option<crate::types::common::HumanReadableByteCount>,
    pub rolling_avg_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de>
    for ShardSearchBackpressureTaskResourceTrackerHeapUsageTrackerStats
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardSearchBackpressureTaskResourceTrackerHeapUsageTrackerStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(
                    "a ShardSearchBackpressureTaskResourceTrackerHeapUsageTrackerStats object",
                )
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cancellation_count: Option<i64> = None;
                let mut seen_cancellation_count = false;
                let mut f_current_avg: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_current_avg = false;
                let mut f_current_avg_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_current_avg_bytes = false;
                let mut f_current_max: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_current_max = false;
                let mut f_current_max_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_current_max_bytes = false;
                let mut f_rolling_avg: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_rolling_avg = false;
                let mut f_rolling_avg_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_rolling_avg_bytes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cancellation_count" => {
                            if seen_cancellation_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "cancellation_count",
                                ));
                            }
                            seen_cancellation_count = true;
                            f_cancellation_count = map.next_value::<Option<i64>>()?;
                        }
                        "current_avg" => {
                            if seen_current_avg {
                                return Err(serde::de::Error::duplicate_field("current_avg"));
                            }
                            seen_current_avg = true;
                            f_current_avg = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "current_avg_bytes" => {
                            if seen_current_avg_bytes {
                                return Err(serde::de::Error::duplicate_field("current_avg_bytes"));
                            }
                            seen_current_avg_bytes = true;
                            f_current_avg_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "current_max" => {
                            if seen_current_max {
                                return Err(serde::de::Error::duplicate_field("current_max"));
                            }
                            seen_current_max = true;
                            f_current_max = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "current_max_bytes" => {
                            if seen_current_max_bytes {
                                return Err(serde::de::Error::duplicate_field("current_max_bytes"));
                            }
                            seen_current_max_bytes = true;
                            f_current_max_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "rolling_avg" => {
                            if seen_rolling_avg {
                                return Err(serde::de::Error::duplicate_field("rolling_avg"));
                            }
                            seen_rolling_avg = true;
                            f_rolling_avg = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "rolling_avg_bytes" => {
                            if seen_rolling_avg_bytes {
                                return Err(serde::de::Error::duplicate_field("rolling_avg_bytes"));
                            }
                            seen_rolling_avg_bytes = true;
                            f_rolling_avg_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(
                    ShardSearchBackpressureTaskResourceTrackerHeapUsageTrackerStats {
                        cancellation_count: f_cancellation_count,
                        current_avg: f_current_avg,
                        current_avg_bytes: f_current_avg_bytes,
                        current_max: f_current_max,
                        current_max_bytes: f_current_max_bytes,
                        rolling_avg: f_rolling_avg,
                        rolling_avg_bytes: f_rolling_avg_bytes,
                        unknown_fields: residual,
                    },
                )
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardSearchBackpressureTaskResourceTrackerHeapUsageTrackerStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cancellation_count {
            map.serialize_entry("cancellation_count", v)?;
        }
        if let Some(v) = &self.current_avg {
            map.serialize_entry("current_avg", v)?;
        }
        if let Some(v) = &self.current_avg_bytes {
            map.serialize_entry("current_avg_bytes", v)?;
        }
        if let Some(v) = &self.current_max {
            map.serialize_entry("current_max", v)?;
        }
        if let Some(v) = &self.current_max_bytes {
            map.serialize_entry("current_max_bytes", v)?;
        }
        if let Some(v) = &self.rolling_avg {
            map.serialize_entry("rolling_avg", v)?;
        }
        if let Some(v) = &self.rolling_avg_bytes {
            map.serialize_entry("rolling_avg_bytes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardSearchBackpressureTaskResourceTrackerStats {
    pub cpu_usage_tracker: Option<crate::types::nodes_stats::ShardSearchBackpressureTaskResourceTrackerCpuUsageTrackerStats>,
    pub elapsed_time_tracker: Option<crate::types::nodes_stats::ShardSearchBackpressureTaskResourceTrackerElapsedTimeTrackerStats>,
    pub heap_usage_tracker: Option<crate::types::nodes_stats::ShardSearchBackpressureTaskResourceTrackerHeapUsageTrackerStats>,
#[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardSearchBackpressureTaskResourceTrackerStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardSearchBackpressureTaskResourceTrackerStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardSearchBackpressureTaskResourceTrackerStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cpu_usage_tracker: Option<crate::types::nodes_stats::ShardSearchBackpressureTaskResourceTrackerCpuUsageTrackerStats> = None;
                let mut seen_cpu_usage_tracker = false;
                let mut f_elapsed_time_tracker: Option<crate::types::nodes_stats::ShardSearchBackpressureTaskResourceTrackerElapsedTimeTrackerStats> = None;
                let mut seen_elapsed_time_tracker = false;
                let mut f_heap_usage_tracker: Option<crate::types::nodes_stats::ShardSearchBackpressureTaskResourceTrackerHeapUsageTrackerStats> = None;
                let mut seen_heap_usage_tracker = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cpu_usage_tracker" => {
                            if seen_cpu_usage_tracker {
                                return Err(serde::de::Error::duplicate_field("cpu_usage_tracker"));
                            }
                            seen_cpu_usage_tracker = true;
                            f_cpu_usage_tracker = map.next_value::<Option<crate::types::nodes_stats::ShardSearchBackpressureTaskResourceTrackerCpuUsageTrackerStats>>()?;
                        }
                        "elapsed_time_tracker" => {
                            if seen_elapsed_time_tracker {
                                return Err(serde::de::Error::duplicate_field(
                                    "elapsed_time_tracker",
                                ));
                            }
                            seen_elapsed_time_tracker = true;
                            f_elapsed_time_tracker = map.next_value::<Option<crate::types::nodes_stats::ShardSearchBackpressureTaskResourceTrackerElapsedTimeTrackerStats>>()?;
                        }
                        "heap_usage_tracker" => {
                            if seen_heap_usage_tracker {
                                return Err(serde::de::Error::duplicate_field(
                                    "heap_usage_tracker",
                                ));
                            }
                            seen_heap_usage_tracker = true;
                            f_heap_usage_tracker = map.next_value::<Option<crate::types::nodes_stats::ShardSearchBackpressureTaskResourceTrackerHeapUsageTrackerStats>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardSearchBackpressureTaskResourceTrackerStats {
                    cpu_usage_tracker: f_cpu_usage_tracker,
                    elapsed_time_tracker: f_elapsed_time_tracker,
                    heap_usage_tracker: f_heap_usage_tracker,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardSearchBackpressureTaskResourceTrackerStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cpu_usage_tracker {
            map.serialize_entry("cpu_usage_tracker", v)?;
        }
        if let Some(v) = &self.elapsed_time_tracker {
            map.serialize_entry("elapsed_time_tracker", v)?;
        }
        if let Some(v) = &self.heap_usage_tracker {
            map.serialize_entry("heap_usage_tracker", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardSearchBackpressureTaskStats {
    pub cancellation_stats:
        Option<crate::types::nodes_stats::ShardSearchBackpressureTaskCancellationStats>,
    pub completion_count: Option<i64>,
    pub resource_tracker_stats:
        Option<crate::types::nodes_stats::ShardSearchBackpressureTaskResourceTrackerStats>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardSearchBackpressureTaskStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardSearchBackpressureTaskStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardSearchBackpressureTaskStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cancellation_stats: Option<
                    crate::types::nodes_stats::ShardSearchBackpressureTaskCancellationStats,
                > = None;
                let mut seen_cancellation_stats = false;
                let mut f_completion_count: Option<i64> = None;
                let mut seen_completion_count = false;
                let mut f_resource_tracker_stats: Option<
                    crate::types::nodes_stats::ShardSearchBackpressureTaskResourceTrackerStats,
                > = None;
                let mut seen_resource_tracker_stats = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cancellation_stats" => {
                            if seen_cancellation_stats {
                                return Err(serde::de::Error::duplicate_field(
                                    "cancellation_stats",
                                ));
                            }
                            seen_cancellation_stats = true;
                            f_cancellation_stats = map.next_value::<Option<crate::types::nodes_stats::ShardSearchBackpressureTaskCancellationStats>>()?;
                        }
                        "completion_count" => {
                            if seen_completion_count {
                                return Err(serde::de::Error::duplicate_field("completion_count"));
                            }
                            seen_completion_count = true;
                            f_completion_count = map.next_value::<Option<i64>>()?;
                        }
                        "resource_tracker_stats" => {
                            if seen_resource_tracker_stats {
                                return Err(serde::de::Error::duplicate_field(
                                    "resource_tracker_stats",
                                ));
                            }
                            seen_resource_tracker_stats = true;
                            f_resource_tracker_stats = map.next_value::<Option<crate::types::nodes_stats::ShardSearchBackpressureTaskResourceTrackerStats>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardSearchBackpressureTaskStats {
                    cancellation_stats: f_cancellation_stats,
                    completion_count: f_completion_count,
                    resource_tracker_stats: f_resource_tracker_stats,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardSearchBackpressureTaskStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cancellation_stats {
            map.serialize_entry("cancellation_stats", v)?;
        }
        if let Some(v) = &self.completion_count {
            map.serialize_entry("completion_count", v)?;
        }
        if let Some(v) = &self.resource_tracker_stats {
            map.serialize_entry("resource_tracker_stats", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardSearchPipelineOperationStats {
    pub count: Option<i64>,
    pub current: Option<i64>,
    pub failed: Option<i64>,
    pub time: Option<crate::types::common::Duration>,
    pub time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardSearchPipelineOperationStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardSearchPipelineOperationStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardSearchPipelineOperationStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_count: Option<i64> = None;
                let mut seen_count = false;
                let mut f_current: Option<i64> = None;
                let mut seen_current = false;
                let mut f_failed: Option<i64> = None;
                let mut seen_failed = false;
                let mut f_time: Option<crate::types::common::Duration> = None;
                let mut seen_time = false;
                let mut f_time_in_millis: Option<crate::types::common::DurationValueUnitMillis> =
                    None;
                let mut seen_time_in_millis = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "count" => {
                            if seen_count {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            seen_count = true;
                            f_count = map.next_value::<Option<i64>>()?;
                        }
                        "current" => {
                            if seen_current {
                                return Err(serde::de::Error::duplicate_field("current"));
                            }
                            seen_current = true;
                            f_current = map.next_value::<Option<i64>>()?;
                        }
                        "failed" => {
                            if seen_failed {
                                return Err(serde::de::Error::duplicate_field("failed"));
                            }
                            seen_failed = true;
                            f_failed = map.next_value::<Option<i64>>()?;
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
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardSearchPipelineOperationStats {
                    count: f_count,
                    current: f_current,
                    failed: f_failed,
                    time: f_time,
                    time_in_millis: f_time_in_millis,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardSearchPipelineOperationStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.count {
            map.serialize_entry("count", v)?;
        }
        if let Some(v) = &self.current {
            map.serialize_entry("current", v)?;
        }
        if let Some(v) = &self.failed {
            map.serialize_entry("failed", v)?;
        }
        if let Some(v) = &self.time {
            map.serialize_entry("time", v)?;
        }
        if let Some(v) = &self.time_in_millis {
            map.serialize_entry("time_in_millis", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardSearchPipelinePerPipelineProcessorStats {
    pub stats: Option<crate::types::nodes_stats::ShardSearchPipelineOperationStats>,
    pub r#type: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardSearchPipelinePerPipelineProcessorStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardSearchPipelinePerPipelineProcessorStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardSearchPipelinePerPipelineProcessorStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_stats: Option<
                    crate::types::nodes_stats::ShardSearchPipelineOperationStats,
                > = None;
                let mut seen_stats = false;
                let mut f_type: Option<String> = None;
                let mut seen_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "stats" => {
                            if seen_stats {
                                return Err(serde::de::Error::duplicate_field("stats"));
                            }
                            seen_stats = true;
                            f_stats = map.next_value::<Option<
                                crate::types::nodes_stats::ShardSearchPipelineOperationStats,
                            >>()?;
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
                Ok(ShardSearchPipelinePerPipelineProcessorStats {
                    stats: f_stats,
                    r#type: f_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardSearchPipelinePerPipelineProcessorStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.stats {
            map.serialize_entry("stats", v)?;
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

#[derive(Debug, Clone)]
pub struct ShardSearchPipelinePerPipelineStats {
    pub request: Option<crate::types::nodes_stats::ShardSearchPipelineOperationStats>,
    pub request_processors:
        Option<Vec<crate::types::nodes_stats::ShardSearchPipelinePerPipelineProcessorStats>>,
    pub response: Option<crate::types::nodes_stats::ShardSearchPipelineOperationStats>,
    pub response_processors: Option<
        Vec<
            std::collections::BTreeMap<
                String,
                crate::types::nodes_stats::ShardSearchPipelinePerPipelineProcessorStats,
            >,
        >,
    >,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardSearchPipelinePerPipelineStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardSearchPipelinePerPipelineStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardSearchPipelinePerPipelineStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_request: Option<
                    crate::types::nodes_stats::ShardSearchPipelineOperationStats,
                > = None;
                let mut seen_request = false;
                let mut f_request_processors: Option<
                    Vec<crate::types::nodes_stats::ShardSearchPipelinePerPipelineProcessorStats>,
                > = None;
                let mut seen_request_processors = false;
                let mut f_response: Option<
                    crate::types::nodes_stats::ShardSearchPipelineOperationStats,
                > = None;
                let mut seen_response = false;
                let mut f_response_processors: Option<
                    Vec<
                        std::collections::BTreeMap<
                            String,
                            crate::types::nodes_stats::ShardSearchPipelinePerPipelineProcessorStats,
                        >,
                    >,
                > = None;
                let mut seen_response_processors = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "request" => {
                            if seen_request {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            seen_request = true;
                            f_request = map.next_value::<Option<
                                crate::types::nodes_stats::ShardSearchPipelineOperationStats,
                            >>()?;
                        }
                        "request_processors" => {
                            if seen_request_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "request_processors",
                                ));
                            }
                            seen_request_processors = true;
                            f_request_processors = map.next_value::<Option<Vec<crate::types::nodes_stats::ShardSearchPipelinePerPipelineProcessorStats>>>()?;
                        }
                        "response" => {
                            if seen_response {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            seen_response = true;
                            f_response = map.next_value::<Option<
                                crate::types::nodes_stats::ShardSearchPipelineOperationStats,
                            >>()?;
                        }
                        "response_processors" => {
                            if seen_response_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "response_processors",
                                ));
                            }
                            seen_response_processors = true;
                            f_response_processors = map.next_value::<Option<Vec<std::collections::BTreeMap<String, crate::types::nodes_stats::ShardSearchPipelinePerPipelineProcessorStats>>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardSearchPipelinePerPipelineStats {
                    request: f_request,
                    request_processors: f_request_processors,
                    response: f_response,
                    response_processors: f_response_processors,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardSearchPipelinePerPipelineStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.request {
            map.serialize_entry("request", v)?;
        }
        if let Some(v) = &self.request_processors {
            map.serialize_entry("request_processors", v)?;
        }
        if let Some(v) = &self.response {
            map.serialize_entry("response", v)?;
        }
        if let Some(v) = &self.response_processors {
            map.serialize_entry("response_processors", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardSearchPipelineStats {
    pub pipelines: Option<
        std::collections::BTreeMap<
            String,
            crate::types::nodes_stats::ShardSearchPipelinePerPipelineStats,
        >,
    >,
    pub total_request: Option<crate::types::nodes_stats::ShardSearchPipelineOperationStats>,
    pub total_response: Option<crate::types::nodes_stats::ShardSearchPipelineOperationStats>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardSearchPipelineStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardSearchPipelineStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardSearchPipelineStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_pipelines: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::nodes_stats::ShardSearchPipelinePerPipelineStats,
                    >,
                > = None;
                let mut seen_pipelines = false;
                let mut f_total_request: Option<
                    crate::types::nodes_stats::ShardSearchPipelineOperationStats,
                > = None;
                let mut seen_total_request = false;
                let mut f_total_response: Option<
                    crate::types::nodes_stats::ShardSearchPipelineOperationStats,
                > = None;
                let mut seen_total_response = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "pipelines" => {
                            if seen_pipelines {
                                return Err(serde::de::Error::duplicate_field("pipelines"));
                            }
                            seen_pipelines = true;
                            f_pipelines = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::nodes_stats::ShardSearchPipelinePerPipelineStats,
                                >,
                            >>()?;
                        }
                        "total_request" => {
                            if seen_total_request {
                                return Err(serde::de::Error::duplicate_field("total_request"));
                            }
                            seen_total_request = true;
                            f_total_request = map.next_value::<Option<
                                crate::types::nodes_stats::ShardSearchPipelineOperationStats,
                            >>()?;
                        }
                        "total_response" => {
                            if seen_total_response {
                                return Err(serde::de::Error::duplicate_field("total_response"));
                            }
                            seen_total_response = true;
                            f_total_response = map.next_value::<Option<
                                crate::types::nodes_stats::ShardSearchPipelineOperationStats,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardSearchPipelineStats {
                    pipelines: f_pipelines,
                    total_request: f_total_request,
                    total_response: f_total_response,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardSearchPipelineStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.pipelines {
            map.serialize_entry("pipelines", v)?;
        }
        if let Some(v) = &self.total_request {
            map.serialize_entry("total_request", v)?;
        }
        if let Some(v) = &self.total_response {
            map.serialize_entry("total_response", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardSegmentReplicationBackpressureStats {
    pub total_rejected_requests: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardSegmentReplicationBackpressureStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardSegmentReplicationBackpressureStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardSegmentReplicationBackpressureStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_total_rejected_requests: Option<i64> = None;
                let mut seen_total_rejected_requests = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "total_rejected_requests" => {
                            if seen_total_rejected_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_rejected_requests",
                                ));
                            }
                            seen_total_rejected_requests = true;
                            f_total_rejected_requests = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardSegmentReplicationBackpressureStats {
                    total_rejected_requests: f_total_rejected_requests,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardSegmentReplicationBackpressureStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.total_rejected_requests {
            map.serialize_entry("total_rejected_requests", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardTaskCancellationStats {
    pub search_shard_task: Option<crate::types::nodes_stats::ShardTaskCancellationStatsDetail>,
    pub search_task: Option<crate::types::nodes_stats::ShardTaskCancellationStatsDetail>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardTaskCancellationStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardTaskCancellationStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardTaskCancellationStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_search_shard_task: Option<
                    crate::types::nodes_stats::ShardTaskCancellationStatsDetail,
                > = None;
                let mut seen_search_shard_task = false;
                let mut f_search_task: Option<
                    crate::types::nodes_stats::ShardTaskCancellationStatsDetail,
                > = None;
                let mut seen_search_task = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "search_shard_task" => {
                            if seen_search_shard_task {
                                return Err(serde::de::Error::duplicate_field("search_shard_task"));
                            }
                            seen_search_shard_task = true;
                            f_search_shard_task = map.next_value::<Option<
                                crate::types::nodes_stats::ShardTaskCancellationStatsDetail,
                            >>()?;
                        }
                        "search_task" => {
                            if seen_search_task {
                                return Err(serde::de::Error::duplicate_field("search_task"));
                            }
                            seen_search_task = true;
                            f_search_task = map.next_value::<Option<
                                crate::types::nodes_stats::ShardTaskCancellationStatsDetail,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardTaskCancellationStats {
                    search_shard_task: f_search_shard_task,
                    search_task: f_search_task,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardTaskCancellationStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.search_shard_task {
            map.serialize_entry("search_shard_task", v)?;
        }
        if let Some(v) = &self.search_task {
            map.serialize_entry("search_task", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardTaskCancellationStatsDetail {
    pub current_count_post_cancel: Option<i64>,
    pub total_count_post_cancel: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardTaskCancellationStatsDetail {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardTaskCancellationStatsDetail;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardTaskCancellationStatsDetail object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_current_count_post_cancel: Option<i64> = None;
                let mut seen_current_count_post_cancel = false;
                let mut f_total_count_post_cancel: Option<i64> = None;
                let mut seen_total_count_post_cancel = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "current_count_post_cancel" => {
                            if seen_current_count_post_cancel {
                                return Err(serde::de::Error::duplicate_field(
                                    "current_count_post_cancel",
                                ));
                            }
                            seen_current_count_post_cancel = true;
                            f_current_count_post_cancel = map.next_value::<Option<i64>>()?;
                        }
                        "total_count_post_cancel" => {
                            if seen_total_count_post_cancel {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_count_post_cancel",
                                ));
                            }
                            seen_total_count_post_cancel = true;
                            f_total_count_post_cancel = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardTaskCancellationStatsDetail {
                    current_count_post_cancel: f_current_count_post_cancel,
                    total_count_post_cancel: f_total_count_post_cancel,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardTaskCancellationStatsDetail {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.current_count_post_cancel {
            map.serialize_entry("current_count_post_cancel", v)?;
        }
        if let Some(v) = &self.total_count_post_cancel {
            map.serialize_entry("total_count_post_cancel", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardWeightedRoutingStats {
    pub stats: Option<crate::types::nodes_stats::ShardWeightedRoutingStatsDetail>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardWeightedRoutingStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardWeightedRoutingStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardWeightedRoutingStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_stats: Option<
                    crate::types::nodes_stats::ShardWeightedRoutingStatsDetail,
                > = None;
                let mut seen_stats = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "stats" => {
                            if seen_stats {
                                return Err(serde::de::Error::duplicate_field("stats"));
                            }
                            seen_stats = true;
                            f_stats = map.next_value::<Option<
                                crate::types::nodes_stats::ShardWeightedRoutingStatsDetail,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardWeightedRoutingStats {
                    stats: f_stats,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardWeightedRoutingStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
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
pub struct ShardWeightedRoutingStatsDetail {
    pub fail_open_count: Option<i32>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardWeightedRoutingStatsDetail {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardWeightedRoutingStatsDetail;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardWeightedRoutingStatsDetail object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_fail_open_count: Option<i32> = None;
                let mut seen_fail_open_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "fail_open_count" => {
                            if seen_fail_open_count {
                                return Err(serde::de::Error::duplicate_field("fail_open_count"));
                            }
                            seen_fail_open_count = true;
                            f_fail_open_count = map.next_value::<Option<i32>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardWeightedRoutingStatsDetail {
                    fail_open_count: f_fail_open_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardWeightedRoutingStatsDetail {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.fail_open_count {
            map.serialize_entry("fail_open_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Stats {
    #[doc = "Statistics about adaptive replica selection."]
    pub adaptive_selection:
        Option<std::collections::BTreeMap<String, crate::types::nodes_stats::AdaptiveSelection>>,
    pub admission_control: Option<crate::types::nodes_stats::ShardAdmissionControlStats>,
    #[doc = "Contains a list of attributes for the node."]
    pub attributes: Option<std::collections::BTreeMap<String, String>>,
    #[doc = "Statistics about the field data circuit breaker."]
    pub breakers: Option<std::collections::BTreeMap<String, crate::types::nodes_stats::Breaker>>,
    pub caches: Option<crate::types::nodes_stats::CachesStats>,
    pub cluster_manager_throttling:
        Option<crate::types::nodes_stats::ShardClusterManagerThrottlingStats>,
    pub discovery: Option<crate::types::nodes_stats::Discovery>,
    pub fs: Option<crate::types::nodes_stats::FileSystem>,
    pub host: Option<crate::types::common::Host>,
    pub http: Option<crate::types::nodes_stats::Http>,
    pub indexing_pressure: Option<crate::types::nodes_stats::IndexingPressure>,
    pub indices: Option<crate::types::nodes_stats::NodeIndicesStats>,
    pub ingest: Option<crate::types::nodes_stats::Ingest>,
    #[doc = "IP address and port for the node."]
    pub ip: Option<crate::types::nodes_stats::StatsIp>,
    pub jvm: Option<crate::types::nodes_stats::Jvm>,
    pub name: Option<crate::types::common::Name>,
    pub os: Option<crate::types::nodes_stats::OperatingSystem>,
    pub process: Option<crate::types::nodes_stats::Process>,
    pub remote_store: Option<crate::types::nodes_stats::RemoteStoreStats>,
    pub repositories: Option<crate::types::nodes_stats::ShardRepositoriesStats>,
    pub resource_usage_stats: Option<crate::types::nodes_stats::ShardResourceUsageStats>,
    pub roles: Option<crate::types::common::NodeRoles>,
    pub script: Option<crate::types::nodes_stats::ScriptStats>,
    pub script_cache: Option<crate::types::nodes_stats::ScriptCacheStats>,
    pub search_backpressure: Option<crate::types::nodes_stats::ShardSearchBackpressureStats>,
    pub search_pipeline: Option<crate::types::nodes_stats::ShardSearchPipelineStats>,
    pub segment_replication_backpressure:
        Option<crate::types::nodes_stats::ShardSegmentReplicationBackpressureStats>,
    pub shard_indexing_pressure: Option<crate::types::nodes_stats::ShardIndexingPressureStats>,
    pub task_cancellation: Option<crate::types::nodes_stats::ShardTaskCancellationStats>,
    #[doc = "Statistics about each thread pool, including current size, queue and rejected tasks."]
    pub thread_pool:
        Option<std::collections::BTreeMap<String, crate::types::nodes_stats::ThreadCount>>,
    pub timestamp: Option<crate::types::common::EpochTimeUnitMillis>,
    pub transport: Option<crate::types::nodes_stats::Transport>,
    pub transport_address: Option<crate::types::common::TransportAddress>,
    pub weighted_routing: Option<crate::types::nodes_stats::ShardWeightedRoutingStats>,
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
                let mut f_adaptive_selection: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::nodes_stats::AdaptiveSelection,
                    >,
                > = None;
                let mut seen_adaptive_selection = false;
                let mut f_admission_control: Option<
                    crate::types::nodes_stats::ShardAdmissionControlStats,
                > = None;
                let mut seen_admission_control = false;
                let mut f_attributes: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_attributes = false;
                let mut f_breakers: Option<
                    std::collections::BTreeMap<String, crate::types::nodes_stats::Breaker>,
                > = None;
                let mut seen_breakers = false;
                let mut f_caches: Option<crate::types::nodes_stats::CachesStats> = None;
                let mut seen_caches = false;
                let mut f_cluster_manager_throttling: Option<
                    crate::types::nodes_stats::ShardClusterManagerThrottlingStats,
                > = None;
                let mut seen_cluster_manager_throttling = false;
                let mut f_discovery: Option<crate::types::nodes_stats::Discovery> = None;
                let mut seen_discovery = false;
                let mut f_fs: Option<crate::types::nodes_stats::FileSystem> = None;
                let mut seen_fs = false;
                let mut f_host: Option<crate::types::common::Host> = None;
                let mut seen_host = false;
                let mut f_http: Option<crate::types::nodes_stats::Http> = None;
                let mut seen_http = false;
                let mut f_indexing_pressure: Option<crate::types::nodes_stats::IndexingPressure> =
                    None;
                let mut seen_indexing_pressure = false;
                let mut f_indices: Option<crate::types::nodes_stats::NodeIndicesStats> = None;
                let mut seen_indices = false;
                let mut f_ingest: Option<crate::types::nodes_stats::Ingest> = None;
                let mut seen_ingest = false;
                let mut f_ip: Option<crate::types::nodes_stats::StatsIp> = None;
                let mut seen_ip = false;
                let mut f_jvm: Option<crate::types::nodes_stats::Jvm> = None;
                let mut seen_jvm = false;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut seen_name = false;
                let mut f_os: Option<crate::types::nodes_stats::OperatingSystem> = None;
                let mut seen_os = false;
                let mut f_process: Option<crate::types::nodes_stats::Process> = None;
                let mut seen_process = false;
                let mut f_remote_store: Option<crate::types::nodes_stats::RemoteStoreStats> = None;
                let mut seen_remote_store = false;
                let mut f_repositories: Option<crate::types::nodes_stats::ShardRepositoriesStats> =
                    None;
                let mut seen_repositories = false;
                let mut f_resource_usage_stats: Option<
                    crate::types::nodes_stats::ShardResourceUsageStats,
                > = None;
                let mut seen_resource_usage_stats = false;
                let mut f_roles: Option<crate::types::common::NodeRoles> = None;
                let mut seen_roles = false;
                let mut f_script: Option<crate::types::nodes_stats::ScriptStats> = None;
                let mut seen_script = false;
                let mut f_script_cache: Option<crate::types::nodes_stats::ScriptCacheStats> = None;
                let mut seen_script_cache = false;
                let mut f_search_backpressure: Option<
                    crate::types::nodes_stats::ShardSearchBackpressureStats,
                > = None;
                let mut seen_search_backpressure = false;
                let mut f_search_pipeline: Option<
                    crate::types::nodes_stats::ShardSearchPipelineStats,
                > = None;
                let mut seen_search_pipeline = false;
                let mut f_segment_replication_backpressure: Option<
                    crate::types::nodes_stats::ShardSegmentReplicationBackpressureStats,
                > = None;
                let mut seen_segment_replication_backpressure = false;
                let mut f_shard_indexing_pressure: Option<
                    crate::types::nodes_stats::ShardIndexingPressureStats,
                > = None;
                let mut seen_shard_indexing_pressure = false;
                let mut f_task_cancellation: Option<
                    crate::types::nodes_stats::ShardTaskCancellationStats,
                > = None;
                let mut seen_task_cancellation = false;
                let mut f_thread_pool: Option<
                    std::collections::BTreeMap<String, crate::types::nodes_stats::ThreadCount>,
                > = None;
                let mut seen_thread_pool = false;
                let mut f_timestamp: Option<crate::types::common::EpochTimeUnitMillis> = None;
                let mut seen_timestamp = false;
                let mut f_transport: Option<crate::types::nodes_stats::Transport> = None;
                let mut seen_transport = false;
                let mut f_transport_address: Option<crate::types::common::TransportAddress> = None;
                let mut seen_transport_address = false;
                let mut f_weighted_routing: Option<
                    crate::types::nodes_stats::ShardWeightedRoutingStats,
                > = None;
                let mut seen_weighted_routing = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "adaptive_selection" => {
                            if seen_adaptive_selection {
                                return Err(serde::de::Error::duplicate_field(
                                    "adaptive_selection",
                                ));
                            }
                            seen_adaptive_selection = true;
                            f_adaptive_selection = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::nodes_stats::AdaptiveSelection,
                                >,
                            >>()?;
                        }
                        "admission_control" => {
                            if seen_admission_control {
                                return Err(serde::de::Error::duplicate_field("admission_control"));
                            }
                            seen_admission_control = true;
                            f_admission_control = map.next_value::<Option<crate::types::nodes_stats::ShardAdmissionControlStats>>()?;
                        }
                        "attributes" => {
                            if seen_attributes {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            seen_attributes = true;
                            f_attributes = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "breakers" => {
                            if seen_breakers {
                                return Err(serde::de::Error::duplicate_field("breakers"));
                            }
                            seen_breakers = true;
                            f_breakers = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::nodes_stats::Breaker,
                                >,
                            >>()?;
                        }
                        "caches" => {
                            if seen_caches {
                                return Err(serde::de::Error::duplicate_field("caches"));
                            }
                            seen_caches = true;
                            f_caches =
                                map.next_value::<Option<crate::types::nodes_stats::CachesStats>>()?;
                        }
                        "cluster_manager_throttling" => {
                            if seen_cluster_manager_throttling {
                                return Err(serde::de::Error::duplicate_field(
                                    "cluster_manager_throttling",
                                ));
                            }
                            seen_cluster_manager_throttling = true;
                            f_cluster_manager_throttling = map.next_value::<Option<
                                crate::types::nodes_stats::ShardClusterManagerThrottlingStats,
                            >>()?;
                        }
                        "discovery" => {
                            if seen_discovery {
                                return Err(serde::de::Error::duplicate_field("discovery"));
                            }
                            seen_discovery = true;
                            f_discovery =
                                map.next_value::<Option<crate::types::nodes_stats::Discovery>>()?;
                        }
                        "fs" => {
                            if seen_fs {
                                return Err(serde::de::Error::duplicate_field("fs"));
                            }
                            seen_fs = true;
                            f_fs =
                                map.next_value::<Option<crate::types::nodes_stats::FileSystem>>()?;
                        }
                        "host" => {
                            if seen_host {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            seen_host = true;
                            f_host = map.next_value::<Option<crate::types::common::Host>>()?;
                        }
                        "http" => {
                            if seen_http {
                                return Err(serde::de::Error::duplicate_field("http"));
                            }
                            seen_http = true;
                            f_http = map.next_value::<Option<crate::types::nodes_stats::Http>>()?;
                        }
                        "indexing_pressure" => {
                            if seen_indexing_pressure {
                                return Err(serde::de::Error::duplicate_field("indexing_pressure"));
                            }
                            seen_indexing_pressure = true;
                            f_indexing_pressure = map
                                .next_value::<Option<crate::types::nodes_stats::IndexingPressure>>(
                                )?;
                        }
                        "indices" => {
                            if seen_indices {
                                return Err(serde::de::Error::duplicate_field("indices"));
                            }
                            seen_indices = true;
                            f_indices = map
                                .next_value::<Option<crate::types::nodes_stats::NodeIndicesStats>>(
                                )?;
                        }
                        "ingest" => {
                            if seen_ingest {
                                return Err(serde::de::Error::duplicate_field("ingest"));
                            }
                            seen_ingest = true;
                            f_ingest =
                                map.next_value::<Option<crate::types::nodes_stats::Ingest>>()?;
                        }
                        "ip" => {
                            if seen_ip {
                                return Err(serde::de::Error::duplicate_field("ip"));
                            }
                            seen_ip = true;
                            f_ip =
                                map.next_value::<Option<crate::types::nodes_stats::StatsIp>>()?;
                        }
                        "jvm" => {
                            if seen_jvm {
                                return Err(serde::de::Error::duplicate_field("jvm"));
                            }
                            seen_jvm = true;
                            f_jvm = map.next_value::<Option<crate::types::nodes_stats::Jvm>>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "os" => {
                            if seen_os {
                                return Err(serde::de::Error::duplicate_field("os"));
                            }
                            seen_os = true;
                            f_os = map
                                .next_value::<Option<crate::types::nodes_stats::OperatingSystem>>(
                                )?;
                        }
                        "process" => {
                            if seen_process {
                                return Err(serde::de::Error::duplicate_field("process"));
                            }
                            seen_process = true;
                            f_process =
                                map.next_value::<Option<crate::types::nodes_stats::Process>>()?;
                        }
                        "remote_store" => {
                            if seen_remote_store {
                                return Err(serde::de::Error::duplicate_field("remote_store"));
                            }
                            seen_remote_store = true;
                            f_remote_store = map
                                .next_value::<Option<crate::types::nodes_stats::RemoteStoreStats>>(
                                )?;
                        }
                        "repositories" => {
                            if seen_repositories {
                                return Err(serde::de::Error::duplicate_field("repositories"));
                            }
                            seen_repositories = true;
                            f_repositories = map.next_value::<Option<crate::types::nodes_stats::ShardRepositoriesStats>>()?;
                        }
                        "resource_usage_stats" => {
                            if seen_resource_usage_stats {
                                return Err(serde::de::Error::duplicate_field(
                                    "resource_usage_stats",
                                ));
                            }
                            seen_resource_usage_stats = true;
                            f_resource_usage_stats = map.next_value::<Option<crate::types::nodes_stats::ShardResourceUsageStats>>()?;
                        }
                        "roles" => {
                            if seen_roles {
                                return Err(serde::de::Error::duplicate_field("roles"));
                            }
                            seen_roles = true;
                            f_roles =
                                map.next_value::<Option<crate::types::common::NodeRoles>>()?;
                        }
                        "script" => {
                            if seen_script {
                                return Err(serde::de::Error::duplicate_field("script"));
                            }
                            seen_script = true;
                            f_script =
                                map.next_value::<Option<crate::types::nodes_stats::ScriptStats>>()?;
                        }
                        "script_cache" => {
                            if seen_script_cache {
                                return Err(serde::de::Error::duplicate_field("script_cache"));
                            }
                            seen_script_cache = true;
                            f_script_cache = map
                                .next_value::<Option<crate::types::nodes_stats::ScriptCacheStats>>(
                                )?;
                        }
                        "search_backpressure" => {
                            if seen_search_backpressure {
                                return Err(serde::de::Error::duplicate_field(
                                    "search_backpressure",
                                ));
                            }
                            seen_search_backpressure = true;
                            f_search_backpressure = map.next_value::<Option<crate::types::nodes_stats::ShardSearchBackpressureStats>>()?;
                        }
                        "search_pipeline" => {
                            if seen_search_pipeline {
                                return Err(serde::de::Error::duplicate_field("search_pipeline"));
                            }
                            seen_search_pipeline = true;
                            f_search_pipeline = map.next_value::<Option<crate::types::nodes_stats::ShardSearchPipelineStats>>()?;
                        }
                        "segment_replication_backpressure" => {
                            if seen_segment_replication_backpressure {
                                return Err(serde::de::Error::duplicate_field(
                                    "segment_replication_backpressure",
                                ));
                            }
                            seen_segment_replication_backpressure = true;
                            f_segment_replication_backpressure = map.next_value::<Option<
                                crate::types::nodes_stats::ShardSegmentReplicationBackpressureStats,
                            >>(
                            )?;
                        }
                        "shard_indexing_pressure" => {
                            if seen_shard_indexing_pressure {
                                return Err(serde::de::Error::duplicate_field(
                                    "shard_indexing_pressure",
                                ));
                            }
                            seen_shard_indexing_pressure = true;
                            f_shard_indexing_pressure = map.next_value::<Option<crate::types::nodes_stats::ShardIndexingPressureStats>>()?;
                        }
                        "task_cancellation" => {
                            if seen_task_cancellation {
                                return Err(serde::de::Error::duplicate_field("task_cancellation"));
                            }
                            seen_task_cancellation = true;
                            f_task_cancellation = map.next_value::<Option<crate::types::nodes_stats::ShardTaskCancellationStats>>()?;
                        }
                        "thread_pool" => {
                            if seen_thread_pool {
                                return Err(serde::de::Error::duplicate_field("thread_pool"));
                            }
                            seen_thread_pool = true;
                            f_thread_pool = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::nodes_stats::ThreadCount,
                                >,
                            >>()?;
                        }
                        "timestamp" => {
                            if seen_timestamp {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            seen_timestamp = true;
                            f_timestamp = map
                                .next_value::<Option<crate::types::common::EpochTimeUnitMillis>>(
                                )?;
                        }
                        "transport" => {
                            if seen_transport {
                                return Err(serde::de::Error::duplicate_field("transport"));
                            }
                            seen_transport = true;
                            f_transport =
                                map.next_value::<Option<crate::types::nodes_stats::Transport>>()?;
                        }
                        "transport_address" => {
                            if seen_transport_address {
                                return Err(serde::de::Error::duplicate_field("transport_address"));
                            }
                            seen_transport_address = true;
                            f_transport_address =
                                map.next_value::<Option<crate::types::common::TransportAddress>>()?;
                        }
                        "weighted_routing" => {
                            if seen_weighted_routing {
                                return Err(serde::de::Error::duplicate_field("weighted_routing"));
                            }
                            seen_weighted_routing = true;
                            f_weighted_routing = map.next_value::<Option<crate::types::nodes_stats::ShardWeightedRoutingStats>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Stats {
                    adaptive_selection: f_adaptive_selection,
                    admission_control: f_admission_control,
                    attributes: f_attributes,
                    breakers: f_breakers,
                    caches: f_caches,
                    cluster_manager_throttling: f_cluster_manager_throttling,
                    discovery: f_discovery,
                    fs: f_fs,
                    host: f_host,
                    http: f_http,
                    indexing_pressure: f_indexing_pressure,
                    indices: f_indices,
                    ingest: f_ingest,
                    ip: f_ip,
                    jvm: f_jvm,
                    name: f_name,
                    os: f_os,
                    process: f_process,
                    remote_store: f_remote_store,
                    repositories: f_repositories,
                    resource_usage_stats: f_resource_usage_stats,
                    roles: f_roles,
                    script: f_script,
                    script_cache: f_script_cache,
                    search_backpressure: f_search_backpressure,
                    search_pipeline: f_search_pipeline,
                    segment_replication_backpressure: f_segment_replication_backpressure,
                    shard_indexing_pressure: f_shard_indexing_pressure,
                    task_cancellation: f_task_cancellation,
                    thread_pool: f_thread_pool,
                    timestamp: f_timestamp,
                    transport: f_transport,
                    transport_address: f_transport_address,
                    weighted_routing: f_weighted_routing,
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
        if let Some(v) = &self.adaptive_selection {
            map.serialize_entry("adaptive_selection", v)?;
        }
        if let Some(v) = &self.admission_control {
            map.serialize_entry("admission_control", v)?;
        }
        if let Some(v) = &self.attributes {
            map.serialize_entry("attributes", v)?;
        }
        if let Some(v) = &self.breakers {
            map.serialize_entry("breakers", v)?;
        }
        if let Some(v) = &self.caches {
            map.serialize_entry("caches", v)?;
        }
        if let Some(v) = &self.cluster_manager_throttling {
            map.serialize_entry("cluster_manager_throttling", v)?;
        }
        if let Some(v) = &self.discovery {
            map.serialize_entry("discovery", v)?;
        }
        if let Some(v) = &self.fs {
            map.serialize_entry("fs", v)?;
        }
        if let Some(v) = &self.host {
            map.serialize_entry("host", v)?;
        }
        if let Some(v) = &self.http {
            map.serialize_entry("http", v)?;
        }
        if let Some(v) = &self.indexing_pressure {
            map.serialize_entry("indexing_pressure", v)?;
        }
        if let Some(v) = &self.indices {
            map.serialize_entry("indices", v)?;
        }
        if let Some(v) = &self.ingest {
            map.serialize_entry("ingest", v)?;
        }
        if let Some(v) = &self.ip {
            map.serialize_entry("ip", v)?;
        }
        if let Some(v) = &self.jvm {
            map.serialize_entry("jvm", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.os {
            map.serialize_entry("os", v)?;
        }
        if let Some(v) = &self.process {
            map.serialize_entry("process", v)?;
        }
        if let Some(v) = &self.remote_store {
            map.serialize_entry("remote_store", v)?;
        }
        if let Some(v) = &self.repositories {
            map.serialize_entry("repositories", v)?;
        }
        if let Some(v) = &self.resource_usage_stats {
            map.serialize_entry("resource_usage_stats", v)?;
        }
        if let Some(v) = &self.roles {
            map.serialize_entry("roles", v)?;
        }
        if let Some(v) = &self.script {
            map.serialize_entry("script", v)?;
        }
        if let Some(v) = &self.script_cache {
            map.serialize_entry("script_cache", v)?;
        }
        if let Some(v) = &self.search_backpressure {
            map.serialize_entry("search_backpressure", v)?;
        }
        if let Some(v) = &self.search_pipeline {
            map.serialize_entry("search_pipeline", v)?;
        }
        if let Some(v) = &self.segment_replication_backpressure {
            map.serialize_entry("segment_replication_backpressure", v)?;
        }
        if let Some(v) = &self.shard_indexing_pressure {
            map.serialize_entry("shard_indexing_pressure", v)?;
        }
        if let Some(v) = &self.task_cancellation {
            map.serialize_entry("task_cancellation", v)?;
        }
        if let Some(v) = &self.thread_pool {
            map.serialize_entry("thread_pool", v)?;
        }
        if let Some(v) = &self.timestamp {
            map.serialize_entry("timestamp", v)?;
        }
        if let Some(v) = &self.transport {
            map.serialize_entry("transport", v)?;
        }
        if let Some(v) = &self.transport_address {
            map.serialize_entry("transport_address", v)?;
        }
        if let Some(v) = &self.weighted_routing {
            map.serialize_entry("weighted_routing", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "IP address and port for the node."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum StatsIp {
    Ip(crate::types::common::Ip),
    Array(Vec<crate::types::common::Ip>),
}
impl<'de> serde::Deserialize<'de> for StatsIp {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'"') => serde_json::from_str(text).map(StatsIp::Ip),
            Some(b'[') => serde_json::from_str(text).map(StatsIp::Array),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a StatsIp value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The response of the `nodes.stats` API"]
pub type StatsResponse = crate::types::nodes::StatsResponse;

#[derive(Debug, Clone)]
pub struct ThreadCount {
    #[doc = "Number of active threads in the thread pool."]
    pub active: Option<i64>,
    #[doc = "Number of tasks completed by the thread pool executor."]
    pub completed: Option<i64>,
    #[doc = "Highest number of active threads in the thread pool."]
    pub largest: Option<i64>,
    #[doc = "Number of tasks in queue for the thread pool."]
    pub queue: Option<i64>,
    #[doc = "Number of tasks rejected by the thread pool executor."]
    pub rejected: Option<i64>,
    #[doc = "Number of threads in the thread pool."]
    pub threads: Option<i64>,
    #[doc = "The total amount of time that tasks spend waiting in the thread pool queue. Currently, only `search`, `search_throttled`, and `index_searcher` thread pools support this metric. "]
    pub total_wait_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time that tasks spend waiting in the thread pool queue. Currently, only `search`, `search_throttled`, and `index_searcher` thread pools support this metric. "]
    pub total_wait_time_in_nanos: Option<crate::types::common::DurationValueUnitNanos>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ThreadCount {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ThreadCount;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ThreadCount object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_active: Option<i64> = None;
                let mut seen_active = false;
                let mut f_completed: Option<i64> = None;
                let mut seen_completed = false;
                let mut f_largest: Option<i64> = None;
                let mut seen_largest = false;
                let mut f_queue: Option<i64> = None;
                let mut seen_queue = false;
                let mut f_rejected: Option<i64> = None;
                let mut seen_rejected = false;
                let mut f_threads: Option<i64> = None;
                let mut seen_threads = false;
                let mut f_total_wait_time: Option<crate::types::common::Duration> = None;
                let mut seen_total_wait_time = false;
                let mut f_total_wait_time_in_nanos: Option<
                    crate::types::common::DurationValueUnitNanos,
                > = None;
                let mut seen_total_wait_time_in_nanos = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "active" => {
                            if seen_active {
                                return Err(serde::de::Error::duplicate_field("active"));
                            }
                            seen_active = true;
                            f_active = map.next_value::<Option<i64>>()?;
                        }
                        "completed" => {
                            if seen_completed {
                                return Err(serde::de::Error::duplicate_field("completed"));
                            }
                            seen_completed = true;
                            f_completed = map.next_value::<Option<i64>>()?;
                        }
                        "largest" => {
                            if seen_largest {
                                return Err(serde::de::Error::duplicate_field("largest"));
                            }
                            seen_largest = true;
                            f_largest = map.next_value::<Option<i64>>()?;
                        }
                        "queue" => {
                            if seen_queue {
                                return Err(serde::de::Error::duplicate_field("queue"));
                            }
                            seen_queue = true;
                            f_queue = map.next_value::<Option<i64>>()?;
                        }
                        "rejected" => {
                            if seen_rejected {
                                return Err(serde::de::Error::duplicate_field("rejected"));
                            }
                            seen_rejected = true;
                            f_rejected = map.next_value::<Option<i64>>()?;
                        }
                        "threads" => {
                            if seen_threads {
                                return Err(serde::de::Error::duplicate_field("threads"));
                            }
                            seen_threads = true;
                            f_threads = map.next_value::<Option<i64>>()?;
                        }
                        "total_wait_time" => {
                            if seen_total_wait_time {
                                return Err(serde::de::Error::duplicate_field("total_wait_time"));
                            }
                            seen_total_wait_time = true;
                            f_total_wait_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "total_wait_time_in_nanos" => {
                            if seen_total_wait_time_in_nanos {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_wait_time_in_nanos",
                                ));
                            }
                            seen_total_wait_time_in_nanos = true;
                            f_total_wait_time_in_nanos = map
                                .next_value::<Option<crate::types::common::DurationValueUnitNanos>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ThreadCount {
                    active: f_active,
                    completed: f_completed,
                    largest: f_largest,
                    queue: f_queue,
                    rejected: f_rejected,
                    threads: f_threads,
                    total_wait_time: f_total_wait_time,
                    total_wait_time_in_nanos: f_total_wait_time_in_nanos,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ThreadCount {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.active {
            map.serialize_entry("active", v)?;
        }
        if let Some(v) = &self.completed {
            map.serialize_entry("completed", v)?;
        }
        if let Some(v) = &self.largest {
            map.serialize_entry("largest", v)?;
        }
        if let Some(v) = &self.queue {
            map.serialize_entry("queue", v)?;
        }
        if let Some(v) = &self.rejected {
            map.serialize_entry("rejected", v)?;
        }
        if let Some(v) = &self.threads {
            map.serialize_entry("threads", v)?;
        }
        if let Some(v) = &self.total_wait_time {
            map.serialize_entry("total_wait_time", v)?;
        }
        if let Some(v) = &self.total_wait_time_in_nanos {
            map.serialize_entry("total_wait_time_in_nanos", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Transport {
    #[doc = "Total number of RX (receive) packets received by the node during internal cluster communication."]
    pub rx_count: Option<i64>,
    #[doc = "Size of RX packets received by the node during internal cluster communication."]
    pub rx_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Size, in bytes, of RX packets received by the node during internal cluster communication."]
    pub rx_size_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Current number of inbound TCP connections used for internal communication between nodes."]
    pub server_open: Option<i64>,
    #[doc = "The cumulative number of outbound transport connections that this node has opened since it started. Each transport connection may comprise multiple TCP connections but is only counted once in this statistic. Transport connections are typically long-lived so this statistic should remain constant in a stable cluster."]
    pub total_outbound_connections: Option<i64>,
    #[doc = "Total number of TX (transmit) packets sent by the node during internal cluster communication."]
    pub tx_count: Option<i64>,
    #[doc = "Size of TX packets sent by the node during internal cluster communication."]
    pub tx_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Size, in bytes, of TX packets sent by the node during internal cluster communication."]
    pub tx_size_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Transport {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Transport;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Transport object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_rx_count: Option<i64> = None;
                let mut seen_rx_count = false;
                let mut f_rx_size: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_rx_size = false;
                let mut f_rx_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_rx_size_in_bytes = false;
                let mut f_server_open: Option<i64> = None;
                let mut seen_server_open = false;
                let mut f_total_outbound_connections: Option<i64> = None;
                let mut seen_total_outbound_connections = false;
                let mut f_tx_count: Option<i64> = None;
                let mut seen_tx_count = false;
                let mut f_tx_size: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_tx_size = false;
                let mut f_tx_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_tx_size_in_bytes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "rx_count" => {
                            if seen_rx_count {
                                return Err(serde::de::Error::duplicate_field("rx_count"));
                            }
                            seen_rx_count = true;
                            f_rx_count = map.next_value::<Option<i64>>()?;
                        }
                        "rx_size" => {
                            if seen_rx_size {
                                return Err(serde::de::Error::duplicate_field("rx_size"));
                            }
                            seen_rx_size = true;
                            f_rx_size = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "rx_size_in_bytes" => {
                            if seen_rx_size_in_bytes {
                                return Err(serde::de::Error::duplicate_field("rx_size_in_bytes"));
                            }
                            seen_rx_size_in_bytes = true;
                            f_rx_size_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "server_open" => {
                            if seen_server_open {
                                return Err(serde::de::Error::duplicate_field("server_open"));
                            }
                            seen_server_open = true;
                            f_server_open = map.next_value::<Option<i64>>()?;
                        }
                        "total_outbound_connections" => {
                            if seen_total_outbound_connections {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_outbound_connections",
                                ));
                            }
                            seen_total_outbound_connections = true;
                            f_total_outbound_connections = map.next_value::<Option<i64>>()?;
                        }
                        "tx_count" => {
                            if seen_tx_count {
                                return Err(serde::de::Error::duplicate_field("tx_count"));
                            }
                            seen_tx_count = true;
                            f_tx_count = map.next_value::<Option<i64>>()?;
                        }
                        "tx_size" => {
                            if seen_tx_size {
                                return Err(serde::de::Error::duplicate_field("tx_size"));
                            }
                            seen_tx_size = true;
                            f_tx_size = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "tx_size_in_bytes" => {
                            if seen_tx_size_in_bytes {
                                return Err(serde::de::Error::duplicate_field("tx_size_in_bytes"));
                            }
                            seen_tx_size_in_bytes = true;
                            f_tx_size_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Transport {
                    rx_count: f_rx_count,
                    rx_size: f_rx_size,
                    rx_size_in_bytes: f_rx_size_in_bytes,
                    server_open: f_server_open,
                    total_outbound_connections: f_total_outbound_connections,
                    tx_count: f_tx_count,
                    tx_size: f_tx_size,
                    tx_size_in_bytes: f_tx_size_in_bytes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Transport {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.rx_count {
            map.serialize_entry("rx_count", v)?;
        }
        if let Some(v) = &self.rx_size {
            map.serialize_entry("rx_size", v)?;
        }
        if let Some(v) = &self.rx_size_in_bytes {
            map.serialize_entry("rx_size_in_bytes", v)?;
        }
        if let Some(v) = &self.server_open {
            map.serialize_entry("server_open", v)?;
        }
        if let Some(v) = &self.total_outbound_connections {
            map.serialize_entry("total_outbound_connections", v)?;
        }
        if let Some(v) = &self.tx_count {
            map.serialize_entry("tx_count", v)?;
        }
        if let Some(v) = &self.tx_size {
            map.serialize_entry("tx_size", v)?;
        }
        if let Some(v) = &self.tx_size_in_bytes {
            map.serialize_entry("tx_size_in_bytes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TransportUsageStats {
    pub rejection_count: Option<std::collections::BTreeMap<String, i64>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TransportUsageStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TransportUsageStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TransportUsageStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_rejection_count: Option<std::collections::BTreeMap<String, i64>> = None;
                let mut seen_rejection_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "rejection_count" => {
                            if seen_rejection_count {
                                return Err(serde::de::Error::duplicate_field("rejection_count"));
                            }
                            seen_rejection_count = true;
                            f_rejection_count = map
                                .next_value::<Option<std::collections::BTreeMap<String, i64>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TransportUsageStats {
                    rejection_count: f_rejection_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TransportUsageStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.rejection_count {
            map.serialize_entry("rejection_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct UsageStats {
    pub transport: Option<crate::types::nodes_stats::TransportUsageStats>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UsageStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UsageStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UsageStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_transport: Option<crate::types::nodes_stats::TransportUsageStats> = None;
                let mut seen_transport = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "transport" => {
                            if seen_transport {
                                return Err(serde::de::Error::duplicate_field("transport"));
                            }
                            seen_transport = true;
                            f_transport = map.next_value::<Option<crate::types::nodes_stats::TransportUsageStats>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UsageStats {
                    transport: f_transport,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UsageStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.transport {
            map.serialize_entry("transport", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
