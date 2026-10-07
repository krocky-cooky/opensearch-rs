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

//! Response types of the `search` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct AggregationBreakdown {
    pub build_aggregation: i64,
    pub build_aggregation_count: i64,
    pub build_leaf_collector: i64,
    pub build_leaf_collector_count: i64,
    pub collect: i64,
    pub collect_count: i64,
    pub initialize: i64,
    pub initialize_count: i64,
    pub post_collection: Option<i64>,
    pub post_collection_count: Option<i64>,
    pub reduce: i64,
    pub reduce_count: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AggregationBreakdown {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AggregationBreakdown;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AggregationBreakdown object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_build_aggregation: Option<i64> = None;
                let mut f_build_aggregation_count: Option<i64> = None;
                let mut f_build_leaf_collector: Option<i64> = None;
                let mut f_build_leaf_collector_count: Option<i64> = None;
                let mut f_collect: Option<i64> = None;
                let mut f_collect_count: Option<i64> = None;
                let mut f_initialize: Option<i64> = None;
                let mut f_initialize_count: Option<i64> = None;
                let mut f_post_collection: Option<i64> = None;
                let mut seen_post_collection = false;
                let mut f_post_collection_count: Option<i64> = None;
                let mut seen_post_collection_count = false;
                let mut f_reduce: Option<i64> = None;
                let mut f_reduce_count: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "build_aggregation" => {
                            if f_build_aggregation.is_some() {
                                return Err(serde::de::Error::duplicate_field("build_aggregation"));
                            }
                            f_build_aggregation = Some(map.next_value::<i64>()?);
                        }
                        "build_aggregation_count" => {
                            if f_build_aggregation_count.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "build_aggregation_count",
                                ));
                            }
                            f_build_aggregation_count = Some(map.next_value::<i64>()?);
                        }
                        "build_leaf_collector" => {
                            if f_build_leaf_collector.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "build_leaf_collector",
                                ));
                            }
                            f_build_leaf_collector = Some(map.next_value::<i64>()?);
                        }
                        "build_leaf_collector_count" => {
                            if f_build_leaf_collector_count.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "build_leaf_collector_count",
                                ));
                            }
                            f_build_leaf_collector_count = Some(map.next_value::<i64>()?);
                        }
                        "collect" => {
                            if f_collect.is_some() {
                                return Err(serde::de::Error::duplicate_field("collect"));
                            }
                            f_collect = Some(map.next_value::<i64>()?);
                        }
                        "collect_count" => {
                            if f_collect_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("collect_count"));
                            }
                            f_collect_count = Some(map.next_value::<i64>()?);
                        }
                        "initialize" => {
                            if f_initialize.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialize"));
                            }
                            f_initialize = Some(map.next_value::<i64>()?);
                        }
                        "initialize_count" => {
                            if f_initialize_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialize_count"));
                            }
                            f_initialize_count = Some(map.next_value::<i64>()?);
                        }
                        "post_collection" => {
                            if seen_post_collection {
                                return Err(serde::de::Error::duplicate_field("post_collection"));
                            }
                            seen_post_collection = true;
                            f_post_collection = map.next_value::<Option<i64>>()?;
                        }
                        "post_collection_count" => {
                            if seen_post_collection_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "post_collection_count",
                                ));
                            }
                            seen_post_collection_count = true;
                            f_post_collection_count = map.next_value::<Option<i64>>()?;
                        }
                        "reduce" => {
                            if f_reduce.is_some() {
                                return Err(serde::de::Error::duplicate_field("reduce"));
                            }
                            f_reduce = Some(map.next_value::<i64>()?);
                        }
                        "reduce_count" => {
                            if f_reduce_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("reduce_count"));
                            }
                            f_reduce_count = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AggregationBreakdown {
                    build_aggregation: f_build_aggregation
                        .ok_or_else(|| serde::de::Error::missing_field("build_aggregation"))?,
                    build_aggregation_count: f_build_aggregation_count.ok_or_else(|| {
                        serde::de::Error::missing_field("build_aggregation_count")
                    })?,
                    build_leaf_collector: f_build_leaf_collector
                        .ok_or_else(|| serde::de::Error::missing_field("build_leaf_collector"))?,
                    build_leaf_collector_count: f_build_leaf_collector_count.ok_or_else(|| {
                        serde::de::Error::missing_field("build_leaf_collector_count")
                    })?,
                    collect: f_collect.ok_or_else(|| serde::de::Error::missing_field("collect"))?,
                    collect_count: f_collect_count
                        .ok_or_else(|| serde::de::Error::missing_field("collect_count"))?,
                    initialize: f_initialize
                        .ok_or_else(|| serde::de::Error::missing_field("initialize"))?,
                    initialize_count: f_initialize_count
                        .ok_or_else(|| serde::de::Error::missing_field("initialize_count"))?,
                    post_collection: f_post_collection,
                    post_collection_count: f_post_collection_count,
                    reduce: f_reduce.ok_or_else(|| serde::de::Error::missing_field("reduce"))?,
                    reduce_count: f_reduce_count
                        .ok_or_else(|| serde::de::Error::missing_field("reduce_count"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AggregationBreakdown {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("build_aggregation", &self.build_aggregation)?;
        map.serialize_entry("build_aggregation_count", &self.build_aggregation_count)?;
        map.serialize_entry("build_leaf_collector", &self.build_leaf_collector)?;
        map.serialize_entry(
            "build_leaf_collector_count",
            &self.build_leaf_collector_count,
        )?;
        map.serialize_entry("collect", &self.collect)?;
        map.serialize_entry("collect_count", &self.collect_count)?;
        map.serialize_entry("initialize", &self.initialize)?;
        map.serialize_entry("initialize_count", &self.initialize_count)?;
        if let Some(v) = &self.post_collection {
            map.serialize_entry("post_collection", v)?;
        }
        if let Some(v) = &self.post_collection_count {
            map.serialize_entry("post_collection_count", v)?;
        }
        map.serialize_entry("reduce", &self.reduce)?;
        map.serialize_entry("reduce_count", &self.reduce_count)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct AggregationProfile {
    pub breakdown: crate::types::search::AggregationBreakdown,
    pub children: Option<Vec<crate::types::search::AggregationProfile>>,
    pub debug: Option<crate::types::search::AggregationProfileDebug>,
    pub description: String,
    pub time_in_nanos: crate::types::common::DurationValueUnitNanos,
    pub r#type: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AggregationProfile {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AggregationProfile;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AggregationProfile object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_breakdown: Option<crate::types::search::AggregationBreakdown> = None;
                let mut f_children: Option<Vec<crate::types::search::AggregationProfile>> = None;
                let mut seen_children = false;
                let mut f_debug: Option<crate::types::search::AggregationProfileDebug> = None;
                let mut seen_debug = false;
                let mut f_description: Option<String> = None;
                let mut f_time_in_nanos: Option<crate::types::common::DurationValueUnitNanos> =
                    None;
                let mut f_type: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "breakdown" => {
                            if f_breakdown.is_some() {
                                return Err(serde::de::Error::duplicate_field("breakdown"));
                            }
                            f_breakdown = Some(
                                map.next_value::<crate::types::search::AggregationBreakdown>()?,
                            );
                        }
                        "children" => {
                            if seen_children {
                                return Err(serde::de::Error::duplicate_field("children"));
                            }
                            seen_children = true;
                            f_children = map.next_value::<Option<Vec<crate::types::search::AggregationProfile>>>()?;
                        }
                        "debug" => {
                            if seen_debug {
                                return Err(serde::de::Error::duplicate_field("debug"));
                            }
                            seen_debug = true;
                            f_debug = map.next_value::<Option<crate::types::search::AggregationProfileDebug>>()?;
                        }
                        "description" => {
                            if f_description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            f_description = Some(map.next_value::<String>()?);
                        }
                        "time_in_nanos" => {
                            if f_time_in_nanos.is_some() {
                                return Err(serde::de::Error::duplicate_field("time_in_nanos"));
                            }
                            f_time_in_nanos = Some(
                                map.next_value::<crate::types::common::DurationValueUnitNanos>()?,
                            );
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AggregationProfile {
                    breakdown: f_breakdown
                        .ok_or_else(|| serde::de::Error::missing_field("breakdown"))?,
                    children: f_children,
                    debug: f_debug,
                    description: f_description
                        .ok_or_else(|| serde::de::Error::missing_field("description"))?,
                    time_in_nanos: f_time_in_nanos
                        .ok_or_else(|| serde::de::Error::missing_field("time_in_nanos"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AggregationProfile {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("breakdown", &self.breakdown)?;
        if let Some(v) = &self.children {
            map.serialize_entry("children", v)?;
        }
        if let Some(v) = &self.debug {
            map.serialize_entry("debug", v)?;
        }
        map.serialize_entry("description", &self.description)?;
        map.serialize_entry("time_in_nanos", &self.time_in_nanos)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct AggregationProfileDebug {
    pub built_buckets: Option<i32>,
    pub chars_fetched: Option<i32>,
    pub collect_analyzed_count: Option<i32>,
    pub collect_analyzed_ns: Option<i32>,
    pub collection_strategy: Option<String>,
    pub deferred_aggregators: Option<Vec<String>>,
    pub delegate: Option<String>,
    pub delegate_debug: Option<crate::types::search::AggregationProfileDelegateDebug>,
    pub empty_collectors_used: Option<i32>,
    pub extract_count: Option<i32>,
    pub extract_ns: Option<i32>,
    pub has_filter: Option<bool>,
    pub map_reducer: Option<String>,
    pub numeric_collectors_used: Option<i32>,
    pub ordinals_collectors_overhead_too_high: Option<i32>,
    pub ordinals_collectors_used: Option<i32>,
    pub result_strategy: Option<String>,
    pub segments_with_multi_valued_ords: Option<i32>,
    pub segments_with_single_valued_ords: Option<i32>,
    pub string_hashing_collectors_used: Option<i32>,
    pub surviving_buckets: Option<i32>,
    pub total_buckets: Option<i32>,
    pub values_fetched: Option<i32>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AggregationProfileDebug {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AggregationProfileDebug;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AggregationProfileDebug object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_built_buckets: Option<i32> = None;
                let mut seen_built_buckets = false;
                let mut f_chars_fetched: Option<i32> = None;
                let mut seen_chars_fetched = false;
                let mut f_collect_analyzed_count: Option<i32> = None;
                let mut seen_collect_analyzed_count = false;
                let mut f_collect_analyzed_ns: Option<i32> = None;
                let mut seen_collect_analyzed_ns = false;
                let mut f_collection_strategy: Option<String> = None;
                let mut seen_collection_strategy = false;
                let mut f_deferred_aggregators: Option<Vec<String>> = None;
                let mut seen_deferred_aggregators = false;
                let mut f_delegate: Option<String> = None;
                let mut seen_delegate = false;
                let mut f_delegate_debug: Option<
                    crate::types::search::AggregationProfileDelegateDebug,
                > = None;
                let mut seen_delegate_debug = false;
                let mut f_empty_collectors_used: Option<i32> = None;
                let mut seen_empty_collectors_used = false;
                let mut f_extract_count: Option<i32> = None;
                let mut seen_extract_count = false;
                let mut f_extract_ns: Option<i32> = None;
                let mut seen_extract_ns = false;
                let mut f_has_filter: Option<bool> = None;
                let mut seen_has_filter = false;
                let mut f_map_reducer: Option<String> = None;
                let mut seen_map_reducer = false;
                let mut f_numeric_collectors_used: Option<i32> = None;
                let mut seen_numeric_collectors_used = false;
                let mut f_ordinals_collectors_overhead_too_high: Option<i32> = None;
                let mut seen_ordinals_collectors_overhead_too_high = false;
                let mut f_ordinals_collectors_used: Option<i32> = None;
                let mut seen_ordinals_collectors_used = false;
                let mut f_result_strategy: Option<String> = None;
                let mut seen_result_strategy = false;
                let mut f_segments_with_multi_valued_ords: Option<i32> = None;
                let mut seen_segments_with_multi_valued_ords = false;
                let mut f_segments_with_single_valued_ords: Option<i32> = None;
                let mut seen_segments_with_single_valued_ords = false;
                let mut f_string_hashing_collectors_used: Option<i32> = None;
                let mut seen_string_hashing_collectors_used = false;
                let mut f_surviving_buckets: Option<i32> = None;
                let mut seen_surviving_buckets = false;
                let mut f_total_buckets: Option<i32> = None;
                let mut seen_total_buckets = false;
                let mut f_values_fetched: Option<i32> = None;
                let mut seen_values_fetched = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "built_buckets" => {
                            if seen_built_buckets {
                                return Err(serde::de::Error::duplicate_field("built_buckets"));
                            }
                            seen_built_buckets = true;
                            f_built_buckets = map.next_value::<Option<i32>>()?;
                        }
                        "chars_fetched" => {
                            if seen_chars_fetched {
                                return Err(serde::de::Error::duplicate_field("chars_fetched"));
                            }
                            seen_chars_fetched = true;
                            f_chars_fetched = map.next_value::<Option<i32>>()?;
                        }
                        "collect_analyzed_count" => {
                            if seen_collect_analyzed_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "collect_analyzed_count",
                                ));
                            }
                            seen_collect_analyzed_count = true;
                            f_collect_analyzed_count = map.next_value::<Option<i32>>()?;
                        }
                        "collect_analyzed_ns" => {
                            if seen_collect_analyzed_ns {
                                return Err(serde::de::Error::duplicate_field(
                                    "collect_analyzed_ns",
                                ));
                            }
                            seen_collect_analyzed_ns = true;
                            f_collect_analyzed_ns = map.next_value::<Option<i32>>()?;
                        }
                        "collection_strategy" => {
                            if seen_collection_strategy {
                                return Err(serde::de::Error::duplicate_field(
                                    "collection_strategy",
                                ));
                            }
                            seen_collection_strategy = true;
                            f_collection_strategy = map.next_value::<Option<String>>()?;
                        }
                        "deferred_aggregators" => {
                            if seen_deferred_aggregators {
                                return Err(serde::de::Error::duplicate_field(
                                    "deferred_aggregators",
                                ));
                            }
                            seen_deferred_aggregators = true;
                            f_deferred_aggregators = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "delegate" => {
                            if seen_delegate {
                                return Err(serde::de::Error::duplicate_field("delegate"));
                            }
                            seen_delegate = true;
                            f_delegate = map.next_value::<Option<String>>()?;
                        }
                        "delegate_debug" => {
                            if seen_delegate_debug {
                                return Err(serde::de::Error::duplicate_field("delegate_debug"));
                            }
                            seen_delegate_debug = true;
                            f_delegate_debug = map.next_value::<Option<crate::types::search::AggregationProfileDelegateDebug>>()?;
                        }
                        "empty_collectors_used" => {
                            if seen_empty_collectors_used {
                                return Err(serde::de::Error::duplicate_field(
                                    "empty_collectors_used",
                                ));
                            }
                            seen_empty_collectors_used = true;
                            f_empty_collectors_used = map.next_value::<Option<i32>>()?;
                        }
                        "extract_count" => {
                            if seen_extract_count {
                                return Err(serde::de::Error::duplicate_field("extract_count"));
                            }
                            seen_extract_count = true;
                            f_extract_count = map.next_value::<Option<i32>>()?;
                        }
                        "extract_ns" => {
                            if seen_extract_ns {
                                return Err(serde::de::Error::duplicate_field("extract_ns"));
                            }
                            seen_extract_ns = true;
                            f_extract_ns = map.next_value::<Option<i32>>()?;
                        }
                        "has_filter" => {
                            if seen_has_filter {
                                return Err(serde::de::Error::duplicate_field("has_filter"));
                            }
                            seen_has_filter = true;
                            f_has_filter = map.next_value::<Option<bool>>()?;
                        }
                        "map_reducer" => {
                            if seen_map_reducer {
                                return Err(serde::de::Error::duplicate_field("map_reducer"));
                            }
                            seen_map_reducer = true;
                            f_map_reducer = map.next_value::<Option<String>>()?;
                        }
                        "numeric_collectors_used" => {
                            if seen_numeric_collectors_used {
                                return Err(serde::de::Error::duplicate_field(
                                    "numeric_collectors_used",
                                ));
                            }
                            seen_numeric_collectors_used = true;
                            f_numeric_collectors_used = map.next_value::<Option<i32>>()?;
                        }
                        "ordinals_collectors_overhead_too_high" => {
                            if seen_ordinals_collectors_overhead_too_high {
                                return Err(serde::de::Error::duplicate_field(
                                    "ordinals_collectors_overhead_too_high",
                                ));
                            }
                            seen_ordinals_collectors_overhead_too_high = true;
                            f_ordinals_collectors_overhead_too_high =
                                map.next_value::<Option<i32>>()?;
                        }
                        "ordinals_collectors_used" => {
                            if seen_ordinals_collectors_used {
                                return Err(serde::de::Error::duplicate_field(
                                    "ordinals_collectors_used",
                                ));
                            }
                            seen_ordinals_collectors_used = true;
                            f_ordinals_collectors_used = map.next_value::<Option<i32>>()?;
                        }
                        "result_strategy" => {
                            if seen_result_strategy {
                                return Err(serde::de::Error::duplicate_field("result_strategy"));
                            }
                            seen_result_strategy = true;
                            f_result_strategy = map.next_value::<Option<String>>()?;
                        }
                        "segments_with_multi_valued_ords" => {
                            if seen_segments_with_multi_valued_ords {
                                return Err(serde::de::Error::duplicate_field(
                                    "segments_with_multi_valued_ords",
                                ));
                            }
                            seen_segments_with_multi_valued_ords = true;
                            f_segments_with_multi_valued_ords = map.next_value::<Option<i32>>()?;
                        }
                        "segments_with_single_valued_ords" => {
                            if seen_segments_with_single_valued_ords {
                                return Err(serde::de::Error::duplicate_field(
                                    "segments_with_single_valued_ords",
                                ));
                            }
                            seen_segments_with_single_valued_ords = true;
                            f_segments_with_single_valued_ords = map.next_value::<Option<i32>>()?;
                        }
                        "string_hashing_collectors_used" => {
                            if seen_string_hashing_collectors_used {
                                return Err(serde::de::Error::duplicate_field(
                                    "string_hashing_collectors_used",
                                ));
                            }
                            seen_string_hashing_collectors_used = true;
                            f_string_hashing_collectors_used = map.next_value::<Option<i32>>()?;
                        }
                        "surviving_buckets" => {
                            if seen_surviving_buckets {
                                return Err(serde::de::Error::duplicate_field("surviving_buckets"));
                            }
                            seen_surviving_buckets = true;
                            f_surviving_buckets = map.next_value::<Option<i32>>()?;
                        }
                        "total_buckets" => {
                            if seen_total_buckets {
                                return Err(serde::de::Error::duplicate_field("total_buckets"));
                            }
                            seen_total_buckets = true;
                            f_total_buckets = map.next_value::<Option<i32>>()?;
                        }
                        "values_fetched" => {
                            if seen_values_fetched {
                                return Err(serde::de::Error::duplicate_field("values_fetched"));
                            }
                            seen_values_fetched = true;
                            f_values_fetched = map.next_value::<Option<i32>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AggregationProfileDebug {
                    built_buckets: f_built_buckets,
                    chars_fetched: f_chars_fetched,
                    collect_analyzed_count: f_collect_analyzed_count,
                    collect_analyzed_ns: f_collect_analyzed_ns,
                    collection_strategy: f_collection_strategy,
                    deferred_aggregators: f_deferred_aggregators,
                    delegate: f_delegate,
                    delegate_debug: f_delegate_debug,
                    empty_collectors_used: f_empty_collectors_used,
                    extract_count: f_extract_count,
                    extract_ns: f_extract_ns,
                    has_filter: f_has_filter,
                    map_reducer: f_map_reducer,
                    numeric_collectors_used: f_numeric_collectors_used,
                    ordinals_collectors_overhead_too_high: f_ordinals_collectors_overhead_too_high,
                    ordinals_collectors_used: f_ordinals_collectors_used,
                    result_strategy: f_result_strategy,
                    segments_with_multi_valued_ords: f_segments_with_multi_valued_ords,
                    segments_with_single_valued_ords: f_segments_with_single_valued_ords,
                    string_hashing_collectors_used: f_string_hashing_collectors_used,
                    surviving_buckets: f_surviving_buckets,
                    total_buckets: f_total_buckets,
                    values_fetched: f_values_fetched,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AggregationProfileDebug {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.built_buckets {
            map.serialize_entry("built_buckets", v)?;
        }
        if let Some(v) = &self.chars_fetched {
            map.serialize_entry("chars_fetched", v)?;
        }
        if let Some(v) = &self.collect_analyzed_count {
            map.serialize_entry("collect_analyzed_count", v)?;
        }
        if let Some(v) = &self.collect_analyzed_ns {
            map.serialize_entry("collect_analyzed_ns", v)?;
        }
        if let Some(v) = &self.collection_strategy {
            map.serialize_entry("collection_strategy", v)?;
        }
        if let Some(v) = &self.deferred_aggregators {
            map.serialize_entry("deferred_aggregators", v)?;
        }
        if let Some(v) = &self.delegate {
            map.serialize_entry("delegate", v)?;
        }
        if let Some(v) = &self.delegate_debug {
            map.serialize_entry("delegate_debug", v)?;
        }
        if let Some(v) = &self.empty_collectors_used {
            map.serialize_entry("empty_collectors_used", v)?;
        }
        if let Some(v) = &self.extract_count {
            map.serialize_entry("extract_count", v)?;
        }
        if let Some(v) = &self.extract_ns {
            map.serialize_entry("extract_ns", v)?;
        }
        if let Some(v) = &self.has_filter {
            map.serialize_entry("has_filter", v)?;
        }
        if let Some(v) = &self.map_reducer {
            map.serialize_entry("map_reducer", v)?;
        }
        if let Some(v) = &self.numeric_collectors_used {
            map.serialize_entry("numeric_collectors_used", v)?;
        }
        if let Some(v) = &self.ordinals_collectors_overhead_too_high {
            map.serialize_entry("ordinals_collectors_overhead_too_high", v)?;
        }
        if let Some(v) = &self.ordinals_collectors_used {
            map.serialize_entry("ordinals_collectors_used", v)?;
        }
        if let Some(v) = &self.result_strategy {
            map.serialize_entry("result_strategy", v)?;
        }
        if let Some(v) = &self.segments_with_multi_valued_ords {
            map.serialize_entry("segments_with_multi_valued_ords", v)?;
        }
        if let Some(v) = &self.segments_with_single_valued_ords {
            map.serialize_entry("segments_with_single_valued_ords", v)?;
        }
        if let Some(v) = &self.string_hashing_collectors_used {
            map.serialize_entry("string_hashing_collectors_used", v)?;
        }
        if let Some(v) = &self.surviving_buckets {
            map.serialize_entry("surviving_buckets", v)?;
        }
        if let Some(v) = &self.total_buckets {
            map.serialize_entry("total_buckets", v)?;
        }
        if let Some(v) = &self.values_fetched {
            map.serialize_entry("values_fetched", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct AggregationProfileDelegateDebug {
    pub filters: Option<Vec<crate::types::search::AggregationProfileDelegateDebugFilter>>,
    pub segments_collected: Option<i32>,
    pub segments_counted: Option<i32>,
    pub segments_with_deleted_docs: Option<i32>,
    pub segments_with_doc_count_field: Option<i32>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AggregationProfileDelegateDebug {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AggregationProfileDelegateDebug;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AggregationProfileDelegateDebug object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_filters: Option<
                    Vec<crate::types::search::AggregationProfileDelegateDebugFilter>,
                > = None;
                let mut seen_filters = false;
                let mut f_segments_collected: Option<i32> = None;
                let mut seen_segments_collected = false;
                let mut f_segments_counted: Option<i32> = None;
                let mut seen_segments_counted = false;
                let mut f_segments_with_deleted_docs: Option<i32> = None;
                let mut seen_segments_with_deleted_docs = false;
                let mut f_segments_with_doc_count_field: Option<i32> = None;
                let mut seen_segments_with_doc_count_field = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "filters" => {
                            if seen_filters {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            seen_filters = true;
                            f_filters = map.next_value::<Option<
                                Vec<crate::types::search::AggregationProfileDelegateDebugFilter>,
                            >>()?;
                        }
                        "segments_collected" => {
                            if seen_segments_collected {
                                return Err(serde::de::Error::duplicate_field(
                                    "segments_collected",
                                ));
                            }
                            seen_segments_collected = true;
                            f_segments_collected = map.next_value::<Option<i32>>()?;
                        }
                        "segments_counted" => {
                            if seen_segments_counted {
                                return Err(serde::de::Error::duplicate_field("segments_counted"));
                            }
                            seen_segments_counted = true;
                            f_segments_counted = map.next_value::<Option<i32>>()?;
                        }
                        "segments_with_deleted_docs" => {
                            if seen_segments_with_deleted_docs {
                                return Err(serde::de::Error::duplicate_field(
                                    "segments_with_deleted_docs",
                                ));
                            }
                            seen_segments_with_deleted_docs = true;
                            f_segments_with_deleted_docs = map.next_value::<Option<i32>>()?;
                        }
                        "segments_with_doc_count_field" => {
                            if seen_segments_with_doc_count_field {
                                return Err(serde::de::Error::duplicate_field(
                                    "segments_with_doc_count_field",
                                ));
                            }
                            seen_segments_with_doc_count_field = true;
                            f_segments_with_doc_count_field = map.next_value::<Option<i32>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AggregationProfileDelegateDebug {
                    filters: f_filters,
                    segments_collected: f_segments_collected,
                    segments_counted: f_segments_counted,
                    segments_with_deleted_docs: f_segments_with_deleted_docs,
                    segments_with_doc_count_field: f_segments_with_doc_count_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AggregationProfileDelegateDebug {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.filters {
            map.serialize_entry("filters", v)?;
        }
        if let Some(v) = &self.segments_collected {
            map.serialize_entry("segments_collected", v)?;
        }
        if let Some(v) = &self.segments_counted {
            map.serialize_entry("segments_counted", v)?;
        }
        if let Some(v) = &self.segments_with_deleted_docs {
            map.serialize_entry("segments_with_deleted_docs", v)?;
        }
        if let Some(v) = &self.segments_with_doc_count_field {
            map.serialize_entry("segments_with_doc_count_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct AggregationProfileDelegateDebugFilter {
    pub query: Option<String>,
    pub results_from_metadata: Option<i32>,
    pub segments_counted_in_constant_time: Option<i32>,
    pub specialized_for: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AggregationProfileDelegateDebugFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AggregationProfileDelegateDebugFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AggregationProfileDelegateDebugFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_query: Option<String> = None;
                let mut seen_query = false;
                let mut f_results_from_metadata: Option<i32> = None;
                let mut seen_results_from_metadata = false;
                let mut f_segments_counted_in_constant_time: Option<i32> = None;
                let mut seen_segments_counted_in_constant_time = false;
                let mut f_specialized_for: Option<String> = None;
                let mut seen_specialized_for = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "query" => {
                            if seen_query {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            seen_query = true;
                            f_query = map.next_value::<Option<String>>()?;
                        }
                        "results_from_metadata" => {
                            if seen_results_from_metadata {
                                return Err(serde::de::Error::duplicate_field(
                                    "results_from_metadata",
                                ));
                            }
                            seen_results_from_metadata = true;
                            f_results_from_metadata = map.next_value::<Option<i32>>()?;
                        }
                        "segments_counted_in_constant_time" => {
                            if seen_segments_counted_in_constant_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "segments_counted_in_constant_time",
                                ));
                            }
                            seen_segments_counted_in_constant_time = true;
                            f_segments_counted_in_constant_time =
                                map.next_value::<Option<i32>>()?;
                        }
                        "specialized_for" => {
                            if seen_specialized_for {
                                return Err(serde::de::Error::duplicate_field("specialized_for"));
                            }
                            seen_specialized_for = true;
                            f_specialized_for = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AggregationProfileDelegateDebugFilter {
                    query: f_query,
                    results_from_metadata: f_results_from_metadata,
                    segments_counted_in_constant_time: f_segments_counted_in_constant_time,
                    specialized_for: f_specialized_for,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AggregationProfileDelegateDebugFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.query {
            map.serialize_entry("query", v)?;
        }
        if let Some(v) = &self.results_from_metadata {
            map.serialize_entry("results_from_metadata", v)?;
        }
        if let Some(v) = &self.segments_counted_in_constant_time {
            map.serialize_entry("segments_counted_in_constant_time", v)?;
        }
        if let Some(v) = &self.specialized_for {
            map.serialize_entry("specialized_for", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BoundaryScanner {
    #[doc = "`chars`"]
    Chars,
    #[doc = "`sentence`"]
    Sentence,
    #[doc = "`word`"]
    Word,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl BoundaryScanner {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            BoundaryScanner::Chars => "chars",
            BoundaryScanner::Sentence => "sentence",
            BoundaryScanner::Word => "word",
            BoundaryScanner::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for BoundaryScanner {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BoundaryScanner {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "chars" => BoundaryScanner::Chars,
            "sentence" => BoundaryScanner::Sentence,
            "word" => BoundaryScanner::Word,
            _ => BoundaryScanner::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuiltinHighlighterType {
    #[doc = "`plain`"]
    Plain,
    #[doc = "`fvh`"]
    Fvh,
    #[doc = "`unified`"]
    Unified,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl BuiltinHighlighterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            BuiltinHighlighterType::Plain => "plain",
            BuiltinHighlighterType::Fvh => "fvh",
            BuiltinHighlighterType::Unified => "unified",
            BuiltinHighlighterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for BuiltinHighlighterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BuiltinHighlighterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "plain" => BuiltinHighlighterType::Plain,
            "fvh" => BuiltinHighlighterType::Fvh,
            "unified" => BuiltinHighlighterType::Unified,
            _ => BuiltinHighlighterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Collector {
    pub children: Option<Vec<crate::types::search::Collector>>,
    pub name: String,
    pub reason: String,
    pub time_in_nanos: crate::types::common::DurationValueUnitNanos,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Collector {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Collector;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Collector object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_children: Option<Vec<crate::types::search::Collector>> = None;
                let mut seen_children = false;
                let mut f_name: Option<String> = None;
                let mut f_reason: Option<String> = None;
                let mut f_time_in_nanos: Option<crate::types::common::DurationValueUnitNanos> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "children" => {
                            if seen_children {
                                return Err(serde::de::Error::duplicate_field("children"));
                            }
                            seen_children = true;
                            f_children =
                                map.next_value::<Option<Vec<crate::types::search::Collector>>>()?;
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<String>()?);
                        }
                        "reason" => {
                            if f_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            f_reason = Some(map.next_value::<String>()?);
                        }
                        "time_in_nanos" => {
                            if f_time_in_nanos.is_some() {
                                return Err(serde::de::Error::duplicate_field("time_in_nanos"));
                            }
                            f_time_in_nanos = Some(
                                map.next_value::<crate::types::common::DurationValueUnitNanos>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Collector {
                    children: f_children,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    reason: f_reason.ok_or_else(|| serde::de::Error::missing_field("reason"))?,
                    time_in_nanos: f_time_in_nanos
                        .ok_or_else(|| serde::de::Error::missing_field("time_in_nanos"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Collector {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.children {
            map.serialize_entry("children", v)?;
        }
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("reason", &self.reason)?;
        map.serialize_entry("time_in_nanos", &self.time_in_nanos)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct CompletionContext {
    raw: Box<serde_json::value::RawValue>,
}
impl CompletionContext {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `context` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_context(
        &self,
    ) -> std::result::Result<crate::types::search::Context, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "context")
    }
    #[doc = "Parses the value as the `object` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_object(
        &self,
    ) -> std::result::Result<
        crate::types::search::CompletionContextObject,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "object")
    }
}

#[derive(Debug, Clone)]
pub struct CompletionContextObject {
    pub boost: Option<f64>,
    pub context: crate::types::search::Context,
    pub neighbours: Option<Vec<crate::types::common::GeoHashPrecision>>,
    pub precision: Option<crate::types::common::GeoHashPrecision>,
    pub prefix: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CompletionContextObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CompletionContextObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CompletionContextObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_context: Option<crate::types::search::Context> = None;
                let mut f_neighbours: Option<Vec<crate::types::common::GeoHashPrecision>> = None;
                let mut seen_neighbours = false;
                let mut f_precision: Option<crate::types::common::GeoHashPrecision> = None;
                let mut seen_precision = false;
                let mut f_prefix: Option<bool> = None;
                let mut seen_prefix = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "context" => {
                            if f_context.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            f_context = Some(map.next_value::<crate::types::search::Context>()?);
                        }
                        "neighbours" => {
                            if seen_neighbours {
                                return Err(serde::de::Error::duplicate_field("neighbours"));
                            }
                            seen_neighbours = true;
                            f_neighbours = map
                                .next_value::<Option<Vec<crate::types::common::GeoHashPrecision>>>(
                                )?;
                        }
                        "precision" => {
                            if seen_precision {
                                return Err(serde::de::Error::duplicate_field("precision"));
                            }
                            seen_precision = true;
                            f_precision =
                                map.next_value::<Option<crate::types::common::GeoHashPrecision>>()?;
                        }
                        "prefix" => {
                            if seen_prefix {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            seen_prefix = true;
                            f_prefix = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CompletionContextObject {
                    boost: f_boost,
                    context: f_context.ok_or_else(|| serde::de::Error::missing_field("context"))?,
                    neighbours: f_neighbours,
                    precision: f_precision,
                    prefix: f_prefix,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CompletionContextObject {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        map.serialize_entry("context", &self.context)?;
        if let Some(v) = &self.neighbours {
            map.serialize_entry("neighbours", v)?;
        }
        if let Some(v) = &self.precision {
            map.serialize_entry("precision", v)?;
        }
        if let Some(v) = &self.prefix {
            map.serialize_entry("prefix", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct CompletionSuggester {
    pub analyzer: Option<String>,
    pub field: String,
    pub size: Option<i32>,
    pub contexts:
        Option<std::collections::BTreeMap<String, Vec<crate::types::search::CompletionContext>>>,
    pub fuzzy: Option<crate::types::search::SuggestFuzziness>,
    pub regex: Option<String>,
    pub skip_duplicates: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CompletionSuggester {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CompletionSuggester;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CompletionSuggester object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_analyzer: Option<String> = None;
                let mut seen_analyzer = false;
                let mut f_field: Option<String> = None;
                let mut f_size: Option<i32> = None;
                let mut seen_size = false;
                let mut f_contexts: Option<
                    std::collections::BTreeMap<
                        String,
                        Vec<crate::types::search::CompletionContext>,
                    >,
                > = None;
                let mut seen_contexts = false;
                let mut f_fuzzy: Option<crate::types::search::SuggestFuzziness> = None;
                let mut seen_fuzzy = false;
                let mut f_regex: Option<String> = None;
                let mut seen_regex = false;
                let mut f_skip_duplicates: Option<bool> = None;
                let mut seen_skip_duplicates = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "analyzer" => {
                            if seen_analyzer {
                                return Err(serde::de::Error::duplicate_field("analyzer"));
                            }
                            seen_analyzer = true;
                            f_analyzer = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<String>()?);
                        }
                        "size" => {
                            if seen_size {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            seen_size = true;
                            f_size = map.next_value::<Option<i32>>()?;
                        }
                        "contexts" => {
                            if seen_contexts {
                                return Err(serde::de::Error::duplicate_field("contexts"));
                            }
                            seen_contexts = true;
                            f_contexts = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Vec<crate::types::search::CompletionContext>,
                                >,
                            >>()?;
                        }
                        "fuzzy" => {
                            if seen_fuzzy {
                                return Err(serde::de::Error::duplicate_field("fuzzy"));
                            }
                            seen_fuzzy = true;
                            f_fuzzy =
                                map.next_value::<Option<crate::types::search::SuggestFuzziness>>()?;
                        }
                        "regex" => {
                            if seen_regex {
                                return Err(serde::de::Error::duplicate_field("regex"));
                            }
                            seen_regex = true;
                            f_regex = map.next_value::<Option<String>>()?;
                        }
                        "skip_duplicates" => {
                            if seen_skip_duplicates {
                                return Err(serde::de::Error::duplicate_field("skip_duplicates"));
                            }
                            seen_skip_duplicates = true;
                            f_skip_duplicates = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CompletionSuggester {
                    analyzer: f_analyzer,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    size: f_size,
                    contexts: f_contexts,
                    fuzzy: f_fuzzy,
                    regex: f_regex,
                    skip_duplicates: f_skip_duplicates,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CompletionSuggester {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.analyzer {
            map.serialize_entry("analyzer", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.size {
            map.serialize_entry("size", v)?;
        }
        if let Some(v) = &self.contexts {
            map.serialize_entry("contexts", v)?;
        }
        if let Some(v) = &self.fuzzy {
            map.serialize_entry("fuzzy", v)?;
        }
        if let Some(v) = &self.regex {
            map.serialize_entry("regex", v)?;
        }
        if let Some(v) = &self.skip_duplicates {
            map.serialize_entry("skip_duplicates", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Text or location that we want similar documents for or a lookup to a document's field for the text."]
#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Context {
    raw: Box<serde_json::value::RawValue>,
}
impl Context {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `category` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_category(&self) -> std::result::Result<String, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "category")
    }
    #[doc = "Parses the value as the `location` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_location(
        &self,
    ) -> std::result::Result<crate::types::common::GeoLocation, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "location")
    }
}

#[derive(Debug, Clone)]
pub struct DirectGenerator {
    pub field: String,
    pub max_edits: Option<i32>,
    pub max_inspections: Option<f64>,
    pub max_term_freq: Option<f64>,
    pub min_doc_freq: Option<f64>,
    pub min_word_length: Option<i32>,
    pub post_filter: Option<String>,
    pub pre_filter: Option<String>,
    pub prefix_length: Option<i32>,
    pub size: Option<i32>,
    pub suggest_mode: Option<crate::types::common::SuggestMode>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DirectGenerator {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DirectGenerator;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DirectGenerator object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_field: Option<String> = None;
                let mut f_max_edits: Option<i32> = None;
                let mut seen_max_edits = false;
                let mut f_max_inspections: Option<f64> = None;
                let mut seen_max_inspections = false;
                let mut f_max_term_freq: Option<f64> = None;
                let mut seen_max_term_freq = false;
                let mut f_min_doc_freq: Option<f64> = None;
                let mut seen_min_doc_freq = false;
                let mut f_min_word_length: Option<i32> = None;
                let mut seen_min_word_length = false;
                let mut f_post_filter: Option<String> = None;
                let mut seen_post_filter = false;
                let mut f_pre_filter: Option<String> = None;
                let mut seen_pre_filter = false;
                let mut f_prefix_length: Option<i32> = None;
                let mut seen_prefix_length = false;
                let mut f_size: Option<i32> = None;
                let mut seen_size = false;
                let mut f_suggest_mode: Option<crate::types::common::SuggestMode> = None;
                let mut seen_suggest_mode = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<String>()?);
                        }
                        "max_edits" => {
                            if seen_max_edits {
                                return Err(serde::de::Error::duplicate_field("max_edits"));
                            }
                            seen_max_edits = true;
                            f_max_edits = map.next_value::<Option<i32>>()?;
                        }
                        "max_inspections" => {
                            if seen_max_inspections {
                                return Err(serde::de::Error::duplicate_field("max_inspections"));
                            }
                            seen_max_inspections = true;
                            f_max_inspections = map.next_value::<Option<f64>>()?;
                        }
                        "max_term_freq" => {
                            if seen_max_term_freq {
                                return Err(serde::de::Error::duplicate_field("max_term_freq"));
                            }
                            seen_max_term_freq = true;
                            f_max_term_freq = map.next_value::<Option<f64>>()?;
                        }
                        "min_doc_freq" => {
                            if seen_min_doc_freq {
                                return Err(serde::de::Error::duplicate_field("min_doc_freq"));
                            }
                            seen_min_doc_freq = true;
                            f_min_doc_freq = map.next_value::<Option<f64>>()?;
                        }
                        "min_word_length" => {
                            if seen_min_word_length {
                                return Err(serde::de::Error::duplicate_field("min_word_length"));
                            }
                            seen_min_word_length = true;
                            f_min_word_length = map.next_value::<Option<i32>>()?;
                        }
                        "post_filter" => {
                            if seen_post_filter {
                                return Err(serde::de::Error::duplicate_field("post_filter"));
                            }
                            seen_post_filter = true;
                            f_post_filter = map.next_value::<Option<String>>()?;
                        }
                        "pre_filter" => {
                            if seen_pre_filter {
                                return Err(serde::de::Error::duplicate_field("pre_filter"));
                            }
                            seen_pre_filter = true;
                            f_pre_filter = map.next_value::<Option<String>>()?;
                        }
                        "prefix_length" => {
                            if seen_prefix_length {
                                return Err(serde::de::Error::duplicate_field("prefix_length"));
                            }
                            seen_prefix_length = true;
                            f_prefix_length = map.next_value::<Option<i32>>()?;
                        }
                        "size" => {
                            if seen_size {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            seen_size = true;
                            f_size = map.next_value::<Option<i32>>()?;
                        }
                        "suggest_mode" => {
                            if seen_suggest_mode {
                                return Err(serde::de::Error::duplicate_field("suggest_mode"));
                            }
                            seen_suggest_mode = true;
                            f_suggest_mode =
                                map.next_value::<Option<crate::types::common::SuggestMode>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DirectGenerator {
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    max_edits: f_max_edits,
                    max_inspections: f_max_inspections,
                    max_term_freq: f_max_term_freq,
                    min_doc_freq: f_min_doc_freq,
                    min_word_length: f_min_word_length,
                    post_filter: f_post_filter,
                    pre_filter: f_pre_filter,
                    prefix_length: f_prefix_length,
                    size: f_size,
                    suggest_mode: f_suggest_mode,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DirectGenerator {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.max_edits {
            map.serialize_entry("max_edits", v)?;
        }
        if let Some(v) = &self.max_inspections {
            map.serialize_entry("max_inspections", v)?;
        }
        if let Some(v) = &self.max_term_freq {
            map.serialize_entry("max_term_freq", v)?;
        }
        if let Some(v) = &self.min_doc_freq {
            map.serialize_entry("min_doc_freq", v)?;
        }
        if let Some(v) = &self.min_word_length {
            map.serialize_entry("min_word_length", v)?;
        }
        if let Some(v) = &self.post_filter {
            map.serialize_entry("post_filter", v)?;
        }
        if let Some(v) = &self.pre_filter {
            map.serialize_entry("pre_filter", v)?;
        }
        if let Some(v) = &self.prefix_length {
            map.serialize_entry("prefix_length", v)?;
        }
        if let Some(v) = &self.size {
            map.serialize_entry("size", v)?;
        }
        if let Some(v) = &self.suggest_mode {
            map.serialize_entry("suggest_mode", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct FetchProfile {
    pub breakdown: crate::types::search::FetchProfileBreakdown,
    pub children: Option<Vec<crate::types::search::FetchProfile>>,
    pub debug: Option<crate::types::search::FetchProfileDebug>,
    pub description: String,
    pub time_in_nanos: crate::types::common::DurationValueUnitNanos,
    pub r#type: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FetchProfile {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FetchProfile;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FetchProfile object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_breakdown: Option<crate::types::search::FetchProfileBreakdown> = None;
                let mut f_children: Option<Vec<crate::types::search::FetchProfile>> = None;
                let mut seen_children = false;
                let mut f_debug: Option<crate::types::search::FetchProfileDebug> = None;
                let mut seen_debug = false;
                let mut f_description: Option<String> = None;
                let mut f_time_in_nanos: Option<crate::types::common::DurationValueUnitNanos> =
                    None;
                let mut f_type: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "breakdown" => {
                            if f_breakdown.is_some() {
                                return Err(serde::de::Error::duplicate_field("breakdown"));
                            }
                            f_breakdown = Some(
                                map.next_value::<crate::types::search::FetchProfileBreakdown>()?,
                            );
                        }
                        "children" => {
                            if seen_children {
                                return Err(serde::de::Error::duplicate_field("children"));
                            }
                            seen_children = true;
                            f_children = map
                                .next_value::<Option<Vec<crate::types::search::FetchProfile>>>()?;
                        }
                        "debug" => {
                            if seen_debug {
                                return Err(serde::de::Error::duplicate_field("debug"));
                            }
                            seen_debug = true;
                            f_debug = map
                                .next_value::<Option<crate::types::search::FetchProfileDebug>>()?;
                        }
                        "description" => {
                            if f_description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            f_description = Some(map.next_value::<String>()?);
                        }
                        "time_in_nanos" => {
                            if f_time_in_nanos.is_some() {
                                return Err(serde::de::Error::duplicate_field("time_in_nanos"));
                            }
                            f_time_in_nanos = Some(
                                map.next_value::<crate::types::common::DurationValueUnitNanos>()?,
                            );
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FetchProfile {
                    breakdown: f_breakdown
                        .ok_or_else(|| serde::de::Error::missing_field("breakdown"))?,
                    children: f_children,
                    debug: f_debug,
                    description: f_description
                        .ok_or_else(|| serde::de::Error::missing_field("description"))?,
                    time_in_nanos: f_time_in_nanos
                        .ok_or_else(|| serde::de::Error::missing_field("time_in_nanos"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FetchProfile {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("breakdown", &self.breakdown)?;
        if let Some(v) = &self.children {
            map.serialize_entry("children", v)?;
        }
        if let Some(v) = &self.debug {
            map.serialize_entry("debug", v)?;
        }
        map.serialize_entry("description", &self.description)?;
        map.serialize_entry("time_in_nanos", &self.time_in_nanos)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct FetchProfileBreakdown {
    pub load_source: Option<i32>,
    pub load_source_count: Option<i32>,
    pub load_stored_fields: Option<i32>,
    pub load_stored_fields_count: Option<i32>,
    pub next_reader: Option<i32>,
    pub next_reader_count: Option<i32>,
    pub process: Option<i32>,
    pub process_count: Option<i32>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FetchProfileBreakdown {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FetchProfileBreakdown;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FetchProfileBreakdown object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_load_source: Option<i32> = None;
                let mut seen_load_source = false;
                let mut f_load_source_count: Option<i32> = None;
                let mut seen_load_source_count = false;
                let mut f_load_stored_fields: Option<i32> = None;
                let mut seen_load_stored_fields = false;
                let mut f_load_stored_fields_count: Option<i32> = None;
                let mut seen_load_stored_fields_count = false;
                let mut f_next_reader: Option<i32> = None;
                let mut seen_next_reader = false;
                let mut f_next_reader_count: Option<i32> = None;
                let mut seen_next_reader_count = false;
                let mut f_process: Option<i32> = None;
                let mut seen_process = false;
                let mut f_process_count: Option<i32> = None;
                let mut seen_process_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "load_source" => {
                            if seen_load_source {
                                return Err(serde::de::Error::duplicate_field("load_source"));
                            }
                            seen_load_source = true;
                            f_load_source = map.next_value::<Option<i32>>()?;
                        }
                        "load_source_count" => {
                            if seen_load_source_count {
                                return Err(serde::de::Error::duplicate_field("load_source_count"));
                            }
                            seen_load_source_count = true;
                            f_load_source_count = map.next_value::<Option<i32>>()?;
                        }
                        "load_stored_fields" => {
                            if seen_load_stored_fields {
                                return Err(serde::de::Error::duplicate_field(
                                    "load_stored_fields",
                                ));
                            }
                            seen_load_stored_fields = true;
                            f_load_stored_fields = map.next_value::<Option<i32>>()?;
                        }
                        "load_stored_fields_count" => {
                            if seen_load_stored_fields_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "load_stored_fields_count",
                                ));
                            }
                            seen_load_stored_fields_count = true;
                            f_load_stored_fields_count = map.next_value::<Option<i32>>()?;
                        }
                        "next_reader" => {
                            if seen_next_reader {
                                return Err(serde::de::Error::duplicate_field("next_reader"));
                            }
                            seen_next_reader = true;
                            f_next_reader = map.next_value::<Option<i32>>()?;
                        }
                        "next_reader_count" => {
                            if seen_next_reader_count {
                                return Err(serde::de::Error::duplicate_field("next_reader_count"));
                            }
                            seen_next_reader_count = true;
                            f_next_reader_count = map.next_value::<Option<i32>>()?;
                        }
                        "process" => {
                            if seen_process {
                                return Err(serde::de::Error::duplicate_field("process"));
                            }
                            seen_process = true;
                            f_process = map.next_value::<Option<i32>>()?;
                        }
                        "process_count" => {
                            if seen_process_count {
                                return Err(serde::de::Error::duplicate_field("process_count"));
                            }
                            seen_process_count = true;
                            f_process_count = map.next_value::<Option<i32>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FetchProfileBreakdown {
                    load_source: f_load_source,
                    load_source_count: f_load_source_count,
                    load_stored_fields: f_load_stored_fields,
                    load_stored_fields_count: f_load_stored_fields_count,
                    next_reader: f_next_reader,
                    next_reader_count: f_next_reader_count,
                    process: f_process,
                    process_count: f_process_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FetchProfileBreakdown {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.load_source {
            map.serialize_entry("load_source", v)?;
        }
        if let Some(v) = &self.load_source_count {
            map.serialize_entry("load_source_count", v)?;
        }
        if let Some(v) = &self.load_stored_fields {
            map.serialize_entry("load_stored_fields", v)?;
        }
        if let Some(v) = &self.load_stored_fields_count {
            map.serialize_entry("load_stored_fields_count", v)?;
        }
        if let Some(v) = &self.next_reader {
            map.serialize_entry("next_reader", v)?;
        }
        if let Some(v) = &self.next_reader_count {
            map.serialize_entry("next_reader_count", v)?;
        }
        if let Some(v) = &self.process {
            map.serialize_entry("process", v)?;
        }
        if let Some(v) = &self.process_count {
            map.serialize_entry("process_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct FetchProfileDebug {
    pub fast_path: Option<i32>,
    pub stored_fields: Option<Vec<String>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FetchProfileDebug {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FetchProfileDebug;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FetchProfileDebug object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_fast_path: Option<i32> = None;
                let mut seen_fast_path = false;
                let mut f_stored_fields: Option<Vec<String>> = None;
                let mut seen_stored_fields = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "fast_path" => {
                            if seen_fast_path {
                                return Err(serde::de::Error::duplicate_field("fast_path"));
                            }
                            seen_fast_path = true;
                            f_fast_path = map.next_value::<Option<i32>>()?;
                        }
                        "stored_fields" => {
                            if seen_stored_fields {
                                return Err(serde::de::Error::duplicate_field("stored_fields"));
                            }
                            seen_stored_fields = true;
                            f_stored_fields = map.next_value::<Option<Vec<String>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FetchProfileDebug {
                    fast_path: f_fast_path,
                    stored_fields: f_stored_fields,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FetchProfileDebug {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.fast_path {
            map.serialize_entry("fast_path", v)?;
        }
        if let Some(v) = &self.stored_fields {
            map.serialize_entry("stored_fields", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct FieldCollapse {
    pub field: crate::types::common::Field,
    #[doc = "The number of inner hits and their sort order."]
    pub inner_hits: Option<Box<crate::types::search::FieldCollapseInnerHits>>,
    #[doc = "The number of concurrent requests that are allowed to be retrieved by the `inner_hits` parameter per group."]
    pub max_concurrent_group_searches: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FieldCollapse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FieldCollapse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FieldCollapse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_inner_hits: Option<Box<crate::types::search::FieldCollapseInnerHits>> =
                    None;
                let mut seen_inner_hits = false;
                let mut f_max_concurrent_group_searches: Option<i64> = None;
                let mut seen_max_concurrent_group_searches = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "inner_hits" => {
                            if seen_inner_hits {
                                return Err(serde::de::Error::duplicate_field("inner_hits"));
                            }
                            seen_inner_hits = true;
                            f_inner_hits = map.next_value::<Option<Box<crate::types::search::FieldCollapseInnerHits>>>()?;
                        }
                        "max_concurrent_group_searches" => {
                            if seen_max_concurrent_group_searches {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_concurrent_group_searches",
                                ));
                            }
                            seen_max_concurrent_group_searches = true;
                            f_max_concurrent_group_searches = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FieldCollapse {
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    inner_hits: f_inner_hits,
                    max_concurrent_group_searches: f_max_concurrent_group_searches,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FieldCollapse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.inner_hits {
            map.serialize_entry("inner_hits", v)?;
        }
        if let Some(v) = &self.max_concurrent_group_searches {
            map.serialize_entry("max_concurrent_group_searches", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The number of inner hits and their sort order."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum FieldCollapseInnerHits {
    InnerHits(Box<crate::types::search::InnerHits>),
    Array(Vec<crate::types::search::InnerHits>),
}
impl<'de> serde::Deserialize<'de> for FieldCollapseInnerHits {
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
            Some(b'{') => serde_json::from_str(text).map(FieldCollapseInnerHits::InnerHits),
            Some(b'[') => serde_json::from_str(text).map(FieldCollapseInnerHits::Array),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a FieldCollapseInnerHits value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct FieldSuggester {
    pub prefix: Option<String>,
    pub regex: Option<String>,
    pub text: Option<String>,
    pub completion: Option<crate::types::search::CompletionSuggester>,
    pub phrase: Option<crate::types::search::PhraseSuggester>,
    pub term: Option<crate::types::search::TermSuggester>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FieldSuggester {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FieldSuggester;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FieldSuggester object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_prefix: Option<String> = None;
                let mut seen_prefix = false;
                let mut f_regex: Option<String> = None;
                let mut seen_regex = false;
                let mut f_text: Option<String> = None;
                let mut seen_text = false;
                let mut f_completion: Option<crate::types::search::CompletionSuggester> = None;
                let mut seen_completion = false;
                let mut f_phrase: Option<crate::types::search::PhraseSuggester> = None;
                let mut seen_phrase = false;
                let mut f_term: Option<crate::types::search::TermSuggester> = None;
                let mut seen_term = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "prefix" => {
                            if seen_prefix {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            seen_prefix = true;
                            f_prefix = map.next_value::<Option<String>>()?;
                        }
                        "regex" => {
                            if seen_regex {
                                return Err(serde::de::Error::duplicate_field("regex"));
                            }
                            seen_regex = true;
                            f_regex = map.next_value::<Option<String>>()?;
                        }
                        "text" => {
                            if seen_text {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            seen_text = true;
                            f_text = map.next_value::<Option<String>>()?;
                        }
                        "completion" => {
                            if seen_completion {
                                return Err(serde::de::Error::duplicate_field("completion"));
                            }
                            seen_completion = true;
                            f_completion = map
                                .next_value::<Option<crate::types::search::CompletionSuggester>>(
                                )?;
                        }
                        "phrase" => {
                            if seen_phrase {
                                return Err(serde::de::Error::duplicate_field("phrase"));
                            }
                            seen_phrase = true;
                            f_phrase =
                                map.next_value::<Option<crate::types::search::PhraseSuggester>>()?;
                        }
                        "term" => {
                            if seen_term {
                                return Err(serde::de::Error::duplicate_field("term"));
                            }
                            seen_term = true;
                            f_term =
                                map.next_value::<Option<crate::types::search::TermSuggester>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FieldSuggester {
                    prefix: f_prefix,
                    regex: f_regex,
                    text: f_text,
                    completion: f_completion,
                    phrase: f_phrase,
                    term: f_term,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FieldSuggester {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.prefix {
            map.serialize_entry("prefix", v)?;
        }
        if let Some(v) = &self.regex {
            map.serialize_entry("regex", v)?;
        }
        if let Some(v) = &self.text {
            map.serialize_entry("text", v)?;
        }
        if let Some(v) = &self.completion {
            map.serialize_entry("completion", v)?;
        }
        if let Some(v) = &self.phrase {
            map.serialize_entry("phrase", v)?;
        }
        if let Some(v) = &self.term {
            map.serialize_entry("term", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Highlight {
    #[doc = "A string that contains each boundary character."]
    pub boundary_chars: Option<String>,
    #[doc = "How far to scan for boundary characters."]
    pub boundary_max_scan: Option<i32>,
    pub boundary_scanner: Option<crate::types::search::BoundaryScanner>,
    #[doc = "Controls which locale is used to search for sentence and word boundaries. This parameter takes the form of a language tag, for example, `\"en-US\"`, `\"fr-FR\"`, or `\"ja-JP\"`."]
    pub boundary_scanner_locale: Option<String>,
    pub force_source: Option<bool>,
    pub fragment_offset: Option<i32>,
    #[doc = "The size of the highlighted fragment in characters."]
    pub fragment_size: Option<i32>,
    pub fragmenter: Option<crate::types::search::HighlighterFragmenter>,
    pub highlight_filter: Option<bool>,
    pub highlight_query: Option<Box<crate::types::common_query_dsl::QueryContainer>>,
    #[doc = "If set to a non-negative value, highlighting stops at this defined maximum limit. The rest of the text is not processed or highlighted, and no error is returned. The `max_analyzer_offset` query setting does not override the `index.highlight.max_analyzed_offset` setting, which takes precedence when it is set to a lower value than the query setting."]
    pub max_analyzer_offset: Option<i32>,
    pub max_fragment_length: Option<i32>,
    #[doc = "The amount of text you want to return from the beginning of the field if there are no matching fragments to highlight."]
    pub no_match_size: Option<i32>,
    #[doc = "The maximum number of fragments to return. When the number of fragments is set to `0`, no fragments are returned. Instead, the entirety of a field's contents are highlighted and returned. This is useful when you need to highlight short texts, such as a title or address, in which fragmentation is not required. If `number_of_fragments` is set to `0`, the `fragment_size` is ignored."]
    pub number_of_fragments: Option<i32>,
    pub options: Option<
        std::collections::BTreeMap<
            String,
            std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
        >,
    >,
    pub order: Option<crate::types::search::HighlighterOrder>,
    #[doc = "Controls the number of matching phrases in a document that are considered. This prevents the `fvh` highlighter from analyzing too many phrases and consuming too much memory. When using `matched_fields`, phrase-limited phrases per matched field are considered. Raising the limit increases the query time and consumes more memory. This setting is only supported by the `fvh` highlighter."]
    pub phrase_limit: Option<i32>,
    #[doc = "When used in conjunction with `pre_tags`, defines the HTML tags to use for the highlighted text. By default, highlighted text is wrapped in `<em>` and `</em>` tags."]
    pub post_tags: Option<Vec<String>>,
    #[doc = "When used in conjunction with `post_tags`, defines the HTML tags to use for the highlighted text. By default, highlighted text is wrapped in `<em>` and `</em>` tags."]
    pub pre_tags: Option<Vec<String>>,
    #[doc = "By default, only fields that contains a query match are highlighted. Set to `false` to highlight all fields."]
    pub require_field_match: Option<bool>,
    pub tags_schema: Option<crate::types::search::HighlighterTagsSchema>,
    pub r#type: Option<crate::types::search::HighlighterType>,
    pub encoder: Option<crate::types::search::HighlighterEncoder>,
    pub fields: crate::types::search::HighlightFields,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Highlight {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Highlight;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Highlight object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_boundary_chars: Option<String> = None;
                let mut seen_boundary_chars = false;
                let mut f_boundary_max_scan: Option<i32> = None;
                let mut seen_boundary_max_scan = false;
                let mut f_boundary_scanner: Option<crate::types::search::BoundaryScanner> = None;
                let mut seen_boundary_scanner = false;
                let mut f_boundary_scanner_locale: Option<String> = None;
                let mut seen_boundary_scanner_locale = false;
                let mut f_force_source: Option<bool> = None;
                let mut seen_force_source = false;
                let mut f_fragment_offset: Option<i32> = None;
                let mut seen_fragment_offset = false;
                let mut f_fragment_size: Option<i32> = None;
                let mut seen_fragment_size = false;
                let mut f_fragmenter: Option<crate::types::search::HighlighterFragmenter> = None;
                let mut seen_fragmenter = false;
                let mut f_highlight_filter: Option<bool> = None;
                let mut seen_highlight_filter = false;
                let mut f_highlight_query: Option<
                    Box<crate::types::common_query_dsl::QueryContainer>,
                > = None;
                let mut seen_highlight_query = false;
                let mut f_max_analyzer_offset: Option<i32> = None;
                let mut seen_max_analyzer_offset = false;
                let mut f_max_fragment_length: Option<i32> = None;
                let mut seen_max_fragment_length = false;
                let mut f_no_match_size: Option<i32> = None;
                let mut seen_no_match_size = false;
                let mut f_number_of_fragments: Option<i32> = None;
                let mut seen_number_of_fragments = false;
                let mut f_options: Option<
                    std::collections::BTreeMap<
                        String,
                        std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                    >,
                > = None;
                let mut seen_options = false;
                let mut f_order: Option<crate::types::search::HighlighterOrder> = None;
                let mut seen_order = false;
                let mut f_phrase_limit: Option<i32> = None;
                let mut seen_phrase_limit = false;
                let mut f_post_tags: Option<Vec<String>> = None;
                let mut seen_post_tags = false;
                let mut f_pre_tags: Option<Vec<String>> = None;
                let mut seen_pre_tags = false;
                let mut f_require_field_match: Option<bool> = None;
                let mut seen_require_field_match = false;
                let mut f_tags_schema: Option<crate::types::search::HighlighterTagsSchema> = None;
                let mut seen_tags_schema = false;
                let mut f_type: Option<crate::types::search::HighlighterType> = None;
                let mut seen_type = false;
                let mut f_encoder: Option<crate::types::search::HighlighterEncoder> = None;
                let mut seen_encoder = false;
                let mut f_fields: Option<crate::types::search::HighlightFields> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "boundary_chars" => {
                            if seen_boundary_chars {
                                return Err(serde::de::Error::duplicate_field("boundary_chars"));
                            }
                            seen_boundary_chars = true;
                            f_boundary_chars = map.next_value::<Option<String>>()?;
                        }
                        "boundary_max_scan" => {
                            if seen_boundary_max_scan {
                                return Err(serde::de::Error::duplicate_field("boundary_max_scan"));
                            }
                            seen_boundary_max_scan = true;
                            f_boundary_max_scan = map.next_value::<Option<i32>>()?;
                        }
                        "boundary_scanner" => {
                            if seen_boundary_scanner {
                                return Err(serde::de::Error::duplicate_field("boundary_scanner"));
                            }
                            seen_boundary_scanner = true;
                            f_boundary_scanner =
                                map.next_value::<Option<crate::types::search::BoundaryScanner>>()?;
                        }
                        "boundary_scanner_locale" => {
                            if seen_boundary_scanner_locale {
                                return Err(serde::de::Error::duplicate_field(
                                    "boundary_scanner_locale",
                                ));
                            }
                            seen_boundary_scanner_locale = true;
                            f_boundary_scanner_locale = map.next_value::<Option<String>>()?;
                        }
                        "force_source" => {
                            if seen_force_source {
                                return Err(serde::de::Error::duplicate_field("force_source"));
                            }
                            seen_force_source = true;
                            f_force_source = map.next_value::<Option<bool>>()?;
                        }
                        "fragment_offset" => {
                            if seen_fragment_offset {
                                return Err(serde::de::Error::duplicate_field("fragment_offset"));
                            }
                            seen_fragment_offset = true;
                            f_fragment_offset = map.next_value::<Option<i32>>()?;
                        }
                        "fragment_size" => {
                            if seen_fragment_size {
                                return Err(serde::de::Error::duplicate_field("fragment_size"));
                            }
                            seen_fragment_size = true;
                            f_fragment_size = map.next_value::<Option<i32>>()?;
                        }
                        "fragmenter" => {
                            if seen_fragmenter {
                                return Err(serde::de::Error::duplicate_field("fragmenter"));
                            }
                            seen_fragmenter = true;
                            f_fragmenter = map
                                .next_value::<Option<crate::types::search::HighlighterFragmenter>>(
                                )?;
                        }
                        "highlight_filter" => {
                            if seen_highlight_filter {
                                return Err(serde::de::Error::duplicate_field("highlight_filter"));
                            }
                            seen_highlight_filter = true;
                            f_highlight_filter = map.next_value::<Option<bool>>()?;
                        }
                        "highlight_query" => {
                            if seen_highlight_query {
                                return Err(serde::de::Error::duplicate_field("highlight_query"));
                            }
                            seen_highlight_query = true;
                            f_highlight_query = map.next_value::<Option<Box<crate::types::common_query_dsl::QueryContainer>>>()?;
                        }
                        "max_analyzer_offset" => {
                            if seen_max_analyzer_offset {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_analyzer_offset",
                                ));
                            }
                            seen_max_analyzer_offset = true;
                            f_max_analyzer_offset = map.next_value::<Option<i32>>()?;
                        }
                        "max_fragment_length" => {
                            if seen_max_fragment_length {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_fragment_length",
                                ));
                            }
                            seen_max_fragment_length = true;
                            f_max_fragment_length = map.next_value::<Option<i32>>()?;
                        }
                        "no_match_size" => {
                            if seen_no_match_size {
                                return Err(serde::de::Error::duplicate_field("no_match_size"));
                            }
                            seen_no_match_size = true;
                            f_no_match_size = map.next_value::<Option<i32>>()?;
                        }
                        "number_of_fragments" => {
                            if seen_number_of_fragments {
                                return Err(serde::de::Error::duplicate_field(
                                    "number_of_fragments",
                                ));
                            }
                            seen_number_of_fragments = true;
                            f_number_of_fragments = map.next_value::<Option<i32>>()?;
                        }
                        "options" => {
                            if seen_options {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            seen_options = true;
                            f_options = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    std::collections::BTreeMap<
                                        String,
                                        Box<serde_json::value::RawValue>,
                                    >,
                                >,
                            >>()?;
                        }
                        "order" => {
                            if seen_order {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            seen_order = true;
                            f_order =
                                map.next_value::<Option<crate::types::search::HighlighterOrder>>()?;
                        }
                        "phrase_limit" => {
                            if seen_phrase_limit {
                                return Err(serde::de::Error::duplicate_field("phrase_limit"));
                            }
                            seen_phrase_limit = true;
                            f_phrase_limit = map.next_value::<Option<i32>>()?;
                        }
                        "post_tags" => {
                            if seen_post_tags {
                                return Err(serde::de::Error::duplicate_field("post_tags"));
                            }
                            seen_post_tags = true;
                            f_post_tags = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "pre_tags" => {
                            if seen_pre_tags {
                                return Err(serde::de::Error::duplicate_field("pre_tags"));
                            }
                            seen_pre_tags = true;
                            f_pre_tags = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "require_field_match" => {
                            if seen_require_field_match {
                                return Err(serde::de::Error::duplicate_field(
                                    "require_field_match",
                                ));
                            }
                            seen_require_field_match = true;
                            f_require_field_match = map.next_value::<Option<bool>>()?;
                        }
                        "tags_schema" => {
                            if seen_tags_schema {
                                return Err(serde::de::Error::duplicate_field("tags_schema"));
                            }
                            seen_tags_schema = true;
                            f_tags_schema = map
                                .next_value::<Option<crate::types::search::HighlighterTagsSchema>>(
                                )?;
                        }
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type =
                                map.next_value::<Option<crate::types::search::HighlighterType>>()?;
                        }
                        "encoder" => {
                            if seen_encoder {
                                return Err(serde::de::Error::duplicate_field("encoder"));
                            }
                            seen_encoder = true;
                            f_encoder = map
                                .next_value::<Option<crate::types::search::HighlighterEncoder>>()?;
                        }
                        "fields" => {
                            if f_fields.is_some() {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            f_fields =
                                Some(map.next_value::<crate::types::search::HighlightFields>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Highlight {
                    boundary_chars: f_boundary_chars,
                    boundary_max_scan: f_boundary_max_scan,
                    boundary_scanner: f_boundary_scanner,
                    boundary_scanner_locale: f_boundary_scanner_locale,
                    force_source: f_force_source,
                    fragment_offset: f_fragment_offset,
                    fragment_size: f_fragment_size,
                    fragmenter: f_fragmenter,
                    highlight_filter: f_highlight_filter,
                    highlight_query: f_highlight_query,
                    max_analyzer_offset: f_max_analyzer_offset,
                    max_fragment_length: f_max_fragment_length,
                    no_match_size: f_no_match_size,
                    number_of_fragments: f_number_of_fragments,
                    options: f_options,
                    order: f_order,
                    phrase_limit: f_phrase_limit,
                    post_tags: f_post_tags,
                    pre_tags: f_pre_tags,
                    require_field_match: f_require_field_match,
                    tags_schema: f_tags_schema,
                    r#type: f_type,
                    encoder: f_encoder,
                    fields: f_fields.ok_or_else(|| serde::de::Error::missing_field("fields"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Highlight {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.boundary_chars {
            map.serialize_entry("boundary_chars", v)?;
        }
        if let Some(v) = &self.boundary_max_scan {
            map.serialize_entry("boundary_max_scan", v)?;
        }
        if let Some(v) = &self.boundary_scanner {
            map.serialize_entry("boundary_scanner", v)?;
        }
        if let Some(v) = &self.boundary_scanner_locale {
            map.serialize_entry("boundary_scanner_locale", v)?;
        }
        if let Some(v) = &self.force_source {
            map.serialize_entry("force_source", v)?;
        }
        if let Some(v) = &self.fragment_offset {
            map.serialize_entry("fragment_offset", v)?;
        }
        if let Some(v) = &self.fragment_size {
            map.serialize_entry("fragment_size", v)?;
        }
        if let Some(v) = &self.fragmenter {
            map.serialize_entry("fragmenter", v)?;
        }
        if let Some(v) = &self.highlight_filter {
            map.serialize_entry("highlight_filter", v)?;
        }
        if let Some(v) = &self.highlight_query {
            map.serialize_entry("highlight_query", v)?;
        }
        if let Some(v) = &self.max_analyzer_offset {
            map.serialize_entry("max_analyzer_offset", v)?;
        }
        if let Some(v) = &self.max_fragment_length {
            map.serialize_entry("max_fragment_length", v)?;
        }
        if let Some(v) = &self.no_match_size {
            map.serialize_entry("no_match_size", v)?;
        }
        if let Some(v) = &self.number_of_fragments {
            map.serialize_entry("number_of_fragments", v)?;
        }
        if let Some(v) = &self.options {
            map.serialize_entry("options", v)?;
        }
        if let Some(v) = &self.order {
            map.serialize_entry("order", v)?;
        }
        if let Some(v) = &self.phrase_limit {
            map.serialize_entry("phrase_limit", v)?;
        }
        if let Some(v) = &self.post_tags {
            map.serialize_entry("post_tags", v)?;
        }
        if let Some(v) = &self.pre_tags {
            map.serialize_entry("pre_tags", v)?;
        }
        if let Some(v) = &self.require_field_match {
            map.serialize_entry("require_field_match", v)?;
        }
        if let Some(v) = &self.tags_schema {
            map.serialize_entry("tags_schema", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("type", v)?;
        }
        if let Some(v) = &self.encoder {
            map.serialize_entry("encoder", v)?;
        }
        map.serialize_entry("fields", &self.fields)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct HighlightField {
    #[doc = "A string that contains each boundary character."]
    pub boundary_chars: Option<String>,
    #[doc = "How far to scan for boundary characters."]
    pub boundary_max_scan: Option<i32>,
    pub boundary_scanner: Option<crate::types::search::BoundaryScanner>,
    #[doc = "Controls which locale is used to search for sentence and word boundaries. This parameter takes the form of a language tag, for example, `\"en-US\"`, `\"fr-FR\"`, or `\"ja-JP\"`."]
    pub boundary_scanner_locale: Option<String>,
    pub force_source: Option<bool>,
    pub fragment_offset: Option<i32>,
    #[doc = "The size of the highlighted fragment in characters."]
    pub fragment_size: Option<i32>,
    pub fragmenter: Option<crate::types::search::HighlighterFragmenter>,
    pub highlight_filter: Option<bool>,
    pub highlight_query: Option<crate::types::common_query_dsl::QueryContainer>,
    #[doc = "If set to a non-negative value, highlighting stops at this defined maximum limit. The rest of the text is not processed or highlighted, and no error is returned. The `max_analyzer_offset` query setting does not override the `index.highlight.max_analyzed_offset` setting, which takes precedence when it is set to a lower value than the query setting."]
    pub max_analyzer_offset: Option<i32>,
    pub max_fragment_length: Option<i32>,
    #[doc = "The amount of text you want to return from the beginning of the field if there are no matching fragments to highlight."]
    pub no_match_size: Option<i32>,
    #[doc = "The maximum number of fragments to return. When the number of fragments is set to `0`, no fragments are returned. Instead, the entirety of a field's contents are highlighted and returned. This is useful when you need to highlight short texts, such as a title or address, in which fragmentation is not required. If `number_of_fragments` is set to `0`, the `fragment_size` is ignored."]
    pub number_of_fragments: Option<i32>,
    pub options: Option<
        std::collections::BTreeMap<
            String,
            std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
        >,
    >,
    pub order: Option<crate::types::search::HighlighterOrder>,
    #[doc = "Controls the number of matching phrases in a document that are considered. This prevents the `fvh` highlighter from analyzing too many phrases and consuming too much memory. When using `matched_fields`, phrase-limited phrases per matched field are considered. Raising the limit increases the query time and consumes more memory. This setting is only supported by the `fvh` highlighter."]
    pub phrase_limit: Option<i32>,
    #[doc = "When used in conjunction with `pre_tags`, defines the HTML tags to use for the highlighted text. By default, highlighted text is wrapped in `<em>` and `</em>` tags."]
    pub post_tags: Option<Vec<String>>,
    #[doc = "When used in conjunction with `post_tags`, defines the HTML tags to use for the highlighted text. By default, highlighted text is wrapped in `<em>` and `</em>` tags."]
    pub pre_tags: Option<Vec<String>>,
    #[doc = "By default, only fields that contains a query match are highlighted. Set to `false` to highlight all fields."]
    pub require_field_match: Option<bool>,
    pub tags_schema: Option<crate::types::search::HighlighterTagsSchema>,
    pub r#type: Option<crate::types::search::HighlighterType>,
    pub matched_fields: Option<crate::types::common::Fields>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HighlightField {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HighlightField;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HighlightField object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_boundary_chars: Option<String> = None;
                let mut seen_boundary_chars = false;
                let mut f_boundary_max_scan: Option<i32> = None;
                let mut seen_boundary_max_scan = false;
                let mut f_boundary_scanner: Option<crate::types::search::BoundaryScanner> = None;
                let mut seen_boundary_scanner = false;
                let mut f_boundary_scanner_locale: Option<String> = None;
                let mut seen_boundary_scanner_locale = false;
                let mut f_force_source: Option<bool> = None;
                let mut seen_force_source = false;
                let mut f_fragment_offset: Option<i32> = None;
                let mut seen_fragment_offset = false;
                let mut f_fragment_size: Option<i32> = None;
                let mut seen_fragment_size = false;
                let mut f_fragmenter: Option<crate::types::search::HighlighterFragmenter> = None;
                let mut seen_fragmenter = false;
                let mut f_highlight_filter: Option<bool> = None;
                let mut seen_highlight_filter = false;
                let mut f_highlight_query: Option<crate::types::common_query_dsl::QueryContainer> =
                    None;
                let mut seen_highlight_query = false;
                let mut f_max_analyzer_offset: Option<i32> = None;
                let mut seen_max_analyzer_offset = false;
                let mut f_max_fragment_length: Option<i32> = None;
                let mut seen_max_fragment_length = false;
                let mut f_no_match_size: Option<i32> = None;
                let mut seen_no_match_size = false;
                let mut f_number_of_fragments: Option<i32> = None;
                let mut seen_number_of_fragments = false;
                let mut f_options: Option<
                    std::collections::BTreeMap<
                        String,
                        std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                    >,
                > = None;
                let mut seen_options = false;
                let mut f_order: Option<crate::types::search::HighlighterOrder> = None;
                let mut seen_order = false;
                let mut f_phrase_limit: Option<i32> = None;
                let mut seen_phrase_limit = false;
                let mut f_post_tags: Option<Vec<String>> = None;
                let mut seen_post_tags = false;
                let mut f_pre_tags: Option<Vec<String>> = None;
                let mut seen_pre_tags = false;
                let mut f_require_field_match: Option<bool> = None;
                let mut seen_require_field_match = false;
                let mut f_tags_schema: Option<crate::types::search::HighlighterTagsSchema> = None;
                let mut seen_tags_schema = false;
                let mut f_type: Option<crate::types::search::HighlighterType> = None;
                let mut seen_type = false;
                let mut f_matched_fields: Option<crate::types::common::Fields> = None;
                let mut seen_matched_fields = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "boundary_chars" => {
                            if seen_boundary_chars {
                                return Err(serde::de::Error::duplicate_field("boundary_chars"));
                            }
                            seen_boundary_chars = true;
                            f_boundary_chars = map.next_value::<Option<String>>()?;
                        }
                        "boundary_max_scan" => {
                            if seen_boundary_max_scan {
                                return Err(serde::de::Error::duplicate_field("boundary_max_scan"));
                            }
                            seen_boundary_max_scan = true;
                            f_boundary_max_scan = map.next_value::<Option<i32>>()?;
                        }
                        "boundary_scanner" => {
                            if seen_boundary_scanner {
                                return Err(serde::de::Error::duplicate_field("boundary_scanner"));
                            }
                            seen_boundary_scanner = true;
                            f_boundary_scanner =
                                map.next_value::<Option<crate::types::search::BoundaryScanner>>()?;
                        }
                        "boundary_scanner_locale" => {
                            if seen_boundary_scanner_locale {
                                return Err(serde::de::Error::duplicate_field(
                                    "boundary_scanner_locale",
                                ));
                            }
                            seen_boundary_scanner_locale = true;
                            f_boundary_scanner_locale = map.next_value::<Option<String>>()?;
                        }
                        "force_source" => {
                            if seen_force_source {
                                return Err(serde::de::Error::duplicate_field("force_source"));
                            }
                            seen_force_source = true;
                            f_force_source = map.next_value::<Option<bool>>()?;
                        }
                        "fragment_offset" => {
                            if seen_fragment_offset {
                                return Err(serde::de::Error::duplicate_field("fragment_offset"));
                            }
                            seen_fragment_offset = true;
                            f_fragment_offset = map.next_value::<Option<i32>>()?;
                        }
                        "fragment_size" => {
                            if seen_fragment_size {
                                return Err(serde::de::Error::duplicate_field("fragment_size"));
                            }
                            seen_fragment_size = true;
                            f_fragment_size = map.next_value::<Option<i32>>()?;
                        }
                        "fragmenter" => {
                            if seen_fragmenter {
                                return Err(serde::de::Error::duplicate_field("fragmenter"));
                            }
                            seen_fragmenter = true;
                            f_fragmenter = map
                                .next_value::<Option<crate::types::search::HighlighterFragmenter>>(
                                )?;
                        }
                        "highlight_filter" => {
                            if seen_highlight_filter {
                                return Err(serde::de::Error::duplicate_field("highlight_filter"));
                            }
                            seen_highlight_filter = true;
                            f_highlight_filter = map.next_value::<Option<bool>>()?;
                        }
                        "highlight_query" => {
                            if seen_highlight_query {
                                return Err(serde::de::Error::duplicate_field("highlight_query"));
                            }
                            seen_highlight_query = true;
                            f_highlight_query = map.next_value::<Option<crate::types::common_query_dsl::QueryContainer>>()?;
                        }
                        "max_analyzer_offset" => {
                            if seen_max_analyzer_offset {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_analyzer_offset",
                                ));
                            }
                            seen_max_analyzer_offset = true;
                            f_max_analyzer_offset = map.next_value::<Option<i32>>()?;
                        }
                        "max_fragment_length" => {
                            if seen_max_fragment_length {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_fragment_length",
                                ));
                            }
                            seen_max_fragment_length = true;
                            f_max_fragment_length = map.next_value::<Option<i32>>()?;
                        }
                        "no_match_size" => {
                            if seen_no_match_size {
                                return Err(serde::de::Error::duplicate_field("no_match_size"));
                            }
                            seen_no_match_size = true;
                            f_no_match_size = map.next_value::<Option<i32>>()?;
                        }
                        "number_of_fragments" => {
                            if seen_number_of_fragments {
                                return Err(serde::de::Error::duplicate_field(
                                    "number_of_fragments",
                                ));
                            }
                            seen_number_of_fragments = true;
                            f_number_of_fragments = map.next_value::<Option<i32>>()?;
                        }
                        "options" => {
                            if seen_options {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            seen_options = true;
                            f_options = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    std::collections::BTreeMap<
                                        String,
                                        Box<serde_json::value::RawValue>,
                                    >,
                                >,
                            >>()?;
                        }
                        "order" => {
                            if seen_order {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            seen_order = true;
                            f_order =
                                map.next_value::<Option<crate::types::search::HighlighterOrder>>()?;
                        }
                        "phrase_limit" => {
                            if seen_phrase_limit {
                                return Err(serde::de::Error::duplicate_field("phrase_limit"));
                            }
                            seen_phrase_limit = true;
                            f_phrase_limit = map.next_value::<Option<i32>>()?;
                        }
                        "post_tags" => {
                            if seen_post_tags {
                                return Err(serde::de::Error::duplicate_field("post_tags"));
                            }
                            seen_post_tags = true;
                            f_post_tags = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "pre_tags" => {
                            if seen_pre_tags {
                                return Err(serde::de::Error::duplicate_field("pre_tags"));
                            }
                            seen_pre_tags = true;
                            f_pre_tags = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "require_field_match" => {
                            if seen_require_field_match {
                                return Err(serde::de::Error::duplicate_field(
                                    "require_field_match",
                                ));
                            }
                            seen_require_field_match = true;
                            f_require_field_match = map.next_value::<Option<bool>>()?;
                        }
                        "tags_schema" => {
                            if seen_tags_schema {
                                return Err(serde::de::Error::duplicate_field("tags_schema"));
                            }
                            seen_tags_schema = true;
                            f_tags_schema = map
                                .next_value::<Option<crate::types::search::HighlighterTagsSchema>>(
                                )?;
                        }
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type =
                                map.next_value::<Option<crate::types::search::HighlighterType>>()?;
                        }
                        "matched_fields" => {
                            if seen_matched_fields {
                                return Err(serde::de::Error::duplicate_field("matched_fields"));
                            }
                            seen_matched_fields = true;
                            f_matched_fields =
                                map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HighlightField {
                    boundary_chars: f_boundary_chars,
                    boundary_max_scan: f_boundary_max_scan,
                    boundary_scanner: f_boundary_scanner,
                    boundary_scanner_locale: f_boundary_scanner_locale,
                    force_source: f_force_source,
                    fragment_offset: f_fragment_offset,
                    fragment_size: f_fragment_size,
                    fragmenter: f_fragmenter,
                    highlight_filter: f_highlight_filter,
                    highlight_query: f_highlight_query,
                    max_analyzer_offset: f_max_analyzer_offset,
                    max_fragment_length: f_max_fragment_length,
                    no_match_size: f_no_match_size,
                    number_of_fragments: f_number_of_fragments,
                    options: f_options,
                    order: f_order,
                    phrase_limit: f_phrase_limit,
                    post_tags: f_post_tags,
                    pre_tags: f_pre_tags,
                    require_field_match: f_require_field_match,
                    tags_schema: f_tags_schema,
                    r#type: f_type,
                    matched_fields: f_matched_fields,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HighlightField {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.boundary_chars {
            map.serialize_entry("boundary_chars", v)?;
        }
        if let Some(v) = &self.boundary_max_scan {
            map.serialize_entry("boundary_max_scan", v)?;
        }
        if let Some(v) = &self.boundary_scanner {
            map.serialize_entry("boundary_scanner", v)?;
        }
        if let Some(v) = &self.boundary_scanner_locale {
            map.serialize_entry("boundary_scanner_locale", v)?;
        }
        if let Some(v) = &self.force_source {
            map.serialize_entry("force_source", v)?;
        }
        if let Some(v) = &self.fragment_offset {
            map.serialize_entry("fragment_offset", v)?;
        }
        if let Some(v) = &self.fragment_size {
            map.serialize_entry("fragment_size", v)?;
        }
        if let Some(v) = &self.fragmenter {
            map.serialize_entry("fragmenter", v)?;
        }
        if let Some(v) = &self.highlight_filter {
            map.serialize_entry("highlight_filter", v)?;
        }
        if let Some(v) = &self.highlight_query {
            map.serialize_entry("highlight_query", v)?;
        }
        if let Some(v) = &self.max_analyzer_offset {
            map.serialize_entry("max_analyzer_offset", v)?;
        }
        if let Some(v) = &self.max_fragment_length {
            map.serialize_entry("max_fragment_length", v)?;
        }
        if let Some(v) = &self.no_match_size {
            map.serialize_entry("no_match_size", v)?;
        }
        if let Some(v) = &self.number_of_fragments {
            map.serialize_entry("number_of_fragments", v)?;
        }
        if let Some(v) = &self.options {
            map.serialize_entry("options", v)?;
        }
        if let Some(v) = &self.order {
            map.serialize_entry("order", v)?;
        }
        if let Some(v) = &self.phrase_limit {
            map.serialize_entry("phrase_limit", v)?;
        }
        if let Some(v) = &self.post_tags {
            map.serialize_entry("post_tags", v)?;
        }
        if let Some(v) = &self.pre_tags {
            map.serialize_entry("pre_tags", v)?;
        }
        if let Some(v) = &self.require_field_match {
            map.serialize_entry("require_field_match", v)?;
        }
        if let Some(v) = &self.tags_schema {
            map.serialize_entry("tags_schema", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("type", v)?;
        }
        if let Some(v) = &self.matched_fields {
            map.serialize_entry("matched_fields", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum HighlightFields {
    Object(std::collections::BTreeMap<String, crate::types::search::HighlightField>),
    Array(Vec<std::collections::BTreeMap<String, crate::types::search::HighlightField>>),
}
impl<'de> serde::Deserialize<'de> for HighlightFields {
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
            Some(b'{') => serde_json::from_str(text).map(HighlightFields::Object),
            Some(b'[') => serde_json::from_str(text).map(HighlightFields::Array),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a HighlightFields value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HighlighterEncoder {
    #[doc = "`default`"]
    Default,
    #[doc = "`html`"]
    Html,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl HighlighterEncoder {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            HighlighterEncoder::Default => "default",
            HighlighterEncoder::Html => "html",
            HighlighterEncoder::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for HighlighterEncoder {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HighlighterEncoder {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "default" => HighlighterEncoder::Default,
            "html" => HighlighterEncoder::Html,
            _ => HighlighterEncoder::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HighlighterFragmenter {
    #[doc = "`simple`"]
    Simple,
    #[doc = "`span`"]
    Span,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl HighlighterFragmenter {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            HighlighterFragmenter::Simple => "simple",
            HighlighterFragmenter::Span => "span",
            HighlighterFragmenter::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for HighlighterFragmenter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HighlighterFragmenter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "simple" => HighlighterFragmenter::Simple,
            "span" => HighlighterFragmenter::Span,
            _ => HighlighterFragmenter::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HighlighterOrder {
    #[doc = "`score`"]
    Score,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl HighlighterOrder {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            HighlighterOrder::Score => "score",
            HighlighterOrder::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for HighlighterOrder {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HighlighterOrder {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "score" => HighlighterOrder::Score,
            _ => HighlighterOrder::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HighlighterTagsSchema {
    #[doc = "`default`"]
    Default,
    #[doc = "`styled`"]
    Styled,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl HighlighterTagsSchema {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            HighlighterTagsSchema::Default => "default",
            HighlighterTagsSchema::Styled => "styled",
            HighlighterTagsSchema::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for HighlighterTagsSchema {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HighlighterTagsSchema {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "default" => HighlighterTagsSchema::Default,
            "styled" => HighlighterTagsSchema::Styled,
            _ => HighlighterTagsSchema::Unknown(s),
        })
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct HighlighterType {
    raw: Box<serde_json::value::RawValue>,
}
impl HighlighterType {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `builtin` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_builtin(
        &self,
    ) -> std::result::Result<
        crate::types::search::BuiltinHighlighterType,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "builtin")
    }
    #[doc = "Parses the value as the `custom` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_custom(&self) -> std::result::Result<String, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "custom")
    }
}

#[derive(Debug, Clone)]
pub struct HitsMetadata {
    pub hits: Vec<crate::types::search::HitsMetadataHitsItem>,
    pub max_score: Option<f64>,
    #[doc = "The total number of hits, present only if `track_total_hits` is not set to `false` in the search request."]
    pub total: Option<crate::types::search::HitsMetadataTotal>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HitsMetadata {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HitsMetadata;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HitsMetadata object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_hits: Option<Vec<crate::types::search::HitsMetadataHitsItem>> = None;
                let mut f_max_score: Option<f64> = None;
                let mut seen_max_score = false;
                let mut f_total: Option<crate::types::search::HitsMetadataTotal> = None;
                let mut seen_total = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "hits" => {
                            if f_hits.is_some() {
                                return Err(serde::de::Error::duplicate_field("hits"));
                            }
                            f_hits = Some(
                                map.next_value::<Vec<crate::types::search::HitsMetadataHitsItem>>(
                                )?,
                            );
                        }
                        "max_score" => {
                            if seen_max_score {
                                return Err(serde::de::Error::duplicate_field("max_score"));
                            }
                            seen_max_score = true;
                            f_max_score = map.next_value::<Option<f64>>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map
                                .next_value::<Option<crate::types::search::HitsMetadataTotal>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HitsMetadata {
                    hits: f_hits.ok_or_else(|| serde::de::Error::missing_field("hits"))?,
                    max_score: f_max_score,
                    total: f_total,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HitsMetadata {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("hits", &self.hits)?;
        if let Some(v) = &self.max_score {
            map.serialize_entry("max_score", v)?;
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
impl HitsMetadata {
    #[doc = "The total number of hits (`total`), or `None` when the request disabled counting (`track_total_hits: false`).

The number may be a lower bound: by default the count is only tracked accurately up to 10,000 hits. Use `total` and its [`is_accurate`](crate::types::TotalHitCount::is_accurate) when that matters."]
    pub fn total_value(&self) -> Option<i64> {
        self.total.as_ref().map(|total| total.value())
    }
}

#[derive(Debug, Clone)]
pub struct HitsMetadataHitsItem {
    pub explanation: Option<crate::types::explain::Explanation>,
    pub id: Option<crate::types::common::Id>,
    pub ignored: Option<Vec<String>>,
    pub index: Option<crate::types::common::IndexName>,
    pub nested: Option<crate::types::search::NestedIdentity>,
    pub node: Option<String>,
    pub primary_term: Option<i64>,
    pub routing: Option<String>,
    pub score: Option<f64>,
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    pub shard: Option<String>,
    pub source: Option<Box<serde_json::value::RawValue>>,
    pub r#type: Option<crate::types::common::Type>,
    pub version: Option<crate::types::common::VersionNumber>,
    pub fields: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub highlight: Option<std::collections::BTreeMap<String, Vec<String>>>,
    pub ignored_field_values: Option<std::collections::BTreeMap<String, Vec<String>>>,
    pub inner_hits:
        Option<std::collections::BTreeMap<String, crate::types::search::InnerHitsResult>>,
    #[doc = "The names of queries that matched the document. When `include_named_queries_score` is false (default), returns an array of query names. When true, returns an object mapping query names to their scores."]
    pub matched_queries: Option<crate::types::search::HitsMetadataHitsItemMatchedQueries>,
    pub sort: Option<crate::types::common::SortResults>,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HitsMetadataHitsItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HitsMetadataHitsItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HitsMetadataHitsItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_explanation: Option<crate::types::explain::Explanation> = None;
                let mut seen_explanation = false;
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut seen_id = false;
                let mut f_ignored: Option<Vec<String>> = None;
                let mut seen_ignored = false;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut seen_index = false;
                let mut f_nested: Option<crate::types::search::NestedIdentity> = None;
                let mut seen_nested = false;
                let mut f_node: Option<String> = None;
                let mut seen_node = false;
                let mut f_primary_term: Option<i64> = None;
                let mut seen_primary_term = false;
                let mut f_routing: Option<String> = None;
                let mut seen_routing = false;
                let mut f_score: Option<f64> = None;
                let mut seen_score = false;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut seen_seq_no = false;
                let mut f_shard: Option<String> = None;
                let mut seen_shard = false;
                let mut f_source: Option<Box<serde_json::value::RawValue>> = None;
                let mut seen_source = false;
                let mut f_type: Option<crate::types::common::Type> = None;
                let mut seen_type = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_fields = false;
                let mut f_highlight: Option<std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut seen_highlight = false;
                let mut f_ignored_field_values: Option<
                    std::collections::BTreeMap<String, Vec<String>>,
                > = None;
                let mut seen_ignored_field_values = false;
                let mut f_inner_hits: Option<
                    std::collections::BTreeMap<String, crate::types::search::InnerHitsResult>,
                > = None;
                let mut seen_inner_hits = false;
                let mut f_matched_queries: Option<
                    crate::types::search::HitsMetadataHitsItemMatchedQueries,
                > = None;
                let mut seen_matched_queries = false;
                let mut f_sort: Option<crate::types::common::SortResults> = None;
                let mut seen_sort = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_explanation" => {
                            if seen_explanation {
                                return Err(serde::de::Error::duplicate_field("_explanation"));
                            }
                            seen_explanation = true;
                            f_explanation =
                                map.next_value::<Option<crate::types::explain::Explanation>>()?;
                        }
                        "_id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("_id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        "_ignored" => {
                            if seen_ignored {
                                return Err(serde::de::Error::duplicate_field("_ignored"));
                            }
                            seen_ignored = true;
                            f_ignored = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "_index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("_index"));
                            }
                            seen_index = true;
                            f_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "_nested" => {
                            if seen_nested {
                                return Err(serde::de::Error::duplicate_field("_nested"));
                            }
                            seen_nested = true;
                            f_nested =
                                map.next_value::<Option<crate::types::search::NestedIdentity>>()?;
                        }
                        "_node" => {
                            if seen_node {
                                return Err(serde::de::Error::duplicate_field("_node"));
                            }
                            seen_node = true;
                            f_node = map.next_value::<Option<String>>()?;
                        }
                        "_primary_term" => {
                            if seen_primary_term {
                                return Err(serde::de::Error::duplicate_field("_primary_term"));
                            }
                            seen_primary_term = true;
                            f_primary_term = map.next_value::<Option<i64>>()?;
                        }
                        "_routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("_routing"));
                            }
                            seen_routing = true;
                            f_routing = map.next_value::<Option<String>>()?;
                        }
                        "_score" => {
                            if seen_score {
                                return Err(serde::de::Error::duplicate_field("_score"));
                            }
                            seen_score = true;
                            f_score = map.next_value::<Option<f64>>()?;
                        }
                        "_seq_no" => {
                            if seen_seq_no {
                                return Err(serde::de::Error::duplicate_field("_seq_no"));
                            }
                            seen_seq_no = true;
                            f_seq_no =
                                map.next_value::<Option<crate::types::common::SequenceNumber>>()?;
                        }
                        "_shard" => {
                            if seen_shard {
                                return Err(serde::de::Error::duplicate_field("_shard"));
                            }
                            seen_shard = true;
                            f_shard = map.next_value::<Option<String>>()?;
                        }
                        "_source" => {
                            if seen_source {
                                return Err(serde::de::Error::duplicate_field("_source"));
                            }
                            seen_source = true;
                            f_source =
                                map.next_value::<Option<Box<serde_json::value::RawValue>>>()?;
                        }
                        "_type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<crate::types::common::Type>>()?;
                        }
                        "_version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionNumber>>()?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "highlight" => {
                            if seen_highlight {
                                return Err(serde::de::Error::duplicate_field("highlight"));
                            }
                            seen_highlight = true;
                            f_highlight = map.next_value::<Option<std::collections::BTreeMap<String, Vec<String>>>>()?;
                        }
                        "ignored_field_values" => {
                            if seen_ignored_field_values {
                                return Err(serde::de::Error::duplicate_field(
                                    "ignored_field_values",
                                ));
                            }
                            seen_ignored_field_values = true;
                            f_ignored_field_values = map.next_value::<Option<std::collections::BTreeMap<String, Vec<String>>>>()?;
                        }
                        "inner_hits" => {
                            if seen_inner_hits {
                                return Err(serde::de::Error::duplicate_field("inner_hits"));
                            }
                            seen_inner_hits = true;
                            f_inner_hits = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::search::InnerHitsResult,
                                >,
                            >>()?;
                        }
                        "matched_queries" => {
                            if seen_matched_queries {
                                return Err(serde::de::Error::duplicate_field("matched_queries"));
                            }
                            seen_matched_queries = true;
                            f_matched_queries =
                                map.next_value::<Option<
                                    crate::types::search::HitsMetadataHitsItemMatchedQueries,
                                >>()?;
                        }
                        "sort" => {
                            if seen_sort {
                                return Err(serde::de::Error::duplicate_field("sort"));
                            }
                            seen_sort = true;
                            f_sort =
                                map.next_value::<Option<crate::types::common::SortResults>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HitsMetadataHitsItem {
                    explanation: f_explanation,
                    id: f_id,
                    ignored: f_ignored,
                    index: f_index,
                    nested: f_nested,
                    node: f_node,
                    primary_term: f_primary_term,
                    routing: f_routing,
                    score: f_score,
                    seq_no: f_seq_no,
                    shard: f_shard,
                    source: f_source,
                    r#type: f_type,
                    version: f_version,
                    fields: f_fields,
                    highlight: f_highlight,
                    ignored_field_values: f_ignored_field_values,
                    inner_hits: f_inner_hits,
                    matched_queries: f_matched_queries,
                    sort: f_sort,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HitsMetadataHitsItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.explanation {
            map.serialize_entry("_explanation", v)?;
        }
        if let Some(v) = &self.id {
            map.serialize_entry("_id", v)?;
        }
        if let Some(v) = &self.ignored {
            map.serialize_entry("_ignored", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("_index", v)?;
        }
        if let Some(v) = &self.nested {
            map.serialize_entry("_nested", v)?;
        }
        if let Some(v) = &self.node {
            map.serialize_entry("_node", v)?;
        }
        if let Some(v) = &self.primary_term {
            map.serialize_entry("_primary_term", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("_routing", v)?;
        }
        if let Some(v) = &self.score {
            map.serialize_entry("_score", v)?;
        }
        if let Some(v) = &self.seq_no {
            map.serialize_entry("_seq_no", v)?;
        }
        if let Some(v) = &self.shard {
            map.serialize_entry("_shard", v)?;
        }
        if let Some(v) = &self.source {
            map.serialize_entry("_source", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("_type", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("_version", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.highlight {
            map.serialize_entry("highlight", v)?;
        }
        if let Some(v) = &self.ignored_field_values {
            map.serialize_entry("ignored_field_values", v)?;
        }
        if let Some(v) = &self.inner_hits {
            map.serialize_entry("inner_hits", v)?;
        }
        if let Some(v) = &self.matched_queries {
            map.serialize_entry("matched_queries", v)?;
        }
        if let Some(v) = &self.sort {
            map.serialize_entry("sort", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
impl HitsMetadataHitsItem {
    #[doc = "Decodes the document body (`_source`) into `T`.\n\nReturns `Ok(None)` when the response carries no `_source` (`_source: false` in the request, or a mapping that disables it), and an error when the document does not match `T`.\n\n`T` may borrow from the response to avoid copying strings: use `#[serde(borrow)] Cow<'a, str>` fields rather than `&'a str`, which cannot hold a string containing JSON escapes."]
    pub fn source_as<'a, T>(&'a self) -> std::result::Result<Option<T>, serde_json::Error>
    where
        T: serde::Deserialize<'a>,
    {
        self.source
            .as_deref()
            .map(|raw| serde_json::from_str(raw.get()))
            .transpose()
    }
}

#[doc = "The names of queries that matched the document. When `include_named_queries_score` is false (default), returns an array of query names. When true, returns an object mapping query names to their scores."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum HitsMetadataHitsItemMatchedQueries {
    Names(Vec<String>),
    Scores(std::collections::BTreeMap<String, f64>),
}
impl<'de> serde::Deserialize<'de> for HitsMetadataHitsItemMatchedQueries {
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
            Some(b'[') => serde_json::from_str(text).map(HitsMetadataHitsItemMatchedQueries::Names),
            Some(b'{') => {
                serde_json::from_str(text).map(HitsMetadataHitsItemMatchedQueries::Scores)
            }
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a HitsMetadataHitsItemMatchedQueries value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct HitsMetadataJsonValue {
    pub hits: Vec<crate::types::search::HitsMetadataJsonValueHitsItem>,
    pub max_score: Option<f64>,
    #[doc = "The total number of hits, present only if `track_total_hits` is not set to `false` in the search request."]
    pub total: Option<crate::types::search::HitsMetadataJsonValueTotal>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HitsMetadataJsonValue {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HitsMetadataJsonValue;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HitsMetadataJsonValue object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_hits: Option<Vec<crate::types::search::HitsMetadataJsonValueHitsItem>> =
                    None;
                let mut f_max_score: Option<f64> = None;
                let mut seen_max_score = false;
                let mut f_total: Option<crate::types::search::HitsMetadataJsonValueTotal> = None;
                let mut seen_total = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "hits" => {
                            if f_hits.is_some() {
                                return Err(serde::de::Error::duplicate_field("hits"));
                            }
                            f_hits = Some(map.next_value::<Vec<crate::types::search::HitsMetadataJsonValueHitsItem>>()?);
                        }
                        "max_score" => {
                            if seen_max_score {
                                return Err(serde::de::Error::duplicate_field("max_score"));
                            }
                            seen_max_score = true;
                            f_max_score = map.next_value::<Option<f64>>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map.next_value::<Option<crate::types::search::HitsMetadataJsonValueTotal>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HitsMetadataJsonValue {
                    hits: f_hits.ok_or_else(|| serde::de::Error::missing_field("hits"))?,
                    max_score: f_max_score,
                    total: f_total,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HitsMetadataJsonValue {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("hits", &self.hits)?;
        if let Some(v) = &self.max_score {
            map.serialize_entry("max_score", v)?;
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
impl HitsMetadataJsonValue {
    #[doc = "The total number of hits (`total`), or `None` when the request disabled counting (`track_total_hits: false`).

The number may be a lower bound: by default the count is only tracked accurately up to 10,000 hits. Use `total` and its [`is_accurate`](crate::types::TotalHitCount::is_accurate) when that matters."]
    pub fn total_value(&self) -> Option<i64> {
        self.total.as_ref().map(|total| total.value())
    }
}

#[derive(Debug, Clone)]
pub struct HitsMetadataJsonValueHitsItem {
    pub explanation: Option<crate::types::explain::Explanation>,
    pub id: Option<crate::types::common::Id>,
    pub ignored: Option<Vec<String>>,
    pub index: Option<crate::types::common::IndexName>,
    pub nested: Option<crate::types::search::NestedIdentity>,
    pub node: Option<String>,
    pub primary_term: Option<i64>,
    pub routing: Option<String>,
    pub score: Option<f64>,
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    pub shard: Option<String>,
    pub source: Option<Box<serde_json::value::RawValue>>,
    pub r#type: Option<crate::types::common::Type>,
    pub version: Option<crate::types::common::VersionNumber>,
    pub fields: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub highlight: Option<std::collections::BTreeMap<String, Vec<String>>>,
    pub ignored_field_values: Option<std::collections::BTreeMap<String, Vec<String>>>,
    pub inner_hits:
        Option<std::collections::BTreeMap<String, crate::types::search::InnerHitsResult>>,
    #[doc = "The names of queries that matched the document. When `include_named_queries_score` is false (default), returns an array of query names. When true, returns an object mapping query names to their scores."]
    pub matched_queries: Option<crate::types::search::HitsMetadataJsonValueHitsItemMatchedQueries>,
    pub sort: Option<crate::types::common::SortResults>,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HitsMetadataJsonValueHitsItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HitsMetadataJsonValueHitsItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HitsMetadataJsonValueHitsItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_explanation: Option<crate::types::explain::Explanation> = None;
                let mut seen_explanation = false;
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut seen_id = false;
                let mut f_ignored: Option<Vec<String>> = None;
                let mut seen_ignored = false;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut seen_index = false;
                let mut f_nested: Option<crate::types::search::NestedIdentity> = None;
                let mut seen_nested = false;
                let mut f_node: Option<String> = None;
                let mut seen_node = false;
                let mut f_primary_term: Option<i64> = None;
                let mut seen_primary_term = false;
                let mut f_routing: Option<String> = None;
                let mut seen_routing = false;
                let mut f_score: Option<f64> = None;
                let mut seen_score = false;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut seen_seq_no = false;
                let mut f_shard: Option<String> = None;
                let mut seen_shard = false;
                let mut f_source: Option<Box<serde_json::value::RawValue>> = None;
                let mut seen_source = false;
                let mut f_type: Option<crate::types::common::Type> = None;
                let mut seen_type = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_fields = false;
                let mut f_highlight: Option<std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut seen_highlight = false;
                let mut f_ignored_field_values: Option<
                    std::collections::BTreeMap<String, Vec<String>>,
                > = None;
                let mut seen_ignored_field_values = false;
                let mut f_inner_hits: Option<
                    std::collections::BTreeMap<String, crate::types::search::InnerHitsResult>,
                > = None;
                let mut seen_inner_hits = false;
                let mut f_matched_queries: Option<
                    crate::types::search::HitsMetadataJsonValueHitsItemMatchedQueries,
                > = None;
                let mut seen_matched_queries = false;
                let mut f_sort: Option<crate::types::common::SortResults> = None;
                let mut seen_sort = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_explanation" => {
                            if seen_explanation {
                                return Err(serde::de::Error::duplicate_field("_explanation"));
                            }
                            seen_explanation = true;
                            f_explanation =
                                map.next_value::<Option<crate::types::explain::Explanation>>()?;
                        }
                        "_id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("_id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        "_ignored" => {
                            if seen_ignored {
                                return Err(serde::de::Error::duplicate_field("_ignored"));
                            }
                            seen_ignored = true;
                            f_ignored = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "_index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("_index"));
                            }
                            seen_index = true;
                            f_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "_nested" => {
                            if seen_nested {
                                return Err(serde::de::Error::duplicate_field("_nested"));
                            }
                            seen_nested = true;
                            f_nested =
                                map.next_value::<Option<crate::types::search::NestedIdentity>>()?;
                        }
                        "_node" => {
                            if seen_node {
                                return Err(serde::de::Error::duplicate_field("_node"));
                            }
                            seen_node = true;
                            f_node = map.next_value::<Option<String>>()?;
                        }
                        "_primary_term" => {
                            if seen_primary_term {
                                return Err(serde::de::Error::duplicate_field("_primary_term"));
                            }
                            seen_primary_term = true;
                            f_primary_term = map.next_value::<Option<i64>>()?;
                        }
                        "_routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("_routing"));
                            }
                            seen_routing = true;
                            f_routing = map.next_value::<Option<String>>()?;
                        }
                        "_score" => {
                            if seen_score {
                                return Err(serde::de::Error::duplicate_field("_score"));
                            }
                            seen_score = true;
                            f_score = map.next_value::<Option<f64>>()?;
                        }
                        "_seq_no" => {
                            if seen_seq_no {
                                return Err(serde::de::Error::duplicate_field("_seq_no"));
                            }
                            seen_seq_no = true;
                            f_seq_no =
                                map.next_value::<Option<crate::types::common::SequenceNumber>>()?;
                        }
                        "_shard" => {
                            if seen_shard {
                                return Err(serde::de::Error::duplicate_field("_shard"));
                            }
                            seen_shard = true;
                            f_shard = map.next_value::<Option<String>>()?;
                        }
                        "_source" => {
                            if seen_source {
                                return Err(serde::de::Error::duplicate_field("_source"));
                            }
                            seen_source = true;
                            f_source =
                                map.next_value::<Option<Box<serde_json::value::RawValue>>>()?;
                        }
                        "_type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<crate::types::common::Type>>()?;
                        }
                        "_version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionNumber>>()?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "highlight" => {
                            if seen_highlight {
                                return Err(serde::de::Error::duplicate_field("highlight"));
                            }
                            seen_highlight = true;
                            f_highlight = map.next_value::<Option<std::collections::BTreeMap<String, Vec<String>>>>()?;
                        }
                        "ignored_field_values" => {
                            if seen_ignored_field_values {
                                return Err(serde::de::Error::duplicate_field(
                                    "ignored_field_values",
                                ));
                            }
                            seen_ignored_field_values = true;
                            f_ignored_field_values = map.next_value::<Option<std::collections::BTreeMap<String, Vec<String>>>>()?;
                        }
                        "inner_hits" => {
                            if seen_inner_hits {
                                return Err(serde::de::Error::duplicate_field("inner_hits"));
                            }
                            seen_inner_hits = true;
                            f_inner_hits = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::search::InnerHitsResult,
                                >,
                            >>()?;
                        }
                        "matched_queries" => {
                            if seen_matched_queries {
                                return Err(serde::de::Error::duplicate_field("matched_queries"));
                            }
                            seen_matched_queries = true;
                            f_matched_queries = map.next_value::<Option<
                                crate::types::search::HitsMetadataJsonValueHitsItemMatchedQueries,
                            >>()?;
                        }
                        "sort" => {
                            if seen_sort {
                                return Err(serde::de::Error::duplicate_field("sort"));
                            }
                            seen_sort = true;
                            f_sort =
                                map.next_value::<Option<crate::types::common::SortResults>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HitsMetadataJsonValueHitsItem {
                    explanation: f_explanation,
                    id: f_id,
                    ignored: f_ignored,
                    index: f_index,
                    nested: f_nested,
                    node: f_node,
                    primary_term: f_primary_term,
                    routing: f_routing,
                    score: f_score,
                    seq_no: f_seq_no,
                    shard: f_shard,
                    source: f_source,
                    r#type: f_type,
                    version: f_version,
                    fields: f_fields,
                    highlight: f_highlight,
                    ignored_field_values: f_ignored_field_values,
                    inner_hits: f_inner_hits,
                    matched_queries: f_matched_queries,
                    sort: f_sort,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HitsMetadataJsonValueHitsItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.explanation {
            map.serialize_entry("_explanation", v)?;
        }
        if let Some(v) = &self.id {
            map.serialize_entry("_id", v)?;
        }
        if let Some(v) = &self.ignored {
            map.serialize_entry("_ignored", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("_index", v)?;
        }
        if let Some(v) = &self.nested {
            map.serialize_entry("_nested", v)?;
        }
        if let Some(v) = &self.node {
            map.serialize_entry("_node", v)?;
        }
        if let Some(v) = &self.primary_term {
            map.serialize_entry("_primary_term", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("_routing", v)?;
        }
        if let Some(v) = &self.score {
            map.serialize_entry("_score", v)?;
        }
        if let Some(v) = &self.seq_no {
            map.serialize_entry("_seq_no", v)?;
        }
        if let Some(v) = &self.shard {
            map.serialize_entry("_shard", v)?;
        }
        if let Some(v) = &self.source {
            map.serialize_entry("_source", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("_type", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("_version", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.highlight {
            map.serialize_entry("highlight", v)?;
        }
        if let Some(v) = &self.ignored_field_values {
            map.serialize_entry("ignored_field_values", v)?;
        }
        if let Some(v) = &self.inner_hits {
            map.serialize_entry("inner_hits", v)?;
        }
        if let Some(v) = &self.matched_queries {
            map.serialize_entry("matched_queries", v)?;
        }
        if let Some(v) = &self.sort {
            map.serialize_entry("sort", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
impl HitsMetadataJsonValueHitsItem {
    #[doc = "Decodes the document body (`_source`) into `T`.\n\nReturns `Ok(None)` when the response carries no `_source` (`_source: false` in the request, or a mapping that disables it), and an error when the document does not match `T`.\n\n`T` may borrow from the response to avoid copying strings: use `#[serde(borrow)] Cow<'a, str>` fields rather than `&'a str`, which cannot hold a string containing JSON escapes."]
    pub fn source_as<'a, T>(&'a self) -> std::result::Result<Option<T>, serde_json::Error>
    where
        T: serde::Deserialize<'a>,
    {
        self.source
            .as_deref()
            .map(|raw| serde_json::from_str(raw.get()))
            .transpose()
    }
}

#[doc = "The names of queries that matched the document. When `include_named_queries_score` is false (default), returns an array of query names. When true, returns an object mapping query names to their scores."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum HitsMetadataJsonValueHitsItemMatchedQueries {
    Names(Vec<String>),
    Scores(std::collections::BTreeMap<String, f64>),
}
impl<'de> serde::Deserialize<'de> for HitsMetadataJsonValueHitsItemMatchedQueries {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed =
            match text.as_bytes().first() {
                Some(b'[') => serde_json::from_str(text)
                    .map(HitsMetadataJsonValueHitsItemMatchedQueries::Names),
                Some(b'{') => serde_json::from_str(text)
                    .map(HitsMetadataJsonValueHitsItemMatchedQueries::Scores),
                other => {
                    return Err(serde::de::Error::custom(format_args!(
                "invalid type: {}, expected a HitsMetadataJsonValueHitsItemMatchedQueries value",
                crate::types::union::json_kind_name(other.copied())
            )))
                }
            };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The total number of hits, present only if `track_total_hits` is not set to `false` in the search request."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum HitsMetadataJsonValueTotal {
    TotalHits(crate::types::search::TotalHits),
    Integer(i64),
}
impl<'de> serde::Deserialize<'de> for HitsMetadataJsonValueTotal {
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
            Some(b'{') => serde_json::from_str(text).map(HitsMetadataJsonValueTotal::TotalHits),
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(HitsMetadataJsonValueTotal::Integer)
            }
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a HitsMetadataJsonValueTotal value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}
impl crate::types::TotalHitCount for HitsMetadataJsonValueTotal {
    fn value(&self) -> i64 {
        match self {
            Self::TotalHits(total) => total.value,
            Self::Integer(value) => *value,
        }
    }

    fn relation(&self) -> &crate::types::search::TotalHitsRelation {
        match self {
            Self::TotalHits(total) => &total.relation,
            Self::Integer(_) => &crate::types::total_hits::ACCURATE,
        }
    }
}

impl HitsMetadataJsonValueTotal {
    #[doc = "The number of hits: exact when [`is_accurate`](Self::is_accurate) is true, and otherwise a lower bound. See [`TotalHitCount`](crate::types::TotalHitCount)."]
    pub fn value(&self) -> i64 {
        crate::types::TotalHitCount::value(self)
    }

    #[doc = "How [`value`](Self::value) relates to the actual number of hits: `Eq` when exact, `Gte` when a lower bound. The integer form is always `Eq`."]
    pub fn relation(&self) -> &crate::types::search::TotalHitsRelation {
        crate::types::TotalHitCount::relation(self)
    }

    #[doc = "Whether [`value`](Self::value) is the exact number of hits (relation `eq`). False for a lower bound, and for a relation this client does not know."]
    pub fn is_accurate(&self) -> bool {
        crate::types::TotalHitCount::is_accurate(self)
    }
}

#[doc = "The total number of hits, present only if `track_total_hits` is not set to `false` in the search request."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum HitsMetadataTotal {
    TotalHits(crate::types::search::TotalHits),
    Integer(i64),
}
impl<'de> serde::Deserialize<'de> for HitsMetadataTotal {
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
            Some(b'{') => serde_json::from_str(text).map(HitsMetadataTotal::TotalHits),
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(HitsMetadataTotal::Integer)
            }
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a HitsMetadataTotal value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}
impl crate::types::TotalHitCount for HitsMetadataTotal {
    fn value(&self) -> i64 {
        match self {
            Self::TotalHits(total) => total.value,
            Self::Integer(value) => *value,
        }
    }

    fn relation(&self) -> &crate::types::search::TotalHitsRelation {
        match self {
            Self::TotalHits(total) => &total.relation,
            Self::Integer(_) => &crate::types::total_hits::ACCURATE,
        }
    }
}

impl HitsMetadataTotal {
    #[doc = "The number of hits: exact when [`is_accurate`](Self::is_accurate) is true, and otherwise a lower bound. See [`TotalHitCount`](crate::types::TotalHitCount)."]
    pub fn value(&self) -> i64 {
        crate::types::TotalHitCount::value(self)
    }

    #[doc = "How [`value`](Self::value) relates to the actual number of hits: `Eq` when exact, `Gte` when a lower bound. The integer form is always `Eq`."]
    pub fn relation(&self) -> &crate::types::search::TotalHitsRelation {
        crate::types::TotalHitCount::relation(self)
    }

    #[doc = "Whether [`value`](Self::value) is the exact number of hits (relation `eq`). False for a lower bound, and for a relation this client does not know."]
    pub fn is_accurate(&self) -> bool {
        crate::types::TotalHitCount::is_accurate(self)
    }
}

#[derive(Debug, Clone)]
pub struct InnerHits {
    pub source: Option<crate::types::search::SourceConfig>,
    pub collapse: Option<Box<crate::types::search::FieldCollapse>>,
    pub docvalue_fields: Option<Vec<crate::types::common_query_dsl::FieldAndFormat>>,
    pub explain: Option<bool>,
    pub fields: Option<Vec<crate::types::common_query_dsl::FieldAndFormat>>,
    #[doc = "The inner hit that initiates document offset."]
    pub from: Option<i32>,
    pub highlight: Option<Box<crate::types::search::Highlight>>,
    pub ignore_unmapped: Option<bool>,
    pub name: Option<crate::types::common::Name>,
    pub script_fields:
        Option<std::collections::BTreeMap<String, crate::types::common::ScriptField>>,
    pub seq_no_primary_term: Option<bool>,
    #[doc = "The maximum number of hits to return per `inner_hits`."]
    pub size: Option<i32>,
    pub sort: Option<crate::types::common::Sort>,
    pub stored_fields: Option<crate::types::common::Fields>,
    pub track_scores: Option<bool>,
    pub version: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for InnerHits {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = InnerHits;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a InnerHits object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_source: Option<crate::types::search::SourceConfig> = None;
                let mut seen_source = false;
                let mut f_collapse: Option<Box<crate::types::search::FieldCollapse>> = None;
                let mut seen_collapse = false;
                let mut f_docvalue_fields: Option<
                    Vec<crate::types::common_query_dsl::FieldAndFormat>,
                > = None;
                let mut seen_docvalue_fields = false;
                let mut f_explain: Option<bool> = None;
                let mut seen_explain = false;
                let mut f_fields: Option<Vec<crate::types::common_query_dsl::FieldAndFormat>> =
                    None;
                let mut seen_fields = false;
                let mut f_from: Option<i32> = None;
                let mut seen_from = false;
                let mut f_highlight: Option<Box<crate::types::search::Highlight>> = None;
                let mut seen_highlight = false;
                let mut f_ignore_unmapped: Option<bool> = None;
                let mut seen_ignore_unmapped = false;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut seen_name = false;
                let mut f_script_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common::ScriptField>,
                > = None;
                let mut seen_script_fields = false;
                let mut f_seq_no_primary_term: Option<bool> = None;
                let mut seen_seq_no_primary_term = false;
                let mut f_size: Option<i32> = None;
                let mut seen_size = false;
                let mut f_sort: Option<crate::types::common::Sort> = None;
                let mut seen_sort = false;
                let mut f_stored_fields: Option<crate::types::common::Fields> = None;
                let mut seen_stored_fields = false;
                let mut f_track_scores: Option<bool> = None;
                let mut seen_track_scores = false;
                let mut f_version: Option<bool> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_source" => {
                            if seen_source {
                                return Err(serde::de::Error::duplicate_field("_source"));
                            }
                            seen_source = true;
                            f_source =
                                map.next_value::<Option<crate::types::search::SourceConfig>>()?;
                        }
                        "collapse" => {
                            if seen_collapse {
                                return Err(serde::de::Error::duplicate_field("collapse"));
                            }
                            seen_collapse = true;
                            f_collapse = map
                                .next_value::<Option<Box<crate::types::search::FieldCollapse>>>()?;
                        }
                        "docvalue_fields" => {
                            if seen_docvalue_fields {
                                return Err(serde::de::Error::duplicate_field("docvalue_fields"));
                            }
                            seen_docvalue_fields = true;
                            f_docvalue_fields = map.next_value::<Option<Vec<crate::types::common_query_dsl::FieldAndFormat>>>()?;
                        }
                        "explain" => {
                            if seen_explain {
                                return Err(serde::de::Error::duplicate_field("explain"));
                            }
                            seen_explain = true;
                            f_explain = map.next_value::<Option<bool>>()?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<Vec<crate::types::common_query_dsl::FieldAndFormat>>>()?;
                        }
                        "from" => {
                            if seen_from {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            seen_from = true;
                            f_from = map.next_value::<Option<i32>>()?;
                        }
                        "highlight" => {
                            if seen_highlight {
                                return Err(serde::de::Error::duplicate_field("highlight"));
                            }
                            seen_highlight = true;
                            f_highlight =
                                map.next_value::<Option<Box<crate::types::search::Highlight>>>()?;
                        }
                        "ignore_unmapped" => {
                            if seen_ignore_unmapped {
                                return Err(serde::de::Error::duplicate_field("ignore_unmapped"));
                            }
                            seen_ignore_unmapped = true;
                            f_ignore_unmapped = map.next_value::<Option<bool>>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "script_fields" => {
                            if seen_script_fields {
                                return Err(serde::de::Error::duplicate_field("script_fields"));
                            }
                            seen_script_fields = true;
                            f_script_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common::ScriptField,
                                >,
                            >>()?;
                        }
                        "seq_no_primary_term" => {
                            if seen_seq_no_primary_term {
                                return Err(serde::de::Error::duplicate_field(
                                    "seq_no_primary_term",
                                ));
                            }
                            seen_seq_no_primary_term = true;
                            f_seq_no_primary_term = map.next_value::<Option<bool>>()?;
                        }
                        "size" => {
                            if seen_size {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            seen_size = true;
                            f_size = map.next_value::<Option<i32>>()?;
                        }
                        "sort" => {
                            if seen_sort {
                                return Err(serde::de::Error::duplicate_field("sort"));
                            }
                            seen_sort = true;
                            f_sort = map.next_value::<Option<crate::types::common::Sort>>()?;
                        }
                        "stored_fields" => {
                            if seen_stored_fields {
                                return Err(serde::de::Error::duplicate_field("stored_fields"));
                            }
                            seen_stored_fields = true;
                            f_stored_fields =
                                map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "track_scores" => {
                            if seen_track_scores {
                                return Err(serde::de::Error::duplicate_field("track_scores"));
                            }
                            seen_track_scores = true;
                            f_track_scores = map.next_value::<Option<bool>>()?;
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(InnerHits {
                    source: f_source,
                    collapse: f_collapse,
                    docvalue_fields: f_docvalue_fields,
                    explain: f_explain,
                    fields: f_fields,
                    from: f_from,
                    highlight: f_highlight,
                    ignore_unmapped: f_ignore_unmapped,
                    name: f_name,
                    script_fields: f_script_fields,
                    seq_no_primary_term: f_seq_no_primary_term,
                    size: f_size,
                    sort: f_sort,
                    stored_fields: f_stored_fields,
                    track_scores: f_track_scores,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for InnerHits {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.source {
            map.serialize_entry("_source", v)?;
        }
        if let Some(v) = &self.collapse {
            map.serialize_entry("collapse", v)?;
        }
        if let Some(v) = &self.docvalue_fields {
            map.serialize_entry("docvalue_fields", v)?;
        }
        if let Some(v) = &self.explain {
            map.serialize_entry("explain", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.from {
            map.serialize_entry("from", v)?;
        }
        if let Some(v) = &self.highlight {
            map.serialize_entry("highlight", v)?;
        }
        if let Some(v) = &self.ignore_unmapped {
            map.serialize_entry("ignore_unmapped", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.script_fields {
            map.serialize_entry("script_fields", v)?;
        }
        if let Some(v) = &self.seq_no_primary_term {
            map.serialize_entry("seq_no_primary_term", v)?;
        }
        if let Some(v) = &self.size {
            map.serialize_entry("size", v)?;
        }
        if let Some(v) = &self.sort {
            map.serialize_entry("sort", v)?;
        }
        if let Some(v) = &self.stored_fields {
            map.serialize_entry("stored_fields", v)?;
        }
        if let Some(v) = &self.track_scores {
            map.serialize_entry("track_scores", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct InnerHitsResult {
    pub hits: crate::types::search::HitsMetadata,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for InnerHitsResult {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = InnerHitsResult;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a InnerHitsResult object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_hits: Option<crate::types::search::HitsMetadata> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "hits" => {
                            if f_hits.is_some() {
                                return Err(serde::de::Error::duplicate_field("hits"));
                            }
                            f_hits = Some(map.next_value::<crate::types::search::HitsMetadata>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(InnerHitsResult {
                    hits: f_hits.ok_or_else(|| serde::de::Error::missing_field("hits"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for InnerHitsResult {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("hits", &self.hits)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct LaplaceSmoothingModel {
    pub alpha: f64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for LaplaceSmoothingModel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = LaplaceSmoothingModel;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a LaplaceSmoothingModel object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_alpha: Option<f64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "alpha" => {
                            if f_alpha.is_some() {
                                return Err(serde::de::Error::duplicate_field("alpha"));
                            }
                            f_alpha = Some(map.next_value::<f64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(LaplaceSmoothingModel {
                    alpha: f_alpha.ok_or_else(|| serde::de::Error::missing_field("alpha"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for LaplaceSmoothingModel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("alpha", &self.alpha)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct LinearInterpolationSmoothingModel {
    pub bigram_lambda: f64,
    pub trigram_lambda: f64,
    pub unigram_lambda: f64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for LinearInterpolationSmoothingModel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = LinearInterpolationSmoothingModel;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a LinearInterpolationSmoothingModel object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_bigram_lambda: Option<f64> = None;
                let mut f_trigram_lambda: Option<f64> = None;
                let mut f_unigram_lambda: Option<f64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "bigram_lambda" => {
                            if f_bigram_lambda.is_some() {
                                return Err(serde::de::Error::duplicate_field("bigram_lambda"));
                            }
                            f_bigram_lambda = Some(map.next_value::<f64>()?);
                        }
                        "trigram_lambda" => {
                            if f_trigram_lambda.is_some() {
                                return Err(serde::de::Error::duplicate_field("trigram_lambda"));
                            }
                            f_trigram_lambda = Some(map.next_value::<f64>()?);
                        }
                        "unigram_lambda" => {
                            if f_unigram_lambda.is_some() {
                                return Err(serde::de::Error::duplicate_field("unigram_lambda"));
                            }
                            f_unigram_lambda = Some(map.next_value::<f64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(LinearInterpolationSmoothingModel {
                    bigram_lambda: f_bigram_lambda
                        .ok_or_else(|| serde::de::Error::missing_field("bigram_lambda"))?,
                    trigram_lambda: f_trigram_lambda
                        .ok_or_else(|| serde::de::Error::missing_field("trigram_lambda"))?,
                    unigram_lambda: f_unigram_lambda
                        .ok_or_else(|| serde::de::Error::missing_field("unigram_lambda"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for LinearInterpolationSmoothingModel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("bigram_lambda", &self.bigram_lambda)?;
        map.serialize_entry("trigram_lambda", &self.trigram_lambda)?;
        map.serialize_entry("unigram_lambda", &self.unigram_lambda)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NestedIdentity {
    pub nested: Option<Box<crate::types::search::NestedIdentity>>,
    pub field: crate::types::common::Field,
    pub offset: i32,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NestedIdentity {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NestedIdentity;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NestedIdentity object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_nested: Option<Box<crate::types::search::NestedIdentity>> = None;
                let mut seen_nested = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_offset: Option<i32> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_nested" => {
                            if seen_nested {
                                return Err(serde::de::Error::duplicate_field("_nested"));
                            }
                            seen_nested = true;
                            f_nested = map
                                .next_value::<Option<Box<crate::types::search::NestedIdentity>>>(
                                )?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "offset" => {
                            if f_offset.is_some() {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            f_offset = Some(map.next_value::<i32>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NestedIdentity {
                    nested: f_nested,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    offset: f_offset.ok_or_else(|| serde::de::Error::missing_field("offset"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NestedIdentity {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.nested {
            map.serialize_entry("_nested", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        map.serialize_entry("offset", &self.offset)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PhraseSuggest {
    pub length: i32,
    pub offset: i32,
    pub text: String,
    pub options: crate::types::search::PhraseSuggestOptions,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PhraseSuggest {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PhraseSuggest;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PhraseSuggest object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_length: Option<i32> = None;
                let mut f_offset: Option<i32> = None;
                let mut f_text: Option<String> = None;
                let mut f_options: Option<crate::types::search::PhraseSuggestOptions> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "length" => {
                            if f_length.is_some() {
                                return Err(serde::de::Error::duplicate_field("length"));
                            }
                            f_length = Some(map.next_value::<i32>()?);
                        }
                        "offset" => {
                            if f_offset.is_some() {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            f_offset = Some(map.next_value::<i32>()?);
                        }
                        "text" => {
                            if f_text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            f_text = Some(map.next_value::<String>()?);
                        }
                        "options" => {
                            if f_options.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            f_options = Some(
                                map.next_value::<crate::types::search::PhraseSuggestOptions>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PhraseSuggest {
                    length: f_length.ok_or_else(|| serde::de::Error::missing_field("length"))?,
                    offset: f_offset.ok_or_else(|| serde::de::Error::missing_field("offset"))?,
                    text: f_text.ok_or_else(|| serde::de::Error::missing_field("text"))?,
                    options: f_options.ok_or_else(|| serde::de::Error::missing_field("options"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PhraseSuggest {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("length", &self.length)?;
        map.serialize_entry("offset", &self.offset)?;
        map.serialize_entry("text", &self.text)?;
        map.serialize_entry("options", &self.options)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PhraseSuggestCollate {
    pub params: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub prune: Option<bool>,
    pub query: crate::types::search::PhraseSuggestCollateQuery,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PhraseSuggestCollate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PhraseSuggestCollate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PhraseSuggestCollate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_params: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_params = false;
                let mut f_prune: Option<bool> = None;
                let mut seen_prune = false;
                let mut f_query: Option<crate::types::search::PhraseSuggestCollateQuery> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "params" => {
                            if seen_params {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            seen_params = true;
                            f_params = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "prune" => {
                            if seen_prune {
                                return Err(serde::de::Error::duplicate_field("prune"));
                            }
                            seen_prune = true;
                            f_prune = map.next_value::<Option<bool>>()?;
                        }
                        "query" => {
                            if f_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            f_query = Some(
                                map.next_value::<crate::types::search::PhraseSuggestCollateQuery>(
                                )?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PhraseSuggestCollate {
                    params: f_params,
                    prune: f_prune,
                    query: f_query.ok_or_else(|| serde::de::Error::missing_field("query"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PhraseSuggestCollate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.params {
            map.serialize_entry("params", v)?;
        }
        if let Some(v) = &self.prune {
            map.serialize_entry("prune", v)?;
        }
        map.serialize_entry("query", &self.query)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PhraseSuggestCollateQuery {
    pub id: Option<String>,
    pub source: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PhraseSuggestCollateQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PhraseSuggestCollateQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PhraseSuggestCollateQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_id: Option<String> = None;
                let mut seen_id = false;
                let mut f_source: Option<String> = None;
                let mut seen_source = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<String>>()?;
                        }
                        "source" => {
                            if seen_source {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            seen_source = true;
                            f_source = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PhraseSuggestCollateQuery {
                    id: f_id,
                    source: f_source,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PhraseSuggestCollateQuery {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.id {
            map.serialize_entry("id", v)?;
        }
        if let Some(v) = &self.source {
            map.serialize_entry("source", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PhraseSuggestHighlight {
    pub post_tag: String,
    pub pre_tag: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PhraseSuggestHighlight {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PhraseSuggestHighlight;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PhraseSuggestHighlight object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_post_tag: Option<String> = None;
                let mut f_pre_tag: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "post_tag" => {
                            if f_post_tag.is_some() {
                                return Err(serde::de::Error::duplicate_field("post_tag"));
                            }
                            f_post_tag = Some(map.next_value::<String>()?);
                        }
                        "pre_tag" => {
                            if f_pre_tag.is_some() {
                                return Err(serde::de::Error::duplicate_field("pre_tag"));
                            }
                            f_pre_tag = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PhraseSuggestHighlight {
                    post_tag: f_post_tag
                        .ok_or_else(|| serde::de::Error::missing_field("post_tag"))?,
                    pre_tag: f_pre_tag.ok_or_else(|| serde::de::Error::missing_field("pre_tag"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PhraseSuggestHighlight {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("post_tag", &self.post_tag)?;
        map.serialize_entry("pre_tag", &self.pre_tag)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PhraseSuggestOption {
    pub collate_match: Option<bool>,
    pub highlighted: Option<String>,
    pub score: f64,
    pub text: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PhraseSuggestOption {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PhraseSuggestOption;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PhraseSuggestOption object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_collate_match: Option<bool> = None;
                let mut seen_collate_match = false;
                let mut f_highlighted: Option<String> = None;
                let mut seen_highlighted = false;
                let mut f_score: Option<f64> = None;
                let mut f_text: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "collate_match" => {
                            if seen_collate_match {
                                return Err(serde::de::Error::duplicate_field("collate_match"));
                            }
                            seen_collate_match = true;
                            f_collate_match = map.next_value::<Option<bool>>()?;
                        }
                        "highlighted" => {
                            if seen_highlighted {
                                return Err(serde::de::Error::duplicate_field("highlighted"));
                            }
                            seen_highlighted = true;
                            f_highlighted = map.next_value::<Option<String>>()?;
                        }
                        "score" => {
                            if f_score.is_some() {
                                return Err(serde::de::Error::duplicate_field("score"));
                            }
                            f_score = Some(map.next_value::<f64>()?);
                        }
                        "text" => {
                            if f_text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            f_text = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PhraseSuggestOption {
                    collate_match: f_collate_match,
                    highlighted: f_highlighted,
                    score: f_score.ok_or_else(|| serde::de::Error::missing_field("score"))?,
                    text: f_text.ok_or_else(|| serde::de::Error::missing_field("text"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PhraseSuggestOption {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.collate_match {
            map.serialize_entry("collate_match", v)?;
        }
        if let Some(v) = &self.highlighted {
            map.serialize_entry("highlighted", v)?;
        }
        map.serialize_entry("score", &self.score)?;
        map.serialize_entry("text", &self.text)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum PhraseSuggestOptions {
    PhraseSuggestOption(crate::types::search::PhraseSuggestOption),
    Array(Vec<crate::types::search::PhraseSuggestOption>),
}
impl<'de> serde::Deserialize<'de> for PhraseSuggestOptions {
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
            Some(b'{') => serde_json::from_str(text).map(PhraseSuggestOptions::PhraseSuggestOption),
            Some(b'[') => serde_json::from_str(text).map(PhraseSuggestOptions::Array),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a PhraseSuggestOptions value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct PhraseSuggester {
    pub analyzer: Option<String>,
    pub field: String,
    pub size: Option<i32>,
    pub collate: Option<crate::types::search::PhraseSuggestCollate>,
    pub confidence: Option<f64>,
    pub direct_generator: Option<Vec<crate::types::search::DirectGenerator>>,
    pub force_unigrams: Option<bool>,
    pub gram_size: Option<i32>,
    pub highlight: Option<crate::types::search::PhraseSuggestHighlight>,
    pub max_errors: Option<f64>,
    pub real_word_error_likelihood: Option<f64>,
    pub separator: Option<String>,
    pub shard_size: Option<i32>,
    pub smoothing: Option<crate::types::search::SmoothingModel>,
    pub text: Option<String>,
    pub token_limit: Option<i32>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PhraseSuggester {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PhraseSuggester;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PhraseSuggester object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_analyzer: Option<String> = None;
                let mut seen_analyzer = false;
                let mut f_field: Option<String> = None;
                let mut f_size: Option<i32> = None;
                let mut seen_size = false;
                let mut f_collate: Option<crate::types::search::PhraseSuggestCollate> = None;
                let mut seen_collate = false;
                let mut f_confidence: Option<f64> = None;
                let mut seen_confidence = false;
                let mut f_direct_generator: Option<Vec<crate::types::search::DirectGenerator>> =
                    None;
                let mut seen_direct_generator = false;
                let mut f_force_unigrams: Option<bool> = None;
                let mut seen_force_unigrams = false;
                let mut f_gram_size: Option<i32> = None;
                let mut seen_gram_size = false;
                let mut f_highlight: Option<crate::types::search::PhraseSuggestHighlight> = None;
                let mut seen_highlight = false;
                let mut f_max_errors: Option<f64> = None;
                let mut seen_max_errors = false;
                let mut f_real_word_error_likelihood: Option<f64> = None;
                let mut seen_real_word_error_likelihood = false;
                let mut f_separator: Option<String> = None;
                let mut seen_separator = false;
                let mut f_shard_size: Option<i32> = None;
                let mut seen_shard_size = false;
                let mut f_smoothing: Option<crate::types::search::SmoothingModel> = None;
                let mut seen_smoothing = false;
                let mut f_text: Option<String> = None;
                let mut seen_text = false;
                let mut f_token_limit: Option<i32> = None;
                let mut seen_token_limit = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "analyzer" => {
                            if seen_analyzer {
                                return Err(serde::de::Error::duplicate_field("analyzer"));
                            }
                            seen_analyzer = true;
                            f_analyzer = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<String>()?);
                        }
                        "size" => {
                            if seen_size {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            seen_size = true;
                            f_size = map.next_value::<Option<i32>>()?;
                        }
                        "collate" => {
                            if seen_collate {
                                return Err(serde::de::Error::duplicate_field("collate"));
                            }
                            seen_collate = true;
                            f_collate = map
                                .next_value::<Option<crate::types::search::PhraseSuggestCollate>>(
                                )?;
                        }
                        "confidence" => {
                            if seen_confidence {
                                return Err(serde::de::Error::duplicate_field("confidence"));
                            }
                            seen_confidence = true;
                            f_confidence = map.next_value::<Option<f64>>()?;
                        }
                        "direct_generator" => {
                            if seen_direct_generator {
                                return Err(serde::de::Error::duplicate_field("direct_generator"));
                            }
                            seen_direct_generator = true;
                            f_direct_generator = map
                                .next_value::<Option<Vec<crate::types::search::DirectGenerator>>>(
                                )?;
                        }
                        "force_unigrams" => {
                            if seen_force_unigrams {
                                return Err(serde::de::Error::duplicate_field("force_unigrams"));
                            }
                            seen_force_unigrams = true;
                            f_force_unigrams = map.next_value::<Option<bool>>()?;
                        }
                        "gram_size" => {
                            if seen_gram_size {
                                return Err(serde::de::Error::duplicate_field("gram_size"));
                            }
                            seen_gram_size = true;
                            f_gram_size = map.next_value::<Option<i32>>()?;
                        }
                        "highlight" => {
                            if seen_highlight {
                                return Err(serde::de::Error::duplicate_field("highlight"));
                            }
                            seen_highlight = true;
                            f_highlight = map
                                .next_value::<Option<crate::types::search::PhraseSuggestHighlight>>(
                                )?;
                        }
                        "max_errors" => {
                            if seen_max_errors {
                                return Err(serde::de::Error::duplicate_field("max_errors"));
                            }
                            seen_max_errors = true;
                            f_max_errors = map.next_value::<Option<f64>>()?;
                        }
                        "real_word_error_likelihood" => {
                            if seen_real_word_error_likelihood {
                                return Err(serde::de::Error::duplicate_field(
                                    "real_word_error_likelihood",
                                ));
                            }
                            seen_real_word_error_likelihood = true;
                            f_real_word_error_likelihood = map.next_value::<Option<f64>>()?;
                        }
                        "separator" => {
                            if seen_separator {
                                return Err(serde::de::Error::duplicate_field("separator"));
                            }
                            seen_separator = true;
                            f_separator = map.next_value::<Option<String>>()?;
                        }
                        "shard_size" => {
                            if seen_shard_size {
                                return Err(serde::de::Error::duplicate_field("shard_size"));
                            }
                            seen_shard_size = true;
                            f_shard_size = map.next_value::<Option<i32>>()?;
                        }
                        "smoothing" => {
                            if seen_smoothing {
                                return Err(serde::de::Error::duplicate_field("smoothing"));
                            }
                            seen_smoothing = true;
                            f_smoothing =
                                map.next_value::<Option<crate::types::search::SmoothingModel>>()?;
                        }
                        "text" => {
                            if seen_text {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            seen_text = true;
                            f_text = map.next_value::<Option<String>>()?;
                        }
                        "token_limit" => {
                            if seen_token_limit {
                                return Err(serde::de::Error::duplicate_field("token_limit"));
                            }
                            seen_token_limit = true;
                            f_token_limit = map.next_value::<Option<i32>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PhraseSuggester {
                    analyzer: f_analyzer,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    size: f_size,
                    collate: f_collate,
                    confidence: f_confidence,
                    direct_generator: f_direct_generator,
                    force_unigrams: f_force_unigrams,
                    gram_size: f_gram_size,
                    highlight: f_highlight,
                    max_errors: f_max_errors,
                    real_word_error_likelihood: f_real_word_error_likelihood,
                    separator: f_separator,
                    shard_size: f_shard_size,
                    smoothing: f_smoothing,
                    text: f_text,
                    token_limit: f_token_limit,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PhraseSuggester {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.analyzer {
            map.serialize_entry("analyzer", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.size {
            map.serialize_entry("size", v)?;
        }
        if let Some(v) = &self.collate {
            map.serialize_entry("collate", v)?;
        }
        if let Some(v) = &self.confidence {
            map.serialize_entry("confidence", v)?;
        }
        if let Some(v) = &self.direct_generator {
            map.serialize_entry("direct_generator", v)?;
        }
        if let Some(v) = &self.force_unigrams {
            map.serialize_entry("force_unigrams", v)?;
        }
        if let Some(v) = &self.gram_size {
            map.serialize_entry("gram_size", v)?;
        }
        if let Some(v) = &self.highlight {
            map.serialize_entry("highlight", v)?;
        }
        if let Some(v) = &self.max_errors {
            map.serialize_entry("max_errors", v)?;
        }
        if let Some(v) = &self.real_word_error_likelihood {
            map.serialize_entry("real_word_error_likelihood", v)?;
        }
        if let Some(v) = &self.separator {
            map.serialize_entry("separator", v)?;
        }
        if let Some(v) = &self.shard_size {
            map.serialize_entry("shard_size", v)?;
        }
        if let Some(v) = &self.smoothing {
            map.serialize_entry("smoothing", v)?;
        }
        if let Some(v) = &self.text {
            map.serialize_entry("text", v)?;
        }
        if let Some(v) = &self.token_limit {
            map.serialize_entry("token_limit", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PointInTimeReference {
    pub id: crate::types::common::Id,
    pub keep_alive: Option<crate::types::common::Duration>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PointInTimeReference {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PointInTimeReference;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PointInTimeReference object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut f_keep_alive: Option<crate::types::common::Duration> = None;
                let mut seen_keep_alive = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "id" => {
                            if f_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            f_id = Some(map.next_value::<crate::types::common::Id>()?);
                        }
                        "keep_alive" => {
                            if seen_keep_alive {
                                return Err(serde::de::Error::duplicate_field("keep_alive"));
                            }
                            seen_keep_alive = true;
                            f_keep_alive =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PointInTimeReference {
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("id"))?,
                    keep_alive: f_keep_alive,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PointInTimeReference {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("id", &self.id)?;
        if let Some(v) = &self.keep_alive {
            map.serialize_entry("keep_alive", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ProcessorExecutionDetail {
    pub duration_millis: Option<i64>,
    pub error: Option<String>,
    pub input_data: Option<Box<serde_json::value::RawValue>>,
    pub output_data: Option<Box<serde_json::value::RawValue>>,
    pub processor_name: Option<String>,
    pub status: Option<String>,
    pub tag: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ProcessorExecutionDetail {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ProcessorExecutionDetail;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ProcessorExecutionDetail object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_duration_millis: Option<i64> = None;
                let mut seen_duration_millis = false;
                let mut f_error: Option<String> = None;
                let mut seen_error = false;
                let mut f_input_data: Option<Box<serde_json::value::RawValue>> = None;
                let mut seen_input_data = false;
                let mut f_output_data: Option<Box<serde_json::value::RawValue>> = None;
                let mut seen_output_data = false;
                let mut f_processor_name: Option<String> = None;
                let mut seen_processor_name = false;
                let mut f_status: Option<String> = None;
                let mut seen_status = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "duration_millis" => {
                            if seen_duration_millis {
                                return Err(serde::de::Error::duplicate_field("duration_millis"));
                            }
                            seen_duration_millis = true;
                            f_duration_millis = map.next_value::<Option<i64>>()?;
                        }
                        "error" => {
                            if seen_error {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            seen_error = true;
                            f_error = map.next_value::<Option<String>>()?;
                        }
                        "input_data" => {
                            if seen_input_data {
                                return Err(serde::de::Error::duplicate_field("input_data"));
                            }
                            seen_input_data = true;
                            f_input_data =
                                map.next_value::<Option<Box<serde_json::value::RawValue>>>()?;
                        }
                        "output_data" => {
                            if seen_output_data {
                                return Err(serde::de::Error::duplicate_field("output_data"));
                            }
                            seen_output_data = true;
                            f_output_data =
                                map.next_value::<Option<Box<serde_json::value::RawValue>>>()?;
                        }
                        "processor_name" => {
                            if seen_processor_name {
                                return Err(serde::de::Error::duplicate_field("processor_name"));
                            }
                            seen_processor_name = true;
                            f_processor_name = map.next_value::<Option<String>>()?;
                        }
                        "status" => {
                            if seen_status {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            seen_status = true;
                            f_status = map.next_value::<Option<String>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ProcessorExecutionDetail {
                    duration_millis: f_duration_millis,
                    error: f_error,
                    input_data: f_input_data,
                    output_data: f_output_data,
                    processor_name: f_processor_name,
                    status: f_status,
                    tag: f_tag,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ProcessorExecutionDetail {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.duration_millis {
            map.serialize_entry("duration_millis", v)?;
        }
        if let Some(v) = &self.error {
            map.serialize_entry("error", v)?;
        }
        if let Some(v) = &self.input_data {
            map.serialize_entry("input_data", v)?;
        }
        if let Some(v) = &self.output_data {
            map.serialize_entry("output_data", v)?;
        }
        if let Some(v) = &self.processor_name {
            map.serialize_entry("processor_name", v)?;
        }
        if let Some(v) = &self.status {
            map.serialize_entry("status", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Profile {
    pub shards: Vec<crate::types::search::ShardProfile>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Profile {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Profile;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Profile object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_shards: Option<Vec<crate::types::search::ShardProfile>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "shards" => {
                            if f_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("shards"));
                            }
                            f_shards =
                                Some(map.next_value::<Vec<crate::types::search::ShardProfile>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Profile {
                    shards: f_shards.ok_or_else(|| serde::de::Error::missing_field("shards"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Profile {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("shards", &self.shards)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct QueryBreakdown {
    pub advance: i64,
    pub advance_count: i64,
    pub build_scorer: i64,
    pub build_scorer_count: i64,
    pub compute_max_score: i64,
    pub compute_max_score_count: i64,
    pub create_weight: i64,
    pub create_weight_count: i64,
    pub r#match: i64,
    pub match_count: i64,
    pub next_doc: i64,
    pub next_doc_count: i64,
    pub score: i64,
    pub score_count: i64,
    pub set_min_competitive_score: i64,
    pub set_min_competitive_score_count: i64,
    pub shallow_advance: i64,
    pub shallow_advance_count: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for QueryBreakdown {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = QueryBreakdown;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a QueryBreakdown object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_advance: Option<i64> = None;
                let mut f_advance_count: Option<i64> = None;
                let mut f_build_scorer: Option<i64> = None;
                let mut f_build_scorer_count: Option<i64> = None;
                let mut f_compute_max_score: Option<i64> = None;
                let mut f_compute_max_score_count: Option<i64> = None;
                let mut f_create_weight: Option<i64> = None;
                let mut f_create_weight_count: Option<i64> = None;
                let mut f_match: Option<i64> = None;
                let mut f_match_count: Option<i64> = None;
                let mut f_next_doc: Option<i64> = None;
                let mut f_next_doc_count: Option<i64> = None;
                let mut f_score: Option<i64> = None;
                let mut f_score_count: Option<i64> = None;
                let mut f_set_min_competitive_score: Option<i64> = None;
                let mut f_set_min_competitive_score_count: Option<i64> = None;
                let mut f_shallow_advance: Option<i64> = None;
                let mut f_shallow_advance_count: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "advance" => {
                            if f_advance.is_some() {
                                return Err(serde::de::Error::duplicate_field("advance"));
                            }
                            f_advance = Some(map.next_value::<i64>()?);
                        }
                        "advance_count" => {
                            if f_advance_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("advance_count"));
                            }
                            f_advance_count = Some(map.next_value::<i64>()?);
                        }
                        "build_scorer" => {
                            if f_build_scorer.is_some() {
                                return Err(serde::de::Error::duplicate_field("build_scorer"));
                            }
                            f_build_scorer = Some(map.next_value::<i64>()?);
                        }
                        "build_scorer_count" => {
                            if f_build_scorer_count.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "build_scorer_count",
                                ));
                            }
                            f_build_scorer_count = Some(map.next_value::<i64>()?);
                        }
                        "compute_max_score" => {
                            if f_compute_max_score.is_some() {
                                return Err(serde::de::Error::duplicate_field("compute_max_score"));
                            }
                            f_compute_max_score = Some(map.next_value::<i64>()?);
                        }
                        "compute_max_score_count" => {
                            if f_compute_max_score_count.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "compute_max_score_count",
                                ));
                            }
                            f_compute_max_score_count = Some(map.next_value::<i64>()?);
                        }
                        "create_weight" => {
                            if f_create_weight.is_some() {
                                return Err(serde::de::Error::duplicate_field("create_weight"));
                            }
                            f_create_weight = Some(map.next_value::<i64>()?);
                        }
                        "create_weight_count" => {
                            if f_create_weight_count.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "create_weight_count",
                                ));
                            }
                            f_create_weight_count = Some(map.next_value::<i64>()?);
                        }
                        "match" => {
                            if f_match.is_some() {
                                return Err(serde::de::Error::duplicate_field("match"));
                            }
                            f_match = Some(map.next_value::<i64>()?);
                        }
                        "match_count" => {
                            if f_match_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("match_count"));
                            }
                            f_match_count = Some(map.next_value::<i64>()?);
                        }
                        "next_doc" => {
                            if f_next_doc.is_some() {
                                return Err(serde::de::Error::duplicate_field("next_doc"));
                            }
                            f_next_doc = Some(map.next_value::<i64>()?);
                        }
                        "next_doc_count" => {
                            if f_next_doc_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("next_doc_count"));
                            }
                            f_next_doc_count = Some(map.next_value::<i64>()?);
                        }
                        "score" => {
                            if f_score.is_some() {
                                return Err(serde::de::Error::duplicate_field("score"));
                            }
                            f_score = Some(map.next_value::<i64>()?);
                        }
                        "score_count" => {
                            if f_score_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("score_count"));
                            }
                            f_score_count = Some(map.next_value::<i64>()?);
                        }
                        "set_min_competitive_score" => {
                            if f_set_min_competitive_score.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "set_min_competitive_score",
                                ));
                            }
                            f_set_min_competitive_score = Some(map.next_value::<i64>()?);
                        }
                        "set_min_competitive_score_count" => {
                            if f_set_min_competitive_score_count.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "set_min_competitive_score_count",
                                ));
                            }
                            f_set_min_competitive_score_count = Some(map.next_value::<i64>()?);
                        }
                        "shallow_advance" => {
                            if f_shallow_advance.is_some() {
                                return Err(serde::de::Error::duplicate_field("shallow_advance"));
                            }
                            f_shallow_advance = Some(map.next_value::<i64>()?);
                        }
                        "shallow_advance_count" => {
                            if f_shallow_advance_count.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "shallow_advance_count",
                                ));
                            }
                            f_shallow_advance_count = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(QueryBreakdown {
                    advance: f_advance.ok_or_else(|| serde::de::Error::missing_field("advance"))?,
                    advance_count: f_advance_count
                        .ok_or_else(|| serde::de::Error::missing_field("advance_count"))?,
                    build_scorer: f_build_scorer
                        .ok_or_else(|| serde::de::Error::missing_field("build_scorer"))?,
                    build_scorer_count: f_build_scorer_count
                        .ok_or_else(|| serde::de::Error::missing_field("build_scorer_count"))?,
                    compute_max_score: f_compute_max_score
                        .ok_or_else(|| serde::de::Error::missing_field("compute_max_score"))?,
                    compute_max_score_count: f_compute_max_score_count.ok_or_else(|| {
                        serde::de::Error::missing_field("compute_max_score_count")
                    })?,
                    create_weight: f_create_weight
                        .ok_or_else(|| serde::de::Error::missing_field("create_weight"))?,
                    create_weight_count: f_create_weight_count
                        .ok_or_else(|| serde::de::Error::missing_field("create_weight_count"))?,
                    r#match: f_match.ok_or_else(|| serde::de::Error::missing_field("match"))?,
                    match_count: f_match_count
                        .ok_or_else(|| serde::de::Error::missing_field("match_count"))?,
                    next_doc: f_next_doc
                        .ok_or_else(|| serde::de::Error::missing_field("next_doc"))?,
                    next_doc_count: f_next_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("next_doc_count"))?,
                    score: f_score.ok_or_else(|| serde::de::Error::missing_field("score"))?,
                    score_count: f_score_count
                        .ok_or_else(|| serde::de::Error::missing_field("score_count"))?,
                    set_min_competitive_score: f_set_min_competitive_score.ok_or_else(|| {
                        serde::de::Error::missing_field("set_min_competitive_score")
                    })?,
                    set_min_competitive_score_count: f_set_min_competitive_score_count.ok_or_else(
                        || serde::de::Error::missing_field("set_min_competitive_score_count"),
                    )?,
                    shallow_advance: f_shallow_advance
                        .ok_or_else(|| serde::de::Error::missing_field("shallow_advance"))?,
                    shallow_advance_count: f_shallow_advance_count
                        .ok_or_else(|| serde::de::Error::missing_field("shallow_advance_count"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for QueryBreakdown {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("advance", &self.advance)?;
        map.serialize_entry("advance_count", &self.advance_count)?;
        map.serialize_entry("build_scorer", &self.build_scorer)?;
        map.serialize_entry("build_scorer_count", &self.build_scorer_count)?;
        map.serialize_entry("compute_max_score", &self.compute_max_score)?;
        map.serialize_entry("compute_max_score_count", &self.compute_max_score_count)?;
        map.serialize_entry("create_weight", &self.create_weight)?;
        map.serialize_entry("create_weight_count", &self.create_weight_count)?;
        map.serialize_entry("match", &self.r#match)?;
        map.serialize_entry("match_count", &self.match_count)?;
        map.serialize_entry("next_doc", &self.next_doc)?;
        map.serialize_entry("next_doc_count", &self.next_doc_count)?;
        map.serialize_entry("score", &self.score)?;
        map.serialize_entry("score_count", &self.score_count)?;
        map.serialize_entry("set_min_competitive_score", &self.set_min_competitive_score)?;
        map.serialize_entry(
            "set_min_competitive_score_count",
            &self.set_min_competitive_score_count,
        )?;
        map.serialize_entry("shallow_advance", &self.shallow_advance)?;
        map.serialize_entry("shallow_advance_count", &self.shallow_advance_count)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct QueryProfile {
    pub breakdown: crate::types::search::QueryBreakdown,
    pub children: Option<Vec<crate::types::search::QueryProfile>>,
    pub description: String,
    pub time_in_nanos: crate::types::common::DurationValueUnitNanos,
    pub r#type: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for QueryProfile {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = QueryProfile;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a QueryProfile object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_breakdown: Option<crate::types::search::QueryBreakdown> = None;
                let mut f_children: Option<Vec<crate::types::search::QueryProfile>> = None;
                let mut seen_children = false;
                let mut f_description: Option<String> = None;
                let mut f_time_in_nanos: Option<crate::types::common::DurationValueUnitNanos> =
                    None;
                let mut f_type: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "breakdown" => {
                            if f_breakdown.is_some() {
                                return Err(serde::de::Error::duplicate_field("breakdown"));
                            }
                            f_breakdown =
                                Some(map.next_value::<crate::types::search::QueryBreakdown>()?);
                        }
                        "children" => {
                            if seen_children {
                                return Err(serde::de::Error::duplicate_field("children"));
                            }
                            seen_children = true;
                            f_children = map
                                .next_value::<Option<Vec<crate::types::search::QueryProfile>>>()?;
                        }
                        "description" => {
                            if f_description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            f_description = Some(map.next_value::<String>()?);
                        }
                        "time_in_nanos" => {
                            if f_time_in_nanos.is_some() {
                                return Err(serde::de::Error::duplicate_field("time_in_nanos"));
                            }
                            f_time_in_nanos = Some(
                                map.next_value::<crate::types::common::DurationValueUnitNanos>()?,
                            );
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(QueryProfile {
                    breakdown: f_breakdown
                        .ok_or_else(|| serde::de::Error::missing_field("breakdown"))?,
                    children: f_children,
                    description: f_description
                        .ok_or_else(|| serde::de::Error::missing_field("description"))?,
                    time_in_nanos: f_time_in_nanos
                        .ok_or_else(|| serde::de::Error::missing_field("time_in_nanos"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for QueryProfile {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("breakdown", &self.breakdown)?;
        if let Some(v) = &self.children {
            map.serialize_entry("children", v)?;
        }
        map.serialize_entry("description", &self.description)?;
        map.serialize_entry("time_in_nanos", &self.time_in_nanos)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SearchProfile {
    pub collector: Vec<crate::types::search::Collector>,
    pub query: Vec<crate::types::search::QueryProfile>,
    pub rewrite_time: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SearchProfile {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SearchProfile;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchProfile object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_collector: Option<Vec<crate::types::search::Collector>> = None;
                let mut f_query: Option<Vec<crate::types::search::QueryProfile>> = None;
                let mut f_rewrite_time: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "collector" => {
                            if f_collector.is_some() {
                                return Err(serde::de::Error::duplicate_field("collector"));
                            }
                            f_collector =
                                Some(map.next_value::<Vec<crate::types::search::Collector>>()?);
                        }
                        "query" => {
                            if f_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            f_query =
                                Some(map.next_value::<Vec<crate::types::search::QueryProfile>>()?);
                        }
                        "rewrite_time" => {
                            if f_rewrite_time.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewrite_time"));
                            }
                            f_rewrite_time = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SearchProfile {
                    collector: f_collector
                        .ok_or_else(|| serde::de::Error::missing_field("collector"))?,
                    query: f_query.ok_or_else(|| serde::de::Error::missing_field("query"))?,
                    rewrite_time: f_rewrite_time
                        .ok_or_else(|| serde::de::Error::missing_field("rewrite_time"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SearchProfile {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("collector", &self.collector)?;
        map.serialize_entry("query", &self.query)?;
        map.serialize_entry("rewrite_time", &self.rewrite_time)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SearchResponse {
    pub clusters: Option<crate::types::common::ClusterStatistics>,
    pub scroll_id: Option<crate::types::common::ScrollId>,
    pub shards: crate::types::common::ShardStatistics,
    pub aggregations:
        Option<std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>>,
    pub hits: crate::types::search::SearchResponseHits,
    pub num_reduce_phases: Option<i32>,
    pub phase_took: Option<crate::types::common::PhaseTook>,
    pub pit_id: Option<crate::types::common::Id>,
    pub processor_results: Option<Vec<crate::types::search::ProcessorExecutionDetail>>,
    pub profile: Option<crate::types::search::Profile>,
    pub suggest: Option<std::collections::BTreeMap<String, Vec<crate::types::search::Suggest>>>,
    pub terminated_early: Option<bool>,
    pub timed_out: bool,
    pub took: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SearchResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SearchResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_clusters: Option<crate::types::common::ClusterStatistics> = None;
                let mut seen_clusters = false;
                let mut f_scroll_id: Option<crate::types::common::ScrollId> = None;
                let mut seen_scroll_id = false;
                let mut f_shards: Option<crate::types::common::ShardStatistics> = None;
                let mut f_aggregations: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::common_aggregations::Aggregate,
                    >,
                > = None;
                let mut seen_aggregations = false;
                let mut f_hits: Option<crate::types::search::SearchResponseHits> = None;
                let mut f_num_reduce_phases: Option<i32> = None;
                let mut seen_num_reduce_phases = false;
                let mut f_phase_took: Option<crate::types::common::PhaseTook> = None;
                let mut seen_phase_took = false;
                let mut f_pit_id: Option<crate::types::common::Id> = None;
                let mut seen_pit_id = false;
                let mut f_processor_results: Option<
                    Vec<crate::types::search::ProcessorExecutionDetail>,
                > = None;
                let mut seen_processor_results = false;
                let mut f_profile: Option<crate::types::search::Profile> = None;
                let mut seen_profile = false;
                let mut f_suggest: Option<
                    std::collections::BTreeMap<String, Vec<crate::types::search::Suggest>>,
                > = None;
                let mut seen_suggest = false;
                let mut f_terminated_early: Option<bool> = None;
                let mut seen_terminated_early = false;
                let mut f_timed_out: Option<bool> = None;
                let mut f_took: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_clusters" => {
                            if seen_clusters {
                                return Err(serde::de::Error::duplicate_field("_clusters"));
                            }
                            seen_clusters = true;
                            f_clusters = map
                                .next_value::<Option<crate::types::common::ClusterStatistics>>()?;
                        }
                        "_scroll_id" => {
                            if seen_scroll_id {
                                return Err(serde::de::Error::duplicate_field("_scroll_id"));
                            }
                            seen_scroll_id = true;
                            f_scroll_id =
                                map.next_value::<Option<crate::types::common::ScrollId>>()?;
                        }
                        "_shards" => {
                            if f_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("_shards"));
                            }
                            f_shards =
                                Some(map.next_value::<crate::types::common::ShardStatistics>()?);
                        }
                        "aggregations" => {
                            if seen_aggregations {
                                return Err(serde::de::Error::duplicate_field("aggregations"));
                            }
                            seen_aggregations = true;
                            f_aggregations = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_aggregations::Aggregate,
                                >,
                            >>()?;
                        }
                        "hits" => {
                            if f_hits.is_some() {
                                return Err(serde::de::Error::duplicate_field("hits"));
                            }
                            f_hits =
                                Some(map.next_value::<crate::types::search::SearchResponseHits>()?);
                        }
                        "num_reduce_phases" => {
                            if seen_num_reduce_phases {
                                return Err(serde::de::Error::duplicate_field("num_reduce_phases"));
                            }
                            seen_num_reduce_phases = true;
                            f_num_reduce_phases = map.next_value::<Option<i32>>()?;
                        }
                        "phase_took" => {
                            if seen_phase_took {
                                return Err(serde::de::Error::duplicate_field("phase_took"));
                            }
                            seen_phase_took = true;
                            f_phase_took =
                                map.next_value::<Option<crate::types::common::PhaseTook>>()?;
                        }
                        "pit_id" => {
                            if seen_pit_id {
                                return Err(serde::de::Error::duplicate_field("pit_id"));
                            }
                            seen_pit_id = true;
                            f_pit_id = map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        "processor_results" => {
                            if seen_processor_results {
                                return Err(serde::de::Error::duplicate_field("processor_results"));
                            }
                            seen_processor_results = true;
                            f_processor_results = map.next_value::<Option<Vec<crate::types::search::ProcessorExecutionDetail>>>()?;
                        }
                        "profile" => {
                            if seen_profile {
                                return Err(serde::de::Error::duplicate_field("profile"));
                            }
                            seen_profile = true;
                            f_profile =
                                map.next_value::<Option<crate::types::search::Profile>>()?;
                        }
                        "suggest" => {
                            if seen_suggest {
                                return Err(serde::de::Error::duplicate_field("suggest"));
                            }
                            seen_suggest = true;
                            f_suggest = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Vec<crate::types::search::Suggest>,
                                >,
                            >>()?;
                        }
                        "terminated_early" => {
                            if seen_terminated_early {
                                return Err(serde::de::Error::duplicate_field("terminated_early"));
                            }
                            seen_terminated_early = true;
                            f_terminated_early = map.next_value::<Option<bool>>()?;
                        }
                        "timed_out" => {
                            if f_timed_out.is_some() {
                                return Err(serde::de::Error::duplicate_field("timed_out"));
                            }
                            f_timed_out = Some(map.next_value::<bool>()?);
                        }
                        "took" => {
                            if f_took.is_some() {
                                return Err(serde::de::Error::duplicate_field("took"));
                            }
                            f_took = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SearchResponse {
                    clusters: f_clusters,
                    scroll_id: f_scroll_id,
                    shards: f_shards.ok_or_else(|| serde::de::Error::missing_field("_shards"))?,
                    aggregations: f_aggregations,
                    hits: f_hits.ok_or_else(|| serde::de::Error::missing_field("hits"))?,
                    num_reduce_phases: f_num_reduce_phases,
                    phase_took: f_phase_took,
                    pit_id: f_pit_id,
                    processor_results: f_processor_results,
                    profile: f_profile,
                    suggest: f_suggest,
                    terminated_early: f_terminated_early,
                    timed_out: f_timed_out
                        .ok_or_else(|| serde::de::Error::missing_field("timed_out"))?,
                    took: f_took.ok_or_else(|| serde::de::Error::missing_field("took"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SearchResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.clusters {
            map.serialize_entry("_clusters", v)?;
        }
        if let Some(v) = &self.scroll_id {
            map.serialize_entry("_scroll_id", v)?;
        }
        map.serialize_entry("_shards", &self.shards)?;
        if let Some(v) = &self.aggregations {
            map.serialize_entry("aggregations", v)?;
        }
        map.serialize_entry("hits", &self.hits)?;
        if let Some(v) = &self.num_reduce_phases {
            map.serialize_entry("num_reduce_phases", v)?;
        }
        if let Some(v) = &self.phase_took {
            map.serialize_entry("phase_took", v)?;
        }
        if let Some(v) = &self.pit_id {
            map.serialize_entry("pit_id", v)?;
        }
        if let Some(v) = &self.processor_results {
            map.serialize_entry("processor_results", v)?;
        }
        if let Some(v) = &self.profile {
            map.serialize_entry("profile", v)?;
        }
        if let Some(v) = &self.suggest {
            map.serialize_entry("suggest", v)?;
        }
        if let Some(v) = &self.terminated_early {
            map.serialize_entry("terminated_early", v)?;
        }
        map.serialize_entry("timed_out", &self.timed_out)?;
        map.serialize_entry("took", &self.took)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SearchResponseHits {
    pub hits: Vec<crate::types::search::SearchResponseHitsHitsItem>,
    pub max_score: Option<f64>,
    #[doc = "The total number of hits, present only if `track_total_hits` is not set to `false` in the search request."]
    pub total: Option<crate::types::search::SearchResponseHitsTotal>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SearchResponseHits {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SearchResponseHits;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchResponseHits object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_hits: Option<Vec<crate::types::search::SearchResponseHitsHitsItem>> =
                    None;
                let mut f_max_score: Option<f64> = None;
                let mut seen_max_score = false;
                let mut f_total: Option<crate::types::search::SearchResponseHitsTotal> = None;
                let mut seen_total = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "hits" => {
                            if f_hits.is_some() {
                                return Err(serde::de::Error::duplicate_field("hits"));
                            }
                            f_hits = Some(map.next_value::<Vec<crate::types::search::SearchResponseHitsHitsItem>>()?);
                        }
                        "max_score" => {
                            if seen_max_score {
                                return Err(serde::de::Error::duplicate_field("max_score"));
                            }
                            seen_max_score = true;
                            f_max_score = map.next_value::<Option<f64>>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map.next_value::<Option<crate::types::search::SearchResponseHitsTotal>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SearchResponseHits {
                    hits: f_hits.ok_or_else(|| serde::de::Error::missing_field("hits"))?,
                    max_score: f_max_score,
                    total: f_total,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SearchResponseHits {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("hits", &self.hits)?;
        if let Some(v) = &self.max_score {
            map.serialize_entry("max_score", v)?;
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
impl SearchResponseHits {
    #[doc = "The total number of hits (`total`), or `None` when the request disabled counting (`track_total_hits: false`).

The number may be a lower bound: by default the count is only tracked accurately up to 10,000 hits. Use `total` and its [`is_accurate`](crate::types::TotalHitCount::is_accurate) when that matters."]
    pub fn total_value(&self) -> Option<i64> {
        self.total.as_ref().map(|total| total.value())
    }
}

#[derive(Debug, Clone)]
pub struct SearchResponseHitsHitsItem {
    pub explanation: Option<crate::types::explain::Explanation>,
    pub id: Option<crate::types::common::Id>,
    pub ignored: Option<Vec<String>>,
    pub index: Option<crate::types::common::IndexName>,
    pub nested: Option<crate::types::search::NestedIdentity>,
    pub node: Option<String>,
    pub primary_term: Option<i64>,
    pub routing: Option<String>,
    pub score: Option<f64>,
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    pub shard: Option<String>,
    pub source: Option<Box<serde_json::value::RawValue>>,
    pub r#type: Option<crate::types::common::Type>,
    pub version: Option<crate::types::common::VersionNumber>,
    pub fields: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub highlight: Option<std::collections::BTreeMap<String, Vec<String>>>,
    pub ignored_field_values: Option<std::collections::BTreeMap<String, Vec<String>>>,
    pub inner_hits:
        Option<std::collections::BTreeMap<String, crate::types::search::InnerHitsResult>>,
    #[doc = "The names of queries that matched the document. When `include_named_queries_score` is false (default), returns an array of query names. When true, returns an object mapping query names to their scores."]
    pub matched_queries: Option<crate::types::search::SearchResponseHitsHitsItemMatchedQueries>,
    pub sort: Option<crate::types::common::SortResults>,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SearchResponseHitsHitsItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SearchResponseHitsHitsItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchResponseHitsHitsItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_explanation: Option<crate::types::explain::Explanation> = None;
                let mut seen_explanation = false;
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut seen_id = false;
                let mut f_ignored: Option<Vec<String>> = None;
                let mut seen_ignored = false;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut seen_index = false;
                let mut f_nested: Option<crate::types::search::NestedIdentity> = None;
                let mut seen_nested = false;
                let mut f_node: Option<String> = None;
                let mut seen_node = false;
                let mut f_primary_term: Option<i64> = None;
                let mut seen_primary_term = false;
                let mut f_routing: Option<String> = None;
                let mut seen_routing = false;
                let mut f_score: Option<f64> = None;
                let mut seen_score = false;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut seen_seq_no = false;
                let mut f_shard: Option<String> = None;
                let mut seen_shard = false;
                let mut f_source: Option<Box<serde_json::value::RawValue>> = None;
                let mut seen_source = false;
                let mut f_type: Option<crate::types::common::Type> = None;
                let mut seen_type = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_fields = false;
                let mut f_highlight: Option<std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut seen_highlight = false;
                let mut f_ignored_field_values: Option<
                    std::collections::BTreeMap<String, Vec<String>>,
                > = None;
                let mut seen_ignored_field_values = false;
                let mut f_inner_hits: Option<
                    std::collections::BTreeMap<String, crate::types::search::InnerHitsResult>,
                > = None;
                let mut seen_inner_hits = false;
                let mut f_matched_queries: Option<
                    crate::types::search::SearchResponseHitsHitsItemMatchedQueries,
                > = None;
                let mut seen_matched_queries = false;
                let mut f_sort: Option<crate::types::common::SortResults> = None;
                let mut seen_sort = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_explanation" => {
                            if seen_explanation {
                                return Err(serde::de::Error::duplicate_field("_explanation"));
                            }
                            seen_explanation = true;
                            f_explanation =
                                map.next_value::<Option<crate::types::explain::Explanation>>()?;
                        }
                        "_id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("_id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        "_ignored" => {
                            if seen_ignored {
                                return Err(serde::de::Error::duplicate_field("_ignored"));
                            }
                            seen_ignored = true;
                            f_ignored = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "_index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("_index"));
                            }
                            seen_index = true;
                            f_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "_nested" => {
                            if seen_nested {
                                return Err(serde::de::Error::duplicate_field("_nested"));
                            }
                            seen_nested = true;
                            f_nested =
                                map.next_value::<Option<crate::types::search::NestedIdentity>>()?;
                        }
                        "_node" => {
                            if seen_node {
                                return Err(serde::de::Error::duplicate_field("_node"));
                            }
                            seen_node = true;
                            f_node = map.next_value::<Option<String>>()?;
                        }
                        "_primary_term" => {
                            if seen_primary_term {
                                return Err(serde::de::Error::duplicate_field("_primary_term"));
                            }
                            seen_primary_term = true;
                            f_primary_term = map.next_value::<Option<i64>>()?;
                        }
                        "_routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("_routing"));
                            }
                            seen_routing = true;
                            f_routing = map.next_value::<Option<String>>()?;
                        }
                        "_score" => {
                            if seen_score {
                                return Err(serde::de::Error::duplicate_field("_score"));
                            }
                            seen_score = true;
                            f_score = map.next_value::<Option<f64>>()?;
                        }
                        "_seq_no" => {
                            if seen_seq_no {
                                return Err(serde::de::Error::duplicate_field("_seq_no"));
                            }
                            seen_seq_no = true;
                            f_seq_no =
                                map.next_value::<Option<crate::types::common::SequenceNumber>>()?;
                        }
                        "_shard" => {
                            if seen_shard {
                                return Err(serde::de::Error::duplicate_field("_shard"));
                            }
                            seen_shard = true;
                            f_shard = map.next_value::<Option<String>>()?;
                        }
                        "_source" => {
                            if seen_source {
                                return Err(serde::de::Error::duplicate_field("_source"));
                            }
                            seen_source = true;
                            f_source =
                                map.next_value::<Option<Box<serde_json::value::RawValue>>>()?;
                        }
                        "_type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<crate::types::common::Type>>()?;
                        }
                        "_version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionNumber>>()?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "highlight" => {
                            if seen_highlight {
                                return Err(serde::de::Error::duplicate_field("highlight"));
                            }
                            seen_highlight = true;
                            f_highlight = map.next_value::<Option<std::collections::BTreeMap<String, Vec<String>>>>()?;
                        }
                        "ignored_field_values" => {
                            if seen_ignored_field_values {
                                return Err(serde::de::Error::duplicate_field(
                                    "ignored_field_values",
                                ));
                            }
                            seen_ignored_field_values = true;
                            f_ignored_field_values = map.next_value::<Option<std::collections::BTreeMap<String, Vec<String>>>>()?;
                        }
                        "inner_hits" => {
                            if seen_inner_hits {
                                return Err(serde::de::Error::duplicate_field("inner_hits"));
                            }
                            seen_inner_hits = true;
                            f_inner_hits = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::search::InnerHitsResult,
                                >,
                            >>()?;
                        }
                        "matched_queries" => {
                            if seen_matched_queries {
                                return Err(serde::de::Error::duplicate_field("matched_queries"));
                            }
                            seen_matched_queries = true;
                            f_matched_queries = map.next_value::<Option<
                                crate::types::search::SearchResponseHitsHitsItemMatchedQueries,
                            >>()?;
                        }
                        "sort" => {
                            if seen_sort {
                                return Err(serde::de::Error::duplicate_field("sort"));
                            }
                            seen_sort = true;
                            f_sort =
                                map.next_value::<Option<crate::types::common::SortResults>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SearchResponseHitsHitsItem {
                    explanation: f_explanation,
                    id: f_id,
                    ignored: f_ignored,
                    index: f_index,
                    nested: f_nested,
                    node: f_node,
                    primary_term: f_primary_term,
                    routing: f_routing,
                    score: f_score,
                    seq_no: f_seq_no,
                    shard: f_shard,
                    source: f_source,
                    r#type: f_type,
                    version: f_version,
                    fields: f_fields,
                    highlight: f_highlight,
                    ignored_field_values: f_ignored_field_values,
                    inner_hits: f_inner_hits,
                    matched_queries: f_matched_queries,
                    sort: f_sort,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SearchResponseHitsHitsItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.explanation {
            map.serialize_entry("_explanation", v)?;
        }
        if let Some(v) = &self.id {
            map.serialize_entry("_id", v)?;
        }
        if let Some(v) = &self.ignored {
            map.serialize_entry("_ignored", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("_index", v)?;
        }
        if let Some(v) = &self.nested {
            map.serialize_entry("_nested", v)?;
        }
        if let Some(v) = &self.node {
            map.serialize_entry("_node", v)?;
        }
        if let Some(v) = &self.primary_term {
            map.serialize_entry("_primary_term", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("_routing", v)?;
        }
        if let Some(v) = &self.score {
            map.serialize_entry("_score", v)?;
        }
        if let Some(v) = &self.seq_no {
            map.serialize_entry("_seq_no", v)?;
        }
        if let Some(v) = &self.shard {
            map.serialize_entry("_shard", v)?;
        }
        if let Some(v) = &self.source {
            map.serialize_entry("_source", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("_type", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("_version", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.highlight {
            map.serialize_entry("highlight", v)?;
        }
        if let Some(v) = &self.ignored_field_values {
            map.serialize_entry("ignored_field_values", v)?;
        }
        if let Some(v) = &self.inner_hits {
            map.serialize_entry("inner_hits", v)?;
        }
        if let Some(v) = &self.matched_queries {
            map.serialize_entry("matched_queries", v)?;
        }
        if let Some(v) = &self.sort {
            map.serialize_entry("sort", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
impl SearchResponseHitsHitsItem {
    #[doc = "Decodes the document body (`_source`) into `T`.\n\nReturns `Ok(None)` when the response carries no `_source` (`_source: false` in the request, or a mapping that disables it), and an error when the document does not match `T`.\n\n`T` may borrow from the response to avoid copying strings: use `#[serde(borrow)] Cow<'a, str>` fields rather than `&'a str`, which cannot hold a string containing JSON escapes."]
    pub fn source_as<'a, T>(&'a self) -> std::result::Result<Option<T>, serde_json::Error>
    where
        T: serde::Deserialize<'a>,
    {
        self.source
            .as_deref()
            .map(|raw| serde_json::from_str(raw.get()))
            .transpose()
    }
}

#[doc = "The names of queries that matched the document. When `include_named_queries_score` is false (default), returns an array of query names. When true, returns an object mapping query names to their scores."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SearchResponseHitsHitsItemMatchedQueries {
    Names(Vec<String>),
    Scores(std::collections::BTreeMap<String, f64>),
}
impl<'de> serde::Deserialize<'de> for SearchResponseHitsHitsItemMatchedQueries {
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
            Some(b'[') => {
                serde_json::from_str(text).map(SearchResponseHitsHitsItemMatchedQueries::Names)
            }
            Some(b'{') => {
                serde_json::from_str(text).map(SearchResponseHitsHitsItemMatchedQueries::Scores)
            }
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a SearchResponseHitsHitsItemMatchedQueries value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The total number of hits, present only if `track_total_hits` is not set to `false` in the search request."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SearchResponseHitsTotal {
    TotalHits(crate::types::search::TotalHits),
    Integer(i64),
}
impl<'de> serde::Deserialize<'de> for SearchResponseHitsTotal {
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
            Some(b'{') => serde_json::from_str(text).map(SearchResponseHitsTotal::TotalHits),
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(SearchResponseHitsTotal::Integer)
            }
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a SearchResponseHitsTotal value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}
impl crate::types::TotalHitCount for SearchResponseHitsTotal {
    fn value(&self) -> i64 {
        match self {
            Self::TotalHits(total) => total.value,
            Self::Integer(value) => *value,
        }
    }

    fn relation(&self) -> &crate::types::search::TotalHitsRelation {
        match self {
            Self::TotalHits(total) => &total.relation,
            Self::Integer(_) => &crate::types::total_hits::ACCURATE,
        }
    }
}

impl SearchResponseHitsTotal {
    #[doc = "The number of hits: exact when [`is_accurate`](Self::is_accurate) is true, and otherwise a lower bound. See [`TotalHitCount`](crate::types::TotalHitCount)."]
    pub fn value(&self) -> i64 {
        crate::types::TotalHitCount::value(self)
    }

    #[doc = "How [`value`](Self::value) relates to the actual number of hits: `Eq` when exact, `Gte` when a lower bound. The integer form is always `Eq`."]
    pub fn relation(&self) -> &crate::types::search::TotalHitsRelation {
        crate::types::TotalHitCount::relation(self)
    }

    #[doc = "Whether [`value`](Self::value) is the exact number of hits (relation `eq`). False for a lower bound, and for a relation this client does not know."]
    pub fn is_accurate(&self) -> bool {
        crate::types::TotalHitCount::is_accurate(self)
    }
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub clusters: Option<crate::types::common::ClusterStatistics>,
    pub scroll_id: Option<crate::types::common::ScrollId>,
    pub shards: crate::types::common::ShardStatistics,
    pub aggregations:
        Option<std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>>,
    pub hits: crate::types::search::SearchResultHits,
    pub num_reduce_phases: Option<i32>,
    pub phase_took: Option<crate::types::common::PhaseTook>,
    pub pit_id: Option<crate::types::common::Id>,
    pub processor_results: Option<Vec<crate::types::search::ProcessorExecutionDetail>>,
    pub profile: Option<crate::types::search::Profile>,
    pub suggest: Option<std::collections::BTreeMap<String, Vec<crate::types::search::Suggest>>>,
    pub terminated_early: Option<bool>,
    pub timed_out: bool,
    pub took: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SearchResult {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SearchResult;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchResult object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_clusters: Option<crate::types::common::ClusterStatistics> = None;
                let mut seen_clusters = false;
                let mut f_scroll_id: Option<crate::types::common::ScrollId> = None;
                let mut seen_scroll_id = false;
                let mut f_shards: Option<crate::types::common::ShardStatistics> = None;
                let mut f_aggregations: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::common_aggregations::Aggregate,
                    >,
                > = None;
                let mut seen_aggregations = false;
                let mut f_hits: Option<crate::types::search::SearchResultHits> = None;
                let mut f_num_reduce_phases: Option<i32> = None;
                let mut seen_num_reduce_phases = false;
                let mut f_phase_took: Option<crate::types::common::PhaseTook> = None;
                let mut seen_phase_took = false;
                let mut f_pit_id: Option<crate::types::common::Id> = None;
                let mut seen_pit_id = false;
                let mut f_processor_results: Option<
                    Vec<crate::types::search::ProcessorExecutionDetail>,
                > = None;
                let mut seen_processor_results = false;
                let mut f_profile: Option<crate::types::search::Profile> = None;
                let mut seen_profile = false;
                let mut f_suggest: Option<
                    std::collections::BTreeMap<String, Vec<crate::types::search::Suggest>>,
                > = None;
                let mut seen_suggest = false;
                let mut f_terminated_early: Option<bool> = None;
                let mut seen_terminated_early = false;
                let mut f_timed_out: Option<bool> = None;
                let mut f_took: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_clusters" => {
                            if seen_clusters {
                                return Err(serde::de::Error::duplicate_field("_clusters"));
                            }
                            seen_clusters = true;
                            f_clusters = map
                                .next_value::<Option<crate::types::common::ClusterStatistics>>()?;
                        }
                        "_scroll_id" => {
                            if seen_scroll_id {
                                return Err(serde::de::Error::duplicate_field("_scroll_id"));
                            }
                            seen_scroll_id = true;
                            f_scroll_id =
                                map.next_value::<Option<crate::types::common::ScrollId>>()?;
                        }
                        "_shards" => {
                            if f_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("_shards"));
                            }
                            f_shards =
                                Some(map.next_value::<crate::types::common::ShardStatistics>()?);
                        }
                        "aggregations" => {
                            if seen_aggregations {
                                return Err(serde::de::Error::duplicate_field("aggregations"));
                            }
                            seen_aggregations = true;
                            f_aggregations = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_aggregations::Aggregate,
                                >,
                            >>()?;
                        }
                        "hits" => {
                            if f_hits.is_some() {
                                return Err(serde::de::Error::duplicate_field("hits"));
                            }
                            f_hits =
                                Some(map.next_value::<crate::types::search::SearchResultHits>()?);
                        }
                        "num_reduce_phases" => {
                            if seen_num_reduce_phases {
                                return Err(serde::de::Error::duplicate_field("num_reduce_phases"));
                            }
                            seen_num_reduce_phases = true;
                            f_num_reduce_phases = map.next_value::<Option<i32>>()?;
                        }
                        "phase_took" => {
                            if seen_phase_took {
                                return Err(serde::de::Error::duplicate_field("phase_took"));
                            }
                            seen_phase_took = true;
                            f_phase_took =
                                map.next_value::<Option<crate::types::common::PhaseTook>>()?;
                        }
                        "pit_id" => {
                            if seen_pit_id {
                                return Err(serde::de::Error::duplicate_field("pit_id"));
                            }
                            seen_pit_id = true;
                            f_pit_id = map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        "processor_results" => {
                            if seen_processor_results {
                                return Err(serde::de::Error::duplicate_field("processor_results"));
                            }
                            seen_processor_results = true;
                            f_processor_results = map.next_value::<Option<Vec<crate::types::search::ProcessorExecutionDetail>>>()?;
                        }
                        "profile" => {
                            if seen_profile {
                                return Err(serde::de::Error::duplicate_field("profile"));
                            }
                            seen_profile = true;
                            f_profile =
                                map.next_value::<Option<crate::types::search::Profile>>()?;
                        }
                        "suggest" => {
                            if seen_suggest {
                                return Err(serde::de::Error::duplicate_field("suggest"));
                            }
                            seen_suggest = true;
                            f_suggest = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Vec<crate::types::search::Suggest>,
                                >,
                            >>()?;
                        }
                        "terminated_early" => {
                            if seen_terminated_early {
                                return Err(serde::de::Error::duplicate_field("terminated_early"));
                            }
                            seen_terminated_early = true;
                            f_terminated_early = map.next_value::<Option<bool>>()?;
                        }
                        "timed_out" => {
                            if f_timed_out.is_some() {
                                return Err(serde::de::Error::duplicate_field("timed_out"));
                            }
                            f_timed_out = Some(map.next_value::<bool>()?);
                        }
                        "took" => {
                            if f_took.is_some() {
                                return Err(serde::de::Error::duplicate_field("took"));
                            }
                            f_took = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SearchResult {
                    clusters: f_clusters,
                    scroll_id: f_scroll_id,
                    shards: f_shards.ok_or_else(|| serde::de::Error::missing_field("_shards"))?,
                    aggregations: f_aggregations,
                    hits: f_hits.ok_or_else(|| serde::de::Error::missing_field("hits"))?,
                    num_reduce_phases: f_num_reduce_phases,
                    phase_took: f_phase_took,
                    pit_id: f_pit_id,
                    processor_results: f_processor_results,
                    profile: f_profile,
                    suggest: f_suggest,
                    terminated_early: f_terminated_early,
                    timed_out: f_timed_out
                        .ok_or_else(|| serde::de::Error::missing_field("timed_out"))?,
                    took: f_took.ok_or_else(|| serde::de::Error::missing_field("took"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SearchResult {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.clusters {
            map.serialize_entry("_clusters", v)?;
        }
        if let Some(v) = &self.scroll_id {
            map.serialize_entry("_scroll_id", v)?;
        }
        map.serialize_entry("_shards", &self.shards)?;
        if let Some(v) = &self.aggregations {
            map.serialize_entry("aggregations", v)?;
        }
        map.serialize_entry("hits", &self.hits)?;
        if let Some(v) = &self.num_reduce_phases {
            map.serialize_entry("num_reduce_phases", v)?;
        }
        if let Some(v) = &self.phase_took {
            map.serialize_entry("phase_took", v)?;
        }
        if let Some(v) = &self.pit_id {
            map.serialize_entry("pit_id", v)?;
        }
        if let Some(v) = &self.processor_results {
            map.serialize_entry("processor_results", v)?;
        }
        if let Some(v) = &self.profile {
            map.serialize_entry("profile", v)?;
        }
        if let Some(v) = &self.suggest {
            map.serialize_entry("suggest", v)?;
        }
        if let Some(v) = &self.terminated_early {
            map.serialize_entry("terminated_early", v)?;
        }
        map.serialize_entry("timed_out", &self.timed_out)?;
        map.serialize_entry("took", &self.took)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SearchResultHits {
    pub hits: Vec<crate::types::search::SearchResultHitsHitsItem>,
    pub max_score: Option<f64>,
    #[doc = "The total number of hits, present only if `track_total_hits` is not set to `false` in the search request."]
    pub total: Option<crate::types::search::SearchResultHitsTotal>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SearchResultHits {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SearchResultHits;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchResultHits object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_hits: Option<Vec<crate::types::search::SearchResultHitsHitsItem>> = None;
                let mut f_max_score: Option<f64> = None;
                let mut seen_max_score = false;
                let mut f_total: Option<crate::types::search::SearchResultHitsTotal> = None;
                let mut seen_total = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "hits" => {
                            if f_hits.is_some() {
                                return Err(serde::de::Error::duplicate_field("hits"));
                            }
                            f_hits = Some(map.next_value::<Vec<crate::types::search::SearchResultHitsHitsItem>>()?);
                        }
                        "max_score" => {
                            if seen_max_score {
                                return Err(serde::de::Error::duplicate_field("max_score"));
                            }
                            seen_max_score = true;
                            f_max_score = map.next_value::<Option<f64>>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map
                                .next_value::<Option<crate::types::search::SearchResultHitsTotal>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SearchResultHits {
                    hits: f_hits.ok_or_else(|| serde::de::Error::missing_field("hits"))?,
                    max_score: f_max_score,
                    total: f_total,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SearchResultHits {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("hits", &self.hits)?;
        if let Some(v) = &self.max_score {
            map.serialize_entry("max_score", v)?;
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
impl SearchResultHits {
    #[doc = "The total number of hits (`total`), or `None` when the request disabled counting (`track_total_hits: false`).

The number may be a lower bound: by default the count is only tracked accurately up to 10,000 hits. Use `total` and its [`is_accurate`](crate::types::TotalHitCount::is_accurate) when that matters."]
    pub fn total_value(&self) -> Option<i64> {
        self.total.as_ref().map(|total| total.value())
    }
}

#[derive(Debug, Clone)]
pub struct SearchResultHitsHitsItem {
    pub explanation: Option<crate::types::explain::Explanation>,
    pub id: Option<crate::types::common::Id>,
    pub ignored: Option<Vec<String>>,
    pub index: Option<crate::types::common::IndexName>,
    pub nested: Option<crate::types::search::NestedIdentity>,
    pub node: Option<String>,
    pub primary_term: Option<i64>,
    pub routing: Option<String>,
    pub score: Option<f64>,
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    pub shard: Option<String>,
    pub source: Option<Box<serde_json::value::RawValue>>,
    pub r#type: Option<crate::types::common::Type>,
    pub version: Option<crate::types::common::VersionNumber>,
    pub fields: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub highlight: Option<std::collections::BTreeMap<String, Vec<String>>>,
    pub ignored_field_values: Option<std::collections::BTreeMap<String, Vec<String>>>,
    pub inner_hits:
        Option<std::collections::BTreeMap<String, crate::types::search::InnerHitsResult>>,
    #[doc = "The names of queries that matched the document. When `include_named_queries_score` is false (default), returns an array of query names. When true, returns an object mapping query names to their scores."]
    pub matched_queries: Option<crate::types::search::SearchResultHitsHitsItemMatchedQueries>,
    pub sort: Option<crate::types::common::SortResults>,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SearchResultHitsHitsItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SearchResultHitsHitsItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchResultHitsHitsItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_explanation: Option<crate::types::explain::Explanation> = None;
                let mut seen_explanation = false;
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut seen_id = false;
                let mut f_ignored: Option<Vec<String>> = None;
                let mut seen_ignored = false;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut seen_index = false;
                let mut f_nested: Option<crate::types::search::NestedIdentity> = None;
                let mut seen_nested = false;
                let mut f_node: Option<String> = None;
                let mut seen_node = false;
                let mut f_primary_term: Option<i64> = None;
                let mut seen_primary_term = false;
                let mut f_routing: Option<String> = None;
                let mut seen_routing = false;
                let mut f_score: Option<f64> = None;
                let mut seen_score = false;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut seen_seq_no = false;
                let mut f_shard: Option<String> = None;
                let mut seen_shard = false;
                let mut f_source: Option<Box<serde_json::value::RawValue>> = None;
                let mut seen_source = false;
                let mut f_type: Option<crate::types::common::Type> = None;
                let mut seen_type = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_fields = false;
                let mut f_highlight: Option<std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut seen_highlight = false;
                let mut f_ignored_field_values: Option<
                    std::collections::BTreeMap<String, Vec<String>>,
                > = None;
                let mut seen_ignored_field_values = false;
                let mut f_inner_hits: Option<
                    std::collections::BTreeMap<String, crate::types::search::InnerHitsResult>,
                > = None;
                let mut seen_inner_hits = false;
                let mut f_matched_queries: Option<
                    crate::types::search::SearchResultHitsHitsItemMatchedQueries,
                > = None;
                let mut seen_matched_queries = false;
                let mut f_sort: Option<crate::types::common::SortResults> = None;
                let mut seen_sort = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_explanation" => {
                            if seen_explanation {
                                return Err(serde::de::Error::duplicate_field("_explanation"));
                            }
                            seen_explanation = true;
                            f_explanation =
                                map.next_value::<Option<crate::types::explain::Explanation>>()?;
                        }
                        "_id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("_id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        "_ignored" => {
                            if seen_ignored {
                                return Err(serde::de::Error::duplicate_field("_ignored"));
                            }
                            seen_ignored = true;
                            f_ignored = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "_index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("_index"));
                            }
                            seen_index = true;
                            f_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "_nested" => {
                            if seen_nested {
                                return Err(serde::de::Error::duplicate_field("_nested"));
                            }
                            seen_nested = true;
                            f_nested =
                                map.next_value::<Option<crate::types::search::NestedIdentity>>()?;
                        }
                        "_node" => {
                            if seen_node {
                                return Err(serde::de::Error::duplicate_field("_node"));
                            }
                            seen_node = true;
                            f_node = map.next_value::<Option<String>>()?;
                        }
                        "_primary_term" => {
                            if seen_primary_term {
                                return Err(serde::de::Error::duplicate_field("_primary_term"));
                            }
                            seen_primary_term = true;
                            f_primary_term = map.next_value::<Option<i64>>()?;
                        }
                        "_routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("_routing"));
                            }
                            seen_routing = true;
                            f_routing = map.next_value::<Option<String>>()?;
                        }
                        "_score" => {
                            if seen_score {
                                return Err(serde::de::Error::duplicate_field("_score"));
                            }
                            seen_score = true;
                            f_score = map.next_value::<Option<f64>>()?;
                        }
                        "_seq_no" => {
                            if seen_seq_no {
                                return Err(serde::de::Error::duplicate_field("_seq_no"));
                            }
                            seen_seq_no = true;
                            f_seq_no =
                                map.next_value::<Option<crate::types::common::SequenceNumber>>()?;
                        }
                        "_shard" => {
                            if seen_shard {
                                return Err(serde::de::Error::duplicate_field("_shard"));
                            }
                            seen_shard = true;
                            f_shard = map.next_value::<Option<String>>()?;
                        }
                        "_source" => {
                            if seen_source {
                                return Err(serde::de::Error::duplicate_field("_source"));
                            }
                            seen_source = true;
                            f_source =
                                map.next_value::<Option<Box<serde_json::value::RawValue>>>()?;
                        }
                        "_type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<crate::types::common::Type>>()?;
                        }
                        "_version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionNumber>>()?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "highlight" => {
                            if seen_highlight {
                                return Err(serde::de::Error::duplicate_field("highlight"));
                            }
                            seen_highlight = true;
                            f_highlight = map.next_value::<Option<std::collections::BTreeMap<String, Vec<String>>>>()?;
                        }
                        "ignored_field_values" => {
                            if seen_ignored_field_values {
                                return Err(serde::de::Error::duplicate_field(
                                    "ignored_field_values",
                                ));
                            }
                            seen_ignored_field_values = true;
                            f_ignored_field_values = map.next_value::<Option<std::collections::BTreeMap<String, Vec<String>>>>()?;
                        }
                        "inner_hits" => {
                            if seen_inner_hits {
                                return Err(serde::de::Error::duplicate_field("inner_hits"));
                            }
                            seen_inner_hits = true;
                            f_inner_hits = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::search::InnerHitsResult,
                                >,
                            >>()?;
                        }
                        "matched_queries" => {
                            if seen_matched_queries {
                                return Err(serde::de::Error::duplicate_field("matched_queries"));
                            }
                            seen_matched_queries = true;
                            f_matched_queries = map.next_value::<Option<
                                crate::types::search::SearchResultHitsHitsItemMatchedQueries,
                            >>()?;
                        }
                        "sort" => {
                            if seen_sort {
                                return Err(serde::de::Error::duplicate_field("sort"));
                            }
                            seen_sort = true;
                            f_sort =
                                map.next_value::<Option<crate::types::common::SortResults>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SearchResultHitsHitsItem {
                    explanation: f_explanation,
                    id: f_id,
                    ignored: f_ignored,
                    index: f_index,
                    nested: f_nested,
                    node: f_node,
                    primary_term: f_primary_term,
                    routing: f_routing,
                    score: f_score,
                    seq_no: f_seq_no,
                    shard: f_shard,
                    source: f_source,
                    r#type: f_type,
                    version: f_version,
                    fields: f_fields,
                    highlight: f_highlight,
                    ignored_field_values: f_ignored_field_values,
                    inner_hits: f_inner_hits,
                    matched_queries: f_matched_queries,
                    sort: f_sort,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SearchResultHitsHitsItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.explanation {
            map.serialize_entry("_explanation", v)?;
        }
        if let Some(v) = &self.id {
            map.serialize_entry("_id", v)?;
        }
        if let Some(v) = &self.ignored {
            map.serialize_entry("_ignored", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("_index", v)?;
        }
        if let Some(v) = &self.nested {
            map.serialize_entry("_nested", v)?;
        }
        if let Some(v) = &self.node {
            map.serialize_entry("_node", v)?;
        }
        if let Some(v) = &self.primary_term {
            map.serialize_entry("_primary_term", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("_routing", v)?;
        }
        if let Some(v) = &self.score {
            map.serialize_entry("_score", v)?;
        }
        if let Some(v) = &self.seq_no {
            map.serialize_entry("_seq_no", v)?;
        }
        if let Some(v) = &self.shard {
            map.serialize_entry("_shard", v)?;
        }
        if let Some(v) = &self.source {
            map.serialize_entry("_source", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("_type", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("_version", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.highlight {
            map.serialize_entry("highlight", v)?;
        }
        if let Some(v) = &self.ignored_field_values {
            map.serialize_entry("ignored_field_values", v)?;
        }
        if let Some(v) = &self.inner_hits {
            map.serialize_entry("inner_hits", v)?;
        }
        if let Some(v) = &self.matched_queries {
            map.serialize_entry("matched_queries", v)?;
        }
        if let Some(v) = &self.sort {
            map.serialize_entry("sort", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
impl SearchResultHitsHitsItem {
    #[doc = "Decodes the document body (`_source`) into `T`.\n\nReturns `Ok(None)` when the response carries no `_source` (`_source: false` in the request, or a mapping that disables it), and an error when the document does not match `T`.\n\n`T` may borrow from the response to avoid copying strings: use `#[serde(borrow)] Cow<'a, str>` fields rather than `&'a str`, which cannot hold a string containing JSON escapes."]
    pub fn source_as<'a, T>(&'a self) -> std::result::Result<Option<T>, serde_json::Error>
    where
        T: serde::Deserialize<'a>,
    {
        self.source
            .as_deref()
            .map(|raw| serde_json::from_str(raw.get()))
            .transpose()
    }
}

#[doc = "The names of queries that matched the document. When `include_named_queries_score` is false (default), returns an array of query names. When true, returns an object mapping query names to their scores."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SearchResultHitsHitsItemMatchedQueries {
    Names(Vec<String>),
    Scores(std::collections::BTreeMap<String, f64>),
}
impl<'de> serde::Deserialize<'de> for SearchResultHitsHitsItemMatchedQueries {
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
            Some(b'[') => {
                serde_json::from_str(text).map(SearchResultHitsHitsItemMatchedQueries::Names)
            }
            Some(b'{') => {
                serde_json::from_str(text).map(SearchResultHitsHitsItemMatchedQueries::Scores)
            }
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a SearchResultHitsHitsItemMatchedQueries value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The total number of hits, present only if `track_total_hits` is not set to `false` in the search request."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SearchResultHitsTotal {
    TotalHits(crate::types::search::TotalHits),
    Integer(i64),
}
impl<'de> serde::Deserialize<'de> for SearchResultHitsTotal {
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
            Some(b'{') => serde_json::from_str(text).map(SearchResultHitsTotal::TotalHits),
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(SearchResultHitsTotal::Integer)
            }
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a SearchResultHitsTotal value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}
impl crate::types::TotalHitCount for SearchResultHitsTotal {
    fn value(&self) -> i64 {
        match self {
            Self::TotalHits(total) => total.value,
            Self::Integer(value) => *value,
        }
    }

    fn relation(&self) -> &crate::types::search::TotalHitsRelation {
        match self {
            Self::TotalHits(total) => &total.relation,
            Self::Integer(_) => &crate::types::total_hits::ACCURATE,
        }
    }
}

impl SearchResultHitsTotal {
    #[doc = "The number of hits: exact when [`is_accurate`](Self::is_accurate) is true, and otherwise a lower bound. See [`TotalHitCount`](crate::types::TotalHitCount)."]
    pub fn value(&self) -> i64 {
        crate::types::TotalHitCount::value(self)
    }

    #[doc = "How [`value`](Self::value) relates to the actual number of hits: `Eq` when exact, `Gte` when a lower bound. The integer form is always `Eq`."]
    pub fn relation(&self) -> &crate::types::search::TotalHitsRelation {
        crate::types::TotalHitCount::relation(self)
    }

    #[doc = "Whether [`value`](Self::value) is the exact number of hits (relation `eq`). False for a lower bound, and for a relation this client does not know."]
    pub fn is_accurate(&self) -> bool {
        crate::types::TotalHitCount::is_accurate(self)
    }
}

#[derive(Debug, Clone)]
pub struct ShardProfile {
    pub aggregations: Vec<crate::types::search::AggregationProfile>,
    pub fetch: Option<crate::types::search::FetchProfile>,
    pub id: String,
    pub searches: Vec<crate::types::search::SearchProfile>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardProfile {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardProfile;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardProfile object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_aggregations: Option<Vec<crate::types::search::AggregationProfile>> =
                    None;
                let mut f_fetch: Option<crate::types::search::FetchProfile> = None;
                let mut seen_fetch = false;
                let mut f_id: Option<String> = None;
                let mut f_searches: Option<Vec<crate::types::search::SearchProfile>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "aggregations" => {
                            if f_aggregations.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggregations"));
                            }
                            f_aggregations = Some(
                                map.next_value::<Vec<crate::types::search::AggregationProfile>>()?,
                            );
                        }
                        "fetch" => {
                            if seen_fetch {
                                return Err(serde::de::Error::duplicate_field("fetch"));
                            }
                            seen_fetch = true;
                            f_fetch =
                                map.next_value::<Option<crate::types::search::FetchProfile>>()?;
                        }
                        "id" => {
                            if f_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            f_id = Some(map.next_value::<String>()?);
                        }
                        "searches" => {
                            if f_searches.is_some() {
                                return Err(serde::de::Error::duplicate_field("searches"));
                            }
                            f_searches =
                                Some(map.next_value::<Vec<crate::types::search::SearchProfile>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardProfile {
                    aggregations: f_aggregations
                        .ok_or_else(|| serde::de::Error::missing_field("aggregations"))?,
                    fetch: f_fetch,
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("id"))?,
                    searches: f_searches
                        .ok_or_else(|| serde::de::Error::missing_field("searches"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardProfile {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("aggregations", &self.aggregations)?;
        if let Some(v) = &self.fetch {
            map.serialize_entry("fetch", v)?;
        }
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("searches", &self.searches)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SmoothingModel {
    pub laplace: Option<crate::types::search::LaplaceSmoothingModel>,
    pub linear_interpolation: Option<crate::types::search::LinearInterpolationSmoothingModel>,
    pub stupid_backoff: Option<crate::types::search::StupidBackoffSmoothingModel>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SmoothingModel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SmoothingModel;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SmoothingModel object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_laplace: Option<crate::types::search::LaplaceSmoothingModel> = None;
                let mut seen_laplace = false;
                let mut f_linear_interpolation: Option<
                    crate::types::search::LinearInterpolationSmoothingModel,
                > = None;
                let mut seen_linear_interpolation = false;
                let mut f_stupid_backoff: Option<
                    crate::types::search::StupidBackoffSmoothingModel,
                > = None;
                let mut seen_stupid_backoff = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "laplace" => {
                            if seen_laplace {
                                return Err(serde::de::Error::duplicate_field("laplace"));
                            }
                            seen_laplace = true;
                            f_laplace = map
                                .next_value::<Option<crate::types::search::LaplaceSmoothingModel>>(
                                )?;
                        }
                        "linear_interpolation" => {
                            if seen_linear_interpolation {
                                return Err(serde::de::Error::duplicate_field(
                                    "linear_interpolation",
                                ));
                            }
                            seen_linear_interpolation = true;
                            f_linear_interpolation = map.next_value::<Option<crate::types::search::LinearInterpolationSmoothingModel>>()?;
                        }
                        "stupid_backoff" => {
                            if seen_stupid_backoff {
                                return Err(serde::de::Error::duplicate_field("stupid_backoff"));
                            }
                            seen_stupid_backoff = true;
                            f_stupid_backoff = map.next_value::<Option<crate::types::search::StupidBackoffSmoothingModel>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SmoothingModel {
                    laplace: f_laplace,
                    linear_interpolation: f_linear_interpolation,
                    stupid_backoff: f_stupid_backoff,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SmoothingModel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.laplace {
            map.serialize_entry("laplace", v)?;
        }
        if let Some(v) = &self.linear_interpolation {
            map.serialize_entry("linear_interpolation", v)?;
        }
        if let Some(v) = &self.stupid_backoff {
            map.serialize_entry("stupid_backoff", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Defines how to fetch a source. Fetching can be disabled entirely, or the source can be filtered."]
#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SourceConfig {
    raw: Box<serde_json::value::RawValue>,
}
impl SourceConfig {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `fetch` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_fetch(&self) -> std::result::Result<bool, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "fetch")
    }
    #[doc = "Parses the value as the `filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_filter(
        &self,
    ) -> std::result::Result<crate::types::search::SourceFilter, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "filter")
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SourceFilter {
    raw: Box<serde_json::value::RawValue>,
}
impl SourceFilter {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `includes` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_includes(
        &self,
    ) -> std::result::Result<crate::types::common::Fields, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "includes")
    }
    #[doc = "Parses the value as the `object` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_object(
        &self,
    ) -> std::result::Result<crate::types::search::SourceFilterObject, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "object")
    }
}

#[derive(Debug, Clone)]
pub struct SourceFilterObject {
    pub excludes: Option<crate::types::common::Fields>,
    pub includes: Option<crate::types::common::Fields>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SourceFilterObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SourceFilterObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SourceFilterObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_excludes: Option<crate::types::common::Fields> = None;
                let mut seen_excludes = false;
                let mut f_includes: Option<crate::types::common::Fields> = None;
                let mut seen_includes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "excludes" => {
                            if seen_excludes {
                                return Err(serde::de::Error::duplicate_field("excludes"));
                            }
                            seen_excludes = true;
                            f_excludes =
                                map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "includes" => {
                            if seen_includes {
                                return Err(serde::de::Error::duplicate_field("includes"));
                            }
                            seen_includes = true;
                            f_includes =
                                map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SourceFilterObject {
                    excludes: f_excludes,
                    includes: f_includes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SourceFilterObject {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.excludes {
            map.serialize_entry("excludes", v)?;
        }
        if let Some(v) = &self.includes {
            map.serialize_entry("includes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StringDistance {
    #[doc = "`damerau_levenshtein`"]
    DamerauLevenshtein,
    #[doc = "`internal`"]
    Internal,
    #[doc = "`jaro_winkler`"]
    JaroWinkler,
    #[doc = "`levenshtein`"]
    Levenshtein,
    #[doc = "`ngram`"]
    Ngram,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl StringDistance {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            StringDistance::DamerauLevenshtein => "damerau_levenshtein",
            StringDistance::Internal => "internal",
            StringDistance::JaroWinkler => "jaro_winkler",
            StringDistance::Levenshtein => "levenshtein",
            StringDistance::Ngram => "ngram",
            StringDistance::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for StringDistance {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StringDistance {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "damerau_levenshtein" => StringDistance::DamerauLevenshtein,
            "internal" => StringDistance::Internal,
            "jaro_winkler" => StringDistance::JaroWinkler,
            "levenshtein" => StringDistance::Levenshtein,
            "ngram" => StringDistance::Ngram,
            _ => StringDistance::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct StupidBackoffSmoothingModel {
    pub discount: f64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StupidBackoffSmoothingModel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StupidBackoffSmoothingModel;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StupidBackoffSmoothingModel object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_discount: Option<f64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "discount" => {
                            if f_discount.is_some() {
                                return Err(serde::de::Error::duplicate_field("discount"));
                            }
                            f_discount = Some(map.next_value::<f64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StupidBackoffSmoothingModel {
                    discount: f_discount
                        .ok_or_else(|| serde::de::Error::missing_field("discount"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StupidBackoffSmoothingModel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("discount", &self.discount)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Suggest {
    raw: Box<serde_json::value::RawValue>,
}
impl Suggest {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `completion` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_completion(
        &self,
    ) -> std::result::Result<crate::types::search::SuggestCompletion, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "completion")
    }
    #[doc = "Parses the value as the `phrase` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_phrase(
        &self,
    ) -> std::result::Result<crate::types::search::PhraseSuggest, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "phrase")
    }
    #[doc = "Parses the value as the `term` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_term(
        &self,
    ) -> std::result::Result<crate::types::search::TermSuggest, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "term")
    }
}

#[derive(Debug, Clone)]
pub struct SuggestCompletion {
    pub length: i32,
    pub offset: i32,
    pub text: String,
    pub options: crate::types::search::SuggestCompletionOptions,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SuggestCompletion {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SuggestCompletion;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SuggestCompletion object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_length: Option<i32> = None;
                let mut f_offset: Option<i32> = None;
                let mut f_text: Option<String> = None;
                let mut f_options: Option<crate::types::search::SuggestCompletionOptions> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "length" => {
                            if f_length.is_some() {
                                return Err(serde::de::Error::duplicate_field("length"));
                            }
                            f_length = Some(map.next_value::<i32>()?);
                        }
                        "offset" => {
                            if f_offset.is_some() {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            f_offset = Some(map.next_value::<i32>()?);
                        }
                        "text" => {
                            if f_text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            f_text = Some(map.next_value::<String>()?);
                        }
                        "options" => {
                            if f_options.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            f_options = Some(
                                map.next_value::<crate::types::search::SuggestCompletionOptions>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SuggestCompletion {
                    length: f_length.ok_or_else(|| serde::de::Error::missing_field("length"))?,
                    offset: f_offset.ok_or_else(|| serde::de::Error::missing_field("offset"))?,
                    text: f_text.ok_or_else(|| serde::de::Error::missing_field("text"))?,
                    options: f_options.ok_or_else(|| serde::de::Error::missing_field("options"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SuggestCompletion {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("length", &self.length)?;
        map.serialize_entry("offset", &self.offset)?;
        map.serialize_entry("text", &self.text)?;
        map.serialize_entry("options", &self.options)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SuggestCompletionOptions {
    Object(crate::types::search::SuggestCompletionOptionsObject),
    Array(Vec<crate::types::search::SuggestCompletionOptionsArrayItem>),
}
impl<'de> serde::Deserialize<'de> for SuggestCompletionOptions {
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
            Some(b'{') => serde_json::from_str(text).map(SuggestCompletionOptions::Object),
            Some(b'[') => serde_json::from_str(text).map(SuggestCompletionOptions::Array),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a SuggestCompletionOptions value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct SuggestCompletionOptionsArrayItem {
    pub id: Option<String>,
    pub index: Option<crate::types::common::IndexName>,
    pub routing: Option<crate::types::common::Routing>,
    pub score: Option<f64>,
    pub source: Option<Box<serde_json::value::RawValue>>,
    pub collate_match: Option<bool>,
    pub contexts: Option<std::collections::BTreeMap<String, Vec<crate::types::search::Context>>>,
    pub fields: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub _score: Option<f64>,
    pub text: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SuggestCompletionOptionsArrayItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SuggestCompletionOptionsArrayItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SuggestCompletionOptionsArrayItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_id: Option<String> = None;
                let mut seen_id = false;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut seen_index = false;
                let mut f_routing: Option<crate::types::common::Routing> = None;
                let mut seen_routing = false;
                let mut f_score: Option<f64> = None;
                let mut seen_score = false;
                let mut f_source: Option<Box<serde_json::value::RawValue>> = None;
                let mut seen_source = false;
                let mut f_collate_match: Option<bool> = None;
                let mut seen_collate_match = false;
                let mut f_contexts: Option<
                    std::collections::BTreeMap<String, Vec<crate::types::search::Context>>,
                > = None;
                let mut seen_contexts = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_fields = false;
                let mut f__score: Option<f64> = None;
                let mut seen__score = false;
                let mut f_text: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("_id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<String>>()?;
                        }
                        "_index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("_index"));
                            }
                            seen_index = true;
                            f_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "_routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("_routing"));
                            }
                            seen_routing = true;
                            f_routing =
                                map.next_value::<Option<crate::types::common::Routing>>()?;
                        }
                        "_score" => {
                            if seen_score {
                                return Err(serde::de::Error::duplicate_field("_score"));
                            }
                            seen_score = true;
                            f_score = map.next_value::<Option<f64>>()?;
                        }
                        "_source" => {
                            if seen_source {
                                return Err(serde::de::Error::duplicate_field("_source"));
                            }
                            seen_source = true;
                            f_source =
                                map.next_value::<Option<Box<serde_json::value::RawValue>>>()?;
                        }
                        "collate_match" => {
                            if seen_collate_match {
                                return Err(serde::de::Error::duplicate_field("collate_match"));
                            }
                            seen_collate_match = true;
                            f_collate_match = map.next_value::<Option<bool>>()?;
                        }
                        "contexts" => {
                            if seen_contexts {
                                return Err(serde::de::Error::duplicate_field("contexts"));
                            }
                            seen_contexts = true;
                            f_contexts = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Vec<crate::types::search::Context>,
                                >,
                            >>()?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "score" => {
                            if seen__score {
                                return Err(serde::de::Error::duplicate_field("score"));
                            }
                            seen__score = true;
                            f__score = map.next_value::<Option<f64>>()?;
                        }
                        "text" => {
                            if f_text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            f_text = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SuggestCompletionOptionsArrayItem {
                    id: f_id,
                    index: f_index,
                    routing: f_routing,
                    score: f_score,
                    source: f_source,
                    collate_match: f_collate_match,
                    contexts: f_contexts,
                    fields: f_fields,
                    _score: f__score,
                    text: f_text.ok_or_else(|| serde::de::Error::missing_field("text"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SuggestCompletionOptionsArrayItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.id {
            map.serialize_entry("_id", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("_index", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("_routing", v)?;
        }
        if let Some(v) = &self.score {
            map.serialize_entry("_score", v)?;
        }
        if let Some(v) = &self.source {
            map.serialize_entry("_source", v)?;
        }
        if let Some(v) = &self.collate_match {
            map.serialize_entry("collate_match", v)?;
        }
        if let Some(v) = &self.contexts {
            map.serialize_entry("contexts", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self._score {
            map.serialize_entry("score", v)?;
        }
        map.serialize_entry("text", &self.text)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
impl SuggestCompletionOptionsArrayItem {
    #[doc = "Decodes the document body (`_source`) into `T`.\n\nReturns `Ok(None)` when the response carries no `_source` (`_source: false` in the request, or a mapping that disables it), and an error when the document does not match `T`.\n\n`T` may borrow from the response to avoid copying strings: use `#[serde(borrow)] Cow<'a, str>` fields rather than `&'a str`, which cannot hold a string containing JSON escapes."]
    pub fn source_as<'a, T>(&'a self) -> std::result::Result<Option<T>, serde_json::Error>
    where
        T: serde::Deserialize<'a>,
    {
        self.source
            .as_deref()
            .map(|raw| serde_json::from_str(raw.get()))
            .transpose()
    }
}

#[derive(Debug, Clone)]
pub struct SuggestCompletionOptionsObject {
    pub id: Option<String>,
    pub index: Option<crate::types::common::IndexName>,
    pub routing: Option<crate::types::common::Routing>,
    pub score: Option<f64>,
    pub source: Option<Box<serde_json::value::RawValue>>,
    pub collate_match: Option<bool>,
    pub contexts: Option<std::collections::BTreeMap<String, Vec<crate::types::search::Context>>>,
    pub fields: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub _score: Option<f64>,
    pub text: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SuggestCompletionOptionsObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SuggestCompletionOptionsObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SuggestCompletionOptionsObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_id: Option<String> = None;
                let mut seen_id = false;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut seen_index = false;
                let mut f_routing: Option<crate::types::common::Routing> = None;
                let mut seen_routing = false;
                let mut f_score: Option<f64> = None;
                let mut seen_score = false;
                let mut f_source: Option<Box<serde_json::value::RawValue>> = None;
                let mut seen_source = false;
                let mut f_collate_match: Option<bool> = None;
                let mut seen_collate_match = false;
                let mut f_contexts: Option<
                    std::collections::BTreeMap<String, Vec<crate::types::search::Context>>,
                > = None;
                let mut seen_contexts = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_fields = false;
                let mut f__score: Option<f64> = None;
                let mut seen__score = false;
                let mut f_text: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("_id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<String>>()?;
                        }
                        "_index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("_index"));
                            }
                            seen_index = true;
                            f_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "_routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("_routing"));
                            }
                            seen_routing = true;
                            f_routing =
                                map.next_value::<Option<crate::types::common::Routing>>()?;
                        }
                        "_score" => {
                            if seen_score {
                                return Err(serde::de::Error::duplicate_field("_score"));
                            }
                            seen_score = true;
                            f_score = map.next_value::<Option<f64>>()?;
                        }
                        "_source" => {
                            if seen_source {
                                return Err(serde::de::Error::duplicate_field("_source"));
                            }
                            seen_source = true;
                            f_source =
                                map.next_value::<Option<Box<serde_json::value::RawValue>>>()?;
                        }
                        "collate_match" => {
                            if seen_collate_match {
                                return Err(serde::de::Error::duplicate_field("collate_match"));
                            }
                            seen_collate_match = true;
                            f_collate_match = map.next_value::<Option<bool>>()?;
                        }
                        "contexts" => {
                            if seen_contexts {
                                return Err(serde::de::Error::duplicate_field("contexts"));
                            }
                            seen_contexts = true;
                            f_contexts = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Vec<crate::types::search::Context>,
                                >,
                            >>()?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "score" => {
                            if seen__score {
                                return Err(serde::de::Error::duplicate_field("score"));
                            }
                            seen__score = true;
                            f__score = map.next_value::<Option<f64>>()?;
                        }
                        "text" => {
                            if f_text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            f_text = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SuggestCompletionOptionsObject {
                    id: f_id,
                    index: f_index,
                    routing: f_routing,
                    score: f_score,
                    source: f_source,
                    collate_match: f_collate_match,
                    contexts: f_contexts,
                    fields: f_fields,
                    _score: f__score,
                    text: f_text.ok_or_else(|| serde::de::Error::missing_field("text"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SuggestCompletionOptionsObject {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.id {
            map.serialize_entry("_id", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("_index", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("_routing", v)?;
        }
        if let Some(v) = &self.score {
            map.serialize_entry("_score", v)?;
        }
        if let Some(v) = &self.source {
            map.serialize_entry("_source", v)?;
        }
        if let Some(v) = &self.collate_match {
            map.serialize_entry("collate_match", v)?;
        }
        if let Some(v) = &self.contexts {
            map.serialize_entry("contexts", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self._score {
            map.serialize_entry("score", v)?;
        }
        map.serialize_entry("text", &self.text)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
impl SuggestCompletionOptionsObject {
    #[doc = "Decodes the document body (`_source`) into `T`.\n\nReturns `Ok(None)` when the response carries no `_source` (`_source: false` in the request, or a mapping that disables it), and an error when the document does not match `T`.\n\n`T` may borrow from the response to avoid copying strings: use `#[serde(borrow)] Cow<'a, str>` fields rather than `&'a str`, which cannot hold a string containing JSON escapes."]
    pub fn source_as<'a, T>(&'a self) -> std::result::Result<Option<T>, serde_json::Error>
    where
        T: serde::Deserialize<'a>,
    {
        self.source
            .as_deref()
            .map(|raw| serde_json::from_str(raw.get()))
            .transpose()
    }
}

#[derive(Debug, Clone)]
pub struct SuggestFuzziness {
    pub fuzziness: String,
    pub min_length: i32,
    pub prefix_length: i32,
    pub transpositions: bool,
    pub unicode_aware: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SuggestFuzziness {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SuggestFuzziness;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SuggestFuzziness object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_fuzziness: Option<String> = None;
                let mut f_min_length: Option<i32> = None;
                let mut f_prefix_length: Option<i32> = None;
                let mut f_transpositions: Option<bool> = None;
                let mut f_unicode_aware: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "fuzziness" => {
                            if f_fuzziness.is_some() {
                                return Err(serde::de::Error::duplicate_field("fuzziness"));
                            }
                            f_fuzziness = Some(map.next_value::<String>()?);
                        }
                        "min_length" => {
                            if f_min_length.is_some() {
                                return Err(serde::de::Error::duplicate_field("min_length"));
                            }
                            f_min_length = Some(map.next_value::<i32>()?);
                        }
                        "prefix_length" => {
                            if f_prefix_length.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix_length"));
                            }
                            f_prefix_length = Some(map.next_value::<i32>()?);
                        }
                        "transpositions" => {
                            if f_transpositions.is_some() {
                                return Err(serde::de::Error::duplicate_field("transpositions"));
                            }
                            f_transpositions = Some(map.next_value::<bool>()?);
                        }
                        "unicode_aware" => {
                            if f_unicode_aware.is_some() {
                                return Err(serde::de::Error::duplicate_field("unicode_aware"));
                            }
                            f_unicode_aware = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SuggestFuzziness {
                    fuzziness: f_fuzziness
                        .ok_or_else(|| serde::de::Error::missing_field("fuzziness"))?,
                    min_length: f_min_length
                        .ok_or_else(|| serde::de::Error::missing_field("min_length"))?,
                    prefix_length: f_prefix_length
                        .ok_or_else(|| serde::de::Error::missing_field("prefix_length"))?,
                    transpositions: f_transpositions
                        .ok_or_else(|| serde::de::Error::missing_field("transpositions"))?,
                    unicode_aware: f_unicode_aware
                        .ok_or_else(|| serde::de::Error::missing_field("unicode_aware"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SuggestFuzziness {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("fuzziness", &self.fuzziness)?;
        map.serialize_entry("min_length", &self.min_length)?;
        map.serialize_entry("prefix_length", &self.prefix_length)?;
        map.serialize_entry("transpositions", &self.transpositions)?;
        map.serialize_entry("unicode_aware", &self.unicode_aware)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SuggestSort {
    #[doc = "`frequency`"]
    Frequency,
    #[doc = "`score`"]
    Score,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SuggestSort {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SuggestSort::Frequency => "frequency",
            SuggestSort::Score => "score",
            SuggestSort::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SuggestSort {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SuggestSort {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "frequency" => SuggestSort::Frequency,
            "score" => SuggestSort::Score,
            _ => SuggestSort::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Suggester {
    #[doc = "The global suggest text, which avoids repetition when the same text is used in several suggesters."]
    pub text: Option<String>,
    pub additional: std::collections::BTreeMap<String, crate::types::search::FieldSuggester>,
}
impl<'de> serde::Deserialize<'de> for Suggester {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Suggester;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Suggester object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_text: Option<String> = None;
                let mut seen_text = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::search::FieldSuggester,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "text" => {
                            if seen_text {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            seen_text = true;
                            f_text = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Suggester {
                    text: f_text,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Suggester {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.text {
            map.serialize_entry("text", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TermSuggest {
    pub length: i32,
    pub offset: i32,
    pub text: String,
    pub options: crate::types::search::TermSuggestOptions,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TermSuggest {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TermSuggest;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TermSuggest object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_length: Option<i32> = None;
                let mut f_offset: Option<i32> = None;
                let mut f_text: Option<String> = None;
                let mut f_options: Option<crate::types::search::TermSuggestOptions> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "length" => {
                            if f_length.is_some() {
                                return Err(serde::de::Error::duplicate_field("length"));
                            }
                            f_length = Some(map.next_value::<i32>()?);
                        }
                        "offset" => {
                            if f_offset.is_some() {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            f_offset = Some(map.next_value::<i32>()?);
                        }
                        "text" => {
                            if f_text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            f_text = Some(map.next_value::<String>()?);
                        }
                        "options" => {
                            if f_options.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            f_options =
                                Some(map.next_value::<crate::types::search::TermSuggestOptions>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TermSuggest {
                    length: f_length.ok_or_else(|| serde::de::Error::missing_field("length"))?,
                    offset: f_offset.ok_or_else(|| serde::de::Error::missing_field("offset"))?,
                    text: f_text.ok_or_else(|| serde::de::Error::missing_field("text"))?,
                    options: f_options.ok_or_else(|| serde::de::Error::missing_field("options"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TermSuggest {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("length", &self.length)?;
        map.serialize_entry("offset", &self.offset)?;
        map.serialize_entry("text", &self.text)?;
        map.serialize_entry("options", &self.options)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TermSuggestOption {
    pub collate_match: Option<bool>,
    pub freq: Option<i64>,
    pub highlighted: Option<String>,
    pub score: f64,
    pub text: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TermSuggestOption {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TermSuggestOption;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TermSuggestOption object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_collate_match: Option<bool> = None;
                let mut seen_collate_match = false;
                let mut f_freq: Option<i64> = None;
                let mut seen_freq = false;
                let mut f_highlighted: Option<String> = None;
                let mut seen_highlighted = false;
                let mut f_score: Option<f64> = None;
                let mut f_text: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "collate_match" => {
                            if seen_collate_match {
                                return Err(serde::de::Error::duplicate_field("collate_match"));
                            }
                            seen_collate_match = true;
                            f_collate_match = map.next_value::<Option<bool>>()?;
                        }
                        "freq" => {
                            if seen_freq {
                                return Err(serde::de::Error::duplicate_field("freq"));
                            }
                            seen_freq = true;
                            f_freq = map.next_value::<Option<i64>>()?;
                        }
                        "highlighted" => {
                            if seen_highlighted {
                                return Err(serde::de::Error::duplicate_field("highlighted"));
                            }
                            seen_highlighted = true;
                            f_highlighted = map.next_value::<Option<String>>()?;
                        }
                        "score" => {
                            if f_score.is_some() {
                                return Err(serde::de::Error::duplicate_field("score"));
                            }
                            f_score = Some(map.next_value::<f64>()?);
                        }
                        "text" => {
                            if f_text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            f_text = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TermSuggestOption {
                    collate_match: f_collate_match,
                    freq: f_freq,
                    highlighted: f_highlighted,
                    score: f_score.ok_or_else(|| serde::de::Error::missing_field("score"))?,
                    text: f_text.ok_or_else(|| serde::de::Error::missing_field("text"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TermSuggestOption {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.collate_match {
            map.serialize_entry("collate_match", v)?;
        }
        if let Some(v) = &self.freq {
            map.serialize_entry("freq", v)?;
        }
        if let Some(v) = &self.highlighted {
            map.serialize_entry("highlighted", v)?;
        }
        map.serialize_entry("score", &self.score)?;
        map.serialize_entry("text", &self.text)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum TermSuggestOptions {
    TermSuggestOption(crate::types::search::TermSuggestOption),
    Array(Vec<crate::types::search::TermSuggestOption>),
}
impl<'de> serde::Deserialize<'de> for TermSuggestOptions {
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
            Some(b'{') => serde_json::from_str(text).map(TermSuggestOptions::TermSuggestOption),
            Some(b'[') => serde_json::from_str(text).map(TermSuggestOptions::Array),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a TermSuggestOptions value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct TermSuggester {
    pub analyzer: Option<String>,
    pub field: String,
    pub size: Option<i32>,
    pub lowercase_terms: Option<bool>,
    pub max_edits: Option<i32>,
    pub max_inspections: Option<i32>,
    pub max_term_freq: Option<f64>,
    pub min_doc_freq: Option<f64>,
    pub min_word_length: Option<i32>,
    pub prefix_length: Option<i32>,
    pub shard_size: Option<i32>,
    pub sort: Option<crate::types::search::SuggestSort>,
    pub string_distance: Option<crate::types::search::StringDistance>,
    pub suggest_mode: Option<crate::types::common::SuggestMode>,
    pub text: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TermSuggester {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TermSuggester;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TermSuggester object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_analyzer: Option<String> = None;
                let mut seen_analyzer = false;
                let mut f_field: Option<String> = None;
                let mut f_size: Option<i32> = None;
                let mut seen_size = false;
                let mut f_lowercase_terms: Option<bool> = None;
                let mut seen_lowercase_terms = false;
                let mut f_max_edits: Option<i32> = None;
                let mut seen_max_edits = false;
                let mut f_max_inspections: Option<i32> = None;
                let mut seen_max_inspections = false;
                let mut f_max_term_freq: Option<f64> = None;
                let mut seen_max_term_freq = false;
                let mut f_min_doc_freq: Option<f64> = None;
                let mut seen_min_doc_freq = false;
                let mut f_min_word_length: Option<i32> = None;
                let mut seen_min_word_length = false;
                let mut f_prefix_length: Option<i32> = None;
                let mut seen_prefix_length = false;
                let mut f_shard_size: Option<i32> = None;
                let mut seen_shard_size = false;
                let mut f_sort: Option<crate::types::search::SuggestSort> = None;
                let mut seen_sort = false;
                let mut f_string_distance: Option<crate::types::search::StringDistance> = None;
                let mut seen_string_distance = false;
                let mut f_suggest_mode: Option<crate::types::common::SuggestMode> = None;
                let mut seen_suggest_mode = false;
                let mut f_text: Option<String> = None;
                let mut seen_text = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "analyzer" => {
                            if seen_analyzer {
                                return Err(serde::de::Error::duplicate_field("analyzer"));
                            }
                            seen_analyzer = true;
                            f_analyzer = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<String>()?);
                        }
                        "size" => {
                            if seen_size {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            seen_size = true;
                            f_size = map.next_value::<Option<i32>>()?;
                        }
                        "lowercase_terms" => {
                            if seen_lowercase_terms {
                                return Err(serde::de::Error::duplicate_field("lowercase_terms"));
                            }
                            seen_lowercase_terms = true;
                            f_lowercase_terms = map.next_value::<Option<bool>>()?;
                        }
                        "max_edits" => {
                            if seen_max_edits {
                                return Err(serde::de::Error::duplicate_field("max_edits"));
                            }
                            seen_max_edits = true;
                            f_max_edits = map.next_value::<Option<i32>>()?;
                        }
                        "max_inspections" => {
                            if seen_max_inspections {
                                return Err(serde::de::Error::duplicate_field("max_inspections"));
                            }
                            seen_max_inspections = true;
                            f_max_inspections = map.next_value::<Option<i32>>()?;
                        }
                        "max_term_freq" => {
                            if seen_max_term_freq {
                                return Err(serde::de::Error::duplicate_field("max_term_freq"));
                            }
                            seen_max_term_freq = true;
                            f_max_term_freq = map.next_value::<Option<f64>>()?;
                        }
                        "min_doc_freq" => {
                            if seen_min_doc_freq {
                                return Err(serde::de::Error::duplicate_field("min_doc_freq"));
                            }
                            seen_min_doc_freq = true;
                            f_min_doc_freq = map.next_value::<Option<f64>>()?;
                        }
                        "min_word_length" => {
                            if seen_min_word_length {
                                return Err(serde::de::Error::duplicate_field("min_word_length"));
                            }
                            seen_min_word_length = true;
                            f_min_word_length = map.next_value::<Option<i32>>()?;
                        }
                        "prefix_length" => {
                            if seen_prefix_length {
                                return Err(serde::de::Error::duplicate_field("prefix_length"));
                            }
                            seen_prefix_length = true;
                            f_prefix_length = map.next_value::<Option<i32>>()?;
                        }
                        "shard_size" => {
                            if seen_shard_size {
                                return Err(serde::de::Error::duplicate_field("shard_size"));
                            }
                            seen_shard_size = true;
                            f_shard_size = map.next_value::<Option<i32>>()?;
                        }
                        "sort" => {
                            if seen_sort {
                                return Err(serde::de::Error::duplicate_field("sort"));
                            }
                            seen_sort = true;
                            f_sort =
                                map.next_value::<Option<crate::types::search::SuggestSort>>()?;
                        }
                        "string_distance" => {
                            if seen_string_distance {
                                return Err(serde::de::Error::duplicate_field("string_distance"));
                            }
                            seen_string_distance = true;
                            f_string_distance =
                                map.next_value::<Option<crate::types::search::StringDistance>>()?;
                        }
                        "suggest_mode" => {
                            if seen_suggest_mode {
                                return Err(serde::de::Error::duplicate_field("suggest_mode"));
                            }
                            seen_suggest_mode = true;
                            f_suggest_mode =
                                map.next_value::<Option<crate::types::common::SuggestMode>>()?;
                        }
                        "text" => {
                            if seen_text {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            seen_text = true;
                            f_text = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TermSuggester {
                    analyzer: f_analyzer,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    size: f_size,
                    lowercase_terms: f_lowercase_terms,
                    max_edits: f_max_edits,
                    max_inspections: f_max_inspections,
                    max_term_freq: f_max_term_freq,
                    min_doc_freq: f_min_doc_freq,
                    min_word_length: f_min_word_length,
                    prefix_length: f_prefix_length,
                    shard_size: f_shard_size,
                    sort: f_sort,
                    string_distance: f_string_distance,
                    suggest_mode: f_suggest_mode,
                    text: f_text,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TermSuggester {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.analyzer {
            map.serialize_entry("analyzer", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.size {
            map.serialize_entry("size", v)?;
        }
        if let Some(v) = &self.lowercase_terms {
            map.serialize_entry("lowercase_terms", v)?;
        }
        if let Some(v) = &self.max_edits {
            map.serialize_entry("max_edits", v)?;
        }
        if let Some(v) = &self.max_inspections {
            map.serialize_entry("max_inspections", v)?;
        }
        if let Some(v) = &self.max_term_freq {
            map.serialize_entry("max_term_freq", v)?;
        }
        if let Some(v) = &self.min_doc_freq {
            map.serialize_entry("min_doc_freq", v)?;
        }
        if let Some(v) = &self.min_word_length {
            map.serialize_entry("min_word_length", v)?;
        }
        if let Some(v) = &self.prefix_length {
            map.serialize_entry("prefix_length", v)?;
        }
        if let Some(v) = &self.shard_size {
            map.serialize_entry("shard_size", v)?;
        }
        if let Some(v) = &self.sort {
            map.serialize_entry("sort", v)?;
        }
        if let Some(v) = &self.string_distance {
            map.serialize_entry("string_distance", v)?;
        }
        if let Some(v) = &self.suggest_mode {
            map.serialize_entry("suggest_mode", v)?;
        }
        if let Some(v) = &self.text {
            map.serialize_entry("text", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TotalHits {
    pub relation: crate::types::search::TotalHitsRelation,
    pub value: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TotalHits {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TotalHits;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TotalHits object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_relation: Option<crate::types::search::TotalHitsRelation> = None;
                let mut f_value: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "relation" => {
                            if f_relation.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            f_relation =
                                Some(map.next_value::<crate::types::search::TotalHitsRelation>()?);
                        }
                        "value" => {
                            if f_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            f_value = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TotalHits {
                    relation: f_relation
                        .ok_or_else(|| serde::de::Error::missing_field("relation"))?,
                    value: f_value.ok_or_else(|| serde::de::Error::missing_field("value"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TotalHits {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("relation", &self.relation)?;
        map.serialize_entry("value", &self.value)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TotalHitsRelation {
    #[doc = "`eq`"]
    Eq,
    #[doc = "`gte`"]
    Gte,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TotalHitsRelation {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TotalHitsRelation::Eq => "eq",
            TotalHitsRelation::Gte => "gte",
            TotalHitsRelation::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TotalHitsRelation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TotalHitsRelation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "eq" => TotalHitsRelation::Eq,
            "gte" => TotalHitsRelation::Gte,
            _ => TotalHitsRelation::Unknown(s),
        })
    }
}

#[doc = "The number of hits matching the query. When `true`, the exact number of hits is returned at the cost of some performance. When `false`, the response does not include the total number of hits matching the query. Default is `10,000` hits."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum TrackHits {
    Enabled(bool),
    Count(i32),
}
impl<'de> serde::Deserialize<'de> for TrackHits {
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
            Some(b't') | Some(b'f') => serde_json::from_str(text).map(TrackHits::Enabled),
            Some(b'-') | Some(b'0'..=b'9') => serde_json::from_str(text).map(TrackHits::Count),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a TrackHits value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}
