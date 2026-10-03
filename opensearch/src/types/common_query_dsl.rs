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

//! Response types of the `common_query_dsl` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AgenticQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "The memory ID from a previous response, used to continue a conversation with prior context. Applicable for conversational agents only."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_id: Option<String>,
    #[doc = "A list of index fields that the agent should consider when generating the search query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_fields: Option<Vec<crate::types::common::Field>>,
    #[doc = "The natural language question to be answered by the agent."]
    pub query_text: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BoolQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "Ensures correct behavior when a query contains only `must_not` clauses. By default set to true, OpenSearch adds a match-all clause to ensure results are returned from Lucene, with the `must_not` conditions applied as filters. If set to false, the query may return no results, as Lucene typically requires at least one positive condition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adjust_pure_negative: Option<bool>,
    #[doc = "The clause (query) must appear in matching documents. However, unlike `must`, the score of the query will be ignored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<crate::types::common_query_dsl::BoolQueryFilter>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_should_match: Option<crate::types::common::MinimumShouldMatch>,
    #[doc = "The clause (query) must appear in matching documents and will contribute to the score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub must: Option<Box<crate::types::common_query_dsl::BoolQueryMust>>,
    #[doc = "The clause (query) must not appear in the matching documents. Because scoring is ignored, a score of `0` is returned for all documents."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub must_not: Option<Box<crate::types::common_query_dsl::BoolQueryMustNot>>,
    #[doc = "The clause (query) should appear in the matching document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub should: Option<Box<crate::types::common_query_dsl::BoolQueryShould>>,
}

#[doc = "The clause (query) must appear in matching documents. However, unlike `must`, the score of the query will be ignored."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum BoolQueryFilter {
    QueryContainer(Box<crate::types::common_query_dsl::QueryContainer>),
    Array(Vec<crate::types::common_query_dsl::QueryContainer>),
}
impl<'de> serde::Deserialize<'de> for BoolQueryFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = BoolQueryFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a BoolQueryFilter value")
            }
            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(BoolQueryFilter::QueryContainer)
            }
            fn visit_seq<A>(self, seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
                    .map(BoolQueryFilter::Array)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[doc = "The clause (query) must appear in matching documents and will contribute to the score."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum BoolQueryMust {
    QueryContainer(Box<crate::types::common_query_dsl::QueryContainer>),
    Array(Vec<crate::types::common_query_dsl::QueryContainer>),
}
impl<'de> serde::Deserialize<'de> for BoolQueryMust {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = BoolQueryMust;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a BoolQueryMust value")
            }
            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(BoolQueryMust::QueryContainer)
            }
            fn visit_seq<A>(self, seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
                    .map(BoolQueryMust::Array)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[doc = "The clause (query) must not appear in the matching documents. Because scoring is ignored, a score of `0` is returned for all documents."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum BoolQueryMustNot {
    QueryContainer(Box<crate::types::common_query_dsl::QueryContainer>),
    Array(Vec<crate::types::common_query_dsl::QueryContainer>),
}
impl<'de> serde::Deserialize<'de> for BoolQueryMustNot {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = BoolQueryMustNot;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a BoolQueryMustNot value")
            }
            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(BoolQueryMustNot::QueryContainer)
            }
            fn visit_seq<A>(self, seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
                    .map(BoolQueryMustNot::Array)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[doc = "The clause (query) should appear in the matching document."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum BoolQueryShould {
    QueryContainer(Box<crate::types::common_query_dsl::QueryContainer>),
    Array(Vec<crate::types::common_query_dsl::QueryContainer>),
}
impl<'de> serde::Deserialize<'de> for BoolQueryShould {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = BoolQueryShould;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a BoolQueryShould value")
            }
            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(BoolQueryShould::QueryContainer)
            }
            fn visit_seq<A>(self, seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
                    .map(BoolQueryShould::Array)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BoostingQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    pub negative: Box<crate::types::common_query_dsl::QueryContainer>,
    #[doc = "Floating point number between 0 and 1.0 used to decrease the relevance scores of documents matching the `negative` query."]
    pub negative_boost: f64,
    pub positive: Box<crate::types::common_query_dsl::QueryContainer>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ChildScoreMode {
    #[serde(rename = "avg")]
    Avg,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "sum")]
    Sum,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CombinedFieldsOperator {
    #[serde(rename = "and")]
    And,
    #[serde(rename = "or")]
    Or,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CombinedFieldsQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "If `true`, match phrase queries are automatically created for multi-term synonyms."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_generate_synonyms_phrase_query: Option<bool>,
    #[doc = "List of fields to search. Field wildcard patterns are allowed. Only `text` fields are supported, and they must all have the same search `analyzer`."]
    pub fields: Vec<crate::types::common::Field>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_should_match: Option<crate::types::common::MinimumShouldMatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<crate::types::common_query_dsl::CombinedFieldsOperator>,
    #[doc = "Text to search for in the provided `fields`. The `combined_fields` query analyzes the provided text before performing a search."]
    pub query: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zero_terms_query: Option<crate::types::common_query_dsl::CombinedFieldsZeroTerms>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CombinedFieldsZeroTerms {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum CommonTermsQuery {
    Query(String),
    Object(crate::types::common_query_dsl::CommonTermsQueryObject),
}
impl<'de> serde::Deserialize<'de> for CommonTermsQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = CommonTermsQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CommonTermsQuery value")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(CommonTermsQuery::Query)
            }
            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(CommonTermsQuery::Object)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CommonTermsQueryObject {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cutoff_frequency: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub high_freq_operator: Option<crate::types::common_query_dsl::Operator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub low_freq_operator: Option<crate::types::common_query_dsl::Operator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_should_match: Option<crate::types::common::MinimumShouldMatch>,
    pub query: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConstantScoreQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    pub filter: Box<crate::types::common_query_dsl::QueryContainer>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DateDecayPlacement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decay: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<crate::types::common::Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<crate::types::common::DateTime>,
    pub scale: crate::types::common::Duration,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DateRangeQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation: Option<crate::types::common_query_dsl::RangeRelation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<crate::types::common::DateFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<crate::types::common::DateMath>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gt: Option<crate::types::common::DateMath>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gte: Option<crate::types::common::DateMath>,
    #[doc = "Include the lower bound"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_lower: Option<bool>,
    #[doc = "Include the upper bound"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_upper: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lt: Option<crate::types::common::DateMath>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lte: Option<crate::types::common::DateMath>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<crate::types::common::TimeZone>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<crate::types::common::DateMath>,
}

#[derive(Debug, Clone)]
pub struct DecayFunction {
    pub multi_value_mode: Option<crate::types::common_query_dsl::MultiValueMode>,
    pub additional:
        std::collections::BTreeMap<String, crate::types::common_query_dsl::DecayPlacement>,
}
impl<'de> serde::Deserialize<'de> for DecayFunction {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DecayFunction;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DecayFunction object")
            }
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_multi_value_mode: Option<crate::types::common_query_dsl::MultiValueMode> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_query_dsl::DecayPlacement,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "multi_value_mode" => f_multi_value_mode = map
                            .next_value::<Option<crate::types::common_query_dsl::MultiValueMode>>(
                            )?,
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DecayFunction {
                    multi_value_mode: f_multi_value_mode,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DecayFunction {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.multi_value_mode {
            map.serialize_entry("multi_value_mode", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct DecayPlacement {
    raw: Box<serde_json::value::RawValue>,
}
impl DecayPlacement {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `date_decay_placement` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_date_decay_placement(
        &self,
    ) -> std::result::Result<
        crate::types::common_query_dsl::DateDecayPlacement,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "date_decay_placement")
    }
    #[doc = "Parses the value as the `geo_decay_placement` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_geo_decay_placement(
        &self,
    ) -> std::result::Result<
        crate::types::common_query_dsl::GeoDecayPlacement,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "geo_decay_placement")
    }
    #[doc = "Parses the value as the `numeric_decay_placement` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_numeric_decay_placement(
        &self,
    ) -> std::result::Result<
        crate::types::common_query_dsl::NumericDecayPlacement,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "numeric_decay_placement")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DisMaxQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "One or more query clauses. Returned documents must match one or more of these queries. If a document matches multiple queries, OpenSearch uses the highest relevance score."]
    pub queries: Vec<crate::types::common_query_dsl::QueryContainer>,
    #[doc = "Floating point number between 0 and 1.0 used to increase the relevance scores of documents matching multiple query clauses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tie_breaker: Option<f64>,
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct DistanceFeatureQuery {
    raw: Box<serde_json::value::RawValue>,
}
impl DistanceFeatureQuery {
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
        crate::types::common_query_dsl::DistanceFeatureQueryObject,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "object")
    }
    #[doc = "Parses the value as the `object_object` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_object_object(
        &self,
    ) -> std::result::Result<
        crate::types::common_query_dsl::DistanceFeatureQueryObjectObject,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "object_object")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DistanceFeatureQueryObject {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    pub field: crate::types::common::Field,
    pub origin: crate::types::common::GeoLocation,
    pub pivot: crate::types::common::Distance,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DistanceFeatureQueryObjectObject {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    pub field: crate::types::common::Field,
    pub origin: crate::types::common::DateMath,
    pub pivot: crate::types::common::Duration,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExistsQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    pub field: crate::types::common::Field,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum FieldAndFormat {
    Field(crate::types::common::Field),
    Object(crate::types::common_query_dsl::FieldAndFormatObject),
}
impl<'de> serde::Deserialize<'de> for FieldAndFormat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = FieldAndFormat;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FieldAndFormat value")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(FieldAndFormat::Field)
            }
            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(FieldAndFormat::Object)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FieldAndFormatObject {
    pub field: crate::types::common::Field,
    #[doc = "Format in which the values are returned."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FieldLookup {
    pub id: crate::types::common::Id,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<crate::types::common::IndexName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<crate::types::common::Field>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing: Option<crate::types::common::Routing>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FieldValueFactorModifier {
    #[serde(rename = "ln")]
    Ln,
    #[serde(rename = "ln1p")]
    Ln1P,
    #[serde(rename = "ln2p")]
    Ln2P,
    #[serde(rename = "log")]
    Log,
    #[serde(rename = "log1p")]
    Log1P,
    #[serde(rename = "log2p")]
    Log2P,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "reciprocal")]
    Reciprocal,
    #[serde(rename = "sqrt")]
    Sqrt,
    #[serde(rename = "square")]
    Square,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FieldValueFactorScoreFunction {
    #[doc = "Optional factor to multiply the field value with."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub factor: Option<f64>,
    pub field: crate::types::common::Field,
    #[doc = "Value used if the document doesn't have that field. The modifier and factor are still applied to it as though it were read from the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modifier: Option<crate::types::common_query_dsl::FieldValueFactorModifier>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FunctionBoostMode {
    #[serde(rename = "avg")]
    Avg,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "multiply")]
    Multiply,
    #[serde(rename = "replace")]
    Replace,
    #[serde(rename = "sum")]
    Sum,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FunctionScoreContainer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<crate::types::common_query_dsl::QueryContainer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<crate::types::common_query_dsl::DecayFunction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_value_factor: Option<crate::types::common_query_dsl::FieldValueFactorScoreFunction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gauss: Option<crate::types::common_query_dsl::DecayFunction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linear: Option<crate::types::common_query_dsl::DecayFunction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub random_score: Option<crate::types::common_query_dsl::RandomScoreFunction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script_score: Option<crate::types::common_query_dsl::ScriptScoreFunction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FunctionScoreMode {
    #[serde(rename = "avg")]
    Avg,
    #[serde(rename = "first")]
    First,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "multiply")]
    Multiply,
    #[serde(rename = "sum")]
    Sum,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FunctionScoreQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost_mode: Option<crate::types::common_query_dsl::FunctionBoostMode>,
    #[doc = "One or more functions that compute a new score for each document returned by the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<crate::types::common_query_dsl::FunctionScoreContainer>>,
    #[doc = "Restricts the new score to not exceed the provided limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_boost: Option<f64>,
    #[doc = "Excludes documents that do not meet the provided score threshold."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<Box<crate::types::common_query_dsl::QueryContainer>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score_mode: Option<crate::types::common_query_dsl::FunctionScoreMode>,
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct FuzzyQuery {
    raw: Box<serde_json::value::RawValue>,
}
impl FuzzyQuery {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `value` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_value(
        &self,
    ) -> std::result::Result<crate::types::common::FieldValue, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "value")
    }
    #[doc = "Parses the value as the `object` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_object(
        &self,
    ) -> std::result::Result<
        crate::types::common_query_dsl::FuzzyQueryObject,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "object")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FuzzyQueryObject {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzziness: Option<crate::types::common::Fuzziness>,
    #[doc = "Maximum number of variations created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_expansions: Option<i32>,
    #[doc = "Number of beginning characters left unchanged when creating expansions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix_length: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rewrite: Option<crate::types::common::MultiTermQueryRewrite>,
    #[doc = "Indicates whether edits include transpositions of two adjacent characters (for example, `ab` to `ba`)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transpositions: Option<bool>,
    #[doc = "Term you wish to find in the provided field."]
    pub value: crate::types::common::FieldValue,
}

