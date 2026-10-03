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

//! Response types of the `knn` specification namespace

#![allow(missing_docs)]

pub type GetModelResponse = std::collections::BTreeMap<String, serde_json::Value>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchModelsResponse {
    #[serde(rename = "_clusters")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<crate::types::common::ClusterStatistics>,
    #[serde(rename = "_scroll_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scroll_id: Option<crate::types::common::ScrollId>,
    #[serde(rename = "_shards")]
    pub shards: crate::types::common::ShardStatistics,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregations:
        Option<std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>>,
    pub hits: crate::types::knn::SearchModelsResponseHits,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_reduce_phases: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase_took: Option<crate::types::common::PhaseTook>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pit_id: Option<crate::types::common::Id>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processor_results: Option<Vec<crate::types::search::ProcessorExecutionDetail>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<crate::types::search::Profile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggest: Option<
        std::collections::BTreeMap<
            String,
            Vec<crate::types::knn::SearchModelsResponseSuggestValueItem>,
        >,
    >,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminated_early: Option<bool>,
    pub timed_out: bool,
    pub took: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchModelsResponseHits {
    pub hits: Vec<crate::types::knn::SearchModelsResponseHitsHitsItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_score: Option<f64>,
    #[doc = "The total number of hits, present only if `track_total_hits` is not set to `false` in the search request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::knn::SearchModelsResponseHitsTotal>,
}

#[derive(Debug, Clone)]
pub struct SearchModelsResponseHitsHitsItem {
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
    pub matched_queries: Option<crate::types::knn::SearchModelsResponseHitsHitsItemMatchedQueries>,
    pub sort: Option<crate::types::common::SortResults>,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SearchModelsResponseHitsHitsItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SearchModelsResponseHitsHitsItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchModelsResponseHitsHitsItem object")
            }
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_explanation: Option<crate::types::explain::Explanation> = None;
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut f_ignored: Option<Vec<String>> = None;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut f_nested: Option<crate::types::search::NestedIdentity> = None;
                let mut f_node: Option<String> = None;
                let mut f_primary_term: Option<i64> = None;
                let mut f_routing: Option<String> = None;
                let mut f_score: Option<f64> = None;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut f_shard: Option<String> = None;
                let mut f_source: Option<Box<serde_json::value::RawValue>> = None;
                let mut f_type: Option<crate::types::common::Type> = None;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut f_highlight: Option<std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut f_ignored_field_values: Option<
                    std::collections::BTreeMap<String, Vec<String>>,
                > = None;
                let mut f_inner_hits: Option<
                    std::collections::BTreeMap<String, crate::types::search::InnerHitsResult>,
                > = None;
                let mut f_matched_queries: Option<
                    crate::types::knn::SearchModelsResponseHitsHitsItemMatchedQueries,
                > = None;
                let mut f_sort: Option<crate::types::common::SortResults> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_explanation" => {
                            f_explanation =
                                map.next_value::<Option<crate::types::explain::Explanation>>()?
                        }
                        "_id" => f_id = map.next_value::<Option<crate::types::common::Id>>()?,
                        "_ignored" => f_ignored = map.next_value::<Option<Vec<String>>>()?,
                        "_index" => {
                            f_index = map.next_value::<Option<crate::types::common::IndexName>>()?
                        }
                        "_nested" => {
                            f_nested =
                                map.next_value::<Option<crate::types::search::NestedIdentity>>()?
                        }
                        "_node" => f_node = map.next_value::<Option<String>>()?,
                        "_primary_term" => f_primary_term = map.next_value::<Option<i64>>()?,
                        "_routing" => f_routing = map.next_value::<Option<String>>()?,
                        "_score" => f_score = map.next_value::<Option<f64>>()?,
                        "_seq_no" => {
                            f_seq_no =
                                map.next_value::<Option<crate::types::common::SequenceNumber>>()?
                        }
                        "_shard" => f_shard = map.next_value::<Option<String>>()?,
                        "_source" => {
                            f_source =
                                map.next_value::<Option<Box<serde_json::value::RawValue>>>()?
                        }
                        "_type" => {
                            f_type = map.next_value::<Option<crate::types::common::Type>>()?
                        }
                        "_version" => {
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionNumber>>()?
                        }
                        "fields" => {
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?
                        }
                        "highlight" => f_highlight = map
                            .next_value::<Option<std::collections::BTreeMap<String, Vec<String>>>>(
                            )?,
                        "ignored_field_values" => f_ignored_field_values = map
                            .next_value::<Option<std::collections::BTreeMap<String, Vec<String>>>>(
                            )?,
                        "inner_hits" => {
                            f_inner_hits = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::search::InnerHitsResult,
                                >,
                            >>()?
                        }
                        "matched_queries" => {
                            f_matched_queries = map.next_value::<Option<
                                crate::types::knn::SearchModelsResponseHitsHitsItemMatchedQueries,
                            >>()?
                        }
                        "sort" => {
                            f_sort =
                                map.next_value::<Option<crate::types::common::SortResults>>()?
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SearchModelsResponseHitsHitsItem {
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
impl serde::Serialize for SearchModelsResponseHitsHitsItem {
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

#[doc = "The names of queries that matched the document. When `include_named_queries_score` is false (default), returns an array of query names. When true, returns an object mapping query names to their scores."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SearchModelsResponseHitsHitsItemMatchedQueries {
    Names(Vec<String>),
    Scores(std::collections::BTreeMap<String, f64>),
}
impl<'de> serde::Deserialize<'de> for SearchModelsResponseHitsHitsItemMatchedQueries {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = SearchModelsResponseHitsHitsItemMatchedQueries;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchModelsResponseHitsHitsItemMatchedQueries value")
            }
            fn visit_seq<A>(self, seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
                    .map(SearchModelsResponseHitsHitsItemMatchedQueries::Names)
            }
            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(SearchModelsResponseHitsHitsItemMatchedQueries::Scores)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[doc = "The total number of hits, present only if `track_total_hits` is not set to `false` in the search request."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SearchModelsResponseHitsTotal {
    TotalHits(crate::types::search::TotalHits),
    Integer(i64),
}
impl<'de> serde::Deserialize<'de> for SearchModelsResponseHitsTotal {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = SearchModelsResponseHitsTotal;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchModelsResponseHitsTotal value")
            }
            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(SearchModelsResponseHitsTotal::TotalHits)
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::I64Deserializer::new(v))
                    .map(SearchModelsResponseHitsTotal::Integer)
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::U64Deserializer::new(v))
                    .map(SearchModelsResponseHitsTotal::Integer)
            }
            fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::F64Deserializer::new(v))
                    .map(SearchModelsResponseHitsTotal::Integer)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SearchModelsResponseSuggestValueItem {
    raw: Box<serde_json::value::RawValue>,
}
impl SearchModelsResponseSuggestValueItem {
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
    ) -> std::result::Result<
        crate::types::knn::SearchModelsResponseSuggestValueItemCompletion,
        crate::types::UnionBranchError,
    > {
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchModelsResponseSuggestValueItemCompletion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<crate::types::knn::SearchModelsResponseSuggestValueItemCompletionOptions>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SearchModelsResponseSuggestValueItemCompletionOptions {
    Object(crate::types::knn::SearchModelsResponseSuggestValueItemCompletionOptionsObject),
    Array(Vec<crate::types::knn::SearchModelsResponseSuggestValueItemCompletionOptionsArrayItem>),
}
impl<'de> serde::Deserialize<'de> for SearchModelsResponseSuggestValueItemCompletionOptions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = SearchModelsResponseSuggestValueItemCompletionOptions;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchModelsResponseSuggestValueItemCompletionOptions value")
            }
            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(SearchModelsResponseSuggestValueItemCompletionOptions::Object)
            }
            fn visit_seq<A>(self, seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
                    .map(SearchModelsResponseSuggestValueItemCompletionOptions::Array)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchModelsResponseSuggestValueItemCompletionOptionsArrayItem {
    #[serde(rename = "_source")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<serde_json::value::RawValue>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchModelsResponseSuggestValueItemCompletionOptionsObject {
    #[serde(rename = "_source")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<serde_json::value::RawValue>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TrainModelResponse {
    pub model_id: String,
}
