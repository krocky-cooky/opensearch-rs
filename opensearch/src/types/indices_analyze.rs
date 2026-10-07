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

//! Response types of the `indices_analyze` specification namespace

#![allow(missing_docs)]

#[doc = "The detailed analysis results."]
#[derive(Debug, Clone)]
pub struct AnalyzeDetail {
    pub analyzer: Option<crate::types::indices_analyze::AnalyzerDetail>,
    #[doc = "The list of character filter results."]
    pub charfilters: Option<Vec<crate::types::indices_analyze::CharFilterDetail>>,
    #[doc = "Whether a custom analyzer was used."]
    pub custom_analyzer: bool,
    #[doc = "The list of token filter results."]
    pub tokenfilters: Option<Vec<crate::types::indices_analyze::TokenDetail>>,
    pub tokenizer: Option<crate::types::indices_analyze::TokenDetail>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AnalyzeDetail {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AnalyzeDetail;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AnalyzeDetail object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_analyzer: Option<crate::types::indices_analyze::AnalyzerDetail> = None;
                let mut seen_analyzer = false;
                let mut f_charfilters: Option<
                    Vec<crate::types::indices_analyze::CharFilterDetail>,
                > = None;
                let mut seen_charfilters = false;
                let mut f_custom_analyzer: Option<bool> = None;
                let mut f_tokenfilters: Option<Vec<crate::types::indices_analyze::TokenDetail>> =
                    None;
                let mut seen_tokenfilters = false;
                let mut f_tokenizer: Option<crate::types::indices_analyze::TokenDetail> = None;
                let mut seen_tokenizer = false;
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
                            f_analyzer = map.next_value::<Option<crate::types::indices_analyze::AnalyzerDetail>>()?;
                        }
                        "charfilters" => {
                            if seen_charfilters {
                                return Err(serde::de::Error::duplicate_field("charfilters"));
                            }
                            seen_charfilters = true;
                            f_charfilters = map.next_value::<Option<Vec<crate::types::indices_analyze::CharFilterDetail>>>()?;
                        }
                        "custom_analyzer" => {
                            if f_custom_analyzer.is_some() {
                                return Err(serde::de::Error::duplicate_field("custom_analyzer"));
                            }
                            f_custom_analyzer = Some(map.next_value::<bool>()?);
                        }
                        "tokenfilters" => {
                            if seen_tokenfilters {
                                return Err(serde::de::Error::duplicate_field("tokenfilters"));
                            }
                            seen_tokenfilters = true;
                            f_tokenfilters = map.next_value::<Option<Vec<crate::types::indices_analyze::TokenDetail>>>()?;
                        }
                        "tokenizer" => {
                            if seen_tokenizer {
                                return Err(serde::de::Error::duplicate_field("tokenizer"));
                            }
                            seen_tokenizer = true;
                            f_tokenizer = map
                                .next_value::<Option<crate::types::indices_analyze::TokenDetail>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AnalyzeDetail {
                    analyzer: f_analyzer,
                    charfilters: f_charfilters,
                    custom_analyzer: f_custom_analyzer
                        .ok_or_else(|| serde::de::Error::missing_field("custom_analyzer"))?,
                    tokenfilters: f_tokenfilters,
                    tokenizer: f_tokenizer,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AnalyzeDetail {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.analyzer {
            map.serialize_entry("analyzer", v)?;
        }
        if let Some(v) = &self.charfilters {
            map.serialize_entry("charfilters", v)?;
        }
        map.serialize_entry("custom_analyzer", &self.custom_analyzer)?;
        if let Some(v) = &self.tokenfilters {
            map.serialize_entry("tokenfilters", v)?;
        }
        if let Some(v) = &self.tokenizer {
            map.serialize_entry("tokenizer", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `indices.analyze` API"]
pub type AnalyzeResponse = crate::types::indices::AnalyzeResponse;

#[doc = "The basic token information."]
#[derive(Debug, Clone)]
pub struct AnalyzeToken {
    #[doc = "The ending character offset of the token."]
    pub end_offset: i32,
    #[doc = "The position of the token."]
    pub position: i32,
    #[doc = "The length of the token position."]
    pub position_length: Option<i32>,
    #[doc = "The starting character offset of the token."]
    pub start_offset: i32,
    #[doc = "The token string."]
    pub token: String,
    #[doc = "The type of the token."]
    pub r#type: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AnalyzeToken {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AnalyzeToken;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AnalyzeToken object")
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
                let mut f_position: Option<i32> = None;
                let mut f_position_length: Option<i32> = None;
                let mut seen_position_length = false;
                let mut f_start_offset: Option<i32> = None;
                let mut f_token: Option<String> = None;
                let mut f_type: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "end_offset" => {
                            if f_end_offset.is_some() {
                                return Err(serde::de::Error::duplicate_field("end_offset"));
                            }
                            f_end_offset = Some(map.next_value::<i32>()?);
                        }
                        "position" => {
                            if f_position.is_some() {
                                return Err(serde::de::Error::duplicate_field("position"));
                            }
                            f_position = Some(map.next_value::<i32>()?);
                        }
                        "positionLength" => {
                            if seen_position_length {
                                return Err(serde::de::Error::duplicate_field("positionLength"));
                            }
                            seen_position_length = true;
                            f_position_length = map.next_value::<Option<i32>>()?;
                        }
                        "start_offset" => {
                            if f_start_offset.is_some() {
                                return Err(serde::de::Error::duplicate_field("start_offset"));
                            }
                            f_start_offset = Some(map.next_value::<i32>()?);
                        }
                        "token" => {
                            if f_token.is_some() {
                                return Err(serde::de::Error::duplicate_field("token"));
                            }
                            f_token = Some(map.next_value::<String>()?);
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
                Ok(AnalyzeToken {
                    end_offset: f_end_offset
                        .ok_or_else(|| serde::de::Error::missing_field("end_offset"))?,
                    position: f_position
                        .ok_or_else(|| serde::de::Error::missing_field("position"))?,
                    position_length: f_position_length,
                    start_offset: f_start_offset
                        .ok_or_else(|| serde::de::Error::missing_field("start_offset"))?,
                    token: f_token.ok_or_else(|| serde::de::Error::missing_field("token"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AnalyzeToken {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("end_offset", &self.end_offset)?;
        map.serialize_entry("position", &self.position)?;
        if let Some(v) = &self.position_length {
            map.serialize_entry("positionLength", v)?;
        }
        map.serialize_entry("start_offset", &self.start_offset)?;
        map.serialize_entry("token", &self.token)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The analyzer results."]
#[derive(Debug, Clone)]
pub struct AnalyzerDetail {
    #[doc = "The name of the analyzer."]
    pub name: String,
    #[doc = "The list of tokens produced by the analyzer."]
    pub tokens: Vec<crate::types::indices_analyze::ExplainAnalyzeToken>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AnalyzerDetail {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AnalyzerDetail;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AnalyzerDetail object")
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
                let mut f_tokens: Option<Vec<crate::types::indices_analyze::ExplainAnalyzeToken>> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<String>()?);
                        }
                        "tokens" => {
                            if f_tokens.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokens"));
                            }
                            f_tokens = Some(map.next_value::<Vec<crate::types::indices_analyze::ExplainAnalyzeToken>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AnalyzerDetail {
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    tokens: f_tokens.ok_or_else(|| serde::de::Error::missing_field("tokens"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AnalyzerDetail {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("tokens", &self.tokens)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The character filter results."]
#[derive(Debug, Clone)]
pub struct CharFilterDetail {
    #[doc = "The text after character filtering."]
    pub filtered_text: Vec<String>,
    #[doc = "The name of the character filter."]
    pub name: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CharFilterDetail {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CharFilterDetail;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CharFilterDetail object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_filtered_text: Option<Vec<String>> = None;
                let mut f_name: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "filtered_text" => {
                            if f_filtered_text.is_some() {
                                return Err(serde::de::Error::duplicate_field("filtered_text"));
                            }
                            f_filtered_text = Some(map.next_value::<Vec<String>>()?);
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CharFilterDetail {
                    filtered_text: f_filtered_text
                        .ok_or_else(|| serde::de::Error::missing_field("filtered_text"))?,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CharFilterDetail {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("filtered_text", &self.filtered_text)?;
        map.serialize_entry("name", &self.name)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The detailed token information."]
#[derive(Debug, Clone)]
pub struct ExplainAnalyzeToken {
    #[doc = "The bytes representation of the token."]
    pub bytes: String,
    #[doc = "The ending character offset of the token."]
    pub end_offset: i32,
    #[doc = "Whether the token is a keyword."]
    pub keyword: Option<bool>,
    #[doc = "The position of the token."]
    pub position: i32,
    #[doc = "The length of the token position."]
    pub position_length: i32,
    #[doc = "The starting character offset of the token."]
    pub start_offset: i32,
    #[doc = "The frequency of the term."]
    pub term_frequency: i32,
    #[doc = "The token string."]
    pub token: String,
    #[doc = "The type of the token."]
    pub r#type: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ExplainAnalyzeToken {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ExplainAnalyzeToken;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ExplainAnalyzeToken object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_bytes: Option<String> = None;
                let mut f_end_offset: Option<i32> = None;
                let mut f_keyword: Option<bool> = None;
                let mut seen_keyword = false;
                let mut f_position: Option<i32> = None;
                let mut f_position_length: Option<i32> = None;
                let mut f_start_offset: Option<i32> = None;
                let mut f_term_frequency: Option<i32> = None;
                let mut f_token: Option<String> = None;
                let mut f_type: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "bytes" => {
                            if f_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("bytes"));
                            }
                            f_bytes = Some(map.next_value::<String>()?);
                        }
                        "end_offset" => {
                            if f_end_offset.is_some() {
                                return Err(serde::de::Error::duplicate_field("end_offset"));
                            }
                            f_end_offset = Some(map.next_value::<i32>()?);
                        }
                        "keyword" => {
                            if seen_keyword {
                                return Err(serde::de::Error::duplicate_field("keyword"));
                            }
                            seen_keyword = true;
                            f_keyword = map.next_value::<Option<bool>>()?;
                        }
                        "position" => {
                            if f_position.is_some() {
                                return Err(serde::de::Error::duplicate_field("position"));
                            }
                            f_position = Some(map.next_value::<i32>()?);
                        }
                        "positionLength" => {
                            if f_position_length.is_some() {
                                return Err(serde::de::Error::duplicate_field("positionLength"));
                            }
                            f_position_length = Some(map.next_value::<i32>()?);
                        }
                        "start_offset" => {
                            if f_start_offset.is_some() {
                                return Err(serde::de::Error::duplicate_field("start_offset"));
                            }
                            f_start_offset = Some(map.next_value::<i32>()?);
                        }
                        "termFrequency" => {
                            if f_term_frequency.is_some() {
                                return Err(serde::de::Error::duplicate_field("termFrequency"));
                            }
                            f_term_frequency = Some(map.next_value::<i32>()?);
                        }
                        "token" => {
                            if f_token.is_some() {
                                return Err(serde::de::Error::duplicate_field("token"));
                            }
                            f_token = Some(map.next_value::<String>()?);
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
                Ok(ExplainAnalyzeToken {
                    bytes: f_bytes.ok_or_else(|| serde::de::Error::missing_field("bytes"))?,
                    end_offset: f_end_offset
                        .ok_or_else(|| serde::de::Error::missing_field("end_offset"))?,
                    keyword: f_keyword,
                    position: f_position
                        .ok_or_else(|| serde::de::Error::missing_field("position"))?,
                    position_length: f_position_length
                        .ok_or_else(|| serde::de::Error::missing_field("positionLength"))?,
                    start_offset: f_start_offset
                        .ok_or_else(|| serde::de::Error::missing_field("start_offset"))?,
                    term_frequency: f_term_frequency
                        .ok_or_else(|| serde::de::Error::missing_field("termFrequency"))?,
                    token: f_token.ok_or_else(|| serde::de::Error::missing_field("token"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ExplainAnalyzeToken {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("bytes", &self.bytes)?;
        map.serialize_entry("end_offset", &self.end_offset)?;
        if let Some(v) = &self.keyword {
            map.serialize_entry("keyword", v)?;
        }
        map.serialize_entry("position", &self.position)?;
        map.serialize_entry("positionLength", &self.position_length)?;
        map.serialize_entry("start_offset", &self.start_offset)?;
        map.serialize_entry("termFrequency", &self.term_frequency)?;
        map.serialize_entry("token", &self.token)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The token filter or tokenizer results."]
#[derive(Debug, Clone)]
pub struct TokenDetail {
    #[doc = "The name of the token filter or tokenizer."]
    pub name: String,
    #[doc = "The list of tokens produced."]
    pub tokens: Vec<crate::types::indices_analyze::ExplainAnalyzeToken>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TokenDetail {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TokenDetail;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TokenDetail object")
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
                let mut f_tokens: Option<Vec<crate::types::indices_analyze::ExplainAnalyzeToken>> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<String>()?);
                        }
                        "tokens" => {
                            if f_tokens.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokens"));
                            }
                            f_tokens = Some(map.next_value::<Vec<crate::types::indices_analyze::ExplainAnalyzeToken>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TokenDetail {
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    tokens: f_tokens.ok_or_else(|| serde::de::Error::missing_field("tokens"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TokenDetail {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("tokens", &self.tokens)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