#[derive(Debug, Clone)]
pub struct GeoBoundingBoxQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped>,
    pub r#type: Option<crate::types::common_query_dsl::GeoExecution>,
    pub validation_method: Option<crate::types::common_query_dsl::GeoValidationMethod>,
    pub additional: std::collections::BTreeMap<String, crate::types::common::GeoBounds>,
}
impl<'de> serde::Deserialize<'de> for GeoBoundingBoxQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GeoBoundingBoxQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoBoundingBoxQuery object")
            }
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut f_boost: Option<f64> = None;
                let mut f_ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped> =
                    None;
                let mut f_type: Option<crate::types::common_query_dsl::GeoExecution> = None;
                let mut f_validation_method: Option<
                    crate::types::common_query_dsl::GeoValidationMethod,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common::GeoBounds,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => f_name = map.next_value::<Option<String>>()?,
                        "boost" => f_boost = map.next_value::<Option<f64>>()?,
                        "ignore_unmapped" => f_ignore_unmapped = map.next_value::<Option<crate::types::common_query_dsl::IgnoreUnmapped>>()?,
                        "type" => f_type = map.next_value::<Option<crate::types::common_query_dsl::GeoExecution>>()?,
                        "validation_method" => f_validation_method = map.next_value::<Option<crate::types::common_query_dsl::GeoValidationMethod>>()?,
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GeoBoundingBoxQuery {
                    name: f_name,
                    boost: f_boost,
                    ignore_unmapped: f_ignore_unmapped,
                    r#type: f_type,
                    validation_method: f_validation_method,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GeoBoundingBoxQuery {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.name {
            map.serialize_entry("_name", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.ignore_unmapped {
            map.serialize_entry("ignore_unmapped", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("type", v)?;
        }
        if let Some(v) = &self.validation_method {
            map.serialize_entry("validation_method", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeoDecayPlacement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decay: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<crate::types::common::Distance>,
    pub origin: crate::types::common::GeoLocation,
    pub scale: crate::types::common::Distance,
}

#[derive(Debug, Clone)]
pub struct GeoDistanceQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub distance: crate::types::common::Distance,
    pub distance_type: Option<crate::types::common::GeoDistanceType>,
    pub ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped>,
    #[doc = "The unit of distance measurement."]
    pub unit: Option<crate::types::common::DistanceUnit>,
    pub validation_method: Option<crate::types::common_query_dsl::GeoValidationMethod>,
    pub additional: std::collections::BTreeMap<String, crate::types::common::GeoLocation>,
}
impl<'de> serde::Deserialize<'de> for GeoDistanceQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GeoDistanceQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoDistanceQuery object")
            }
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut f_boost: Option<f64> = None;
                let mut f_distance: Option<crate::types::common::Distance> = None;
                let mut f_distance_type: Option<crate::types::common::GeoDistanceType> = None;
                let mut f_ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped> =
                    None;
                let mut f_unit: Option<crate::types::common::DistanceUnit> = None;
                let mut f_validation_method: Option<
                    crate::types::common_query_dsl::GeoValidationMethod,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common::GeoLocation,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => f_name = map.next_value::<Option<String>>()?,
                        "boost" => f_boost = map.next_value::<Option<f64>>()?,
                        "distance" => f_distance = Some(map.next_value::<crate::types::common::Distance>()?),
                        "distance_type" => f_distance_type = map.next_value::<Option<crate::types::common::GeoDistanceType>>()?,
                        "ignore_unmapped" => f_ignore_unmapped = map.next_value::<Option<crate::types::common_query_dsl::IgnoreUnmapped>>()?,
                        "unit" => f_unit = map.next_value::<Option<crate::types::common::DistanceUnit>>()?,
                        "validation_method" => f_validation_method = map.next_value::<Option<crate::types::common_query_dsl::GeoValidationMethod>>()?,
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GeoDistanceQuery {
                    name: f_name,
                    boost: f_boost,
                    distance: f_distance
                        .ok_or_else(|| serde::de::Error::missing_field("distance"))?,
                    distance_type: f_distance_type,
                    ignore_unmapped: f_ignore_unmapped,
                    unit: f_unit,
                    validation_method: f_validation_method,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GeoDistanceQuery {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.name {
            map.serialize_entry("_name", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        map.serialize_entry("distance", &self.distance)?;
        if let Some(v) = &self.distance_type {
            map.serialize_entry("distance_type", v)?;
        }
        if let Some(v) = &self.ignore_unmapped {
            map.serialize_entry("ignore_unmapped", v)?;
        }
        if let Some(v) = &self.unit {
            map.serialize_entry("unit", v)?;
        }
        if let Some(v) = &self.validation_method {
            map.serialize_entry("validation_method", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GeoExecution {
    #[serde(rename = "indexed")]
    Indexed,
    #[serde(rename = "memory")]
    Memory,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeoPolygonPoints {
    pub points: Vec<crate::types::common::GeoLocation>,
}

#[derive(Debug, Clone)]
pub struct GeoPolygonQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped>,
    pub validation_method: Option<crate::types::common_query_dsl::GeoValidationMethod>,
    pub additional:
        std::collections::BTreeMap<String, crate::types::common_query_dsl::GeoPolygonPoints>,
}
impl<'de> serde::Deserialize<'de> for GeoPolygonQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GeoPolygonQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoPolygonQuery object")
            }
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut f_boost: Option<f64> = None;
                let mut f_ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped> =
                    None;
                let mut f_validation_method: Option<
                    crate::types::common_query_dsl::GeoValidationMethod,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_query_dsl::GeoPolygonPoints,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => f_name = map.next_value::<Option<String>>()?,
                        "boost" => f_boost = map.next_value::<Option<f64>>()?,
                        "ignore_unmapped" => f_ignore_unmapped = map.next_value::<Option<crate::types::common_query_dsl::IgnoreUnmapped>>()?,
                        "validation_method" => f_validation_method = map.next_value::<Option<crate::types::common_query_dsl::GeoValidationMethod>>()?,
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GeoPolygonQuery {
                    name: f_name,
                    boost: f_boost,
                    ignore_unmapped: f_ignore_unmapped,
                    validation_method: f_validation_method,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GeoPolygonQuery {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.name {
            map.serialize_entry("_name", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.ignore_unmapped {
            map.serialize_entry("ignore_unmapped", v)?;
        }
        if let Some(v) = &self.validation_method {
            map.serialize_entry("validation_method", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeoShape {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<Vec<Box<serde_json::value::RawValue>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GeoShapeQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped>,
    pub additional:
        std::collections::BTreeMap<String, crate::types::common_query_dsl::GeoShapeQueryField>,
}
impl<'de> serde::Deserialize<'de> for GeoShapeQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GeoShapeQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoShapeQuery object")
            }
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut f_boost: Option<f64> = None;
                let mut f_ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_query_dsl::GeoShapeQueryField,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => f_name = map.next_value::<Option<String>>()?,
                        "boost" => f_boost = map.next_value::<Option<f64>>()?,
                        "ignore_unmapped" => f_ignore_unmapped = map
                            .next_value::<Option<crate::types::common_query_dsl::IgnoreUnmapped>>(
                            )?,
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GeoShapeQuery {
                    name: f_name,
                    boost: f_boost,
                    ignore_unmapped: f_ignore_unmapped,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GeoShapeQuery {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.name {
            map.serialize_entry("_name", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.ignore_unmapped {
            map.serialize_entry("ignore_unmapped", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeoShapeQueryField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexed_shape: Option<crate::types::common_query_dsl::FieldLookup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation: Option<crate::types::common::GeoShapeRelation>,
    pub shape: crate::types::common_query_dsl::GeoShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GeoValidationMethod {
    #[serde(rename = "coerce")]
    Coerce,
    #[serde(rename = "ignore_malformed")]
    IgnoreMalformed,
    #[serde(rename = "strict")]
    Strict,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HasChildQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "Indicates whether to ignore an unmapped `type` and not return any documents instead of an error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inner_hits: Option<Box<crate::types::search::InnerHits>>,
    #[doc = "Maximum number of child documents that match the query allowed for a returned parent document. If the parent document exceeds this limit, it is excluded from the search results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_children: Option<i32>,
    #[doc = "Minimum number of child documents that match the query required to match the query for a returned parent document. If the parent document does not meet this limit, it is excluded from the search results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_children: Option<i32>,
    pub query: Box<crate::types::common_query_dsl::QueryContainer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score_mode: Option<crate::types::common_query_dsl::ChildScoreMode>,
    pub r#type: crate::types::common::RelationName,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HasParentQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "Indicates whether to ignore an unmapped `parent_type` and not return any documents instead of an error. You can use this parameter to query multiple indexes that may not contain the `parent_type`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inner_hits: Option<Box<crate::types::search::InnerHits>>,
    pub parent_type: crate::types::common::RelationName,
    pub query: Box<crate::types::common_query_dsl::QueryContainer>,
    #[doc = "Indicates whether the relevance score of a matching parent document is aggregated into its child documents."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HybridQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<crate::types::common_query_dsl::QueryContainer>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pagination_depth: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queries: Option<Vec<crate::types::common_query_dsl::QueryContainer>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IdsQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<crate::types::common::Ids>,
}

#[doc = "Set to `true` to ignore an unmapped field and not match any documents for this query. Set to `false` to throw an exception if the field is not mapped."]
pub type IgnoreUnmapped = bool;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntervalsAllOf {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<crate::types::common_query_dsl::IntervalsFilter>>,
    #[doc = "An array of rules to combine. All rules must produce a match in a document for the overall source to match."]
    pub intervals: Vec<crate::types::common_query_dsl::IntervalsContainer>,
    #[doc = "Maximum number of positions between the matching terms. Intervals produced by the rules further apart than this are not considered matches."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_gaps: Option<i32>,
    #[doc = "If `true`, intervals produced by the rules should appear in the order in which they are specified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ordered: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntervalsAnyOf {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<crate::types::common_query_dsl::IntervalsFilter>>,
    #[doc = "An array of rules to match."]
    pub intervals: Vec<crate::types::common_query_dsl::IntervalsContainer>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntervalsContainer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_of: Option<Box<crate::types::common_query_dsl::IntervalsAllOf>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub any_of: Option<Box<crate::types::common_query_dsl::IntervalsAnyOf>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzzy: Option<crate::types::common_query_dsl::IntervalsFuzzy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#match: Option<Box<crate::types::common_query_dsl::IntervalsMatch>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<crate::types::common_query_dsl::IntervalsPrefix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wildcard: Option<crate::types::common_query_dsl::IntervalsWildcard>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntervalsFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<Box<crate::types::common_query_dsl::IntervalsContainer>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<Box<crate::types::common_query_dsl::IntervalsContainer>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contained_by: Option<Box<crate::types::common_query_dsl::IntervalsContainer>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containing: Option<Box<crate::types::common_query_dsl::IntervalsContainer>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_contained_by: Option<Box<crate::types::common_query_dsl::IntervalsContainer>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_containing: Option<Box<crate::types::common_query_dsl::IntervalsContainer>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_overlapping: Option<Box<crate::types::common_query_dsl::IntervalsContainer>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overlapping: Option<Box<crate::types::common_query_dsl::IntervalsContainer>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<crate::types::common::Script>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntervalsFuzzy {
    #[doc = "Analyzer used to normalize the term."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzziness: Option<crate::types::common::Fuzziness>,
    #[doc = "Number of beginning characters left unchanged when creating expansions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix_length: Option<i32>,
    #[doc = "The term to match."]
    pub term: String,
    #[doc = "Indicates whether edits include transpositions of two adjacent characters (for example, `ab` to `ba`)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transpositions: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_field: Option<crate::types::common::Field>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntervalsMatch {
    #[doc = "Analyzer used to analyze terms in the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<crate::types::common_query_dsl::IntervalsFilter>>,
    #[doc = "Maximum number of positions between the matching terms. Terms further apart than this are not considered matches."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_gaps: Option<i32>,
    #[doc = "If `true`, matching terms must appear in their specified order."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ordered: Option<bool>,
    #[doc = "Text you wish to find in the provided field."]
    pub query: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_field: Option<crate::types::common::Field>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntervalsPrefix {
    #[doc = "Analyzer used to analyze the `prefix`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    #[doc = "Beginning characters of terms you wish to find in the top-level field."]
    pub prefix: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_field: Option<crate::types::common::Field>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntervalsQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_of: Option<crate::types::common_query_dsl::IntervalsAllOf>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub any_of: Option<crate::types::common_query_dsl::IntervalsAnyOf>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzzy: Option<crate::types::common_query_dsl::IntervalsFuzzy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#match: Option<crate::types::common_query_dsl::IntervalsMatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<crate::types::common_query_dsl::IntervalsPrefix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wildcard: Option<crate::types::common_query_dsl::IntervalsWildcard>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntervalsWildcard {
    #[doc = "Analyzer used to analyze the `pattern`. Defaults to the top-level field's analyzer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    #[doc = "Wildcard pattern used to find matching terms."]
    pub pattern: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_field: Option<crate::types::common::Field>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KnnQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand_nested_docs: Option<bool>,
    #[doc = "The filter for the k-NN search query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<crate::types::common_query_dsl::QueryContainer>,
    #[doc = "The total number of nearest neighbors to return as top hits."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub k: Option<i32>,
    #[doc = "The maximum physical vector space distance required in order for a neighbor to be considered a hit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_distance: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method_parameters:
        Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The minimum similarity score required in order for a neighbor to be considered a hit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rescore: Option<crate::types::common_query_dsl::KnnQueryRescore>,
    pub vector: crate::types::common_query_dsl::QueryVector,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum KnnQueryRescore {
    Enable(bool),
    Context(crate::types::common_query_dsl::RescoreContext),
}
impl<'de> serde::Deserialize<'de> for KnnQueryRescore {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = KnnQueryRescore;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a KnnQueryRescore value")
            }
            fn visit_bool<E>(self, v: bool) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::BoolDeserializer::new(v))
                    .map(KnnQueryRescore::Enable)
            }
            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(KnnQueryRescore::Context)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[doc = "Text that we want similar documents for or a lookup to a document's field for the text."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum Like {
    Text(String),
    Document(crate::types::common_query_dsl::LikeDocument),
}
impl<'de> serde::Deserialize<'de> for Like {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = Like;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Like value")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(Like::Text)
            }
            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(Like::Document)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LikeDocument {
    #[serde(rename = "_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::types::common::Id>,
    #[serde(rename = "_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<crate::types::common::IndexName>,
    #[serde(rename = "_type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::types::common::Type>,
    #[doc = "A document not present in the index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<Box<serde_json::value::RawValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<crate::types::common::Field>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_field_analyzer: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing: Option<crate::types::common::Routing>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionNumber>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_type: Option<crate::types::common::VersionType>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MatchAllQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum MatchBoolPrefixQuery {
    Query(String),
    Object(crate::types::common_query_dsl::MatchBoolPrefixQueryObject),
}
impl<'de> serde::Deserialize<'de> for MatchBoolPrefixQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = MatchBoolPrefixQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MatchBoolPrefixQuery value")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(MatchBoolPrefixQuery::Query)
            }
            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(MatchBoolPrefixQuery::Object)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MatchBoolPrefixQueryObject {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "Analyzer used to convert the text in the query value into tokens."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzziness: Option<crate::types::common::Fuzziness>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzzy_rewrite: Option<crate::types::common::MultiTermQueryRewrite>,
    #[doc = "If `true`, edits for fuzzy matching include transpositions of two adjacent characters (for example, `ab` to `ba`). Can be applied to the term subqueries constructed for all terms but the final term."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzzy_transpositions: Option<bool>,
    #[doc = "Maximum number of terms to which the query will expand. Can be applied to the term subqueries constructed for all terms but the final term."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_expansions: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_should_match: Option<crate::types::common::MinimumShouldMatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<crate::types::common_query_dsl::Operator>,
    #[doc = "Number of beginning characters left unchanged for fuzzy matching. Can be applied to the term subqueries constructed for all terms but the final term."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix_length: Option<i32>,
    #[doc = "Terms you wish to find in the provided field. The last term is used in a prefix query."]
    pub query: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MatchNoneQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum MatchPhrasePrefixQuery {
    Query(String),
    Object(crate::types::common_query_dsl::MatchPhrasePrefixQueryObject),
}
impl<'de> serde::Deserialize<'de> for MatchPhrasePrefixQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = MatchPhrasePrefixQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MatchPhrasePrefixQuery value")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(MatchPhrasePrefixQuery::Query)
            }
            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(MatchPhrasePrefixQuery::Object)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MatchPhrasePrefixQueryObject {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "Analyzer used to convert text in the query value into tokens."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    #[doc = "Maximum number of terms to which the last provided term of the query value will expand."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_expansions: Option<i32>,
    #[doc = "Text you wish to find in the provided field."]
    pub query: String,
    #[doc = "Maximum number of positions allowed between matching tokens."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slop: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zero_terms_query: Option<crate::types::common_query_dsl::ZeroTermsQuery>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum MatchPhraseQuery {
    Query(String),
    Object(crate::types::common_query_dsl::MatchPhraseQueryObject),
}
impl<'de> serde::Deserialize<'de> for MatchPhraseQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = MatchPhraseQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MatchPhraseQuery value")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(MatchPhraseQuery::Query)
            }
            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(MatchPhraseQuery::Object)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MatchPhraseQueryObject {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "Analyzer used to convert the text in the query value into tokens."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    #[doc = "Query terms that are analyzed and turned into a phrase query."]
    pub query: String,
    #[doc = "Maximum number of positions allowed between matching tokens."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slop: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zero_terms_query: Option<crate::types::common_query_dsl::ZeroTermsQuery>,
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MatchQuery {
    raw: Box<serde_json::value::RawValue>,
}
impl MatchQuery {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `query` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_query(
        &self,
    ) -> std::result::Result<crate::types::common::FieldValue, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "query")
    }
    #[doc = "Parses the value as the `object` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_object(
        &self,
    ) -> std::result::Result<
        crate::types::common_query_dsl::MatchQueryObject,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "object")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MatchQueryObject {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "Analyzer used to convert the text in the query value into tokens."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    #[doc = "If `true`, match phrase queries are automatically created for multi-term synonyms."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_generate_synonyms_phrase_query: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cutoff_frequency: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzziness: Option<crate::types::common::Fuzziness>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzzy_rewrite: Option<crate::types::common::MultiTermQueryRewrite>,
    #[doc = "If `true`, edits for fuzzy matching include transpositions of two adjacent characters (for example, `ab` to `ba`)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzzy_transpositions: Option<bool>,
    #[doc = "If `true`, format-based errors, such as providing a text query value for a numeric field, are ignored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lenient: Option<bool>,
    #[doc = "Maximum number of terms to which the query will expand."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_expansions: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_should_match: Option<crate::types::common::MinimumShouldMatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<crate::types::common_query_dsl::Operator>,
    #[doc = "Number of beginning characters left unchanged for fuzzy matching."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix_length: Option<i32>,
    #[doc = "Text, number, Boolean value or date you wish to find in the provided field."]
    pub query: crate::types::common::FieldValue,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zero_terms_query: Option<crate::types::common_query_dsl::ZeroTermsQuery>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MoreLikeThisQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "The analyzer that is used to analyze the free form text. Defaults to the analyzer associated with the first field in fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    #[doc = "Each term in the formed query could be further boosted by their tf-idf score. This sets the boost factor to use when using this feature. Defaults to deactivated (0)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost_terms: Option<f64>,
    #[doc = "Controls whether the query should fail (throw an exception) if any of the specified fields are not of the supported types (`text` or `keyword`)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fail_on_unsupported_field: Option<bool>,
    #[doc = "A list of fields to fetch and analyze the text from. Defaults to the `index.query.default_field` index setting, which has a default value of `*`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<crate::types::common::Field>>,
    #[doc = "Specifies whether the input documents should also be included in the search results returned."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include: Option<bool>,
    #[doc = "Specifies free form text and/or a single or multiple documents for which you want to find similar documents."]
    pub like: crate::types::common_query_dsl::MoreLikeThisQueryLike,
    #[doc = "The maximum document frequency above which the terms are ignored from the input document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_doc_freq: Option<i32>,
    #[doc = "The maximum number of query terms that can be selected."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_query_terms: Option<i32>,
    #[doc = "The maximum word length above which the terms are ignored. Defaults to unbounded (`0`)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_word_length: Option<i32>,
    #[doc = "The minimum document frequency below which the terms are ignored from the input document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_doc_freq: Option<i32>,
    #[doc = "The minimum term frequency below which the terms are ignored from the input document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_term_freq: Option<i32>,
    #[doc = "The minimum word length below which the terms are ignored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_word_length: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_should_match: Option<crate::types::common::MinimumShouldMatch>,
    #[doc = "Overrides the default analyzer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_field_analyzer: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing: Option<crate::types::common::Routing>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_words: Option<crate::types::common_analysis::StopWords>,
    #[doc = "Used in combination with `like` to exclude documents that match a set of terms."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unlike: Option<crate::types::common_query_dsl::MoreLikeThisQueryUnlike>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionNumber>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_type: Option<crate::types::common::VersionType>,
}

#[doc = "Specifies free form text and/or a single or multiple documents for which you want to find similar documents."]
#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MoreLikeThisQueryLike {
    raw: Box<serde_json::value::RawValue>,
}
impl MoreLikeThisQueryLike {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `like` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_like(
        &self,
    ) -> std::result::Result<crate::types::common_query_dsl::Like, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "like")
    }
    #[doc = "Parses the value as the `array` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_array(
        &self,
    ) -> std::result::Result<
        Vec<crate::types::common_query_dsl::Like>,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "array")
    }
}

#[doc = "Used in combination with `like` to exclude documents that match a set of terms."]
#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MoreLikeThisQueryUnlike {
    raw: Box<serde_json::value::RawValue>,
}
impl MoreLikeThisQueryUnlike {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `like` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_like(
        &self,
    ) -> std::result::Result<crate::types::common_query_dsl::Like, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "like")
    }
    #[doc = "Parses the value as the `array` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_array(
        &self,
    ) -> std::result::Result<
        Vec<crate::types::common_query_dsl::Like>,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "array")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MultiMatchQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "Analyzer used to convert the text in the query value into tokens."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    #[doc = "If `true`, match phrase queries are automatically created for multi-term synonyms."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_generate_synonyms_phrase_query: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cutoff_frequency: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzziness: Option<crate::types::common::Fuzziness>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzzy_rewrite: Option<crate::types::common::MultiTermQueryRewrite>,
    #[doc = "If `true`, edits for fuzzy matching include transpositions of two adjacent characters (for example, `ab` to `ba`). Can be applied to the term subqueries constructed for all terms but the final term."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzzy_transpositions: Option<bool>,
    #[doc = "If `true`, format-based errors, such as providing a text query value for a numeric field, are ignored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lenient: Option<bool>,
    #[doc = "Maximum number of terms to which the query will expand."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_expansions: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_should_match: Option<crate::types::common::MinimumShouldMatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<crate::types::common_query_dsl::Operator>,
    #[doc = "Number of beginning characters left unchanged for fuzzy matching."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix_length: Option<i32>,
    #[doc = "Text, number, Boolean value or date you wish to find in the provided field."]
    pub query: String,
    #[doc = "Maximum number of positions allowed between matching tokens."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slop: Option<i32>,
    #[doc = "Determines how scores for each per-term blended query and scores across groups are combined."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tie_breaker: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::types::common_query_dsl::TextQueryType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zero_terms_query: Option<crate::types::common_query_dsl::ZeroTermsQuery>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MultiValueMode {
    #[serde(rename = "avg")]
    Avg,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "sum")]
    Sum,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "Indicates whether to ignore an unmapped path and not return any documents instead of an error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inner_hits: Option<Box<crate::types::search::InnerHits>>,
    pub path: crate::types::common::Field,
    pub query: Box<crate::types::common_query_dsl::QueryContainer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score_mode: Option<crate::types::common_query_dsl::ChildScoreMode>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NeuralQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<crate::types::common_query_dsl::QueryContainer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub k: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_distance: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_text: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NumberRangeQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation: Option<crate::types::common_query_dsl::RangeRelation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<crate::types::common_query_dsl::NumberRangeQueryFrom>,
    #[doc = "Greater than."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gt: Option<f64>,
    #[doc = "Greater than or equal to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gte: Option<f64>,
    #[doc = "Include the lower bound"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_lower: Option<bool>,
    #[doc = "Include the upper bound"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_upper: Option<bool>,
    #[doc = "Less than."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lt: Option<f64>,
    #[doc = "Less than or equal to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lte: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<crate::types::common_query_dsl::NumberRangeQueryTo>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum NumberRangeQueryFrom {
    Number(f64),
    String(String),
    #[doc = "An explicit null value."]
    Null,
}
impl<'de> serde::Deserialize<'de> for NumberRangeQueryFrom {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = NumberRangeQueryFrom;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NumberRangeQueryFrom value")
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::I64Deserializer::new(v))
                    .map(NumberRangeQueryFrom::Number)
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::U64Deserializer::new(v))
                    .map(NumberRangeQueryFrom::Number)
            }
            fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::F64Deserializer::new(v))
                    .map(NumberRangeQueryFrom::Number)
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(NumberRangeQueryFrom::String)
            }
            fn visit_unit<E>(self) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NumberRangeQueryFrom::Null)
            }
            fn visit_none<E>(self) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NumberRangeQueryFrom::Null)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum NumberRangeQueryTo {
    Number(f64),
    String(String),
    #[doc = "An explicit null value."]
    Null,
}
impl<'de> serde::Deserialize<'de> for NumberRangeQueryTo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = NumberRangeQueryTo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NumberRangeQueryTo value")
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::I64Deserializer::new(v))
                    .map(NumberRangeQueryTo::Number)
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::U64Deserializer::new(v))
                    .map(NumberRangeQueryTo::Number)
            }
            fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::F64Deserializer::new(v))
                    .map(NumberRangeQueryTo::Number)
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(NumberRangeQueryTo::String)
            }
            fn visit_unit<E>(self) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NumberRangeQueryTo::Null)
            }
            fn visit_none<E>(self) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NumberRangeQueryTo::Null)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NumericDecayPlacement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decay: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
    pub origin: f64,
    pub scale: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Operator {
    #[serde(rename = "and")]
    And,
    #[serde(rename = "or")]
    Or,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParentIdQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::types::common::Id>,
    #[doc = "Indicates whether to ignore an unmapped `type` and not return any documents instead of an error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::types::common::RelationName>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PercolateQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "The source of the document being percolated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<Box<serde_json::value::RawValue>>,
    #[doc = "An array of sources of the documents being percolated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<Box<serde_json::value::RawValue>>>,
    pub field: crate::types::common::Field,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::types::common::Id>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<crate::types::common::IndexName>,
    #[doc = "The suffix used for the `_percolator_document_slot` field when multiple `percolate` queries are specified."]
    #[serde(rename = "name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _name: Option<String>,
    #[doc = "Preference used to fetch document to percolate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing: Option<crate::types::common::Routing>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionNumber>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum PrefixQuery {
    Value(String),
    Object(crate::types::common_query_dsl::PrefixQueryObject),
}
impl<'de> serde::Deserialize<'de> for PrefixQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = PrefixQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PrefixQuery value")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(PrefixQuery::Value)
            }
            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(PrefixQuery::Object)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PrefixQueryObject {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "Allows ASCII case insensitive matching of the value with the indexed field values when set to `true`. Default is `false` which means the case sensitivity of matching depends on the underlying field's mapping."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub case_insensitive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rewrite: Option<crate::types::common::MultiTermQueryRewrite>,
    #[doc = "Beginning characters of terms you wish to find in the provided field."]
    pub value: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryContainer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agentic: Option<crate::types::common_query_dsl::AgenticQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bool: Option<Box<crate::types::common_query_dsl::BoolQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boosting: Option<Box<crate::types::common_query_dsl::BoostingQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub combined_fields: Option<crate::types::common_query_dsl::CombinedFieldsQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common: Option<
        std::collections::BTreeMap<String, crate::types::common_query_dsl::CommonTermsQuery>,
    >,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constant_score: Option<Box<crate::types::common_query_dsl::ConstantScoreQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dis_max: Option<crate::types::common_query_dsl::DisMaxQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distance_feature: Option<crate::types::common_query_dsl::DistanceFeatureQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exists: Option<crate::types::common_query_dsl::ExistsQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_masking_span: Option<Box<crate::types::common_query_dsl::SpanFieldMaskingQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function_score: Option<Box<crate::types::common_query_dsl::FunctionScoreQuery>>,
    #[doc = "Returns documents that contain terms similar to the search term, as measured by a Levenshtein edit distance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzzy:
        Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::FuzzyQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geo_bounding_box: Option<crate::types::common_query_dsl::GeoBoundingBoxQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geo_distance: Option<crate::types::common_query_dsl::GeoDistanceQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geo_polygon: Option<crate::types::common_query_dsl::GeoPolygonQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geo_shape: Option<crate::types::common_query_dsl::GeoShapeQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_child: Option<Box<crate::types::common_query_dsl::HasChildQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_parent: Option<Box<crate::types::common_query_dsl::HasParentQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hybrid: Option<Box<crate::types::common_query_dsl::HybridQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ids: Option<crate::types::common_query_dsl::IdsQuery>,
    #[doc = "Returns documents based on the order and proximity of matching terms."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intervals:
        Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::IntervalsQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub knn: Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::KnnQuery>>,
    #[doc = "Returns documents that match a provided text, number, date or Boolean value. The provided text is analyzed before matching."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#match:
        Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::MatchQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_all: Option<crate::types::common_query_dsl::MatchAllQuery>,
    #[doc = "Analyzes its input and constructs a `bool` query from the terms. Each term except the last is used in a `term` query. The last term is used in a prefix query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_bool_prefix: Option<
        std::collections::BTreeMap<String, crate::types::common_query_dsl::MatchBoolPrefixQuery>,
    >,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_none: Option<crate::types::common_query_dsl::MatchNoneQuery>,
    #[doc = "Analyzes the text and creates a phrase query out of the analyzed text."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_phrase: Option<
        std::collections::BTreeMap<String, crate::types::common_query_dsl::MatchPhraseQuery>,
    >,
    #[doc = "Returns documents that contain the words of a provided text, in the same order as provided. The last term of the provided text is treated as a prefix, matching any words that begin with that term."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_phrase_prefix: Option<
        std::collections::BTreeMap<String, crate::types::common_query_dsl::MatchPhrasePrefixQuery>,
    >,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub more_like_this: Option<crate::types::common_query_dsl::MoreLikeThisQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_match: Option<crate::types::common_query_dsl::MultiMatchQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nested: Option<Box<crate::types::common_query_dsl::NestedQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neural:
        Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::NeuralQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<crate::types::common_query_dsl::ParentIdQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percolate: Option<crate::types::common_query_dsl::PercolateQuery>,
    #[doc = "Returns documents that contain a specific prefix in a provided field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix:
        Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::PrefixQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_string: Option<crate::types::common_query_dsl::QueryStringQuery>,
    #[doc = "Returns documents that contain terms within a provided range."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range:
        Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::RangeQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank_feature: Option<crate::types::common_query_dsl::RankFeatureQuery>,
    #[doc = "Returns documents that contain terms matching a regular expression."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regexp:
        Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::RegexpQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<crate::types::common_query_dsl::ScriptQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script_score: Option<Box<crate::types::common_query_dsl::ScriptScoreQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub simple_query_string: Option<crate::types::common_query_dsl::SimpleQueryStringQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_containing: Option<Box<crate::types::common_query_dsl::SpanContainingQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_first: Option<Box<crate::types::common_query_dsl::SpanFirstQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_multi: Option<Box<crate::types::common_query_dsl::SpanMultiTermQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_near: Option<crate::types::common_query_dsl::SpanNearQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_not: Option<Box<crate::types::common_query_dsl::SpanNotQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_or: Option<crate::types::common_query_dsl::SpanOrQuery>,
    #[doc = "Matches spans containing a term."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_term:
        Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::SpanTermQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_within: Option<Box<crate::types::common_query_dsl::SpanWithinQuery>>,
    #[doc = "Return documents using a template query contains placeholders that are resolved by search request processors during query execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "Returns documents that contain an exact term in a provided field. To return a document, the query term must exactly match the queried field's value, including white space and capitalization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::TermQuery>>,
    #[doc = "Returns documents that contain one or more exact terms in a provided field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terms: Option<crate::types::common_query_dsl::TermsQuery>,
    #[doc = "Returns documents that contain a minimum number of exact terms in a provided field. To return a document, a required number of terms must exactly match the field values, including white space and capitalization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terms_set:
        Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::TermsSetQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::types::common_query_dsl::TypeQuery>,
    #[doc = "Returns documents that contain terms matching a wildcard pattern."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wildcard:
        Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::WildcardQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wrapper: Option<crate::types::common_query_dsl::WrapperQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xy_shape: Option<crate::types::common_query_dsl::XyShapeQuery>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryStringQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "If `true`, the wildcard characters `*` and `?` are allowed as the first character of the query string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_leading_wildcard: Option<bool>,
    #[doc = "If `true`, the query attempts to analyze wildcard terms in the query string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyze_wildcard: Option<bool>,
    #[doc = "Analyzer used to convert text in the query string into tokens."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    #[doc = "If `true`, match phrase queries are automatically created for multi-term synonyms."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_generate_synonyms_phrase_query: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_field: Option<crate::types::common::Field>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_operator: Option<crate::types::common_query_dsl::Operator>,
    #[doc = "If `true`, enable position increments in queries constructed from a `query_string` search."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_position_increments: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub escape: Option<bool>,
    #[doc = "Array of fields to search. Supports wildcards (`*`)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<crate::types::common::Field>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzziness: Option<crate::types::common::Fuzziness>,
    #[doc = "Maximum number of terms to which the query expands for fuzzy matching."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzzy_max_expansions: Option<i32>,
    #[doc = "Number of beginning characters left unchanged for fuzzy matching."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzzy_prefix_length: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzzy_rewrite: Option<crate::types::common::MultiTermQueryRewrite>,
    #[doc = "If `true`, edits for fuzzy matching include transpositions of two adjacent characters (for example, `ab` to `ba`)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzzy_transpositions: Option<bool>,
    #[doc = "If `true`, format-based errors, such as providing a text value for a numeric field, are ignored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lenient: Option<bool>,
    #[doc = "Maximum number of automaton states required for the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_determinized_states: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_should_match: Option<crate::types::common::MinimumShouldMatch>,
    #[doc = "Maximum number of positions allowed between matching tokens for phrases."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phrase_slop: Option<i32>,
    #[doc = "Query string you wish to parse and use for search."]
    pub query: String,
    #[doc = "Analyzer used to convert quoted text in the query string into tokens. For quoted text, this parameter overrides the analyzer specified in the `analyzer` parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_analyzer: Option<String>,
    #[doc = "Suffix appended to quoted text in the query string. You can use this suffix to use a different analysis method for exact matches."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_field_suffix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rewrite: Option<crate::types::common::MultiTermQueryRewrite>,
    #[doc = "How to combine the queries generated from the individual search terms in the resulting `dis_max` query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tie_breaker: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<crate::types::common::TimeZone>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::types::common_query_dsl::TextQueryType>,
}

