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

//! Response types of the `insights_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GroupingType {
    #[doc = "`NONE` (also accepted: `none`)"]
    None,
    #[doc = "`SIMILARITY` (also accepted: `similarity`)"]
    Similarity,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl GroupingType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            GroupingType::None => "NONE",
            GroupingType::Similarity => "SIMILARITY",
            GroupingType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for GroupingType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GroupingType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "NONE" | "none" => GroupingType::None,
            "SIMILARITY" | "similarity" => GroupingType::Similarity,
            _ => GroupingType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Measurement {
    pub aggregation_type: Option<String>,
    pub count: Option<i64>,
    pub number: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Measurement {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Measurement;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Measurement object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_aggregation_type: Option<String> = None;
                let mut seen_aggregation_type = false;
                let mut f_count: Option<i64> = None;
                let mut seen_count = false;
                let mut f_number: Option<i64> = None;
                let mut seen_number = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "aggregationType" => {
                            if seen_aggregation_type {
                                return Err(serde::de::Error::duplicate_field("aggregationType"));
                            }
                            seen_aggregation_type = true;
                            f_aggregation_type = map.next_value::<Option<String>>()?;
                        }
                        "count" => {
                            if seen_count {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            seen_count = true;
                            f_count = map.next_value::<Option<i64>>()?;
                        }
                        "number" => {
                            if seen_number {
                                return Err(serde::de::Error::duplicate_field("number"));
                            }
                            seen_number = true;
                            f_number = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Measurement {
                    aggregation_type: f_aggregation_type,
                    count: f_count,
                    number: f_number,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Measurement {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.aggregation_type {
            map.serialize_entry("aggregationType", v)?;
        }
        if let Some(v) = &self.count {
            map.serialize_entry("count", v)?;
        }
        if let Some(v) = &self.number {
            map.serialize_entry("number", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Measurements {
    pub cpu: Option<crate::types::insights_common::Measurement>,
    pub latency: Option<crate::types::insights_common::Measurement>,
    pub memory: Option<crate::types::insights_common::Measurement>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Measurements {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Measurements;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Measurements object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cpu: Option<crate::types::insights_common::Measurement> = None;
                let mut seen_cpu = false;
                let mut f_latency: Option<crate::types::insights_common::Measurement> = None;
                let mut seen_latency = false;
                let mut f_memory: Option<crate::types::insights_common::Measurement> = None;
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
                                .next_value::<Option<crate::types::insights_common::Measurement>>(
                                )?;
                        }
                        "latency" => {
                            if seen_latency {
                                return Err(serde::de::Error::duplicate_field("latency"));
                            }
                            seen_latency = true;
                            f_latency = map
                                .next_value::<Option<crate::types::insights_common::Measurement>>(
                                )?;
                        }
                        "memory" => {
                            if seen_memory {
                                return Err(serde::de::Error::duplicate_field("memory"));
                            }
                            seen_memory = true;
                            f_memory = map
                                .next_value::<Option<crate::types::insights_common::Measurement>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Measurements {
                    cpu: f_cpu,
                    latency: f_latency,
                    memory: f_memory,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Measurements {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cpu {
            map.serialize_entry("cpu", v)?;
        }
        if let Some(v) = &self.latency {
            map.serialize_entry("latency", v)?;
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
pub struct Source {
    pub source: Option<crate::types::search::SourceConfig>,
    #[doc = "Defines the aggregations that are run as part of the search request."]
    pub aggregations: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub collapse: Option<crate::types::search::FieldCollapse>,
    #[doc = "Array of wildcard (`*`) patterns. The request returns doc values for field names matching these patterns in the `hits.fields` property of the response."]
    pub docvalue_fields: Option<Vec<crate::types::common_query_dsl::FieldAndFormat>>,
    #[doc = "If `true`, returns detailed information about score computation as part of a hit."]
    pub explain: Option<bool>,
    #[doc = "Configuration of search extensions defined by OpenSearch plugins."]
    pub ext: Option<
        std::collections::BTreeMap<
            String,
            std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
        >,
    >,
    #[doc = "Array of wildcard (`*`) patterns. The request returns values for field names matching these patterns in the `hits.fields` property of the response."]
    pub fields: Option<Vec<crate::types::common_query_dsl::FieldAndFormat>>,
    #[doc = "Starting document offset. Needs to be non-negative. By default, you cannot page through more than 10,000 hits using the `from` and `size` parameters. To page through more hits, use the `search_after` parameter."]
    pub from: Option<f64>,
    pub highlight: Option<crate::types::search::Highlight>,
    #[doc = "Boosts the `_score` of documents from specified indexes."]
    pub indices_boost: Option<Vec<std::collections::BTreeMap<String, f64>>>,
    #[doc = "Minimum `_score` for matching documents. Documents with a lower `_score` are not included in the search results."]
    pub min_score: Option<f64>,
    pub pit: Option<crate::types::search::PointInTimeReference>,
    pub post_filter: Option<crate::types::common_query_dsl::QueryContainer>,
    #[doc = "Set to `true` to return detailed timing information about the execution of individual components in a search request. NOTE: This is a debugging tool and adds significant overhead to search execution."]
    pub profile: Option<bool>,
    pub query: Option<crate::types::common_query_dsl::QueryContainer>,
    #[doc = "Retrieve a script evaluation (based on different fields) for each hit."]
    pub script_fields:
        Option<std::collections::BTreeMap<String, crate::types::common::ScriptField>>,
    pub search_after: Option<crate::types::common::SortResults>,
    #[doc = "If `true`, returns sequence number and primary term of the last modification of each hit."]
    pub seq_no_primary_term: Option<bool>,
    #[doc = "The number of hits to return. By default, you cannot page through more than 10,000 hits using the `from` and `size` parameters. To page through more hits, use the `search_after` parameter."]
    pub size: Option<f64>,
    pub slice: Option<crate::types::common::SlicedScroll>,
    pub sort: Option<crate::types::common::Sort>,
    #[doc = "Stats groups to associate with the search. Each group maintains a statistics aggregation for its associated searches. You can retrieve these stats using the indexes stats API."]
    pub stats: Option<Vec<String>>,
    pub stored_fields: Option<crate::types::common::Fields>,
    pub suggest: Option<crate::types::search::Suggester>,
    #[doc = "Maximum number of documents to collect for each shard. If a query reaches this limit, OpenSearch terminates the query early. OpenSearch collects documents before sorting. Use with caution. OpenSearch applies this parameter to each shard handling the request. When possible, let OpenSearch perform early termination automatically. Avoid specifying this parameter for requests that target data streams with backing indexes across multiple data tiers. If set to `0` (default), the query does not terminate early."]
    pub terminate_after: Option<i32>,
    #[doc = "Specifies the period of time to wait for a response from each shard. If no response is received before the timeout expires, the request fails and returns an error. Defaults to no timeout."]
    pub timeout: Option<String>,
    #[doc = "If `true`, calculate and return document scores, even if the scores are not used for sorting."]
    pub track_scores: Option<bool>,
    pub track_total_hits: Option<crate::types::search::TrackHits>,
    #[doc = "If `true`, returns document version as part of a hit."]
    pub version: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Source {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Source;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Source object")
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
                let mut f_aggregations: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_aggregations = false;
                let mut f_collapse: Option<crate::types::search::FieldCollapse> = None;
                let mut seen_collapse = false;
                let mut f_docvalue_fields: Option<
                    Vec<crate::types::common_query_dsl::FieldAndFormat>,
                > = None;
                let mut seen_docvalue_fields = false;
                let mut f_explain: Option<bool> = None;
                let mut seen_explain = false;
                let mut f_ext: Option<
                    std::collections::BTreeMap<
                        String,
                        std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                    >,
                > = None;
                let mut seen_ext = false;
                let mut f_fields: Option<Vec<crate::types::common_query_dsl::FieldAndFormat>> =
                    None;
                let mut seen_fields = false;
                let mut f_from: Option<f64> = None;
                let mut seen_from = false;
                let mut f_highlight: Option<crate::types::search::Highlight> = None;
                let mut seen_highlight = false;
                let mut f_indices_boost: Option<Vec<std::collections::BTreeMap<String, f64>>> =
                    None;
                let mut seen_indices_boost = false;
                let mut f_min_score: Option<f64> = None;
                let mut seen_min_score = false;
                let mut f_pit: Option<crate::types::search::PointInTimeReference> = None;
                let mut seen_pit = false;
                let mut f_post_filter: Option<crate::types::common_query_dsl::QueryContainer> =
                    None;
                let mut seen_post_filter = false;
                let mut f_profile: Option<bool> = None;
                let mut seen_profile = false;
                let mut f_query: Option<crate::types::common_query_dsl::QueryContainer> = None;
                let mut seen_query = false;
                let mut f_script_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common::ScriptField>,
                > = None;
                let mut seen_script_fields = false;
                let mut f_search_after: Option<crate::types::common::SortResults> = None;
                let mut seen_search_after = false;
                let mut f_seq_no_primary_term: Option<bool> = None;
                let mut seen_seq_no_primary_term = false;
                let mut f_size: Option<f64> = None;
                let mut seen_size = false;
                let mut f_slice: Option<crate::types::common::SlicedScroll> = None;
                let mut seen_slice = false;
                let mut f_sort: Option<crate::types::common::Sort> = None;
                let mut seen_sort = false;
                let mut f_stats: Option<Vec<String>> = None;
                let mut seen_stats = false;
                let mut f_stored_fields: Option<crate::types::common::Fields> = None;
                let mut seen_stored_fields = false;
                let mut f_suggest: Option<crate::types::search::Suggester> = None;
                let mut seen_suggest = false;
                let mut f_terminate_after: Option<i32> = None;
                let mut seen_terminate_after = false;
                let mut f_timeout: Option<String> = None;
                let mut seen_timeout = false;
                let mut f_track_scores: Option<bool> = None;
                let mut seen_track_scores = false;
                let mut f_track_total_hits: Option<crate::types::search::TrackHits> = None;
                let mut seen_track_total_hits = false;
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
                        "aggregations" => {
                            if seen_aggregations {
                                return Err(serde::de::Error::duplicate_field("aggregations"));
                            }
                            seen_aggregations = true;
                            f_aggregations = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "collapse" => {
                            if seen_collapse {
                                return Err(serde::de::Error::duplicate_field("collapse"));
                            }
                            seen_collapse = true;
                            f_collapse =
                                map.next_value::<Option<crate::types::search::FieldCollapse>>()?;
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
                        "ext" => {
                            if seen_ext {
                                return Err(serde::de::Error::duplicate_field("ext"));
                            }
                            seen_ext = true;
                            f_ext = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    std::collections::BTreeMap<
                                        String,
                                        Box<serde_json::value::RawValue>,
                                    >,
                                >,
                            >>()?;
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
                            f_from = map.next_value::<Option<f64>>()?;
                        }
                        "highlight" => {
                            if seen_highlight {
                                return Err(serde::de::Error::duplicate_field("highlight"));
                            }
                            seen_highlight = true;
                            f_highlight =
                                map.next_value::<Option<crate::types::search::Highlight>>()?;
                        }
                        "indices_boost" => {
                            if seen_indices_boost {
                                return Err(serde::de::Error::duplicate_field("indices_boost"));
                            }
                            seen_indices_boost = true;
                            f_indices_boost = map
                                .next_value::<Option<Vec<std::collections::BTreeMap<String, f64>>>>(
                                )?;
                        }
                        "min_score" => {
                            if seen_min_score {
                                return Err(serde::de::Error::duplicate_field("min_score"));
                            }
                            seen_min_score = true;
                            f_min_score = map.next_value::<Option<f64>>()?;
                        }
                        "pit" => {
                            if seen_pit {
                                return Err(serde::de::Error::duplicate_field("pit"));
                            }
                            seen_pit = true;
                            f_pit = map
                                .next_value::<Option<crate::types::search::PointInTimeReference>>(
                                )?;
                        }
                        "post_filter" => {
                            if seen_post_filter {
                                return Err(serde::de::Error::duplicate_field("post_filter"));
                            }
                            seen_post_filter = true;
                            f_post_filter = map.next_value::<Option<crate::types::common_query_dsl::QueryContainer>>()?;
                        }
                        "profile" => {
                            if seen_profile {
                                return Err(serde::de::Error::duplicate_field("profile"));
                            }
                            seen_profile = true;
                            f_profile = map.next_value::<Option<bool>>()?;
                        }
                        "query" => {
                            if seen_query {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            seen_query = true;
                            f_query = map.next_value::<Option<crate::types::common_query_dsl::QueryContainer>>()?;
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
                        "search_after" => {
                            if seen_search_after {
                                return Err(serde::de::Error::duplicate_field("search_after"));
                            }
                            seen_search_after = true;
                            f_search_after =
                                map.next_value::<Option<crate::types::common::SortResults>>()?;
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
                            f_size = map.next_value::<Option<f64>>()?;
                        }
                        "slice" => {
                            if seen_slice {
                                return Err(serde::de::Error::duplicate_field("slice"));
                            }
                            seen_slice = true;
                            f_slice =
                                map.next_value::<Option<crate::types::common::SlicedScroll>>()?;
                        }
                        "sort" => {
                            if seen_sort {
                                return Err(serde::de::Error::duplicate_field("sort"));
                            }
                            seen_sort = true;
                            f_sort = map.next_value::<Option<crate::types::common::Sort>>()?;
                        }
                        "stats" => {
                            if seen_stats {
                                return Err(serde::de::Error::duplicate_field("stats"));
                            }
                            seen_stats = true;
                            f_stats = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "stored_fields" => {
                            if seen_stored_fields {
                                return Err(serde::de::Error::duplicate_field("stored_fields"));
                            }
                            seen_stored_fields = true;
                            f_stored_fields =
                                map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "suggest" => {
                            if seen_suggest {
                                return Err(serde::de::Error::duplicate_field("suggest"));
                            }
                            seen_suggest = true;
                            f_suggest =
                                map.next_value::<Option<crate::types::search::Suggester>>()?;
                        }
                        "terminate_after" => {
                            if seen_terminate_after {
                                return Err(serde::de::Error::duplicate_field("terminate_after"));
                            }
                            seen_terminate_after = true;
                            f_terminate_after = map.next_value::<Option<i32>>()?;
                        }
                        "timeout" => {
                            if seen_timeout {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            seen_timeout = true;
                            f_timeout = map.next_value::<Option<String>>()?;
                        }
                        "track_scores" => {
                            if seen_track_scores {
                                return Err(serde::de::Error::duplicate_field("track_scores"));
                            }
                            seen_track_scores = true;
                            f_track_scores = map.next_value::<Option<bool>>()?;
                        }
                        "track_total_hits" => {
                            if seen_track_total_hits {
                                return Err(serde::de::Error::duplicate_field("track_total_hits"));
                            }
                            seen_track_total_hits = true;
                            f_track_total_hits =
                                map.next_value::<Option<crate::types::search::TrackHits>>()?;
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
                Ok(Source {
                    source: f_source,
                    aggregations: f_aggregations,
                    collapse: f_collapse,
                    docvalue_fields: f_docvalue_fields,
                    explain: f_explain,
                    ext: f_ext,
                    fields: f_fields,
                    from: f_from,
                    highlight: f_highlight,
                    indices_boost: f_indices_boost,
                    min_score: f_min_score,
                    pit: f_pit,
                    post_filter: f_post_filter,
                    profile: f_profile,
                    query: f_query,
                    script_fields: f_script_fields,
                    search_after: f_search_after,
                    seq_no_primary_term: f_seq_no_primary_term,
                    size: f_size,
                    slice: f_slice,
                    sort: f_sort,
                    stats: f_stats,
                    stored_fields: f_stored_fields,
                    suggest: f_suggest,
                    terminate_after: f_terminate_after,
                    timeout: f_timeout,
                    track_scores: f_track_scores,
                    track_total_hits: f_track_total_hits,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Source {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.source {
            map.serialize_entry("_source", v)?;
        }
        if let Some(v) = &self.aggregations {
            map.serialize_entry("aggregations", v)?;
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
        if let Some(v) = &self.ext {
            map.serialize_entry("ext", v)?;
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
        if let Some(v) = &self.indices_boost {
            map.serialize_entry("indices_boost", v)?;
        }
        if let Some(v) = &self.min_score {
            map.serialize_entry("min_score", v)?;
        }
        if let Some(v) = &self.pit {
            map.serialize_entry("pit", v)?;
        }
        if let Some(v) = &self.post_filter {
            map.serialize_entry("post_filter", v)?;
        }
        if let Some(v) = &self.profile {
            map.serialize_entry("profile", v)?;
        }
        if let Some(v) = &self.query {
            map.serialize_entry("query", v)?;
        }
        if let Some(v) = &self.script_fields {
            map.serialize_entry("script_fields", v)?;
        }
        if let Some(v) = &self.search_after {
            map.serialize_entry("search_after", v)?;
        }
        if let Some(v) = &self.seq_no_primary_term {
            map.serialize_entry("seq_no_primary_term", v)?;
        }
        if let Some(v) = &self.size {
            map.serialize_entry("size", v)?;
        }
        if let Some(v) = &self.slice {
            map.serialize_entry("slice", v)?;
        }
        if let Some(v) = &self.sort {
            map.serialize_entry("sort", v)?;
        }
        if let Some(v) = &self.stats {
            map.serialize_entry("stats", v)?;
        }
        if let Some(v) = &self.stored_fields {
            map.serialize_entry("stored_fields", v)?;
        }
        if let Some(v) = &self.suggest {
            map.serialize_entry("suggest", v)?;
        }
        if let Some(v) = &self.terminate_after {
            map.serialize_entry("terminate_after", v)?;
        }
        if let Some(v) = &self.timeout {
            map.serialize_entry("timeout", v)?;
        }
        if let Some(v) = &self.track_scores {
            map.serialize_entry("track_scores", v)?;
        }
        if let Some(v) = &self.track_total_hits {
            map.serialize_entry("track_total_hits", v)?;
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
pub struct TaskResourceUsage {
    #[doc = "The CPU time used in nanoseconds."]
    pub cpu_time_in_nanos: Option<i64>,
    #[doc = "The memory usage in bytes."]
    pub memory_in_bytes: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TaskResourceUsage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TaskResourceUsage;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TaskResourceUsage object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cpu_time_in_nanos: Option<i64> = None;
                let mut seen_cpu_time_in_nanos = false;
                let mut f_memory_in_bytes: Option<i64> = None;
                let mut seen_memory_in_bytes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cpu_time_in_nanos" => {
                            if seen_cpu_time_in_nanos {
                                return Err(serde::de::Error::duplicate_field("cpu_time_in_nanos"));
                            }
                            seen_cpu_time_in_nanos = true;
                            f_cpu_time_in_nanos = map.next_value::<Option<i64>>()?;
                        }
                        "memory_in_bytes" => {
                            if seen_memory_in_bytes {
                                return Err(serde::de::Error::duplicate_field("memory_in_bytes"));
                            }
                            seen_memory_in_bytes = true;
                            f_memory_in_bytes = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TaskResourceUsage {
                    cpu_time_in_nanos: f_cpu_time_in_nanos,
                    memory_in_bytes: f_memory_in_bytes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TaskResourceUsage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cpu_time_in_nanos {
            map.serialize_entry("cpu_time_in_nanos", v)?;
        }
        if let Some(v) = &self.memory_in_bytes {
            map.serialize_entry("memory_in_bytes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TaskResourceUsages {
    #[doc = "The action type of the task."]
    pub action: Option<String>,
    #[doc = "The node ID where the task was executed."]
    pub node_id: Option<String>,
    #[doc = "The parent task ID."]
    pub parent_task_id: Option<i64>,
    #[doc = "The task ID."]
    pub task_id: Option<i64>,
    pub task_resource_usage: Option<crate::types::insights_common::TaskResourceUsage>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TaskResourceUsages {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TaskResourceUsages;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TaskResourceUsages object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_action: Option<String> = None;
                let mut seen_action = false;
                let mut f_node_id: Option<String> = None;
                let mut seen_node_id = false;
                let mut f_parent_task_id: Option<i64> = None;
                let mut seen_parent_task_id = false;
                let mut f_task_id: Option<i64> = None;
                let mut seen_task_id = false;
                let mut f_task_resource_usage: Option<
                    crate::types::insights_common::TaskResourceUsage,
                > = None;
                let mut seen_task_resource_usage = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "action" => {
                            if seen_action {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            seen_action = true;
                            f_action = map.next_value::<Option<String>>()?;
                        }
                        "nodeId" => {
                            if seen_node_id {
                                return Err(serde::de::Error::duplicate_field("nodeId"));
                            }
                            seen_node_id = true;
                            f_node_id = map.next_value::<Option<String>>()?;
                        }
                        "parentTaskId" => {
                            if seen_parent_task_id {
                                return Err(serde::de::Error::duplicate_field("parentTaskId"));
                            }
                            seen_parent_task_id = true;
                            f_parent_task_id = map.next_value::<Option<i64>>()?;
                        }
                        "taskId" => {
                            if seen_task_id {
                                return Err(serde::de::Error::duplicate_field("taskId"));
                            }
                            seen_task_id = true;
                            f_task_id = map.next_value::<Option<i64>>()?;
                        }
                        "taskResourceUsage" => {
                            if seen_task_resource_usage {
                                return Err(serde::de::Error::duplicate_field("taskResourceUsage"));
                            }
                            seen_task_resource_usage = true;
                            f_task_resource_usage = map.next_value::<Option<crate::types::insights_common::TaskResourceUsage>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TaskResourceUsages {
                    action: f_action,
                    node_id: f_node_id,
                    parent_task_id: f_parent_task_id,
                    task_id: f_task_id,
                    task_resource_usage: f_task_resource_usage,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TaskResourceUsages {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.action {
            map.serialize_entry("action", v)?;
        }
        if let Some(v) = &self.node_id {
            map.serialize_entry("nodeId", v)?;
        }
        if let Some(v) = &self.parent_task_id {
            map.serialize_entry("parentTaskId", v)?;
        }
        if let Some(v) = &self.task_id {
            map.serialize_entry("taskId", v)?;
        }
        if let Some(v) = &self.task_resource_usage {
            map.serialize_entry("taskResourceUsage", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TopQueriesResponse {
    pub top_queries: Vec<crate::types::insights_common::TopQuery>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TopQueriesResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TopQueriesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TopQueriesResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_top_queries: Option<Vec<crate::types::insights_common::TopQuery>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "top_queries" => {
                            if f_top_queries.is_some() {
                                return Err(serde::de::Error::duplicate_field("top_queries"));
                            }
                            f_top_queries = Some(
                                map.next_value::<Vec<crate::types::insights_common::TopQuery>>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TopQueriesResponse {
                    top_queries: f_top_queries
                        .ok_or_else(|| serde::de::Error::missing_field("top_queries"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TopQueriesResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("top_queries", &self.top_queries)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TopQuery {
    pub group_by: Option<crate::types::insights_common::GroupingType>,
    pub id: Option<String>,
    #[doc = "The indexes involved in the query."]
    pub indices: Option<Vec<String>>,
    #[doc = "Additional labels for the query."]
    pub labels: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub measurements: Option<crate::types::insights_common::Measurements>,
    #[doc = "The node ID associated with the query."]
    pub node_id: Option<String>,
    pub phase_latency_map:
        Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The hash code of the query."]
    pub query_hashcode: Option<String>,
    #[doc = "The search query type (for example, `query_then_fetch`)."]
    pub search_type: Option<String>,
    pub source: Option<crate::types::insights_common::Source>,
    pub task_resource_usages: Option<Vec<crate::types::insights_common::TaskResourceUsages>>,
    #[doc = "The timestamp of the query execution."]
    pub timestamp: Option<i64>,
    #[doc = "The total number of shards involved in the query."]
    pub total_shards: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TopQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TopQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TopQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_group_by: Option<crate::types::insights_common::GroupingType> = None;
                let mut seen_group_by = false;
                let mut f_id: Option<String> = None;
                let mut seen_id = false;
                let mut f_indices: Option<Vec<String>> = None;
                let mut seen_indices = false;
                let mut f_labels: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_labels = false;
                let mut f_measurements: Option<crate::types::insights_common::Measurements> = None;
                let mut seen_measurements = false;
                let mut f_node_id: Option<String> = None;
                let mut seen_node_id = false;
                let mut f_phase_latency_map: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_phase_latency_map = false;
                let mut f_query_hashcode: Option<String> = None;
                let mut seen_query_hashcode = false;
                let mut f_search_type: Option<String> = None;
                let mut seen_search_type = false;
                let mut f_source: Option<crate::types::insights_common::Source> = None;
                let mut seen_source = false;
                let mut f_task_resource_usages: Option<
                    Vec<crate::types::insights_common::TaskResourceUsages>,
                > = None;
                let mut seen_task_resource_usages = false;
                let mut f_timestamp: Option<i64> = None;
                let mut seen_timestamp = false;
                let mut f_total_shards: Option<i64> = None;
                let mut seen_total_shards = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "group_by" => {
                            if seen_group_by {
                                return Err(serde::de::Error::duplicate_field("group_by"));
                            }
                            seen_group_by = true;
                            f_group_by = map
                                .next_value::<Option<crate::types::insights_common::GroupingType>>(
                                )?;
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
                            f_indices = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "labels" => {
                            if seen_labels {
                                return Err(serde::de::Error::duplicate_field("labels"));
                            }
                            seen_labels = true;
                            f_labels = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "measurements" => {
                            if seen_measurements {
                                return Err(serde::de::Error::duplicate_field("measurements"));
                            }
                            seen_measurements = true;
                            f_measurements = map
                                .next_value::<Option<crate::types::insights_common::Measurements>>(
                                )?;
                        }
                        "node_id" => {
                            if seen_node_id {
                                return Err(serde::de::Error::duplicate_field("node_id"));
                            }
                            seen_node_id = true;
                            f_node_id = map.next_value::<Option<String>>()?;
                        }
                        "phase_latency_map" => {
                            if seen_phase_latency_map {
                                return Err(serde::de::Error::duplicate_field("phase_latency_map"));
                            }
                            seen_phase_latency_map = true;
                            f_phase_latency_map = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "query_hashcode" => {
                            if seen_query_hashcode {
                                return Err(serde::de::Error::duplicate_field("query_hashcode"));
                            }
                            seen_query_hashcode = true;
                            f_query_hashcode = map.next_value::<Option<String>>()?;
                        }
                        "search_type" => {
                            if seen_search_type {
                                return Err(serde::de::Error::duplicate_field("search_type"));
                            }
                            seen_search_type = true;
                            f_search_type = map.next_value::<Option<String>>()?;
                        }
                        "source" => {
                            if seen_source {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            seen_source = true;
                            f_source =
                                map.next_value::<Option<crate::types::insights_common::Source>>()?;
                        }
                        "task_resource_usages" => {
                            if seen_task_resource_usages {
                                return Err(serde::de::Error::duplicate_field(
                                    "task_resource_usages",
                                ));
                            }
                            seen_task_resource_usages = true;
                            f_task_resource_usages = map.next_value::<Option<Vec<crate::types::insights_common::TaskResourceUsages>>>()?;
                        }
                        "timestamp" => {
                            if seen_timestamp {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            seen_timestamp = true;
                            f_timestamp = map.next_value::<Option<i64>>()?;
                        }
                        "total_shards" => {
                            if seen_total_shards {
                                return Err(serde::de::Error::duplicate_field("total_shards"));
                            }
                            seen_total_shards = true;
                            f_total_shards = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TopQuery {
                    group_by: f_group_by,
                    id: f_id,
                    indices: f_indices,
                    labels: f_labels,
                    measurements: f_measurements,
                    node_id: f_node_id,
                    phase_latency_map: f_phase_latency_map,
                    query_hashcode: f_query_hashcode,
                    search_type: f_search_type,
                    source: f_source,
                    task_resource_usages: f_task_resource_usages,
                    timestamp: f_timestamp,
                    total_shards: f_total_shards,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TopQuery {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.group_by {
            map.serialize_entry("group_by", v)?;
        }
        if let Some(v) = &self.id {
            map.serialize_entry("id", v)?;
        }
        if let Some(v) = &self.indices {
            map.serialize_entry("indices", v)?;
        }
        if let Some(v) = &self.labels {
            map.serialize_entry("labels", v)?;
        }
        if let Some(v) = &self.measurements {
            map.serialize_entry("measurements", v)?;
        }
        if let Some(v) = &self.node_id {
            map.serialize_entry("node_id", v)?;
        }
        if let Some(v) = &self.phase_latency_map {
            map.serialize_entry("phase_latency_map", v)?;
        }
        if let Some(v) = &self.query_hashcode {
            map.serialize_entry("query_hashcode", v)?;
        }
        if let Some(v) = &self.search_type {
            map.serialize_entry("search_type", v)?;
        }
        if let Some(v) = &self.source {
            map.serialize_entry("source", v)?;
        }
        if let Some(v) = &self.task_resource_usages {
            map.serialize_entry("task_resource_usages", v)?;
        }
        if let Some(v) = &self.timestamp {
            map.serialize_entry("timestamp", v)?;
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
