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

//! Response types of the `termvectors` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct FieldStatistics {
    pub doc_count: i32,
    pub sum_doc_freq: i64,
    pub sum_ttf: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FieldStatistics {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FieldStatistics;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FieldStatistics object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_doc_count: Option<i32> = None;
                let mut f_sum_doc_freq: Option<i64> = None;
                let mut f_sum_ttf: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "doc_count" => {
                            if f_doc_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            f_doc_count = Some(map.next_value::<i32>()?);
                        }
                        "sum_doc_freq" => {
                            if f_sum_doc_freq.is_some() {
                                return Err(serde::de::Error::duplicate_field("sum_doc_freq"));
                            }
                            f_sum_doc_freq = Some(map.next_value::<i64>()?);
                        }
                        "sum_ttf" => {
                            if f_sum_ttf.is_some() {
                                return Err(serde::de::Error::duplicate_field("sum_ttf"));
                            }
                            f_sum_ttf = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FieldStatistics {
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    sum_doc_freq: f_sum_doc_freq
                        .ok_or_else(|| serde::de::Error::missing_field("sum_doc_freq"))?,
                    sum_ttf: f_sum_ttf.ok_or_else(|| serde::de::Error::missing_field("sum_ttf"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FieldStatistics {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("doc_count", &self.doc_count)?;
        map.serialize_entry("sum_doc_freq", &self.sum_doc_freq)?;
        map.serialize_entry("sum_ttf", &self.sum_ttf)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Term {
    pub doc_freq: Option<i32>,
    pub score: Option<f64>,
    pub term_freq: i32,
    pub tokens: Option<Vec<crate::types::termvectors::Token>>,
    pub ttf: Option<i32>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Term {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Term;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Term object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_doc_freq: Option<i32> = None;
                let mut seen_doc_freq = false;
                let mut f_score: Option<f64> = None;
                let mut seen_score = false;
                let mut f_term_freq: Option<i32> = None;
                let mut f_tokens: Option<Vec<crate::types::termvectors::Token>> = None;
                let mut seen_tokens = false;
                let mut f_ttf: Option<i32> = None;
                let mut seen_ttf = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "doc_freq" => {
                            if seen_doc_freq {
                                return Err(serde::de::Error::duplicate_field("doc_freq"));
                            }
                            seen_doc_freq = true;
                            f_doc_freq = map.next_value::<Option<i32>>()?;
                        }
                        "score" => {
                            if seen_score {
                                return Err(serde::de::Error::duplicate_field("score"));
                            }
                            seen_score = true;
                            f_score = map.next_value::<Option<f64>>()?;
                        }
                        "term_freq" => {
                            if f_term_freq.is_some() {
                                return Err(serde::de::Error::duplicate_field("term_freq"));
                            }
                            f_term_freq = Some(map.next_value::<i32>()?);
                        }
                        "tokens" => {
                            if seen_tokens {
                                return Err(serde::de::Error::duplicate_field("tokens"));
                            }
                            seen_tokens = true;
                            f_tokens =
                                map.next_value::<Option<Vec<crate::types::termvectors::Token>>>()?;
                        }
                        "ttf" => {
                            if seen_ttf {
                                return Err(serde::de::Error::duplicate_field("ttf"));
                            }
                            seen_ttf = true;
                            f_ttf = map.next_value::<Option<i32>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Term {
                    doc_freq: f_doc_freq,
                    score: f_score,
                    term_freq: f_term_freq
                        .ok_or_else(|| serde::de::Error::missing_field("term_freq"))?,
                    tokens: f_tokens,
                    ttf: f_ttf,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Term {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.doc_freq {
            map.serialize_entry("doc_freq", v)?;
        }
        if let Some(v) = &self.score {
            map.serialize_entry("score", v)?;
        }
        map.serialize_entry("term_freq", &self.term_freq)?;
        if let Some(v) = &self.tokens {
            map.serialize_entry("tokens", v)?;
        }
        if let Some(v) = &self.ttf {
            map.serialize_entry("ttf", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TermVector {
    pub field_statistics: Option<crate::types::termvectors::FieldStatistics>,
    pub terms: std::collections::BTreeMap<String, crate::types::termvectors::Term>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TermVector {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TermVector;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TermVector object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_field_statistics: Option<crate::types::termvectors::FieldStatistics> =
                    None;
                let mut seen_field_statistics = false;
                let mut f_terms: Option<
                    std::collections::BTreeMap<String, crate::types::termvectors::Term>,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "field_statistics" => {
                            if seen_field_statistics {
                                return Err(serde::de::Error::duplicate_field("field_statistics"));
                            }
                            seen_field_statistics = true;
                            f_field_statistics = map
                                .next_value::<Option<crate::types::termvectors::FieldStatistics>>(
                                )?;
                        }
                        "terms" => {
                            if f_terms.is_some() {
                                return Err(serde::de::Error::duplicate_field("terms"));
                            }
                            f_terms = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                crate::types::termvectors::Term,
                            >>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TermVector {
                    field_statistics: f_field_statistics,
                    terms: f_terms.ok_or_else(|| serde::de::Error::missing_field("terms"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TermVector {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.field_statistics {
            map.serialize_entry("field_statistics", v)?;
        }
        map.serialize_entry("terms", &self.terms)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TermvectorsResponse {
    pub id: Option<crate::types::common::Id>,
    pub index: crate::types::common::IndexName,
    pub r#type: Option<crate::types::common::Type>,
    pub version: crate::types::common::VersionNumber,
    pub found: bool,
    pub term_vectors:
        Option<std::collections::BTreeMap<String, crate::types::termvectors::TermVector>>,
    pub took: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TermvectorsResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TermvectorsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TermvectorsResponse object")
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
                let mut f_type: Option<crate::types::common::Type> = None;
                let mut seen_type = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut f_found: Option<bool> = None;
                let mut f_term_vectors: Option<
                    std::collections::BTreeMap<String, crate::types::termvectors::TermVector>,
                > = None;
                let mut seen_term_vectors = false;
                let mut f_took: Option<i64> = None;
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
                            if f_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("_index"));
                            }
                            f_index = Some(map.next_value::<crate::types::common::IndexName>()?);
                        }
                        "_type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<crate::types::common::Type>>()?;
                        }
                        "_version" => {
                            if f_version.is_some() {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            f_version =
                                Some(map.next_value::<crate::types::common::VersionNumber>()?);
                        }
                        "found" => {
                            if f_found.is_some() {
                                return Err(serde::de::Error::duplicate_field("found"));
                            }
                            f_found = Some(map.next_value::<bool>()?);
                        }
                        "term_vectors" => {
                            if seen_term_vectors {
                                return Err(serde::de::Error::duplicate_field("term_vectors"));
                            }
                            seen_term_vectors = true;
                            f_term_vectors = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::termvectors::TermVector,
                                >,
                            >>()?;
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
                Ok(TermvectorsResponse {
                    id: f_id,
                    index: f_index.ok_or_else(|| serde::de::Error::missing_field("_index"))?,
                    r#type: f_type,
                    version: f_version
                        .ok_or_else(|| serde::de::Error::missing_field("_version"))?,
                    found: f_found.ok_or_else(|| serde::de::Error::missing_field("found"))?,
                    term_vectors: f_term_vectors,
                    took: f_took.ok_or_else(|| serde::de::Error::missing_field("took"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TermvectorsResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.id {
            map.serialize_entry("_id", v)?;
        }
        map.serialize_entry("_index", &self.index)?;
        if let Some(v) = &self.r#type {
            map.serialize_entry("_type", v)?;
        }
        map.serialize_entry("_version", &self.version)?;
        map.serialize_entry("found", &self.found)?;
        if let Some(v) = &self.term_vectors {
            map.serialize_entry("term_vectors", v)?;
        }
        map.serialize_entry("took", &self.took)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub end_offset: Option<i32>,
    pub payload: Option<String>,
    pub position: i32,
    pub start_offset: Option<i32>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Token {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Token;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Token object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_end_offset: Option<i32> = None;
                let mut seen_end_offset = false;
                let mut f_payload: Option<String> = None;
                let mut seen_payload = false;
                let mut f_position: Option<i32> = None;
                let mut f_start_offset: Option<i32> = None;
                let mut seen_start_offset = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "end_offset" => {
                            if seen_end_offset {
                                return Err(serde::de::Error::duplicate_field("end_offset"));
                            }
                            seen_end_offset = true;
                            f_end_offset = map.next_value::<Option<i32>>()?;
                        }
                        "payload" => {
                            if seen_payload {
                                return Err(serde::de::Error::duplicate_field("payload"));
                            }
                            seen_payload = true;
                            f_payload = map.next_value::<Option<String>>()?;
                        }
                        "position" => {
                            if f_position.is_some() {
                                return Err(serde::de::Error::duplicate_field("position"));
                            }
                            f_position = Some(map.next_value::<i32>()?);
                        }
                        "start_offset" => {
                            if seen_start_offset {
                                return Err(serde::de::Error::duplicate_field("start_offset"));
                            }
                            seen_start_offset = true;
                            f_start_offset = map.next_value::<Option<i32>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Token {
                    end_offset: f_end_offset,
                    payload: f_payload,
                    position: f_position
                        .ok_or_else(|| serde::de::Error::missing_field("position"))?,
                    start_offset: f_start_offset,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Token {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.end_offset {
            map.serialize_entry("end_offset", v)?;
        }
        if let Some(v) = &self.payload {
            map.serialize_entry("payload", v)?;
        }
        map.serialize_entry("position", &self.position)?;
        if let Some(v) = &self.start_offset {
            map.serialize_entry("start_offset", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
