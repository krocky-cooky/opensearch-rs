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

//! Response types of the `rank_eval` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct RankEvalHit {
    pub id: crate::types::common::Id,
    pub index: crate::types::common::IndexName,
    pub score: f64,
    pub r#type: Option<crate::types::common::Type>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RankEvalHit {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RankEvalHit;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RankEvalHit object")
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
                let mut f_score: Option<f64> = None;
                let mut f_type: Option<crate::types::common::Type> = None;
                let mut seen_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_id" => {
                            if f_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("_id"));
                            }
                            f_id = Some(map.next_value::<crate::types::common::Id>()?);
                        }
                        "_index" => {
                            if f_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("_index"));
                            }
                            f_index = Some(map.next_value::<crate::types::common::IndexName>()?);
                        }
                        "_score" => {
                            if f_score.is_some() {
                                return Err(serde::de::Error::duplicate_field("_score"));
                            }
                            f_score = Some(map.next_value::<f64>()?);
                        }
                        "_type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<crate::types::common::Type>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RankEvalHit {
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("_id"))?,
                    index: f_index.ok_or_else(|| serde::de::Error::missing_field("_index"))?,
                    score: f_score.ok_or_else(|| serde::de::Error::missing_field("_score"))?,
                    r#type: f_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RankEvalHit {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("_id", &self.id)?;
        map.serialize_entry("_index", &self.index)?;
        map.serialize_entry("_score", &self.score)?;
        if let Some(v) = &self.r#type {
            map.serialize_entry("_type", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RankEvalHitItem {
    pub hit: crate::types::rank_eval::RankEvalHit,
    pub rating: Option<i32>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RankEvalHitItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RankEvalHitItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RankEvalHitItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_hit: Option<crate::types::rank_eval::RankEvalHit> = None;
                let mut f_rating: Option<i32> = None;
                let mut seen_rating = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "hit" => {
                            if f_hit.is_some() {
                                return Err(serde::de::Error::duplicate_field("hit"));
                            }
                            f_hit = Some(map.next_value::<crate::types::rank_eval::RankEvalHit>()?);
                        }
                        "rating" => {
                            if seen_rating {
                                return Err(serde::de::Error::duplicate_field("rating"));
                            }
                            seen_rating = true;
                            f_rating = map.next_value::<Option<i32>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RankEvalHitItem {
                    hit: f_hit.ok_or_else(|| serde::de::Error::missing_field("hit"))?,
                    rating: f_rating,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RankEvalHitItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("hit", &self.hit)?;
        if let Some(v) = &self.rating {
            map.serialize_entry("rating", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RankEvalMetricDetail {
    #[doc = "The `hits` section provides a grouping of the search results with their supplied ratings."]
    pub hits: Vec<crate::types::rank_eval::RankEvalHitItem>,
    #[doc = "The `metric_details` section provides additional information about the calculated quality metric indicating the number of relevant retrieved documents. The content varies for each metric but allows for better interpretation of the results."]
    pub metric_details: std::collections::BTreeMap<
        String,
        std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
    >,
    #[doc = "The `metric_score`, found in the `metric_details` section, shows the contribution of this query to the global quality metric score."]
    pub metric_score: f64,
    #[doc = "The `unrated_docs` section contains an `_index` and `_id` entry for each document that didn't have a `ratings` value. This can be used to ask the user to supply ratings for these documents."]
    pub unrated_docs: Vec<crate::types::rank_eval::UnratedDocument>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RankEvalMetricDetail {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RankEvalMetricDetail;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RankEvalMetricDetail object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_hits: Option<Vec<crate::types::rank_eval::RankEvalHitItem>> = None;
                let mut f_metric_details: Option<
                    std::collections::BTreeMap<
                        String,
                        std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                    >,
                > = None;
                let mut f_metric_score: Option<f64> = None;
                let mut f_unrated_docs: Option<Vec<crate::types::rank_eval::UnratedDocument>> =
                    None;
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
                                map.next_value::<Vec<crate::types::rank_eval::RankEvalHitItem>>()?,
                            );
                        }
                        "metric_details" => {
                            if f_metric_details.is_some() {
                                return Err(serde::de::Error::duplicate_field("metric_details"));
                            }
                            f_metric_details = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?);
                        }
                        "metric_score" => {
                            if f_metric_score.is_some() {
                                return Err(serde::de::Error::duplicate_field("metric_score"));
                            }
                            f_metric_score = Some(map.next_value::<f64>()?);
                        }
                        "unrated_docs" => {
                            if f_unrated_docs.is_some() {
                                return Err(serde::de::Error::duplicate_field("unrated_docs"));
                            }
                            f_unrated_docs = Some(
                                map.next_value::<Vec<crate::types::rank_eval::UnratedDocument>>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RankEvalMetricDetail {
                    hits: f_hits.ok_or_else(|| serde::de::Error::missing_field("hits"))?,
                    metric_details: f_metric_details
                        .ok_or_else(|| serde::de::Error::missing_field("metric_details"))?,
                    metric_score: f_metric_score
                        .ok_or_else(|| serde::de::Error::missing_field("metric_score"))?,
                    unrated_docs: f_unrated_docs
                        .ok_or_else(|| serde::de::Error::missing_field("unrated_docs"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RankEvalMetricDetail {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("hits", &self.hits)?;
        map.serialize_entry("metric_details", &self.metric_details)?;
        map.serialize_entry("metric_score", &self.metric_score)?;
        map.serialize_entry("unrated_docs", &self.unrated_docs)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RankEvalResponse {
    #[doc = "The details section contains one entry for every query in the original requests section, keyed by the search request id"]
    pub details: std::collections::BTreeMap<String, crate::types::rank_eval::RankEvalMetricDetail>,
    pub failures: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
    #[doc = "The overall evaluation quality calculated by the defined metric"]
    pub metric_score: f64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RankEvalResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RankEvalResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RankEvalResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_details: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::rank_eval::RankEvalMetricDetail,
                    >,
                > = None;
                let mut f_failures: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut f_metric_score: Option<f64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "details" => {
                            if f_details.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            f_details = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                crate::types::rank_eval::RankEvalMetricDetail,
                            >>()?);
                        }
                        "failures" => {
                            if f_failures.is_some() {
                                return Err(serde::de::Error::duplicate_field("failures"));
                            }
                            f_failures = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                Box<serde_json::value::RawValue>,
                            >>()?);
                        }
                        "metric_score" => {
                            if f_metric_score.is_some() {
                                return Err(serde::de::Error::duplicate_field("metric_score"));
                            }
                            f_metric_score = Some(map.next_value::<f64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RankEvalResponse {
                    details: f_details.ok_or_else(|| serde::de::Error::missing_field("details"))?,
                    failures: f_failures
                        .ok_or_else(|| serde::de::Error::missing_field("failures"))?,
                    metric_score: f_metric_score
                        .ok_or_else(|| serde::de::Error::missing_field("metric_score"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RankEvalResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("details", &self.details)?;
        map.serialize_entry("failures", &self.failures)?;
        map.serialize_entry("metric_score", &self.metric_score)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct UnratedDocument {
    pub id: crate::types::common::Id,
    pub index: crate::types::common::IndexName,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UnratedDocument {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UnratedDocument;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UnratedDocument object")
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
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_id" => {
                            if f_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("_id"));
                            }
                            f_id = Some(map.next_value::<crate::types::common::Id>()?);
                        }
                        "_index" => {
                            if f_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("_index"));
                            }
                            f_index = Some(map.next_value::<crate::types::common::IndexName>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UnratedDocument {
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("_id"))?,
                    index: f_index.ok_or_else(|| serde::de::Error::missing_field("_index"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UnratedDocument {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("_id", &self.id)?;
        map.serialize_entry("_index", &self.index)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
