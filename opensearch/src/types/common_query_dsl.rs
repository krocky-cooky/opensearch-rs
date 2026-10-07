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

#[derive(Debug, Clone)]
pub struct AgenticQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "The memory ID from a previous response, used to continue a conversation with prior context. Applicable for conversational agents only."]
    pub memory_id: Option<String>,
    #[doc = "A list of index fields that the agent should consider when generating the search query."]
    pub query_fields: Option<Vec<crate::types::common::Field>>,
    #[doc = "The natural language question to be answered by the agent."]
    pub query_text: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AgenticQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AgenticQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AgenticQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_memory_id: Option<String> = None;
                let mut seen_memory_id = false;
                let mut f_query_fields: Option<Vec<crate::types::common::Field>> = None;
                let mut seen_query_fields = false;
                let mut f_query_text: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "memory_id" => {
                            if seen_memory_id {
                                return Err(serde::de::Error::duplicate_field("memory_id"));
                            }
                            seen_memory_id = true;
                            f_memory_id = map.next_value::<Option<String>>()?;
                        }
                        "query_fields" => {
                            if seen_query_fields {
                                return Err(serde::de::Error::duplicate_field("query_fields"));
                            }
                            seen_query_fields = true;
                            f_query_fields =
                                map.next_value::<Option<Vec<crate::types::common::Field>>>()?;
                        }
                        "query_text" => {
                            if f_query_text.is_some() {
                                return Err(serde::de::Error::duplicate_field("query_text"));
                            }
                            f_query_text = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AgenticQuery {
                    name: f_name,
                    boost: f_boost,
                    memory_id: f_memory_id,
                    query_fields: f_query_fields,
                    query_text: f_query_text
                        .ok_or_else(|| serde::de::Error::missing_field("query_text"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AgenticQuery {
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
        if let Some(v) = &self.memory_id {
            map.serialize_entry("memory_id", v)?;
        }
        if let Some(v) = &self.query_fields {
            map.serialize_entry("query_fields", v)?;
        }
        map.serialize_entry("query_text", &self.query_text)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct BoolQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "Ensures correct behavior when a query contains only `must_not` clauses. By default set to true, OpenSearch adds a match-all clause to ensure results are returned from Lucene, with the `must_not` conditions applied as filters. If set to false, the query may return no results, as Lucene typically requires at least one positive condition."]
    pub adjust_pure_negative: Option<bool>,
    #[doc = "The clause (query) must appear in matching documents. However, unlike `must`, the score of the query will be ignored."]
    pub filter: Option<Box<crate::types::common_query_dsl::BoolQueryFilter>>,
    pub minimum_should_match: Option<crate::types::common::MinimumShouldMatch>,
    #[doc = "The clause (query) must appear in matching documents and will contribute to the score."]
    pub must: Option<Box<crate::types::common_query_dsl::BoolQueryMust>>,
    #[doc = "The clause (query) must not appear in the matching documents. Because scoring is ignored, a score of `0` is returned for all documents."]
    pub must_not: Option<Box<crate::types::common_query_dsl::BoolQueryMustNot>>,
    #[doc = "The clause (query) should appear in the matching document."]
    pub should: Option<Box<crate::types::common_query_dsl::BoolQueryShould>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for BoolQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = BoolQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a BoolQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_adjust_pure_negative: Option<bool> = None;
                let mut seen_adjust_pure_negative = false;
                let mut f_filter: Option<Box<crate::types::common_query_dsl::BoolQueryFilter>> =
                    None;
                let mut seen_filter = false;
                let mut f_minimum_should_match: Option<crate::types::common::MinimumShouldMatch> =
                    None;
                let mut seen_minimum_should_match = false;
                let mut f_must: Option<Box<crate::types::common_query_dsl::BoolQueryMust>> = None;
                let mut seen_must = false;
                let mut f_must_not: Option<Box<crate::types::common_query_dsl::BoolQueryMustNot>> =
                    None;
                let mut seen_must_not = false;
                let mut f_should: Option<Box<crate::types::common_query_dsl::BoolQueryShould>> =
                    None;
                let mut seen_should = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "adjust_pure_negative" => {
                            if seen_adjust_pure_negative {
                                return Err(serde::de::Error::duplicate_field(
                                    "adjust_pure_negative",
                                ));
                            }
                            seen_adjust_pure_negative = true;
                            f_adjust_pure_negative = map.next_value::<Option<bool>>()?;
                        }
                        "filter" => {
                            if seen_filter {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            seen_filter = true;
                            f_filter = map.next_value::<Option<Box<crate::types::common_query_dsl::BoolQueryFilter>>>()?;
                        }
                        "minimum_should_match" => {
                            if seen_minimum_should_match {
                                return Err(serde::de::Error::duplicate_field(
                                    "minimum_should_match",
                                ));
                            }
                            seen_minimum_should_match = true;
                            f_minimum_should_match = map
                                .next_value::<Option<crate::types::common::MinimumShouldMatch>>()?;
                        }
                        "must" => {
                            if seen_must {
                                return Err(serde::de::Error::duplicate_field("must"));
                            }
                            seen_must = true;
                            f_must = map.next_value::<Option<Box<crate::types::common_query_dsl::BoolQueryMust>>>()?;
                        }
                        "must_not" => {
                            if seen_must_not {
                                return Err(serde::de::Error::duplicate_field("must_not"));
                            }
                            seen_must_not = true;
                            f_must_not = map.next_value::<Option<Box<crate::types::common_query_dsl::BoolQueryMustNot>>>()?;
                        }
                        "should" => {
                            if seen_should {
                                return Err(serde::de::Error::duplicate_field("should"));
                            }
                            seen_should = true;
                            f_should = map.next_value::<Option<Box<crate::types::common_query_dsl::BoolQueryShould>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(BoolQuery {
                    name: f_name,
                    boost: f_boost,
                    adjust_pure_negative: f_adjust_pure_negative,
                    filter: f_filter,
                    minimum_should_match: f_minimum_should_match,
                    must: f_must,
                    must_not: f_must_not,
                    should: f_should,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for BoolQuery {
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
        if let Some(v) = &self.adjust_pure_negative {
            map.serialize_entry("adjust_pure_negative", v)?;
        }
        if let Some(v) = &self.filter {
            map.serialize_entry("filter", v)?;
        }
        if let Some(v) = &self.minimum_should_match {
            map.serialize_entry("minimum_should_match", v)?;
        }
        if let Some(v) = &self.must {
            map.serialize_entry("must", v)?;
        }
        if let Some(v) = &self.must_not {
            map.serialize_entry("must_not", v)?;
        }
        if let Some(v) = &self.should {
            map.serialize_entry("should", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'{') => serde_json::from_str(text).map(BoolQueryFilter::QueryContainer),
            Some(b'[') => serde_json::from_str(text).map(BoolQueryFilter::Array),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a BoolQueryFilter value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'{') => serde_json::from_str(text).map(BoolQueryMust::QueryContainer),
            Some(b'[') => serde_json::from_str(text).map(BoolQueryMust::Array),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a BoolQueryMust value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'{') => serde_json::from_str(text).map(BoolQueryMustNot::QueryContainer),
            Some(b'[') => serde_json::from_str(text).map(BoolQueryMustNot::Array),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a BoolQueryMustNot value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'{') => serde_json::from_str(text).map(BoolQueryShould::QueryContainer),
            Some(b'[') => serde_json::from_str(text).map(BoolQueryShould::Array),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a BoolQueryShould value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct BoostingQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub negative: Box<crate::types::common_query_dsl::QueryContainer>,
    #[doc = "Floating point number between 0 and 1.0 used to decrease the relevance scores of documents matching the `negative` query."]
    pub negative_boost: f64,
    pub positive: Box<crate::types::common_query_dsl::QueryContainer>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for BoostingQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = BoostingQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a BoostingQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_negative: Option<Box<crate::types::common_query_dsl::QueryContainer>> =
                    None;
                let mut f_negative_boost: Option<f64> = None;
                let mut f_positive: Option<Box<crate::types::common_query_dsl::QueryContainer>> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "negative" => {
                            if f_negative.is_some() {
                                return Err(serde::de::Error::duplicate_field("negative"));
                            }
                            f_negative = Some(map.next_value::<Box<crate::types::common_query_dsl::QueryContainer>>()?);
                        }
                        "negative_boost" => {
                            if f_negative_boost.is_some() {
                                return Err(serde::de::Error::duplicate_field("negative_boost"));
                            }
                            f_negative_boost = Some(map.next_value::<f64>()?);
                        }
                        "positive" => {
                            if f_positive.is_some() {
                                return Err(serde::de::Error::duplicate_field("positive"));
                            }
                            f_positive = Some(map.next_value::<Box<crate::types::common_query_dsl::QueryContainer>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(BoostingQuery {
                    name: f_name,
                    boost: f_boost,
                    negative: f_negative
                        .ok_or_else(|| serde::de::Error::missing_field("negative"))?,
                    negative_boost: f_negative_boost
                        .ok_or_else(|| serde::de::Error::missing_field("negative_boost"))?,
                    positive: f_positive
                        .ok_or_else(|| serde::de::Error::missing_field("positive"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for BoostingQuery {
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
        map.serialize_entry("negative", &self.negative)?;
        map.serialize_entry("negative_boost", &self.negative_boost)?;
        map.serialize_entry("positive", &self.positive)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChildScoreMode {
    #[doc = "`avg`"]
    Avg,
    #[doc = "`max`"]
    Max,
    #[doc = "`min`"]
    Min,
    #[doc = "`none`"]
    None,
    #[doc = "`sum`"]
    Sum,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ChildScoreMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ChildScoreMode::Avg => "avg",
            ChildScoreMode::Max => "max",
            ChildScoreMode::Min => "min",
            ChildScoreMode::None => "none",
            ChildScoreMode::Sum => "sum",
            ChildScoreMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ChildScoreMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ChildScoreMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "avg" => ChildScoreMode::Avg,
            "max" => ChildScoreMode::Max,
            "min" => ChildScoreMode::Min,
            "none" => ChildScoreMode::None,
            "sum" => ChildScoreMode::Sum,
            _ => ChildScoreMode::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CombinedFieldsOperator {
    #[doc = "`and`"]
    And,
    #[doc = "`or`"]
    Or,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl CombinedFieldsOperator {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            CombinedFieldsOperator::And => "and",
            CombinedFieldsOperator::Or => "or",
            CombinedFieldsOperator::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for CombinedFieldsOperator {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CombinedFieldsOperator {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "and" => CombinedFieldsOperator::And,
            "or" => CombinedFieldsOperator::Or,
            _ => CombinedFieldsOperator::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct CombinedFieldsQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "If `true`, match phrase queries are automatically created for multi-term synonyms."]
    pub auto_generate_synonyms_phrase_query: Option<bool>,
    #[doc = "List of fields to search. Field wildcard patterns are allowed. Only `text` fields are supported, and they must all have the same search `analyzer`."]
    pub fields: Vec<crate::types::common::Field>,
    pub minimum_should_match: Option<crate::types::common::MinimumShouldMatch>,
    pub operator: Option<crate::types::common_query_dsl::CombinedFieldsOperator>,
    #[doc = "Text to search for in the provided `fields`. The `combined_fields` query analyzes the provided text before performing a search."]
    pub query: String,
    pub zero_terms_query: Option<crate::types::common_query_dsl::CombinedFieldsZeroTerms>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CombinedFieldsQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CombinedFieldsQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CombinedFieldsQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_auto_generate_synonyms_phrase_query: Option<bool> = None;
                let mut seen_auto_generate_synonyms_phrase_query = false;
                let mut f_fields: Option<Vec<crate::types::common::Field>> = None;
                let mut f_minimum_should_match: Option<crate::types::common::MinimumShouldMatch> =
                    None;
                let mut seen_minimum_should_match = false;
                let mut f_operator: Option<crate::types::common_query_dsl::CombinedFieldsOperator> =
                    None;
                let mut seen_operator = false;
                let mut f_query: Option<String> = None;
                let mut f_zero_terms_query: Option<
                    crate::types::common_query_dsl::CombinedFieldsZeroTerms,
                > = None;
                let mut seen_zero_terms_query = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "auto_generate_synonyms_phrase_query" => {
                            if seen_auto_generate_synonyms_phrase_query {
                                return Err(serde::de::Error::duplicate_field(
                                    "auto_generate_synonyms_phrase_query",
                                ));
                            }
                            seen_auto_generate_synonyms_phrase_query = true;
                            f_auto_generate_synonyms_phrase_query =
                                map.next_value::<Option<bool>>()?;
                        }
                        "fields" => {
                            if f_fields.is_some() {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            f_fields = Some(map.next_value::<Vec<crate::types::common::Field>>()?);
                        }
                        "minimum_should_match" => {
                            if seen_minimum_should_match {
                                return Err(serde::de::Error::duplicate_field(
                                    "minimum_should_match",
                                ));
                            }
                            seen_minimum_should_match = true;
                            f_minimum_should_match = map
                                .next_value::<Option<crate::types::common::MinimumShouldMatch>>()?;
                        }
                        "operator" => {
                            if seen_operator {
                                return Err(serde::de::Error::duplicate_field("operator"));
                            }
                            seen_operator = true;
                            f_operator = map.next_value::<Option<crate::types::common_query_dsl::CombinedFieldsOperator>>()?;
                        }
                        "query" => {
                            if f_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            f_query = Some(map.next_value::<String>()?);
                        }
                        "zero_terms_query" => {
                            if seen_zero_terms_query {
                                return Err(serde::de::Error::duplicate_field("zero_terms_query"));
                            }
                            seen_zero_terms_query = true;
                            f_zero_terms_query = map.next_value::<Option<crate::types::common_query_dsl::CombinedFieldsZeroTerms>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CombinedFieldsQuery {
                    name: f_name,
                    boost: f_boost,
                    auto_generate_synonyms_phrase_query: f_auto_generate_synonyms_phrase_query,
                    fields: f_fields.ok_or_else(|| serde::de::Error::missing_field("fields"))?,
                    minimum_should_match: f_minimum_should_match,
                    operator: f_operator,
                    query: f_query.ok_or_else(|| serde::de::Error::missing_field("query"))?,
                    zero_terms_query: f_zero_terms_query,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CombinedFieldsQuery {
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
        if let Some(v) = &self.auto_generate_synonyms_phrase_query {
            map.serialize_entry("auto_generate_synonyms_phrase_query", v)?;
        }
        map.serialize_entry("fields", &self.fields)?;
        if let Some(v) = &self.minimum_should_match {
            map.serialize_entry("minimum_should_match", v)?;
        }
        if let Some(v) = &self.operator {
            map.serialize_entry("operator", v)?;
        }
        map.serialize_entry("query", &self.query)?;
        if let Some(v) = &self.zero_terms_query {
            map.serialize_entry("zero_terms_query", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CombinedFieldsZeroTerms {
    #[doc = "`all`"]
    All,
    #[doc = "`none`"]
    None,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl CombinedFieldsZeroTerms {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            CombinedFieldsZeroTerms::All => "all",
            CombinedFieldsZeroTerms::None => "none",
            CombinedFieldsZeroTerms::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for CombinedFieldsZeroTerms {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CombinedFieldsZeroTerms {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "all" => CombinedFieldsZeroTerms::All,
            "none" => CombinedFieldsZeroTerms::None,
            _ => CombinedFieldsZeroTerms::Unknown(s),
        })
    }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'"') => serde_json::from_str(text).map(CommonTermsQuery::Query),
            Some(b'{') => serde_json::from_str(text).map(CommonTermsQuery::Object),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a CommonTermsQuery value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct CommonTermsQueryObject {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub analyzer: Option<String>,
    pub cutoff_frequency: Option<f64>,
    pub high_freq_operator: Option<crate::types::common_query_dsl::Operator>,
    pub low_freq_operator: Option<crate::types::common_query_dsl::Operator>,
    pub minimum_should_match: Option<crate::types::common::MinimumShouldMatch>,
    pub query: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CommonTermsQueryObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CommonTermsQueryObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CommonTermsQueryObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_analyzer: Option<String> = None;
                let mut seen_analyzer = false;
                let mut f_cutoff_frequency: Option<f64> = None;
                let mut seen_cutoff_frequency = false;
                let mut f_high_freq_operator: Option<crate::types::common_query_dsl::Operator> =
                    None;
                let mut seen_high_freq_operator = false;
                let mut f_low_freq_operator: Option<crate::types::common_query_dsl::Operator> =
                    None;
                let mut seen_low_freq_operator = false;
                let mut f_minimum_should_match: Option<crate::types::common::MinimumShouldMatch> =
                    None;
                let mut seen_minimum_should_match = false;
                let mut f_query: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "analyzer" => {
                            if seen_analyzer {
                                return Err(serde::de::Error::duplicate_field("analyzer"));
                            }
                            seen_analyzer = true;
                            f_analyzer = map.next_value::<Option<String>>()?;
                        }
                        "cutoff_frequency" => {
                            if seen_cutoff_frequency {
                                return Err(serde::de::Error::duplicate_field("cutoff_frequency"));
                            }
                            seen_cutoff_frequency = true;
                            f_cutoff_frequency = map.next_value::<Option<f64>>()?;
                        }
                        "high_freq_operator" => {
                            if seen_high_freq_operator {
                                return Err(serde::de::Error::duplicate_field(
                                    "high_freq_operator",
                                ));
                            }
                            seen_high_freq_operator = true;
                            f_high_freq_operator = map
                                .next_value::<Option<crate::types::common_query_dsl::Operator>>()?;
                        }
                        "low_freq_operator" => {
                            if seen_low_freq_operator {
                                return Err(serde::de::Error::duplicate_field("low_freq_operator"));
                            }
                            seen_low_freq_operator = true;
                            f_low_freq_operator = map
                                .next_value::<Option<crate::types::common_query_dsl::Operator>>()?;
                        }
                        "minimum_should_match" => {
                            if seen_minimum_should_match {
                                return Err(serde::de::Error::duplicate_field(
                                    "minimum_should_match",
                                ));
                            }
                            seen_minimum_should_match = true;
                            f_minimum_should_match = map
                                .next_value::<Option<crate::types::common::MinimumShouldMatch>>()?;
                        }
                        "query" => {
                            if f_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            f_query = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CommonTermsQueryObject {
                    name: f_name,
                    boost: f_boost,
                    analyzer: f_analyzer,
                    cutoff_frequency: f_cutoff_frequency,
                    high_freq_operator: f_high_freq_operator,
                    low_freq_operator: f_low_freq_operator,
                    minimum_should_match: f_minimum_should_match,
                    query: f_query.ok_or_else(|| serde::de::Error::missing_field("query"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CommonTermsQueryObject {
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
        if let Some(v) = &self.analyzer {
            map.serialize_entry("analyzer", v)?;
        }
        if let Some(v) = &self.cutoff_frequency {
            map.serialize_entry("cutoff_frequency", v)?;
        }
        if let Some(v) = &self.high_freq_operator {
            map.serialize_entry("high_freq_operator", v)?;
        }
        if let Some(v) = &self.low_freq_operator {
            map.serialize_entry("low_freq_operator", v)?;
        }
        if let Some(v) = &self.minimum_should_match {
            map.serialize_entry("minimum_should_match", v)?;
        }
        map.serialize_entry("query", &self.query)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ConstantScoreQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub filter: Box<crate::types::common_query_dsl::QueryContainer>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ConstantScoreQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ConstantScoreQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ConstantScoreQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_filter: Option<Box<crate::types::common_query_dsl::QueryContainer>> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "filter" => {
                            if f_filter.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            f_filter = Some(map.next_value::<Box<crate::types::common_query_dsl::QueryContainer>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ConstantScoreQuery {
                    name: f_name,
                    boost: f_boost,
                    filter: f_filter.ok_or_else(|| serde::de::Error::missing_field("filter"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ConstantScoreQuery {
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
        map.serialize_entry("filter", &self.filter)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct DateDecayPlacement {
    pub decay: Option<f64>,
    pub offset: Option<crate::types::common::Duration>,
    pub origin: Option<crate::types::common::DateTime>,
    pub scale: crate::types::common::Duration,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DateDecayPlacement {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DateDecayPlacement;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DateDecayPlacement object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_decay: Option<f64> = None;
                let mut seen_decay = false;
                let mut f_offset: Option<crate::types::common::Duration> = None;
                let mut seen_offset = false;
                let mut f_origin: Option<crate::types::common::DateTime> = None;
                let mut seen_origin = false;
                let mut f_scale: Option<crate::types::common::Duration> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "decay" => {
                            if seen_decay {
                                return Err(serde::de::Error::duplicate_field("decay"));
                            }
                            seen_decay = true;
                            f_decay = map.next_value::<Option<f64>>()?;
                        }
                        "offset" => {
                            if seen_offset {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            seen_offset = true;
                            f_offset =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "origin" => {
                            if seen_origin {
                                return Err(serde::de::Error::duplicate_field("origin"));
                            }
                            seen_origin = true;
                            f_origin =
                                map.next_value::<Option<crate::types::common::DateTime>>()?;
                        }
                        "scale" => {
                            if f_scale.is_some() {
                                return Err(serde::de::Error::duplicate_field("scale"));
                            }
                            f_scale = Some(map.next_value::<crate::types::common::Duration>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DateDecayPlacement {
                    decay: f_decay,
                    offset: f_offset,
                    origin: f_origin,
                    scale: f_scale.ok_or_else(|| serde::de::Error::missing_field("scale"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DateDecayPlacement {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.decay {
            map.serialize_entry("decay", v)?;
        }
        if let Some(v) = &self.offset {
            map.serialize_entry("offset", v)?;
        }
        if let Some(v) = &self.origin {
            map.serialize_entry("origin", v)?;
        }
        map.serialize_entry("scale", &self.scale)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct DateRangeQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub relation: Option<crate::types::common_query_dsl::RangeRelation>,
    pub format: Option<crate::types::common::DateFormat>,
    pub from: Option<crate::types::common::DateMath>,
    pub gt: Option<crate::types::common::DateMath>,
    pub gte: Option<crate::types::common::DateMath>,
    #[doc = "Include the lower bound"]
    pub include_lower: Option<bool>,
    #[doc = "Include the upper bound"]
    pub include_upper: Option<bool>,
    pub lt: Option<crate::types::common::DateMath>,
    pub lte: Option<crate::types::common::DateMath>,
    pub time_zone: Option<crate::types::common::TimeZone>,
    pub to: Option<crate::types::common::DateMath>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DateRangeQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DateRangeQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DateRangeQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_relation: Option<crate::types::common_query_dsl::RangeRelation> = None;
                let mut seen_relation = false;
                let mut f_format: Option<crate::types::common::DateFormat> = None;
                let mut seen_format = false;
                let mut f_from: Option<crate::types::common::DateMath> = None;
                let mut seen_from = false;
                let mut f_gt: Option<crate::types::common::DateMath> = None;
                let mut seen_gt = false;
                let mut f_gte: Option<crate::types::common::DateMath> = None;
                let mut seen_gte = false;
                let mut f_include_lower: Option<bool> = None;
                let mut seen_include_lower = false;
                let mut f_include_upper: Option<bool> = None;
                let mut seen_include_upper = false;
                let mut f_lt: Option<crate::types::common::DateMath> = None;
                let mut seen_lt = false;
                let mut f_lte: Option<crate::types::common::DateMath> = None;
                let mut seen_lte = false;
                let mut f_time_zone: Option<crate::types::common::TimeZone> = None;
                let mut seen_time_zone = false;
                let mut f_to: Option<crate::types::common::DateMath> = None;
                let mut seen_to = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "relation" => {
                            if seen_relation {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            seen_relation = true;
                            f_relation = map.next_value::<Option<crate::types::common_query_dsl::RangeRelation>>()?;
                        }
                        "format" => {
                            if seen_format {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            seen_format = true;
                            f_format =
                                map.next_value::<Option<crate::types::common::DateFormat>>()?;
                        }
                        "from" => {
                            if seen_from {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            seen_from = true;
                            f_from = map.next_value::<Option<crate::types::common::DateMath>>()?;
                        }
                        "gt" => {
                            if seen_gt {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            seen_gt = true;
                            f_gt = map.next_value::<Option<crate::types::common::DateMath>>()?;
                        }
                        "gte" => {
                            if seen_gte {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            seen_gte = true;
                            f_gte = map.next_value::<Option<crate::types::common::DateMath>>()?;
                        }
                        "include_lower" => {
                            if seen_include_lower {
                                return Err(serde::de::Error::duplicate_field("include_lower"));
                            }
                            seen_include_lower = true;
                            f_include_lower = map.next_value::<Option<bool>>()?;
                        }
                        "include_upper" => {
                            if seen_include_upper {
                                return Err(serde::de::Error::duplicate_field("include_upper"));
                            }
                            seen_include_upper = true;
                            f_include_upper = map.next_value::<Option<bool>>()?;
                        }
                        "lt" => {
                            if seen_lt {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            seen_lt = true;
                            f_lt = map.next_value::<Option<crate::types::common::DateMath>>()?;
                        }
                        "lte" => {
                            if seen_lte {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            seen_lte = true;
                            f_lte = map.next_value::<Option<crate::types::common::DateMath>>()?;
                        }
                        "time_zone" => {
                            if seen_time_zone {
                                return Err(serde::de::Error::duplicate_field("time_zone"));
                            }
                            seen_time_zone = true;
                            f_time_zone =
                                map.next_value::<Option<crate::types::common::TimeZone>>()?;
                        }
                        "to" => {
                            if seen_to {
                                return Err(serde::de::Error::duplicate_field("to"));
                            }
                            seen_to = true;
                            f_to = map.next_value::<Option<crate::types::common::DateMath>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DateRangeQuery {
                    name: f_name,
                    boost: f_boost,
                    relation: f_relation,
                    format: f_format,
                    from: f_from,
                    gt: f_gt,
                    gte: f_gte,
                    include_lower: f_include_lower,
                    include_upper: f_include_upper,
                    lt: f_lt,
                    lte: f_lte,
                    time_zone: f_time_zone,
                    to: f_to,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DateRangeQuery {
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
        if let Some(v) = &self.relation {
            map.serialize_entry("relation", v)?;
        }
        if let Some(v) = &self.format {
            map.serialize_entry("format", v)?;
        }
        if let Some(v) = &self.from {
            map.serialize_entry("from", v)?;
        }
        if let Some(v) = &self.gt {
            map.serialize_entry("gt", v)?;
        }
        if let Some(v) = &self.gte {
            map.serialize_entry("gte", v)?;
        }
        if let Some(v) = &self.include_lower {
            map.serialize_entry("include_lower", v)?;
        }
        if let Some(v) = &self.include_upper {
            map.serialize_entry("include_upper", v)?;
        }
        if let Some(v) = &self.lt {
            map.serialize_entry("lt", v)?;
        }
        if let Some(v) = &self.lte {
            map.serialize_entry("lte", v)?;
        }
        if let Some(v) = &self.time_zone {
            map.serialize_entry("time_zone", v)?;
        }
        if let Some(v) = &self.to {
            map.serialize_entry("to", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_multi_value_mode: Option<crate::types::common_query_dsl::MultiValueMode> =
                    None;
                let mut seen_multi_value_mode = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_query_dsl::DecayPlacement,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "multi_value_mode" => {
                            if seen_multi_value_mode {
                                return Err(serde::de::Error::duplicate_field("multi_value_mode"));
                            }
                            seen_multi_value_mode = true;
                            f_multi_value_mode = map.next_value::<Option<crate::types::common_query_dsl::MultiValueMode>>()?;
                        }
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

#[derive(Debug, Clone)]
pub struct DisMaxQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "One or more query clauses. Returned documents must match one or more of these queries. If a document matches multiple queries, OpenSearch uses the highest relevance score."]
    pub queries: Vec<crate::types::common_query_dsl::QueryContainer>,
    #[doc = "Floating point number between 0 and 1.0 used to increase the relevance scores of documents matching multiple query clauses."]
    pub tie_breaker: Option<f64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DisMaxQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DisMaxQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DisMaxQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_queries: Option<Vec<crate::types::common_query_dsl::QueryContainer>> =
                    None;
                let mut f_tie_breaker: Option<f64> = None;
                let mut seen_tie_breaker = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "queries" => {
                            if f_queries.is_some() {
                                return Err(serde::de::Error::duplicate_field("queries"));
                            }
                            f_queries = Some(map.next_value::<Vec<crate::types::common_query_dsl::QueryContainer>>()?);
                        }
                        "tie_breaker" => {
                            if seen_tie_breaker {
                                return Err(serde::de::Error::duplicate_field("tie_breaker"));
                            }
                            seen_tie_breaker = true;
                            f_tie_breaker = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DisMaxQuery {
                    name: f_name,
                    boost: f_boost,
                    queries: f_queries.ok_or_else(|| serde::de::Error::missing_field("queries"))?,
                    tie_breaker: f_tie_breaker,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DisMaxQuery {
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
        map.serialize_entry("queries", &self.queries)?;
        if let Some(v) = &self.tie_breaker {
            map.serialize_entry("tie_breaker", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct DistanceFeatureQueryObject {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub field: crate::types::common::Field,
    pub origin: crate::types::common::GeoLocation,
    pub pivot: crate::types::common::Distance,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DistanceFeatureQueryObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DistanceFeatureQueryObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DistanceFeatureQueryObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_origin: Option<crate::types::common::GeoLocation> = None;
                let mut f_pivot: Option<crate::types::common::Distance> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "origin" => {
                            if f_origin.is_some() {
                                return Err(serde::de::Error::duplicate_field("origin"));
                            }
                            f_origin = Some(map.next_value::<crate::types::common::GeoLocation>()?);
                        }
                        "pivot" => {
                            if f_pivot.is_some() {
                                return Err(serde::de::Error::duplicate_field("pivot"));
                            }
                            f_pivot = Some(map.next_value::<crate::types::common::Distance>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DistanceFeatureQueryObject {
                    name: f_name,
                    boost: f_boost,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    origin: f_origin.ok_or_else(|| serde::de::Error::missing_field("origin"))?,
                    pivot: f_pivot.ok_or_else(|| serde::de::Error::missing_field("pivot"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DistanceFeatureQueryObject {
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
        map.serialize_entry("field", &self.field)?;
        map.serialize_entry("origin", &self.origin)?;
        map.serialize_entry("pivot", &self.pivot)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct DistanceFeatureQueryObjectObject {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub field: crate::types::common::Field,
    pub origin: crate::types::common::DateMath,
    pub pivot: crate::types::common::Duration,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DistanceFeatureQueryObjectObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DistanceFeatureQueryObjectObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DistanceFeatureQueryObjectObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_origin: Option<crate::types::common::DateMath> = None;
                let mut f_pivot: Option<crate::types::common::Duration> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "origin" => {
                            if f_origin.is_some() {
                                return Err(serde::de::Error::duplicate_field("origin"));
                            }
                            f_origin = Some(map.next_value::<crate::types::common::DateMath>()?);
                        }
                        "pivot" => {
                            if f_pivot.is_some() {
                                return Err(serde::de::Error::duplicate_field("pivot"));
                            }
                            f_pivot = Some(map.next_value::<crate::types::common::Duration>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DistanceFeatureQueryObjectObject {
                    name: f_name,
                    boost: f_boost,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    origin: f_origin.ok_or_else(|| serde::de::Error::missing_field("origin"))?,
                    pivot: f_pivot.ok_or_else(|| serde::de::Error::missing_field("pivot"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DistanceFeatureQueryObjectObject {
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
        map.serialize_entry("field", &self.field)?;
        map.serialize_entry("origin", &self.origin)?;
        map.serialize_entry("pivot", &self.pivot)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ExistsQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub field: crate::types::common::Field,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ExistsQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ExistsQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ExistsQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ExistsQuery {
                    name: f_name,
                    boost: f_boost,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ExistsQuery {
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
        map.serialize_entry("field", &self.field)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'"') => serde_json::from_str(text).map(FieldAndFormat::Field),
            Some(b'{') => serde_json::from_str(text).map(FieldAndFormat::Object),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a FieldAndFormat value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct FieldAndFormatObject {
    pub field: crate::types::common::Field,
    #[doc = "Format in which the values are returned."]
    pub format: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FieldAndFormatObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FieldAndFormatObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FieldAndFormatObject object")
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
                let mut f_format: Option<String> = None;
                let mut seen_format = false;
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
                        "format" => {
                            if seen_format {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            seen_format = true;
                            f_format = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FieldAndFormatObject {
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    format: f_format,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FieldAndFormatObject {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.format {
            map.serialize_entry("format", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct FieldLookup {
    pub id: crate::types::common::Id,
    pub index: Option<crate::types::common::IndexName>,
    pub path: Option<crate::types::common::Field>,
    pub routing: Option<crate::types::common::Routing>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FieldLookup {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FieldLookup;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FieldLookup object")
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
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut seen_index = false;
                let mut f_path: Option<crate::types::common::Field> = None;
                let mut seen_path = false;
                let mut f_routing: Option<crate::types::common::Routing> = None;
                let mut seen_routing = false;
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
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "path" => {
                            if seen_path {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            seen_path = true;
                            f_path = map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        "routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("routing"));
                            }
                            seen_routing = true;
                            f_routing =
                                map.next_value::<Option<crate::types::common::Routing>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FieldLookup {
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("id"))?,
                    index: f_index,
                    path: f_path,
                    routing: f_routing,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FieldLookup {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("id", &self.id)?;
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.path {
            map.serialize_entry("path", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("routing", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldValueFactorModifier {
    #[doc = "`ln`"]
    Ln,
    #[doc = "`ln1p`"]
    Ln1P,
    #[doc = "`ln2p`"]
    Ln2P,
    #[doc = "`log`"]
    Log,
    #[doc = "`log1p`"]
    Log1P,
    #[doc = "`log2p`"]
    Log2P,
    #[doc = "`none`"]
    None,
    #[doc = "`reciprocal`"]
    Reciprocal,
    #[doc = "`sqrt`"]
    Sqrt,
    #[doc = "`square`"]
    Square,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl FieldValueFactorModifier {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            FieldValueFactorModifier::Ln => "ln",
            FieldValueFactorModifier::Ln1P => "ln1p",
            FieldValueFactorModifier::Ln2P => "ln2p",
            FieldValueFactorModifier::Log => "log",
            FieldValueFactorModifier::Log1P => "log1p",
            FieldValueFactorModifier::Log2P => "log2p",
            FieldValueFactorModifier::None => "none",
            FieldValueFactorModifier::Reciprocal => "reciprocal",
            FieldValueFactorModifier::Sqrt => "sqrt",
            FieldValueFactorModifier::Square => "square",
            FieldValueFactorModifier::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for FieldValueFactorModifier {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FieldValueFactorModifier {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ln" => FieldValueFactorModifier::Ln,
            "ln1p" => FieldValueFactorModifier::Ln1P,
            "ln2p" => FieldValueFactorModifier::Ln2P,
            "log" => FieldValueFactorModifier::Log,
            "log1p" => FieldValueFactorModifier::Log1P,
            "log2p" => FieldValueFactorModifier::Log2P,
            "none" => FieldValueFactorModifier::None,
            "reciprocal" => FieldValueFactorModifier::Reciprocal,
            "sqrt" => FieldValueFactorModifier::Sqrt,
            "square" => FieldValueFactorModifier::Square,
            _ => FieldValueFactorModifier::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct FieldValueFactorScoreFunction {
    #[doc = "Optional factor to multiply the field value with."]
    pub factor: Option<f64>,
    pub field: crate::types::common::Field,
    #[doc = "Value used if the document doesn't have that field. The modifier and factor are still applied to it as though it were read from the document."]
    pub missing: Option<f64>,
    pub modifier: Option<crate::types::common_query_dsl::FieldValueFactorModifier>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FieldValueFactorScoreFunction {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FieldValueFactorScoreFunction;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FieldValueFactorScoreFunction object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_factor: Option<f64> = None;
                let mut seen_factor = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_missing: Option<f64> = None;
                let mut seen_missing = false;
                let mut f_modifier: Option<
                    crate::types::common_query_dsl::FieldValueFactorModifier,
                > = None;
                let mut seen_modifier = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "factor" => {
                            if seen_factor {
                                return Err(serde::de::Error::duplicate_field("factor"));
                            }
                            seen_factor = true;
                            f_factor = map.next_value::<Option<f64>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "missing" => {
                            if seen_missing {
                                return Err(serde::de::Error::duplicate_field("missing"));
                            }
                            seen_missing = true;
                            f_missing = map.next_value::<Option<f64>>()?;
                        }
                        "modifier" => {
                            if seen_modifier {
                                return Err(serde::de::Error::duplicate_field("modifier"));
                            }
                            seen_modifier = true;
                            f_modifier =
                                map.next_value::<Option<
                                    crate::types::common_query_dsl::FieldValueFactorModifier,
                                >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FieldValueFactorScoreFunction {
                    factor: f_factor,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    missing: f_missing,
                    modifier: f_modifier,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FieldValueFactorScoreFunction {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.factor {
            map.serialize_entry("factor", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.missing {
            map.serialize_entry("missing", v)?;
        }
        if let Some(v) = &self.modifier {
            map.serialize_entry("modifier", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FunctionBoostMode {
    #[doc = "`avg`"]
    Avg,
    #[doc = "`max`"]
    Max,
    #[doc = "`min`"]
    Min,
    #[doc = "`multiply`"]
    Multiply,
    #[doc = "`replace`"]
    Replace,
    #[doc = "`sum`"]
    Sum,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl FunctionBoostMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            FunctionBoostMode::Avg => "avg",
            FunctionBoostMode::Max => "max",
            FunctionBoostMode::Min => "min",
            FunctionBoostMode::Multiply => "multiply",
            FunctionBoostMode::Replace => "replace",
            FunctionBoostMode::Sum => "sum",
            FunctionBoostMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for FunctionBoostMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FunctionBoostMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "avg" => FunctionBoostMode::Avg,
            "max" => FunctionBoostMode::Max,
            "min" => FunctionBoostMode::Min,
            "multiply" => FunctionBoostMode::Multiply,
            "replace" => FunctionBoostMode::Replace,
            "sum" => FunctionBoostMode::Sum,
            _ => FunctionBoostMode::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct FunctionScoreContainer {
    pub filter: Option<crate::types::common_query_dsl::QueryContainer>,
    pub weight: Option<f64>,
    pub exp: Option<crate::types::common_query_dsl::DecayFunction>,
    pub field_value_factor: Option<crate::types::common_query_dsl::FieldValueFactorScoreFunction>,
    pub gauss: Option<crate::types::common_query_dsl::DecayFunction>,
    pub linear: Option<crate::types::common_query_dsl::DecayFunction>,
    pub random_score: Option<crate::types::common_query_dsl::RandomScoreFunction>,
    pub script_score: Option<crate::types::common_query_dsl::ScriptScoreFunction>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FunctionScoreContainer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FunctionScoreContainer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FunctionScoreContainer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_filter: Option<crate::types::common_query_dsl::QueryContainer> = None;
                let mut seen_filter = false;
                let mut f_weight: Option<f64> = None;
                let mut seen_weight = false;
                let mut f_exp: Option<crate::types::common_query_dsl::DecayFunction> = None;
                let mut seen_exp = false;
                let mut f_field_value_factor: Option<
                    crate::types::common_query_dsl::FieldValueFactorScoreFunction,
                > = None;
                let mut seen_field_value_factor = false;
                let mut f_gauss: Option<crate::types::common_query_dsl::DecayFunction> = None;
                let mut seen_gauss = false;
                let mut f_linear: Option<crate::types::common_query_dsl::DecayFunction> = None;
                let mut seen_linear = false;
                let mut f_random_score: Option<
                    crate::types::common_query_dsl::RandomScoreFunction,
                > = None;
                let mut seen_random_score = false;
                let mut f_script_score: Option<
                    crate::types::common_query_dsl::ScriptScoreFunction,
                > = None;
                let mut seen_script_score = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "filter" => {
                            if seen_filter {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            seen_filter = true;
                            f_filter = map.next_value::<Option<crate::types::common_query_dsl::QueryContainer>>()?;
                        }
                        "weight" => {
                            if seen_weight {
                                return Err(serde::de::Error::duplicate_field("weight"));
                            }
                            seen_weight = true;
                            f_weight = map.next_value::<Option<f64>>()?;
                        }
                        "exp" => {
                            if seen_exp {
                                return Err(serde::de::Error::duplicate_field("exp"));
                            }
                            seen_exp = true;
                            f_exp = map.next_value::<Option<crate::types::common_query_dsl::DecayFunction>>()?;
                        }
                        "field_value_factor" => {
                            if seen_field_value_factor {
                                return Err(serde::de::Error::duplicate_field(
                                    "field_value_factor",
                                ));
                            }
                            seen_field_value_factor = true;
                            f_field_value_factor = map.next_value::<Option<
                                crate::types::common_query_dsl::FieldValueFactorScoreFunction,
                            >>()?;
                        }
                        "gauss" => {
                            if seen_gauss {
                                return Err(serde::de::Error::duplicate_field("gauss"));
                            }
                            seen_gauss = true;
                            f_gauss = map.next_value::<Option<crate::types::common_query_dsl::DecayFunction>>()?;
                        }
                        "linear" => {
                            if seen_linear {
                                return Err(serde::de::Error::duplicate_field("linear"));
                            }
                            seen_linear = true;
                            f_linear = map.next_value::<Option<crate::types::common_query_dsl::DecayFunction>>()?;
                        }
                        "random_score" => {
                            if seen_random_score {
                                return Err(serde::de::Error::duplicate_field("random_score"));
                            }
                            seen_random_score = true;
                            f_random_score = map.next_value::<Option<crate::types::common_query_dsl::RandomScoreFunction>>()?;
                        }
                        "script_score" => {
                            if seen_script_score {
                                return Err(serde::de::Error::duplicate_field("script_score"));
                            }
                            seen_script_score = true;
                            f_script_score = map.next_value::<Option<crate::types::common_query_dsl::ScriptScoreFunction>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FunctionScoreContainer {
                    filter: f_filter,
                    weight: f_weight,
                    exp: f_exp,
                    field_value_factor: f_field_value_factor,
                    gauss: f_gauss,
                    linear: f_linear,
                    random_score: f_random_score,
                    script_score: f_script_score,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FunctionScoreContainer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.filter {
            map.serialize_entry("filter", v)?;
        }
        if let Some(v) = &self.weight {
            map.serialize_entry("weight", v)?;
        }
        if let Some(v) = &self.exp {
            map.serialize_entry("exp", v)?;
        }
        if let Some(v) = &self.field_value_factor {
            map.serialize_entry("field_value_factor", v)?;
        }
        if let Some(v) = &self.gauss {
            map.serialize_entry("gauss", v)?;
        }
        if let Some(v) = &self.linear {
            map.serialize_entry("linear", v)?;
        }
        if let Some(v) = &self.random_score {
            map.serialize_entry("random_score", v)?;
        }
        if let Some(v) = &self.script_score {
            map.serialize_entry("script_score", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FunctionScoreMode {
    #[doc = "`avg`"]
    Avg,
    #[doc = "`first`"]
    First,
    #[doc = "`max`"]
    Max,
    #[doc = "`min`"]
    Min,
    #[doc = "`multiply`"]
    Multiply,
    #[doc = "`sum`"]
    Sum,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl FunctionScoreMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            FunctionScoreMode::Avg => "avg",
            FunctionScoreMode::First => "first",
            FunctionScoreMode::Max => "max",
            FunctionScoreMode::Min => "min",
            FunctionScoreMode::Multiply => "multiply",
            FunctionScoreMode::Sum => "sum",
            FunctionScoreMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for FunctionScoreMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FunctionScoreMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "avg" => FunctionScoreMode::Avg,
            "first" => FunctionScoreMode::First,
            "max" => FunctionScoreMode::Max,
            "min" => FunctionScoreMode::Min,
            "multiply" => FunctionScoreMode::Multiply,
            "sum" => FunctionScoreMode::Sum,
            _ => FunctionScoreMode::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct FunctionScoreQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub boost_mode: Option<crate::types::common_query_dsl::FunctionBoostMode>,
    #[doc = "One or more functions that compute a new score for each document returned by the query."]
    pub functions: Option<Vec<crate::types::common_query_dsl::FunctionScoreContainer>>,
    #[doc = "Restricts the new score to not exceed the provided limit."]
    pub max_boost: Option<f64>,
    #[doc = "Excludes documents that do not meet the provided score threshold."]
    pub min_score: Option<f64>,
    pub query: Option<Box<crate::types::common_query_dsl::QueryContainer>>,
    pub score_mode: Option<crate::types::common_query_dsl::FunctionScoreMode>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FunctionScoreQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FunctionScoreQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FunctionScoreQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_boost_mode: Option<crate::types::common_query_dsl::FunctionBoostMode> =
                    None;
                let mut seen_boost_mode = false;
                let mut f_functions: Option<
                    Vec<crate::types::common_query_dsl::FunctionScoreContainer>,
                > = None;
                let mut seen_functions = false;
                let mut f_max_boost: Option<f64> = None;
                let mut seen_max_boost = false;
                let mut f_min_score: Option<f64> = None;
                let mut seen_min_score = false;
                let mut f_query: Option<Box<crate::types::common_query_dsl::QueryContainer>> = None;
                let mut seen_query = false;
                let mut f_score_mode: Option<crate::types::common_query_dsl::FunctionScoreMode> =
                    None;
                let mut seen_score_mode = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "boost_mode" => {
                            if seen_boost_mode {
                                return Err(serde::de::Error::duplicate_field("boost_mode"));
                            }
                            seen_boost_mode = true;
                            f_boost_mode = map.next_value::<Option<crate::types::common_query_dsl::FunctionBoostMode>>()?;
                        }
                        "functions" => {
                            if seen_functions {
                                return Err(serde::de::Error::duplicate_field("functions"));
                            }
                            seen_functions = true;
                            f_functions = map.next_value::<Option<
                                Vec<crate::types::common_query_dsl::FunctionScoreContainer>,
                            >>()?;
                        }
                        "max_boost" => {
                            if seen_max_boost {
                                return Err(serde::de::Error::duplicate_field("max_boost"));
                            }
                            seen_max_boost = true;
                            f_max_boost = map.next_value::<Option<f64>>()?;
                        }
                        "min_score" => {
                            if seen_min_score {
                                return Err(serde::de::Error::duplicate_field("min_score"));
                            }
                            seen_min_score = true;
                            f_min_score = map.next_value::<Option<f64>>()?;
                        }
                        "query" => {
                            if seen_query {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            seen_query = true;
                            f_query = map.next_value::<Option<Box<crate::types::common_query_dsl::QueryContainer>>>()?;
                        }
                        "score_mode" => {
                            if seen_score_mode {
                                return Err(serde::de::Error::duplicate_field("score_mode"));
                            }
                            seen_score_mode = true;
                            f_score_mode = map.next_value::<Option<crate::types::common_query_dsl::FunctionScoreMode>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FunctionScoreQuery {
                    name: f_name,
                    boost: f_boost,
                    boost_mode: f_boost_mode,
                    functions: f_functions,
                    max_boost: f_max_boost,
                    min_score: f_min_score,
                    query: f_query,
                    score_mode: f_score_mode,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FunctionScoreQuery {
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
        if let Some(v) = &self.boost_mode {
            map.serialize_entry("boost_mode", v)?;
        }
        if let Some(v) = &self.functions {
            map.serialize_entry("functions", v)?;
        }
        if let Some(v) = &self.max_boost {
            map.serialize_entry("max_boost", v)?;
        }
        if let Some(v) = &self.min_score {
            map.serialize_entry("min_score", v)?;
        }
        if let Some(v) = &self.query {
            map.serialize_entry("query", v)?;
        }
        if let Some(v) = &self.score_mode {
            map.serialize_entry("score_mode", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct FuzzyQueryObject {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub fuzziness: Option<crate::types::common::Fuzziness>,
    #[doc = "Maximum number of variations created."]
    pub max_expansions: Option<i32>,
    #[doc = "Number of beginning characters left unchanged when creating expansions."]
    pub prefix_length: Option<i32>,
    pub rewrite: Option<crate::types::common::MultiTermQueryRewrite>,
    #[doc = "Indicates whether edits include transpositions of two adjacent characters (for example, `ab` to `ba`)."]
    pub transpositions: Option<bool>,
    #[doc = "Term you wish to find in the provided field."]
    pub value: crate::types::common::FieldValue,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FuzzyQueryObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FuzzyQueryObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FuzzyQueryObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_fuzziness: Option<crate::types::common::Fuzziness> = None;
                let mut seen_fuzziness = false;
                let mut f_max_expansions: Option<i32> = None;
                let mut seen_max_expansions = false;
                let mut f_prefix_length: Option<i32> = None;
                let mut seen_prefix_length = false;
                let mut f_rewrite: Option<crate::types::common::MultiTermQueryRewrite> = None;
                let mut seen_rewrite = false;
                let mut f_transpositions: Option<bool> = None;
                let mut seen_transpositions = false;
                let mut f_value: Option<crate::types::common::FieldValue> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "fuzziness" => {
                            if seen_fuzziness {
                                return Err(serde::de::Error::duplicate_field("fuzziness"));
                            }
                            seen_fuzziness = true;
                            f_fuzziness =
                                map.next_value::<Option<crate::types::common::Fuzziness>>()?;
                        }
                        "max_expansions" => {
                            if seen_max_expansions {
                                return Err(serde::de::Error::duplicate_field("max_expansions"));
                            }
                            seen_max_expansions = true;
                            f_max_expansions = map.next_value::<Option<i32>>()?;
                        }
                        "prefix_length" => {
                            if seen_prefix_length {
                                return Err(serde::de::Error::duplicate_field("prefix_length"));
                            }
                            seen_prefix_length = true;
                            f_prefix_length = map.next_value::<Option<i32>>()?;
                        }
                        "rewrite" => {
                            if seen_rewrite {
                                return Err(serde::de::Error::duplicate_field("rewrite"));
                            }
                            seen_rewrite = true;
                            f_rewrite = map
                                .next_value::<Option<crate::types::common::MultiTermQueryRewrite>>(
                                )?;
                        }
                        "transpositions" => {
                            if seen_transpositions {
                                return Err(serde::de::Error::duplicate_field("transpositions"));
                            }
                            seen_transpositions = true;
                            f_transpositions = map.next_value::<Option<bool>>()?;
                        }
                        "value" => {
                            if f_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            f_value = Some(map.next_value::<crate::types::common::FieldValue>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FuzzyQueryObject {
                    name: f_name,
                    boost: f_boost,
                    fuzziness: f_fuzziness,
                    max_expansions: f_max_expansions,
                    prefix_length: f_prefix_length,
                    rewrite: f_rewrite,
                    transpositions: f_transpositions,
                    value: f_value.ok_or_else(|| serde::de::Error::missing_field("value"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FuzzyQueryObject {
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
        if let Some(v) = &self.fuzziness {
            map.serialize_entry("fuzziness", v)?;
        }
        if let Some(v) = &self.max_expansions {
            map.serialize_entry("max_expansions", v)?;
        }
        if let Some(v) = &self.prefix_length {
            map.serialize_entry("prefix_length", v)?;
        }
        if let Some(v) = &self.rewrite {
            map.serialize_entry("rewrite", v)?;
        }
        if let Some(v) = &self.transpositions {
            map.serialize_entry("transpositions", v)?;
        }
        map.serialize_entry("value", &self.value)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped> =
                    None;
                let mut seen_ignore_unmapped = false;
                let mut f_type: Option<crate::types::common_query_dsl::GeoExecution> = None;
                let mut seen_type = false;
                let mut f_validation_method: Option<
                    crate::types::common_query_dsl::GeoValidationMethod,
                > = None;
                let mut seen_validation_method = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common::GeoBounds,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "ignore_unmapped" => {
                            if seen_ignore_unmapped {
                                return Err(serde::de::Error::duplicate_field("ignore_unmapped"));
                            }
                            seen_ignore_unmapped = true;
                            f_ignore_unmapped = map.next_value::<Option<crate::types::common_query_dsl::IgnoreUnmapped>>()?;
                        }
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type = map
                                .next_value::<Option<crate::types::common_query_dsl::GeoExecution>>(
                                )?;
                        }
                        "validation_method" => {
                            if seen_validation_method {
                                return Err(serde::de::Error::duplicate_field("validation_method"));
                            }
                            seen_validation_method = true;
                            f_validation_method = map.next_value::<Option<crate::types::common_query_dsl::GeoValidationMethod>>()?;
                        }
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

#[derive(Debug, Clone)]
pub struct GeoDecayPlacement {
    pub decay: Option<f64>,
    pub offset: Option<crate::types::common::Distance>,
    pub origin: crate::types::common::GeoLocation,
    pub scale: crate::types::common::Distance,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GeoDecayPlacement {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GeoDecayPlacement;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoDecayPlacement object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_decay: Option<f64> = None;
                let mut seen_decay = false;
                let mut f_offset: Option<crate::types::common::Distance> = None;
                let mut seen_offset = false;
                let mut f_origin: Option<crate::types::common::GeoLocation> = None;
                let mut f_scale: Option<crate::types::common::Distance> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "decay" => {
                            if seen_decay {
                                return Err(serde::de::Error::duplicate_field("decay"));
                            }
                            seen_decay = true;
                            f_decay = map.next_value::<Option<f64>>()?;
                        }
                        "offset" => {
                            if seen_offset {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            seen_offset = true;
                            f_offset =
                                map.next_value::<Option<crate::types::common::Distance>>()?;
                        }
                        "origin" => {
                            if f_origin.is_some() {
                                return Err(serde::de::Error::duplicate_field("origin"));
                            }
                            f_origin = Some(map.next_value::<crate::types::common::GeoLocation>()?);
                        }
                        "scale" => {
                            if f_scale.is_some() {
                                return Err(serde::de::Error::duplicate_field("scale"));
                            }
                            f_scale = Some(map.next_value::<crate::types::common::Distance>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GeoDecayPlacement {
                    decay: f_decay,
                    offset: f_offset,
                    origin: f_origin.ok_or_else(|| serde::de::Error::missing_field("origin"))?,
                    scale: f_scale.ok_or_else(|| serde::de::Error::missing_field("scale"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GeoDecayPlacement {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.decay {
            map.serialize_entry("decay", v)?;
        }
        if let Some(v) = &self.offset {
            map.serialize_entry("offset", v)?;
        }
        map.serialize_entry("origin", &self.origin)?;
        map.serialize_entry("scale", &self.scale)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_distance: Option<crate::types::common::Distance> = None;
                let mut f_distance_type: Option<crate::types::common::GeoDistanceType> = None;
                let mut seen_distance_type = false;
                let mut f_ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped> =
                    None;
                let mut seen_ignore_unmapped = false;
                let mut f_unit: Option<crate::types::common::DistanceUnit> = None;
                let mut seen_unit = false;
                let mut f_validation_method: Option<
                    crate::types::common_query_dsl::GeoValidationMethod,
                > = None;
                let mut seen_validation_method = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common::GeoLocation,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "distance" => {
                            if f_distance.is_some() {
                                return Err(serde::de::Error::duplicate_field("distance"));
                            }
                            f_distance = Some(map.next_value::<crate::types::common::Distance>()?);
                        }
                        "distance_type" => {
                            if seen_distance_type {
                                return Err(serde::de::Error::duplicate_field("distance_type"));
                            }
                            seen_distance_type = true;
                            f_distance_type =
                                map.next_value::<Option<crate::types::common::GeoDistanceType>>()?;
                        }
                        "ignore_unmapped" => {
                            if seen_ignore_unmapped {
                                return Err(serde::de::Error::duplicate_field("ignore_unmapped"));
                            }
                            seen_ignore_unmapped = true;
                            f_ignore_unmapped = map.next_value::<Option<crate::types::common_query_dsl::IgnoreUnmapped>>()?;
                        }
                        "unit" => {
                            if seen_unit {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            seen_unit = true;
                            f_unit =
                                map.next_value::<Option<crate::types::common::DistanceUnit>>()?;
                        }
                        "validation_method" => {
                            if seen_validation_method {
                                return Err(serde::de::Error::duplicate_field("validation_method"));
                            }
                            seen_validation_method = true;
                            f_validation_method = map.next_value::<Option<crate::types::common_query_dsl::GeoValidationMethod>>()?;
                        }
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GeoExecution {
    #[doc = "`indexed`"]
    Indexed,
    #[doc = "`memory`"]
    Memory,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl GeoExecution {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            GeoExecution::Indexed => "indexed",
            GeoExecution::Memory => "memory",
            GeoExecution::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for GeoExecution {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GeoExecution {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "indexed" => GeoExecution::Indexed,
            "memory" => GeoExecution::Memory,
            _ => GeoExecution::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct GeoPolygonPoints {
    pub points: Vec<crate::types::common::GeoLocation>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GeoPolygonPoints {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GeoPolygonPoints;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoPolygonPoints object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_points: Option<Vec<crate::types::common::GeoLocation>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "points" => {
                            if f_points.is_some() {
                                return Err(serde::de::Error::duplicate_field("points"));
                            }
                            f_points =
                                Some(map.next_value::<Vec<crate::types::common::GeoLocation>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GeoPolygonPoints {
                    points: f_points.ok_or_else(|| serde::de::Error::missing_field("points"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GeoPolygonPoints {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("points", &self.points)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped> =
                    None;
                let mut seen_ignore_unmapped = false;
                let mut f_validation_method: Option<
                    crate::types::common_query_dsl::GeoValidationMethod,
                > = None;
                let mut seen_validation_method = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_query_dsl::GeoPolygonPoints,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "ignore_unmapped" => {
                            if seen_ignore_unmapped {
                                return Err(serde::de::Error::duplicate_field("ignore_unmapped"));
                            }
                            seen_ignore_unmapped = true;
                            f_ignore_unmapped = map.next_value::<Option<crate::types::common_query_dsl::IgnoreUnmapped>>()?;
                        }
                        "validation_method" => {
                            if seen_validation_method {
                                return Err(serde::de::Error::duplicate_field("validation_method"));
                            }
                            seen_validation_method = true;
                            f_validation_method = map.next_value::<Option<crate::types::common_query_dsl::GeoValidationMethod>>()?;
                        }
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

#[derive(Debug, Clone)]
pub struct GeoShape {
    pub coordinates: Option<Vec<Box<serde_json::value::RawValue>>>,
    pub r#type: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GeoShape {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GeoShape;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoShape object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_coordinates: Option<Vec<Box<serde_json::value::RawValue>>> = None;
                let mut seen_coordinates = false;
                let mut f_type: Option<String> = None;
                let mut seen_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "coordinates" => {
                            if seen_coordinates {
                                return Err(serde::de::Error::duplicate_field("coordinates"));
                            }
                            seen_coordinates = true;
                            f_coordinates =
                                map.next_value::<Option<Vec<Box<serde_json::value::RawValue>>>>()?;
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
                Ok(GeoShape {
                    coordinates: f_coordinates,
                    r#type: f_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GeoShape {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.coordinates {
            map.serialize_entry("coordinates", v)?;
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
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped> =
                    None;
                let mut seen_ignore_unmapped = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_query_dsl::GeoShapeQueryField,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "ignore_unmapped" => {
                            if seen_ignore_unmapped {
                                return Err(serde::de::Error::duplicate_field("ignore_unmapped"));
                            }
                            seen_ignore_unmapped = true;
                            f_ignore_unmapped = map.next_value::<Option<crate::types::common_query_dsl::IgnoreUnmapped>>()?;
                        }
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

#[derive(Debug, Clone)]
pub struct GeoShapeQueryField {
    pub indexed_shape: Option<crate::types::common_query_dsl::FieldLookup>,
    pub relation: Option<crate::types::common::GeoShapeRelation>,
    pub shape: crate::types::common_query_dsl::GeoShape,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GeoShapeQueryField {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GeoShapeQueryField;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoShapeQueryField object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_indexed_shape: Option<crate::types::common_query_dsl::FieldLookup> = None;
                let mut seen_indexed_shape = false;
                let mut f_relation: Option<crate::types::common::GeoShapeRelation> = None;
                let mut seen_relation = false;
                let mut f_shape: Option<crate::types::common_query_dsl::GeoShape> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "indexed_shape" => {
                            if seen_indexed_shape {
                                return Err(serde::de::Error::duplicate_field("indexed_shape"));
                            }
                            seen_indexed_shape = true;
                            f_indexed_shape = map
                                .next_value::<Option<crate::types::common_query_dsl::FieldLookup>>(
                                )?;
                        }
                        "relation" => {
                            if seen_relation {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            seen_relation = true;
                            f_relation =
                                map.next_value::<Option<crate::types::common::GeoShapeRelation>>()?;
                        }
                        "shape" => {
                            if f_shape.is_some() {
                                return Err(serde::de::Error::duplicate_field("shape"));
                            }
                            f_shape =
                                Some(map.next_value::<crate::types::common_query_dsl::GeoShape>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GeoShapeQueryField {
                    indexed_shape: f_indexed_shape,
                    relation: f_relation,
                    shape: f_shape.ok_or_else(|| serde::de::Error::missing_field("shape"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GeoShapeQueryField {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.indexed_shape {
            map.serialize_entry("indexed_shape", v)?;
        }
        if let Some(v) = &self.relation {
            map.serialize_entry("relation", v)?;
        }
        map.serialize_entry("shape", &self.shape)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GeoValidationMethod {
    #[doc = "`coerce`"]
    Coerce,
    #[doc = "`ignore_malformed`"]
    IgnoreMalformed,
    #[doc = "`strict`"]
    Strict,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl GeoValidationMethod {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            GeoValidationMethod::Coerce => "coerce",
            GeoValidationMethod::IgnoreMalformed => "ignore_malformed",
            GeoValidationMethod::Strict => "strict",
            GeoValidationMethod::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for GeoValidationMethod {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GeoValidationMethod {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "coerce" => GeoValidationMethod::Coerce,
            "ignore_malformed" => GeoValidationMethod::IgnoreMalformed,
            "strict" => GeoValidationMethod::Strict,
            _ => GeoValidationMethod::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct HasChildQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "Indicates whether to ignore an unmapped `type` and not return any documents instead of an error."]
    pub ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped>,
    pub inner_hits: Option<Box<crate::types::search::InnerHits>>,
    #[doc = "Maximum number of child documents that match the query allowed for a returned parent document. If the parent document exceeds this limit, it is excluded from the search results."]
    pub max_children: Option<i32>,
    #[doc = "Minimum number of child documents that match the query required to match the query for a returned parent document. If the parent document does not meet this limit, it is excluded from the search results."]
    pub min_children: Option<i32>,
    pub query: Box<crate::types::common_query_dsl::QueryContainer>,
    pub score_mode: Option<crate::types::common_query_dsl::ChildScoreMode>,
    pub r#type: crate::types::common::RelationName,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HasChildQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HasChildQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HasChildQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped> =
                    None;
                let mut seen_ignore_unmapped = false;
                let mut f_inner_hits: Option<Box<crate::types::search::InnerHits>> = None;
                let mut seen_inner_hits = false;
                let mut f_max_children: Option<i32> = None;
                let mut seen_max_children = false;
                let mut f_min_children: Option<i32> = None;
                let mut seen_min_children = false;
                let mut f_query: Option<Box<crate::types::common_query_dsl::QueryContainer>> = None;
                let mut f_score_mode: Option<crate::types::common_query_dsl::ChildScoreMode> = None;
                let mut seen_score_mode = false;
                let mut f_type: Option<crate::types::common::RelationName> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "ignore_unmapped" => {
                            if seen_ignore_unmapped {
                                return Err(serde::de::Error::duplicate_field("ignore_unmapped"));
                            }
                            seen_ignore_unmapped = true;
                            f_ignore_unmapped = map.next_value::<Option<crate::types::common_query_dsl::IgnoreUnmapped>>()?;
                        }
                        "inner_hits" => {
                            if seen_inner_hits {
                                return Err(serde::de::Error::duplicate_field("inner_hits"));
                            }
                            seen_inner_hits = true;
                            f_inner_hits =
                                map.next_value::<Option<Box<crate::types::search::InnerHits>>>()?;
                        }
                        "max_children" => {
                            if seen_max_children {
                                return Err(serde::de::Error::duplicate_field("max_children"));
                            }
                            seen_max_children = true;
                            f_max_children = map.next_value::<Option<i32>>()?;
                        }
                        "min_children" => {
                            if seen_min_children {
                                return Err(serde::de::Error::duplicate_field("min_children"));
                            }
                            seen_min_children = true;
                            f_min_children = map.next_value::<Option<i32>>()?;
                        }
                        "query" => {
                            if f_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            f_query = Some(map.next_value::<Box<crate::types::common_query_dsl::QueryContainer>>()?);
                        }
                        "score_mode" => {
                            if seen_score_mode {
                                return Err(serde::de::Error::duplicate_field("score_mode"));
                            }
                            seen_score_mode = true;
                            f_score_mode = map.next_value::<Option<crate::types::common_query_dsl::ChildScoreMode>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common::RelationName>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HasChildQuery {
                    name: f_name,
                    boost: f_boost,
                    ignore_unmapped: f_ignore_unmapped,
                    inner_hits: f_inner_hits,
                    max_children: f_max_children,
                    min_children: f_min_children,
                    query: f_query.ok_or_else(|| serde::de::Error::missing_field("query"))?,
                    score_mode: f_score_mode,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HasChildQuery {
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
        if let Some(v) = &self.inner_hits {
            map.serialize_entry("inner_hits", v)?;
        }
        if let Some(v) = &self.max_children {
            map.serialize_entry("max_children", v)?;
        }
        if let Some(v) = &self.min_children {
            map.serialize_entry("min_children", v)?;
        }
        map.serialize_entry("query", &self.query)?;
        if let Some(v) = &self.score_mode {
            map.serialize_entry("score_mode", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct HasParentQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "Indicates whether to ignore an unmapped `parent_type` and not return any documents instead of an error. You can use this parameter to query multiple indexes that may not contain the `parent_type`."]
    pub ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped>,
    pub inner_hits: Option<Box<crate::types::search::InnerHits>>,
    pub parent_type: crate::types::common::RelationName,
    pub query: Box<crate::types::common_query_dsl::QueryContainer>,
    #[doc = "Indicates whether the relevance score of a matching parent document is aggregated into its child documents."]
    pub score: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HasParentQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HasParentQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HasParentQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped> =
                    None;
                let mut seen_ignore_unmapped = false;
                let mut f_inner_hits: Option<Box<crate::types::search::InnerHits>> = None;
                let mut seen_inner_hits = false;
                let mut f_parent_type: Option<crate::types::common::RelationName> = None;
                let mut f_query: Option<Box<crate::types::common_query_dsl::QueryContainer>> = None;
                let mut f_score: Option<bool> = None;
                let mut seen_score = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "ignore_unmapped" => {
                            if seen_ignore_unmapped {
                                return Err(serde::de::Error::duplicate_field("ignore_unmapped"));
                            }
                            seen_ignore_unmapped = true;
                            f_ignore_unmapped = map.next_value::<Option<crate::types::common_query_dsl::IgnoreUnmapped>>()?;
                        }
                        "inner_hits" => {
                            if seen_inner_hits {
                                return Err(serde::de::Error::duplicate_field("inner_hits"));
                            }
                            seen_inner_hits = true;
                            f_inner_hits =
                                map.next_value::<Option<Box<crate::types::search::InnerHits>>>()?;
                        }
                        "parent_type" => {
                            if f_parent_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent_type"));
                            }
                            f_parent_type =
                                Some(map.next_value::<crate::types::common::RelationName>()?);
                        }
                        "query" => {
                            if f_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            f_query = Some(map.next_value::<Box<crate::types::common_query_dsl::QueryContainer>>()?);
                        }
                        "score" => {
                            if seen_score {
                                return Err(serde::de::Error::duplicate_field("score"));
                            }
                            seen_score = true;
                            f_score = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HasParentQuery {
                    name: f_name,
                    boost: f_boost,
                    ignore_unmapped: f_ignore_unmapped,
                    inner_hits: f_inner_hits,
                    parent_type: f_parent_type
                        .ok_or_else(|| serde::de::Error::missing_field("parent_type"))?,
                    query: f_query.ok_or_else(|| serde::de::Error::missing_field("query"))?,
                    score: f_score,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HasParentQuery {
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
        if let Some(v) = &self.inner_hits {
            map.serialize_entry("inner_hits", v)?;
        }
        map.serialize_entry("parent_type", &self.parent_type)?;
        map.serialize_entry("query", &self.query)?;
        if let Some(v) = &self.score {
            map.serialize_entry("score", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct HybridQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub filter: Option<Box<crate::types::common_query_dsl::QueryContainer>>,
    pub pagination_depth: Option<i32>,
    pub queries: Option<Vec<crate::types::common_query_dsl::QueryContainer>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HybridQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HybridQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HybridQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_filter: Option<Box<crate::types::common_query_dsl::QueryContainer>> =
                    None;
                let mut seen_filter = false;
                let mut f_pagination_depth: Option<i32> = None;
                let mut seen_pagination_depth = false;
                let mut f_queries: Option<Vec<crate::types::common_query_dsl::QueryContainer>> =
                    None;
                let mut seen_queries = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "filter" => {
                            if seen_filter {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            seen_filter = true;
                            f_filter = map.next_value::<Option<Box<crate::types::common_query_dsl::QueryContainer>>>()?;
                        }
                        "pagination_depth" => {
                            if seen_pagination_depth {
                                return Err(serde::de::Error::duplicate_field("pagination_depth"));
                            }
                            seen_pagination_depth = true;
                            f_pagination_depth = map.next_value::<Option<i32>>()?;
                        }
                        "queries" => {
                            if seen_queries {
                                return Err(serde::de::Error::duplicate_field("queries"));
                            }
                            seen_queries = true;
                            f_queries = map.next_value::<Option<Vec<crate::types::common_query_dsl::QueryContainer>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HybridQuery {
                    name: f_name,
                    boost: f_boost,
                    filter: f_filter,
                    pagination_depth: f_pagination_depth,
                    queries: f_queries,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HybridQuery {
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
        if let Some(v) = &self.filter {
            map.serialize_entry("filter", v)?;
        }
        if let Some(v) = &self.pagination_depth {
            map.serialize_entry("pagination_depth", v)?;
        }
        if let Some(v) = &self.queries {
            map.serialize_entry("queries", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IdsQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub values: Option<crate::types::common::Ids>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IdsQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IdsQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IdsQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_values: Option<crate::types::common::Ids> = None;
                let mut seen_values = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "values" => {
                            if seen_values {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            seen_values = true;
                            f_values = map.next_value::<Option<crate::types::common::Ids>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IdsQuery {
                    name: f_name,
                    boost: f_boost,
                    values: f_values,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IdsQuery {
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
        if let Some(v) = &self.values {
            map.serialize_entry("values", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Set to `true` to ignore an unmapped field and not match any documents for this query. Set to `false` to throw an exception if the field is not mapped."]
pub type IgnoreUnmapped = bool;

#[derive(Debug, Clone)]
pub struct IntervalsAllOf {
    pub filter: Option<Box<crate::types::common_query_dsl::IntervalsFilter>>,
    #[doc = "An array of rules to combine. All rules must produce a match in a document for the overall source to match."]
    pub intervals: Vec<crate::types::common_query_dsl::IntervalsContainer>,
    #[doc = "Maximum number of positions between the matching terms. Intervals produced by the rules further apart than this are not considered matches."]
    pub max_gaps: Option<i32>,
    #[doc = "If `true`, intervals produced by the rules should appear in the order in which they are specified."]
    pub ordered: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IntervalsAllOf {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IntervalsAllOf;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IntervalsAllOf object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_filter: Option<Box<crate::types::common_query_dsl::IntervalsFilter>> =
                    None;
                let mut seen_filter = false;
                let mut f_intervals: Option<
                    Vec<crate::types::common_query_dsl::IntervalsContainer>,
                > = None;
                let mut f_max_gaps: Option<i32> = None;
                let mut seen_max_gaps = false;
                let mut f_ordered: Option<bool> = None;
                let mut seen_ordered = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "filter" => {
                            if seen_filter {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            seen_filter = true;
                            f_filter = map.next_value::<Option<Box<crate::types::common_query_dsl::IntervalsFilter>>>()?;
                        }
                        "intervals" => {
                            if f_intervals.is_some() {
                                return Err(serde::de::Error::duplicate_field("intervals"));
                            }
                            f_intervals = Some(map.next_value::<Vec<crate::types::common_query_dsl::IntervalsContainer>>()?);
                        }
                        "max_gaps" => {
                            if seen_max_gaps {
                                return Err(serde::de::Error::duplicate_field("max_gaps"));
                            }
                            seen_max_gaps = true;
                            f_max_gaps = map.next_value::<Option<i32>>()?;
                        }
                        "ordered" => {
                            if seen_ordered {
                                return Err(serde::de::Error::duplicate_field("ordered"));
                            }
                            seen_ordered = true;
                            f_ordered = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IntervalsAllOf {
                    filter: f_filter,
                    intervals: f_intervals
                        .ok_or_else(|| serde::de::Error::missing_field("intervals"))?,
                    max_gaps: f_max_gaps,
                    ordered: f_ordered,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IntervalsAllOf {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.filter {
            map.serialize_entry("filter", v)?;
        }
        map.serialize_entry("intervals", &self.intervals)?;
        if let Some(v) = &self.max_gaps {
            map.serialize_entry("max_gaps", v)?;
        }
        if let Some(v) = &self.ordered {
            map.serialize_entry("ordered", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IntervalsAnyOf {
    pub filter: Option<Box<crate::types::common_query_dsl::IntervalsFilter>>,
    #[doc = "An array of rules to match."]
    pub intervals: Vec<crate::types::common_query_dsl::IntervalsContainer>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IntervalsAnyOf {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IntervalsAnyOf;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IntervalsAnyOf object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_filter: Option<Box<crate::types::common_query_dsl::IntervalsFilter>> =
                    None;
                let mut seen_filter = false;
                let mut f_intervals: Option<
                    Vec<crate::types::common_query_dsl::IntervalsContainer>,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "filter" => {
                            if seen_filter {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            seen_filter = true;
                            f_filter = map.next_value::<Option<Box<crate::types::common_query_dsl::IntervalsFilter>>>()?;
                        }
                        "intervals" => {
                            if f_intervals.is_some() {
                                return Err(serde::de::Error::duplicate_field("intervals"));
                            }
                            f_intervals = Some(map.next_value::<Vec<crate::types::common_query_dsl::IntervalsContainer>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IntervalsAnyOf {
                    filter: f_filter,
                    intervals: f_intervals
                        .ok_or_else(|| serde::de::Error::missing_field("intervals"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IntervalsAnyOf {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.filter {
            map.serialize_entry("filter", v)?;
        }
        map.serialize_entry("intervals", &self.intervals)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IntervalsContainer {
    pub all_of: Option<Box<crate::types::common_query_dsl::IntervalsAllOf>>,
    pub any_of: Option<Box<crate::types::common_query_dsl::IntervalsAnyOf>>,
    pub fuzzy: Option<crate::types::common_query_dsl::IntervalsFuzzy>,
    pub r#match: Option<Box<crate::types::common_query_dsl::IntervalsMatch>>,
    pub prefix: Option<crate::types::common_query_dsl::IntervalsPrefix>,
    pub wildcard: Option<crate::types::common_query_dsl::IntervalsWildcard>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IntervalsContainer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IntervalsContainer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IntervalsContainer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_all_of: Option<Box<crate::types::common_query_dsl::IntervalsAllOf>> =
                    None;
                let mut seen_all_of = false;
                let mut f_any_of: Option<Box<crate::types::common_query_dsl::IntervalsAnyOf>> =
                    None;
                let mut seen_any_of = false;
                let mut f_fuzzy: Option<crate::types::common_query_dsl::IntervalsFuzzy> = None;
                let mut seen_fuzzy = false;
                let mut f_match: Option<Box<crate::types::common_query_dsl::IntervalsMatch>> = None;
                let mut seen_match = false;
                let mut f_prefix: Option<crate::types::common_query_dsl::IntervalsPrefix> = None;
                let mut seen_prefix = false;
                let mut f_wildcard: Option<crate::types::common_query_dsl::IntervalsWildcard> =
                    None;
                let mut seen_wildcard = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "all_of" => {
                            if seen_all_of {
                                return Err(serde::de::Error::duplicate_field("all_of"));
                            }
                            seen_all_of = true;
                            f_all_of = map.next_value::<Option<Box<crate::types::common_query_dsl::IntervalsAllOf>>>()?;
                        }
                        "any_of" => {
                            if seen_any_of {
                                return Err(serde::de::Error::duplicate_field("any_of"));
                            }
                            seen_any_of = true;
                            f_any_of = map.next_value::<Option<Box<crate::types::common_query_dsl::IntervalsAnyOf>>>()?;
                        }
                        "fuzzy" => {
                            if seen_fuzzy {
                                return Err(serde::de::Error::duplicate_field("fuzzy"));
                            }
                            seen_fuzzy = true;
                            f_fuzzy = map.next_value::<Option<crate::types::common_query_dsl::IntervalsFuzzy>>()?;
                        }
                        "match" => {
                            if seen_match {
                                return Err(serde::de::Error::duplicate_field("match"));
                            }
                            seen_match = true;
                            f_match = map.next_value::<Option<Box<crate::types::common_query_dsl::IntervalsMatch>>>()?;
                        }
                        "prefix" => {
                            if seen_prefix {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            seen_prefix = true;
                            f_prefix = map.next_value::<Option<crate::types::common_query_dsl::IntervalsPrefix>>()?;
                        }
                        "wildcard" => {
                            if seen_wildcard {
                                return Err(serde::de::Error::duplicate_field("wildcard"));
                            }
                            seen_wildcard = true;
                            f_wildcard = map.next_value::<Option<crate::types::common_query_dsl::IntervalsWildcard>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IntervalsContainer {
                    all_of: f_all_of,
                    any_of: f_any_of,
                    fuzzy: f_fuzzy,
                    r#match: f_match,
                    prefix: f_prefix,
                    wildcard: f_wildcard,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IntervalsContainer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.all_of {
            map.serialize_entry("all_of", v)?;
        }
        if let Some(v) = &self.any_of {
            map.serialize_entry("any_of", v)?;
        }
        if let Some(v) = &self.fuzzy {
            map.serialize_entry("fuzzy", v)?;
        }
        if let Some(v) = &self.r#match {
            map.serialize_entry("match", v)?;
        }
        if let Some(v) = &self.prefix {
            map.serialize_entry("prefix", v)?;
        }
        if let Some(v) = &self.wildcard {
            map.serialize_entry("wildcard", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IntervalsFilter {
    pub after: Option<Box<crate::types::common_query_dsl::IntervalsContainer>>,
    pub before: Option<Box<crate::types::common_query_dsl::IntervalsContainer>>,
    pub contained_by: Option<Box<crate::types::common_query_dsl::IntervalsContainer>>,
    pub containing: Option<Box<crate::types::common_query_dsl::IntervalsContainer>>,
    pub not_contained_by: Option<Box<crate::types::common_query_dsl::IntervalsContainer>>,
    pub not_containing: Option<Box<crate::types::common_query_dsl::IntervalsContainer>>,
    pub not_overlapping: Option<Box<crate::types::common_query_dsl::IntervalsContainer>>,
    pub overlapping: Option<Box<crate::types::common_query_dsl::IntervalsContainer>>,
    pub script: Option<crate::types::common::Script>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IntervalsFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IntervalsFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IntervalsFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_after: Option<Box<crate::types::common_query_dsl::IntervalsContainer>> =
                    None;
                let mut seen_after = false;
                let mut f_before: Option<Box<crate::types::common_query_dsl::IntervalsContainer>> =
                    None;
                let mut seen_before = false;
                let mut f_contained_by: Option<
                    Box<crate::types::common_query_dsl::IntervalsContainer>,
                > = None;
                let mut seen_contained_by = false;
                let mut f_containing: Option<
                    Box<crate::types::common_query_dsl::IntervalsContainer>,
                > = None;
                let mut seen_containing = false;
                let mut f_not_contained_by: Option<
                    Box<crate::types::common_query_dsl::IntervalsContainer>,
                > = None;
                let mut seen_not_contained_by = false;
                let mut f_not_containing: Option<
                    Box<crate::types::common_query_dsl::IntervalsContainer>,
                > = None;
                let mut seen_not_containing = false;
                let mut f_not_overlapping: Option<
                    Box<crate::types::common_query_dsl::IntervalsContainer>,
                > = None;
                let mut seen_not_overlapping = false;
                let mut f_overlapping: Option<
                    Box<crate::types::common_query_dsl::IntervalsContainer>,
                > = None;
                let mut seen_overlapping = false;
                let mut f_script: Option<crate::types::common::Script> = None;
                let mut seen_script = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "after" => {
                            if seen_after {
                                return Err(serde::de::Error::duplicate_field("after"));
                            }
                            seen_after = true;
                            f_after = map.next_value::<Option<Box<crate::types::common_query_dsl::IntervalsContainer>>>()?;
                        }
                        "before" => {
                            if seen_before {
                                return Err(serde::de::Error::duplicate_field("before"));
                            }
                            seen_before = true;
                            f_before = map.next_value::<Option<Box<crate::types::common_query_dsl::IntervalsContainer>>>()?;
                        }
                        "contained_by" => {
                            if seen_contained_by {
                                return Err(serde::de::Error::duplicate_field("contained_by"));
                            }
                            seen_contained_by = true;
                            f_contained_by = map.next_value::<Option<Box<crate::types::common_query_dsl::IntervalsContainer>>>()?;
                        }
                        "containing" => {
                            if seen_containing {
                                return Err(serde::de::Error::duplicate_field("containing"));
                            }
                            seen_containing = true;
                            f_containing = map.next_value::<Option<Box<crate::types::common_query_dsl::IntervalsContainer>>>()?;
                        }
                        "not_contained_by" => {
                            if seen_not_contained_by {
                                return Err(serde::de::Error::duplicate_field("not_contained_by"));
                            }
                            seen_not_contained_by = true;
                            f_not_contained_by = map.next_value::<Option<Box<crate::types::common_query_dsl::IntervalsContainer>>>()?;
                        }
                        "not_containing" => {
                            if seen_not_containing {
                                return Err(serde::de::Error::duplicate_field("not_containing"));
                            }
                            seen_not_containing = true;
                            f_not_containing = map.next_value::<Option<Box<crate::types::common_query_dsl::IntervalsContainer>>>()?;
                        }
                        "not_overlapping" => {
                            if seen_not_overlapping {
                                return Err(serde::de::Error::duplicate_field("not_overlapping"));
                            }
                            seen_not_overlapping = true;
                            f_not_overlapping = map.next_value::<Option<Box<crate::types::common_query_dsl::IntervalsContainer>>>()?;
                        }
                        "overlapping" => {
                            if seen_overlapping {
                                return Err(serde::de::Error::duplicate_field("overlapping"));
                            }
                            seen_overlapping = true;
                            f_overlapping = map.next_value::<Option<Box<crate::types::common_query_dsl::IntervalsContainer>>>()?;
                        }
                        "script" => {
                            if seen_script {
                                return Err(serde::de::Error::duplicate_field("script"));
                            }
                            seen_script = true;
                            f_script = map.next_value::<Option<crate::types::common::Script>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IntervalsFilter {
                    after: f_after,
                    before: f_before,
                    contained_by: f_contained_by,
                    containing: f_containing,
                    not_contained_by: f_not_contained_by,
                    not_containing: f_not_containing,
                    not_overlapping: f_not_overlapping,
                    overlapping: f_overlapping,
                    script: f_script,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IntervalsFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.after {
            map.serialize_entry("after", v)?;
        }
        if let Some(v) = &self.before {
            map.serialize_entry("before", v)?;
        }
        if let Some(v) = &self.contained_by {
            map.serialize_entry("contained_by", v)?;
        }
        if let Some(v) = &self.containing {
            map.serialize_entry("containing", v)?;
        }
        if let Some(v) = &self.not_contained_by {
            map.serialize_entry("not_contained_by", v)?;
        }
        if let Some(v) = &self.not_containing {
            map.serialize_entry("not_containing", v)?;
        }
        if let Some(v) = &self.not_overlapping {
            map.serialize_entry("not_overlapping", v)?;
        }
        if let Some(v) = &self.overlapping {
            map.serialize_entry("overlapping", v)?;
        }
        if let Some(v) = &self.script {
            map.serialize_entry("script", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IntervalsFuzzy {
    #[doc = "Analyzer used to normalize the term."]
    pub analyzer: Option<String>,
    pub fuzziness: Option<crate::types::common::Fuzziness>,
    #[doc = "Number of beginning characters left unchanged when creating expansions."]
    pub prefix_length: Option<i32>,
    #[doc = "The term to match."]
    pub term: String,
    #[doc = "Indicates whether edits include transpositions of two adjacent characters (for example, `ab` to `ba`)."]
    pub transpositions: Option<bool>,
    pub use_field: Option<crate::types::common::Field>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IntervalsFuzzy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IntervalsFuzzy;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IntervalsFuzzy object")
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
                let mut f_fuzziness: Option<crate::types::common::Fuzziness> = None;
                let mut seen_fuzziness = false;
                let mut f_prefix_length: Option<i32> = None;
                let mut seen_prefix_length = false;
                let mut f_term: Option<String> = None;
                let mut f_transpositions: Option<bool> = None;
                let mut seen_transpositions = false;
                let mut f_use_field: Option<crate::types::common::Field> = None;
                let mut seen_use_field = false;
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
                        "fuzziness" => {
                            if seen_fuzziness {
                                return Err(serde::de::Error::duplicate_field("fuzziness"));
                            }
                            seen_fuzziness = true;
                            f_fuzziness =
                                map.next_value::<Option<crate::types::common::Fuzziness>>()?;
                        }
                        "prefix_length" => {
                            if seen_prefix_length {
                                return Err(serde::de::Error::duplicate_field("prefix_length"));
                            }
                            seen_prefix_length = true;
                            f_prefix_length = map.next_value::<Option<i32>>()?;
                        }
                        "term" => {
                            if f_term.is_some() {
                                return Err(serde::de::Error::duplicate_field("term"));
                            }
                            f_term = Some(map.next_value::<String>()?);
                        }
                        "transpositions" => {
                            if seen_transpositions {
                                return Err(serde::de::Error::duplicate_field("transpositions"));
                            }
                            seen_transpositions = true;
                            f_transpositions = map.next_value::<Option<bool>>()?;
                        }
                        "use_field" => {
                            if seen_use_field {
                                return Err(serde::de::Error::duplicate_field("use_field"));
                            }
                            seen_use_field = true;
                            f_use_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IntervalsFuzzy {
                    analyzer: f_analyzer,
                    fuzziness: f_fuzziness,
                    prefix_length: f_prefix_length,
                    term: f_term.ok_or_else(|| serde::de::Error::missing_field("term"))?,
                    transpositions: f_transpositions,
                    use_field: f_use_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IntervalsFuzzy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.analyzer {
            map.serialize_entry("analyzer", v)?;
        }
        if let Some(v) = &self.fuzziness {
            map.serialize_entry("fuzziness", v)?;
        }
        if let Some(v) = &self.prefix_length {
            map.serialize_entry("prefix_length", v)?;
        }
        map.serialize_entry("term", &self.term)?;
        if let Some(v) = &self.transpositions {
            map.serialize_entry("transpositions", v)?;
        }
        if let Some(v) = &self.use_field {
            map.serialize_entry("use_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IntervalsMatch {
    #[doc = "Analyzer used to analyze terms in the query."]
    pub analyzer: Option<String>,
    pub filter: Option<Box<crate::types::common_query_dsl::IntervalsFilter>>,
    #[doc = "Maximum number of positions between the matching terms. Terms further apart than this are not considered matches."]
    pub max_gaps: Option<i32>,
    #[doc = "If `true`, matching terms must appear in their specified order."]
    pub ordered: Option<bool>,
    #[doc = "Text you wish to find in the provided field."]
    pub query: String,
    pub use_field: Option<crate::types::common::Field>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IntervalsMatch {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IntervalsMatch;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IntervalsMatch object")
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
                let mut f_filter: Option<Box<crate::types::common_query_dsl::IntervalsFilter>> =
                    None;
                let mut seen_filter = false;
                let mut f_max_gaps: Option<i32> = None;
                let mut seen_max_gaps = false;
                let mut f_ordered: Option<bool> = None;
                let mut seen_ordered = false;
                let mut f_query: Option<String> = None;
                let mut f_use_field: Option<crate::types::common::Field> = None;
                let mut seen_use_field = false;
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
                        "filter" => {
                            if seen_filter {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            seen_filter = true;
                            f_filter = map.next_value::<Option<Box<crate::types::common_query_dsl::IntervalsFilter>>>()?;
                        }
                        "max_gaps" => {
                            if seen_max_gaps {
                                return Err(serde::de::Error::duplicate_field("max_gaps"));
                            }
                            seen_max_gaps = true;
                            f_max_gaps = map.next_value::<Option<i32>>()?;
                        }
                        "ordered" => {
                            if seen_ordered {
                                return Err(serde::de::Error::duplicate_field("ordered"));
                            }
                            seen_ordered = true;
                            f_ordered = map.next_value::<Option<bool>>()?;
                        }
                        "query" => {
                            if f_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            f_query = Some(map.next_value::<String>()?);
                        }
                        "use_field" => {
                            if seen_use_field {
                                return Err(serde::de::Error::duplicate_field("use_field"));
                            }
                            seen_use_field = true;
                            f_use_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IntervalsMatch {
                    analyzer: f_analyzer,
                    filter: f_filter,
                    max_gaps: f_max_gaps,
                    ordered: f_ordered,
                    query: f_query.ok_or_else(|| serde::de::Error::missing_field("query"))?,
                    use_field: f_use_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IntervalsMatch {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.analyzer {
            map.serialize_entry("analyzer", v)?;
        }
        if let Some(v) = &self.filter {
            map.serialize_entry("filter", v)?;
        }
        if let Some(v) = &self.max_gaps {
            map.serialize_entry("max_gaps", v)?;
        }
        if let Some(v) = &self.ordered {
            map.serialize_entry("ordered", v)?;
        }
        map.serialize_entry("query", &self.query)?;
        if let Some(v) = &self.use_field {
            map.serialize_entry("use_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IntervalsPrefix {
    #[doc = "Analyzer used to analyze the `prefix`."]
    pub analyzer: Option<String>,
    #[doc = "Beginning characters of terms you wish to find in the top-level field."]
    pub prefix: String,
    pub use_field: Option<crate::types::common::Field>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IntervalsPrefix {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IntervalsPrefix;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IntervalsPrefix object")
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
                let mut f_prefix: Option<String> = None;
                let mut f_use_field: Option<crate::types::common::Field> = None;
                let mut seen_use_field = false;
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
                        "prefix" => {
                            if f_prefix.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            f_prefix = Some(map.next_value::<String>()?);
                        }
                        "use_field" => {
                            if seen_use_field {
                                return Err(serde::de::Error::duplicate_field("use_field"));
                            }
                            seen_use_field = true;
                            f_use_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IntervalsPrefix {
                    analyzer: f_analyzer,
                    prefix: f_prefix.ok_or_else(|| serde::de::Error::missing_field("prefix"))?,
                    use_field: f_use_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IntervalsPrefix {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.analyzer {
            map.serialize_entry("analyzer", v)?;
        }
        map.serialize_entry("prefix", &self.prefix)?;
        if let Some(v) = &self.use_field {
            map.serialize_entry("use_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IntervalsQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub all_of: Option<crate::types::common_query_dsl::IntervalsAllOf>,
    pub any_of: Option<crate::types::common_query_dsl::IntervalsAnyOf>,
    pub fuzzy: Option<crate::types::common_query_dsl::IntervalsFuzzy>,
    pub r#match: Option<crate::types::common_query_dsl::IntervalsMatch>,
    pub prefix: Option<crate::types::common_query_dsl::IntervalsPrefix>,
    pub wildcard: Option<crate::types::common_query_dsl::IntervalsWildcard>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IntervalsQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IntervalsQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IntervalsQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_all_of: Option<crate::types::common_query_dsl::IntervalsAllOf> = None;
                let mut seen_all_of = false;
                let mut f_any_of: Option<crate::types::common_query_dsl::IntervalsAnyOf> = None;
                let mut seen_any_of = false;
                let mut f_fuzzy: Option<crate::types::common_query_dsl::IntervalsFuzzy> = None;
                let mut seen_fuzzy = false;
                let mut f_match: Option<crate::types::common_query_dsl::IntervalsMatch> = None;
                let mut seen_match = false;
                let mut f_prefix: Option<crate::types::common_query_dsl::IntervalsPrefix> = None;
                let mut seen_prefix = false;
                let mut f_wildcard: Option<crate::types::common_query_dsl::IntervalsWildcard> =
                    None;
                let mut seen_wildcard = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "all_of" => {
                            if seen_all_of {
                                return Err(serde::de::Error::duplicate_field("all_of"));
                            }
                            seen_all_of = true;
                            f_all_of = map.next_value::<Option<crate::types::common_query_dsl::IntervalsAllOf>>()?;
                        }
                        "any_of" => {
                            if seen_any_of {
                                return Err(serde::de::Error::duplicate_field("any_of"));
                            }
                            seen_any_of = true;
                            f_any_of = map.next_value::<Option<crate::types::common_query_dsl::IntervalsAnyOf>>()?;
                        }
                        "fuzzy" => {
                            if seen_fuzzy {
                                return Err(serde::de::Error::duplicate_field("fuzzy"));
                            }
                            seen_fuzzy = true;
                            f_fuzzy = map.next_value::<Option<crate::types::common_query_dsl::IntervalsFuzzy>>()?;
                        }
                        "match" => {
                            if seen_match {
                                return Err(serde::de::Error::duplicate_field("match"));
                            }
                            seen_match = true;
                            f_match = map.next_value::<Option<crate::types::common_query_dsl::IntervalsMatch>>()?;
                        }
                        "prefix" => {
                            if seen_prefix {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            seen_prefix = true;
                            f_prefix = map.next_value::<Option<crate::types::common_query_dsl::IntervalsPrefix>>()?;
                        }
                        "wildcard" => {
                            if seen_wildcard {
                                return Err(serde::de::Error::duplicate_field("wildcard"));
                            }
                            seen_wildcard = true;
                            f_wildcard = map.next_value::<Option<crate::types::common_query_dsl::IntervalsWildcard>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IntervalsQuery {
                    name: f_name,
                    boost: f_boost,
                    all_of: f_all_of,
                    any_of: f_any_of,
                    fuzzy: f_fuzzy,
                    r#match: f_match,
                    prefix: f_prefix,
                    wildcard: f_wildcard,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IntervalsQuery {
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
        if let Some(v) = &self.all_of {
            map.serialize_entry("all_of", v)?;
        }
        if let Some(v) = &self.any_of {
            map.serialize_entry("any_of", v)?;
        }
        if let Some(v) = &self.fuzzy {
            map.serialize_entry("fuzzy", v)?;
        }
        if let Some(v) = &self.r#match {
            map.serialize_entry("match", v)?;
        }
        if let Some(v) = &self.prefix {
            map.serialize_entry("prefix", v)?;
        }
        if let Some(v) = &self.wildcard {
            map.serialize_entry("wildcard", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IntervalsWildcard {
    #[doc = "Analyzer used to analyze the `pattern`. Defaults to the top-level field's analyzer."]
    pub analyzer: Option<String>,
    #[doc = "Wildcard pattern used to find matching terms."]
    pub pattern: String,
    pub use_field: Option<crate::types::common::Field>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IntervalsWildcard {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IntervalsWildcard;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IntervalsWildcard object")
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
                let mut f_pattern: Option<String> = None;
                let mut f_use_field: Option<crate::types::common::Field> = None;
                let mut seen_use_field = false;
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
                        "pattern" => {
                            if f_pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("pattern"));
                            }
                            f_pattern = Some(map.next_value::<String>()?);
                        }
                        "use_field" => {
                            if seen_use_field {
                                return Err(serde::de::Error::duplicate_field("use_field"));
                            }
                            seen_use_field = true;
                            f_use_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IntervalsWildcard {
                    analyzer: f_analyzer,
                    pattern: f_pattern.ok_or_else(|| serde::de::Error::missing_field("pattern"))?,
                    use_field: f_use_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IntervalsWildcard {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.analyzer {
            map.serialize_entry("analyzer", v)?;
        }
        map.serialize_entry("pattern", &self.pattern)?;
        if let Some(v) = &self.use_field {
            map.serialize_entry("use_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct KnnQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub expand_nested_docs: Option<bool>,
    #[doc = "The filter for the k-NN search query."]
    pub filter: Option<crate::types::common_query_dsl::QueryContainer>,
    #[doc = "The total number of nearest neighbors to return as top hits."]
    pub k: Option<i32>,
    #[doc = "The maximum physical vector space distance required in order for a neighbor to be considered a hit."]
    pub max_distance: Option<f64>,
    pub method_parameters:
        Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The minimum similarity score required in order for a neighbor to be considered a hit."]
    pub min_score: Option<f64>,
    pub rescore: Option<crate::types::common_query_dsl::KnnQueryRescore>,
    pub vector: crate::types::common_query_dsl::QueryVector,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for KnnQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = KnnQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a KnnQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_expand_nested_docs: Option<bool> = None;
                let mut seen_expand_nested_docs = false;
                let mut f_filter: Option<crate::types::common_query_dsl::QueryContainer> = None;
                let mut seen_filter = false;
                let mut f_k: Option<i32> = None;
                let mut seen_k = false;
                let mut f_max_distance: Option<f64> = None;
                let mut seen_max_distance = false;
                let mut f_method_parameters: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_method_parameters = false;
                let mut f_min_score: Option<f64> = None;
                let mut seen_min_score = false;
                let mut f_rescore: Option<crate::types::common_query_dsl::KnnQueryRescore> = None;
                let mut seen_rescore = false;
                let mut f_vector: Option<crate::types::common_query_dsl::QueryVector> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "expand_nested_docs" => {
                            if seen_expand_nested_docs {
                                return Err(serde::de::Error::duplicate_field(
                                    "expand_nested_docs",
                                ));
                            }
                            seen_expand_nested_docs = true;
                            f_expand_nested_docs = map.next_value::<Option<bool>>()?;
                        }
                        "filter" => {
                            if seen_filter {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            seen_filter = true;
                            f_filter = map.next_value::<Option<crate::types::common_query_dsl::QueryContainer>>()?;
                        }
                        "k" => {
                            if seen_k {
                                return Err(serde::de::Error::duplicate_field("k"));
                            }
                            seen_k = true;
                            f_k = map.next_value::<Option<i32>>()?;
                        }
                        "max_distance" => {
                            if seen_max_distance {
                                return Err(serde::de::Error::duplicate_field("max_distance"));
                            }
                            seen_max_distance = true;
                            f_max_distance = map.next_value::<Option<f64>>()?;
                        }
                        "method_parameters" => {
                            if seen_method_parameters {
                                return Err(serde::de::Error::duplicate_field("method_parameters"));
                            }
                            seen_method_parameters = true;
                            f_method_parameters = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "min_score" => {
                            if seen_min_score {
                                return Err(serde::de::Error::duplicate_field("min_score"));
                            }
                            seen_min_score = true;
                            f_min_score = map.next_value::<Option<f64>>()?;
                        }
                        "rescore" => {
                            if seen_rescore {
                                return Err(serde::de::Error::duplicate_field("rescore"));
                            }
                            seen_rescore = true;
                            f_rescore = map.next_value::<Option<crate::types::common_query_dsl::KnnQueryRescore>>()?;
                        }
                        "vector" => {
                            if f_vector.is_some() {
                                return Err(serde::de::Error::duplicate_field("vector"));
                            }
                            f_vector = Some(
                                map.next_value::<crate::types::common_query_dsl::QueryVector>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(KnnQuery {
                    name: f_name,
                    boost: f_boost,
                    expand_nested_docs: f_expand_nested_docs,
                    filter: f_filter,
                    k: f_k,
                    max_distance: f_max_distance,
                    method_parameters: f_method_parameters,
                    min_score: f_min_score,
                    rescore: f_rescore,
                    vector: f_vector.ok_or_else(|| serde::de::Error::missing_field("vector"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for KnnQuery {
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
        if let Some(v) = &self.expand_nested_docs {
            map.serialize_entry("expand_nested_docs", v)?;
        }
        if let Some(v) = &self.filter {
            map.serialize_entry("filter", v)?;
        }
        if let Some(v) = &self.k {
            map.serialize_entry("k", v)?;
        }
        if let Some(v) = &self.max_distance {
            map.serialize_entry("max_distance", v)?;
        }
        if let Some(v) = &self.method_parameters {
            map.serialize_entry("method_parameters", v)?;
        }
        if let Some(v) = &self.min_score {
            map.serialize_entry("min_score", v)?;
        }
        if let Some(v) = &self.rescore {
            map.serialize_entry("rescore", v)?;
        }
        map.serialize_entry("vector", &self.vector)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b't') | Some(b'f') => serde_json::from_str(text).map(KnnQueryRescore::Enable),
            Some(b'{') => serde_json::from_str(text).map(KnnQueryRescore::Context),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a KnnQueryRescore value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'"') => serde_json::from_str(text).map(Like::Text),
            Some(b'{') => serde_json::from_str(text).map(Like::Document),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a Like value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct LikeDocument {
    pub id: Option<crate::types::common::Id>,
    pub index: Option<crate::types::common::IndexName>,
    pub r#type: Option<crate::types::common::Type>,
    #[doc = "A document not present in the index."]
    pub doc: Option<Box<serde_json::value::RawValue>>,
    pub fields: Option<Vec<crate::types::common::Field>>,
    pub per_field_analyzer: Option<std::collections::BTreeMap<String, String>>,
    pub routing: Option<crate::types::common::Routing>,
    pub version: Option<crate::types::common::VersionNumber>,
    pub version_type: Option<crate::types::common::VersionType>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for LikeDocument {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = LikeDocument;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a LikeDocument object")
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
                let mut seen_id = false;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut seen_index = false;
                let mut f_type: Option<crate::types::common::Type> = None;
                let mut seen_type = false;
                let mut f_doc: Option<Box<serde_json::value::RawValue>> = None;
                let mut seen_doc = false;
                let mut f_fields: Option<Vec<crate::types::common::Field>> = None;
                let mut seen_fields = false;
                let mut f_per_field_analyzer: Option<std::collections::BTreeMap<String, String>> =
                    None;
                let mut seen_per_field_analyzer = false;
                let mut f_routing: Option<crate::types::common::Routing> = None;
                let mut seen_routing = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version = false;
                let mut f_version_type: Option<crate::types::common::VersionType> = None;
                let mut seen_version_type = false;
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
                            f_id = map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        "_index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("_index"));
                            }
                            seen_index = true;
                            f_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "_type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<crate::types::common::Type>>()?;
                        }
                        "doc" => {
                            if seen_doc {
                                return Err(serde::de::Error::duplicate_field("doc"));
                            }
                            seen_doc = true;
                            f_doc = map.next_value::<Option<Box<serde_json::value::RawValue>>>()?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields =
                                map.next_value::<Option<Vec<crate::types::common::Field>>>()?;
                        }
                        "per_field_analyzer" => {
                            if seen_per_field_analyzer {
                                return Err(serde::de::Error::duplicate_field(
                                    "per_field_analyzer",
                                ));
                            }
                            seen_per_field_analyzer = true;
                            f_per_field_analyzer = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("routing"));
                            }
                            seen_routing = true;
                            f_routing =
                                map.next_value::<Option<crate::types::common::Routing>>()?;
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionNumber>>()?;
                        }
                        "version_type" => {
                            if seen_version_type {
                                return Err(serde::de::Error::duplicate_field("version_type"));
                            }
                            seen_version_type = true;
                            f_version_type =
                                map.next_value::<Option<crate::types::common::VersionType>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(LikeDocument {
                    id: f_id,
                    index: f_index,
                    r#type: f_type,
                    doc: f_doc,
                    fields: f_fields,
                    per_field_analyzer: f_per_field_analyzer,
                    routing: f_routing,
                    version: f_version,
                    version_type: f_version_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for LikeDocument {
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
        if let Some(v) = &self.r#type {
            map.serialize_entry("_type", v)?;
        }
        if let Some(v) = &self.doc {
            map.serialize_entry("doc", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.per_field_analyzer {
            map.serialize_entry("per_field_analyzer", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("routing", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.version_type {
            map.serialize_entry("version_type", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct MatchAllQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MatchAllQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MatchAllQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MatchAllQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MatchAllQuery {
                    name: f_name,
                    boost: f_boost,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MatchAllQuery {
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
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'"') => serde_json::from_str(text).map(MatchBoolPrefixQuery::Query),
            Some(b'{') => serde_json::from_str(text).map(MatchBoolPrefixQuery::Object),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a MatchBoolPrefixQuery value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct MatchBoolPrefixQueryObject {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "Analyzer used to convert the text in the query value into tokens."]
    pub analyzer: Option<String>,
    pub fuzziness: Option<crate::types::common::Fuzziness>,
    pub fuzzy_rewrite: Option<crate::types::common::MultiTermQueryRewrite>,
    #[doc = "If `true`, edits for fuzzy matching include transpositions of two adjacent characters (for example, `ab` to `ba`). Can be applied to the term subqueries constructed for all terms but the final term."]
    pub fuzzy_transpositions: Option<bool>,
    #[doc = "Maximum number of terms to which the query will expand. Can be applied to the term subqueries constructed for all terms but the final term."]
    pub max_expansions: Option<i32>,
    pub minimum_should_match: Option<crate::types::common::MinimumShouldMatch>,
    pub operator: Option<crate::types::common_query_dsl::Operator>,
    #[doc = "Number of beginning characters left unchanged for fuzzy matching. Can be applied to the term subqueries constructed for all terms but the final term."]
    pub prefix_length: Option<i32>,
    #[doc = "Terms you wish to find in the provided field. The last term is used in a prefix query."]
    pub query: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MatchBoolPrefixQueryObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MatchBoolPrefixQueryObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MatchBoolPrefixQueryObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_analyzer: Option<String> = None;
                let mut seen_analyzer = false;
                let mut f_fuzziness: Option<crate::types::common::Fuzziness> = None;
                let mut seen_fuzziness = false;
                let mut f_fuzzy_rewrite: Option<crate::types::common::MultiTermQueryRewrite> = None;
                let mut seen_fuzzy_rewrite = false;
                let mut f_fuzzy_transpositions: Option<bool> = None;
                let mut seen_fuzzy_transpositions = false;
                let mut f_max_expansions: Option<i32> = None;
                let mut seen_max_expansions = false;
                let mut f_minimum_should_match: Option<crate::types::common::MinimumShouldMatch> =
                    None;
                let mut seen_minimum_should_match = false;
                let mut f_operator: Option<crate::types::common_query_dsl::Operator> = None;
                let mut seen_operator = false;
                let mut f_prefix_length: Option<i32> = None;
                let mut seen_prefix_length = false;
                let mut f_query: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "analyzer" => {
                            if seen_analyzer {
                                return Err(serde::de::Error::duplicate_field("analyzer"));
                            }
                            seen_analyzer = true;
                            f_analyzer = map.next_value::<Option<String>>()?;
                        }
                        "fuzziness" => {
                            if seen_fuzziness {
                                return Err(serde::de::Error::duplicate_field("fuzziness"));
                            }
                            seen_fuzziness = true;
                            f_fuzziness =
                                map.next_value::<Option<crate::types::common::Fuzziness>>()?;
                        }
                        "fuzzy_rewrite" => {
                            if seen_fuzzy_rewrite {
                                return Err(serde::de::Error::duplicate_field("fuzzy_rewrite"));
                            }
                            seen_fuzzy_rewrite = true;
                            f_fuzzy_rewrite = map
                                .next_value::<Option<crate::types::common::MultiTermQueryRewrite>>(
                                )?;
                        }
                        "fuzzy_transpositions" => {
                            if seen_fuzzy_transpositions {
                                return Err(serde::de::Error::duplicate_field(
                                    "fuzzy_transpositions",
                                ));
                            }
                            seen_fuzzy_transpositions = true;
                            f_fuzzy_transpositions = map.next_value::<Option<bool>>()?;
                        }
                        "max_expansions" => {
                            if seen_max_expansions {
                                return Err(serde::de::Error::duplicate_field("max_expansions"));
                            }
                            seen_max_expansions = true;
                            f_max_expansions = map.next_value::<Option<i32>>()?;
                        }
                        "minimum_should_match" => {
                            if seen_minimum_should_match {
                                return Err(serde::de::Error::duplicate_field(
                                    "minimum_should_match",
                                ));
                            }
                            seen_minimum_should_match = true;
                            f_minimum_should_match = map
                                .next_value::<Option<crate::types::common::MinimumShouldMatch>>()?;
                        }
                        "operator" => {
                            if seen_operator {
                                return Err(serde::de::Error::duplicate_field("operator"));
                            }
                            seen_operator = true;
                            f_operator = map
                                .next_value::<Option<crate::types::common_query_dsl::Operator>>()?;
                        }
                        "prefix_length" => {
                            if seen_prefix_length {
                                return Err(serde::de::Error::duplicate_field("prefix_length"));
                            }
                            seen_prefix_length = true;
                            f_prefix_length = map.next_value::<Option<i32>>()?;
                        }
                        "query" => {
                            if f_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            f_query = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MatchBoolPrefixQueryObject {
                    name: f_name,
                    boost: f_boost,
                    analyzer: f_analyzer,
                    fuzziness: f_fuzziness,
                    fuzzy_rewrite: f_fuzzy_rewrite,
                    fuzzy_transpositions: f_fuzzy_transpositions,
                    max_expansions: f_max_expansions,
                    minimum_should_match: f_minimum_should_match,
                    operator: f_operator,
                    prefix_length: f_prefix_length,
                    query: f_query.ok_or_else(|| serde::de::Error::missing_field("query"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MatchBoolPrefixQueryObject {
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
        if let Some(v) = &self.analyzer {
            map.serialize_entry("analyzer", v)?;
        }
        if let Some(v) = &self.fuzziness {
            map.serialize_entry("fuzziness", v)?;
        }
        if let Some(v) = &self.fuzzy_rewrite {
            map.serialize_entry("fuzzy_rewrite", v)?;
        }
        if let Some(v) = &self.fuzzy_transpositions {
            map.serialize_entry("fuzzy_transpositions", v)?;
        }
        if let Some(v) = &self.max_expansions {
            map.serialize_entry("max_expansions", v)?;
        }
        if let Some(v) = &self.minimum_should_match {
            map.serialize_entry("minimum_should_match", v)?;
        }
        if let Some(v) = &self.operator {
            map.serialize_entry("operator", v)?;
        }
        if let Some(v) = &self.prefix_length {
            map.serialize_entry("prefix_length", v)?;
        }
        map.serialize_entry("query", &self.query)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct MatchNoneQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MatchNoneQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MatchNoneQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MatchNoneQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MatchNoneQuery {
                    name: f_name,
                    boost: f_boost,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MatchNoneQuery {
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
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'"') => serde_json::from_str(text).map(MatchPhrasePrefixQuery::Query),
            Some(b'{') => serde_json::from_str(text).map(MatchPhrasePrefixQuery::Object),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a MatchPhrasePrefixQuery value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct MatchPhrasePrefixQueryObject {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "Analyzer used to convert text in the query value into tokens."]
    pub analyzer: Option<String>,
    #[doc = "Maximum number of terms to which the last provided term of the query value will expand."]
    pub max_expansions: Option<i32>,
    #[doc = "Text you wish to find in the provided field."]
    pub query: String,
    #[doc = "Maximum number of positions allowed between matching tokens."]
    pub slop: Option<i32>,
    pub zero_terms_query: Option<crate::types::common_query_dsl::ZeroTermsQuery>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MatchPhrasePrefixQueryObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MatchPhrasePrefixQueryObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MatchPhrasePrefixQueryObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_analyzer: Option<String> = None;
                let mut seen_analyzer = false;
                let mut f_max_expansions: Option<i32> = None;
                let mut seen_max_expansions = false;
                let mut f_query: Option<String> = None;
                let mut f_slop: Option<i32> = None;
                let mut seen_slop = false;
                let mut f_zero_terms_query: Option<crate::types::common_query_dsl::ZeroTermsQuery> =
                    None;
                let mut seen_zero_terms_query = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "analyzer" => {
                            if seen_analyzer {
                                return Err(serde::de::Error::duplicate_field("analyzer"));
                            }
                            seen_analyzer = true;
                            f_analyzer = map.next_value::<Option<String>>()?;
                        }
                        "max_expansions" => {
                            if seen_max_expansions {
                                return Err(serde::de::Error::duplicate_field("max_expansions"));
                            }
                            seen_max_expansions = true;
                            f_max_expansions = map.next_value::<Option<i32>>()?;
                        }
                        "query" => {
                            if f_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            f_query = Some(map.next_value::<String>()?);
                        }
                        "slop" => {
                            if seen_slop {
                                return Err(serde::de::Error::duplicate_field("slop"));
                            }
                            seen_slop = true;
                            f_slop = map.next_value::<Option<i32>>()?;
                        }
                        "zero_terms_query" => {
                            if seen_zero_terms_query {
                                return Err(serde::de::Error::duplicate_field("zero_terms_query"));
                            }
                            seen_zero_terms_query = true;
                            f_zero_terms_query = map.next_value::<Option<crate::types::common_query_dsl::ZeroTermsQuery>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MatchPhrasePrefixQueryObject {
                    name: f_name,
                    boost: f_boost,
                    analyzer: f_analyzer,
                    max_expansions: f_max_expansions,
                    query: f_query.ok_or_else(|| serde::de::Error::missing_field("query"))?,
                    slop: f_slop,
                    zero_terms_query: f_zero_terms_query,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MatchPhrasePrefixQueryObject {
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
        if let Some(v) = &self.analyzer {
            map.serialize_entry("analyzer", v)?;
        }
        if let Some(v) = &self.max_expansions {
            map.serialize_entry("max_expansions", v)?;
        }
        map.serialize_entry("query", &self.query)?;
        if let Some(v) = &self.slop {
            map.serialize_entry("slop", v)?;
        }
        if let Some(v) = &self.zero_terms_query {
            map.serialize_entry("zero_terms_query", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'"') => serde_json::from_str(text).map(MatchPhraseQuery::Query),
            Some(b'{') => serde_json::from_str(text).map(MatchPhraseQuery::Object),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a MatchPhraseQuery value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct MatchPhraseQueryObject {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "Analyzer used to convert the text in the query value into tokens."]
    pub analyzer: Option<String>,
    #[doc = "Query terms that are analyzed and turned into a phrase query."]
    pub query: String,
    #[doc = "Maximum number of positions allowed between matching tokens."]
    pub slop: Option<i32>,
    pub zero_terms_query: Option<crate::types::common_query_dsl::ZeroTermsQuery>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MatchPhraseQueryObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MatchPhraseQueryObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MatchPhraseQueryObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_analyzer: Option<String> = None;
                let mut seen_analyzer = false;
                let mut f_query: Option<String> = None;
                let mut f_slop: Option<i32> = None;
                let mut seen_slop = false;
                let mut f_zero_terms_query: Option<crate::types::common_query_dsl::ZeroTermsQuery> =
                    None;
                let mut seen_zero_terms_query = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "analyzer" => {
                            if seen_analyzer {
                                return Err(serde::de::Error::duplicate_field("analyzer"));
                            }
                            seen_analyzer = true;
                            f_analyzer = map.next_value::<Option<String>>()?;
                        }
                        "query" => {
                            if f_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            f_query = Some(map.next_value::<String>()?);
                        }
                        "slop" => {
                            if seen_slop {
                                return Err(serde::de::Error::duplicate_field("slop"));
                            }
                            seen_slop = true;
                            f_slop = map.next_value::<Option<i32>>()?;
                        }
                        "zero_terms_query" => {
                            if seen_zero_terms_query {
                                return Err(serde::de::Error::duplicate_field("zero_terms_query"));
                            }
                            seen_zero_terms_query = true;
                            f_zero_terms_query = map.next_value::<Option<crate::types::common_query_dsl::ZeroTermsQuery>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MatchPhraseQueryObject {
                    name: f_name,
                    boost: f_boost,
                    analyzer: f_analyzer,
                    query: f_query.ok_or_else(|| serde::de::Error::missing_field("query"))?,
                    slop: f_slop,
                    zero_terms_query: f_zero_terms_query,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MatchPhraseQueryObject {
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
        if let Some(v) = &self.analyzer {
            map.serialize_entry("analyzer", v)?;
        }
        map.serialize_entry("query", &self.query)?;
        if let Some(v) = &self.slop {
            map.serialize_entry("slop", v)?;
        }
        if let Some(v) = &self.zero_terms_query {
            map.serialize_entry("zero_terms_query", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct MatchQueryObject {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "Analyzer used to convert the text in the query value into tokens."]
    pub analyzer: Option<String>,
    #[doc = "If `true`, match phrase queries are automatically created for multi-term synonyms."]
    pub auto_generate_synonyms_phrase_query: Option<bool>,
    pub cutoff_frequency: Option<f64>,
    pub fuzziness: Option<crate::types::common::Fuzziness>,
    pub fuzzy_rewrite: Option<crate::types::common::MultiTermQueryRewrite>,
    #[doc = "If `true`, edits for fuzzy matching include transpositions of two adjacent characters (for example, `ab` to `ba`)."]
    pub fuzzy_transpositions: Option<bool>,
    #[doc = "If `true`, format-based errors, such as providing a text query value for a numeric field, are ignored."]
    pub lenient: Option<bool>,
    #[doc = "Maximum number of terms to which the query will expand."]
    pub max_expansions: Option<i32>,
    pub minimum_should_match: Option<crate::types::common::MinimumShouldMatch>,
    pub operator: Option<crate::types::common_query_dsl::Operator>,
    #[doc = "Number of beginning characters left unchanged for fuzzy matching."]
    pub prefix_length: Option<i32>,
    #[doc = "Text, number, Boolean value or date you wish to find in the provided field."]
    pub query: crate::types::common::FieldValue,
    pub zero_terms_query: Option<crate::types::common_query_dsl::ZeroTermsQuery>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MatchQueryObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MatchQueryObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MatchQueryObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_analyzer: Option<String> = None;
                let mut seen_analyzer = false;
                let mut f_auto_generate_synonyms_phrase_query: Option<bool> = None;
                let mut seen_auto_generate_synonyms_phrase_query = false;
                let mut f_cutoff_frequency: Option<f64> = None;
                let mut seen_cutoff_frequency = false;
                let mut f_fuzziness: Option<crate::types::common::Fuzziness> = None;
                let mut seen_fuzziness = false;
                let mut f_fuzzy_rewrite: Option<crate::types::common::MultiTermQueryRewrite> = None;
                let mut seen_fuzzy_rewrite = false;
                let mut f_fuzzy_transpositions: Option<bool> = None;
                let mut seen_fuzzy_transpositions = false;
                let mut f_lenient: Option<bool> = None;
                let mut seen_lenient = false;
                let mut f_max_expansions: Option<i32> = None;
                let mut seen_max_expansions = false;
                let mut f_minimum_should_match: Option<crate::types::common::MinimumShouldMatch> =
                    None;
                let mut seen_minimum_should_match = false;
                let mut f_operator: Option<crate::types::common_query_dsl::Operator> = None;
                let mut seen_operator = false;
                let mut f_prefix_length: Option<i32> = None;
                let mut seen_prefix_length = false;
                let mut f_query: Option<crate::types::common::FieldValue> = None;
                let mut f_zero_terms_query: Option<crate::types::common_query_dsl::ZeroTermsQuery> =
                    None;
                let mut seen_zero_terms_query = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "analyzer" => {
                            if seen_analyzer {
                                return Err(serde::de::Error::duplicate_field("analyzer"));
                            }
                            seen_analyzer = true;
                            f_analyzer = map.next_value::<Option<String>>()?;
                        }
                        "auto_generate_synonyms_phrase_query" => {
                            if seen_auto_generate_synonyms_phrase_query {
                                return Err(serde::de::Error::duplicate_field(
                                    "auto_generate_synonyms_phrase_query",
                                ));
                            }
                            seen_auto_generate_synonyms_phrase_query = true;
                            f_auto_generate_synonyms_phrase_query =
                                map.next_value::<Option<bool>>()?;
                        }
                        "cutoff_frequency" => {
                            if seen_cutoff_frequency {
                                return Err(serde::de::Error::duplicate_field("cutoff_frequency"));
                            }
                            seen_cutoff_frequency = true;
                            f_cutoff_frequency = map.next_value::<Option<f64>>()?;
                        }
                        "fuzziness" => {
                            if seen_fuzziness {
                                return Err(serde::de::Error::duplicate_field("fuzziness"));
                            }
                            seen_fuzziness = true;
                            f_fuzziness =
                                map.next_value::<Option<crate::types::common::Fuzziness>>()?;
                        }
                        "fuzzy_rewrite" => {
                            if seen_fuzzy_rewrite {
                                return Err(serde::de::Error::duplicate_field("fuzzy_rewrite"));
                            }
                            seen_fuzzy_rewrite = true;
                            f_fuzzy_rewrite = map
                                .next_value::<Option<crate::types::common::MultiTermQueryRewrite>>(
                                )?;
                        }
                        "fuzzy_transpositions" => {
                            if seen_fuzzy_transpositions {
                                return Err(serde::de::Error::duplicate_field(
                                    "fuzzy_transpositions",
                                ));
                            }
                            seen_fuzzy_transpositions = true;
                            f_fuzzy_transpositions = map.next_value::<Option<bool>>()?;
                        }
                        "lenient" => {
                            if seen_lenient {
                                return Err(serde::de::Error::duplicate_field("lenient"));
                            }
                            seen_lenient = true;
                            f_lenient = map.next_value::<Option<bool>>()?;
                        }
                        "max_expansions" => {
                            if seen_max_expansions {
                                return Err(serde::de::Error::duplicate_field("max_expansions"));
                            }
                            seen_max_expansions = true;
                            f_max_expansions = map.next_value::<Option<i32>>()?;
                        }
                        "minimum_should_match" => {
                            if seen_minimum_should_match {
                                return Err(serde::de::Error::duplicate_field(
                                    "minimum_should_match",
                                ));
                            }
                            seen_minimum_should_match = true;
                            f_minimum_should_match = map
                                .next_value::<Option<crate::types::common::MinimumShouldMatch>>()?;
                        }
                        "operator" => {
                            if seen_operator {
                                return Err(serde::de::Error::duplicate_field("operator"));
                            }
                            seen_operator = true;
                            f_operator = map
                                .next_value::<Option<crate::types::common_query_dsl::Operator>>()?;
                        }
                        "prefix_length" => {
                            if seen_prefix_length {
                                return Err(serde::de::Error::duplicate_field("prefix_length"));
                            }
                            seen_prefix_length = true;
                            f_prefix_length = map.next_value::<Option<i32>>()?;
                        }
                        "query" => {
                            if f_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            f_query = Some(map.next_value::<crate::types::common::FieldValue>()?);
                        }
                        "zero_terms_query" => {
                            if seen_zero_terms_query {
                                return Err(serde::de::Error::duplicate_field("zero_terms_query"));
                            }
                            seen_zero_terms_query = true;
                            f_zero_terms_query = map.next_value::<Option<crate::types::common_query_dsl::ZeroTermsQuery>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MatchQueryObject {
                    name: f_name,
                    boost: f_boost,
                    analyzer: f_analyzer,
                    auto_generate_synonyms_phrase_query: f_auto_generate_synonyms_phrase_query,
                    cutoff_frequency: f_cutoff_frequency,
                    fuzziness: f_fuzziness,
                    fuzzy_rewrite: f_fuzzy_rewrite,
                    fuzzy_transpositions: f_fuzzy_transpositions,
                    lenient: f_lenient,
                    max_expansions: f_max_expansions,
                    minimum_should_match: f_minimum_should_match,
                    operator: f_operator,
                    prefix_length: f_prefix_length,
                    query: f_query.ok_or_else(|| serde::de::Error::missing_field("query"))?,
                    zero_terms_query: f_zero_terms_query,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MatchQueryObject {
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
        if let Some(v) = &self.analyzer {
            map.serialize_entry("analyzer", v)?;
        }
        if let Some(v) = &self.auto_generate_synonyms_phrase_query {
            map.serialize_entry("auto_generate_synonyms_phrase_query", v)?;
        }
        if let Some(v) = &self.cutoff_frequency {
            map.serialize_entry("cutoff_frequency", v)?;
        }
        if let Some(v) = &self.fuzziness {
            map.serialize_entry("fuzziness", v)?;
        }
        if let Some(v) = &self.fuzzy_rewrite {
            map.serialize_entry("fuzzy_rewrite", v)?;
        }
        if let Some(v) = &self.fuzzy_transpositions {
            map.serialize_entry("fuzzy_transpositions", v)?;
        }
        if let Some(v) = &self.lenient {
            map.serialize_entry("lenient", v)?;
        }
        if let Some(v) = &self.max_expansions {
            map.serialize_entry("max_expansions", v)?;
        }
        if let Some(v) = &self.minimum_should_match {
            map.serialize_entry("minimum_should_match", v)?;
        }
        if let Some(v) = &self.operator {
            map.serialize_entry("operator", v)?;
        }
        if let Some(v) = &self.prefix_length {
            map.serialize_entry("prefix_length", v)?;
        }
        map.serialize_entry("query", &self.query)?;
        if let Some(v) = &self.zero_terms_query {
            map.serialize_entry("zero_terms_query", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct MoreLikeThisQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "The analyzer that is used to analyze the free form text. Defaults to the analyzer associated with the first field in fields."]
    pub analyzer: Option<String>,
    #[doc = "Each term in the formed query could be further boosted by their tf-idf score. This sets the boost factor to use when using this feature. Defaults to deactivated (0)."]
    pub boost_terms: Option<f64>,
    #[doc = "Controls whether the query should fail (throw an exception) if any of the specified fields are not of the supported types (`text` or `keyword`)."]
    pub fail_on_unsupported_field: Option<bool>,
    #[doc = "A list of fields to fetch and analyze the text from. Defaults to the `index.query.default_field` index setting, which has a default value of `*`."]
    pub fields: Option<Vec<crate::types::common::Field>>,
    #[doc = "Specifies whether the input documents should also be included in the search results returned."]
    pub include: Option<bool>,
    #[doc = "Specifies free form text and/or a single or multiple documents for which you want to find similar documents."]
    pub like: crate::types::common_query_dsl::MoreLikeThisQueryLike,
    #[doc = "The maximum document frequency above which the terms are ignored from the input document."]
    pub max_doc_freq: Option<i32>,
    #[doc = "The maximum number of query terms that can be selected."]
    pub max_query_terms: Option<i32>,
    #[doc = "The maximum word length above which the terms are ignored. Defaults to unbounded (`0`)."]
    pub max_word_length: Option<i32>,
    #[doc = "The minimum document frequency below which the terms are ignored from the input document."]
    pub min_doc_freq: Option<i32>,
    #[doc = "The minimum term frequency below which the terms are ignored from the input document."]
    pub min_term_freq: Option<i32>,
    #[doc = "The minimum word length below which the terms are ignored."]
    pub min_word_length: Option<i32>,
    pub minimum_should_match: Option<crate::types::common::MinimumShouldMatch>,
    #[doc = "Overrides the default analyzer."]
    pub per_field_analyzer: Option<std::collections::BTreeMap<String, String>>,
    pub routing: Option<crate::types::common::Routing>,
    pub stop_words: Option<crate::types::common_analysis::StopWords>,
    #[doc = "Used in combination with `like` to exclude documents that match a set of terms."]
    pub unlike: Option<crate::types::common_query_dsl::MoreLikeThisQueryUnlike>,
    pub version: Option<crate::types::common::VersionNumber>,
    pub version_type: Option<crate::types::common::VersionType>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MoreLikeThisQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MoreLikeThisQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MoreLikeThisQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_analyzer: Option<String> = None;
                let mut seen_analyzer = false;
                let mut f_boost_terms: Option<f64> = None;
                let mut seen_boost_terms = false;
                let mut f_fail_on_unsupported_field: Option<bool> = None;
                let mut seen_fail_on_unsupported_field = false;
                let mut f_fields: Option<Vec<crate::types::common::Field>> = None;
                let mut seen_fields = false;
                let mut f_include: Option<bool> = None;
                let mut seen_include = false;
                let mut f_like: Option<crate::types::common_query_dsl::MoreLikeThisQueryLike> =
                    None;
                let mut f_max_doc_freq: Option<i32> = None;
                let mut seen_max_doc_freq = false;
                let mut f_max_query_terms: Option<i32> = None;
                let mut seen_max_query_terms = false;
                let mut f_max_word_length: Option<i32> = None;
                let mut seen_max_word_length = false;
                let mut f_min_doc_freq: Option<i32> = None;
                let mut seen_min_doc_freq = false;
                let mut f_min_term_freq: Option<i32> = None;
                let mut seen_min_term_freq = false;
                let mut f_min_word_length: Option<i32> = None;
                let mut seen_min_word_length = false;
                let mut f_minimum_should_match: Option<crate::types::common::MinimumShouldMatch> =
                    None;
                let mut seen_minimum_should_match = false;
                let mut f_per_field_analyzer: Option<std::collections::BTreeMap<String, String>> =
                    None;
                let mut seen_per_field_analyzer = false;
                let mut f_routing: Option<crate::types::common::Routing> = None;
                let mut seen_routing = false;
                let mut f_stop_words: Option<crate::types::common_analysis::StopWords> = None;
                let mut seen_stop_words = false;
                let mut f_unlike: Option<crate::types::common_query_dsl::MoreLikeThisQueryUnlike> =
                    None;
                let mut seen_unlike = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version = false;
                let mut f_version_type: Option<crate::types::common::VersionType> = None;
                let mut seen_version_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "analyzer" => {
                            if seen_analyzer {
                                return Err(serde::de::Error::duplicate_field("analyzer"));
                            }
                            seen_analyzer = true;
                            f_analyzer = map.next_value::<Option<String>>()?;
                        }
                        "boost_terms" => {
                            if seen_boost_terms {
                                return Err(serde::de::Error::duplicate_field("boost_terms"));
                            }
                            seen_boost_terms = true;
                            f_boost_terms = map.next_value::<Option<f64>>()?;
                        }
                        "fail_on_unsupported_field" => {
                            if seen_fail_on_unsupported_field {
                                return Err(serde::de::Error::duplicate_field(
                                    "fail_on_unsupported_field",
                                ));
                            }
                            seen_fail_on_unsupported_field = true;
                            f_fail_on_unsupported_field = map.next_value::<Option<bool>>()?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields =
                                map.next_value::<Option<Vec<crate::types::common::Field>>>()?;
                        }
                        "include" => {
                            if seen_include {
                                return Err(serde::de::Error::duplicate_field("include"));
                            }
                            seen_include = true;
                            f_include = map.next_value::<Option<bool>>()?;
                        }
                        "like" => {
                            if f_like.is_some() {
                                return Err(serde::de::Error::duplicate_field("like"));
                            }
                            f_like = Some(map.next_value::<crate::types::common_query_dsl::MoreLikeThisQueryLike>()?);
                        }
                        "max_doc_freq" => {
                            if seen_max_doc_freq {
                                return Err(serde::de::Error::duplicate_field("max_doc_freq"));
                            }
                            seen_max_doc_freq = true;
                            f_max_doc_freq = map.next_value::<Option<i32>>()?;
                        }
                        "max_query_terms" => {
                            if seen_max_query_terms {
                                return Err(serde::de::Error::duplicate_field("max_query_terms"));
                            }
                            seen_max_query_terms = true;
                            f_max_query_terms = map.next_value::<Option<i32>>()?;
                        }
                        "max_word_length" => {
                            if seen_max_word_length {
                                return Err(serde::de::Error::duplicate_field("max_word_length"));
                            }
                            seen_max_word_length = true;
                            f_max_word_length = map.next_value::<Option<i32>>()?;
                        }
                        "min_doc_freq" => {
                            if seen_min_doc_freq {
                                return Err(serde::de::Error::duplicate_field("min_doc_freq"));
                            }
                            seen_min_doc_freq = true;
                            f_min_doc_freq = map.next_value::<Option<i32>>()?;
                        }
                        "min_term_freq" => {
                            if seen_min_term_freq {
                                return Err(serde::de::Error::duplicate_field("min_term_freq"));
                            }
                            seen_min_term_freq = true;
                            f_min_term_freq = map.next_value::<Option<i32>>()?;
                        }
                        "min_word_length" => {
                            if seen_min_word_length {
                                return Err(serde::de::Error::duplicate_field("min_word_length"));
                            }
                            seen_min_word_length = true;
                            f_min_word_length = map.next_value::<Option<i32>>()?;
                        }
                        "minimum_should_match" => {
                            if seen_minimum_should_match {
                                return Err(serde::de::Error::duplicate_field(
                                    "minimum_should_match",
                                ));
                            }
                            seen_minimum_should_match = true;
                            f_minimum_should_match = map
                                .next_value::<Option<crate::types::common::MinimumShouldMatch>>()?;
                        }
                        "per_field_analyzer" => {
                            if seen_per_field_analyzer {
                                return Err(serde::de::Error::duplicate_field(
                                    "per_field_analyzer",
                                ));
                            }
                            seen_per_field_analyzer = true;
                            f_per_field_analyzer = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("routing"));
                            }
                            seen_routing = true;
                            f_routing =
                                map.next_value::<Option<crate::types::common::Routing>>()?;
                        }
                        "stop_words" => {
                            if seen_stop_words {
                                return Err(serde::de::Error::duplicate_field("stop_words"));
                            }
                            seen_stop_words = true;
                            f_stop_words = map
                                .next_value::<Option<crate::types::common_analysis::StopWords>>()?;
                        }
                        "unlike" => {
                            if seen_unlike {
                                return Err(serde::de::Error::duplicate_field("unlike"));
                            }
                            seen_unlike = true;
                            f_unlike = map.next_value::<Option<crate::types::common_query_dsl::MoreLikeThisQueryUnlike>>()?;
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionNumber>>()?;
                        }
                        "version_type" => {
                            if seen_version_type {
                                return Err(serde::de::Error::duplicate_field("version_type"));
                            }
                            seen_version_type = true;
                            f_version_type =
                                map.next_value::<Option<crate::types::common::VersionType>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MoreLikeThisQuery {
                    name: f_name,
                    boost: f_boost,
                    analyzer: f_analyzer,
                    boost_terms: f_boost_terms,
                    fail_on_unsupported_field: f_fail_on_unsupported_field,
                    fields: f_fields,
                    include: f_include,
                    like: f_like.ok_or_else(|| serde::de::Error::missing_field("like"))?,
                    max_doc_freq: f_max_doc_freq,
                    max_query_terms: f_max_query_terms,
                    max_word_length: f_max_word_length,
                    min_doc_freq: f_min_doc_freq,
                    min_term_freq: f_min_term_freq,
                    min_word_length: f_min_word_length,
                    minimum_should_match: f_minimum_should_match,
                    per_field_analyzer: f_per_field_analyzer,
                    routing: f_routing,
                    stop_words: f_stop_words,
                    unlike: f_unlike,
                    version: f_version,
                    version_type: f_version_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MoreLikeThisQuery {
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
        if let Some(v) = &self.analyzer {
            map.serialize_entry("analyzer", v)?;
        }
        if let Some(v) = &self.boost_terms {
            map.serialize_entry("boost_terms", v)?;
        }
        if let Some(v) = &self.fail_on_unsupported_field {
            map.serialize_entry("fail_on_unsupported_field", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.include {
            map.serialize_entry("include", v)?;
        }
        map.serialize_entry("like", &self.like)?;
        if let Some(v) = &self.max_doc_freq {
            map.serialize_entry("max_doc_freq", v)?;
        }
        if let Some(v) = &self.max_query_terms {
            map.serialize_entry("max_query_terms", v)?;
        }
        if let Some(v) = &self.max_word_length {
            map.serialize_entry("max_word_length", v)?;
        }
        if let Some(v) = &self.min_doc_freq {
            map.serialize_entry("min_doc_freq", v)?;
        }
        if let Some(v) = &self.min_term_freq {
            map.serialize_entry("min_term_freq", v)?;
        }
        if let Some(v) = &self.min_word_length {
            map.serialize_entry("min_word_length", v)?;
        }
        if let Some(v) = &self.minimum_should_match {
            map.serialize_entry("minimum_should_match", v)?;
        }
        if let Some(v) = &self.per_field_analyzer {
            map.serialize_entry("per_field_analyzer", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("routing", v)?;
        }
        if let Some(v) = &self.stop_words {
            map.serialize_entry("stop_words", v)?;
        }
        if let Some(v) = &self.unlike {
            map.serialize_entry("unlike", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.version_type {
            map.serialize_entry("version_type", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct MultiMatchQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "Analyzer used to convert the text in the query value into tokens."]
    pub analyzer: Option<String>,
    #[doc = "If `true`, match phrase queries are automatically created for multi-term synonyms."]
    pub auto_generate_synonyms_phrase_query: Option<bool>,
    pub cutoff_frequency: Option<f64>,
    pub fields: Option<crate::types::common::Fields>,
    pub fuzziness: Option<crate::types::common::Fuzziness>,
    pub fuzzy_rewrite: Option<crate::types::common::MultiTermQueryRewrite>,
    #[doc = "If `true`, edits for fuzzy matching include transpositions of two adjacent characters (for example, `ab` to `ba`). Can be applied to the term subqueries constructed for all terms but the final term."]
    pub fuzzy_transpositions: Option<bool>,
    #[doc = "If `true`, format-based errors, such as providing a text query value for a numeric field, are ignored."]
    pub lenient: Option<bool>,
    #[doc = "Maximum number of terms to which the query will expand."]
    pub max_expansions: Option<i32>,
    pub minimum_should_match: Option<crate::types::common::MinimumShouldMatch>,
    pub operator: Option<crate::types::common_query_dsl::Operator>,
    #[doc = "Number of beginning characters left unchanged for fuzzy matching."]
    pub prefix_length: Option<i32>,
    #[doc = "Text, number, Boolean value or date you wish to find in the provided field."]
    pub query: String,
    #[doc = "Maximum number of positions allowed between matching tokens."]
    pub slop: Option<i32>,
    #[doc = "Determines how scores for each per-term blended query and scores across groups are combined."]
    pub tie_breaker: Option<f64>,
    pub r#type: Option<crate::types::common_query_dsl::TextQueryType>,
    pub zero_terms_query: Option<crate::types::common_query_dsl::ZeroTermsQuery>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MultiMatchQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MultiMatchQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MultiMatchQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_analyzer: Option<String> = None;
                let mut seen_analyzer = false;
                let mut f_auto_generate_synonyms_phrase_query: Option<bool> = None;
                let mut seen_auto_generate_synonyms_phrase_query = false;
                let mut f_cutoff_frequency: Option<f64> = None;
                let mut seen_cutoff_frequency = false;
                let mut f_fields: Option<crate::types::common::Fields> = None;
                let mut seen_fields = false;
                let mut f_fuzziness: Option<crate::types::common::Fuzziness> = None;
                let mut seen_fuzziness = false;
                let mut f_fuzzy_rewrite: Option<crate::types::common::MultiTermQueryRewrite> = None;
                let mut seen_fuzzy_rewrite = false;
                let mut f_fuzzy_transpositions: Option<bool> = None;
                let mut seen_fuzzy_transpositions = false;
                let mut f_lenient: Option<bool> = None;
                let mut seen_lenient = false;
                let mut f_max_expansions: Option<i32> = None;
                let mut seen_max_expansions = false;
                let mut f_minimum_should_match: Option<crate::types::common::MinimumShouldMatch> =
                    None;
                let mut seen_minimum_should_match = false;
                let mut f_operator: Option<crate::types::common_query_dsl::Operator> = None;
                let mut seen_operator = false;
                let mut f_prefix_length: Option<i32> = None;
                let mut seen_prefix_length = false;
                let mut f_query: Option<String> = None;
                let mut f_slop: Option<i32> = None;
                let mut seen_slop = false;
                let mut f_tie_breaker: Option<f64> = None;
                let mut seen_tie_breaker = false;
                let mut f_type: Option<crate::types::common_query_dsl::TextQueryType> = None;
                let mut seen_type = false;
                let mut f_zero_terms_query: Option<crate::types::common_query_dsl::ZeroTermsQuery> =
                    None;
                let mut seen_zero_terms_query = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "analyzer" => {
                            if seen_analyzer {
                                return Err(serde::de::Error::duplicate_field("analyzer"));
                            }
                            seen_analyzer = true;
                            f_analyzer = map.next_value::<Option<String>>()?;
                        }
                        "auto_generate_synonyms_phrase_query" => {
                            if seen_auto_generate_synonyms_phrase_query {
                                return Err(serde::de::Error::duplicate_field(
                                    "auto_generate_synonyms_phrase_query",
                                ));
                            }
                            seen_auto_generate_synonyms_phrase_query = true;
                            f_auto_generate_synonyms_phrase_query =
                                map.next_value::<Option<bool>>()?;
                        }
                        "cutoff_frequency" => {
                            if seen_cutoff_frequency {
                                return Err(serde::de::Error::duplicate_field("cutoff_frequency"));
                            }
                            seen_cutoff_frequency = true;
                            f_cutoff_frequency = map.next_value::<Option<f64>>()?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "fuzziness" => {
                            if seen_fuzziness {
                                return Err(serde::de::Error::duplicate_field("fuzziness"));
                            }
                            seen_fuzziness = true;
                            f_fuzziness =
                                map.next_value::<Option<crate::types::common::Fuzziness>>()?;
                        }
                        "fuzzy_rewrite" => {
                            if seen_fuzzy_rewrite {
                                return Err(serde::de::Error::duplicate_field("fuzzy_rewrite"));
                            }
                            seen_fuzzy_rewrite = true;
                            f_fuzzy_rewrite = map
                                .next_value::<Option<crate::types::common::MultiTermQueryRewrite>>(
                                )?;
                        }
                        "fuzzy_transpositions" => {
                            if seen_fuzzy_transpositions {
                                return Err(serde::de::Error::duplicate_field(
                                    "fuzzy_transpositions",
                                ));
                            }
                            seen_fuzzy_transpositions = true;
                            f_fuzzy_transpositions = map.next_value::<Option<bool>>()?;
                        }
                        "lenient" => {
                            if seen_lenient {
                                return Err(serde::de::Error::duplicate_field("lenient"));
                            }
                            seen_lenient = true;
                            f_lenient = map.next_value::<Option<bool>>()?;
                        }
                        "max_expansions" => {
                            if seen_max_expansions {
                                return Err(serde::de::Error::duplicate_field("max_expansions"));
                            }
                            seen_max_expansions = true;
                            f_max_expansions = map.next_value::<Option<i32>>()?;
                        }
                        "minimum_should_match" => {
                            if seen_minimum_should_match {
                                return Err(serde::de::Error::duplicate_field(
                                    "minimum_should_match",
                                ));
                            }
                            seen_minimum_should_match = true;
                            f_minimum_should_match = map
                                .next_value::<Option<crate::types::common::MinimumShouldMatch>>()?;
                        }
                        "operator" => {
                            if seen_operator {
                                return Err(serde::de::Error::duplicate_field("operator"));
                            }
                            seen_operator = true;
                            f_operator = map
                                .next_value::<Option<crate::types::common_query_dsl::Operator>>()?;
                        }
                        "prefix_length" => {
                            if seen_prefix_length {
                                return Err(serde::de::Error::duplicate_field("prefix_length"));
                            }
                            seen_prefix_length = true;
                            f_prefix_length = map.next_value::<Option<i32>>()?;
                        }
                        "query" => {
                            if f_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            f_query = Some(map.next_value::<String>()?);
                        }
                        "slop" => {
                            if seen_slop {
                                return Err(serde::de::Error::duplicate_field("slop"));
                            }
                            seen_slop = true;
                            f_slop = map.next_value::<Option<i32>>()?;
                        }
                        "tie_breaker" => {
                            if seen_tie_breaker {
                                return Err(serde::de::Error::duplicate_field("tie_breaker"));
                            }
                            seen_tie_breaker = true;
                            f_tie_breaker = map.next_value::<Option<f64>>()?;
                        }
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<crate::types::common_query_dsl::TextQueryType>>()?;
                        }
                        "zero_terms_query" => {
                            if seen_zero_terms_query {
                                return Err(serde::de::Error::duplicate_field("zero_terms_query"));
                            }
                            seen_zero_terms_query = true;
                            f_zero_terms_query = map.next_value::<Option<crate::types::common_query_dsl::ZeroTermsQuery>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MultiMatchQuery {
                    name: f_name,
                    boost: f_boost,
                    analyzer: f_analyzer,
                    auto_generate_synonyms_phrase_query: f_auto_generate_synonyms_phrase_query,
                    cutoff_frequency: f_cutoff_frequency,
                    fields: f_fields,
                    fuzziness: f_fuzziness,
                    fuzzy_rewrite: f_fuzzy_rewrite,
                    fuzzy_transpositions: f_fuzzy_transpositions,
                    lenient: f_lenient,
                    max_expansions: f_max_expansions,
                    minimum_should_match: f_minimum_should_match,
                    operator: f_operator,
                    prefix_length: f_prefix_length,
                    query: f_query.ok_or_else(|| serde::de::Error::missing_field("query"))?,
                    slop: f_slop,
                    tie_breaker: f_tie_breaker,
                    r#type: f_type,
                    zero_terms_query: f_zero_terms_query,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MultiMatchQuery {
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
        if let Some(v) = &self.analyzer {
            map.serialize_entry("analyzer", v)?;
        }
        if let Some(v) = &self.auto_generate_synonyms_phrase_query {
            map.serialize_entry("auto_generate_synonyms_phrase_query", v)?;
        }
        if let Some(v) = &self.cutoff_frequency {
            map.serialize_entry("cutoff_frequency", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.fuzziness {
            map.serialize_entry("fuzziness", v)?;
        }
        if let Some(v) = &self.fuzzy_rewrite {
            map.serialize_entry("fuzzy_rewrite", v)?;
        }
        if let Some(v) = &self.fuzzy_transpositions {
            map.serialize_entry("fuzzy_transpositions", v)?;
        }
        if let Some(v) = &self.lenient {
            map.serialize_entry("lenient", v)?;
        }
        if let Some(v) = &self.max_expansions {
            map.serialize_entry("max_expansions", v)?;
        }
        if let Some(v) = &self.minimum_should_match {
            map.serialize_entry("minimum_should_match", v)?;
        }
        if let Some(v) = &self.operator {
            map.serialize_entry("operator", v)?;
        }
        if let Some(v) = &self.prefix_length {
            map.serialize_entry("prefix_length", v)?;
        }
        map.serialize_entry("query", &self.query)?;
        if let Some(v) = &self.slop {
            map.serialize_entry("slop", v)?;
        }
        if let Some(v) = &self.tie_breaker {
            map.serialize_entry("tie_breaker", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("type", v)?;
        }
        if let Some(v) = &self.zero_terms_query {
            map.serialize_entry("zero_terms_query", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MultiValueMode {
    #[doc = "`avg`"]
    Avg,
    #[doc = "`max`"]
    Max,
    #[doc = "`min`"]
    Min,
    #[doc = "`sum`"]
    Sum,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl MultiValueMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            MultiValueMode::Avg => "avg",
            MultiValueMode::Max => "max",
            MultiValueMode::Min => "min",
            MultiValueMode::Sum => "sum",
            MultiValueMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for MultiValueMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MultiValueMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "avg" => MultiValueMode::Avg,
            "max" => MultiValueMode::Max,
            "min" => MultiValueMode::Min,
            "sum" => MultiValueMode::Sum,
            _ => MultiValueMode::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct NestedQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "Indicates whether to ignore an unmapped path and not return any documents instead of an error."]
    pub ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped>,
    pub inner_hits: Option<Box<crate::types::search::InnerHits>>,
    pub path: crate::types::common::Field,
    pub query: Box<crate::types::common_query_dsl::QueryContainer>,
    pub score_mode: Option<crate::types::common_query_dsl::ChildScoreMode>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NestedQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NestedQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NestedQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped> =
                    None;
                let mut seen_ignore_unmapped = false;
                let mut f_inner_hits: Option<Box<crate::types::search::InnerHits>> = None;
                let mut seen_inner_hits = false;
                let mut f_path: Option<crate::types::common::Field> = None;
                let mut f_query: Option<Box<crate::types::common_query_dsl::QueryContainer>> = None;
                let mut f_score_mode: Option<crate::types::common_query_dsl::ChildScoreMode> = None;
                let mut seen_score_mode = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "ignore_unmapped" => {
                            if seen_ignore_unmapped {
                                return Err(serde::de::Error::duplicate_field("ignore_unmapped"));
                            }
                            seen_ignore_unmapped = true;
                            f_ignore_unmapped = map.next_value::<Option<crate::types::common_query_dsl::IgnoreUnmapped>>()?;
                        }
                        "inner_hits" => {
                            if seen_inner_hits {
                                return Err(serde::de::Error::duplicate_field("inner_hits"));
                            }
                            seen_inner_hits = true;
                            f_inner_hits =
                                map.next_value::<Option<Box<crate::types::search::InnerHits>>>()?;
                        }
                        "path" => {
                            if f_path.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            f_path = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "query" => {
                            if f_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            f_query = Some(map.next_value::<Box<crate::types::common_query_dsl::QueryContainer>>()?);
                        }
                        "score_mode" => {
                            if seen_score_mode {
                                return Err(serde::de::Error::duplicate_field("score_mode"));
                            }
                            seen_score_mode = true;
                            f_score_mode = map.next_value::<Option<crate::types::common_query_dsl::ChildScoreMode>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NestedQuery {
                    name: f_name,
                    boost: f_boost,
                    ignore_unmapped: f_ignore_unmapped,
                    inner_hits: f_inner_hits,
                    path: f_path.ok_or_else(|| serde::de::Error::missing_field("path"))?,
                    query: f_query.ok_or_else(|| serde::de::Error::missing_field("query"))?,
                    score_mode: f_score_mode,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NestedQuery {
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
        if let Some(v) = &self.inner_hits {
            map.serialize_entry("inner_hits", v)?;
        }
        map.serialize_entry("path", &self.path)?;
        map.serialize_entry("query", &self.query)?;
        if let Some(v) = &self.score_mode {
            map.serialize_entry("score_mode", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NeuralQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub filter: Option<crate::types::common_query_dsl::QueryContainer>,
    pub k: Option<i64>,
    pub max_distance: Option<f64>,
    pub min_score: Option<f64>,
    pub model_id: Option<String>,
    pub query_image: Option<String>,
    pub query_text: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NeuralQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NeuralQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NeuralQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_filter: Option<crate::types::common_query_dsl::QueryContainer> = None;
                let mut seen_filter = false;
                let mut f_k: Option<i64> = None;
                let mut seen_k = false;
                let mut f_max_distance: Option<f64> = None;
                let mut seen_max_distance = false;
                let mut f_min_score: Option<f64> = None;
                let mut seen_min_score = false;
                let mut f_model_id: Option<String> = None;
                let mut seen_model_id = false;
                let mut f_query_image: Option<String> = None;
                let mut seen_query_image = false;
                let mut f_query_text: Option<String> = None;
                let mut seen_query_text = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "filter" => {
                            if seen_filter {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            seen_filter = true;
                            f_filter = map.next_value::<Option<crate::types::common_query_dsl::QueryContainer>>()?;
                        }
                        "k" => {
                            if seen_k {
                                return Err(serde::de::Error::duplicate_field("k"));
                            }
                            seen_k = true;
                            f_k = map.next_value::<Option<i64>>()?;
                        }
                        "max_distance" => {
                            if seen_max_distance {
                                return Err(serde::de::Error::duplicate_field("max_distance"));
                            }
                            seen_max_distance = true;
                            f_max_distance = map.next_value::<Option<f64>>()?;
                        }
                        "min_score" => {
                            if seen_min_score {
                                return Err(serde::de::Error::duplicate_field("min_score"));
                            }
                            seen_min_score = true;
                            f_min_score = map.next_value::<Option<f64>>()?;
                        }
                        "model_id" => {
                            if seen_model_id {
                                return Err(serde::de::Error::duplicate_field("model_id"));
                            }
                            seen_model_id = true;
                            f_model_id = map.next_value::<Option<String>>()?;
                        }
                        "query_image" => {
                            if seen_query_image {
                                return Err(serde::de::Error::duplicate_field("query_image"));
                            }
                            seen_query_image = true;
                            f_query_image = map.next_value::<Option<String>>()?;
                        }
                        "query_text" => {
                            if seen_query_text {
                                return Err(serde::de::Error::duplicate_field("query_text"));
                            }
                            seen_query_text = true;
                            f_query_text = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NeuralQuery {
                    name: f_name,
                    boost: f_boost,
                    filter: f_filter,
                    k: f_k,
                    max_distance: f_max_distance,
                    min_score: f_min_score,
                    model_id: f_model_id,
                    query_image: f_query_image,
                    query_text: f_query_text,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NeuralQuery {
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
        if let Some(v) = &self.filter {
            map.serialize_entry("filter", v)?;
        }
        if let Some(v) = &self.k {
            map.serialize_entry("k", v)?;
        }
        if let Some(v) = &self.max_distance {
            map.serialize_entry("max_distance", v)?;
        }
        if let Some(v) = &self.min_score {
            map.serialize_entry("min_score", v)?;
        }
        if let Some(v) = &self.model_id {
            map.serialize_entry("model_id", v)?;
        }
        if let Some(v) = &self.query_image {
            map.serialize_entry("query_image", v)?;
        }
        if let Some(v) = &self.query_text {
            map.serialize_entry("query_text", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NumberRangeQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub relation: Option<crate::types::common_query_dsl::RangeRelation>,
    pub from: Option<crate::types::common_query_dsl::NumberRangeQueryFrom>,
    #[doc = "Greater than."]
    pub gt: Option<f64>,
    #[doc = "Greater than or equal to."]
    pub gte: Option<f64>,
    #[doc = "Include the lower bound"]
    pub include_lower: Option<bool>,
    #[doc = "Include the upper bound"]
    pub include_upper: Option<bool>,
    #[doc = "Less than."]
    pub lt: Option<f64>,
    #[doc = "Less than or equal to."]
    pub lte: Option<f64>,
    pub to: Option<crate::types::common_query_dsl::NumberRangeQueryTo>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NumberRangeQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NumberRangeQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NumberRangeQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_relation: Option<crate::types::common_query_dsl::RangeRelation> = None;
                let mut seen_relation = false;
                let mut f_from: Option<crate::types::common_query_dsl::NumberRangeQueryFrom> = None;
                let mut seen_from = false;
                let mut f_gt: Option<f64> = None;
                let mut seen_gt = false;
                let mut f_gte: Option<f64> = None;
                let mut seen_gte = false;
                let mut f_include_lower: Option<bool> = None;
                let mut seen_include_lower = false;
                let mut f_include_upper: Option<bool> = None;
                let mut seen_include_upper = false;
                let mut f_lt: Option<f64> = None;
                let mut seen_lt = false;
                let mut f_lte: Option<f64> = None;
                let mut seen_lte = false;
                let mut f_to: Option<crate::types::common_query_dsl::NumberRangeQueryTo> = None;
                let mut seen_to = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "relation" => {
                            if seen_relation {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            seen_relation = true;
                            f_relation = map.next_value::<Option<crate::types::common_query_dsl::RangeRelation>>()?;
                        }
                        "from" => {
                            if seen_from {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            seen_from = true;
                            f_from = map.next_value::<Option<crate::types::common_query_dsl::NumberRangeQueryFrom>>()?;
                        }
                        "gt" => {
                            if seen_gt {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            seen_gt = true;
                            f_gt = map.next_value::<Option<f64>>()?;
                        }
                        "gte" => {
                            if seen_gte {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            seen_gte = true;
                            f_gte = map.next_value::<Option<f64>>()?;
                        }
                        "include_lower" => {
                            if seen_include_lower {
                                return Err(serde::de::Error::duplicate_field("include_lower"));
                            }
                            seen_include_lower = true;
                            f_include_lower = map.next_value::<Option<bool>>()?;
                        }
                        "include_upper" => {
                            if seen_include_upper {
                                return Err(serde::de::Error::duplicate_field("include_upper"));
                            }
                            seen_include_upper = true;
                            f_include_upper = map.next_value::<Option<bool>>()?;
                        }
                        "lt" => {
                            if seen_lt {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            seen_lt = true;
                            f_lt = map.next_value::<Option<f64>>()?;
                        }
                        "lte" => {
                            if seen_lte {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            seen_lte = true;
                            f_lte = map.next_value::<Option<f64>>()?;
                        }
                        "to" => {
                            if seen_to {
                                return Err(serde::de::Error::duplicate_field("to"));
                            }
                            seen_to = true;
                            f_to = map.next_value::<Option<crate::types::common_query_dsl::NumberRangeQueryTo>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NumberRangeQuery {
                    name: f_name,
                    boost: f_boost,
                    relation: f_relation,
                    from: f_from,
                    gt: f_gt,
                    gte: f_gte,
                    include_lower: f_include_lower,
                    include_upper: f_include_upper,
                    lt: f_lt,
                    lte: f_lte,
                    to: f_to,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NumberRangeQuery {
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
        if let Some(v) = &self.relation {
            map.serialize_entry("relation", v)?;
        }
        if let Some(v) = &self.from {
            map.serialize_entry("from", v)?;
        }
        if let Some(v) = &self.gt {
            map.serialize_entry("gt", v)?;
        }
        if let Some(v) = &self.gte {
            map.serialize_entry("gte", v)?;
        }
        if let Some(v) = &self.include_lower {
            map.serialize_entry("include_lower", v)?;
        }
        if let Some(v) = &self.include_upper {
            map.serialize_entry("include_upper", v)?;
        }
        if let Some(v) = &self.lt {
            map.serialize_entry("lt", v)?;
        }
        if let Some(v) = &self.lte {
            map.serialize_entry("lte", v)?;
        }
        if let Some(v) = &self.to {
            map.serialize_entry("to", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(NumberRangeQueryFrom::Number)
            }
            Some(b'"') => serde_json::from_str(text).map(NumberRangeQueryFrom::String),
            Some(b'n') => return Ok(NumberRangeQueryFrom::Null),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a NumberRangeQueryFrom value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(NumberRangeQueryTo::Number)
            }
            Some(b'"') => serde_json::from_str(text).map(NumberRangeQueryTo::String),
            Some(b'n') => return Ok(NumberRangeQueryTo::Null),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a NumberRangeQueryTo value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct NumericDecayPlacement {
    pub decay: Option<f64>,
    pub offset: Option<f64>,
    pub origin: f64,
    pub scale: f64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NumericDecayPlacement {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NumericDecayPlacement;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NumericDecayPlacement object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_decay: Option<f64> = None;
                let mut seen_decay = false;
                let mut f_offset: Option<f64> = None;
                let mut seen_offset = false;
                let mut f_origin: Option<f64> = None;
                let mut f_scale: Option<f64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "decay" => {
                            if seen_decay {
                                return Err(serde::de::Error::duplicate_field("decay"));
                            }
                            seen_decay = true;
                            f_decay = map.next_value::<Option<f64>>()?;
                        }
                        "offset" => {
                            if seen_offset {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            seen_offset = true;
                            f_offset = map.next_value::<Option<f64>>()?;
                        }
                        "origin" => {
                            if f_origin.is_some() {
                                return Err(serde::de::Error::duplicate_field("origin"));
                            }
                            f_origin = Some(map.next_value::<f64>()?);
                        }
                        "scale" => {
                            if f_scale.is_some() {
                                return Err(serde::de::Error::duplicate_field("scale"));
                            }
                            f_scale = Some(map.next_value::<f64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NumericDecayPlacement {
                    decay: f_decay,
                    offset: f_offset,
                    origin: f_origin.ok_or_else(|| serde::de::Error::missing_field("origin"))?,
                    scale: f_scale.ok_or_else(|| serde::de::Error::missing_field("scale"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NumericDecayPlacement {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.decay {
            map.serialize_entry("decay", v)?;
        }
        if let Some(v) = &self.offset {
            map.serialize_entry("offset", v)?;
        }
        map.serialize_entry("origin", &self.origin)?;
        map.serialize_entry("scale", &self.scale)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operator {
    #[doc = "`and` (also accepted: `AND`)"]
    And,
    #[doc = "`or` (also accepted: `OR`)"]
    Or,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl Operator {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            Operator::And => "and",
            Operator::Or => "or",
            Operator::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for Operator {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for Operator {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "and" | "AND" => Operator::And,
            "or" | "OR" => Operator::Or,
            _ => Operator::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ParentIdQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub id: Option<crate::types::common::Id>,
    #[doc = "Indicates whether to ignore an unmapped `type` and not return any documents instead of an error."]
    pub ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped>,
    pub r#type: Option<crate::types::common::RelationName>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ParentIdQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ParentIdQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ParentIdQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut seen_id = false;
                let mut f_ignore_unmapped: Option<crate::types::common_query_dsl::IgnoreUnmapped> =
                    None;
                let mut seen_ignore_unmapped = false;
                let mut f_type: Option<crate::types::common::RelationName> = None;
                let mut seen_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        "ignore_unmapped" => {
                            if seen_ignore_unmapped {
                                return Err(serde::de::Error::duplicate_field("ignore_unmapped"));
                            }
                            seen_ignore_unmapped = true;
                            f_ignore_unmapped = map.next_value::<Option<crate::types::common_query_dsl::IgnoreUnmapped>>()?;
                        }
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type =
                                map.next_value::<Option<crate::types::common::RelationName>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ParentIdQuery {
                    name: f_name,
                    boost: f_boost,
                    id: f_id,
                    ignore_unmapped: f_ignore_unmapped,
                    r#type: f_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ParentIdQuery {
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
        if let Some(v) = &self.id {
            map.serialize_entry("id", v)?;
        }
        if let Some(v) = &self.ignore_unmapped {
            map.serialize_entry("ignore_unmapped", v)?;
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
pub struct PercolateQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "The source of the document being percolated."]
    pub document: Option<Box<serde_json::value::RawValue>>,
    #[doc = "An array of sources of the documents being percolated."]
    pub documents: Option<Vec<Box<serde_json::value::RawValue>>>,
    pub field: crate::types::common::Field,
    pub id: Option<crate::types::common::Id>,
    pub index: Option<crate::types::common::IndexName>,
    #[doc = "The suffix used for the `_percolator_document_slot` field when multiple `percolate` queries are specified."]
    pub _name: Option<String>,
    #[doc = "Preference used to fetch document to percolate."]
    pub preference: Option<String>,
    pub routing: Option<crate::types::common::Routing>,
    pub version: Option<crate::types::common::VersionNumber>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PercolateQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PercolateQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PercolateQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_document: Option<Box<serde_json::value::RawValue>> = None;
                let mut seen_document = false;
                let mut f_documents: Option<Vec<Box<serde_json::value::RawValue>>> = None;
                let mut seen_documents = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut seen_id = false;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut seen_index = false;
                let mut f__name: Option<String> = None;
                let mut seen__name = false;
                let mut f_preference: Option<String> = None;
                let mut seen_preference = false;
                let mut f_routing: Option<crate::types::common::Routing> = None;
                let mut seen_routing = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "document" => {
                            if seen_document {
                                return Err(serde::de::Error::duplicate_field("document"));
                            }
                            seen_document = true;
                            f_document =
                                map.next_value::<Option<Box<serde_json::value::RawValue>>>()?;
                        }
                        "documents" => {
                            if seen_documents {
                                return Err(serde::de::Error::duplicate_field("documents"));
                            }
                            seen_documents = true;
                            f_documents =
                                map.next_value::<Option<Vec<Box<serde_json::value::RawValue>>>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "name" => {
                            if seen__name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen__name = true;
                            f__name = map.next_value::<Option<String>>()?;
                        }
                        "preference" => {
                            if seen_preference {
                                return Err(serde::de::Error::duplicate_field("preference"));
                            }
                            seen_preference = true;
                            f_preference = map.next_value::<Option<String>>()?;
                        }
                        "routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("routing"));
                            }
                            seen_routing = true;
                            f_routing =
                                map.next_value::<Option<crate::types::common::Routing>>()?;
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionNumber>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PercolateQuery {
                    name: f_name,
                    boost: f_boost,
                    document: f_document,
                    documents: f_documents,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    id: f_id,
                    index: f_index,
                    _name: f__name,
                    preference: f_preference,
                    routing: f_routing,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PercolateQuery {
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
        if let Some(v) = &self.document {
            map.serialize_entry("document", v)?;
        }
        if let Some(v) = &self.documents {
            map.serialize_entry("documents", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.id {
            map.serialize_entry("id", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self._name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.preference {
            map.serialize_entry("preference", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("routing", v)?;
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'"') => serde_json::from_str(text).map(PrefixQuery::Value),
            Some(b'{') => serde_json::from_str(text).map(PrefixQuery::Object),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a PrefixQuery value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct PrefixQueryObject {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "Allows ASCII case insensitive matching of the value with the indexed field values when set to `true`. Default is `false` which means the case sensitivity of matching depends on the underlying field's mapping."]
    pub case_insensitive: Option<bool>,
    pub rewrite: Option<crate::types::common::MultiTermQueryRewrite>,
    #[doc = "Beginning characters of terms you wish to find in the provided field."]
    pub value: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PrefixQueryObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PrefixQueryObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PrefixQueryObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_case_insensitive: Option<bool> = None;
                let mut seen_case_insensitive = false;
                let mut f_rewrite: Option<crate::types::common::MultiTermQueryRewrite> = None;
                let mut seen_rewrite = false;
                let mut f_value: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "case_insensitive" => {
                            if seen_case_insensitive {
                                return Err(serde::de::Error::duplicate_field("case_insensitive"));
                            }
                            seen_case_insensitive = true;
                            f_case_insensitive = map.next_value::<Option<bool>>()?;
                        }
                        "rewrite" => {
                            if seen_rewrite {
                                return Err(serde::de::Error::duplicate_field("rewrite"));
                            }
                            seen_rewrite = true;
                            f_rewrite = map
                                .next_value::<Option<crate::types::common::MultiTermQueryRewrite>>(
                                )?;
                        }
                        "value" => {
                            if f_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            f_value = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PrefixQueryObject {
                    name: f_name,
                    boost: f_boost,
                    case_insensitive: f_case_insensitive,
                    rewrite: f_rewrite,
                    value: f_value.ok_or_else(|| serde::de::Error::missing_field("value"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PrefixQueryObject {
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
        if let Some(v) = &self.case_insensitive {
            map.serialize_entry("case_insensitive", v)?;
        }
        if let Some(v) = &self.rewrite {
            map.serialize_entry("rewrite", v)?;
        }
        map.serialize_entry("value", &self.value)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct QueryContainer {
    pub agentic: Option<crate::types::common_query_dsl::AgenticQuery>,
    pub bool: Option<Box<crate::types::common_query_dsl::BoolQuery>>,
    pub boosting: Option<Box<crate::types::common_query_dsl::BoostingQuery>>,
    pub combined_fields: Option<crate::types::common_query_dsl::CombinedFieldsQuery>,
    pub common: Option<
        std::collections::BTreeMap<String, crate::types::common_query_dsl::CommonTermsQuery>,
    >,
    pub constant_score: Option<Box<crate::types::common_query_dsl::ConstantScoreQuery>>,
    pub dis_max: Option<crate::types::common_query_dsl::DisMaxQuery>,
    pub distance_feature: Option<crate::types::common_query_dsl::DistanceFeatureQuery>,
    pub exists: Option<crate::types::common_query_dsl::ExistsQuery>,
    pub field_masking_span: Option<Box<crate::types::common_query_dsl::SpanFieldMaskingQuery>>,
    pub function_score: Option<Box<crate::types::common_query_dsl::FunctionScoreQuery>>,
    #[doc = "Returns documents that contain terms similar to the search term, as measured by a Levenshtein edit distance."]
    pub fuzzy:
        Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::FuzzyQuery>>,
    pub geo_bounding_box: Option<crate::types::common_query_dsl::GeoBoundingBoxQuery>,
    pub geo_distance: Option<crate::types::common_query_dsl::GeoDistanceQuery>,
    pub geo_polygon: Option<crate::types::common_query_dsl::GeoPolygonQuery>,
    pub geo_shape: Option<crate::types::common_query_dsl::GeoShapeQuery>,
    pub has_child: Option<Box<crate::types::common_query_dsl::HasChildQuery>>,
    pub has_parent: Option<Box<crate::types::common_query_dsl::HasParentQuery>>,
    pub hybrid: Option<Box<crate::types::common_query_dsl::HybridQuery>>,
    pub ids: Option<crate::types::common_query_dsl::IdsQuery>,
    #[doc = "Returns documents based on the order and proximity of matching terms."]
    pub intervals:
        Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::IntervalsQuery>>,
    pub knn: Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::KnnQuery>>,
    #[doc = "Returns documents that match a provided text, number, date or Boolean value. The provided text is analyzed before matching."]
    pub r#match:
        Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::MatchQuery>>,
    pub match_all: Option<crate::types::common_query_dsl::MatchAllQuery>,
    #[doc = "Analyzes its input and constructs a `bool` query from the terms. Each term except the last is used in a `term` query. The last term is used in a prefix query."]
    pub match_bool_prefix: Option<
        std::collections::BTreeMap<String, crate::types::common_query_dsl::MatchBoolPrefixQuery>,
    >,
    pub match_none: Option<crate::types::common_query_dsl::MatchNoneQuery>,
    #[doc = "Analyzes the text and creates a phrase query out of the analyzed text."]
    pub match_phrase: Option<
        std::collections::BTreeMap<String, crate::types::common_query_dsl::MatchPhraseQuery>,
    >,
    #[doc = "Returns documents that contain the words of a provided text, in the same order as provided. The last term of the provided text is treated as a prefix, matching any words that begin with that term."]
    pub match_phrase_prefix: Option<
        std::collections::BTreeMap<String, crate::types::common_query_dsl::MatchPhrasePrefixQuery>,
    >,
    pub more_like_this: Option<crate::types::common_query_dsl::MoreLikeThisQuery>,
    pub multi_match: Option<crate::types::common_query_dsl::MultiMatchQuery>,
    pub nested: Option<Box<crate::types::common_query_dsl::NestedQuery>>,
    pub neural:
        Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::NeuralQuery>>,
    pub parent_id: Option<crate::types::common_query_dsl::ParentIdQuery>,
    pub percolate: Option<crate::types::common_query_dsl::PercolateQuery>,
    #[doc = "Returns documents that contain a specific prefix in a provided field."]
    pub prefix:
        Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::PrefixQuery>>,
    pub query_string: Option<crate::types::common_query_dsl::QueryStringQuery>,
    #[doc = "Returns documents that contain terms within a provided range."]
    pub range:
        Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::RangeQuery>>,
    pub rank_feature: Option<crate::types::common_query_dsl::RankFeatureQuery>,
    #[doc = "Returns documents that contain terms matching a regular expression."]
    pub regexp:
        Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::RegexpQuery>>,
    pub script: Option<crate::types::common_query_dsl::ScriptQuery>,
    pub script_score: Option<Box<crate::types::common_query_dsl::ScriptScoreQuery>>,
    pub simple_query_string: Option<crate::types::common_query_dsl::SimpleQueryStringQuery>,
    pub span_containing: Option<Box<crate::types::common_query_dsl::SpanContainingQuery>>,
    pub span_first: Option<Box<crate::types::common_query_dsl::SpanFirstQuery>>,
    pub span_multi: Option<Box<crate::types::common_query_dsl::SpanMultiTermQuery>>,
    pub span_near: Option<crate::types::common_query_dsl::SpanNearQuery>,
    pub span_not: Option<Box<crate::types::common_query_dsl::SpanNotQuery>>,
    pub span_or: Option<crate::types::common_query_dsl::SpanOrQuery>,
    #[doc = "Matches spans containing a term."]
    pub span_term:
        Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::SpanTermQuery>>,
    pub span_within: Option<Box<crate::types::common_query_dsl::SpanWithinQuery>>,
    #[doc = "Return documents using a template query contains placeholders that are resolved by search request processors during query execution."]
    pub template: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "Returns documents that contain an exact term in a provided field. To return a document, the query term must exactly match the queried field's value, including white space and capitalization."]
    pub term: Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::TermQuery>>,
    #[doc = "Returns documents that contain one or more exact terms in a provided field."]
    pub terms: Option<crate::types::common_query_dsl::TermsQuery>,
    #[doc = "Returns documents that contain a minimum number of exact terms in a provided field. To return a document, a required number of terms must exactly match the field values, including white space and capitalization."]
    pub terms_set:
        Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::TermsSetQuery>>,
    pub r#type: Option<crate::types::common_query_dsl::TypeQuery>,
    #[doc = "Returns documents that contain terms matching a wildcard pattern."]
    pub wildcard:
        Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::WildcardQuery>>,
    pub wrapper: Option<crate::types::common_query_dsl::WrapperQuery>,
    pub xy_shape: Option<crate::types::common_query_dsl::XyShapeQuery>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for QueryContainer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = QueryContainer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a QueryContainer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_agentic: Option<crate::types::common_query_dsl::AgenticQuery> = None;
                let mut seen_agentic = false;
                let mut f_bool: Option<Box<crate::types::common_query_dsl::BoolQuery>> = None;
                let mut seen_bool = false;
                let mut f_boosting: Option<Box<crate::types::common_query_dsl::BoostingQuery>> =
                    None;
                let mut seen_boosting = false;
                let mut f_combined_fields: Option<
                    crate::types::common_query_dsl::CombinedFieldsQuery,
                > = None;
                let mut seen_combined_fields = false;
                let mut f_common: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::common_query_dsl::CommonTermsQuery,
                    >,
                > = None;
                let mut seen_common = false;
                let mut f_constant_score: Option<
                    Box<crate::types::common_query_dsl::ConstantScoreQuery>,
                > = None;
                let mut seen_constant_score = false;
                let mut f_dis_max: Option<crate::types::common_query_dsl::DisMaxQuery> = None;
                let mut seen_dis_max = false;
                let mut f_distance_feature: Option<
                    crate::types::common_query_dsl::DistanceFeatureQuery,
                > = None;
                let mut seen_distance_feature = false;
                let mut f_exists: Option<crate::types::common_query_dsl::ExistsQuery> = None;
                let mut seen_exists = false;
                let mut f_field_masking_span: Option<
                    Box<crate::types::common_query_dsl::SpanFieldMaskingQuery>,
                > = None;
                let mut seen_field_masking_span = false;
                let mut f_function_score: Option<
                    Box<crate::types::common_query_dsl::FunctionScoreQuery>,
                > = None;
                let mut seen_function_score = false;
                let mut f_fuzzy: Option<
                    std::collections::BTreeMap<String, crate::types::common_query_dsl::FuzzyQuery>,
                > = None;
                let mut seen_fuzzy = false;
                let mut f_geo_bounding_box: Option<
                    crate::types::common_query_dsl::GeoBoundingBoxQuery,
                > = None;
                let mut seen_geo_bounding_box = false;
                let mut f_geo_distance: Option<crate::types::common_query_dsl::GeoDistanceQuery> =
                    None;
                let mut seen_geo_distance = false;
                let mut f_geo_polygon: Option<crate::types::common_query_dsl::GeoPolygonQuery> =
                    None;
                let mut seen_geo_polygon = false;
                let mut f_geo_shape: Option<crate::types::common_query_dsl::GeoShapeQuery> = None;
                let mut seen_geo_shape = false;
                let mut f_has_child: Option<Box<crate::types::common_query_dsl::HasChildQuery>> =
                    None;
                let mut seen_has_child = false;
                let mut f_has_parent: Option<Box<crate::types::common_query_dsl::HasParentQuery>> =
                    None;
                let mut seen_has_parent = false;
                let mut f_hybrid: Option<Box<crate::types::common_query_dsl::HybridQuery>> = None;
                let mut seen_hybrid = false;
                let mut f_ids: Option<crate::types::common_query_dsl::IdsQuery> = None;
                let mut seen_ids = false;
                let mut f_intervals: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::common_query_dsl::IntervalsQuery,
                    >,
                > = None;
                let mut seen_intervals = false;
                let mut f_knn: Option<
                    std::collections::BTreeMap<String, crate::types::common_query_dsl::KnnQuery>,
                > = None;
                let mut seen_knn = false;
                let mut f_match: Option<
                    std::collections::BTreeMap<String, crate::types::common_query_dsl::MatchQuery>,
                > = None;
                let mut seen_match = false;
                let mut f_match_all: Option<crate::types::common_query_dsl::MatchAllQuery> = None;
                let mut seen_match_all = false;
                let mut f_match_bool_prefix: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::common_query_dsl::MatchBoolPrefixQuery,
                    >,
                > = None;
                let mut seen_match_bool_prefix = false;
                let mut f_match_none: Option<crate::types::common_query_dsl::MatchNoneQuery> = None;
                let mut seen_match_none = false;
                let mut f_match_phrase: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::common_query_dsl::MatchPhraseQuery,
                    >,
                > = None;
                let mut seen_match_phrase = false;
                let mut f_match_phrase_prefix: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::common_query_dsl::MatchPhrasePrefixQuery,
                    >,
                > = None;
                let mut seen_match_phrase_prefix = false;
                let mut f_more_like_this: Option<
                    crate::types::common_query_dsl::MoreLikeThisQuery,
                > = None;
                let mut seen_more_like_this = false;
                let mut f_multi_match: Option<crate::types::common_query_dsl::MultiMatchQuery> =
                    None;
                let mut seen_multi_match = false;
                let mut f_nested: Option<Box<crate::types::common_query_dsl::NestedQuery>> = None;
                let mut seen_nested = false;
                let mut f_neural: Option<
                    std::collections::BTreeMap<String, crate::types::common_query_dsl::NeuralQuery>,
                > = None;
                let mut seen_neural = false;
                let mut f_parent_id: Option<crate::types::common_query_dsl::ParentIdQuery> = None;
                let mut seen_parent_id = false;
                let mut f_percolate: Option<crate::types::common_query_dsl::PercolateQuery> = None;
                let mut seen_percolate = false;
                let mut f_prefix: Option<
                    std::collections::BTreeMap<String, crate::types::common_query_dsl::PrefixQuery>,
                > = None;
                let mut seen_prefix = false;
                let mut f_query_string: Option<crate::types::common_query_dsl::QueryStringQuery> =
                    None;
                let mut seen_query_string = false;
                let mut f_range: Option<
                    std::collections::BTreeMap<String, crate::types::common_query_dsl::RangeQuery>,
                > = None;
                let mut seen_range = false;
                let mut f_rank_feature: Option<crate::types::common_query_dsl::RankFeatureQuery> =
                    None;
                let mut seen_rank_feature = false;
                let mut f_regexp: Option<
                    std::collections::BTreeMap<String, crate::types::common_query_dsl::RegexpQuery>,
                > = None;
                let mut seen_regexp = false;
                let mut f_script: Option<crate::types::common_query_dsl::ScriptQuery> = None;
                let mut seen_script = false;
                let mut f_script_score: Option<
                    Box<crate::types::common_query_dsl::ScriptScoreQuery>,
                > = None;
                let mut seen_script_score = false;
                let mut f_simple_query_string: Option<
                    crate::types::common_query_dsl::SimpleQueryStringQuery,
                > = None;
                let mut seen_simple_query_string = false;
                let mut f_span_containing: Option<
                    Box<crate::types::common_query_dsl::SpanContainingQuery>,
                > = None;
                let mut seen_span_containing = false;
                let mut f_span_first: Option<Box<crate::types::common_query_dsl::SpanFirstQuery>> =
                    None;
                let mut seen_span_first = false;
                let mut f_span_multi: Option<
                    Box<crate::types::common_query_dsl::SpanMultiTermQuery>,
                > = None;
                let mut seen_span_multi = false;
                let mut f_span_near: Option<crate::types::common_query_dsl::SpanNearQuery> = None;
                let mut seen_span_near = false;
                let mut f_span_not: Option<Box<crate::types::common_query_dsl::SpanNotQuery>> =
                    None;
                let mut seen_span_not = false;
                let mut f_span_or: Option<crate::types::common_query_dsl::SpanOrQuery> = None;
                let mut seen_span_or = false;
                let mut f_span_term: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::common_query_dsl::SpanTermQuery,
                    >,
                > = None;
                let mut seen_span_term = false;
                let mut f_span_within: Option<
                    Box<crate::types::common_query_dsl::SpanWithinQuery>,
                > = None;
                let mut seen_span_within = false;
                let mut f_template: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_template = false;
                let mut f_term: Option<
                    std::collections::BTreeMap<String, crate::types::common_query_dsl::TermQuery>,
                > = None;
                let mut seen_term = false;
                let mut f_terms: Option<crate::types::common_query_dsl::TermsQuery> = None;
                let mut seen_terms = false;
                let mut f_terms_set: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::common_query_dsl::TermsSetQuery,
                    >,
                > = None;
                let mut seen_terms_set = false;
                let mut f_type: Option<crate::types::common_query_dsl::TypeQuery> = None;
                let mut seen_type = false;
                let mut f_wildcard: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::common_query_dsl::WildcardQuery,
                    >,
                > = None;
                let mut seen_wildcard = false;
                let mut f_wrapper: Option<crate::types::common_query_dsl::WrapperQuery> = None;
                let mut seen_wrapper = false;
                let mut f_xy_shape: Option<crate::types::common_query_dsl::XyShapeQuery> = None;
                let mut seen_xy_shape = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "agentic" => {
                            if seen_agentic {
                                return Err(serde::de::Error::duplicate_field("agentic"));
                            }
                            seen_agentic = true;
                            f_agentic = map
                                .next_value::<Option<crate::types::common_query_dsl::AgenticQuery>>(
                                )?;
                        }
                        "bool" => {
                            if seen_bool {
                                return Err(serde::de::Error::duplicate_field("bool"));
                            }
                            seen_bool = true;
                            f_bool = map.next_value::<Option<Box<crate::types::common_query_dsl::BoolQuery>>>()?;
                        }
                        "boosting" => {
                            if seen_boosting {
                                return Err(serde::de::Error::duplicate_field("boosting"));
                            }
                            seen_boosting = true;
                            f_boosting = map.next_value::<Option<Box<crate::types::common_query_dsl::BoostingQuery>>>()?;
                        }
                        "combined_fields" => {
                            if seen_combined_fields {
                                return Err(serde::de::Error::duplicate_field("combined_fields"));
                            }
                            seen_combined_fields = true;
                            f_combined_fields = map.next_value::<Option<crate::types::common_query_dsl::CombinedFieldsQuery>>()?;
                        }
                        "common" => {
                            if seen_common {
                                return Err(serde::de::Error::duplicate_field("common"));
                            }
                            seen_common = true;
                            f_common = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_query_dsl::CommonTermsQuery,
                                >,
                            >>()?;
                        }
                        "constant_score" => {
                            if seen_constant_score {
                                return Err(serde::de::Error::duplicate_field("constant_score"));
                            }
                            seen_constant_score = true;
                            f_constant_score = map.next_value::<Option<Box<crate::types::common_query_dsl::ConstantScoreQuery>>>()?;
                        }
                        "dis_max" => {
                            if seen_dis_max {
                                return Err(serde::de::Error::duplicate_field("dis_max"));
                            }
                            seen_dis_max = true;
                            f_dis_max = map
                                .next_value::<Option<crate::types::common_query_dsl::DisMaxQuery>>(
                                )?;
                        }
                        "distance_feature" => {
                            if seen_distance_feature {
                                return Err(serde::de::Error::duplicate_field("distance_feature"));
                            }
                            seen_distance_feature = true;
                            f_distance_feature = map.next_value::<Option<crate::types::common_query_dsl::DistanceFeatureQuery>>()?;
                        }
                        "exists" => {
                            if seen_exists {
                                return Err(serde::de::Error::duplicate_field("exists"));
                            }
                            seen_exists = true;
                            f_exists = map
                                .next_value::<Option<crate::types::common_query_dsl::ExistsQuery>>(
                                )?;
                        }
                        "field_masking_span" => {
                            if seen_field_masking_span {
                                return Err(serde::de::Error::duplicate_field(
                                    "field_masking_span",
                                ));
                            }
                            seen_field_masking_span = true;
                            f_field_masking_span = map.next_value::<Option<
                                Box<crate::types::common_query_dsl::SpanFieldMaskingQuery>,
                            >>()?;
                        }
                        "function_score" => {
                            if seen_function_score {
                                return Err(serde::de::Error::duplicate_field("function_score"));
                            }
                            seen_function_score = true;
                            f_function_score = map.next_value::<Option<Box<crate::types::common_query_dsl::FunctionScoreQuery>>>()?;
                        }
                        "fuzzy" => {
                            if seen_fuzzy {
                                return Err(serde::de::Error::duplicate_field("fuzzy"));
                            }
                            seen_fuzzy = true;
                            f_fuzzy = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_query_dsl::FuzzyQuery,
                                >,
                            >>()?;
                        }
                        "geo_bounding_box" => {
                            if seen_geo_bounding_box {
                                return Err(serde::de::Error::duplicate_field("geo_bounding_box"));
                            }
                            seen_geo_bounding_box = true;
                            f_geo_bounding_box = map.next_value::<Option<crate::types::common_query_dsl::GeoBoundingBoxQuery>>()?;
                        }
                        "geo_distance" => {
                            if seen_geo_distance {
                                return Err(serde::de::Error::duplicate_field("geo_distance"));
                            }
                            seen_geo_distance = true;
                            f_geo_distance = map.next_value::<Option<crate::types::common_query_dsl::GeoDistanceQuery>>()?;
                        }
                        "geo_polygon" => {
                            if seen_geo_polygon {
                                return Err(serde::de::Error::duplicate_field("geo_polygon"));
                            }
                            seen_geo_polygon = true;
                            f_geo_polygon = map.next_value::<Option<crate::types::common_query_dsl::GeoPolygonQuery>>()?;
                        }
                        "geo_shape" => {
                            if seen_geo_shape {
                                return Err(serde::de::Error::duplicate_field("geo_shape"));
                            }
                            seen_geo_shape = true;
                            f_geo_shape = map.next_value::<Option<crate::types::common_query_dsl::GeoShapeQuery>>()?;
                        }
                        "has_child" => {
                            if seen_has_child {
                                return Err(serde::de::Error::duplicate_field("has_child"));
                            }
                            seen_has_child = true;
                            f_has_child = map.next_value::<Option<Box<crate::types::common_query_dsl::HasChildQuery>>>()?;
                        }
                        "has_parent" => {
                            if seen_has_parent {
                                return Err(serde::de::Error::duplicate_field("has_parent"));
                            }
                            seen_has_parent = true;
                            f_has_parent = map.next_value::<Option<Box<crate::types::common_query_dsl::HasParentQuery>>>()?;
                        }
                        "hybrid" => {
                            if seen_hybrid {
                                return Err(serde::de::Error::duplicate_field("hybrid"));
                            }
                            seen_hybrid = true;
                            f_hybrid = map.next_value::<Option<Box<crate::types::common_query_dsl::HybridQuery>>>()?;
                        }
                        "ids" => {
                            if seen_ids {
                                return Err(serde::de::Error::duplicate_field("ids"));
                            }
                            seen_ids = true;
                            f_ids = map
                                .next_value::<Option<crate::types::common_query_dsl::IdsQuery>>()?;
                        }
                        "intervals" => {
                            if seen_intervals {
                                return Err(serde::de::Error::duplicate_field("intervals"));
                            }
                            seen_intervals = true;
                            f_intervals = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_query_dsl::IntervalsQuery,
                                >,
                            >>()?;
                        }
                        "knn" => {
                            if seen_knn {
                                return Err(serde::de::Error::duplicate_field("knn"));
                            }
                            seen_knn = true;
                            f_knn = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_query_dsl::KnnQuery,
                                >,
                            >>()?;
                        }
                        "match" => {
                            if seen_match {
                                return Err(serde::de::Error::duplicate_field("match"));
                            }
                            seen_match = true;
                            f_match = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_query_dsl::MatchQuery,
                                >,
                            >>()?;
                        }
                        "match_all" => {
                            if seen_match_all {
                                return Err(serde::de::Error::duplicate_field("match_all"));
                            }
                            seen_match_all = true;
                            f_match_all = map.next_value::<Option<crate::types::common_query_dsl::MatchAllQuery>>()?;
                        }
                        "match_bool_prefix" => {
                            if seen_match_bool_prefix {
                                return Err(serde::de::Error::duplicate_field("match_bool_prefix"));
                            }
                            seen_match_bool_prefix = true;
                            f_match_bool_prefix = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_query_dsl::MatchBoolPrefixQuery,
                                >,
                            >>()?;
                        }
                        "match_none" => {
                            if seen_match_none {
                                return Err(serde::de::Error::duplicate_field("match_none"));
                            }
                            seen_match_none = true;
                            f_match_none = map.next_value::<Option<crate::types::common_query_dsl::MatchNoneQuery>>()?;
                        }
                        "match_phrase" => {
                            if seen_match_phrase {
                                return Err(serde::de::Error::duplicate_field("match_phrase"));
                            }
                            seen_match_phrase = true;
                            f_match_phrase = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_query_dsl::MatchPhraseQuery,
                                >,
                            >>()?;
                        }
                        "match_phrase_prefix" => {
                            if seen_match_phrase_prefix {
                                return Err(serde::de::Error::duplicate_field(
                                    "match_phrase_prefix",
                                ));
                            }
                            seen_match_phrase_prefix = true;
                            f_match_phrase_prefix = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_query_dsl::MatchPhrasePrefixQuery,
                                >,
                            >>()?;
                        }
                        "more_like_this" => {
                            if seen_more_like_this {
                                return Err(serde::de::Error::duplicate_field("more_like_this"));
                            }
                            seen_more_like_this = true;
                            f_more_like_this = map.next_value::<Option<crate::types::common_query_dsl::MoreLikeThisQuery>>()?;
                        }
                        "multi_match" => {
                            if seen_multi_match {
                                return Err(serde::de::Error::duplicate_field("multi_match"));
                            }
                            seen_multi_match = true;
                            f_multi_match = map.next_value::<Option<crate::types::common_query_dsl::MultiMatchQuery>>()?;
                        }
                        "nested" => {
                            if seen_nested {
                                return Err(serde::de::Error::duplicate_field("nested"));
                            }
                            seen_nested = true;
                            f_nested = map.next_value::<Option<Box<crate::types::common_query_dsl::NestedQuery>>>()?;
                        }
                        "neural" => {
                            if seen_neural {
                                return Err(serde::de::Error::duplicate_field("neural"));
                            }
                            seen_neural = true;
                            f_neural = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_query_dsl::NeuralQuery,
                                >,
                            >>()?;
                        }
                        "parent_id" => {
                            if seen_parent_id {
                                return Err(serde::de::Error::duplicate_field("parent_id"));
                            }
                            seen_parent_id = true;
                            f_parent_id = map.next_value::<Option<crate::types::common_query_dsl::ParentIdQuery>>()?;
                        }
                        "percolate" => {
                            if seen_percolate {
                                return Err(serde::de::Error::duplicate_field("percolate"));
                            }
                            seen_percolate = true;
                            f_percolate = map.next_value::<Option<crate::types::common_query_dsl::PercolateQuery>>()?;
                        }
                        "prefix" => {
                            if seen_prefix {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            seen_prefix = true;
                            f_prefix = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_query_dsl::PrefixQuery,
                                >,
                            >>()?;
                        }
                        "query_string" => {
                            if seen_query_string {
                                return Err(serde::de::Error::duplicate_field("query_string"));
                            }
                            seen_query_string = true;
                            f_query_string = map.next_value::<Option<crate::types::common_query_dsl::QueryStringQuery>>()?;
                        }
                        "range" => {
                            if seen_range {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            seen_range = true;
                            f_range = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_query_dsl::RangeQuery,
                                >,
                            >>()?;
                        }
                        "rank_feature" => {
                            if seen_rank_feature {
                                return Err(serde::de::Error::duplicate_field("rank_feature"));
                            }
                            seen_rank_feature = true;
                            f_rank_feature = map.next_value::<Option<crate::types::common_query_dsl::RankFeatureQuery>>()?;
                        }
                        "regexp" => {
                            if seen_regexp {
                                return Err(serde::de::Error::duplicate_field("regexp"));
                            }
                            seen_regexp = true;
                            f_regexp = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_query_dsl::RegexpQuery,
                                >,
                            >>()?;
                        }
                        "script" => {
                            if seen_script {
                                return Err(serde::de::Error::duplicate_field("script"));
                            }
                            seen_script = true;
                            f_script = map
                                .next_value::<Option<crate::types::common_query_dsl::ScriptQuery>>(
                                )?;
                        }
                        "script_score" => {
                            if seen_script_score {
                                return Err(serde::de::Error::duplicate_field("script_score"));
                            }
                            seen_script_score = true;
                            f_script_score = map.next_value::<Option<Box<crate::types::common_query_dsl::ScriptScoreQuery>>>()?;
                        }
                        "simple_query_string" => {
                            if seen_simple_query_string {
                                return Err(serde::de::Error::duplicate_field(
                                    "simple_query_string",
                                ));
                            }
                            seen_simple_query_string = true;
                            f_simple_query_string = map.next_value::<Option<crate::types::common_query_dsl::SimpleQueryStringQuery>>()?;
                        }
                        "span_containing" => {
                            if seen_span_containing {
                                return Err(serde::de::Error::duplicate_field("span_containing"));
                            }
                            seen_span_containing = true;
                            f_span_containing =
                                map.next_value::<Option<
                                    Box<crate::types::common_query_dsl::SpanContainingQuery>,
                                >>()?;
                        }
                        "span_first" => {
                            if seen_span_first {
                                return Err(serde::de::Error::duplicate_field("span_first"));
                            }
                            seen_span_first = true;
                            f_span_first = map.next_value::<Option<Box<crate::types::common_query_dsl::SpanFirstQuery>>>()?;
                        }
                        "span_multi" => {
                            if seen_span_multi {
                                return Err(serde::de::Error::duplicate_field("span_multi"));
                            }
                            seen_span_multi = true;
                            f_span_multi = map.next_value::<Option<Box<crate::types::common_query_dsl::SpanMultiTermQuery>>>()?;
                        }
                        "span_near" => {
                            if seen_span_near {
                                return Err(serde::de::Error::duplicate_field("span_near"));
                            }
                            seen_span_near = true;
                            f_span_near = map.next_value::<Option<crate::types::common_query_dsl::SpanNearQuery>>()?;
                        }
                        "span_not" => {
                            if seen_span_not {
                                return Err(serde::de::Error::duplicate_field("span_not"));
                            }
                            seen_span_not = true;
                            f_span_not = map.next_value::<Option<Box<crate::types::common_query_dsl::SpanNotQuery>>>()?;
                        }
                        "span_or" => {
                            if seen_span_or {
                                return Err(serde::de::Error::duplicate_field("span_or"));
                            }
                            seen_span_or = true;
                            f_span_or = map
                                .next_value::<Option<crate::types::common_query_dsl::SpanOrQuery>>(
                                )?;
                        }
                        "span_term" => {
                            if seen_span_term {
                                return Err(serde::de::Error::duplicate_field("span_term"));
                            }
                            seen_span_term = true;
                            f_span_term = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_query_dsl::SpanTermQuery,
                                >,
                            >>()?;
                        }
                        "span_within" => {
                            if seen_span_within {
                                return Err(serde::de::Error::duplicate_field("span_within"));
                            }
                            seen_span_within = true;
                            f_span_within = map.next_value::<Option<Box<crate::types::common_query_dsl::SpanWithinQuery>>>()?;
                        }
                        "template" => {
                            if seen_template {
                                return Err(serde::de::Error::duplicate_field("template"));
                            }
                            seen_template = true;
                            f_template = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "term" => {
                            if seen_term {
                                return Err(serde::de::Error::duplicate_field("term"));
                            }
                            seen_term = true;
                            f_term = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_query_dsl::TermQuery,
                                >,
                            >>()?;
                        }
                        "terms" => {
                            if seen_terms {
                                return Err(serde::de::Error::duplicate_field("terms"));
                            }
                            seen_terms = true;
                            f_terms = map
                                .next_value::<Option<crate::types::common_query_dsl::TermsQuery>>(
                                )?;
                        }
                        "terms_set" => {
                            if seen_terms_set {
                                return Err(serde::de::Error::duplicate_field("terms_set"));
                            }
                            seen_terms_set = true;
                            f_terms_set = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_query_dsl::TermsSetQuery,
                                >,
                            >>()?;
                        }
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type = map
                                .next_value::<Option<crate::types::common_query_dsl::TypeQuery>>(
                                )?;
                        }
                        "wildcard" => {
                            if seen_wildcard {
                                return Err(serde::de::Error::duplicate_field("wildcard"));
                            }
                            seen_wildcard = true;
                            f_wildcard = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_query_dsl::WildcardQuery,
                                >,
                            >>()?;
                        }
                        "wrapper" => {
                            if seen_wrapper {
                                return Err(serde::de::Error::duplicate_field("wrapper"));
                            }
                            seen_wrapper = true;
                            f_wrapper = map
                                .next_value::<Option<crate::types::common_query_dsl::WrapperQuery>>(
                                )?;
                        }
                        "xy_shape" => {
                            if seen_xy_shape {
                                return Err(serde::de::Error::duplicate_field("xy_shape"));
                            }
                            seen_xy_shape = true;
                            f_xy_shape = map
                                .next_value::<Option<crate::types::common_query_dsl::XyShapeQuery>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(QueryContainer {
                    agentic: f_agentic,
                    bool: f_bool,
                    boosting: f_boosting,
                    combined_fields: f_combined_fields,
                    common: f_common,
                    constant_score: f_constant_score,
                    dis_max: f_dis_max,
                    distance_feature: f_distance_feature,
                    exists: f_exists,
                    field_masking_span: f_field_masking_span,
                    function_score: f_function_score,
                    fuzzy: f_fuzzy,
                    geo_bounding_box: f_geo_bounding_box,
                    geo_distance: f_geo_distance,
                    geo_polygon: f_geo_polygon,
                    geo_shape: f_geo_shape,
                    has_child: f_has_child,
                    has_parent: f_has_parent,
                    hybrid: f_hybrid,
                    ids: f_ids,
                    intervals: f_intervals,
                    knn: f_knn,
                    r#match: f_match,
                    match_all: f_match_all,
                    match_bool_prefix: f_match_bool_prefix,
                    match_none: f_match_none,
                    match_phrase: f_match_phrase,
                    match_phrase_prefix: f_match_phrase_prefix,
                    more_like_this: f_more_like_this,
                    multi_match: f_multi_match,
                    nested: f_nested,
                    neural: f_neural,
                    parent_id: f_parent_id,
                    percolate: f_percolate,
                    prefix: f_prefix,
                    query_string: f_query_string,
                    range: f_range,
                    rank_feature: f_rank_feature,
                    regexp: f_regexp,
                    script: f_script,
                    script_score: f_script_score,
                    simple_query_string: f_simple_query_string,
                    span_containing: f_span_containing,
                    span_first: f_span_first,
                    span_multi: f_span_multi,
                    span_near: f_span_near,
                    span_not: f_span_not,
                    span_or: f_span_or,
                    span_term: f_span_term,
                    span_within: f_span_within,
                    template: f_template,
                    term: f_term,
                    terms: f_terms,
                    terms_set: f_terms_set,
                    r#type: f_type,
                    wildcard: f_wildcard,
                    wrapper: f_wrapper,
                    xy_shape: f_xy_shape,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for QueryContainer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.agentic {
            map.serialize_entry("agentic", v)?;
        }
        if let Some(v) = &self.bool {
            map.serialize_entry("bool", v)?;
        }
        if let Some(v) = &self.boosting {
            map.serialize_entry("boosting", v)?;
        }
        if let Some(v) = &self.combined_fields {
            map.serialize_entry("combined_fields", v)?;
        }
        if let Some(v) = &self.common {
            map.serialize_entry("common", v)?;
        }
        if let Some(v) = &self.constant_score {
            map.serialize_entry("constant_score", v)?;
        }
        if let Some(v) = &self.dis_max {
            map.serialize_entry("dis_max", v)?;
        }
        if let Some(v) = &self.distance_feature {
            map.serialize_entry("distance_feature", v)?;
        }
        if let Some(v) = &self.exists {
            map.serialize_entry("exists", v)?;
        }
        if let Some(v) = &self.field_masking_span {
            map.serialize_entry("field_masking_span", v)?;
        }
        if let Some(v) = &self.function_score {
            map.serialize_entry("function_score", v)?;
        }
        if let Some(v) = &self.fuzzy {
            map.serialize_entry("fuzzy", v)?;
        }
        if let Some(v) = &self.geo_bounding_box {
            map.serialize_entry("geo_bounding_box", v)?;
        }
        if let Some(v) = &self.geo_distance {
            map.serialize_entry("geo_distance", v)?;
        }
        if let Some(v) = &self.geo_polygon {
            map.serialize_entry("geo_polygon", v)?;
        }
        if let Some(v) = &self.geo_shape {
            map.serialize_entry("geo_shape", v)?;
        }
        if let Some(v) = &self.has_child {
            map.serialize_entry("has_child", v)?;
        }
        if let Some(v) = &self.has_parent {
            map.serialize_entry("has_parent", v)?;
        }
        if let Some(v) = &self.hybrid {
            map.serialize_entry("hybrid", v)?;
        }
        if let Some(v) = &self.ids {
            map.serialize_entry("ids", v)?;
        }
        if let Some(v) = &self.intervals {
            map.serialize_entry("intervals", v)?;
        }
        if let Some(v) = &self.knn {
            map.serialize_entry("knn", v)?;
        }
        if let Some(v) = &self.r#match {
            map.serialize_entry("match", v)?;
        }
        if let Some(v) = &self.match_all {
            map.serialize_entry("match_all", v)?;
        }
        if let Some(v) = &self.match_bool_prefix {
            map.serialize_entry("match_bool_prefix", v)?;
        }
        if let Some(v) = &self.match_none {
            map.serialize_entry("match_none", v)?;
        }
        if let Some(v) = &self.match_phrase {
            map.serialize_entry("match_phrase", v)?;
        }
        if let Some(v) = &self.match_phrase_prefix {
            map.serialize_entry("match_phrase_prefix", v)?;
        }
        if let Some(v) = &self.more_like_this {
            map.serialize_entry("more_like_this", v)?;
        }
        if let Some(v) = &self.multi_match {
            map.serialize_entry("multi_match", v)?;
        }
        if let Some(v) = &self.nested {
            map.serialize_entry("nested", v)?;
        }
        if let Some(v) = &self.neural {
            map.serialize_entry("neural", v)?;
        }
        if let Some(v) = &self.parent_id {
            map.serialize_entry("parent_id", v)?;
        }
        if let Some(v) = &self.percolate {
            map.serialize_entry("percolate", v)?;
        }
        if let Some(v) = &self.prefix {
            map.serialize_entry("prefix", v)?;
        }
        if let Some(v) = &self.query_string {
            map.serialize_entry("query_string", v)?;
        }
        if let Some(v) = &self.range {
            map.serialize_entry("range", v)?;
        }
        if let Some(v) = &self.rank_feature {
            map.serialize_entry("rank_feature", v)?;
        }
        if let Some(v) = &self.regexp {
            map.serialize_entry("regexp", v)?;
        }
        if let Some(v) = &self.script {
            map.serialize_entry("script", v)?;
        }
        if let Some(v) = &self.script_score {
            map.serialize_entry("script_score", v)?;
        }
        if let Some(v) = &self.simple_query_string {
            map.serialize_entry("simple_query_string", v)?;
        }
        if let Some(v) = &self.span_containing {
            map.serialize_entry("span_containing", v)?;
        }
        if let Some(v) = &self.span_first {
            map.serialize_entry("span_first", v)?;
        }
        if let Some(v) = &self.span_multi {
            map.serialize_entry("span_multi", v)?;
        }
        if let Some(v) = &self.span_near {
            map.serialize_entry("span_near", v)?;
        }
        if let Some(v) = &self.span_not {
            map.serialize_entry("span_not", v)?;
        }
        if let Some(v) = &self.span_or {
            map.serialize_entry("span_or", v)?;
        }
        if let Some(v) = &self.span_term {
            map.serialize_entry("span_term", v)?;
        }
        if let Some(v) = &self.span_within {
            map.serialize_entry("span_within", v)?;
        }
        if let Some(v) = &self.template {
            map.serialize_entry("template", v)?;
        }
        if let Some(v) = &self.term {
            map.serialize_entry("term", v)?;
        }
        if let Some(v) = &self.terms {
            map.serialize_entry("terms", v)?;
        }
        if let Some(v) = &self.terms_set {
            map.serialize_entry("terms_set", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("type", v)?;
        }
        if let Some(v) = &self.wildcard {
            map.serialize_entry("wildcard", v)?;
        }
        if let Some(v) = &self.wrapper {
            map.serialize_entry("wrapper", v)?;
        }
        if let Some(v) = &self.xy_shape {
            map.serialize_entry("xy_shape", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct QueryStringQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "If `true`, the wildcard characters `*` and `?` are allowed as the first character of the query string."]
    pub allow_leading_wildcard: Option<bool>,
    #[doc = "If `true`, the query attempts to analyze wildcard terms in the query string."]
    pub analyze_wildcard: Option<bool>,
    #[doc = "Analyzer used to convert text in the query string into tokens."]
    pub analyzer: Option<String>,
    #[doc = "If `true`, match phrase queries are automatically created for multi-term synonyms."]
    pub auto_generate_synonyms_phrase_query: Option<bool>,
    pub default_field: Option<crate::types::common::Field>,
    pub default_operator: Option<crate::types::common_query_dsl::Operator>,
    #[doc = "If `true`, enable position increments in queries constructed from a `query_string` search."]
    pub enable_position_increments: Option<bool>,
    pub escape: Option<bool>,
    #[doc = "Array of fields to search. Supports wildcards (`*`)."]
    pub fields: Option<Vec<crate::types::common::Field>>,
    pub fuzziness: Option<crate::types::common::Fuzziness>,
    #[doc = "Maximum number of terms to which the query expands for fuzzy matching."]
    pub fuzzy_max_expansions: Option<i32>,
    #[doc = "Number of beginning characters left unchanged for fuzzy matching."]
    pub fuzzy_prefix_length: Option<i32>,
    pub fuzzy_rewrite: Option<crate::types::common::MultiTermQueryRewrite>,
    #[doc = "If `true`, edits for fuzzy matching include transpositions of two adjacent characters (for example, `ab` to `ba`)."]
    pub fuzzy_transpositions: Option<bool>,
    #[doc = "If `true`, format-based errors, such as providing a text value for a numeric field, are ignored."]
    pub lenient: Option<bool>,
    #[doc = "Maximum number of automaton states required for the query."]
    pub max_determinized_states: Option<i32>,
    pub minimum_should_match: Option<crate::types::common::MinimumShouldMatch>,
    #[doc = "Maximum number of positions allowed between matching tokens for phrases."]
    pub phrase_slop: Option<i32>,
    #[doc = "Query string you wish to parse and use for search."]
    pub query: String,
    #[doc = "Analyzer used to convert quoted text in the query string into tokens. For quoted text, this parameter overrides the analyzer specified in the `analyzer` parameter."]
    pub quote_analyzer: Option<String>,
    #[doc = "Suffix appended to quoted text in the query string. You can use this suffix to use a different analysis method for exact matches."]
    pub quote_field_suffix: Option<String>,
    pub rewrite: Option<crate::types::common::MultiTermQueryRewrite>,
    #[doc = "How to combine the queries generated from the individual search terms in the resulting `dis_max` query."]
    pub tie_breaker: Option<f64>,
    pub time_zone: Option<crate::types::common::TimeZone>,
    pub r#type: Option<crate::types::common_query_dsl::TextQueryType>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for QueryStringQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = QueryStringQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a QueryStringQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_allow_leading_wildcard: Option<bool> = None;
                let mut seen_allow_leading_wildcard = false;
                let mut f_analyze_wildcard: Option<bool> = None;
                let mut seen_analyze_wildcard = false;
                let mut f_analyzer: Option<String> = None;
                let mut seen_analyzer = false;
                let mut f_auto_generate_synonyms_phrase_query: Option<bool> = None;
                let mut seen_auto_generate_synonyms_phrase_query = false;
                let mut f_default_field: Option<crate::types::common::Field> = None;
                let mut seen_default_field = false;
                let mut f_default_operator: Option<crate::types::common_query_dsl::Operator> = None;
                let mut seen_default_operator = false;
                let mut f_enable_position_increments: Option<bool> = None;
                let mut seen_enable_position_increments = false;
                let mut f_escape: Option<bool> = None;
                let mut seen_escape = false;
                let mut f_fields: Option<Vec<crate::types::common::Field>> = None;
                let mut seen_fields = false;
                let mut f_fuzziness: Option<crate::types::common::Fuzziness> = None;
                let mut seen_fuzziness = false;
                let mut f_fuzzy_max_expansions: Option<i32> = None;
                let mut seen_fuzzy_max_expansions = false;
                let mut f_fuzzy_prefix_length: Option<i32> = None;
                let mut seen_fuzzy_prefix_length = false;
                let mut f_fuzzy_rewrite: Option<crate::types::common::MultiTermQueryRewrite> = None;
                let mut seen_fuzzy_rewrite = false;
                let mut f_fuzzy_transpositions: Option<bool> = None;
                let mut seen_fuzzy_transpositions = false;
                let mut f_lenient: Option<bool> = None;
                let mut seen_lenient = false;
                let mut f_max_determinized_states: Option<i32> = None;
                let mut seen_max_determinized_states = false;
                let mut f_minimum_should_match: Option<crate::types::common::MinimumShouldMatch> =
                    None;
                let mut seen_minimum_should_match = false;
                let mut f_phrase_slop: Option<i32> = None;
                let mut seen_phrase_slop = false;
                let mut f_query: Option<String> = None;
                let mut f_quote_analyzer: Option<String> = None;
                let mut seen_quote_analyzer = false;
                let mut f_quote_field_suffix: Option<String> = None;
                let mut seen_quote_field_suffix = false;
                let mut f_rewrite: Option<crate::types::common::MultiTermQueryRewrite> = None;
                let mut seen_rewrite = false;
                let mut f_tie_breaker: Option<f64> = None;
                let mut seen_tie_breaker = false;
                let mut f_time_zone: Option<crate::types::common::TimeZone> = None;
                let mut seen_time_zone = false;
                let mut f_type: Option<crate::types::common_query_dsl::TextQueryType> = None;
                let mut seen_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "allow_leading_wildcard" => {
                            if seen_allow_leading_wildcard {
                                return Err(serde::de::Error::duplicate_field(
                                    "allow_leading_wildcard",
                                ));
                            }
                            seen_allow_leading_wildcard = true;
                            f_allow_leading_wildcard = map.next_value::<Option<bool>>()?;
                        }
                        "analyze_wildcard" => {
                            if seen_analyze_wildcard {
                                return Err(serde::de::Error::duplicate_field("analyze_wildcard"));
                            }
                            seen_analyze_wildcard = true;
                            f_analyze_wildcard = map.next_value::<Option<bool>>()?;
                        }
                        "analyzer" => {
                            if seen_analyzer {
                                return Err(serde::de::Error::duplicate_field("analyzer"));
                            }
                            seen_analyzer = true;
                            f_analyzer = map.next_value::<Option<String>>()?;
                        }
                        "auto_generate_synonyms_phrase_query" => {
                            if seen_auto_generate_synonyms_phrase_query {
                                return Err(serde::de::Error::duplicate_field(
                                    "auto_generate_synonyms_phrase_query",
                                ));
                            }
                            seen_auto_generate_synonyms_phrase_query = true;
                            f_auto_generate_synonyms_phrase_query =
                                map.next_value::<Option<bool>>()?;
                        }
                        "default_field" => {
                            if seen_default_field {
                                return Err(serde::de::Error::duplicate_field("default_field"));
                            }
                            seen_default_field = true;
                            f_default_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        "default_operator" => {
                            if seen_default_operator {
                                return Err(serde::de::Error::duplicate_field("default_operator"));
                            }
                            seen_default_operator = true;
                            f_default_operator = map
                                .next_value::<Option<crate::types::common_query_dsl::Operator>>()?;
                        }
                        "enable_position_increments" => {
                            if seen_enable_position_increments {
                                return Err(serde::de::Error::duplicate_field(
                                    "enable_position_increments",
                                ));
                            }
                            seen_enable_position_increments = true;
                            f_enable_position_increments = map.next_value::<Option<bool>>()?;
                        }
                        "escape" => {
                            if seen_escape {
                                return Err(serde::de::Error::duplicate_field("escape"));
                            }
                            seen_escape = true;
                            f_escape = map.next_value::<Option<bool>>()?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields =
                                map.next_value::<Option<Vec<crate::types::common::Field>>>()?;
                        }
                        "fuzziness" => {
                            if seen_fuzziness {
                                return Err(serde::de::Error::duplicate_field("fuzziness"));
                            }
                            seen_fuzziness = true;
                            f_fuzziness =
                                map.next_value::<Option<crate::types::common::Fuzziness>>()?;
                        }
                        "fuzzy_max_expansions" => {
                            if seen_fuzzy_max_expansions {
                                return Err(serde::de::Error::duplicate_field(
                                    "fuzzy_max_expansions",
                                ));
                            }
                            seen_fuzzy_max_expansions = true;
                            f_fuzzy_max_expansions = map.next_value::<Option<i32>>()?;
                        }
                        "fuzzy_prefix_length" => {
                            if seen_fuzzy_prefix_length {
                                return Err(serde::de::Error::duplicate_field(
                                    "fuzzy_prefix_length",
                                ));
                            }
                            seen_fuzzy_prefix_length = true;
                            f_fuzzy_prefix_length = map.next_value::<Option<i32>>()?;
                        }
                        "fuzzy_rewrite" => {
                            if seen_fuzzy_rewrite {
                                return Err(serde::de::Error::duplicate_field("fuzzy_rewrite"));
                            }
                            seen_fuzzy_rewrite = true;
                            f_fuzzy_rewrite = map
                                .next_value::<Option<crate::types::common::MultiTermQueryRewrite>>(
                                )?;
                        }
                        "fuzzy_transpositions" => {
                            if seen_fuzzy_transpositions {
                                return Err(serde::de::Error::duplicate_field(
                                    "fuzzy_transpositions",
                                ));
                            }
                            seen_fuzzy_transpositions = true;
                            f_fuzzy_transpositions = map.next_value::<Option<bool>>()?;
                        }
                        "lenient" => {
                            if seen_lenient {
                                return Err(serde::de::Error::duplicate_field("lenient"));
                            }
                            seen_lenient = true;
                            f_lenient = map.next_value::<Option<bool>>()?;
                        }
                        "max_determinized_states" => {
                            if seen_max_determinized_states {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_determinized_states",
                                ));
                            }
                            seen_max_determinized_states = true;
                            f_max_determinized_states = map.next_value::<Option<i32>>()?;
                        }
                        "minimum_should_match" => {
                            if seen_minimum_should_match {
                                return Err(serde::de::Error::duplicate_field(
                                    "minimum_should_match",
                                ));
                            }
                            seen_minimum_should_match = true;
                            f_minimum_should_match = map
                                .next_value::<Option<crate::types::common::MinimumShouldMatch>>()?;
                        }
                        "phrase_slop" => {
                            if seen_phrase_slop {
                                return Err(serde::de::Error::duplicate_field("phrase_slop"));
                            }
                            seen_phrase_slop = true;
                            f_phrase_slop = map.next_value::<Option<i32>>()?;
                        }
                        "query" => {
                            if f_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            f_query = Some(map.next_value::<String>()?);
                        }
                        "quote_analyzer" => {
                            if seen_quote_analyzer {
                                return Err(serde::de::Error::duplicate_field("quote_analyzer"));
                            }
                            seen_quote_analyzer = true;
                            f_quote_analyzer = map.next_value::<Option<String>>()?;
                        }
                        "quote_field_suffix" => {
                            if seen_quote_field_suffix {
                                return Err(serde::de::Error::duplicate_field(
                                    "quote_field_suffix",
                                ));
                            }
                            seen_quote_field_suffix = true;
                            f_quote_field_suffix = map.next_value::<Option<String>>()?;
                        }
                        "rewrite" => {
                            if seen_rewrite {
                                return Err(serde::de::Error::duplicate_field("rewrite"));
                            }
                            seen_rewrite = true;
                            f_rewrite = map
                                .next_value::<Option<crate::types::common::MultiTermQueryRewrite>>(
                                )?;
                        }
                        "tie_breaker" => {
                            if seen_tie_breaker {
                                return Err(serde::de::Error::duplicate_field("tie_breaker"));
                            }
                            seen_tie_breaker = true;
                            f_tie_breaker = map.next_value::<Option<f64>>()?;
                        }
                        "time_zone" => {
                            if seen_time_zone {
                                return Err(serde::de::Error::duplicate_field("time_zone"));
                            }
                            seen_time_zone = true;
                            f_time_zone =
                                map.next_value::<Option<crate::types::common::TimeZone>>()?;
                        }
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<crate::types::common_query_dsl::TextQueryType>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(QueryStringQuery {
                    name: f_name,
                    boost: f_boost,
                    allow_leading_wildcard: f_allow_leading_wildcard,
                    analyze_wildcard: f_analyze_wildcard,
                    analyzer: f_analyzer,
                    auto_generate_synonyms_phrase_query: f_auto_generate_synonyms_phrase_query,
                    default_field: f_default_field,
                    default_operator: f_default_operator,
                    enable_position_increments: f_enable_position_increments,
                    escape: f_escape,
                    fields: f_fields,
                    fuzziness: f_fuzziness,
                    fuzzy_max_expansions: f_fuzzy_max_expansions,
                    fuzzy_prefix_length: f_fuzzy_prefix_length,
                    fuzzy_rewrite: f_fuzzy_rewrite,
                    fuzzy_transpositions: f_fuzzy_transpositions,
                    lenient: f_lenient,
                    max_determinized_states: f_max_determinized_states,
                    minimum_should_match: f_minimum_should_match,
                    phrase_slop: f_phrase_slop,
                    query: f_query.ok_or_else(|| serde::de::Error::missing_field("query"))?,
                    quote_analyzer: f_quote_analyzer,
                    quote_field_suffix: f_quote_field_suffix,
                    rewrite: f_rewrite,
                    tie_breaker: f_tie_breaker,
                    time_zone: f_time_zone,
                    r#type: f_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for QueryStringQuery {
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
        if let Some(v) = &self.allow_leading_wildcard {
            map.serialize_entry("allow_leading_wildcard", v)?;
        }
        if let Some(v) = &self.analyze_wildcard {
            map.serialize_entry("analyze_wildcard", v)?;
        }
        if let Some(v) = &self.analyzer {
            map.serialize_entry("analyzer", v)?;
        }
        if let Some(v) = &self.auto_generate_synonyms_phrase_query {
            map.serialize_entry("auto_generate_synonyms_phrase_query", v)?;
        }
        if let Some(v) = &self.default_field {
            map.serialize_entry("default_field", v)?;
        }
        if let Some(v) = &self.default_operator {
            map.serialize_entry("default_operator", v)?;
        }
        if let Some(v) = &self.enable_position_increments {
            map.serialize_entry("enable_position_increments", v)?;
        }
        if let Some(v) = &self.escape {
            map.serialize_entry("escape", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.fuzziness {
            map.serialize_entry("fuzziness", v)?;
        }
        if let Some(v) = &self.fuzzy_max_expansions {
            map.serialize_entry("fuzzy_max_expansions", v)?;
        }
        if let Some(v) = &self.fuzzy_prefix_length {
            map.serialize_entry("fuzzy_prefix_length", v)?;
        }
        if let Some(v) = &self.fuzzy_rewrite {
            map.serialize_entry("fuzzy_rewrite", v)?;
        }
        if let Some(v) = &self.fuzzy_transpositions {
            map.serialize_entry("fuzzy_transpositions", v)?;
        }
        if let Some(v) = &self.lenient {
            map.serialize_entry("lenient", v)?;
        }
        if let Some(v) = &self.max_determinized_states {
            map.serialize_entry("max_determinized_states", v)?;
        }
        if let Some(v) = &self.minimum_should_match {
            map.serialize_entry("minimum_should_match", v)?;
        }
        if let Some(v) = &self.phrase_slop {
            map.serialize_entry("phrase_slop", v)?;
        }
        map.serialize_entry("query", &self.query)?;
        if let Some(v) = &self.quote_analyzer {
            map.serialize_entry("quote_analyzer", v)?;
        }
        if let Some(v) = &self.quote_field_suffix {
            map.serialize_entry("quote_field_suffix", v)?;
        }
        if let Some(v) = &self.rewrite {
            map.serialize_entry("rewrite", v)?;
        }
        if let Some(v) = &self.tie_breaker {
            map.serialize_entry("tie_breaker", v)?;
        }
        if let Some(v) = &self.time_zone {
            map.serialize_entry("time_zone", v)?;
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

pub type QueryVector = Vec<f64>;

#[derive(Debug, Clone)]
pub struct RandomScoreFunction {
    pub field: Option<crate::types::common::Field>,
    pub seed: Option<crate::types::common_query_dsl::RandomScoreFunctionSeed>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RandomScoreFunction {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RandomScoreFunction;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RandomScoreFunction object")
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
                let mut seen_field = false;
                let mut f_seed: Option<crate::types::common_query_dsl::RandomScoreFunctionSeed> =
                    None;
                let mut seen_seed = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "field" => {
                            if seen_field {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            seen_field = true;
                            f_field = map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        "seed" => {
                            if seen_seed {
                                return Err(serde::de::Error::duplicate_field("seed"));
                            }
                            seen_seed = true;
                            f_seed = map.next_value::<Option<crate::types::common_query_dsl::RandomScoreFunctionSeed>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RandomScoreFunction {
                    field: f_field,
                    seed: f_seed,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RandomScoreFunction {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.field {
            map.serialize_entry("field", v)?;
        }
        if let Some(v) = &self.seed {
            map.serialize_entry("seed", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RangeRelation {
    #[doc = "`contains`"]
    Contains,
    #[doc = "`intersects`"]
    Intersects,
    #[doc = "`within`"]
    Within,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl RangeRelation {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            RangeRelation::Contains => "contains",
            RangeRelation::Intersects => "intersects",
            RangeRelation::Within => "within",
            RangeRelation::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for RangeRelation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RangeRelation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "contains" => RangeRelation::Contains,
            "intersects" => RangeRelation::Intersects,
            "within" => RangeRelation::Within,
            _ => RangeRelation::Unknown(s),
        })
    }
}

pub type RankFeatureFunctionLinear = std::collections::BTreeMap<String, serde_json::Value>;

#[derive(Debug, Clone)]
pub struct RankFeatureFunctionLogarithm {
    #[doc = "Configurable scaling factor."]
    pub scaling_factor: f64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RankFeatureFunctionLogarithm {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RankFeatureFunctionLogarithm;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RankFeatureFunctionLogarithm object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_scaling_factor: Option<f64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "scaling_factor" => {
                            if f_scaling_factor.is_some() {
                                return Err(serde::de::Error::duplicate_field("scaling_factor"));
                            }
                            f_scaling_factor = Some(map.next_value::<f64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RankFeatureFunctionLogarithm {
                    scaling_factor: f_scaling_factor
                        .ok_or_else(|| serde::de::Error::missing_field("scaling_factor"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RankFeatureFunctionLogarithm {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("scaling_factor", &self.scaling_factor)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RankFeatureFunctionSaturation {
    #[doc = "Configurable pivot value so that the result will be less than 0.5."]
    pub pivot: Option<f64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RankFeatureFunctionSaturation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RankFeatureFunctionSaturation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RankFeatureFunctionSaturation object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_pivot: Option<f64> = None;
                let mut seen_pivot = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "pivot" => {
                            if seen_pivot {
                                return Err(serde::de::Error::duplicate_field("pivot"));
                            }
                            seen_pivot = true;
                            f_pivot = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RankFeatureFunctionSaturation {
                    pivot: f_pivot,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RankFeatureFunctionSaturation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.pivot {
            map.serialize_entry("pivot", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RankFeatureFunctionSigmoid {
    #[doc = "Configurable Exponent."]
    pub exponent: f64,
    #[doc = "Configurable pivot value so that the result will be less than 0.5."]
    pub pivot: f64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RankFeatureFunctionSigmoid {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RankFeatureFunctionSigmoid;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RankFeatureFunctionSigmoid object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_exponent: Option<f64> = None;
                let mut f_pivot: Option<f64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "exponent" => {
                            if f_exponent.is_some() {
                                return Err(serde::de::Error::duplicate_field("exponent"));
                            }
                            f_exponent = Some(map.next_value::<f64>()?);
                        }
                        "pivot" => {
                            if f_pivot.is_some() {
                                return Err(serde::de::Error::duplicate_field("pivot"));
                            }
                            f_pivot = Some(map.next_value::<f64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RankFeatureFunctionSigmoid {
                    exponent: f_exponent
                        .ok_or_else(|| serde::de::Error::missing_field("exponent"))?,
                    pivot: f_pivot.ok_or_else(|| serde::de::Error::missing_field("pivot"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RankFeatureFunctionSigmoid {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("exponent", &self.exponent)?;
        map.serialize_entry("pivot", &self.pivot)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RankFeatureQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub field: crate::types::common::Field,
    pub linear: Option<crate::types::common_query_dsl::RankFeatureFunctionLinear>,
    pub log: Option<crate::types::common_query_dsl::RankFeatureFunctionLogarithm>,
    pub saturation: Option<crate::types::common_query_dsl::RankFeatureFunctionSaturation>,
    pub sigmoid: Option<crate::types::common_query_dsl::RankFeatureFunctionSigmoid>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RankFeatureQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RankFeatureQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RankFeatureQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_linear: Option<
                    crate::types::common_query_dsl::RankFeatureFunctionLinear,
                > = None;
                let mut seen_linear = false;
                let mut f_log: Option<
                    crate::types::common_query_dsl::RankFeatureFunctionLogarithm,
                > = None;
                let mut seen_log = false;
                let mut f_saturation: Option<
                    crate::types::common_query_dsl::RankFeatureFunctionSaturation,
                > = None;
                let mut seen_saturation = false;
                let mut f_sigmoid: Option<
                    crate::types::common_query_dsl::RankFeatureFunctionSigmoid,
                > = None;
                let mut seen_sigmoid = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "linear" => {
                            if seen_linear {
                                return Err(serde::de::Error::duplicate_field("linear"));
                            }
                            seen_linear = true;
                            f_linear =
                                map.next_value::<Option<
                                    crate::types::common_query_dsl::RankFeatureFunctionLinear,
                                >>()?;
                        }
                        "log" => {
                            if seen_log {
                                return Err(serde::de::Error::duplicate_field("log"));
                            }
                            seen_log = true;
                            f_log = map.next_value::<Option<
                                crate::types::common_query_dsl::RankFeatureFunctionLogarithm,
                            >>()?;
                        }
                        "saturation" => {
                            if seen_saturation {
                                return Err(serde::de::Error::duplicate_field("saturation"));
                            }
                            seen_saturation = true;
                            f_saturation = map.next_value::<Option<
                                crate::types::common_query_dsl::RankFeatureFunctionSaturation,
                            >>()?;
                        }
                        "sigmoid" => {
                            if seen_sigmoid {
                                return Err(serde::de::Error::duplicate_field("sigmoid"));
                            }
                            seen_sigmoid = true;
                            f_sigmoid = map.next_value::<Option<
                                crate::types::common_query_dsl::RankFeatureFunctionSigmoid,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RankFeatureQuery {
                    name: f_name,
                    boost: f_boost,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    linear: f_linear,
                    log: f_log,
                    saturation: f_saturation,
                    sigmoid: f_sigmoid,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RankFeatureQuery {
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
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.linear {
            map.serialize_entry("linear", v)?;
        }
        if let Some(v) = &self.log {
            map.serialize_entry("log", v)?;
        }
        if let Some(v) = &self.saturation {
            map.serialize_entry("saturation", v)?;
        }
        if let Some(v) = &self.sigmoid {
            map.serialize_entry("sigmoid", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'"') => serde_json::from_str(text).map(RegexpQuery::Value),
            Some(b'{') => serde_json::from_str(text).map(RegexpQuery::Object),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a RegexpQuery value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct RegexpQueryObject {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "Allows case insensitive matching of the regular expression value with the indexed field values when set to `true`. When `false`, case sensitivity of matching depends on the underlying field's mapping."]
    pub case_insensitive: Option<bool>,
    #[doc = "Enables optional operators for the regular expression."]
    pub flags: Option<String>,
    #[doc = "Maximum number of automaton states required for the query."]
    pub max_determinized_states: Option<i32>,
    pub rewrite: Option<crate::types::common::MultiTermQueryRewrite>,
    #[doc = "Regular expression for terms you wish to find in the provided field."]
    pub value: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RegexpQueryObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RegexpQueryObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RegexpQueryObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_case_insensitive: Option<bool> = None;
                let mut seen_case_insensitive = false;
                let mut f_flags: Option<String> = None;
                let mut seen_flags = false;
                let mut f_max_determinized_states: Option<i32> = None;
                let mut seen_max_determinized_states = false;
                let mut f_rewrite: Option<crate::types::common::MultiTermQueryRewrite> = None;
                let mut seen_rewrite = false;
                let mut f_value: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "case_insensitive" => {
                            if seen_case_insensitive {
                                return Err(serde::de::Error::duplicate_field("case_insensitive"));
                            }
                            seen_case_insensitive = true;
                            f_case_insensitive = map.next_value::<Option<bool>>()?;
                        }
                        "flags" => {
                            if seen_flags {
                                return Err(serde::de::Error::duplicate_field("flags"));
                            }
                            seen_flags = true;
                            f_flags = map.next_value::<Option<String>>()?;
                        }
                        "max_determinized_states" => {
                            if seen_max_determinized_states {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_determinized_states",
                                ));
                            }
                            seen_max_determinized_states = true;
                            f_max_determinized_states = map.next_value::<Option<i32>>()?;
                        }
                        "rewrite" => {
                            if seen_rewrite {
                                return Err(serde::de::Error::duplicate_field("rewrite"));
                            }
                            seen_rewrite = true;
                            f_rewrite = map
                                .next_value::<Option<crate::types::common::MultiTermQueryRewrite>>(
                                )?;
                        }
                        "value" => {
                            if f_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            f_value = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RegexpQueryObject {
                    name: f_name,
                    boost: f_boost,
                    case_insensitive: f_case_insensitive,
                    flags: f_flags,
                    max_determinized_states: f_max_determinized_states,
                    rewrite: f_rewrite,
                    value: f_value.ok_or_else(|| serde::de::Error::missing_field("value"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RegexpQueryObject {
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
        if let Some(v) = &self.case_insensitive {
            map.serialize_entry("case_insensitive", v)?;
        }
        if let Some(v) = &self.flags {
            map.serialize_entry("flags", v)?;
        }
        if let Some(v) = &self.max_determinized_states {
            map.serialize_entry("max_determinized_states", v)?;
        }
        if let Some(v) = &self.rewrite {
            map.serialize_entry("rewrite", v)?;
        }
        map.serialize_entry("value", &self.value)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RescoreContext {
    pub oversample_factor: Option<f64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RescoreContext {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RescoreContext;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RescoreContext object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_oversample_factor: Option<f64> = None;
                let mut seen_oversample_factor = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "oversample_factor" => {
                            if seen_oversample_factor {
                                return Err(serde::de::Error::duplicate_field("oversample_factor"));
                            }
                            seen_oversample_factor = true;
                            f_oversample_factor = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RescoreContext {
                    oversample_factor: f_oversample_factor,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RescoreContext {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.oversample_factor {
            map.serialize_entry("oversample_factor", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ScriptQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub script: crate::types::common::Script,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ScriptQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ScriptQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ScriptQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_script: Option<crate::types::common::Script> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "script" => {
                            if f_script.is_some() {
                                return Err(serde::de::Error::duplicate_field("script"));
                            }
                            f_script = Some(map.next_value::<crate::types::common::Script>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ScriptQuery {
                    name: f_name,
                    boost: f_boost,
                    script: f_script.ok_or_else(|| serde::de::Error::missing_field("script"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ScriptQuery {
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
        map.serialize_entry("script", &self.script)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ScriptScoreFunction {
    pub script: crate::types::common::Script,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ScriptScoreFunction {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ScriptScoreFunction;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ScriptScoreFunction object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_script: Option<crate::types::common::Script> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "script" => {
                            if f_script.is_some() {
                                return Err(serde::de::Error::duplicate_field("script"));
                            }
                            f_script = Some(map.next_value::<crate::types::common::Script>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ScriptScoreFunction {
                    script: f_script.ok_or_else(|| serde::de::Error::missing_field("script"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ScriptScoreFunction {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("script", &self.script)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ScriptScoreQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "Documents with a score lower than this floating point number are excluded from the search results."]
    pub min_score: Option<f64>,
    pub query: Box<crate::types::common_query_dsl::QueryContainer>,
    pub script: crate::types::common::Script,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ScriptScoreQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ScriptScoreQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ScriptScoreQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_min_score: Option<f64> = None;
                let mut seen_min_score = false;
                let mut f_query: Option<Box<crate::types::common_query_dsl::QueryContainer>> = None;
                let mut f_script: Option<crate::types::common::Script> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "min_score" => {
                            if seen_min_score {
                                return Err(serde::de::Error::duplicate_field("min_score"));
                            }
                            seen_min_score = true;
                            f_min_score = map.next_value::<Option<f64>>()?;
                        }
                        "query" => {
                            if f_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            f_query = Some(map.next_value::<Box<crate::types::common_query_dsl::QueryContainer>>()?);
                        }
                        "script" => {
                            if f_script.is_some() {
                                return Err(serde::de::Error::duplicate_field("script"));
                            }
                            f_script = Some(map.next_value::<crate::types::common::Script>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ScriptScoreQuery {
                    name: f_name,
                    boost: f_boost,
                    min_score: f_min_score,
                    query: f_query.ok_or_else(|| serde::de::Error::missing_field("query"))?,
                    script: f_script.ok_or_else(|| serde::de::Error::missing_field("script"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ScriptScoreQuery {
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
        if let Some(v) = &self.min_score {
            map.serialize_entry("min_score", v)?;
        }
        map.serialize_entry("query", &self.query)?;
        map.serialize_entry("script", &self.script)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimpleQueryStringFlag {
    #[doc = "`ALL`"]
    All,
    #[doc = "`AND`"]
    And,
    #[doc = "`ESCAPE`"]
    Escape,
    #[doc = "`FUZZY`"]
    Fuzzy,
    #[doc = "`NEAR`"]
    Near,
    #[doc = "`NONE`"]
    None,
    #[doc = "`NOT`"]
    Not,
    #[doc = "`OR`"]
    Or,
    #[doc = "`PHRASE`"]
    Phrase,
    #[doc = "`PRECEDENCE`"]
    Precedence,
    #[doc = "`PREFIX`"]
    Prefix,
    #[doc = "`SLOP`"]
    Slop,
    #[doc = "`WHITESPACE`"]
    Whitespace,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SimpleQueryStringFlag {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SimpleQueryStringFlag::All => "ALL",
            SimpleQueryStringFlag::And => "AND",
            SimpleQueryStringFlag::Escape => "ESCAPE",
            SimpleQueryStringFlag::Fuzzy => "FUZZY",
            SimpleQueryStringFlag::Near => "NEAR",
            SimpleQueryStringFlag::None => "NONE",
            SimpleQueryStringFlag::Not => "NOT",
            SimpleQueryStringFlag::Or => "OR",
            SimpleQueryStringFlag::Phrase => "PHRASE",
            SimpleQueryStringFlag::Precedence => "PRECEDENCE",
            SimpleQueryStringFlag::Prefix => "PREFIX",
            SimpleQueryStringFlag::Slop => "SLOP",
            SimpleQueryStringFlag::Whitespace => "WHITESPACE",
            SimpleQueryStringFlag::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SimpleQueryStringFlag {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SimpleQueryStringFlag {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ALL" => SimpleQueryStringFlag::All,
            "AND" => SimpleQueryStringFlag::And,
            "ESCAPE" => SimpleQueryStringFlag::Escape,
            "FUZZY" => SimpleQueryStringFlag::Fuzzy,
            "NEAR" => SimpleQueryStringFlag::Near,
            "NONE" => SimpleQueryStringFlag::None,
            "NOT" => SimpleQueryStringFlag::Not,
            "OR" => SimpleQueryStringFlag::Or,
            "PHRASE" => SimpleQueryStringFlag::Phrase,
            "PRECEDENCE" => SimpleQueryStringFlag::Precedence,
            "PREFIX" => SimpleQueryStringFlag::Prefix,
            "SLOP" => SimpleQueryStringFlag::Slop,
            "WHITESPACE" => SimpleQueryStringFlag::Whitespace,
            _ => SimpleQueryStringFlag::Unknown(s),
        })
    }
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

#[derive(Debug, Clone)]
pub struct SimpleQueryStringQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "If `true`, the query attempts to analyze wildcard terms in the query string."]
    pub analyze_wildcard: Option<bool>,
    #[doc = "Analyzer used to convert text in the query string into tokens."]
    pub analyzer: Option<String>,
    #[doc = "If `true`, the parser creates a `match_phrase` query for each multi-position token."]
    pub auto_generate_synonyms_phrase_query: Option<bool>,
    pub default_operator: Option<crate::types::common_query_dsl::Operator>,
    #[doc = "Array of fields you wish to search. Accepts wildcard expressions. You also can boost relevance scores for matches to particular fields using a caret (`^`) notation. Defaults to the `index.query.default_field index` setting, which has a default value of `*`."]
    pub fields: Option<Vec<crate::types::common::Field>>,
    pub flags: Option<crate::types::common_query_dsl::SimpleQueryStringFlags>,
    #[doc = "Maximum number of terms to which the query expands for fuzzy matching."]
    pub fuzzy_max_expansions: Option<i32>,
    #[doc = "Number of beginning characters left unchanged for fuzzy matching."]
    pub fuzzy_prefix_length: Option<i32>,
    #[doc = "If `true`, edits for fuzzy matching include transpositions of two adjacent characters (for example, `ab` to `ba`)."]
    pub fuzzy_transpositions: Option<bool>,
    #[doc = "If `true`, format-based errors, such as providing a text value for a numeric field, are ignored."]
    pub lenient: Option<bool>,
    pub minimum_should_match: Option<crate::types::common::MinimumShouldMatch>,
    #[doc = "Query string in the simple query string syntax you wish to parse and use for search."]
    pub query: String,
    #[doc = "Suffix appended to quoted text in the query string."]
    pub quote_field_suffix: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SimpleQueryStringQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SimpleQueryStringQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SimpleQueryStringQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_analyze_wildcard: Option<bool> = None;
                let mut seen_analyze_wildcard = false;
                let mut f_analyzer: Option<String> = None;
                let mut seen_analyzer = false;
                let mut f_auto_generate_synonyms_phrase_query: Option<bool> = None;
                let mut seen_auto_generate_synonyms_phrase_query = false;
                let mut f_default_operator: Option<crate::types::common_query_dsl::Operator> = None;
                let mut seen_default_operator = false;
                let mut f_fields: Option<Vec<crate::types::common::Field>> = None;
                let mut seen_fields = false;
                let mut f_flags: Option<crate::types::common_query_dsl::SimpleQueryStringFlags> =
                    None;
                let mut seen_flags = false;
                let mut f_fuzzy_max_expansions: Option<i32> = None;
                let mut seen_fuzzy_max_expansions = false;
                let mut f_fuzzy_prefix_length: Option<i32> = None;
                let mut seen_fuzzy_prefix_length = false;
                let mut f_fuzzy_transpositions: Option<bool> = None;
                let mut seen_fuzzy_transpositions = false;
                let mut f_lenient: Option<bool> = None;
                let mut seen_lenient = false;
                let mut f_minimum_should_match: Option<crate::types::common::MinimumShouldMatch> =
                    None;
                let mut seen_minimum_should_match = false;
                let mut f_query: Option<String> = None;
                let mut f_quote_field_suffix: Option<String> = None;
                let mut seen_quote_field_suffix = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "analyze_wildcard" => {
                            if seen_analyze_wildcard {
                                return Err(serde::de::Error::duplicate_field("analyze_wildcard"));
                            }
                            seen_analyze_wildcard = true;
                            f_analyze_wildcard = map.next_value::<Option<bool>>()?;
                        }
                        "analyzer" => {
                            if seen_analyzer {
                                return Err(serde::de::Error::duplicate_field("analyzer"));
                            }
                            seen_analyzer = true;
                            f_analyzer = map.next_value::<Option<String>>()?;
                        }
                        "auto_generate_synonyms_phrase_query" => {
                            if seen_auto_generate_synonyms_phrase_query {
                                return Err(serde::de::Error::duplicate_field(
                                    "auto_generate_synonyms_phrase_query",
                                ));
                            }
                            seen_auto_generate_synonyms_phrase_query = true;
                            f_auto_generate_synonyms_phrase_query =
                                map.next_value::<Option<bool>>()?;
                        }
                        "default_operator" => {
                            if seen_default_operator {
                                return Err(serde::de::Error::duplicate_field("default_operator"));
                            }
                            seen_default_operator = true;
                            f_default_operator = map
                                .next_value::<Option<crate::types::common_query_dsl::Operator>>()?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields =
                                map.next_value::<Option<Vec<crate::types::common::Field>>>()?;
                        }
                        "flags" => {
                            if seen_flags {
                                return Err(serde::de::Error::duplicate_field("flags"));
                            }
                            seen_flags = true;
                            f_flags = map.next_value::<Option<crate::types::common_query_dsl::SimpleQueryStringFlags>>()?;
                        }
                        "fuzzy_max_expansions" => {
                            if seen_fuzzy_max_expansions {
                                return Err(serde::de::Error::duplicate_field(
                                    "fuzzy_max_expansions",
                                ));
                            }
                            seen_fuzzy_max_expansions = true;
                            f_fuzzy_max_expansions = map.next_value::<Option<i32>>()?;
                        }
                        "fuzzy_prefix_length" => {
                            if seen_fuzzy_prefix_length {
                                return Err(serde::de::Error::duplicate_field(
                                    "fuzzy_prefix_length",
                                ));
                            }
                            seen_fuzzy_prefix_length = true;
                            f_fuzzy_prefix_length = map.next_value::<Option<i32>>()?;
                        }
                        "fuzzy_transpositions" => {
                            if seen_fuzzy_transpositions {
                                return Err(serde::de::Error::duplicate_field(
                                    "fuzzy_transpositions",
                                ));
                            }
                            seen_fuzzy_transpositions = true;
                            f_fuzzy_transpositions = map.next_value::<Option<bool>>()?;
                        }
                        "lenient" => {
                            if seen_lenient {
                                return Err(serde::de::Error::duplicate_field("lenient"));
                            }
                            seen_lenient = true;
                            f_lenient = map.next_value::<Option<bool>>()?;
                        }
                        "minimum_should_match" => {
                            if seen_minimum_should_match {
                                return Err(serde::de::Error::duplicate_field(
                                    "minimum_should_match",
                                ));
                            }
                            seen_minimum_should_match = true;
                            f_minimum_should_match = map
                                .next_value::<Option<crate::types::common::MinimumShouldMatch>>()?;
                        }
                        "query" => {
                            if f_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            f_query = Some(map.next_value::<String>()?);
                        }
                        "quote_field_suffix" => {
                            if seen_quote_field_suffix {
                                return Err(serde::de::Error::duplicate_field(
                                    "quote_field_suffix",
                                ));
                            }
                            seen_quote_field_suffix = true;
                            f_quote_field_suffix = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SimpleQueryStringQuery {
                    name: f_name,
                    boost: f_boost,
                    analyze_wildcard: f_analyze_wildcard,
                    analyzer: f_analyzer,
                    auto_generate_synonyms_phrase_query: f_auto_generate_synonyms_phrase_query,
                    default_operator: f_default_operator,
                    fields: f_fields,
                    flags: f_flags,
                    fuzzy_max_expansions: f_fuzzy_max_expansions,
                    fuzzy_prefix_length: f_fuzzy_prefix_length,
                    fuzzy_transpositions: f_fuzzy_transpositions,
                    lenient: f_lenient,
                    minimum_should_match: f_minimum_should_match,
                    query: f_query.ok_or_else(|| serde::de::Error::missing_field("query"))?,
                    quote_field_suffix: f_quote_field_suffix,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SimpleQueryStringQuery {
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
        if let Some(v) = &self.analyze_wildcard {
            map.serialize_entry("analyze_wildcard", v)?;
        }
        if let Some(v) = &self.analyzer {
            map.serialize_entry("analyzer", v)?;
        }
        if let Some(v) = &self.auto_generate_synonyms_phrase_query {
            map.serialize_entry("auto_generate_synonyms_phrase_query", v)?;
        }
        if let Some(v) = &self.default_operator {
            map.serialize_entry("default_operator", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.flags {
            map.serialize_entry("flags", v)?;
        }
        if let Some(v) = &self.fuzzy_max_expansions {
            map.serialize_entry("fuzzy_max_expansions", v)?;
        }
        if let Some(v) = &self.fuzzy_prefix_length {
            map.serialize_entry("fuzzy_prefix_length", v)?;
        }
        if let Some(v) = &self.fuzzy_transpositions {
            map.serialize_entry("fuzzy_transpositions", v)?;
        }
        if let Some(v) = &self.lenient {
            map.serialize_entry("lenient", v)?;
        }
        if let Some(v) = &self.minimum_should_match {
            map.serialize_entry("minimum_should_match", v)?;
        }
        map.serialize_entry("query", &self.query)?;
        if let Some(v) = &self.quote_field_suffix {
            map.serialize_entry("quote_field_suffix", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SpanContainingQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub big: Box<crate::types::common_query_dsl::SpanQuery>,
    pub little: Box<crate::types::common_query_dsl::SpanQuery>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SpanContainingQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SpanContainingQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SpanContainingQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_big: Option<Box<crate::types::common_query_dsl::SpanQuery>> = None;
                let mut f_little: Option<Box<crate::types::common_query_dsl::SpanQuery>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "big" => {
                            if f_big.is_some() {
                                return Err(serde::de::Error::duplicate_field("big"));
                            }
                            f_big = Some(
                                map.next_value::<Box<crate::types::common_query_dsl::SpanQuery>>()?,
                            );
                        }
                        "little" => {
                            if f_little.is_some() {
                                return Err(serde::de::Error::duplicate_field("little"));
                            }
                            f_little = Some(
                                map.next_value::<Box<crate::types::common_query_dsl::SpanQuery>>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SpanContainingQuery {
                    name: f_name,
                    boost: f_boost,
                    big: f_big.ok_or_else(|| serde::de::Error::missing_field("big"))?,
                    little: f_little.ok_or_else(|| serde::de::Error::missing_field("little"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SpanContainingQuery {
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
        map.serialize_entry("big", &self.big)?;
        map.serialize_entry("little", &self.little)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SpanFieldMaskingQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub field: crate::types::common::Field,
    pub query: Box<crate::types::common_query_dsl::SpanQuery>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SpanFieldMaskingQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SpanFieldMaskingQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SpanFieldMaskingQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut f_query: Option<Box<crate::types::common_query_dsl::SpanQuery>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "query" => {
                            if f_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            f_query = Some(
                                map.next_value::<Box<crate::types::common_query_dsl::SpanQuery>>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SpanFieldMaskingQuery {
                    name: f_name,
                    boost: f_boost,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    query: f_query.ok_or_else(|| serde::de::Error::missing_field("query"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SpanFieldMaskingQuery {
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
        map.serialize_entry("field", &self.field)?;
        map.serialize_entry("query", &self.query)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SpanFirstQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "Controls the maximum end position permitted in a match."]
    pub end: i32,
    pub r#match: Box<crate::types::common_query_dsl::SpanQuery>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SpanFirstQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SpanFirstQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SpanFirstQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_end: Option<i32> = None;
                let mut f_match: Option<Box<crate::types::common_query_dsl::SpanQuery>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "end" => {
                            if f_end.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            f_end = Some(map.next_value::<i32>()?);
                        }
                        "match" => {
                            if f_match.is_some() {
                                return Err(serde::de::Error::duplicate_field("match"));
                            }
                            f_match = Some(
                                map.next_value::<Box<crate::types::common_query_dsl::SpanQuery>>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SpanFirstQuery {
                    name: f_name,
                    boost: f_boost,
                    end: f_end.ok_or_else(|| serde::de::Error::missing_field("end"))?,
                    r#match: f_match.ok_or_else(|| serde::de::Error::missing_field("match"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SpanFirstQuery {
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
        map.serialize_entry("end", &self.end)?;
        map.serialize_entry("match", &self.r#match)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Can only be used as a clause in a `span_near` query."]
pub type SpanGapQuery = std::collections::BTreeMap<String, i32>;

#[derive(Debug, Clone)]
pub struct SpanMultiTermQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub r#match: Box<crate::types::common_query_dsl::QueryContainer>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SpanMultiTermQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SpanMultiTermQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SpanMultiTermQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_match: Option<Box<crate::types::common_query_dsl::QueryContainer>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "match" => {
                            if f_match.is_some() {
                                return Err(serde::de::Error::duplicate_field("match"));
                            }
                            f_match = Some(map.next_value::<Box<crate::types::common_query_dsl::QueryContainer>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SpanMultiTermQuery {
                    name: f_name,
                    boost: f_boost,
                    r#match: f_match.ok_or_else(|| serde::de::Error::missing_field("match"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SpanMultiTermQuery {
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
        map.serialize_entry("match", &self.r#match)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SpanNearQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "Array of one or more other span type queries."]
    pub clauses: Vec<crate::types::common_query_dsl::SpanQuery>,
    #[doc = "Controls whether matches are required to be in-order."]
    pub in_order: Option<bool>,
    #[doc = "Controls the maximum number of intervening unmatched positions permitted."]
    pub slop: Option<i32>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SpanNearQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SpanNearQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SpanNearQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_clauses: Option<Vec<crate::types::common_query_dsl::SpanQuery>> = None;
                let mut f_in_order: Option<bool> = None;
                let mut seen_in_order = false;
                let mut f_slop: Option<i32> = None;
                let mut seen_slop = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "clauses" => {
                            if f_clauses.is_some() {
                                return Err(serde::de::Error::duplicate_field("clauses"));
                            }
                            f_clauses = Some(
                                map.next_value::<Vec<crate::types::common_query_dsl::SpanQuery>>()?,
                            );
                        }
                        "in_order" => {
                            if seen_in_order {
                                return Err(serde::de::Error::duplicate_field("in_order"));
                            }
                            seen_in_order = true;
                            f_in_order = map.next_value::<Option<bool>>()?;
                        }
                        "slop" => {
                            if seen_slop {
                                return Err(serde::de::Error::duplicate_field("slop"));
                            }
                            seen_slop = true;
                            f_slop = map.next_value::<Option<i32>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SpanNearQuery {
                    name: f_name,
                    boost: f_boost,
                    clauses: f_clauses.ok_or_else(|| serde::de::Error::missing_field("clauses"))?,
                    in_order: f_in_order,
                    slop: f_slop,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SpanNearQuery {
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
        map.serialize_entry("clauses", &self.clauses)?;
        if let Some(v) = &self.in_order {
            map.serialize_entry("in_order", v)?;
        }
        if let Some(v) = &self.slop {
            map.serialize_entry("slop", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SpanNotQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "The number of tokens from within the include span that can't have overlap with the exclude span. Equivalent to setting both `pre` and `post`."]
    pub dist: Option<i32>,
    pub exclude: Box<crate::types::common_query_dsl::SpanQuery>,
    pub include: Box<crate::types::common_query_dsl::SpanQuery>,
    #[doc = "The number of tokens after the include span that can't have overlap with the exclude span."]
    pub post: Option<i32>,
    #[doc = "The number of tokens before the include span that can't have overlap with the exclude span."]
    pub pre: Option<i32>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SpanNotQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SpanNotQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SpanNotQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_dist: Option<i32> = None;
                let mut seen_dist = false;
                let mut f_exclude: Option<Box<crate::types::common_query_dsl::SpanQuery>> = None;
                let mut f_include: Option<Box<crate::types::common_query_dsl::SpanQuery>> = None;
                let mut f_post: Option<i32> = None;
                let mut seen_post = false;
                let mut f_pre: Option<i32> = None;
                let mut seen_pre = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "dist" => {
                            if seen_dist {
                                return Err(serde::de::Error::duplicate_field("dist"));
                            }
                            seen_dist = true;
                            f_dist = map.next_value::<Option<i32>>()?;
                        }
                        "exclude" => {
                            if f_exclude.is_some() {
                                return Err(serde::de::Error::duplicate_field("exclude"));
                            }
                            f_exclude = Some(
                                map.next_value::<Box<crate::types::common_query_dsl::SpanQuery>>()?,
                            );
                        }
                        "include" => {
                            if f_include.is_some() {
                                return Err(serde::de::Error::duplicate_field("include"));
                            }
                            f_include = Some(
                                map.next_value::<Box<crate::types::common_query_dsl::SpanQuery>>()?,
                            );
                        }
                        "post" => {
                            if seen_post {
                                return Err(serde::de::Error::duplicate_field("post"));
                            }
                            seen_post = true;
                            f_post = map.next_value::<Option<i32>>()?;
                        }
                        "pre" => {
                            if seen_pre {
                                return Err(serde::de::Error::duplicate_field("pre"));
                            }
                            seen_pre = true;
                            f_pre = map.next_value::<Option<i32>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SpanNotQuery {
                    name: f_name,
                    boost: f_boost,
                    dist: f_dist,
                    exclude: f_exclude.ok_or_else(|| serde::de::Error::missing_field("exclude"))?,
                    include: f_include.ok_or_else(|| serde::de::Error::missing_field("include"))?,
                    post: f_post,
                    pre: f_pre,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SpanNotQuery {
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
        if let Some(v) = &self.dist {
            map.serialize_entry("dist", v)?;
        }
        map.serialize_entry("exclude", &self.exclude)?;
        map.serialize_entry("include", &self.include)?;
        if let Some(v) = &self.post {
            map.serialize_entry("post", v)?;
        }
        if let Some(v) = &self.pre {
            map.serialize_entry("pre", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SpanOrQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "Array of one or more other span type queries."]
    pub clauses: Vec<crate::types::common_query_dsl::SpanQuery>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SpanOrQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SpanOrQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SpanOrQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_clauses: Option<Vec<crate::types::common_query_dsl::SpanQuery>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "clauses" => {
                            if f_clauses.is_some() {
                                return Err(serde::de::Error::duplicate_field("clauses"));
                            }
                            f_clauses = Some(
                                map.next_value::<Vec<crate::types::common_query_dsl::SpanQuery>>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SpanOrQuery {
                    name: f_name,
                    boost: f_boost,
                    clauses: f_clauses.ok_or_else(|| serde::de::Error::missing_field("clauses"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SpanOrQuery {
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
        map.serialize_entry("clauses", &self.clauses)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SpanQuery {
    pub field_masking_span: Option<Box<crate::types::common_query_dsl::SpanFieldMaskingQuery>>,
    pub span_containing: Option<Box<crate::types::common_query_dsl::SpanContainingQuery>>,
    pub span_first: Option<Box<crate::types::common_query_dsl::SpanFirstQuery>>,
    pub span_gap: Option<crate::types::common_query_dsl::SpanGapQuery>,
    pub span_multi: Option<Box<crate::types::common_query_dsl::SpanMultiTermQuery>>,
    pub span_near: Option<crate::types::common_query_dsl::SpanNearQuery>,
    pub span_not: Option<Box<crate::types::common_query_dsl::SpanNotQuery>>,
    pub span_or: Option<crate::types::common_query_dsl::SpanOrQuery>,
    #[doc = "The equivalent of the `term` query but for use with other span queries."]
    pub span_term:
        Option<std::collections::BTreeMap<String, crate::types::common_query_dsl::SpanTermQuery>>,
    pub span_within: Option<Box<crate::types::common_query_dsl::SpanWithinQuery>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SpanQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SpanQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SpanQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_field_masking_span: Option<
                    Box<crate::types::common_query_dsl::SpanFieldMaskingQuery>,
                > = None;
                let mut seen_field_masking_span = false;
                let mut f_span_containing: Option<
                    Box<crate::types::common_query_dsl::SpanContainingQuery>,
                > = None;
                let mut seen_span_containing = false;
                let mut f_span_first: Option<Box<crate::types::common_query_dsl::SpanFirstQuery>> =
                    None;
                let mut seen_span_first = false;
                let mut f_span_gap: Option<crate::types::common_query_dsl::SpanGapQuery> = None;
                let mut seen_span_gap = false;
                let mut f_span_multi: Option<
                    Box<crate::types::common_query_dsl::SpanMultiTermQuery>,
                > = None;
                let mut seen_span_multi = false;
                let mut f_span_near: Option<crate::types::common_query_dsl::SpanNearQuery> = None;
                let mut seen_span_near = false;
                let mut f_span_not: Option<Box<crate::types::common_query_dsl::SpanNotQuery>> =
                    None;
                let mut seen_span_not = false;
                let mut f_span_or: Option<crate::types::common_query_dsl::SpanOrQuery> = None;
                let mut seen_span_or = false;
                let mut f_span_term: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::common_query_dsl::SpanTermQuery,
                    >,
                > = None;
                let mut seen_span_term = false;
                let mut f_span_within: Option<
                    Box<crate::types::common_query_dsl::SpanWithinQuery>,
                > = None;
                let mut seen_span_within = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "field_masking_span" => {
                            if seen_field_masking_span {
                                return Err(serde::de::Error::duplicate_field(
                                    "field_masking_span",
                                ));
                            }
                            seen_field_masking_span = true;
                            f_field_masking_span = map.next_value::<Option<
                                Box<crate::types::common_query_dsl::SpanFieldMaskingQuery>,
                            >>()?;
                        }
                        "span_containing" => {
                            if seen_span_containing {
                                return Err(serde::de::Error::duplicate_field("span_containing"));
                            }
                            seen_span_containing = true;
                            f_span_containing =
                                map.next_value::<Option<
                                    Box<crate::types::common_query_dsl::SpanContainingQuery>,
                                >>()?;
                        }
                        "span_first" => {
                            if seen_span_first {
                                return Err(serde::de::Error::duplicate_field("span_first"));
                            }
                            seen_span_first = true;
                            f_span_first = map.next_value::<Option<Box<crate::types::common_query_dsl::SpanFirstQuery>>>()?;
                        }
                        "span_gap" => {
                            if seen_span_gap {
                                return Err(serde::de::Error::duplicate_field("span_gap"));
                            }
                            seen_span_gap = true;
                            f_span_gap = map
                                .next_value::<Option<crate::types::common_query_dsl::SpanGapQuery>>(
                                )?;
                        }
                        "span_multi" => {
                            if seen_span_multi {
                                return Err(serde::de::Error::duplicate_field("span_multi"));
                            }
                            seen_span_multi = true;
                            f_span_multi = map.next_value::<Option<Box<crate::types::common_query_dsl::SpanMultiTermQuery>>>()?;
                        }
                        "span_near" => {
                            if seen_span_near {
                                return Err(serde::de::Error::duplicate_field("span_near"));
                            }
                            seen_span_near = true;
                            f_span_near = map.next_value::<Option<crate::types::common_query_dsl::SpanNearQuery>>()?;
                        }
                        "span_not" => {
                            if seen_span_not {
                                return Err(serde::de::Error::duplicate_field("span_not"));
                            }
                            seen_span_not = true;
                            f_span_not = map.next_value::<Option<Box<crate::types::common_query_dsl::SpanNotQuery>>>()?;
                        }
                        "span_or" => {
                            if seen_span_or {
                                return Err(serde::de::Error::duplicate_field("span_or"));
                            }
                            seen_span_or = true;
                            f_span_or = map
                                .next_value::<Option<crate::types::common_query_dsl::SpanOrQuery>>(
                                )?;
                        }
                        "span_term" => {
                            if seen_span_term {
                                return Err(serde::de::Error::duplicate_field("span_term"));
                            }
                            seen_span_term = true;
                            f_span_term = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_query_dsl::SpanTermQuery,
                                >,
                            >>()?;
                        }
                        "span_within" => {
                            if seen_span_within {
                                return Err(serde::de::Error::duplicate_field("span_within"));
                            }
                            seen_span_within = true;
                            f_span_within = map.next_value::<Option<Box<crate::types::common_query_dsl::SpanWithinQuery>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SpanQuery {
                    field_masking_span: f_field_masking_span,
                    span_containing: f_span_containing,
                    span_first: f_span_first,
                    span_gap: f_span_gap,
                    span_multi: f_span_multi,
                    span_near: f_span_near,
                    span_not: f_span_not,
                    span_or: f_span_or,
                    span_term: f_span_term,
                    span_within: f_span_within,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SpanQuery {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.field_masking_span {
            map.serialize_entry("field_masking_span", v)?;
        }
        if let Some(v) = &self.span_containing {
            map.serialize_entry("span_containing", v)?;
        }
        if let Some(v) = &self.span_first {
            map.serialize_entry("span_first", v)?;
        }
        if let Some(v) = &self.span_gap {
            map.serialize_entry("span_gap", v)?;
        }
        if let Some(v) = &self.span_multi {
            map.serialize_entry("span_multi", v)?;
        }
        if let Some(v) = &self.span_near {
            map.serialize_entry("span_near", v)?;
        }
        if let Some(v) = &self.span_not {
            map.serialize_entry("span_not", v)?;
        }
        if let Some(v) = &self.span_or {
            map.serialize_entry("span_or", v)?;
        }
        if let Some(v) = &self.span_term {
            map.serialize_entry("span_term", v)?;
        }
        if let Some(v) = &self.span_within {
            map.serialize_entry("span_within", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'"') => serde_json::from_str(text).map(SpanTermQuery::Value),
            Some(b'{') => serde_json::from_str(text).map(SpanTermQuery::Object),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a SpanTermQuery value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct SpanTermQueryObject {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub value: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SpanTermQueryObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SpanTermQueryObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SpanTermQueryObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_value: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "value" => {
                            if f_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            f_value = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SpanTermQueryObject {
                    name: f_name,
                    boost: f_boost,
                    value: f_value.ok_or_else(|| serde::de::Error::missing_field("value"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SpanTermQueryObject {
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
        map.serialize_entry("value", &self.value)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SpanWithinQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub big: Box<crate::types::common_query_dsl::SpanQuery>,
    pub little: Box<crate::types::common_query_dsl::SpanQuery>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SpanWithinQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SpanWithinQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SpanWithinQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_big: Option<Box<crate::types::common_query_dsl::SpanQuery>> = None;
                let mut f_little: Option<Box<crate::types::common_query_dsl::SpanQuery>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "big" => {
                            if f_big.is_some() {
                                return Err(serde::de::Error::duplicate_field("big"));
                            }
                            f_big = Some(
                                map.next_value::<Box<crate::types::common_query_dsl::SpanQuery>>()?,
                            );
                        }
                        "little" => {
                            if f_little.is_some() {
                                return Err(serde::de::Error::duplicate_field("little"));
                            }
                            f_little = Some(
                                map.next_value::<Box<crate::types::common_query_dsl::SpanQuery>>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SpanWithinQuery {
                    name: f_name,
                    boost: f_boost,
                    big: f_big.ok_or_else(|| serde::de::Error::missing_field("big"))?,
                    little: f_little.ok_or_else(|| serde::de::Error::missing_field("little"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SpanWithinQuery {
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
        map.serialize_entry("big", &self.big)?;
        map.serialize_entry("little", &self.little)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct TermQueryObject {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "Allows ASCII case insensitive matching of the value with the indexed field values when set to `true`. When `false`, the case sensitivity of matching depends on the underlying field's mapping."]
    pub case_insensitive: Option<bool>,
    pub value: crate::types::common::FieldValue,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TermQueryObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TermQueryObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TermQueryObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_case_insensitive: Option<bool> = None;
                let mut seen_case_insensitive = false;
                let mut f_value: Option<crate::types::common::FieldValue> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "case_insensitive" => {
                            if seen_case_insensitive {
                                return Err(serde::de::Error::duplicate_field("case_insensitive"));
                            }
                            seen_case_insensitive = true;
                            f_case_insensitive = map.next_value::<Option<bool>>()?;
                        }
                        "value" => {
                            if f_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            f_value = Some(map.next_value::<crate::types::common::FieldValue>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TermQueryObject {
                    name: f_name,
                    boost: f_boost,
                    case_insensitive: f_case_insensitive,
                    value: f_value.ok_or_else(|| serde::de::Error::missing_field("value"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TermQueryObject {
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
        if let Some(v) = &self.case_insensitive {
            map.serialize_entry("case_insensitive", v)?;
        }
        map.serialize_entry("value", &self.value)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TermsLookup {
    pub id: crate::types::common::Id,
    pub index: crate::types::common::IndexName,
    pub path: crate::types::common::Field,
    pub routing: Option<crate::types::common::Routing>,
    pub store: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TermsLookup {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TermsLookup;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TermsLookup object")
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
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut f_path: Option<crate::types::common::Field> = None;
                let mut f_routing: Option<crate::types::common::Routing> = None;
                let mut seen_routing = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
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
                        "index" => {
                            if f_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            f_index = Some(map.next_value::<crate::types::common::IndexName>()?);
                        }
                        "path" => {
                            if f_path.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            f_path = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("routing"));
                            }
                            seen_routing = true;
                            f_routing =
                                map.next_value::<Option<crate::types::common::Routing>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TermsLookup {
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("id"))?,
                    index: f_index.ok_or_else(|| serde::de::Error::missing_field("index"))?,
                    path: f_path.ok_or_else(|| serde::de::Error::missing_field("path"))?,
                    routing: f_routing,
                    store: f_store,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TermsLookup {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("index", &self.index)?;
        map.serialize_entry("path", &self.path)?;
        if let Some(v) = &self.routing {
            map.serialize_entry("routing", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_value_type: Option<crate::types::common_query_dsl::TermsQueryValueType> =
                    None;
                let mut seen_value_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_query_dsl::TermsQueryField,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "value_type" => {
                            if seen_value_type {
                                return Err(serde::de::Error::duplicate_field("value_type"));
                            }
                            seen_value_type = true;
                            f_value_type = map.next_value::<Option<crate::types::common_query_dsl::TermsQueryValueType>>()?;
                        }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'[') => serde_json::from_str(text).map(TermsQueryField::Value),
            Some(b'{') => serde_json::from_str(text).map(TermsQueryField::Lookup),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a TermsQueryField value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TermsQueryValueType {
    #[doc = "`bitmap`"]
    Bitmap,
    #[doc = "`default`"]
    Default,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TermsQueryValueType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TermsQueryValueType::Bitmap => "bitmap",
            TermsQueryValueType::Default => "default",
            TermsQueryValueType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TermsQueryValueType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TermsQueryValueType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "bitmap" => TermsQueryValueType::Bitmap,
            "default" => TermsQueryValueType::Default,
            _ => TermsQueryValueType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct TermsSetQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub minimum_should_match_field: Option<crate::types::common::Field>,
    pub minimum_should_match_script: Option<crate::types::common::Script>,
    pub terms: Vec<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TermsSetQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TermsSetQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TermsSetQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_minimum_should_match_field: Option<crate::types::common::Field> = None;
                let mut seen_minimum_should_match_field = false;
                let mut f_minimum_should_match_script: Option<crate::types::common::Script> = None;
                let mut seen_minimum_should_match_script = false;
                let mut f_terms: Option<Vec<String>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "minimum_should_match_field" => {
                            if seen_minimum_should_match_field {
                                return Err(serde::de::Error::duplicate_field(
                                    "minimum_should_match_field",
                                ));
                            }
                            seen_minimum_should_match_field = true;
                            f_minimum_should_match_field =
                                map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        "minimum_should_match_script" => {
                            if seen_minimum_should_match_script {
                                return Err(serde::de::Error::duplicate_field(
                                    "minimum_should_match_script",
                                ));
                            }
                            seen_minimum_should_match_script = true;
                            f_minimum_should_match_script =
                                map.next_value::<Option<crate::types::common::Script>>()?;
                        }
                        "terms" => {
                            if f_terms.is_some() {
                                return Err(serde::de::Error::duplicate_field("terms"));
                            }
                            f_terms = Some(map.next_value::<Vec<String>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TermsSetQuery {
                    name: f_name,
                    boost: f_boost,
                    minimum_should_match_field: f_minimum_should_match_field,
                    minimum_should_match_script: f_minimum_should_match_script,
                    terms: f_terms.ok_or_else(|| serde::de::Error::missing_field("terms"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TermsSetQuery {
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
        if let Some(v) = &self.minimum_should_match_field {
            map.serialize_entry("minimum_should_match_field", v)?;
        }
        if let Some(v) = &self.minimum_should_match_script {
            map.serialize_entry("minimum_should_match_script", v)?;
        }
        map.serialize_entry("terms", &self.terms)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextQueryType {
    #[doc = "`best_fields`"]
    BestFields,
    #[doc = "`bool_prefix`"]
    BoolPrefix,
    #[doc = "`cross_fields`"]
    CrossFields,
    #[doc = "`most_fields`"]
    MostFields,
    #[doc = "`phrase`"]
    Phrase,
    #[doc = "`phrase_prefix`"]
    PhrasePrefix,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TextQueryType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TextQueryType::BestFields => "best_fields",
            TextQueryType::BoolPrefix => "bool_prefix",
            TextQueryType::CrossFields => "cross_fields",
            TextQueryType::MostFields => "most_fields",
            TextQueryType::Phrase => "phrase",
            TextQueryType::PhrasePrefix => "phrase_prefix",
            TextQueryType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TextQueryType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TextQueryType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "best_fields" => TextQueryType::BestFields,
            "bool_prefix" => TextQueryType::BoolPrefix,
            "cross_fields" => TextQueryType::CrossFields,
            "most_fields" => TextQueryType::MostFields,
            "phrase" => TextQueryType::Phrase,
            "phrase_prefix" => TextQueryType::PhrasePrefix,
            _ => TextQueryType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct TypeQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    pub value: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TypeQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TypeQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TypeQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_value: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "value" => {
                            if f_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            f_value = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TypeQuery {
                    name: f_name,
                    boost: f_boost,
                    value: f_value.ok_or_else(|| serde::de::Error::missing_field("value"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TypeQuery {
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
        map.serialize_entry("value", &self.value)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'"') => serde_json::from_str(text).map(WildcardQuery::Value),
            Some(b'{') => serde_json::from_str(text).map(WildcardQuery::Object),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a WildcardQuery value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct WildcardQueryObject {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "Allows case insensitive matching of the pattern with the indexed field values when set to `true`. Default is `false` which means the case sensitivity of matching depends on the underlying field's mapping."]
    pub case_insensitive: Option<bool>,
    pub rewrite: Option<crate::types::common::MultiTermQueryRewrite>,
    #[doc = "Wildcard pattern for terms you wish to find in the provided field. Required, when wildcard is not set."]
    pub value: Option<String>,
    #[doc = "Wildcard pattern for terms you wish to find in the provided field. Required, when value is not set."]
    pub wildcard: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for WildcardQueryObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = WildcardQueryObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a WildcardQueryObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_case_insensitive: Option<bool> = None;
                let mut seen_case_insensitive = false;
                let mut f_rewrite: Option<crate::types::common::MultiTermQueryRewrite> = None;
                let mut seen_rewrite = false;
                let mut f_value: Option<String> = None;
                let mut seen_value = false;
                let mut f_wildcard: Option<String> = None;
                let mut seen_wildcard = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "case_insensitive" => {
                            if seen_case_insensitive {
                                return Err(serde::de::Error::duplicate_field("case_insensitive"));
                            }
                            seen_case_insensitive = true;
                            f_case_insensitive = map.next_value::<Option<bool>>()?;
                        }
                        "rewrite" => {
                            if seen_rewrite {
                                return Err(serde::de::Error::duplicate_field("rewrite"));
                            }
                            seen_rewrite = true;
                            f_rewrite = map
                                .next_value::<Option<crate::types::common::MultiTermQueryRewrite>>(
                                )?;
                        }
                        "value" => {
                            if seen_value {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            seen_value = true;
                            f_value = map.next_value::<Option<String>>()?;
                        }
                        "wildcard" => {
                            if seen_wildcard {
                                return Err(serde::de::Error::duplicate_field("wildcard"));
                            }
                            seen_wildcard = true;
                            f_wildcard = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(WildcardQueryObject {
                    name: f_name,
                    boost: f_boost,
                    case_insensitive: f_case_insensitive,
                    rewrite: f_rewrite,
                    value: f_value,
                    wildcard: f_wildcard,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for WildcardQueryObject {
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
        if let Some(v) = &self.case_insensitive {
            map.serialize_entry("case_insensitive", v)?;
        }
        if let Some(v) = &self.rewrite {
            map.serialize_entry("rewrite", v)?;
        }
        if let Some(v) = &self.value {
            map.serialize_entry("value", v)?;
        }
        if let Some(v) = &self.wildcard {
            map.serialize_entry("wildcard", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct WrapperQuery {
    pub name: Option<String>,
    #[doc = "Floating point number used to decrease or increase the relevance scores of the query. Boost values are relative to the default value of 1.0. A boost value between 0 and 1.0 decreases the relevance score. A value greater than 1.0 increases the relevance score."]
    pub boost: Option<f64>,
    #[doc = "A base64 encoded query. The binary data format can be any of JSON, YAML, CBOR or SMILE encodings"]
    pub query: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for WrapperQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = WrapperQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a WrapperQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_query: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "query" => {
                            if f_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            f_query = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(WrapperQuery {
                    name: f_name,
                    boost: f_boost,
                    query: f_query.ok_or_else(|| serde::de::Error::missing_field("query"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for WrapperQuery {
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
        map.serialize_entry("query", &self.query)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct XyShape {
    pub coordinates: Option<Vec<Box<serde_json::value::RawValue>>>,
    pub r#type: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for XyShape {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = XyShape;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a XyShape object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_coordinates: Option<Vec<Box<serde_json::value::RawValue>>> = None;
                let mut seen_coordinates = false;
                let mut f_type: Option<String> = None;
                let mut seen_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "coordinates" => {
                            if seen_coordinates {
                                return Err(serde::de::Error::duplicate_field("coordinates"));
                            }
                            seen_coordinates = true;
                            f_coordinates =
                                map.next_value::<Option<Vec<Box<serde_json::value::RawValue>>>>()?;
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
                Ok(XyShape {
                    coordinates: f_coordinates,
                    r#type: f_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for XyShape {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.coordinates {
            map.serialize_entry("coordinates", v)?;
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
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_ignore_unmapped: Option<bool> = None;
                let mut seen_ignore_unmapped = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_query_dsl::XyShapeQueryField,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "ignore_unmapped" => {
                            if seen_ignore_unmapped {
                                return Err(serde::de::Error::duplicate_field("ignore_unmapped"));
                            }
                            seen_ignore_unmapped = true;
                            f_ignore_unmapped = map.next_value::<Option<bool>>()?;
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

#[derive(Debug, Clone)]
pub struct XyShapeQueryField {
    pub indexed_shape: Option<crate::types::common_query_dsl::FieldLookup>,
    pub relation: Option<crate::types::common::GeoShapeRelation>,
    pub shape: crate::types::common_query_dsl::XyShape,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for XyShapeQueryField {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = XyShapeQueryField;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a XyShapeQueryField object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_indexed_shape: Option<crate::types::common_query_dsl::FieldLookup> = None;
                let mut seen_indexed_shape = false;
                let mut f_relation: Option<crate::types::common::GeoShapeRelation> = None;
                let mut seen_relation = false;
                let mut f_shape: Option<crate::types::common_query_dsl::XyShape> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "indexed_shape" => {
                            if seen_indexed_shape {
                                return Err(serde::de::Error::duplicate_field("indexed_shape"));
                            }
                            seen_indexed_shape = true;
                            f_indexed_shape = map
                                .next_value::<Option<crate::types::common_query_dsl::FieldLookup>>(
                                )?;
                        }
                        "relation" => {
                            if seen_relation {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            seen_relation = true;
                            f_relation =
                                map.next_value::<Option<crate::types::common::GeoShapeRelation>>()?;
                        }
                        "shape" => {
                            if f_shape.is_some() {
                                return Err(serde::de::Error::duplicate_field("shape"));
                            }
                            f_shape =
                                Some(map.next_value::<crate::types::common_query_dsl::XyShape>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(XyShapeQueryField {
                    indexed_shape: f_indexed_shape,
                    relation: f_relation,
                    shape: f_shape.ok_or_else(|| serde::de::Error::missing_field("shape"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for XyShapeQueryField {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.indexed_shape {
            map.serialize_entry("indexed_shape", v)?;
        }
        if let Some(v) = &self.relation {
            map.serialize_entry("relation", v)?;
        }
        map.serialize_entry("shape", &self.shape)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ZeroTermsQuery {
    #[doc = "`all` (also accepted: `ALL`)"]
    All,
    #[doc = "`none` (also accepted: `NONE`)"]
    None,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ZeroTermsQuery {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ZeroTermsQuery::All => "all",
            ZeroTermsQuery::None => "none",
            ZeroTermsQuery::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ZeroTermsQuery {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ZeroTermsQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "all" | "ALL" => ZeroTermsQuery::All,
            "none" | "NONE" => ZeroTermsQuery::None,
            _ => ZeroTermsQuery::Unknown(s),
        })
    }
}