pub type QueryVector = Vec<f64>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RandomScoreFunction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<crate::types::common::Field>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed: Option<crate::types::common_query_dsl::RandomScoreFunctionSeed>,
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct RandomScoreFunctionSeed {
    raw: Box<serde_json::value::RawValue>,
}
impl RandomScoreFunctionSeed {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `integer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_integer(&self) -> std::result::Result<i32, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "integer")
    }
    #[doc = "Parses the value as the `integer_integer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_integer_integer(&self) -> std::result::Result<i64, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "integer_integer")
    }
    #[doc = "Parses the value as the `string` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_string(&self) -> std::result::Result<String, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "string")
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct RangeQuery {
    raw: Box<serde_json::value::RawValue>,
}
impl RangeQuery {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `number_range_query` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_number_range_query(
        &self,
    ) -> std::result::Result<
        crate::types::common_query_dsl::NumberRangeQuery,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "number_range_query")
    }
    #[doc = "Parses the value as the `date_range_query` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_date_range_query(
        &self,
    ) -> std::result::Result<
        crate::types::common_query_dsl::DateRangeQuery,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "date_range_query")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RangeRelation {
    #[serde(rename = "contains")]
    Contains,
    #[serde(rename = "intersects")]
    Intersects,
    #[serde(rename = "within")]
    Within,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

pub type RankFeatureFunctionLinear = std::collections::BTreeMap<String, serde_json::Value>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RankFeatureFunctionLogarithm {
    #[doc = "Configurable scaling factor."]
    pub scaling_factor: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RankFeatureFunctionSaturation {
    #[doc = "Configurable pivot value so that the result will be less than 0.5."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pivot: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RankFeatureFunctionSigmoid {
    #[doc = "Configurable Exponent."]
    pub exponent: f64,
    #[doc = "Configurable pivot value so that the result will be less than 0.5."]
    pub pivot: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RankFeatureQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    pub field: crate::types::common::Field,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linear: Option<crate::types::common_query_dsl::RankFeatureFunctionLinear>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log: Option<crate::types::common_query_dsl::RankFeatureFunctionLogarithm>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saturation: Option<crate::types::common_query_dsl::RankFeatureFunctionSaturation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sigmoid: Option<crate::types::common_query_dsl::RankFeatureFunctionSigmoid>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum RegexpQuery {
    Value(String),
    Object(crate::types::common_query_dsl::RegexpQueryObject),
}
impl<'de> serde::Deserialize<'de> for RegexpQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = RegexpQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RegexpQuery value")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(RegexpQuery::Value)
            }
            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(RegexpQuery::Object)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegexpQueryObject {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "Allows case insensitive matching of the regular expression value with the indexed field values when set to `true`. When `false`, case sensitivity of matching depends on the underlying field's mapping."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub case_insensitive: Option<bool>,
    #[doc = "Enables optional operators for the regular expression."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
    #[doc = "Maximum number of automaton states required for the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_determinized_states: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rewrite: Option<crate::types::common::MultiTermQueryRewrite>,
    #[doc = "Regular expression for terms you wish to find in the provided field."]
    pub value: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RescoreContext {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oversample_factor: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScriptQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    pub script: crate::types::common::Script,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScriptScoreFunction {
    pub script: crate::types::common::Script,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScriptScoreQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "Documents with a score lower than this floating point number are excluded from the search results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_score: Option<f64>,
    pub query: Box<crate::types::common_query_dsl::QueryContainer>,
    pub script: crate::types::common::Script,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SimpleQueryStringFlag {
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "AND")]
    And,
    #[serde(rename = "ESCAPE")]
    Escape,
    #[serde(rename = "FUZZY")]
    Fuzzy,
    #[serde(rename = "NEAR")]
    Near,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "NOT")]
    Not,
    #[serde(rename = "OR")]
    Or,
    #[serde(rename = "PHRASE")]
    Phrase,
    #[serde(rename = "PRECEDENCE")]
    Precedence,
    #[serde(rename = "PREFIX")]
    Prefix,
    #[serde(rename = "SLOP")]
    Slop,
    #[serde(rename = "WHITESPACE")]
    Whitespace,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[doc = "Query flags can be either a single flag or a combination of flags (e.g. `OR|AND|PREFIX`)."]
#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SimpleQueryStringFlags {
    raw: Box<serde_json::value::RawValue>,
}
impl SimpleQueryStringFlags {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `single` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_single(
        &self,
    ) -> std::result::Result<
        crate::types::common_query_dsl::SimpleQueryStringFlag,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "single")
    }
    #[doc = "Parses the value as the `multiple` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_multiple(&self) -> std::result::Result<String, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "multiple")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimpleQueryStringQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "If `true`, the query attempts to analyze wildcard terms in the query string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyze_wildcard: Option<bool>,
    #[doc = "Analyzer used to convert text in the query string into tokens."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    #[doc = "If `true`, the parser creates a `match_phrase` query for each multi-position token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_generate_synonyms_phrase_query: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_operator: Option<crate::types::common_query_dsl::Operator>,
    #[doc = "Array of fields you wish to search. Accepts wildcard expressions. You also can boost relevance scores for matches to particular fields using a caret (`^`) notation. Defaults to the `index.query.default_field index` setting, which has a default value of `*`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<crate::types::common::Field>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<crate::types::common_query_dsl::SimpleQueryStringFlags>,
    #[doc = "Maximum number of terms to which the query expands for fuzzy matching."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzzy_max_expansions: Option<i32>,
    #[doc = "Number of beginning characters left unchanged for fuzzy matching."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzzy_prefix_length: Option<i32>,
    #[doc = "If `true`, edits for fuzzy matching include transpositions of two adjacent characters (for example, `ab` to `ba`)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzzy_transpositions: Option<bool>,
    #[doc = "If `true`, format-based errors, such as providing a text value for a numeric field, are ignored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lenient: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_should_match: Option<crate::types::common::MinimumShouldMatch>,
    #[doc = "Query string in the simple query string syntax you wish to parse and use for search."]
    pub query: String,
    #[doc = "Suffix appended to quoted text in the query string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_field_suffix: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpanContainingQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    pub big: Box<crate::types::common_query_dsl::SpanQuery>,
    pub little: Box<crate::types::common_query_dsl::SpanQuery>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpanFieldMaskingQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    pub field: crate::types::common::Field,
    pub query: Box<crate::types::common_query_dsl::SpanQuery>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpanFirstQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "Controls the maximum end position permitted in a match."]
    pub end: i32,
    pub r#match: Box<crate::types::common_query_dsl::SpanQuery>,
}

#[doc = "Can only be used as a clause in a `span_near` query."]
pub type SpanGapQuery = std::collections::BTreeMap<String, i32>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpanMultiTermQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    pub r#match: Box<crate::types::common_query_dsl::QueryContainer>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpanNearQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "Array of one or more other span type queries."]
    pub clauses: Vec<crate::types::common_query_dsl::SpanQuery>,
    #[doc = "Controls whether matches are required to be in-order."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_order: Option<bool>,
    #[doc = "Controls the maximum number of intervening unmatched positions permitted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slop: Option<i32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpanNotQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "The number of tokens from within the include span that can't have overlap with the exclude span. Equivalent to setting both `pre` and `post`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dist: Option<i32>,
    pub exclude: Box<crate::types::common_query_dsl::SpanQuery>,
    pub include: Box<crate::types::common_query_dsl::SpanQuery>,
    #[doc = "The number of tokens after the include span that can't have overlap with the exclude span."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post: Option<i32>,
    #[doc = "The number of tokens before the include span that can't have overlap with the exclude span."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre: Option<i32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpanOrQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "Array of one or more other span type queries."]
    pub clauses: Vec<crate::types::common_query_dsl::SpanQuery>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpanQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_masking_span: Option<Box<crate::types::common_query_dsl::SpanFieldMaskingQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_containing: Option<Box<crate::types::common_query_dsl::SpanContainingQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_first: Option<Box<crate::types::common_query_dsl::SpanFirstQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_gap: Option<crate::types::common_query_dsl::SpanGapQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_multi: Option<Box<crate::types::common_query_dsl::SpanMultiTermQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_near: Option<crate::types::common_query_dsl::SpanNearQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_not: Option<Box<crate::types::common_query_dsl::SpanNotQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_or: Option<crate::types::common_query_dsl::SpanOrQuery>,
    #[doc = "The equivalent of the `term` query but for use with other span queries."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_term:
        Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::SpanTermQuery>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_within: Option<Box<crate::types::common_query_dsl::SpanWithinQuery>>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SpanTermQuery {
    Value(String),
    Object(crate::types::common_query_dsl::SpanTermQueryObject),
}
impl<'de> serde::Deserialize<'de> for SpanTermQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = SpanTermQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SpanTermQuery value")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(SpanTermQuery::Value)
            }
            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(SpanTermQuery::Object)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpanTermQueryObject {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    pub value: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpanWithinQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    pub big: Box<crate::types::common_query_dsl::SpanQuery>,
    pub little: Box<crate::types::common_query_dsl::SpanQuery>,
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TermQuery {
    raw: Box<serde_json::value::RawValue>,
}
impl TermQuery {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `value` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_value(
        &self,
    ) -> std::result::Result<crate::types::common::FieldValue, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "value")
    }
    #[doc = "Parses the value as the `object` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_object(
        &self,
    ) -> std::result::Result<
        crate::types::common_query_dsl::TermQueryObject,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "object")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TermQueryObject {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "Allows ASCII case insensitive matching of the value with the indexed field values when set to `true`. When `false`, the case sensitivity of matching depends on the underlying field's mapping."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub case_insensitive: Option<bool>,
    pub value: crate::types::common::FieldValue,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TermsLookup {
    pub id: crate::types::common::Id,
    pub index: crate::types::common::IndexName,
    pub path: crate::types::common::Field,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing: Option<crate::types::common::Routing>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct TermsQuery {
    pub name: Option<String>,
    pub boost: Option<f64>,
    #[doc = "Specifies the types of values used for filtering. Valid values are `default` and `bitmap`. Default is `default`."]
    pub value_type: Option<crate::types::common_query_dsl::TermsQueryValueType>,
    pub additional:
        std::collections::BTreeMap<String, crate::types::common_query_dsl::TermsQueryField>,
}
impl<'de> serde::Deserialize<'de> for TermsQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TermsQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TermsQuery object")
            }
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut f_boost: Option<f64> = None;
                let mut f_value_type: Option<crate::types::common_query_dsl::TermsQueryValueType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_query_dsl::TermsQueryField,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => f_name = map.next_value::<Option<String>>()?,
                        "boost" => f_boost = map.next_value::<Option<f64>>()?,
                        "value_type" => f_value_type = map.next_value::<Option<crate::types::common_query_dsl::TermsQueryValueType>>()?,
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TermsQuery {
                    name: f_name,
                    boost: f_boost,
                    value_type: f_value_type,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TermsQuery {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.name {
            map.serialize_entry("_name", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.value_type {
            map.serialize_entry("value_type", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum TermsQueryField {
    Value(Vec<crate::types::common::FieldValue>),
    Lookup(crate::types::common_query_dsl::TermsLookup),
}
impl<'de> serde::Deserialize<'de> for TermsQueryField {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = TermsQueryField;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TermsQueryField value")
            }
            fn visit_seq<A>(self, seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
                    .map(TermsQueryField::Value)
            }
            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(TermsQueryField::Lookup)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TermsQueryValueType {
    #[serde(rename = "bitmap")]
    Bitmap,
    #[serde(rename = "default")]
    Default,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TermsSetQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_should_match_field: Option<crate::types::common::Field>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_should_match_script: Option<crate::types::common::Script>,
    pub terms: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TextQueryType {
    #[serde(rename = "best_fields")]
    BestFields,
    #[serde(rename = "bool_prefix")]
    BoolPrefix,
    #[serde(rename = "cross_fields")]
    CrossFields,
    #[serde(rename = "most_fields")]
    MostFields,
    #[serde(rename = "phrase")]
    Phrase,
    #[serde(rename = "phrase_prefix")]
    PhrasePrefix,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TypeQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    pub value: String,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum WildcardQuery {
    Value(String),
    Object(crate::types::common_query_dsl::WildcardQueryObject),
}
impl<'de> serde::Deserialize<'de> for WildcardQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = WildcardQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a WildcardQuery value")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(WildcardQuery::Value)
            }
            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(WildcardQuery::Object)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WildcardQueryObject {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "Allows case insensitive matching of the pattern with the indexed field values when set to `true`. Default is `false` which means the case sensitivity of matching depends on the underlying field's mapping."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub case_insensitive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rewrite: Option<crate::types::common::MultiTermQueryRewrite>,
    #[doc = "Wildcard pattern for terms you wish to find in the provided field. Required, when wildcard is not set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Wildcard pattern for terms you wish to find in the provided field. Required, when value is not set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wildcard: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WrapperQuery {
    #[serde(rename = "_name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[doc = "A base64 encoded query. The binary data format can be any of JSON, YAML, CBOR or SMILE encodings"]
    pub query: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct XyShape {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<Vec<Box<serde_json::value::RawValue>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct XyShapeQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub ignore_unmapped: Option<bool>,
    pub additional:
        std::collections::BTreeMap<String, crate::types::common_query_dsl::XyShapeQueryField>,
}
impl<'de> serde::Deserialize<'de> for XyShapeQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = XyShapeQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a XyShapeQuery object")
            }
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut f_boost: Option<f64> = None;
                let mut f_ignore_unmapped: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_query_dsl::XyShapeQueryField,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => f_name = map.next_value::<Option<String>>()?,
                        "boost" => f_boost = map.next_value::<Option<f64>>()?,
                        "ignore_unmapped" => {
                            f_ignore_unmapped = map.next_value::<Option<bool>>()?
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(XyShapeQuery {
                    name: f_name,
                    boost: f_boost,
                    ignore_unmapped: f_ignore_unmapped,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for XyShapeQuery {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.name {
            map.serialize_entry("_name", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.ignore_unmapped {
            map.serialize_entry("ignore_unmapped", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct XyShapeQueryField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexed_shape: Option<crate::types::common_query_dsl::FieldLookup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation: Option<crate::types::common::GeoShapeRelation>,
    pub shape: crate::types::common_query_dsl::XyShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ZeroTermsQuery {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}
