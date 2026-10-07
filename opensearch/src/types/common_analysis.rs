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

//! Response types of the `common_analysis` specification namespace

#![allow(missing_docs)]

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Analyzer {
    raw: Box<serde_json::value::RawValue>,
}
impl Analyzer {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `custom_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_custom_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::CustomAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "custom_analyzer")
    }
    #[doc = "Parses the value as the `fingerprint_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_fingerprint_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::FingerprintAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "fingerprint_analyzer")
    }
    #[doc = "Parses the value as the `keyword_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_keyword_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KeywordAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "keyword_analyzer")
    }
    #[doc = "Parses the value as the `language_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_language_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::LanguageAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "language_analyzer")
    }
    #[doc = "Parses the value as the `nori_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_nori_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::NoriAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "nori_analyzer")
    }
    #[doc = "Parses the value as the `pattern_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_pattern_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PatternAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "pattern_analyzer")
    }
    #[doc = "Parses the value as the `simple_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_simple_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::SimpleAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "simple_analyzer")
    }
    #[doc = "Parses the value as the `standard_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_standard_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::StandardAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "standard_analyzer")
    }
    #[doc = "Parses the value as the `stop_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_stop_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::StopAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "stop_analyzer")
    }
    #[doc = "Parses the value as the `whitespace_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_whitespace_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::WhitespaceAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "whitespace_analyzer")
    }
    #[doc = "Parses the value as the `icu_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_icu_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::IcuAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "icu_analyzer")
    }
    #[doc = "Parses the value as the `kuromoji_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_kuromoji_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KuromojiAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "kuromoji_analyzer")
    }
    #[doc = "Parses the value as the `snowball_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_snowball_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::SnowballAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "snowball_analyzer")
    }
    #[doc = "Parses the value as the `dutch_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_dutch_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::DutchAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "dutch_analyzer")
    }
    #[doc = "Parses the value as the `smartcn_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_smartcn_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::SmartcnAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "smartcn_analyzer")
    }
    #[doc = "Parses the value as the `cjk_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_cjk_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::CjkAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "cjk_analyzer")
    }
    #[doc = "Parses the value as the `phone_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_phone_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PhoneAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "phone_analyzer")
    }
    #[doc = "Parses the value as the `phone_search_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_phone_search_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PhoneSearchAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "phone_search_analyzer")
    }
}

#[derive(Debug, Clone)]
pub struct AsciiFoldingTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub preserve_original: Option<crate::types::common::StringifiedBoolean>,
    pub r#type: crate::types::common_analysis::AsciiFoldingTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AsciiFoldingTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AsciiFoldingTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AsciiFoldingTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_preserve_original: Option<crate::types::common::StringifiedBoolean> =
                    None;
                let mut seen_preserve_original = false;
                let mut f_type: Option<crate::types::common_analysis::AsciiFoldingTokenFilterType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "preserve_original" => {
                            if seen_preserve_original {
                                return Err(serde::de::Error::duplicate_field("preserve_original"));
                            }
                            seen_preserve_original = true;
                            f_preserve_original = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::AsciiFoldingTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AsciiFoldingTokenFilter {
                    version: f_version,
                    preserve_original: f_preserve_original,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AsciiFoldingTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.preserve_original {
            map.serialize_entry("preserve_original", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AsciiFoldingTokenFilterType {
    #[doc = "`asciifolding`"]
    Asciifolding,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl AsciiFoldingTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            AsciiFoldingTokenFilterType::Asciifolding => "asciifolding",
            AsciiFoldingTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for AsciiFoldingTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AsciiFoldingTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "asciifolding" => AsciiFoldingTokenFilterType::Asciifolding,
            _ => AsciiFoldingTokenFilterType::Unknown(s),
        })
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct CharFilter {
    raw: Box<serde_json::value::RawValue>,
}
impl CharFilter {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `name` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_name(&self) -> std::result::Result<String, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "name")
    }
    #[doc = "Parses the value as the `definition` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_definition(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::CharFilterDefinition,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "definition")
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct CharFilterDefinition {
    raw: Box<serde_json::value::RawValue>,
}
impl CharFilterDefinition {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `html_strip_char_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_html_strip_char_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::HtmlStripCharFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "html_strip_char_filter")
    }
    #[doc = "Parses the value as the `mapping_char_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_mapping_char_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::MappingCharFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "mapping_char_filter")
    }
    #[doc = "Parses the value as the `pattern_replace_char_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_pattern_replace_char_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PatternReplaceCharFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "pattern_replace_char_filter")
    }
    #[doc = "Parses the value as the `icu_normalization_char_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_icu_normalization_char_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::IcuNormalizationCharFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "icu_normalization_char_filter")
    }
    #[doc = "Parses the value as the `kuromoji_iteration_mark_char_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_kuromoji_iteration_mark_char_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KuromojiIterationMarkCharFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "kuromoji_iteration_mark_char_filter")
    }
}

#[derive(Debug, Clone)]
pub struct CharGroupTokenizer {
    pub version: Option<crate::types::common::VersionString>,
    pub max_token_length: Option<i64>,
    pub tokenize_on_chars: Vec<String>,
    pub r#type: crate::types::common_analysis::CharGroupTokenizerType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CharGroupTokenizer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CharGroupTokenizer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CharGroupTokenizer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_max_token_length: Option<i64> = None;
                let mut seen_max_token_length = false;
                let mut f_tokenize_on_chars: Option<Vec<String>> = None;
                let mut f_type: Option<crate::types::common_analysis::CharGroupTokenizerType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "max_token_length" => {
                            if seen_max_token_length {
                                return Err(serde::de::Error::duplicate_field("max_token_length"));
                            }
                            seen_max_token_length = true;
                            f_max_token_length = map.next_value::<Option<i64>>()?;
                        }
                        "tokenize_on_chars" => {
                            if f_tokenize_on_chars.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenize_on_chars"));
                            }
                            f_tokenize_on_chars = Some(map.next_value::<Vec<String>>()?);
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::CharGroupTokenizerType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CharGroupTokenizer {
                    version: f_version,
                    max_token_length: f_max_token_length,
                    tokenize_on_chars: f_tokenize_on_chars
                        .ok_or_else(|| serde::de::Error::missing_field("tokenize_on_chars"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CharGroupTokenizer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.max_token_length {
            map.serialize_entry("max_token_length", v)?;
        }
        map.serialize_entry("tokenize_on_chars", &self.tokenize_on_chars)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CharGroupTokenizerType {
    #[doc = "`char_group`"]
    CharGroup,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl CharGroupTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            CharGroupTokenizerType::CharGroup => "char_group",
            CharGroupTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for CharGroupTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CharGroupTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "char_group" => CharGroupTokenizerType::CharGroup,
            _ => CharGroupTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct CjkAnalyzer {
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    pub stopwords_path: Option<String>,
    pub r#type: Option<crate::types::common_analysis::CjkAnalyzerType>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CjkAnalyzer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CjkAnalyzer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CjkAnalyzer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_stopwords: Option<crate::types::common_analysis::StopWords> = None;
                let mut seen_stopwords = false;
                let mut f_stopwords_path: Option<String> = None;
                let mut seen_stopwords_path = false;
                let mut f_type: Option<crate::types::common_analysis::CjkAnalyzerType> = None;
                let mut seen_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "stopwords" => {
                            if seen_stopwords {
                                return Err(serde::de::Error::duplicate_field("stopwords"));
                            }
                            seen_stopwords = true;
                            f_stopwords = map
                                .next_value::<Option<crate::types::common_analysis::StopWords>>()?;
                        }
                        "stopwords_path" => {
                            if seen_stopwords_path {
                                return Err(serde::de::Error::duplicate_field("stopwords_path"));
                            }
                            seen_stopwords_path = true;
                            f_stopwords_path = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<crate::types::common_analysis::CjkAnalyzerType>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CjkAnalyzer {
                    stopwords: f_stopwords,
                    stopwords_path: f_stopwords_path,
                    r#type: f_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CjkAnalyzer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.stopwords {
            map.serialize_entry("stopwords", v)?;
        }
        if let Some(v) = &self.stopwords_path {
            map.serialize_entry("stopwords_path", v)?;
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CjkAnalyzerType {
    #[doc = "`cjk`"]
    Cjk,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl CjkAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            CjkAnalyzerType::Cjk => "cjk",
            CjkAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for CjkAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CjkAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "cjk" => CjkAnalyzerType::Cjk,
            _ => CjkAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct CommonGramsTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub common_words: Option<Vec<String>>,
    pub common_words_path: Option<String>,
    pub ignore_case: Option<bool>,
    pub query_mode: Option<bool>,
    pub r#type: crate::types::common_analysis::CommonGramsTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CommonGramsTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CommonGramsTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CommonGramsTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_common_words: Option<Vec<String>> = None;
                let mut seen_common_words = false;
                let mut f_common_words_path: Option<String> = None;
                let mut seen_common_words_path = false;
                let mut f_ignore_case: Option<bool> = None;
                let mut seen_ignore_case = false;
                let mut f_query_mode: Option<bool> = None;
                let mut seen_query_mode = false;
                let mut f_type: Option<crate::types::common_analysis::CommonGramsTokenFilterType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "common_words" => {
                            if seen_common_words {
                                return Err(serde::de::Error::duplicate_field("common_words"));
                            }
                            seen_common_words = true;
                            f_common_words = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "common_words_path" => {
                            if seen_common_words_path {
                                return Err(serde::de::Error::duplicate_field("common_words_path"));
                            }
                            seen_common_words_path = true;
                            f_common_words_path = map.next_value::<Option<String>>()?;
                        }
                        "ignore_case" => {
                            if seen_ignore_case {
                                return Err(serde::de::Error::duplicate_field("ignore_case"));
                            }
                            seen_ignore_case = true;
                            f_ignore_case = map.next_value::<Option<bool>>()?;
                        }
                        "query_mode" => {
                            if seen_query_mode {
                                return Err(serde::de::Error::duplicate_field("query_mode"));
                            }
                            seen_query_mode = true;
                            f_query_mode = map.next_value::<Option<bool>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::CommonGramsTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CommonGramsTokenFilter {
                    version: f_version,
                    common_words: f_common_words,
                    common_words_path: f_common_words_path,
                    ignore_case: f_ignore_case,
                    query_mode: f_query_mode,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CommonGramsTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.common_words {
            map.serialize_entry("common_words", v)?;
        }
        if let Some(v) = &self.common_words_path {
            map.serialize_entry("common_words_path", v)?;
        }
        if let Some(v) = &self.ignore_case {
            map.serialize_entry("ignore_case", v)?;
        }
        if let Some(v) = &self.query_mode {
            map.serialize_entry("query_mode", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommonGramsTokenFilterType {
    #[doc = "`common_grams`"]
    CommonGrams,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl CommonGramsTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            CommonGramsTokenFilterType::CommonGrams => "common_grams",
            CommonGramsTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for CommonGramsTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CommonGramsTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "common_grams" => CommonGramsTokenFilterType::CommonGrams,
            _ => CommonGramsTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ConditionTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub filter: Vec<String>,
    pub script: crate::types::common::Script,
    pub r#type: crate::types::common_analysis::ConditionTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ConditionTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ConditionTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ConditionTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_filter: Option<Vec<String>> = None;
                let mut f_script: Option<crate::types::common::Script> = None;
                let mut f_type: Option<crate::types::common_analysis::ConditionTokenFilterType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "filter" => {
                            if f_filter.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            f_filter = Some(map.next_value::<Vec<String>>()?);
                        }
                        "script" => {
                            if f_script.is_some() {
                                return Err(serde::de::Error::duplicate_field("script"));
                            }
                            f_script = Some(map.next_value::<crate::types::common::Script>()?);
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::ConditionTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ConditionTokenFilter {
                    version: f_version,
                    filter: f_filter.ok_or_else(|| serde::de::Error::missing_field("filter"))?,
                    script: f_script.ok_or_else(|| serde::de::Error::missing_field("script"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ConditionTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("filter", &self.filter)?;
        map.serialize_entry("script", &self.script)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConditionTokenFilterType {
    #[doc = "`condition`"]
    Condition,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ConditionTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ConditionTokenFilterType::Condition => "condition",
            ConditionTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ConditionTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ConditionTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "condition" => ConditionTokenFilterType::Condition,
            _ => ConditionTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct CustomAnalyzer {
    pub char_filter: Option<Vec<String>>,
    pub filter: Option<Vec<String>>,
    pub position_increment_gap: Option<i64>,
    pub position_offset_gap: Option<i64>,
    pub tokenizer: String,
    pub r#type: crate::types::common_analysis::CustomAnalyzerType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CustomAnalyzer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CustomAnalyzer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CustomAnalyzer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_char_filter: Option<Vec<String>> = None;
                let mut seen_char_filter = false;
                let mut f_filter: Option<Vec<String>> = None;
                let mut seen_filter = false;
                let mut f_position_increment_gap: Option<i64> = None;
                let mut seen_position_increment_gap = false;
                let mut f_position_offset_gap: Option<i64> = None;
                let mut seen_position_offset_gap = false;
                let mut f_tokenizer: Option<String> = None;
                let mut f_type: Option<crate::types::common_analysis::CustomAnalyzerType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "char_filter" => {
                            if seen_char_filter {
                                return Err(serde::de::Error::duplicate_field("char_filter"));
                            }
                            seen_char_filter = true;
                            f_char_filter = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "filter" => {
                            if seen_filter {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            seen_filter = true;
                            f_filter = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "position_increment_gap" => {
                            if seen_position_increment_gap {
                                return Err(serde::de::Error::duplicate_field(
                                    "position_increment_gap",
                                ));
                            }
                            seen_position_increment_gap = true;
                            f_position_increment_gap = map.next_value::<Option<i64>>()?;
                        }
                        "position_offset_gap" => {
                            if seen_position_offset_gap {
                                return Err(serde::de::Error::duplicate_field(
                                    "position_offset_gap",
                                ));
                            }
                            seen_position_offset_gap = true;
                            f_position_offset_gap = map.next_value::<Option<i64>>()?;
                        }
                        "tokenizer" => {
                            if f_tokenizer.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenizer"));
                            }
                            f_tokenizer = Some(map.next_value::<String>()?);
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::CustomAnalyzerType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CustomAnalyzer {
                    char_filter: f_char_filter,
                    filter: f_filter,
                    position_increment_gap: f_position_increment_gap,
                    position_offset_gap: f_position_offset_gap,
                    tokenizer: f_tokenizer
                        .ok_or_else(|| serde::de::Error::missing_field("tokenizer"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CustomAnalyzer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.char_filter {
            map.serialize_entry("char_filter", v)?;
        }
        if let Some(v) = &self.filter {
            map.serialize_entry("filter", v)?;
        }
        if let Some(v) = &self.position_increment_gap {
            map.serialize_entry("position_increment_gap", v)?;
        }
        if let Some(v) = &self.position_offset_gap {
            map.serialize_entry("position_offset_gap", v)?;
        }
        map.serialize_entry("tokenizer", &self.tokenizer)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CustomAnalyzerType {
    #[doc = "`custom`"]
    Custom,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl CustomAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            CustomAnalyzerType::Custom => "custom",
            CustomAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for CustomAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "custom" => CustomAnalyzerType::Custom,
            _ => CustomAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct CustomNormalizer {
    pub char_filter: Option<Vec<String>>,
    pub filter: Option<Vec<String>>,
    pub r#type: crate::types::common_analysis::CustomNormalizerType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CustomNormalizer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CustomNormalizer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CustomNormalizer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_char_filter: Option<Vec<String>> = None;
                let mut seen_char_filter = false;
                let mut f_filter: Option<Vec<String>> = None;
                let mut seen_filter = false;
                let mut f_type: Option<crate::types::common_analysis::CustomNormalizerType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "char_filter" => {
                            if seen_char_filter {
                                return Err(serde::de::Error::duplicate_field("char_filter"));
                            }
                            seen_char_filter = true;
                            f_char_filter = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "filter" => {
                            if seen_filter {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            seen_filter = true;
                            f_filter = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::CustomNormalizerType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CustomNormalizer {
                    char_filter: f_char_filter,
                    filter: f_filter,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CustomNormalizer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.char_filter {
            map.serialize_entry("char_filter", v)?;
        }
        if let Some(v) = &self.filter {
            map.serialize_entry("filter", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CustomNormalizerType {
    #[doc = "`custom`"]
    Custom,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl CustomNormalizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            CustomNormalizerType::Custom => "custom",
            CustomNormalizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for CustomNormalizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomNormalizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "custom" => CustomNormalizerType::Custom,
            _ => CustomNormalizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DelimitedPayloadEncoding {
    #[doc = "`float`"]
    Float,
    #[doc = "`identity`"]
    Identity,
    #[doc = "`int`"]
    Int,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DelimitedPayloadEncoding {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DelimitedPayloadEncoding::Float => "float",
            DelimitedPayloadEncoding::Identity => "identity",
            DelimitedPayloadEncoding::Int => "int",
            DelimitedPayloadEncoding::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DelimitedPayloadEncoding {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DelimitedPayloadEncoding {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "float" => DelimitedPayloadEncoding::Float,
            "identity" => DelimitedPayloadEncoding::Identity,
            "int" => DelimitedPayloadEncoding::Int,
            _ => DelimitedPayloadEncoding::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct DelimitedPayloadTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub delimiter: Option<String>,
    pub encoding: Option<crate::types::common_analysis::DelimitedPayloadEncoding>,
    pub r#type: crate::types::common_analysis::DelimitedPayloadTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DelimitedPayloadTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DelimitedPayloadTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DelimitedPayloadTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_delimiter: Option<String> = None;
                let mut seen_delimiter = false;
                let mut f_encoding: Option<
                    crate::types::common_analysis::DelimitedPayloadEncoding,
                > = None;
                let mut seen_encoding = false;
                let mut f_type: Option<
                    crate::types::common_analysis::DelimitedPayloadTokenFilterType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "delimiter" => {
                            if seen_delimiter {
                                return Err(serde::de::Error::duplicate_field("delimiter"));
                            }
                            seen_delimiter = true;
                            f_delimiter = map.next_value::<Option<String>>()?;
                        }
                        "encoding" => {
                            if seen_encoding {
                                return Err(serde::de::Error::duplicate_field("encoding"));
                            }
                            seen_encoding = true;
                            f_encoding = map.next_value::<Option<crate::types::common_analysis::DelimitedPayloadEncoding>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::DelimitedPayloadTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DelimitedPayloadTokenFilter {
                    version: f_version,
                    delimiter: f_delimiter,
                    encoding: f_encoding,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DelimitedPayloadTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.delimiter {
            map.serialize_entry("delimiter", v)?;
        }
        if let Some(v) = &self.encoding {
            map.serialize_entry("encoding", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DelimitedPayloadTokenFilterType {
    #[doc = "`delimited_payload`"]
    DelimitedPayload,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DelimitedPayloadTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DelimitedPayloadTokenFilterType::DelimitedPayload => "delimited_payload",
            DelimitedPayloadTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DelimitedPayloadTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DelimitedPayloadTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "delimited_payload" => DelimitedPayloadTokenFilterType::DelimitedPayload,
            _ => DelimitedPayloadTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct DictionaryDecompounderTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub hyphenation_patterns_path: Option<String>,
    pub max_subword_size: Option<i64>,
    pub min_subword_size: Option<i64>,
    pub min_word_size: Option<i64>,
    pub only_longest_match: Option<bool>,
    pub word_list: Option<Vec<String>>,
    pub word_list_path: Option<String>,
    pub r#type: crate::types::common_analysis::DictionaryDecompounderTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DictionaryDecompounderTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DictionaryDecompounderTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DictionaryDecompounderTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_hyphenation_patterns_path: Option<String> = None;
                let mut seen_hyphenation_patterns_path = false;
                let mut f_max_subword_size: Option<i64> = None;
                let mut seen_max_subword_size = false;
                let mut f_min_subword_size: Option<i64> = None;
                let mut seen_min_subword_size = false;
                let mut f_min_word_size: Option<i64> = None;
                let mut seen_min_word_size = false;
                let mut f_only_longest_match: Option<bool> = None;
                let mut seen_only_longest_match = false;
                let mut f_word_list: Option<Vec<String>> = None;
                let mut seen_word_list = false;
                let mut f_word_list_path: Option<String> = None;
                let mut seen_word_list_path = false;
                let mut f_type: Option<
                    crate::types::common_analysis::DictionaryDecompounderTokenFilterType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "hyphenation_patterns_path" => {
                            if seen_hyphenation_patterns_path {
                                return Err(serde::de::Error::duplicate_field(
                                    "hyphenation_patterns_path",
                                ));
                            }
                            seen_hyphenation_patterns_path = true;
                            f_hyphenation_patterns_path = map.next_value::<Option<String>>()?;
                        }
                        "max_subword_size" => {
                            if seen_max_subword_size {
                                return Err(serde::de::Error::duplicate_field("max_subword_size"));
                            }
                            seen_max_subword_size = true;
                            f_max_subword_size = map.next_value::<Option<i64>>()?;
                        }
                        "min_subword_size" => {
                            if seen_min_subword_size {
                                return Err(serde::de::Error::duplicate_field("min_subword_size"));
                            }
                            seen_min_subword_size = true;
                            f_min_subword_size = map.next_value::<Option<i64>>()?;
                        }
                        "min_word_size" => {
                            if seen_min_word_size {
                                return Err(serde::de::Error::duplicate_field("min_word_size"));
                            }
                            seen_min_word_size = true;
                            f_min_word_size = map.next_value::<Option<i64>>()?;
                        }
                        "only_longest_match" => {
                            if seen_only_longest_match {
                                return Err(serde::de::Error::duplicate_field(
                                    "only_longest_match",
                                ));
                            }
                            seen_only_longest_match = true;
                            f_only_longest_match = map.next_value::<Option<bool>>()?;
                        }
                        "word_list" => {
                            if seen_word_list {
                                return Err(serde::de::Error::duplicate_field("word_list"));
                            }
                            seen_word_list = true;
                            f_word_list = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "word_list_path" => {
                            if seen_word_list_path {
                                return Err(serde::de::Error::duplicate_field("word_list_path"));
                            }
                            seen_word_list_path = true;
                            f_word_list_path = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::DictionaryDecompounderTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DictionaryDecompounderTokenFilter {
                    version: f_version,
                    hyphenation_patterns_path: f_hyphenation_patterns_path,
                    max_subword_size: f_max_subword_size,
                    min_subword_size: f_min_subword_size,
                    min_word_size: f_min_word_size,
                    only_longest_match: f_only_longest_match,
                    word_list: f_word_list,
                    word_list_path: f_word_list_path,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DictionaryDecompounderTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.hyphenation_patterns_path {
            map.serialize_entry("hyphenation_patterns_path", v)?;
        }
        if let Some(v) = &self.max_subword_size {
            map.serialize_entry("max_subword_size", v)?;
        }
        if let Some(v) = &self.min_subword_size {
            map.serialize_entry("min_subword_size", v)?;
        }
        if let Some(v) = &self.min_word_size {
            map.serialize_entry("min_word_size", v)?;
        }
        if let Some(v) = &self.only_longest_match {
            map.serialize_entry("only_longest_match", v)?;
        }
        if let Some(v) = &self.word_list {
            map.serialize_entry("word_list", v)?;
        }
        if let Some(v) = &self.word_list_path {
            map.serialize_entry("word_list_path", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DictionaryDecompounderTokenFilterType {
    #[doc = "`dictionary_decompounder`"]
    DictionaryDecompounder,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DictionaryDecompounderTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DictionaryDecompounderTokenFilterType::DictionaryDecompounder => {
                "dictionary_decompounder"
            }
            DictionaryDecompounderTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DictionaryDecompounderTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DictionaryDecompounderTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "dictionary_decompounder" => {
                DictionaryDecompounderTokenFilterType::DictionaryDecompounder
            }
            _ => DictionaryDecompounderTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct DutchAnalyzer {
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    pub r#type: crate::types::common_analysis::DutchAnalyzerType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DutchAnalyzer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DutchAnalyzer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DutchAnalyzer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_stopwords: Option<crate::types::common_analysis::StopWords> = None;
                let mut seen_stopwords = false;
                let mut f_type: Option<crate::types::common_analysis::DutchAnalyzerType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "stopwords" => {
                            if seen_stopwords {
                                return Err(serde::de::Error::duplicate_field("stopwords"));
                            }
                            seen_stopwords = true;
                            f_stopwords = map
                                .next_value::<Option<crate::types::common_analysis::StopWords>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(
                                map.next_value::<crate::types::common_analysis::DutchAnalyzerType>(
                                )?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DutchAnalyzer {
                    stopwords: f_stopwords,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DutchAnalyzer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.stopwords {
            map.serialize_entry("stopwords", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DutchAnalyzerType {
    #[doc = "`dutch`"]
    Dutch,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DutchAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DutchAnalyzerType::Dutch => "dutch",
            DutchAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DutchAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DutchAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "dutch" => DutchAnalyzerType::Dutch,
            _ => DutchAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EdgeNGramSide {
    #[doc = "`back`"]
    Back,
    #[doc = "`front`"]
    Front,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl EdgeNGramSide {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            EdgeNGramSide::Back => "back",
            EdgeNGramSide::Front => "front",
            EdgeNGramSide::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for EdgeNGramSide {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for EdgeNGramSide {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "back" => EdgeNGramSide::Back,
            "front" => EdgeNGramSide::Front,
            _ => EdgeNGramSide::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct EdgeNGramTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub max_gram: Option<i64>,
    pub min_gram: Option<i64>,
    pub preserve_original: Option<crate::types::common::StringifiedBoolean>,
    pub side: Option<crate::types::common_analysis::EdgeNGramSide>,
    pub r#type: crate::types::common_analysis::EdgeNGramTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for EdgeNGramTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = EdgeNGramTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a EdgeNGramTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_max_gram: Option<i64> = None;
                let mut seen_max_gram = false;
                let mut f_min_gram: Option<i64> = None;
                let mut seen_min_gram = false;
                let mut f_preserve_original: Option<crate::types::common::StringifiedBoolean> =
                    None;
                let mut seen_preserve_original = false;
                let mut f_side: Option<crate::types::common_analysis::EdgeNGramSide> = None;
                let mut seen_side = false;
                let mut f_type: Option<crate::types::common_analysis::EdgeNGramTokenFilterType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "max_gram" => {
                            if seen_max_gram {
                                return Err(serde::de::Error::duplicate_field("max_gram"));
                            }
                            seen_max_gram = true;
                            f_max_gram = map.next_value::<Option<i64>>()?;
                        }
                        "min_gram" => {
                            if seen_min_gram {
                                return Err(serde::de::Error::duplicate_field("min_gram"));
                            }
                            seen_min_gram = true;
                            f_min_gram = map.next_value::<Option<i64>>()?;
                        }
                        "preserve_original" => {
                            if seen_preserve_original {
                                return Err(serde::de::Error::duplicate_field("preserve_original"));
                            }
                            seen_preserve_original = true;
                            f_preserve_original = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "side" => {
                            if seen_side {
                                return Err(serde::de::Error::duplicate_field("side"));
                            }
                            seen_side = true;
                            f_side = map
                                .next_value::<Option<crate::types::common_analysis::EdgeNGramSide>>(
                                )?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::EdgeNGramTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(EdgeNGramTokenFilter {
                    version: f_version,
                    max_gram: f_max_gram,
                    min_gram: f_min_gram,
                    preserve_original: f_preserve_original,
                    side: f_side,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for EdgeNGramTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.max_gram {
            map.serialize_entry("max_gram", v)?;
        }
        if let Some(v) = &self.min_gram {
            map.serialize_entry("min_gram", v)?;
        }
        if let Some(v) = &self.preserve_original {
            map.serialize_entry("preserve_original", v)?;
        }
        if let Some(v) = &self.side {
            map.serialize_entry("side", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EdgeNGramTokenFilterType {
    #[doc = "`edge_ngram`"]
    EdgeNgram,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl EdgeNGramTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            EdgeNGramTokenFilterType::EdgeNgram => "edge_ngram",
            EdgeNGramTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for EdgeNGramTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for EdgeNGramTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "edge_ngram" => EdgeNGramTokenFilterType::EdgeNgram,
            _ => EdgeNGramTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct EdgeNGramTokenizer {
    pub version: Option<crate::types::common::VersionString>,
    pub custom_token_chars: Option<String>,
    pub max_gram: i64,
    pub min_gram: i64,
    pub token_chars: Vec<crate::types::common_analysis::TokenChar>,
    pub r#type: crate::types::common_analysis::EdgeNGramTokenizerType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for EdgeNGramTokenizer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = EdgeNGramTokenizer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a EdgeNGramTokenizer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_custom_token_chars: Option<String> = None;
                let mut seen_custom_token_chars = false;
                let mut f_max_gram: Option<i64> = None;
                let mut f_min_gram: Option<i64> = None;
                let mut f_token_chars: Option<Vec<crate::types::common_analysis::TokenChar>> = None;
                let mut f_type: Option<crate::types::common_analysis::EdgeNGramTokenizerType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "custom_token_chars" => {
                            if seen_custom_token_chars {
                                return Err(serde::de::Error::duplicate_field(
                                    "custom_token_chars",
                                ));
                            }
                            seen_custom_token_chars = true;
                            f_custom_token_chars = map.next_value::<Option<String>>()?;
                        }
                        "max_gram" => {
                            if f_max_gram.is_some() {
                                return Err(serde::de::Error::duplicate_field("max_gram"));
                            }
                            f_max_gram = Some(map.next_value::<i64>()?);
                        }
                        "min_gram" => {
                            if f_min_gram.is_some() {
                                return Err(serde::de::Error::duplicate_field("min_gram"));
                            }
                            f_min_gram = Some(map.next_value::<i64>()?);
                        }
                        "token_chars" => {
                            if f_token_chars.is_some() {
                                return Err(serde::de::Error::duplicate_field("token_chars"));
                            }
                            f_token_chars = Some(
                                map.next_value::<Vec<crate::types::common_analysis::TokenChar>>()?,
                            );
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::EdgeNGramTokenizerType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(EdgeNGramTokenizer {
                    version: f_version,
                    custom_token_chars: f_custom_token_chars,
                    max_gram: f_max_gram
                        .ok_or_else(|| serde::de::Error::missing_field("max_gram"))?,
                    min_gram: f_min_gram
                        .ok_or_else(|| serde::de::Error::missing_field("min_gram"))?,
                    token_chars: f_token_chars
                        .ok_or_else(|| serde::de::Error::missing_field("token_chars"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for EdgeNGramTokenizer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.custom_token_chars {
            map.serialize_entry("custom_token_chars", v)?;
        }
        map.serialize_entry("max_gram", &self.max_gram)?;
        map.serialize_entry("min_gram", &self.min_gram)?;
        map.serialize_entry("token_chars", &self.token_chars)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EdgeNGramTokenizerType {
    #[doc = "`edge_ngram`"]
    EdgeNgram,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl EdgeNGramTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            EdgeNGramTokenizerType::EdgeNgram => "edge_ngram",
            EdgeNGramTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for EdgeNGramTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for EdgeNGramTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "edge_ngram" => EdgeNGramTokenizerType::EdgeNgram,
            _ => EdgeNGramTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ElisionTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub articles: Option<Vec<String>>,
    pub articles_case: Option<crate::types::common::StringifiedBoolean>,
    pub articles_path: Option<String>,
    pub r#type: crate::types::common_analysis::ElisionTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ElisionTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ElisionTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ElisionTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_articles: Option<Vec<String>> = None;
                let mut seen_articles = false;
                let mut f_articles_case: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_articles_case = false;
                let mut f_articles_path: Option<String> = None;
                let mut seen_articles_path = false;
                let mut f_type: Option<crate::types::common_analysis::ElisionTokenFilterType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "articles" => {
                            if seen_articles {
                                return Err(serde::de::Error::duplicate_field("articles"));
                            }
                            seen_articles = true;
                            f_articles = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "articles_case" => {
                            if seen_articles_case {
                                return Err(serde::de::Error::duplicate_field("articles_case"));
                            }
                            seen_articles_case = true;
                            f_articles_case = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "articles_path" => {
                            if seen_articles_path {
                                return Err(serde::de::Error::duplicate_field("articles_path"));
                            }
                            seen_articles_path = true;
                            f_articles_path = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::ElisionTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ElisionTokenFilter {
                    version: f_version,
                    articles: f_articles,
                    articles_case: f_articles_case,
                    articles_path: f_articles_path,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ElisionTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.articles {
            map.serialize_entry("articles", v)?;
        }
        if let Some(v) = &self.articles_case {
            map.serialize_entry("articles_case", v)?;
        }
        if let Some(v) = &self.articles_path {
            map.serialize_entry("articles_path", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElisionTokenFilterType {
    #[doc = "`elision`"]
    Elision,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ElisionTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ElisionTokenFilterType::Elision => "elision",
            ElisionTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ElisionTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ElisionTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "elision" => ElisionTokenFilterType::Elision,
            _ => ElisionTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct FingerprintAnalyzer {
    pub max_output_size: i64,
    pub preserve_original: bool,
    pub separator: String,
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    pub stopwords_path: Option<String>,
    pub r#type: crate::types::common_analysis::FingerprintAnalyzerType,
    pub version: Option<crate::types::common::VersionString>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FingerprintAnalyzer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FingerprintAnalyzer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FingerprintAnalyzer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_max_output_size: Option<i64> = None;
                let mut f_preserve_original: Option<bool> = None;
                let mut f_separator: Option<String> = None;
                let mut f_stopwords: Option<crate::types::common_analysis::StopWords> = None;
                let mut seen_stopwords = false;
                let mut f_stopwords_path: Option<String> = None;
                let mut seen_stopwords_path = false;
                let mut f_type: Option<crate::types::common_analysis::FingerprintAnalyzerType> =
                    None;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "max_output_size" => {
                            if f_max_output_size.is_some() {
                                return Err(serde::de::Error::duplicate_field("max_output_size"));
                            }
                            f_max_output_size = Some(map.next_value::<i64>()?);
                        }
                        "preserve_original" => {
                            if f_preserve_original.is_some() {
                                return Err(serde::de::Error::duplicate_field("preserve_original"));
                            }
                            f_preserve_original = Some(map.next_value::<bool>()?);
                        }
                        "separator" => {
                            if f_separator.is_some() {
                                return Err(serde::de::Error::duplicate_field("separator"));
                            }
                            f_separator = Some(map.next_value::<String>()?);
                        }
                        "stopwords" => {
                            if seen_stopwords {
                                return Err(serde::de::Error::duplicate_field("stopwords"));
                            }
                            seen_stopwords = true;
                            f_stopwords = map
                                .next_value::<Option<crate::types::common_analysis::StopWords>>()?;
                        }
                        "stopwords_path" => {
                            if seen_stopwords_path {
                                return Err(serde::de::Error::duplicate_field("stopwords_path"));
                            }
                            seen_stopwords_path = true;
                            f_stopwords_path = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::FingerprintAnalyzerType>()?);
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FingerprintAnalyzer {
                    max_output_size: f_max_output_size
                        .ok_or_else(|| serde::de::Error::missing_field("max_output_size"))?,
                    preserve_original: f_preserve_original
                        .ok_or_else(|| serde::de::Error::missing_field("preserve_original"))?,
                    separator: f_separator
                        .ok_or_else(|| serde::de::Error::missing_field("separator"))?,
                    stopwords: f_stopwords,
                    stopwords_path: f_stopwords_path,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FingerprintAnalyzer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("max_output_size", &self.max_output_size)?;
        map.serialize_entry("preserve_original", &self.preserve_original)?;
        map.serialize_entry("separator", &self.separator)?;
        if let Some(v) = &self.stopwords {
            map.serialize_entry("stopwords", v)?;
        }
        if let Some(v) = &self.stopwords_path {
            map.serialize_entry("stopwords_path", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FingerprintAnalyzerType {
    #[doc = "`fingerprint`"]
    Fingerprint,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl FingerprintAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            FingerprintAnalyzerType::Fingerprint => "fingerprint",
            FingerprintAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for FingerprintAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FingerprintAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "fingerprint" => FingerprintAnalyzerType::Fingerprint,
            _ => FingerprintAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct FingerprintTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub max_output_size: Option<i64>,
    pub separator: Option<String>,
    pub r#type: crate::types::common_analysis::FingerprintTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FingerprintTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FingerprintTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FingerprintTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_max_output_size: Option<i64> = None;
                let mut seen_max_output_size = false;
                let mut f_separator: Option<String> = None;
                let mut seen_separator = false;
                let mut f_type: Option<crate::types::common_analysis::FingerprintTokenFilterType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "max_output_size" => {
                            if seen_max_output_size {
                                return Err(serde::de::Error::duplicate_field("max_output_size"));
                            }
                            seen_max_output_size = true;
                            f_max_output_size = map.next_value::<Option<i64>>()?;
                        }
                        "separator" => {
                            if seen_separator {
                                return Err(serde::de::Error::duplicate_field("separator"));
                            }
                            seen_separator = true;
                            f_separator = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::FingerprintTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FingerprintTokenFilter {
                    version: f_version,
                    max_output_size: f_max_output_size,
                    separator: f_separator,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FingerprintTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.max_output_size {
            map.serialize_entry("max_output_size", v)?;
        }
        if let Some(v) = &self.separator {
            map.serialize_entry("separator", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FingerprintTokenFilterType {
    #[doc = "`fingerprint`"]
    Fingerprint,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl FingerprintTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            FingerprintTokenFilterType::Fingerprint => "fingerprint",
            FingerprintTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for FingerprintTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FingerprintTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "fingerprint" => FingerprintTokenFilterType::Fingerprint,
            _ => FingerprintTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct HtmlStripCharFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::HtmlStripCharFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HtmlStripCharFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HtmlStripCharFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HtmlStripCharFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_type: Option<crate::types::common_analysis::HtmlStripCharFilterType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::HtmlStripCharFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HtmlStripCharFilter {
                    version: f_version,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HtmlStripCharFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HtmlStripCharFilterType {
    #[doc = "`html_strip`"]
    HtmlStrip,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl HtmlStripCharFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            HtmlStripCharFilterType::HtmlStrip => "html_strip",
            HtmlStripCharFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for HtmlStripCharFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HtmlStripCharFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "html_strip" => HtmlStripCharFilterType::HtmlStrip,
            _ => HtmlStripCharFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct HunspellTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub dedup: Option<bool>,
    pub dictionary: Option<String>,
    pub locale: String,
    pub longest_only: Option<bool>,
    pub r#type: crate::types::common_analysis::HunspellTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HunspellTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HunspellTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HunspellTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_dedup: Option<bool> = None;
                let mut seen_dedup = false;
                let mut f_dictionary: Option<String> = None;
                let mut seen_dictionary = false;
                let mut f_locale: Option<String> = None;
                let mut f_longest_only: Option<bool> = None;
                let mut seen_longest_only = false;
                let mut f_type: Option<crate::types::common_analysis::HunspellTokenFilterType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "dedup" => {
                            if seen_dedup {
                                return Err(serde::de::Error::duplicate_field("dedup"));
                            }
                            seen_dedup = true;
                            f_dedup = map.next_value::<Option<bool>>()?;
                        }
                        "dictionary" => {
                            if seen_dictionary {
                                return Err(serde::de::Error::duplicate_field("dictionary"));
                            }
                            seen_dictionary = true;
                            f_dictionary = map.next_value::<Option<String>>()?;
                        }
                        "locale" => {
                            if f_locale.is_some() {
                                return Err(serde::de::Error::duplicate_field("locale"));
                            }
                            f_locale = Some(map.next_value::<String>()?);
                        }
                        "longest_only" => {
                            if seen_longest_only {
                                return Err(serde::de::Error::duplicate_field("longest_only"));
                            }
                            seen_longest_only = true;
                            f_longest_only = map.next_value::<Option<bool>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::HunspellTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HunspellTokenFilter {
                    version: f_version,
                    dedup: f_dedup,
                    dictionary: f_dictionary,
                    locale: f_locale.ok_or_else(|| serde::de::Error::missing_field("locale"))?,
                    longest_only: f_longest_only,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HunspellTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.dedup {
            map.serialize_entry("dedup", v)?;
        }
        if let Some(v) = &self.dictionary {
            map.serialize_entry("dictionary", v)?;
        }
        map.serialize_entry("locale", &self.locale)?;
        if let Some(v) = &self.longest_only {
            map.serialize_entry("longest_only", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HunspellTokenFilterType {
    #[doc = "`hunspell`"]
    Hunspell,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl HunspellTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            HunspellTokenFilterType::Hunspell => "hunspell",
            HunspellTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for HunspellTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HunspellTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "hunspell" => HunspellTokenFilterType::Hunspell,
            _ => HunspellTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct HyphenationDecompounderTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub hyphenation_patterns_path: Option<String>,
    pub max_subword_size: Option<i64>,
    pub min_subword_size: Option<i64>,
    pub min_word_size: Option<i64>,
    pub only_longest_match: Option<bool>,
    pub word_list: Option<Vec<String>>,
    pub word_list_path: Option<String>,
    pub r#type: crate::types::common_analysis::HyphenationDecompounderTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HyphenationDecompounderTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HyphenationDecompounderTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HyphenationDecompounderTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_hyphenation_patterns_path: Option<String> = None;
                let mut seen_hyphenation_patterns_path = false;
                let mut f_max_subword_size: Option<i64> = None;
                let mut seen_max_subword_size = false;
                let mut f_min_subword_size: Option<i64> = None;
                let mut seen_min_subword_size = false;
                let mut f_min_word_size: Option<i64> = None;
                let mut seen_min_word_size = false;
                let mut f_only_longest_match: Option<bool> = None;
                let mut seen_only_longest_match = false;
                let mut f_word_list: Option<Vec<String>> = None;
                let mut seen_word_list = false;
                let mut f_word_list_path: Option<String> = None;
                let mut seen_word_list_path = false;
                let mut f_type: Option<
                    crate::types::common_analysis::HyphenationDecompounderTokenFilterType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "hyphenation_patterns_path" => {
                            if seen_hyphenation_patterns_path {
                                return Err(serde::de::Error::duplicate_field(
                                    "hyphenation_patterns_path",
                                ));
                            }
                            seen_hyphenation_patterns_path = true;
                            f_hyphenation_patterns_path = map.next_value::<Option<String>>()?;
                        }
                        "max_subword_size" => {
                            if seen_max_subword_size {
                                return Err(serde::de::Error::duplicate_field("max_subword_size"));
                            }
                            seen_max_subword_size = true;
                            f_max_subword_size = map.next_value::<Option<i64>>()?;
                        }
                        "min_subword_size" => {
                            if seen_min_subword_size {
                                return Err(serde::de::Error::duplicate_field("min_subword_size"));
                            }
                            seen_min_subword_size = true;
                            f_min_subword_size = map.next_value::<Option<i64>>()?;
                        }
                        "min_word_size" => {
                            if seen_min_word_size {
                                return Err(serde::de::Error::duplicate_field("min_word_size"));
                            }
                            seen_min_word_size = true;
                            f_min_word_size = map.next_value::<Option<i64>>()?;
                        }
                        "only_longest_match" => {
                            if seen_only_longest_match {
                                return Err(serde::de::Error::duplicate_field(
                                    "only_longest_match",
                                ));
                            }
                            seen_only_longest_match = true;
                            f_only_longest_match = map.next_value::<Option<bool>>()?;
                        }
                        "word_list" => {
                            if seen_word_list {
                                return Err(serde::de::Error::duplicate_field("word_list"));
                            }
                            seen_word_list = true;
                            f_word_list = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "word_list_path" => {
                            if seen_word_list_path {
                                return Err(serde::de::Error::duplicate_field("word_list_path"));
                            }
                            seen_word_list_path = true;
                            f_word_list_path = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::HyphenationDecompounderTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HyphenationDecompounderTokenFilter {
                    version: f_version,
                    hyphenation_patterns_path: f_hyphenation_patterns_path,
                    max_subword_size: f_max_subword_size,
                    min_subword_size: f_min_subword_size,
                    min_word_size: f_min_word_size,
                    only_longest_match: f_only_longest_match,
                    word_list: f_word_list,
                    word_list_path: f_word_list_path,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HyphenationDecompounderTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.hyphenation_patterns_path {
            map.serialize_entry("hyphenation_patterns_path", v)?;
        }
        if let Some(v) = &self.max_subword_size {
            map.serialize_entry("max_subword_size", v)?;
        }
        if let Some(v) = &self.min_subword_size {
            map.serialize_entry("min_subword_size", v)?;
        }
        if let Some(v) = &self.min_word_size {
            map.serialize_entry("min_word_size", v)?;
        }
        if let Some(v) = &self.only_longest_match {
            map.serialize_entry("only_longest_match", v)?;
        }
        if let Some(v) = &self.word_list {
            map.serialize_entry("word_list", v)?;
        }
        if let Some(v) = &self.word_list_path {
            map.serialize_entry("word_list_path", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HyphenationDecompounderTokenFilterType {
    #[doc = "`hyphenation_decompounder`"]
    HyphenationDecompounder,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl HyphenationDecompounderTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            HyphenationDecompounderTokenFilterType::HyphenationDecompounder => {
                "hyphenation_decompounder"
            }
            HyphenationDecompounderTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for HyphenationDecompounderTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HyphenationDecompounderTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "hyphenation_decompounder" => {
                HyphenationDecompounderTokenFilterType::HyphenationDecompounder
            }
            _ => HyphenationDecompounderTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct IcuAnalyzer {
    pub method: crate::types::common_analysis::IcuNormalizationType,
    pub mode: crate::types::common_analysis::IcuNormalizationMode,
    pub r#type: crate::types::common_analysis::IcuAnalyzerType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IcuAnalyzer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IcuAnalyzer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IcuAnalyzer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_method: Option<crate::types::common_analysis::IcuNormalizationType> =
                    None;
                let mut f_mode: Option<crate::types::common_analysis::IcuNormalizationMode> = None;
                let mut f_type: Option<crate::types::common_analysis::IcuAnalyzerType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "method" => {
                            if f_method.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            f_method = Some(map.next_value::<crate::types::common_analysis::IcuNormalizationType>()?);
                        }
                        "mode" => {
                            if f_mode.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            f_mode = Some(map.next_value::<crate::types::common_analysis::IcuNormalizationMode>()?);
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(
                                map.next_value::<crate::types::common_analysis::IcuAnalyzerType>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IcuAnalyzer {
                    method: f_method.ok_or_else(|| serde::de::Error::missing_field("method"))?,
                    mode: f_mode.ok_or_else(|| serde::de::Error::missing_field("mode"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IcuAnalyzer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("method", &self.method)?;
        map.serialize_entry("mode", &self.mode)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuAnalyzerType {
    #[doc = "`icu_analyzer`"]
    IcuAnalyzer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuAnalyzerType::IcuAnalyzer => "icu_analyzer",
            IcuAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "icu_analyzer" => IcuAnalyzerType::IcuAnalyzer,
            _ => IcuAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuCollationAlternate {
    #[doc = "`non-ignorable`"]
    NonIgnorable,
    #[doc = "`shifted`"]
    Shifted,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuCollationAlternate {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuCollationAlternate::NonIgnorable => "non-ignorable",
            IcuCollationAlternate::Shifted => "shifted",
            IcuCollationAlternate::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuCollationAlternate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuCollationAlternate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "non-ignorable" => IcuCollationAlternate::NonIgnorable,
            "shifted" => IcuCollationAlternate::Shifted,
            _ => IcuCollationAlternate::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuCollationCaseFirst {
    #[doc = "`lower`"]
    Lower,
    #[doc = "`upper`"]
    Upper,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuCollationCaseFirst {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuCollationCaseFirst::Lower => "lower",
            IcuCollationCaseFirst::Upper => "upper",
            IcuCollationCaseFirst::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuCollationCaseFirst {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuCollationCaseFirst {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "lower" => IcuCollationCaseFirst::Lower,
            "upper" => IcuCollationCaseFirst::Upper,
            _ => IcuCollationCaseFirst::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuCollationDecomposition {
    #[doc = "`canonical`"]
    Canonical,
    #[doc = "`no`"]
    No,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuCollationDecomposition {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuCollationDecomposition::Canonical => "canonical",
            IcuCollationDecomposition::No => "no",
            IcuCollationDecomposition::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuCollationDecomposition {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuCollationDecomposition {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "canonical" => IcuCollationDecomposition::Canonical,
            "no" => IcuCollationDecomposition::No,
            _ => IcuCollationDecomposition::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuCollationStrength {
    #[doc = "`identical`"]
    Identical,
    #[doc = "`primary`"]
    Primary,
    #[doc = "`quaternary`"]
    Quaternary,
    #[doc = "`secondary`"]
    Secondary,
    #[doc = "`tertiary`"]
    Tertiary,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuCollationStrength {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuCollationStrength::Identical => "identical",
            IcuCollationStrength::Primary => "primary",
            IcuCollationStrength::Quaternary => "quaternary",
            IcuCollationStrength::Secondary => "secondary",
            IcuCollationStrength::Tertiary => "tertiary",
            IcuCollationStrength::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuCollationStrength {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuCollationStrength {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "identical" => IcuCollationStrength::Identical,
            "primary" => IcuCollationStrength::Primary,
            "quaternary" => IcuCollationStrength::Quaternary,
            "secondary" => IcuCollationStrength::Secondary,
            "tertiary" => IcuCollationStrength::Tertiary,
            _ => IcuCollationStrength::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct IcuCollationTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub alternate: Option<crate::types::common_analysis::IcuCollationAlternate>,
    pub case_first: Option<crate::types::common_analysis::IcuCollationCaseFirst>,
    pub case_level: Option<bool>,
    pub country: Option<String>,
    pub decomposition: Option<crate::types::common_analysis::IcuCollationDecomposition>,
    pub hiragana_quaternary_mode: Option<bool>,
    pub language: Option<String>,
    pub numeric: Option<bool>,
    pub rules: Option<String>,
    pub strength: Option<crate::types::common_analysis::IcuCollationStrength>,
    pub r#type: crate::types::common_analysis::IcuCollationTokenFilterType,
    pub variable_top: Option<String>,
    pub variant: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IcuCollationTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IcuCollationTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IcuCollationTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_alternate: Option<crate::types::common_analysis::IcuCollationAlternate> =
                    None;
                let mut seen_alternate = false;
                let mut f_case_first: Option<crate::types::common_analysis::IcuCollationCaseFirst> =
                    None;
                let mut seen_case_first = false;
                let mut f_case_level: Option<bool> = None;
                let mut seen_case_level = false;
                let mut f_country: Option<String> = None;
                let mut seen_country = false;
                let mut f_decomposition: Option<
                    crate::types::common_analysis::IcuCollationDecomposition,
                > = None;
                let mut seen_decomposition = false;
                let mut f_hiragana_quaternary_mode: Option<bool> = None;
                let mut seen_hiragana_quaternary_mode = false;
                let mut f_language: Option<String> = None;
                let mut seen_language = false;
                let mut f_numeric: Option<bool> = None;
                let mut seen_numeric = false;
                let mut f_rules: Option<String> = None;
                let mut seen_rules = false;
                let mut f_strength: Option<crate::types::common_analysis::IcuCollationStrength> =
                    None;
                let mut seen_strength = false;
                let mut f_type: Option<crate::types::common_analysis::IcuCollationTokenFilterType> =
                    None;
                let mut f_variable_top: Option<String> = None;
                let mut seen_variable_top = false;
                let mut f_variant: Option<String> = None;
                let mut seen_variant = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "alternate" => {
                            if seen_alternate {
                                return Err(serde::de::Error::duplicate_field("alternate"));
                            }
                            seen_alternate = true;
                            f_alternate = map.next_value::<Option<crate::types::common_analysis::IcuCollationAlternate>>()?;
                        }
                        "caseFirst" => {
                            if seen_case_first {
                                return Err(serde::de::Error::duplicate_field("caseFirst"));
                            }
                            seen_case_first = true;
                            f_case_first = map.next_value::<Option<crate::types::common_analysis::IcuCollationCaseFirst>>()?;
                        }
                        "caseLevel" => {
                            if seen_case_level {
                                return Err(serde::de::Error::duplicate_field("caseLevel"));
                            }
                            seen_case_level = true;
                            f_case_level = map.next_value::<Option<bool>>()?;
                        }
                        "country" => {
                            if seen_country {
                                return Err(serde::de::Error::duplicate_field("country"));
                            }
                            seen_country = true;
                            f_country = map.next_value::<Option<String>>()?;
                        }
                        "decomposition" => {
                            if seen_decomposition {
                                return Err(serde::de::Error::duplicate_field("decomposition"));
                            }
                            seen_decomposition = true;
                            f_decomposition =
                                map.next_value::<Option<
                                    crate::types::common_analysis::IcuCollationDecomposition,
                                >>()?;
                        }
                        "hiraganaQuaternaryMode" => {
                            if seen_hiragana_quaternary_mode {
                                return Err(serde::de::Error::duplicate_field(
                                    "hiraganaQuaternaryMode",
                                ));
                            }
                            seen_hiragana_quaternary_mode = true;
                            f_hiragana_quaternary_mode = map.next_value::<Option<bool>>()?;
                        }
                        "language" => {
                            if seen_language {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            seen_language = true;
                            f_language = map.next_value::<Option<String>>()?;
                        }
                        "numeric" => {
                            if seen_numeric {
                                return Err(serde::de::Error::duplicate_field("numeric"));
                            }
                            seen_numeric = true;
                            f_numeric = map.next_value::<Option<bool>>()?;
                        }
                        "rules" => {
                            if seen_rules {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            seen_rules = true;
                            f_rules = map.next_value::<Option<String>>()?;
                        }
                        "strength" => {
                            if seen_strength {
                                return Err(serde::de::Error::duplicate_field("strength"));
                            }
                            seen_strength = true;
                            f_strength = map.next_value::<Option<crate::types::common_analysis::IcuCollationStrength>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::IcuCollationTokenFilterType>()?);
                        }
                        "variableTop" => {
                            if seen_variable_top {
                                return Err(serde::de::Error::duplicate_field("variableTop"));
                            }
                            seen_variable_top = true;
                            f_variable_top = map.next_value::<Option<String>>()?;
                        }
                        "variant" => {
                            if seen_variant {
                                return Err(serde::de::Error::duplicate_field("variant"));
                            }
                            seen_variant = true;
                            f_variant = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IcuCollationTokenFilter {
                    version: f_version,
                    alternate: f_alternate,
                    case_first: f_case_first,
                    case_level: f_case_level,
                    country: f_country,
                    decomposition: f_decomposition,
                    hiragana_quaternary_mode: f_hiragana_quaternary_mode,
                    language: f_language,
                    numeric: f_numeric,
                    rules: f_rules,
                    strength: f_strength,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    variable_top: f_variable_top,
                    variant: f_variant,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IcuCollationTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.alternate {
            map.serialize_entry("alternate", v)?;
        }
        if let Some(v) = &self.case_first {
            map.serialize_entry("caseFirst", v)?;
        }
        if let Some(v) = &self.case_level {
            map.serialize_entry("caseLevel", v)?;
        }
        if let Some(v) = &self.country {
            map.serialize_entry("country", v)?;
        }
        if let Some(v) = &self.decomposition {
            map.serialize_entry("decomposition", v)?;
        }
        if let Some(v) = &self.hiragana_quaternary_mode {
            map.serialize_entry("hiraganaQuaternaryMode", v)?;
        }
        if let Some(v) = &self.language {
            map.serialize_entry("language", v)?;
        }
        if let Some(v) = &self.numeric {
            map.serialize_entry("numeric", v)?;
        }
        if let Some(v) = &self.rules {
            map.serialize_entry("rules", v)?;
        }
        if let Some(v) = &self.strength {
            map.serialize_entry("strength", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        if let Some(v) = &self.variable_top {
            map.serialize_entry("variableTop", v)?;
        }
        if let Some(v) = &self.variant {
            map.serialize_entry("variant", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuCollationTokenFilterType {
    #[doc = "`icu_collation`"]
    IcuCollation,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuCollationTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuCollationTokenFilterType::IcuCollation => "icu_collation",
            IcuCollationTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuCollationTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuCollationTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "icu_collation" => IcuCollationTokenFilterType::IcuCollation,
            _ => IcuCollationTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct IcuFoldingTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::IcuFoldingTokenFilterType,
    pub unicode_set_filter: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IcuFoldingTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IcuFoldingTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IcuFoldingTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_type: Option<crate::types::common_analysis::IcuFoldingTokenFilterType> =
                    None;
                let mut f_unicode_set_filter: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::IcuFoldingTokenFilterType>()?);
                        }
                        "unicode_set_filter" => {
                            if f_unicode_set_filter.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "unicode_set_filter",
                                ));
                            }
                            f_unicode_set_filter = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IcuFoldingTokenFilter {
                    version: f_version,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unicode_set_filter: f_unicode_set_filter
                        .ok_or_else(|| serde::de::Error::missing_field("unicode_set_filter"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IcuFoldingTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        map.serialize_entry("unicode_set_filter", &self.unicode_set_filter)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuFoldingTokenFilterType {
    #[doc = "`icu_folding`"]
    IcuFolding,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuFoldingTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuFoldingTokenFilterType::IcuFolding => "icu_folding",
            IcuFoldingTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuFoldingTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuFoldingTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "icu_folding" => IcuFoldingTokenFilterType::IcuFolding,
            _ => IcuFoldingTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct IcuNormalizationCharFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub mode: Option<crate::types::common_analysis::IcuNormalizationMode>,
    pub name: Option<crate::types::common_analysis::IcuNormalizationType>,
    pub r#type: crate::types::common_analysis::IcuNormalizationCharFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IcuNormalizationCharFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IcuNormalizationCharFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IcuNormalizationCharFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_mode: Option<crate::types::common_analysis::IcuNormalizationMode> = None;
                let mut seen_mode = false;
                let mut f_name: Option<crate::types::common_analysis::IcuNormalizationType> = None;
                let mut seen_name = false;
                let mut f_type: Option<
                    crate::types::common_analysis::IcuNormalizationCharFilterType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "mode" => {
                            if seen_mode {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            seen_mode = true;
                            f_mode = map.next_value::<Option<crate::types::common_analysis::IcuNormalizationMode>>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<crate::types::common_analysis::IcuNormalizationType>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::IcuNormalizationCharFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IcuNormalizationCharFilter {
                    version: f_version,
                    mode: f_mode,
                    name: f_name,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IcuNormalizationCharFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.mode {
            map.serialize_entry("mode", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuNormalizationCharFilterType {
    #[doc = "`icu_normalizer`"]
    IcuNormalizer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuNormalizationCharFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuNormalizationCharFilterType::IcuNormalizer => "icu_normalizer",
            IcuNormalizationCharFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuNormalizationCharFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuNormalizationCharFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "icu_normalizer" => IcuNormalizationCharFilterType::IcuNormalizer,
            _ => IcuNormalizationCharFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuNormalizationMode {
    #[doc = "`compose`"]
    Compose,
    #[doc = "`decompose`"]
    Decompose,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuNormalizationMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuNormalizationMode::Compose => "compose",
            IcuNormalizationMode::Decompose => "decompose",
            IcuNormalizationMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuNormalizationMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuNormalizationMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "compose" => IcuNormalizationMode::Compose,
            "decompose" => IcuNormalizationMode::Decompose,
            _ => IcuNormalizationMode::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct IcuNormalizationTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub name: crate::types::common_analysis::IcuNormalizationType,
    pub r#type: crate::types::common_analysis::IcuNormalizationTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IcuNormalizationTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IcuNormalizationTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IcuNormalizationTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_name: Option<crate::types::common_analysis::IcuNormalizationType> = None;
                let mut f_type: Option<
                    crate::types::common_analysis::IcuNormalizationTokenFilterType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<crate::types::common_analysis::IcuNormalizationType>()?);
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::IcuNormalizationTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IcuNormalizationTokenFilter {
                    version: f_version,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IcuNormalizationTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuNormalizationTokenFilterType {
    #[doc = "`icu_normalizer`"]
    IcuNormalizer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuNormalizationTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuNormalizationTokenFilterType::IcuNormalizer => "icu_normalizer",
            IcuNormalizationTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuNormalizationTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuNormalizationTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "icu_normalizer" => IcuNormalizationTokenFilterType::IcuNormalizer,
            _ => IcuNormalizationTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuNormalizationType {
    #[doc = "`nfc`"]
    Nfc,
    #[doc = "`nfkc`"]
    Nfkc,
    #[doc = "`nfkc_cf`"]
    NfkcCf,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuNormalizationType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuNormalizationType::Nfc => "nfc",
            IcuNormalizationType::Nfkc => "nfkc",
            IcuNormalizationType::NfkcCf => "nfkc_cf",
            IcuNormalizationType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuNormalizationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuNormalizationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "nfc" => IcuNormalizationType::Nfc,
            "nfkc" => IcuNormalizationType::Nfkc,
            "nfkc_cf" => IcuNormalizationType::NfkcCf,
            _ => IcuNormalizationType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct IcuTokenizer {
    pub version: Option<crate::types::common::VersionString>,
    pub rule_files: String,
    pub r#type: crate::types::common_analysis::IcuTokenizerType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IcuTokenizer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IcuTokenizer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IcuTokenizer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_rule_files: Option<String> = None;
                let mut f_type: Option<crate::types::common_analysis::IcuTokenizerType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "rule_files" => {
                            if f_rule_files.is_some() {
                                return Err(serde::de::Error::duplicate_field("rule_files"));
                            }
                            f_rule_files = Some(map.next_value::<String>()?);
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(
                                map.next_value::<crate::types::common_analysis::IcuTokenizerType>(
                                )?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IcuTokenizer {
                    version: f_version,
                    rule_files: f_rule_files
                        .ok_or_else(|| serde::de::Error::missing_field("rule_files"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IcuTokenizer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("rule_files", &self.rule_files)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuTokenizerType {
    #[doc = "`icu_tokenizer`"]
    IcuTokenizer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuTokenizerType::IcuTokenizer => "icu_tokenizer",
            IcuTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "icu_tokenizer" => IcuTokenizerType::IcuTokenizer,
            _ => IcuTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuTransformDirection {
    #[doc = "`forward`"]
    Forward,
    #[doc = "`reverse`"]
    Reverse,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuTransformDirection {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuTransformDirection::Forward => "forward",
            IcuTransformDirection::Reverse => "reverse",
            IcuTransformDirection::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuTransformDirection {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuTransformDirection {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "forward" => IcuTransformDirection::Forward,
            "reverse" => IcuTransformDirection::Reverse,
            _ => IcuTransformDirection::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct IcuTransformTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub dir: Option<crate::types::common_analysis::IcuTransformDirection>,
    pub id: String,
    pub r#type: crate::types::common_analysis::IcuTransformTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IcuTransformTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IcuTransformTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IcuTransformTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_dir: Option<crate::types::common_analysis::IcuTransformDirection> = None;
                let mut seen_dir = false;
                let mut f_id: Option<String> = None;
                let mut f_type: Option<crate::types::common_analysis::IcuTransformTokenFilterType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "dir" => {
                            if seen_dir {
                                return Err(serde::de::Error::duplicate_field("dir"));
                            }
                            seen_dir = true;
                            f_dir = map.next_value::<Option<crate::types::common_analysis::IcuTransformDirection>>()?;
                        }
                        "id" => {
                            if f_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            f_id = Some(map.next_value::<String>()?);
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::IcuTransformTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IcuTransformTokenFilter {
                    version: f_version,
                    dir: f_dir,
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("id"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IcuTransformTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.dir {
            map.serialize_entry("dir", v)?;
        }
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuTransformTokenFilterType {
    #[doc = "`icu_transform`"]
    IcuTransform,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuTransformTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuTransformTokenFilterType::IcuTransform => "icu_transform",
            IcuTransformTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuTransformTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuTransformTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "icu_transform" => IcuTransformTokenFilterType::IcuTransform,
            _ => IcuTransformTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct KStemTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::KStemTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for KStemTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = KStemTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a KStemTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_type: Option<crate::types::common_analysis::KStemTokenFilterType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::KStemTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(KStemTokenFilter {
                    version: f_version,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for KStemTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KStemTokenFilterType {
    #[doc = "`kstem`"]
    Kstem,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KStemTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KStemTokenFilterType::Kstem => "kstem",
            KStemTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KStemTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KStemTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "kstem" => KStemTokenFilterType::Kstem,
            _ => KStemTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeepTypesMode {
    #[doc = "`exclude`"]
    Exclude,
    #[doc = "`include`"]
    Include,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KeepTypesMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KeepTypesMode::Exclude => "exclude",
            KeepTypesMode::Include => "include",
            KeepTypesMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KeepTypesMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KeepTypesMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "exclude" => KeepTypesMode::Exclude,
            "include" => KeepTypesMode::Include,
            _ => KeepTypesMode::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct KeepTypesTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub mode: Option<crate::types::common_analysis::KeepTypesMode>,
    pub r#type: crate::types::common_analysis::KeepTypesTokenFilterType,
    pub types: Option<Vec<String>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for KeepTypesTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = KeepTypesTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a KeepTypesTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_mode: Option<crate::types::common_analysis::KeepTypesMode> = None;
                let mut seen_mode = false;
                let mut f_type: Option<crate::types::common_analysis::KeepTypesTokenFilterType> =
                    None;
                let mut f_types: Option<Vec<String>> = None;
                let mut seen_types = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "mode" => {
                            if seen_mode {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            seen_mode = true;
                            f_mode = map
                                .next_value::<Option<crate::types::common_analysis::KeepTypesMode>>(
                                )?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::KeepTypesTokenFilterType>()?);
                        }
                        "types" => {
                            if seen_types {
                                return Err(serde::de::Error::duplicate_field("types"));
                            }
                            seen_types = true;
                            f_types = map.next_value::<Option<Vec<String>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(KeepTypesTokenFilter {
                    version: f_version,
                    mode: f_mode,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    types: f_types,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for KeepTypesTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.mode {
            map.serialize_entry("mode", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        if let Some(v) = &self.types {
            map.serialize_entry("types", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeepTypesTokenFilterType {
    #[doc = "`keep_types`"]
    KeepTypes,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KeepTypesTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KeepTypesTokenFilterType::KeepTypes => "keep_types",
            KeepTypesTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KeepTypesTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KeepTypesTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "keep_types" => KeepTypesTokenFilterType::KeepTypes,
            _ => KeepTypesTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct KeepWordsTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub keep_words: Option<Vec<String>>,
    pub keep_words_case: Option<bool>,
    pub keep_words_path: Option<String>,
    pub r#type: crate::types::common_analysis::KeepWordsTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for KeepWordsTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = KeepWordsTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a KeepWordsTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_keep_words: Option<Vec<String>> = None;
                let mut seen_keep_words = false;
                let mut f_keep_words_case: Option<bool> = None;
                let mut seen_keep_words_case = false;
                let mut f_keep_words_path: Option<String> = None;
                let mut seen_keep_words_path = false;
                let mut f_type: Option<crate::types::common_analysis::KeepWordsTokenFilterType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "keep_words" => {
                            if seen_keep_words {
                                return Err(serde::de::Error::duplicate_field("keep_words"));
                            }
                            seen_keep_words = true;
                            f_keep_words = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "keep_words_case" => {
                            if seen_keep_words_case {
                                return Err(serde::de::Error::duplicate_field("keep_words_case"));
                            }
                            seen_keep_words_case = true;
                            f_keep_words_case = map.next_value::<Option<bool>>()?;
                        }
                        "keep_words_path" => {
                            if seen_keep_words_path {
                                return Err(serde::de::Error::duplicate_field("keep_words_path"));
                            }
                            seen_keep_words_path = true;
                            f_keep_words_path = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::KeepWordsTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(KeepWordsTokenFilter {
                    version: f_version,
                    keep_words: f_keep_words,
                    keep_words_case: f_keep_words_case,
                    keep_words_path: f_keep_words_path,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for KeepWordsTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.keep_words {
            map.serialize_entry("keep_words", v)?;
        }
        if let Some(v) = &self.keep_words_case {
            map.serialize_entry("keep_words_case", v)?;
        }
        if let Some(v) = &self.keep_words_path {
            map.serialize_entry("keep_words_path", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeepWordsTokenFilterType {
    #[doc = "`keep`"]
    Keep,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KeepWordsTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KeepWordsTokenFilterType::Keep => "keep",
            KeepWordsTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KeepWordsTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KeepWordsTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "keep" => KeepWordsTokenFilterType::Keep,
            _ => KeepWordsTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct KeywordAnalyzer {
    pub r#type: crate::types::common_analysis::KeywordAnalyzerType,
    pub version: Option<crate::types::common::VersionString>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for KeywordAnalyzer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = KeywordAnalyzer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a KeywordAnalyzer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_type: Option<crate::types::common_analysis::KeywordAnalyzerType> = None;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::KeywordAnalyzerType>()?);
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(KeywordAnalyzer {
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for KeywordAnalyzer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("type", &self.r#type)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeywordAnalyzerType {
    #[doc = "`keyword`"]
    Keyword,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KeywordAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KeywordAnalyzerType::Keyword => "keyword",
            KeywordAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KeywordAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KeywordAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "keyword" => KeywordAnalyzerType::Keyword,
            _ => KeywordAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct KeywordMarkerTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub ignore_case: Option<bool>,
    pub keywords: Option<Vec<String>>,
    pub keywords_path: Option<String>,
    pub keywords_pattern: Option<String>,
    pub r#type: crate::types::common_analysis::KeywordMarkerTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for KeywordMarkerTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = KeywordMarkerTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a KeywordMarkerTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_ignore_case: Option<bool> = None;
                let mut seen_ignore_case = false;
                let mut f_keywords: Option<Vec<String>> = None;
                let mut seen_keywords = false;
                let mut f_keywords_path: Option<String> = None;
                let mut seen_keywords_path = false;
                let mut f_keywords_pattern: Option<String> = None;
                let mut seen_keywords_pattern = false;
                let mut f_type: Option<
                    crate::types::common_analysis::KeywordMarkerTokenFilterType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "ignore_case" => {
                            if seen_ignore_case {
                                return Err(serde::de::Error::duplicate_field("ignore_case"));
                            }
                            seen_ignore_case = true;
                            f_ignore_case = map.next_value::<Option<bool>>()?;
                        }
                        "keywords" => {
                            if seen_keywords {
                                return Err(serde::de::Error::duplicate_field("keywords"));
                            }
                            seen_keywords = true;
                            f_keywords = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "keywords_path" => {
                            if seen_keywords_path {
                                return Err(serde::de::Error::duplicate_field("keywords_path"));
                            }
                            seen_keywords_path = true;
                            f_keywords_path = map.next_value::<Option<String>>()?;
                        }
                        "keywords_pattern" => {
                            if seen_keywords_pattern {
                                return Err(serde::de::Error::duplicate_field("keywords_pattern"));
                            }
                            seen_keywords_pattern = true;
                            f_keywords_pattern = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::KeywordMarkerTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(KeywordMarkerTokenFilter {
                    version: f_version,
                    ignore_case: f_ignore_case,
                    keywords: f_keywords,
                    keywords_path: f_keywords_path,
                    keywords_pattern: f_keywords_pattern,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for KeywordMarkerTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.ignore_case {
            map.serialize_entry("ignore_case", v)?;
        }
        if let Some(v) = &self.keywords {
            map.serialize_entry("keywords", v)?;
        }
        if let Some(v) = &self.keywords_path {
            map.serialize_entry("keywords_path", v)?;
        }
        if let Some(v) = &self.keywords_pattern {
            map.serialize_entry("keywords_pattern", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeywordMarkerTokenFilterType {
    #[doc = "`keyword_marker`"]
    KeywordMarker,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KeywordMarkerTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KeywordMarkerTokenFilterType::KeywordMarker => "keyword_marker",
            KeywordMarkerTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KeywordMarkerTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KeywordMarkerTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "keyword_marker" => KeywordMarkerTokenFilterType::KeywordMarker,
            _ => KeywordMarkerTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct KeywordTokenizer {
    pub version: Option<crate::types::common::VersionString>,
    pub buffer_size: i64,
    pub r#type: crate::types::common_analysis::KeywordTokenizerType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for KeywordTokenizer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = KeywordTokenizer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a KeywordTokenizer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_buffer_size: Option<i64> = None;
                let mut f_type: Option<crate::types::common_analysis::KeywordTokenizerType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "buffer_size" => {
                            if f_buffer_size.is_some() {
                                return Err(serde::de::Error::duplicate_field("buffer_size"));
                            }
                            f_buffer_size = Some(map.next_value::<i64>()?);
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::KeywordTokenizerType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(KeywordTokenizer {
                    version: f_version,
                    buffer_size: f_buffer_size
                        .ok_or_else(|| serde::de::Error::missing_field("buffer_size"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for KeywordTokenizer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("buffer_size", &self.buffer_size)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeywordTokenizerType {
    #[doc = "`keyword`"]
    Keyword,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KeywordTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KeywordTokenizerType::Keyword => "keyword",
            KeywordTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KeywordTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KeywordTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "keyword" => KeywordTokenizerType::Keyword,
            _ => KeywordTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct KuromojiAnalyzer {
    pub mode: crate::types::common_analysis::KuromojiTokenizationMode,
    pub r#type: crate::types::common_analysis::KuromojiAnalyzerType,
    pub user_dictionary: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for KuromojiAnalyzer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = KuromojiAnalyzer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a KuromojiAnalyzer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_mode: Option<crate::types::common_analysis::KuromojiTokenizationMode> =
                    None;
                let mut f_type: Option<crate::types::common_analysis::KuromojiAnalyzerType> = None;
                let mut f_user_dictionary: Option<String> = None;
                let mut seen_user_dictionary = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "mode" => {
                            if f_mode.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            f_mode = Some(map.next_value::<crate::types::common_analysis::KuromojiTokenizationMode>()?);
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::KuromojiAnalyzerType>()?);
                        }
                        "user_dictionary" => {
                            if seen_user_dictionary {
                                return Err(serde::de::Error::duplicate_field("user_dictionary"));
                            }
                            seen_user_dictionary = true;
                            f_user_dictionary = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(KuromojiAnalyzer {
                    mode: f_mode.ok_or_else(|| serde::de::Error::missing_field("mode"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    user_dictionary: f_user_dictionary,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for KuromojiAnalyzer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("mode", &self.mode)?;
        map.serialize_entry("type", &self.r#type)?;
        if let Some(v) = &self.user_dictionary {
            map.serialize_entry("user_dictionary", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KuromojiAnalyzerType {
    #[doc = "`kuromoji`"]
    Kuromoji,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KuromojiAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KuromojiAnalyzerType::Kuromoji => "kuromoji",
            KuromojiAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KuromojiAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KuromojiAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "kuromoji" => KuromojiAnalyzerType::Kuromoji,
            _ => KuromojiAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct KuromojiIterationMarkCharFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub normalize_kana: bool,
    pub normalize_kanji: bool,
    pub r#type: crate::types::common_analysis::KuromojiIterationMarkCharFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for KuromojiIterationMarkCharFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = KuromojiIterationMarkCharFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a KuromojiIterationMarkCharFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_normalize_kana: Option<bool> = None;
                let mut f_normalize_kanji: Option<bool> = None;
                let mut f_type: Option<
                    crate::types::common_analysis::KuromojiIterationMarkCharFilterType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "normalize_kana" => {
                            if f_normalize_kana.is_some() {
                                return Err(serde::de::Error::duplicate_field("normalize_kana"));
                            }
                            f_normalize_kana = Some(map.next_value::<bool>()?);
                        }
                        "normalize_kanji" => {
                            if f_normalize_kanji.is_some() {
                                return Err(serde::de::Error::duplicate_field("normalize_kanji"));
                            }
                            f_normalize_kanji = Some(map.next_value::<bool>()?);
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::KuromojiIterationMarkCharFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(KuromojiIterationMarkCharFilter {
                    version: f_version,
                    normalize_kana: f_normalize_kana
                        .ok_or_else(|| serde::de::Error::missing_field("normalize_kana"))?,
                    normalize_kanji: f_normalize_kanji
                        .ok_or_else(|| serde::de::Error::missing_field("normalize_kanji"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for KuromojiIterationMarkCharFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("normalize_kana", &self.normalize_kana)?;
        map.serialize_entry("normalize_kanji", &self.normalize_kanji)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KuromojiIterationMarkCharFilterType {
    #[doc = "`kuromoji_iteration_mark`"]
    KuromojiIterationMark,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KuromojiIterationMarkCharFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KuromojiIterationMarkCharFilterType::KuromojiIterationMark => "kuromoji_iteration_mark",
            KuromojiIterationMarkCharFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KuromojiIterationMarkCharFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KuromojiIterationMarkCharFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "kuromoji_iteration_mark" => KuromojiIterationMarkCharFilterType::KuromojiIterationMark,
            _ => KuromojiIterationMarkCharFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct KuromojiPartOfSpeechTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub stoptags: Vec<String>,
    pub r#type: crate::types::common_analysis::KuromojiPartOfSpeechTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for KuromojiPartOfSpeechTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = KuromojiPartOfSpeechTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a KuromojiPartOfSpeechTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_stoptags: Option<Vec<String>> = None;
                let mut f_type: Option<
                    crate::types::common_analysis::KuromojiPartOfSpeechTokenFilterType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "stoptags" => {
                            if f_stoptags.is_some() {
                                return Err(serde::de::Error::duplicate_field("stoptags"));
                            }
                            f_stoptags = Some(map.next_value::<Vec<String>>()?);
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::KuromojiPartOfSpeechTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(KuromojiPartOfSpeechTokenFilter {
                    version: f_version,
                    stoptags: f_stoptags
                        .ok_or_else(|| serde::de::Error::missing_field("stoptags"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for KuromojiPartOfSpeechTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("stoptags", &self.stoptags)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KuromojiPartOfSpeechTokenFilterType {
    #[doc = "`kuromoji_part_of_speech`"]
    KuromojiPartOfSpeech,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KuromojiPartOfSpeechTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KuromojiPartOfSpeechTokenFilterType::KuromojiPartOfSpeech => "kuromoji_part_of_speech",
            KuromojiPartOfSpeechTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KuromojiPartOfSpeechTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KuromojiPartOfSpeechTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "kuromoji_part_of_speech" => KuromojiPartOfSpeechTokenFilterType::KuromojiPartOfSpeech,
            _ => KuromojiPartOfSpeechTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct KuromojiReadingFormTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::KuromojiReadingFormTokenFilterType,
    pub use_romaji: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for KuromojiReadingFormTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = KuromojiReadingFormTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a KuromojiReadingFormTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_type: Option<
                    crate::types::common_analysis::KuromojiReadingFormTokenFilterType,
                > = None;
                let mut f_use_romaji: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::KuromojiReadingFormTokenFilterType>()?);
                        }
                        "use_romaji" => {
                            if f_use_romaji.is_some() {
                                return Err(serde::de::Error::duplicate_field("use_romaji"));
                            }
                            f_use_romaji = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(KuromojiReadingFormTokenFilter {
                    version: f_version,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    use_romaji: f_use_romaji
                        .ok_or_else(|| serde::de::Error::missing_field("use_romaji"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for KuromojiReadingFormTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        map.serialize_entry("use_romaji", &self.use_romaji)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KuromojiReadingFormTokenFilterType {
    #[doc = "`kuromoji_readingform`"]
    KuromojiReadingform,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KuromojiReadingFormTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KuromojiReadingFormTokenFilterType::KuromojiReadingform => "kuromoji_readingform",
            KuromojiReadingFormTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KuromojiReadingFormTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KuromojiReadingFormTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "kuromoji_readingform" => KuromojiReadingFormTokenFilterType::KuromojiReadingform,
            _ => KuromojiReadingFormTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct KuromojiStemmerTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub minimum_length: i64,
    pub r#type: crate::types::common_analysis::KuromojiStemmerTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for KuromojiStemmerTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = KuromojiStemmerTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a KuromojiStemmerTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_minimum_length: Option<i64> = None;
                let mut f_type: Option<
                    crate::types::common_analysis::KuromojiStemmerTokenFilterType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "minimum_length" => {
                            if f_minimum_length.is_some() {
                                return Err(serde::de::Error::duplicate_field("minimum_length"));
                            }
                            f_minimum_length = Some(map.next_value::<i64>()?);
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::KuromojiStemmerTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(KuromojiStemmerTokenFilter {
                    version: f_version,
                    minimum_length: f_minimum_length
                        .ok_or_else(|| serde::de::Error::missing_field("minimum_length"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for KuromojiStemmerTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("minimum_length", &self.minimum_length)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KuromojiStemmerTokenFilterType {
    #[doc = "`kuromoji_stemmer`"]
    KuromojiStemmer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KuromojiStemmerTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KuromojiStemmerTokenFilterType::KuromojiStemmer => "kuromoji_stemmer",
            KuromojiStemmerTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KuromojiStemmerTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KuromojiStemmerTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "kuromoji_stemmer" => KuromojiStemmerTokenFilterType::KuromojiStemmer,
            _ => KuromojiStemmerTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KuromojiTokenizationMode {
    #[doc = "`extended`"]
    Extended,
    #[doc = "`normal`"]
    Normal,
    #[doc = "`search`"]
    Search,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KuromojiTokenizationMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KuromojiTokenizationMode::Extended => "extended",
            KuromojiTokenizationMode::Normal => "normal",
            KuromojiTokenizationMode::Search => "search",
            KuromojiTokenizationMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KuromojiTokenizationMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KuromojiTokenizationMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "extended" => KuromojiTokenizationMode::Extended,
            "normal" => KuromojiTokenizationMode::Normal,
            "search" => KuromojiTokenizationMode::Search,
            _ => KuromojiTokenizationMode::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct KuromojiTokenizer {
    pub version: Option<crate::types::common::VersionString>,
    pub discard_compound_token: Option<bool>,
    pub discard_punctuation: Option<bool>,
    pub mode: crate::types::common_analysis::KuromojiTokenizationMode,
    pub nbest_cost: Option<i64>,
    pub nbest_examples: Option<String>,
    pub r#type: crate::types::common_analysis::KuromojiTokenizerType,
    pub user_dictionary: Option<String>,
    pub user_dictionary_rules: Option<Vec<String>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for KuromojiTokenizer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = KuromojiTokenizer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a KuromojiTokenizer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_discard_compound_token: Option<bool> = None;
                let mut seen_discard_compound_token = false;
                let mut f_discard_punctuation: Option<bool> = None;
                let mut seen_discard_punctuation = false;
                let mut f_mode: Option<crate::types::common_analysis::KuromojiTokenizationMode> =
                    None;
                let mut f_nbest_cost: Option<i64> = None;
                let mut seen_nbest_cost = false;
                let mut f_nbest_examples: Option<String> = None;
                let mut seen_nbest_examples = false;
                let mut f_type: Option<crate::types::common_analysis::KuromojiTokenizerType> = None;
                let mut f_user_dictionary: Option<String> = None;
                let mut seen_user_dictionary = false;
                let mut f_user_dictionary_rules: Option<Vec<String>> = None;
                let mut seen_user_dictionary_rules = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "discard_compound_token" => {
                            if seen_discard_compound_token {
                                return Err(serde::de::Error::duplicate_field(
                                    "discard_compound_token",
                                ));
                            }
                            seen_discard_compound_token = true;
                            f_discard_compound_token = map.next_value::<Option<bool>>()?;
                        }
                        "discard_punctuation" => {
                            if seen_discard_punctuation {
                                return Err(serde::de::Error::duplicate_field(
                                    "discard_punctuation",
                                ));
                            }
                            seen_discard_punctuation = true;
                            f_discard_punctuation = map.next_value::<Option<bool>>()?;
                        }
                        "mode" => {
                            if f_mode.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            f_mode = Some(map.next_value::<crate::types::common_analysis::KuromojiTokenizationMode>()?);
                        }
                        "nbest_cost" => {
                            if seen_nbest_cost {
                                return Err(serde::de::Error::duplicate_field("nbest_cost"));
                            }
                            seen_nbest_cost = true;
                            f_nbest_cost = map.next_value::<Option<i64>>()?;
                        }
                        "nbest_examples" => {
                            if seen_nbest_examples {
                                return Err(serde::de::Error::duplicate_field("nbest_examples"));
                            }
                            seen_nbest_examples = true;
                            f_nbest_examples = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::KuromojiTokenizerType>()?);
                        }
                        "user_dictionary" => {
                            if seen_user_dictionary {
                                return Err(serde::de::Error::duplicate_field("user_dictionary"));
                            }
                            seen_user_dictionary = true;
                            f_user_dictionary = map.next_value::<Option<String>>()?;
                        }
                        "user_dictionary_rules" => {
                            if seen_user_dictionary_rules {
                                return Err(serde::de::Error::duplicate_field(
                                    "user_dictionary_rules",
                                ));
                            }
                            seen_user_dictionary_rules = true;
                            f_user_dictionary_rules = map.next_value::<Option<Vec<String>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(KuromojiTokenizer {
                    version: f_version,
                    discard_compound_token: f_discard_compound_token,
                    discard_punctuation: f_discard_punctuation,
                    mode: f_mode.ok_or_else(|| serde::de::Error::missing_field("mode"))?,
                    nbest_cost: f_nbest_cost,
                    nbest_examples: f_nbest_examples,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    user_dictionary: f_user_dictionary,
                    user_dictionary_rules: f_user_dictionary_rules,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for KuromojiTokenizer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.discard_compound_token {
            map.serialize_entry("discard_compound_token", v)?;
        }
        if let Some(v) = &self.discard_punctuation {
            map.serialize_entry("discard_punctuation", v)?;
        }
        map.serialize_entry("mode", &self.mode)?;
        if let Some(v) = &self.nbest_cost {
            map.serialize_entry("nbest_cost", v)?;
        }
        if let Some(v) = &self.nbest_examples {
            map.serialize_entry("nbest_examples", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        if let Some(v) = &self.user_dictionary {
            map.serialize_entry("user_dictionary", v)?;
        }
        if let Some(v) = &self.user_dictionary_rules {
            map.serialize_entry("user_dictionary_rules", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KuromojiTokenizerType {
    #[doc = "`kuromoji_tokenizer`"]
    KuromojiTokenizer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KuromojiTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KuromojiTokenizerType::KuromojiTokenizer => "kuromoji_tokenizer",
            KuromojiTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KuromojiTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KuromojiTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "kuromoji_tokenizer" => KuromojiTokenizerType::KuromojiTokenizer,
            _ => KuromojiTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    #[doc = "`Arabic`"]
    Arabic,
    #[doc = "`Armenian`"]
    Armenian,
    #[doc = "`Basque`"]
    Basque,
    #[doc = "`Brazilian`"]
    Brazilian,
    #[doc = "`Bulgarian`"]
    Bulgarian,
    #[doc = "`Catalan`"]
    Catalan,
    #[doc = "`Chinese`"]
    Chinese,
    #[doc = "`Cjk`"]
    Cjk,
    #[doc = "`Czech`"]
    Czech,
    #[doc = "`Danish`"]
    Danish,
    #[doc = "`Dutch`"]
    Dutch,
    #[doc = "`English`"]
    English,
    #[doc = "`Estonian`"]
    Estonian,
    #[doc = "`Finnish`"]
    Finnish,
    #[doc = "`French`"]
    French,
    #[doc = "`Galician`"]
    Galician,
    #[doc = "`German`"]
    German,
    #[doc = "`Greek`"]
    Greek,
    #[doc = "`Hindi`"]
    Hindi,
    #[doc = "`Hungarian`"]
    Hungarian,
    #[doc = "`Indonesian`"]
    Indonesian,
    #[doc = "`Irish`"]
    Irish,
    #[doc = "`Italian`"]
    Italian,
    #[doc = "`Latvian`"]
    Latvian,
    #[doc = "`Norwegian`"]
    Norwegian,
    #[doc = "`Persian`"]
    Persian,
    #[doc = "`Portuguese`"]
    Portuguese,
    #[doc = "`Romanian`"]
    Romanian,
    #[doc = "`Russian`"]
    Russian,
    #[doc = "`Sorani`"]
    Sorani,
    #[doc = "`Spanish`"]
    Spanish,
    #[doc = "`Swedish`"]
    Swedish,
    #[doc = "`Thai`"]
    Thai,
    #[doc = "`Turkish`"]
    Turkish,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl Language {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            Language::Arabic => "Arabic",
            Language::Armenian => "Armenian",
            Language::Basque => "Basque",
            Language::Brazilian => "Brazilian",
            Language::Bulgarian => "Bulgarian",
            Language::Catalan => "Catalan",
            Language::Chinese => "Chinese",
            Language::Cjk => "Cjk",
            Language::Czech => "Czech",
            Language::Danish => "Danish",
            Language::Dutch => "Dutch",
            Language::English => "English",
            Language::Estonian => "Estonian",
            Language::Finnish => "Finnish",
            Language::French => "French",
            Language::Galician => "Galician",
            Language::German => "German",
            Language::Greek => "Greek",
            Language::Hindi => "Hindi",
            Language::Hungarian => "Hungarian",
            Language::Indonesian => "Indonesian",
            Language::Irish => "Irish",
            Language::Italian => "Italian",
            Language::Latvian => "Latvian",
            Language::Norwegian => "Norwegian",
            Language::Persian => "Persian",
            Language::Portuguese => "Portuguese",
            Language::Romanian => "Romanian",
            Language::Russian => "Russian",
            Language::Sorani => "Sorani",
            Language::Spanish => "Spanish",
            Language::Swedish => "Swedish",
            Language::Thai => "Thai",
            Language::Turkish => "Turkish",
            Language::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for Language {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "Arabic" => Language::Arabic,
            "Armenian" => Language::Armenian,
            "Basque" => Language::Basque,
            "Brazilian" => Language::Brazilian,
            "Bulgarian" => Language::Bulgarian,
            "Catalan" => Language::Catalan,
            "Chinese" => Language::Chinese,
            "Cjk" => Language::Cjk,
            "Czech" => Language::Czech,
            "Danish" => Language::Danish,
            "Dutch" => Language::Dutch,
            "English" => Language::English,
            "Estonian" => Language::Estonian,
            "Finnish" => Language::Finnish,
            "French" => Language::French,
            "Galician" => Language::Galician,
            "German" => Language::German,
            "Greek" => Language::Greek,
            "Hindi" => Language::Hindi,
            "Hungarian" => Language::Hungarian,
            "Indonesian" => Language::Indonesian,
            "Irish" => Language::Irish,
            "Italian" => Language::Italian,
            "Latvian" => Language::Latvian,
            "Norwegian" => Language::Norwegian,
            "Persian" => Language::Persian,
            "Portuguese" => Language::Portuguese,
            "Romanian" => Language::Romanian,
            "Russian" => Language::Russian,
            "Sorani" => Language::Sorani,
            "Spanish" => Language::Spanish,
            "Swedish" => Language::Swedish,
            "Thai" => Language::Thai,
            "Turkish" => Language::Turkish,
            _ => Language::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct LanguageAnalyzer {
    pub language: crate::types::common_analysis::Language,
    pub stem_exclusion: Vec<String>,
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    pub stopwords_path: Option<String>,
    pub r#type: crate::types::common_analysis::LanguageAnalyzerType,
    pub version: Option<crate::types::common::VersionString>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for LanguageAnalyzer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = LanguageAnalyzer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a LanguageAnalyzer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_language: Option<crate::types::common_analysis::Language> = None;
                let mut f_stem_exclusion: Option<Vec<String>> = None;
                let mut f_stopwords: Option<crate::types::common_analysis::StopWords> = None;
                let mut seen_stopwords = false;
                let mut f_stopwords_path: Option<String> = None;
                let mut seen_stopwords_path = false;
                let mut f_type: Option<crate::types::common_analysis::LanguageAnalyzerType> = None;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "language" => {
                            if f_language.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            f_language =
                                Some(map.next_value::<crate::types::common_analysis::Language>()?);
                        }
                        "stem_exclusion" => {
                            if f_stem_exclusion.is_some() {
                                return Err(serde::de::Error::duplicate_field("stem_exclusion"));
                            }
                            f_stem_exclusion = Some(map.next_value::<Vec<String>>()?);
                        }
                        "stopwords" => {
                            if seen_stopwords {
                                return Err(serde::de::Error::duplicate_field("stopwords"));
                            }
                            seen_stopwords = true;
                            f_stopwords = map
                                .next_value::<Option<crate::types::common_analysis::StopWords>>()?;
                        }
                        "stopwords_path" => {
                            if seen_stopwords_path {
                                return Err(serde::de::Error::duplicate_field("stopwords_path"));
                            }
                            seen_stopwords_path = true;
                            f_stopwords_path = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::LanguageAnalyzerType>()?);
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(LanguageAnalyzer {
                    language: f_language
                        .ok_or_else(|| serde::de::Error::missing_field("language"))?,
                    stem_exclusion: f_stem_exclusion
                        .ok_or_else(|| serde::de::Error::missing_field("stem_exclusion"))?,
                    stopwords: f_stopwords,
                    stopwords_path: f_stopwords_path,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for LanguageAnalyzer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("language", &self.language)?;
        map.serialize_entry("stem_exclusion", &self.stem_exclusion)?;
        if let Some(v) = &self.stopwords {
            map.serialize_entry("stopwords", v)?;
        }
        if let Some(v) = &self.stopwords_path {
            map.serialize_entry("stopwords_path", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LanguageAnalyzerType {
    #[doc = "`language`"]
    Language,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl LanguageAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            LanguageAnalyzerType::Language => "language",
            LanguageAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for LanguageAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LanguageAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "language" => LanguageAnalyzerType::Language,
            _ => LanguageAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct LengthTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub max: Option<i64>,
    pub min: Option<i64>,
    pub r#type: crate::types::common_analysis::LengthTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for LengthTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = LengthTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a LengthTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_max: Option<i64> = None;
                let mut seen_max = false;
                let mut f_min: Option<i64> = None;
                let mut seen_min = false;
                let mut f_type: Option<crate::types::common_analysis::LengthTokenFilterType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "max" => {
                            if seen_max {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            seen_max = true;
                            f_max = map.next_value::<Option<i64>>()?;
                        }
                        "min" => {
                            if seen_min {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            seen_min = true;
                            f_min = map.next_value::<Option<i64>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::LengthTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(LengthTokenFilter {
                    version: f_version,
                    max: f_max,
                    min: f_min,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for LengthTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.max {
            map.serialize_entry("max", v)?;
        }
        if let Some(v) = &self.min {
            map.serialize_entry("min", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LengthTokenFilterType {
    #[doc = "`length`"]
    Length,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl LengthTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            LengthTokenFilterType::Length => "length",
            LengthTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for LengthTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LengthTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "length" => LengthTokenFilterType::Length,
            _ => LengthTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct LetterTokenizer {
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::LetterTokenizerType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for LetterTokenizer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = LetterTokenizer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a LetterTokenizer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_type: Option<crate::types::common_analysis::LetterTokenizerType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::LetterTokenizerType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(LetterTokenizer {
                    version: f_version,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for LetterTokenizer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LetterTokenizerType {
    #[doc = "`letter`"]
    Letter,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl LetterTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            LetterTokenizerType::Letter => "letter",
            LetterTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for LetterTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LetterTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "letter" => LetterTokenizerType::Letter,
            _ => LetterTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct LimitTokenCountTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub consume_all_tokens: Option<bool>,
    pub max_token_count: Option<crate::types::common::StringifiedInteger>,
    pub r#type: crate::types::common_analysis::LimitTokenCountTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for LimitTokenCountTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = LimitTokenCountTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a LimitTokenCountTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_consume_all_tokens: Option<bool> = None;
                let mut seen_consume_all_tokens = false;
                let mut f_max_token_count: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_max_token_count = false;
                let mut f_type: Option<
                    crate::types::common_analysis::LimitTokenCountTokenFilterType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "consume_all_tokens" => {
                            if seen_consume_all_tokens {
                                return Err(serde::de::Error::duplicate_field(
                                    "consume_all_tokens",
                                ));
                            }
                            seen_consume_all_tokens = true;
                            f_consume_all_tokens = map.next_value::<Option<bool>>()?;
                        }
                        "max_token_count" => {
                            if seen_max_token_count {
                                return Err(serde::de::Error::duplicate_field("max_token_count"));
                            }
                            seen_max_token_count = true;
                            f_max_token_count = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::LimitTokenCountTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(LimitTokenCountTokenFilter {
                    version: f_version,
                    consume_all_tokens: f_consume_all_tokens,
                    max_token_count: f_max_token_count,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for LimitTokenCountTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.consume_all_tokens {
            map.serialize_entry("consume_all_tokens", v)?;
        }
        if let Some(v) = &self.max_token_count {
            map.serialize_entry("max_token_count", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LimitTokenCountTokenFilterType {
    #[doc = "`limit`"]
    Limit,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl LimitTokenCountTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            LimitTokenCountTokenFilterType::Limit => "limit",
            LimitTokenCountTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for LimitTokenCountTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LimitTokenCountTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "limit" => LimitTokenCountTokenFilterType::Limit,
            _ => LimitTokenCountTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct LowercaseNormalizer {
    pub r#type: crate::types::common_analysis::LowercaseNormalizerType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for LowercaseNormalizer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = LowercaseNormalizer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a LowercaseNormalizer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_type: Option<crate::types::common_analysis::LowercaseNormalizerType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::LowercaseNormalizerType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(LowercaseNormalizer {
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for LowercaseNormalizer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LowercaseNormalizerType {
    #[doc = "`lowercase`"]
    Lowercase,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl LowercaseNormalizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            LowercaseNormalizerType::Lowercase => "lowercase",
            LowercaseNormalizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for LowercaseNormalizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LowercaseNormalizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "lowercase" => LowercaseNormalizerType::Lowercase,
            _ => LowercaseNormalizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct LowercaseTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub language: Option<String>,
    pub r#type: crate::types::common_analysis::LowercaseTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for LowercaseTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = LowercaseTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a LowercaseTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_language: Option<String> = None;
                let mut seen_language = false;
                let mut f_type: Option<crate::types::common_analysis::LowercaseTokenFilterType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "language" => {
                            if seen_language {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            seen_language = true;
                            f_language = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::LowercaseTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(LowercaseTokenFilter {
                    version: f_version,
                    language: f_language,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for LowercaseTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.language {
            map.serialize_entry("language", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LowercaseTokenFilterType {
    #[doc = "`lowercase`"]
    Lowercase,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl LowercaseTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            LowercaseTokenFilterType::Lowercase => "lowercase",
            LowercaseTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for LowercaseTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LowercaseTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "lowercase" => LowercaseTokenFilterType::Lowercase,
            _ => LowercaseTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct LowercaseTokenizer {
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::LowercaseTokenizerType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for LowercaseTokenizer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = LowercaseTokenizer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a LowercaseTokenizer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_type: Option<crate::types::common_analysis::LowercaseTokenizerType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::LowercaseTokenizerType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(LowercaseTokenizer {
                    version: f_version,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for LowercaseTokenizer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LowercaseTokenizerType {
    #[doc = "`lowercase`"]
    Lowercase,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl LowercaseTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            LowercaseTokenizerType::Lowercase => "lowercase",
            LowercaseTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for LowercaseTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LowercaseTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "lowercase" => LowercaseTokenizerType::Lowercase,
            _ => LowercaseTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct MappingCharFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub mappings: Option<Vec<String>>,
    pub mappings_path: Option<String>,
    pub r#type: crate::types::common_analysis::MappingCharFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MappingCharFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MappingCharFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MappingCharFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_mappings: Option<Vec<String>> = None;
                let mut seen_mappings = false;
                let mut f_mappings_path: Option<String> = None;
                let mut seen_mappings_path = false;
                let mut f_type: Option<crate::types::common_analysis::MappingCharFilterType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "mappings" => {
                            if seen_mappings {
                                return Err(serde::de::Error::duplicate_field("mappings"));
                            }
                            seen_mappings = true;
                            f_mappings = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "mappings_path" => {
                            if seen_mappings_path {
                                return Err(serde::de::Error::duplicate_field("mappings_path"));
                            }
                            seen_mappings_path = true;
                            f_mappings_path = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::MappingCharFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MappingCharFilter {
                    version: f_version,
                    mappings: f_mappings,
                    mappings_path: f_mappings_path,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MappingCharFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.mappings {
            map.serialize_entry("mappings", v)?;
        }
        if let Some(v) = &self.mappings_path {
            map.serialize_entry("mappings_path", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MappingCharFilterType {
    #[doc = "`mapping`"]
    Mapping,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl MappingCharFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            MappingCharFilterType::Mapping => "mapping",
            MappingCharFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for MappingCharFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MappingCharFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "mapping" => MappingCharFilterType::Mapping,
            _ => MappingCharFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct MultiplexerTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub filters: Vec<String>,
    pub preserve_original: Option<crate::types::common::StringifiedBoolean>,
    pub r#type: crate::types::common_analysis::MultiplexerTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MultiplexerTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MultiplexerTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MultiplexerTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_filters: Option<Vec<String>> = None;
                let mut f_preserve_original: Option<crate::types::common::StringifiedBoolean> =
                    None;
                let mut seen_preserve_original = false;
                let mut f_type: Option<crate::types::common_analysis::MultiplexerTokenFilterType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "filters" => {
                            if f_filters.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            f_filters = Some(map.next_value::<Vec<String>>()?);
                        }
                        "preserve_original" => {
                            if seen_preserve_original {
                                return Err(serde::de::Error::duplicate_field("preserve_original"));
                            }
                            seen_preserve_original = true;
                            f_preserve_original = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::MultiplexerTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MultiplexerTokenFilter {
                    version: f_version,
                    filters: f_filters.ok_or_else(|| serde::de::Error::missing_field("filters"))?,
                    preserve_original: f_preserve_original,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MultiplexerTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("filters", &self.filters)?;
        if let Some(v) = &self.preserve_original {
            map.serialize_entry("preserve_original", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MultiplexerTokenFilterType {
    #[doc = "`multiplexer`"]
    Multiplexer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl MultiplexerTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            MultiplexerTokenFilterType::Multiplexer => "multiplexer",
            MultiplexerTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for MultiplexerTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MultiplexerTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "multiplexer" => MultiplexerTokenFilterType::Multiplexer,
            _ => MultiplexerTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct NGramTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub max_gram: Option<i64>,
    pub min_gram: Option<i64>,
    pub preserve_original: Option<crate::types::common::StringifiedBoolean>,
    pub r#type: crate::types::common_analysis::NGramTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NGramTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NGramTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NGramTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_max_gram: Option<i64> = None;
                let mut seen_max_gram = false;
                let mut f_min_gram: Option<i64> = None;
                let mut seen_min_gram = false;
                let mut f_preserve_original: Option<crate::types::common::StringifiedBoolean> =
                    None;
                let mut seen_preserve_original = false;
                let mut f_type: Option<crate::types::common_analysis::NGramTokenFilterType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "max_gram" => {
                            if seen_max_gram {
                                return Err(serde::de::Error::duplicate_field("max_gram"));
                            }
                            seen_max_gram = true;
                            f_max_gram = map.next_value::<Option<i64>>()?;
                        }
                        "min_gram" => {
                            if seen_min_gram {
                                return Err(serde::de::Error::duplicate_field("min_gram"));
                            }
                            seen_min_gram = true;
                            f_min_gram = map.next_value::<Option<i64>>()?;
                        }
                        "preserve_original" => {
                            if seen_preserve_original {
                                return Err(serde::de::Error::duplicate_field("preserve_original"));
                            }
                            seen_preserve_original = true;
                            f_preserve_original = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::NGramTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NGramTokenFilter {
                    version: f_version,
                    max_gram: f_max_gram,
                    min_gram: f_min_gram,
                    preserve_original: f_preserve_original,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NGramTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.max_gram {
            map.serialize_entry("max_gram", v)?;
        }
        if let Some(v) = &self.min_gram {
            map.serialize_entry("min_gram", v)?;
        }
        if let Some(v) = &self.preserve_original {
            map.serialize_entry("preserve_original", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NGramTokenFilterType {
    #[doc = "`ngram`"]
    Ngram,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl NGramTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            NGramTokenFilterType::Ngram => "ngram",
            NGramTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for NGramTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NGramTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ngram" => NGramTokenFilterType::Ngram,
            _ => NGramTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct NGramTokenizer {
    pub version: Option<crate::types::common::VersionString>,
    pub custom_token_chars: Option<String>,
    pub max_gram: i64,
    pub min_gram: i64,
    pub token_chars: Vec<crate::types::common_analysis::TokenChar>,
    pub r#type: crate::types::common_analysis::NGramTokenizerType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NGramTokenizer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NGramTokenizer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NGramTokenizer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_custom_token_chars: Option<String> = None;
                let mut seen_custom_token_chars = false;
                let mut f_max_gram: Option<i64> = None;
                let mut f_min_gram: Option<i64> = None;
                let mut f_token_chars: Option<Vec<crate::types::common_analysis::TokenChar>> = None;
                let mut f_type: Option<crate::types::common_analysis::NGramTokenizerType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "custom_token_chars" => {
                            if seen_custom_token_chars {
                                return Err(serde::de::Error::duplicate_field(
                                    "custom_token_chars",
                                ));
                            }
                            seen_custom_token_chars = true;
                            f_custom_token_chars = map.next_value::<Option<String>>()?;
                        }
                        "max_gram" => {
                            if f_max_gram.is_some() {
                                return Err(serde::de::Error::duplicate_field("max_gram"));
                            }
                            f_max_gram = Some(map.next_value::<i64>()?);
                        }
                        "min_gram" => {
                            if f_min_gram.is_some() {
                                return Err(serde::de::Error::duplicate_field("min_gram"));
                            }
                            f_min_gram = Some(map.next_value::<i64>()?);
                        }
                        "token_chars" => {
                            if f_token_chars.is_some() {
                                return Err(serde::de::Error::duplicate_field("token_chars"));
                            }
                            f_token_chars = Some(
                                map.next_value::<Vec<crate::types::common_analysis::TokenChar>>()?,
                            );
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::NGramTokenizerType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NGramTokenizer {
                    version: f_version,
                    custom_token_chars: f_custom_token_chars,
                    max_gram: f_max_gram
                        .ok_or_else(|| serde::de::Error::missing_field("max_gram"))?,
                    min_gram: f_min_gram
                        .ok_or_else(|| serde::de::Error::missing_field("min_gram"))?,
                    token_chars: f_token_chars
                        .ok_or_else(|| serde::de::Error::missing_field("token_chars"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NGramTokenizer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.custom_token_chars {
            map.serialize_entry("custom_token_chars", v)?;
        }
        map.serialize_entry("max_gram", &self.max_gram)?;
        map.serialize_entry("min_gram", &self.min_gram)?;
        map.serialize_entry("token_chars", &self.token_chars)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NGramTokenizerType {
    #[doc = "`ngram`"]
    Ngram,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl NGramTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            NGramTokenizerType::Ngram => "ngram",
            NGramTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for NGramTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NGramTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ngram" => NGramTokenizerType::Ngram,
            _ => NGramTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct NoriAnalyzer {
    pub decompound_mode: Option<crate::types::common_analysis::NoriDecompoundMode>,
    pub stoptags: Option<Vec<String>>,
    pub r#type: crate::types::common_analysis::NoriAnalyzerType,
    pub user_dictionary: Option<String>,
    pub version: Option<crate::types::common::VersionString>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NoriAnalyzer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NoriAnalyzer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NoriAnalyzer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_decompound_mode: Option<
                    crate::types::common_analysis::NoriDecompoundMode,
                > = None;
                let mut seen_decompound_mode = false;
                let mut f_stoptags: Option<Vec<String>> = None;
                let mut seen_stoptags = false;
                let mut f_type: Option<crate::types::common_analysis::NoriAnalyzerType> = None;
                let mut f_user_dictionary: Option<String> = None;
                let mut seen_user_dictionary = false;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "decompound_mode" => {
                            if seen_decompound_mode {
                                return Err(serde::de::Error::duplicate_field("decompound_mode"));
                            }
                            seen_decompound_mode = true;
                            f_decompound_mode = map.next_value::<Option<crate::types::common_analysis::NoriDecompoundMode>>()?;
                        }
                        "stoptags" => {
                            if seen_stoptags {
                                return Err(serde::de::Error::duplicate_field("stoptags"));
                            }
                            seen_stoptags = true;
                            f_stoptags = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(
                                map.next_value::<crate::types::common_analysis::NoriAnalyzerType>(
                                )?,
                            );
                        }
                        "user_dictionary" => {
                            if seen_user_dictionary {
                                return Err(serde::de::Error::duplicate_field("user_dictionary"));
                            }
                            seen_user_dictionary = true;
                            f_user_dictionary = map.next_value::<Option<String>>()?;
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NoriAnalyzer {
                    decompound_mode: f_decompound_mode,
                    stoptags: f_stoptags,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    user_dictionary: f_user_dictionary,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NoriAnalyzer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.decompound_mode {
            map.serialize_entry("decompound_mode", v)?;
        }
        if let Some(v) = &self.stoptags {
            map.serialize_entry("stoptags", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        if let Some(v) = &self.user_dictionary {
            map.serialize_entry("user_dictionary", v)?;
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NoriAnalyzerType {
    #[doc = "`nori`"]
    Nori,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl NoriAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            NoriAnalyzerType::Nori => "nori",
            NoriAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for NoriAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NoriAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "nori" => NoriAnalyzerType::Nori,
            _ => NoriAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NoriDecompoundMode {
    #[doc = "`discard`"]
    Discard,
    #[doc = "`mixed`"]
    Mixed,
    #[doc = "`none`"]
    None,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl NoriDecompoundMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            NoriDecompoundMode::Discard => "discard",
            NoriDecompoundMode::Mixed => "mixed",
            NoriDecompoundMode::None => "none",
            NoriDecompoundMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for NoriDecompoundMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NoriDecompoundMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "discard" => NoriDecompoundMode::Discard,
            "mixed" => NoriDecompoundMode::Mixed,
            "none" => NoriDecompoundMode::None,
            _ => NoriDecompoundMode::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct NoriPartOfSpeechTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub stoptags: Option<Vec<String>>,
    pub r#type: crate::types::common_analysis::NoriPartOfSpeechTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NoriPartOfSpeechTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NoriPartOfSpeechTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NoriPartOfSpeechTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_stoptags: Option<Vec<String>> = None;
                let mut seen_stoptags = false;
                let mut f_type: Option<
                    crate::types::common_analysis::NoriPartOfSpeechTokenFilterType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "stoptags" => {
                            if seen_stoptags {
                                return Err(serde::de::Error::duplicate_field("stoptags"));
                            }
                            seen_stoptags = true;
                            f_stoptags = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::NoriPartOfSpeechTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NoriPartOfSpeechTokenFilter {
                    version: f_version,
                    stoptags: f_stoptags,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NoriPartOfSpeechTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.stoptags {
            map.serialize_entry("stoptags", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NoriPartOfSpeechTokenFilterType {
    #[doc = "`nori_part_of_speech`"]
    NoriPartOfSpeech,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl NoriPartOfSpeechTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            NoriPartOfSpeechTokenFilterType::NoriPartOfSpeech => "nori_part_of_speech",
            NoriPartOfSpeechTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for NoriPartOfSpeechTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NoriPartOfSpeechTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "nori_part_of_speech" => NoriPartOfSpeechTokenFilterType::NoriPartOfSpeech,
            _ => NoriPartOfSpeechTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct NoriTokenizer {
    pub version: Option<crate::types::common::VersionString>,
    pub decompound_mode: Option<crate::types::common_analysis::NoriDecompoundMode>,
    pub discard_punctuation: Option<bool>,
    pub r#type: crate::types::common_analysis::NoriTokenizerType,
    pub user_dictionary: Option<String>,
    pub user_dictionary_rules: Option<Vec<String>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NoriTokenizer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NoriTokenizer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NoriTokenizer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_decompound_mode: Option<
                    crate::types::common_analysis::NoriDecompoundMode,
                > = None;
                let mut seen_decompound_mode = false;
                let mut f_discard_punctuation: Option<bool> = None;
                let mut seen_discard_punctuation = false;
                let mut f_type: Option<crate::types::common_analysis::NoriTokenizerType> = None;
                let mut f_user_dictionary: Option<String> = None;
                let mut seen_user_dictionary = false;
                let mut f_user_dictionary_rules: Option<Vec<String>> = None;
                let mut seen_user_dictionary_rules = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "decompound_mode" => {
                            if seen_decompound_mode {
                                return Err(serde::de::Error::duplicate_field("decompound_mode"));
                            }
                            seen_decompound_mode = true;
                            f_decompound_mode = map.next_value::<Option<crate::types::common_analysis::NoriDecompoundMode>>()?;
                        }
                        "discard_punctuation" => {
                            if seen_discard_punctuation {
                                return Err(serde::de::Error::duplicate_field(
                                    "discard_punctuation",
                                ));
                            }
                            seen_discard_punctuation = true;
                            f_discard_punctuation = map.next_value::<Option<bool>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(
                                map.next_value::<crate::types::common_analysis::NoriTokenizerType>(
                                )?,
                            );
                        }
                        "user_dictionary" => {
                            if seen_user_dictionary {
                                return Err(serde::de::Error::duplicate_field("user_dictionary"));
                            }
                            seen_user_dictionary = true;
                            f_user_dictionary = map.next_value::<Option<String>>()?;
                        }
                        "user_dictionary_rules" => {
                            if seen_user_dictionary_rules {
                                return Err(serde::de::Error::duplicate_field(
                                    "user_dictionary_rules",
                                ));
                            }
                            seen_user_dictionary_rules = true;
                            f_user_dictionary_rules = map.next_value::<Option<Vec<String>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NoriTokenizer {
                    version: f_version,
                    decompound_mode: f_decompound_mode,
                    discard_punctuation: f_discard_punctuation,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    user_dictionary: f_user_dictionary,
                    user_dictionary_rules: f_user_dictionary_rules,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NoriTokenizer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.decompound_mode {
            map.serialize_entry("decompound_mode", v)?;
        }
        if let Some(v) = &self.discard_punctuation {
            map.serialize_entry("discard_punctuation", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        if let Some(v) = &self.user_dictionary {
            map.serialize_entry("user_dictionary", v)?;
        }
        if let Some(v) = &self.user_dictionary_rules {
            map.serialize_entry("user_dictionary_rules", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NoriTokenizerType {
    #[doc = "`nori_tokenizer`"]
    NoriTokenizer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl NoriTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            NoriTokenizerType::NoriTokenizer => "nori_tokenizer",
            NoriTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for NoriTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NoriTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "nori_tokenizer" => NoriTokenizerType::NoriTokenizer,
            _ => NoriTokenizerType::Unknown(s),
        })
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Normalizer {
    raw: Box<serde_json::value::RawValue>,
}
impl Normalizer {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `lowercase_normalizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_lowercase_normalizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::LowercaseNormalizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "lowercase_normalizer")
    }
    #[doc = "Parses the value as the `custom_normalizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_custom_normalizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::CustomNormalizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "custom_normalizer")
    }
}

#[derive(Debug, Clone)]
pub struct PathHierarchyTokenizer {
    pub version: Option<crate::types::common::VersionString>,
    pub buffer_size: crate::types::common::StringifiedInteger,
    pub delimiter: String,
    pub replacement: Option<String>,
    pub reverse: crate::types::common::StringifiedBoolean,
    pub skip: crate::types::common::StringifiedInteger,
    pub r#type: crate::types::common_analysis::PathHierarchyTokenizerType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PathHierarchyTokenizer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PathHierarchyTokenizer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PathHierarchyTokenizer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_buffer_size: Option<crate::types::common::StringifiedInteger> = None;
                let mut f_delimiter: Option<String> = None;
                let mut f_replacement: Option<String> = None;
                let mut seen_replacement = false;
                let mut f_reverse: Option<crate::types::common::StringifiedBoolean> = None;
                let mut f_skip: Option<crate::types::common::StringifiedInteger> = None;
                let mut f_type: Option<crate::types::common_analysis::PathHierarchyTokenizerType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "buffer_size" => {
                            if f_buffer_size.is_some() {
                                return Err(serde::de::Error::duplicate_field("buffer_size"));
                            }
                            f_buffer_size =
                                Some(map.next_value::<crate::types::common::StringifiedInteger>()?);
                        }
                        "delimiter" => {
                            if f_delimiter.is_some() {
                                return Err(serde::de::Error::duplicate_field("delimiter"));
                            }
                            f_delimiter = Some(map.next_value::<String>()?);
                        }
                        "replacement" => {
                            if seen_replacement {
                                return Err(serde::de::Error::duplicate_field("replacement"));
                            }
                            seen_replacement = true;
                            f_replacement = map.next_value::<Option<String>>()?;
                        }
                        "reverse" => {
                            if f_reverse.is_some() {
                                return Err(serde::de::Error::duplicate_field("reverse"));
                            }
                            f_reverse =
                                Some(map.next_value::<crate::types::common::StringifiedBoolean>()?);
                        }
                        "skip" => {
                            if f_skip.is_some() {
                                return Err(serde::de::Error::duplicate_field("skip"));
                            }
                            f_skip =
                                Some(map.next_value::<crate::types::common::StringifiedInteger>()?);
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::PathHierarchyTokenizerType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PathHierarchyTokenizer {
                    version: f_version,
                    buffer_size: f_buffer_size
                        .ok_or_else(|| serde::de::Error::missing_field("buffer_size"))?,
                    delimiter: f_delimiter
                        .ok_or_else(|| serde::de::Error::missing_field("delimiter"))?,
                    replacement: f_replacement,
                    reverse: f_reverse.ok_or_else(|| serde::de::Error::missing_field("reverse"))?,
                    skip: f_skip.ok_or_else(|| serde::de::Error::missing_field("skip"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PathHierarchyTokenizer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("buffer_size", &self.buffer_size)?;
        map.serialize_entry("delimiter", &self.delimiter)?;
        if let Some(v) = &self.replacement {
            map.serialize_entry("replacement", v)?;
        }
        map.serialize_entry("reverse", &self.reverse)?;
        map.serialize_entry("skip", &self.skip)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathHierarchyTokenizerType {
    #[doc = "`path_hierarchy`"]
    PathHierarchy,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PathHierarchyTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PathHierarchyTokenizerType::PathHierarchy => "path_hierarchy",
            PathHierarchyTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PathHierarchyTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PathHierarchyTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "path_hierarchy" => PathHierarchyTokenizerType::PathHierarchy,
            _ => PathHierarchyTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct PatternAnalyzer {
    pub flags: Option<String>,
    pub lowercase: Option<bool>,
    pub pattern: String,
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    pub r#type: crate::types::common_analysis::PatternAnalyzerType,
    pub version: Option<crate::types::common::VersionString>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PatternAnalyzer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PatternAnalyzer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PatternAnalyzer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_flags: Option<String> = None;
                let mut seen_flags = false;
                let mut f_lowercase: Option<bool> = None;
                let mut seen_lowercase = false;
                let mut f_pattern: Option<String> = None;
                let mut f_stopwords: Option<crate::types::common_analysis::StopWords> = None;
                let mut seen_stopwords = false;
                let mut f_type: Option<crate::types::common_analysis::PatternAnalyzerType> = None;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "flags" => {
                            if seen_flags {
                                return Err(serde::de::Error::duplicate_field("flags"));
                            }
                            seen_flags = true;
                            f_flags = map.next_value::<Option<String>>()?;
                        }
                        "lowercase" => {
                            if seen_lowercase {
                                return Err(serde::de::Error::duplicate_field("lowercase"));
                            }
                            seen_lowercase = true;
                            f_lowercase = map.next_value::<Option<bool>>()?;
                        }
                        "pattern" => {
                            if f_pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("pattern"));
                            }
                            f_pattern = Some(map.next_value::<String>()?);
                        }
                        "stopwords" => {
                            if seen_stopwords {
                                return Err(serde::de::Error::duplicate_field("stopwords"));
                            }
                            seen_stopwords = true;
                            f_stopwords = map
                                .next_value::<Option<crate::types::common_analysis::StopWords>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::PatternAnalyzerType>()?);
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PatternAnalyzer {
                    flags: f_flags,
                    lowercase: f_lowercase,
                    pattern: f_pattern.ok_or_else(|| serde::de::Error::missing_field("pattern"))?,
                    stopwords: f_stopwords,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PatternAnalyzer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.flags {
            map.serialize_entry("flags", v)?;
        }
        if let Some(v) = &self.lowercase {
            map.serialize_entry("lowercase", v)?;
        }
        map.serialize_entry("pattern", &self.pattern)?;
        if let Some(v) = &self.stopwords {
            map.serialize_entry("stopwords", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternAnalyzerType {
    #[doc = "`pattern`"]
    Pattern,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PatternAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PatternAnalyzerType::Pattern => "pattern",
            PatternAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PatternAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PatternAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "pattern" => PatternAnalyzerType::Pattern,
            _ => PatternAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct PatternCaptureTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub patterns: Vec<String>,
    pub preserve_original: Option<crate::types::common::StringifiedBoolean>,
    pub r#type: crate::types::common_analysis::PatternCaptureTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PatternCaptureTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PatternCaptureTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PatternCaptureTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_patterns: Option<Vec<String>> = None;
                let mut f_preserve_original: Option<crate::types::common::StringifiedBoolean> =
                    None;
                let mut seen_preserve_original = false;
                let mut f_type: Option<
                    crate::types::common_analysis::PatternCaptureTokenFilterType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "patterns" => {
                            if f_patterns.is_some() {
                                return Err(serde::de::Error::duplicate_field("patterns"));
                            }
                            f_patterns = Some(map.next_value::<Vec<String>>()?);
                        }
                        "preserve_original" => {
                            if seen_preserve_original {
                                return Err(serde::de::Error::duplicate_field("preserve_original"));
                            }
                            seen_preserve_original = true;
                            f_preserve_original = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::PatternCaptureTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PatternCaptureTokenFilter {
                    version: f_version,
                    patterns: f_patterns
                        .ok_or_else(|| serde::de::Error::missing_field("patterns"))?,
                    preserve_original: f_preserve_original,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PatternCaptureTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("patterns", &self.patterns)?;
        if let Some(v) = &self.preserve_original {
            map.serialize_entry("preserve_original", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternCaptureTokenFilterType {
    #[doc = "`pattern_capture`"]
    PatternCapture,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PatternCaptureTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PatternCaptureTokenFilterType::PatternCapture => "pattern_capture",
            PatternCaptureTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PatternCaptureTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PatternCaptureTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "pattern_capture" => PatternCaptureTokenFilterType::PatternCapture,
            _ => PatternCaptureTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct PatternReplaceCharFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub flags: Option<String>,
    pub pattern: String,
    pub replacement: Option<String>,
    pub r#type: crate::types::common_analysis::PatternReplaceCharFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PatternReplaceCharFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PatternReplaceCharFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PatternReplaceCharFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_flags: Option<String> = None;
                let mut seen_flags = false;
                let mut f_pattern: Option<String> = None;
                let mut f_replacement: Option<String> = None;
                let mut seen_replacement = false;
                let mut f_type: Option<
                    crate::types::common_analysis::PatternReplaceCharFilterType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "flags" => {
                            if seen_flags {
                                return Err(serde::de::Error::duplicate_field("flags"));
                            }
                            seen_flags = true;
                            f_flags = map.next_value::<Option<String>>()?;
                        }
                        "pattern" => {
                            if f_pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("pattern"));
                            }
                            f_pattern = Some(map.next_value::<String>()?);
                        }
                        "replacement" => {
                            if seen_replacement {
                                return Err(serde::de::Error::duplicate_field("replacement"));
                            }
                            seen_replacement = true;
                            f_replacement = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::PatternReplaceCharFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PatternReplaceCharFilter {
                    version: f_version,
                    flags: f_flags,
                    pattern: f_pattern.ok_or_else(|| serde::de::Error::missing_field("pattern"))?,
                    replacement: f_replacement,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PatternReplaceCharFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.flags {
            map.serialize_entry("flags", v)?;
        }
        map.serialize_entry("pattern", &self.pattern)?;
        if let Some(v) = &self.replacement {
            map.serialize_entry("replacement", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternReplaceCharFilterType {
    #[doc = "`pattern_replace`"]
    PatternReplace,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PatternReplaceCharFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PatternReplaceCharFilterType::PatternReplace => "pattern_replace",
            PatternReplaceCharFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PatternReplaceCharFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PatternReplaceCharFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "pattern_replace" => PatternReplaceCharFilterType::PatternReplace,
            _ => PatternReplaceCharFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct PatternReplaceTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub all: Option<bool>,
    pub flags: Option<String>,
    pub pattern: String,
    pub replacement: Option<String>,
    pub r#type: crate::types::common_analysis::PatternReplaceTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PatternReplaceTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PatternReplaceTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PatternReplaceTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_all: Option<bool> = None;
                let mut seen_all = false;
                let mut f_flags: Option<String> = None;
                let mut seen_flags = false;
                let mut f_pattern: Option<String> = None;
                let mut f_replacement: Option<String> = None;
                let mut seen_replacement = false;
                let mut f_type: Option<
                    crate::types::common_analysis::PatternReplaceTokenFilterType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "all" => {
                            if seen_all {
                                return Err(serde::de::Error::duplicate_field("all"));
                            }
                            seen_all = true;
                            f_all = map.next_value::<Option<bool>>()?;
                        }
                        "flags" => {
                            if seen_flags {
                                return Err(serde::de::Error::duplicate_field("flags"));
                            }
                            seen_flags = true;
                            f_flags = map.next_value::<Option<String>>()?;
                        }
                        "pattern" => {
                            if f_pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("pattern"));
                            }
                            f_pattern = Some(map.next_value::<String>()?);
                        }
                        "replacement" => {
                            if seen_replacement {
                                return Err(serde::de::Error::duplicate_field("replacement"));
                            }
                            seen_replacement = true;
                            f_replacement = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::PatternReplaceTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PatternReplaceTokenFilter {
                    version: f_version,
                    all: f_all,
                    flags: f_flags,
                    pattern: f_pattern.ok_or_else(|| serde::de::Error::missing_field("pattern"))?,
                    replacement: f_replacement,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PatternReplaceTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.all {
            map.serialize_entry("all", v)?;
        }
        if let Some(v) = &self.flags {
            map.serialize_entry("flags", v)?;
        }
        map.serialize_entry("pattern", &self.pattern)?;
        if let Some(v) = &self.replacement {
            map.serialize_entry("replacement", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternReplaceTokenFilterType {
    #[doc = "`pattern_replace`"]
    PatternReplace,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PatternReplaceTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PatternReplaceTokenFilterType::PatternReplace => "pattern_replace",
            PatternReplaceTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PatternReplaceTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PatternReplaceTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "pattern_replace" => PatternReplaceTokenFilterType::PatternReplace,
            _ => PatternReplaceTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct PatternTokenizer {
    pub version: Option<crate::types::common::VersionString>,
    pub flags: Option<String>,
    pub group: Option<i64>,
    pub pattern: Option<String>,
    pub r#type: crate::types::common_analysis::PatternTokenizerType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PatternTokenizer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PatternTokenizer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PatternTokenizer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_flags: Option<String> = None;
                let mut seen_flags = false;
                let mut f_group: Option<i64> = None;
                let mut seen_group = false;
                let mut f_pattern: Option<String> = None;
                let mut seen_pattern = false;
                let mut f_type: Option<crate::types::common_analysis::PatternTokenizerType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "flags" => {
                            if seen_flags {
                                return Err(serde::de::Error::duplicate_field("flags"));
                            }
                            seen_flags = true;
                            f_flags = map.next_value::<Option<String>>()?;
                        }
                        "group" => {
                            if seen_group {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            seen_group = true;
                            f_group = map.next_value::<Option<i64>>()?;
                        }
                        "pattern" => {
                            if seen_pattern {
                                return Err(serde::de::Error::duplicate_field("pattern"));
                            }
                            seen_pattern = true;
                            f_pattern = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::PatternTokenizerType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PatternTokenizer {
                    version: f_version,
                    flags: f_flags,
                    group: f_group,
                    pattern: f_pattern,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PatternTokenizer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.flags {
            map.serialize_entry("flags", v)?;
        }
        if let Some(v) = &self.group {
            map.serialize_entry("group", v)?;
        }
        if let Some(v) = &self.pattern {
            map.serialize_entry("pattern", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternTokenizerType {
    #[doc = "`pattern`"]
    Pattern,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PatternTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PatternTokenizerType::Pattern => "pattern",
            PatternTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PatternTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PatternTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "pattern" => PatternTokenizerType::Pattern,
            _ => PatternTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct PersianStemTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::PersianStemTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PersianStemTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PersianStemTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PersianStemTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_type: Option<crate::types::common_analysis::PersianStemTokenFilterType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::PersianStemTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PersianStemTokenFilter {
                    version: f_version,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PersianStemTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PersianStemTokenFilterType {
    #[doc = "`persian_stem`"]
    PersianStem,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PersianStemTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PersianStemTokenFilterType::PersianStem => "persian_stem",
            PersianStemTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PersianStemTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PersianStemTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "persian_stem" => PersianStemTokenFilterType::PersianStem,
            _ => PersianStemTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct PhoneAnalyzer {
    #[doc = "Optional ISO 3166 country code, defaults to \"ZZ\" (unknown region)."]
    pub phone_region: Option<String>,
    pub r#type: crate::types::common_analysis::PhoneAnalyzerType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PhoneAnalyzer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PhoneAnalyzer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PhoneAnalyzer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_phone_region: Option<String> = None;
                let mut seen_phone_region = false;
                let mut f_type: Option<crate::types::common_analysis::PhoneAnalyzerType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "phone-region" => {
                            if seen_phone_region {
                                return Err(serde::de::Error::duplicate_field("phone-region"));
                            }
                            seen_phone_region = true;
                            f_phone_region = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(
                                map.next_value::<crate::types::common_analysis::PhoneAnalyzerType>(
                                )?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PhoneAnalyzer {
                    phone_region: f_phone_region,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PhoneAnalyzer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.phone_region {
            map.serialize_entry("phone-region", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhoneAnalyzerType {
    #[doc = "`phone`"]
    Phone,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PhoneAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PhoneAnalyzerType::Phone => "phone",
            PhoneAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PhoneAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PhoneAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "phone" => PhoneAnalyzerType::Phone,
            _ => PhoneAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct PhoneSearchAnalyzer {
    #[doc = "Optional ISO 3166 country code, defaults to \"ZZ\" (unknown region)."]
    pub phone_region: Option<String>,
    pub r#type: crate::types::common_analysis::PhoneSearchAnalyzerType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PhoneSearchAnalyzer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PhoneSearchAnalyzer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PhoneSearchAnalyzer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_phone_region: Option<String> = None;
                let mut seen_phone_region = false;
                let mut f_type: Option<crate::types::common_analysis::PhoneSearchAnalyzerType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "phone-region" => {
                            if seen_phone_region {
                                return Err(serde::de::Error::duplicate_field("phone-region"));
                            }
                            seen_phone_region = true;
                            f_phone_region = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::PhoneSearchAnalyzerType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PhoneSearchAnalyzer {
                    phone_region: f_phone_region,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PhoneSearchAnalyzer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.phone_region {
            map.serialize_entry("phone-region", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhoneSearchAnalyzerType {
    #[doc = "`phone-search`"]
    PhoneSearch,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PhoneSearchAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PhoneSearchAnalyzerType::PhoneSearch => "phone-search",
            PhoneSearchAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PhoneSearchAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PhoneSearchAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "phone-search" => PhoneSearchAnalyzerType::PhoneSearch,
            _ => PhoneSearchAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhoneticEncoder {
    #[doc = "`beider_morse`"]
    BeiderMorse,
    #[doc = "`caverphone1`"]
    Caverphone1,
    #[doc = "`caverphone2`"]
    Caverphone2,
    #[doc = "`cologne`"]
    Cologne,
    #[doc = "`daitch_mokotoff`"]
    DaitchMokotoff,
    #[doc = "`double_metaphone`"]
    DoubleMetaphone,
    #[doc = "`haasephonetik`"]
    Haasephonetik,
    #[doc = "`koelnerphonetik`"]
    Koelnerphonetik,
    #[doc = "`metaphone`"]
    Metaphone,
    #[doc = "`nysiis`"]
    Nysiis,
    #[doc = "`refined_soundex`"]
    RefinedSoundex,
    #[doc = "`soundex`"]
    Soundex,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PhoneticEncoder {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PhoneticEncoder::BeiderMorse => "beider_morse",
            PhoneticEncoder::Caverphone1 => "caverphone1",
            PhoneticEncoder::Caverphone2 => "caverphone2",
            PhoneticEncoder::Cologne => "cologne",
            PhoneticEncoder::DaitchMokotoff => "daitch_mokotoff",
            PhoneticEncoder::DoubleMetaphone => "double_metaphone",
            PhoneticEncoder::Haasephonetik => "haasephonetik",
            PhoneticEncoder::Koelnerphonetik => "koelnerphonetik",
            PhoneticEncoder::Metaphone => "metaphone",
            PhoneticEncoder::Nysiis => "nysiis",
            PhoneticEncoder::RefinedSoundex => "refined_soundex",
            PhoneticEncoder::Soundex => "soundex",
            PhoneticEncoder::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PhoneticEncoder {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PhoneticEncoder {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "beider_morse" => PhoneticEncoder::BeiderMorse,
            "caverphone1" => PhoneticEncoder::Caverphone1,
            "caverphone2" => PhoneticEncoder::Caverphone2,
            "cologne" => PhoneticEncoder::Cologne,
            "daitch_mokotoff" => PhoneticEncoder::DaitchMokotoff,
            "double_metaphone" => PhoneticEncoder::DoubleMetaphone,
            "haasephonetik" => PhoneticEncoder::Haasephonetik,
            "koelnerphonetik" => PhoneticEncoder::Koelnerphonetik,
            "metaphone" => PhoneticEncoder::Metaphone,
            "nysiis" => PhoneticEncoder::Nysiis,
            "refined_soundex" => PhoneticEncoder::RefinedSoundex,
            "soundex" => PhoneticEncoder::Soundex,
            _ => PhoneticEncoder::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhoneticLanguage {
    #[doc = "`any`"]
    Any,
    #[doc = "`common`"]
    Common,
    #[doc = "`cyrillic`"]
    Cyrillic,
    #[doc = "`english`"]
    English,
    #[doc = "`french`"]
    French,
    #[doc = "`german`"]
    German,
    #[doc = "`hebrew`"]
    Hebrew,
    #[doc = "`hungarian`"]
    Hungarian,
    #[doc = "`polish`"]
    Polish,
    #[doc = "`romanian`"]
    Romanian,
    #[doc = "`russian`"]
    Russian,
    #[doc = "`spanish`"]
    Spanish,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PhoneticLanguage {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PhoneticLanguage::Any => "any",
            PhoneticLanguage::Common => "common",
            PhoneticLanguage::Cyrillic => "cyrillic",
            PhoneticLanguage::English => "english",
            PhoneticLanguage::French => "french",
            PhoneticLanguage::German => "german",
            PhoneticLanguage::Hebrew => "hebrew",
            PhoneticLanguage::Hungarian => "hungarian",
            PhoneticLanguage::Polish => "polish",
            PhoneticLanguage::Romanian => "romanian",
            PhoneticLanguage::Russian => "russian",
            PhoneticLanguage::Spanish => "spanish",
            PhoneticLanguage::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PhoneticLanguage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PhoneticLanguage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "any" => PhoneticLanguage::Any,
            "common" => PhoneticLanguage::Common,
            "cyrillic" => PhoneticLanguage::Cyrillic,
            "english" => PhoneticLanguage::English,
            "french" => PhoneticLanguage::French,
            "german" => PhoneticLanguage::German,
            "hebrew" => PhoneticLanguage::Hebrew,
            "hungarian" => PhoneticLanguage::Hungarian,
            "polish" => PhoneticLanguage::Polish,
            "romanian" => PhoneticLanguage::Romanian,
            "russian" => PhoneticLanguage::Russian,
            "spanish" => PhoneticLanguage::Spanish,
            _ => PhoneticLanguage::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhoneticNameType {
    #[doc = "`ashkenazi`"]
    Ashkenazi,
    #[doc = "`generic`"]
    Generic,
    #[doc = "`sephardic`"]
    Sephardic,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PhoneticNameType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PhoneticNameType::Ashkenazi => "ashkenazi",
            PhoneticNameType::Generic => "generic",
            PhoneticNameType::Sephardic => "sephardic",
            PhoneticNameType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PhoneticNameType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PhoneticNameType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ashkenazi" => PhoneticNameType::Ashkenazi,
            "generic" => PhoneticNameType::Generic,
            "sephardic" => PhoneticNameType::Sephardic,
            _ => PhoneticNameType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhoneticRuleType {
    #[doc = "`approx`"]
    Approx,
    #[doc = "`exact`"]
    Exact,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PhoneticRuleType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PhoneticRuleType::Approx => "approx",
            PhoneticRuleType::Exact => "exact",
            PhoneticRuleType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PhoneticRuleType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PhoneticRuleType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "approx" => PhoneticRuleType::Approx,
            "exact" => PhoneticRuleType::Exact,
            _ => PhoneticRuleType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct PhoneticTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub encoder: crate::types::common_analysis::PhoneticEncoder,
    pub languageset: Vec<crate::types::common_analysis::PhoneticLanguage>,
    pub max_code_len: Option<i64>,
    pub name_type: crate::types::common_analysis::PhoneticNameType,
    pub replace: Option<bool>,
    pub rule_type: crate::types::common_analysis::PhoneticRuleType,
    pub r#type: crate::types::common_analysis::PhoneticTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PhoneticTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PhoneticTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PhoneticTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_encoder: Option<crate::types::common_analysis::PhoneticEncoder> = None;
                let mut f_languageset: Option<
                    Vec<crate::types::common_analysis::PhoneticLanguage>,
                > = None;
                let mut f_max_code_len: Option<i64> = None;
                let mut seen_max_code_len = false;
                let mut f_name_type: Option<crate::types::common_analysis::PhoneticNameType> = None;
                let mut f_replace: Option<bool> = None;
                let mut seen_replace = false;
                let mut f_rule_type: Option<crate::types::common_analysis::PhoneticRuleType> = None;
                let mut f_type: Option<crate::types::common_analysis::PhoneticTokenFilterType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "encoder" => {
                            if f_encoder.is_some() {
                                return Err(serde::de::Error::duplicate_field("encoder"));
                            }
                            f_encoder = Some(
                                map.next_value::<crate::types::common_analysis::PhoneticEncoder>()?,
                            );
                        }
                        "languageset" => {
                            if f_languageset.is_some() {
                                return Err(serde::de::Error::duplicate_field("languageset"));
                            }
                            f_languageset = Some(map.next_value::<Vec<crate::types::common_analysis::PhoneticLanguage>>()?);
                        }
                        "max_code_len" => {
                            if seen_max_code_len {
                                return Err(serde::de::Error::duplicate_field("max_code_len"));
                            }
                            seen_max_code_len = true;
                            f_max_code_len = map.next_value::<Option<i64>>()?;
                        }
                        "name_type" => {
                            if f_name_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("name_type"));
                            }
                            f_name_type = Some(
                                map.next_value::<crate::types::common_analysis::PhoneticNameType>(
                                )?,
                            );
                        }
                        "replace" => {
                            if seen_replace {
                                return Err(serde::de::Error::duplicate_field("replace"));
                            }
                            seen_replace = true;
                            f_replace = map.next_value::<Option<bool>>()?;
                        }
                        "rule_type" => {
                            if f_rule_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("rule_type"));
                            }
                            f_rule_type = Some(
                                map.next_value::<crate::types::common_analysis::PhoneticRuleType>(
                                )?,
                            );
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::PhoneticTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PhoneticTokenFilter {
                    version: f_version,
                    encoder: f_encoder.ok_or_else(|| serde::de::Error::missing_field("encoder"))?,
                    languageset: f_languageset
                        .ok_or_else(|| serde::de::Error::missing_field("languageset"))?,
                    max_code_len: f_max_code_len,
                    name_type: f_name_type
                        .ok_or_else(|| serde::de::Error::missing_field("name_type"))?,
                    replace: f_replace,
                    rule_type: f_rule_type
                        .ok_or_else(|| serde::de::Error::missing_field("rule_type"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PhoneticTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("encoder", &self.encoder)?;
        map.serialize_entry("languageset", &self.languageset)?;
        if let Some(v) = &self.max_code_len {
            map.serialize_entry("max_code_len", v)?;
        }
        map.serialize_entry("name_type", &self.name_type)?;
        if let Some(v) = &self.replace {
            map.serialize_entry("replace", v)?;
        }
        map.serialize_entry("rule_type", &self.rule_type)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhoneticTokenFilterType {
    #[doc = "`phonetic`"]
    Phonetic,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PhoneticTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PhoneticTokenFilterType::Phonetic => "phonetic",
            PhoneticTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PhoneticTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PhoneticTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "phonetic" => PhoneticTokenFilterType::Phonetic,
            _ => PhoneticTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct PorterStemTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::PorterStemTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PorterStemTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PorterStemTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PorterStemTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_type: Option<crate::types::common_analysis::PorterStemTokenFilterType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::PorterStemTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PorterStemTokenFilter {
                    version: f_version,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PorterStemTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PorterStemTokenFilterType {
    #[doc = "`porter_stem`"]
    PorterStem,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PorterStemTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PorterStemTokenFilterType::PorterStem => "porter_stem",
            PorterStemTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PorterStemTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PorterStemTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "porter_stem" => PorterStemTokenFilterType::PorterStem,
            _ => PorterStemTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct PredicateTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub script: crate::types::common::Script,
    pub r#type: crate::types::common_analysis::PredicateTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PredicateTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PredicateTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PredicateTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_script: Option<crate::types::common::Script> = None;
                let mut f_type: Option<crate::types::common_analysis::PredicateTokenFilterType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "script" => {
                            if f_script.is_some() {
                                return Err(serde::de::Error::duplicate_field("script"));
                            }
                            f_script = Some(map.next_value::<crate::types::common::Script>()?);
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::PredicateTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PredicateTokenFilter {
                    version: f_version,
                    script: f_script.ok_or_else(|| serde::de::Error::missing_field("script"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PredicateTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("script", &self.script)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PredicateTokenFilterType {
    #[doc = "`predicate_token_filter`"]
    PredicateTokenFilter,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PredicateTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PredicateTokenFilterType::PredicateTokenFilter => "predicate_token_filter",
            PredicateTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PredicateTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PredicateTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "predicate_token_filter" => PredicateTokenFilterType::PredicateTokenFilter,
            _ => PredicateTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct RemoveDuplicatesTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::RemoveDuplicatesTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RemoveDuplicatesTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RemoveDuplicatesTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RemoveDuplicatesTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_type: Option<
                    crate::types::common_analysis::RemoveDuplicatesTokenFilterType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::RemoveDuplicatesTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RemoveDuplicatesTokenFilter {
                    version: f_version,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RemoveDuplicatesTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RemoveDuplicatesTokenFilterType {
    #[doc = "`remove_duplicates`"]
    RemoveDuplicates,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl RemoveDuplicatesTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            RemoveDuplicatesTokenFilterType::RemoveDuplicates => "remove_duplicates",
            RemoveDuplicatesTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for RemoveDuplicatesTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RemoveDuplicatesTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "remove_duplicates" => RemoveDuplicatesTokenFilterType::RemoveDuplicates,
            _ => RemoveDuplicatesTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ReverseTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::ReverseTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ReverseTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ReverseTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ReverseTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_type: Option<crate::types::common_analysis::ReverseTokenFilterType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::ReverseTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ReverseTokenFilter {
                    version: f_version,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ReverseTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReverseTokenFilterType {
    #[doc = "`reverse`"]
    Reverse,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ReverseTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ReverseTokenFilterType::Reverse => "reverse",
            ReverseTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ReverseTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReverseTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "reverse" => ReverseTokenFilterType::Reverse,
            _ => ReverseTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ShingleTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub filler_token: Option<String>,
    pub max_shingle_size: Option<crate::types::common::StringifiedInteger>,
    pub min_shingle_size: Option<crate::types::common::StringifiedInteger>,
    pub output_unigrams: Option<bool>,
    pub output_unigrams_if_no_shingles: Option<bool>,
    pub token_separator: Option<String>,
    pub r#type: crate::types::common_analysis::ShingleTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShingleTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShingleTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShingleTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_filler_token: Option<String> = None;
                let mut seen_filler_token = false;
                let mut f_max_shingle_size: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_max_shingle_size = false;
                let mut f_min_shingle_size: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_min_shingle_size = false;
                let mut f_output_unigrams: Option<bool> = None;
                let mut seen_output_unigrams = false;
                let mut f_output_unigrams_if_no_shingles: Option<bool> = None;
                let mut seen_output_unigrams_if_no_shingles = false;
                let mut f_token_separator: Option<String> = None;
                let mut seen_token_separator = false;
                let mut f_type: Option<crate::types::common_analysis::ShingleTokenFilterType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "filler_token" => {
                            if seen_filler_token {
                                return Err(serde::de::Error::duplicate_field("filler_token"));
                            }
                            seen_filler_token = true;
                            f_filler_token = map.next_value::<Option<String>>()?;
                        }
                        "max_shingle_size" => {
                            if seen_max_shingle_size {
                                return Err(serde::de::Error::duplicate_field("max_shingle_size"));
                            }
                            seen_max_shingle_size = true;
                            f_max_shingle_size = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "min_shingle_size" => {
                            if seen_min_shingle_size {
                                return Err(serde::de::Error::duplicate_field("min_shingle_size"));
                            }
                            seen_min_shingle_size = true;
                            f_min_shingle_size = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "output_unigrams" => {
                            if seen_output_unigrams {
                                return Err(serde::de::Error::duplicate_field("output_unigrams"));
                            }
                            seen_output_unigrams = true;
                            f_output_unigrams = map.next_value::<Option<bool>>()?;
                        }
                        "output_unigrams_if_no_shingles" => {
                            if seen_output_unigrams_if_no_shingles {
                                return Err(serde::de::Error::duplicate_field(
                                    "output_unigrams_if_no_shingles",
                                ));
                            }
                            seen_output_unigrams_if_no_shingles = true;
                            f_output_unigrams_if_no_shingles = map.next_value::<Option<bool>>()?;
                        }
                        "token_separator" => {
                            if seen_token_separator {
                                return Err(serde::de::Error::duplicate_field("token_separator"));
                            }
                            seen_token_separator = true;
                            f_token_separator = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::ShingleTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShingleTokenFilter {
                    version: f_version,
                    filler_token: f_filler_token,
                    max_shingle_size: f_max_shingle_size,
                    min_shingle_size: f_min_shingle_size,
                    output_unigrams: f_output_unigrams,
                    output_unigrams_if_no_shingles: f_output_unigrams_if_no_shingles,
                    token_separator: f_token_separator,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShingleTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.filler_token {
            map.serialize_entry("filler_token", v)?;
        }
        if let Some(v) = &self.max_shingle_size {
            map.serialize_entry("max_shingle_size", v)?;
        }
        if let Some(v) = &self.min_shingle_size {
            map.serialize_entry("min_shingle_size", v)?;
        }
        if let Some(v) = &self.output_unigrams {
            map.serialize_entry("output_unigrams", v)?;
        }
        if let Some(v) = &self.output_unigrams_if_no_shingles {
            map.serialize_entry("output_unigrams_if_no_shingles", v)?;
        }
        if let Some(v) = &self.token_separator {
            map.serialize_entry("token_separator", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShingleTokenFilterType {
    #[doc = "`shingle`"]
    Shingle,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ShingleTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ShingleTokenFilterType::Shingle => "shingle",
            ShingleTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ShingleTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShingleTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "shingle" => ShingleTokenFilterType::Shingle,
            _ => ShingleTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SimpleAnalyzer {
    pub r#type: crate::types::common_analysis::SimpleAnalyzerType,
    pub version: Option<crate::types::common::VersionString>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SimpleAnalyzer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SimpleAnalyzer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SimpleAnalyzer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_type: Option<crate::types::common_analysis::SimpleAnalyzerType> = None;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::SimpleAnalyzerType>()?);
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SimpleAnalyzer {
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SimpleAnalyzer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("type", &self.r#type)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimpleAnalyzerType {
    #[doc = "`simple`"]
    Simple,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SimpleAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SimpleAnalyzerType::Simple => "simple",
            SimpleAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SimpleAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SimpleAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "simple" => SimpleAnalyzerType::Simple,
            _ => SimpleAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SimplePatternSplitTokenizer {
    pub version: Option<crate::types::common::VersionString>,
    pub pattern: Option<String>,
    pub r#type: crate::types::common_analysis::SimplePatternSplitTokenizerType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SimplePatternSplitTokenizer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SimplePatternSplitTokenizer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SimplePatternSplitTokenizer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_pattern: Option<String> = None;
                let mut seen_pattern = false;
                let mut f_type: Option<
                    crate::types::common_analysis::SimplePatternSplitTokenizerType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "pattern" => {
                            if seen_pattern {
                                return Err(serde::de::Error::duplicate_field("pattern"));
                            }
                            seen_pattern = true;
                            f_pattern = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::SimplePatternSplitTokenizerType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SimplePatternSplitTokenizer {
                    version: f_version,
                    pattern: f_pattern,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SimplePatternSplitTokenizer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.pattern {
            map.serialize_entry("pattern", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimplePatternSplitTokenizerType {
    #[doc = "`simple_pattern_split`"]
    SimplePatternSplit,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SimplePatternSplitTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SimplePatternSplitTokenizerType::SimplePatternSplit => "simple_pattern_split",
            SimplePatternSplitTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SimplePatternSplitTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SimplePatternSplitTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "simple_pattern_split" => SimplePatternSplitTokenizerType::SimplePatternSplit,
            _ => SimplePatternSplitTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SimplePatternTokenizer {
    pub version: Option<crate::types::common::VersionString>,
    pub pattern: Option<String>,
    pub r#type: crate::types::common_analysis::SimplePatternTokenizerType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SimplePatternTokenizer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SimplePatternTokenizer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SimplePatternTokenizer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_pattern: Option<String> = None;
                let mut seen_pattern = false;
                let mut f_type: Option<crate::types::common_analysis::SimplePatternTokenizerType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "pattern" => {
                            if seen_pattern {
                                return Err(serde::de::Error::duplicate_field("pattern"));
                            }
                            seen_pattern = true;
                            f_pattern = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::SimplePatternTokenizerType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SimplePatternTokenizer {
                    version: f_version,
                    pattern: f_pattern,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SimplePatternTokenizer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.pattern {
            map.serialize_entry("pattern", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimplePatternTokenizerType {
    #[doc = "`simple_pattern`"]
    SimplePattern,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SimplePatternTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SimplePatternTokenizerType::SimplePattern => "simple_pattern",
            SimplePatternTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SimplePatternTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SimplePatternTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "simple_pattern" => SimplePatternTokenizerType::SimplePattern,
            _ => SimplePatternTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SmartcnAnalyzer {
    pub r#type: Option<crate::types::common_analysis::SmartcnAnalyzerType>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SmartcnAnalyzer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SmartcnAnalyzer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SmartcnAnalyzer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_type: Option<crate::types::common_analysis::SmartcnAnalyzerType> = None;
                let mut seen_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<crate::types::common_analysis::SmartcnAnalyzerType>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SmartcnAnalyzer {
                    r#type: f_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SmartcnAnalyzer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.r#type {
            map.serialize_entry("type", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SmartcnAnalyzerType {
    #[doc = "`smartcn`"]
    Smartcn,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SmartcnAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SmartcnAnalyzerType::Smartcn => "smartcn",
            SmartcnAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SmartcnAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SmartcnAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "smartcn" => SmartcnAnalyzerType::Smartcn,
            _ => SmartcnAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SmartcnStopTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::SmartcnStopTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SmartcnStopTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SmartcnStopTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SmartcnStopTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_type: Option<crate::types::common_analysis::SmartcnStopTokenFilterType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::SmartcnStopTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SmartcnStopTokenFilter {
                    version: f_version,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SmartcnStopTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SmartcnStopTokenFilterType {
    #[doc = "`smartcn_stop`"]
    SmartcnStop,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SmartcnStopTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SmartcnStopTokenFilterType::SmartcnStop => "smartcn_stop",
            SmartcnStopTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SmartcnStopTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SmartcnStopTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "smartcn_stop" => SmartcnStopTokenFilterType::SmartcnStop,
            _ => SmartcnStopTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SmartcnTokenizer {
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::SmartcnTokenizerType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SmartcnTokenizer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SmartcnTokenizer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SmartcnTokenizer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_type: Option<crate::types::common_analysis::SmartcnTokenizerType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::SmartcnTokenizerType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SmartcnTokenizer {
                    version: f_version,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SmartcnTokenizer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SmartcnTokenizerType {
    #[doc = "`smartcn_tokenizer`"]
    SmartcnTokenizer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SmartcnTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SmartcnTokenizerType::SmartcnTokenizer => "smartcn_tokenizer",
            SmartcnTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SmartcnTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SmartcnTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "smartcn_tokenizer" => SmartcnTokenizerType::SmartcnTokenizer,
            _ => SmartcnTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SnowballAnalyzer {
    pub language: crate::types::common_analysis::SnowballLanguage,
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    pub r#type: crate::types::common_analysis::SnowballAnalyzerType,
    pub version: Option<crate::types::common::VersionString>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SnowballAnalyzer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SnowballAnalyzer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SnowballAnalyzer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_language: Option<crate::types::common_analysis::SnowballLanguage> = None;
                let mut f_stopwords: Option<crate::types::common_analysis::StopWords> = None;
                let mut seen_stopwords = false;
                let mut f_type: Option<crate::types::common_analysis::SnowballAnalyzerType> = None;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "language" => {
                            if f_language.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            f_language = Some(
                                map.next_value::<crate::types::common_analysis::SnowballLanguage>(
                                )?,
                            );
                        }
                        "stopwords" => {
                            if seen_stopwords {
                                return Err(serde::de::Error::duplicate_field("stopwords"));
                            }
                            seen_stopwords = true;
                            f_stopwords = map
                                .next_value::<Option<crate::types::common_analysis::StopWords>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::SnowballAnalyzerType>()?);
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SnowballAnalyzer {
                    language: f_language
                        .ok_or_else(|| serde::de::Error::missing_field("language"))?,
                    stopwords: f_stopwords,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SnowballAnalyzer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("language", &self.language)?;
        if let Some(v) = &self.stopwords {
            map.serialize_entry("stopwords", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SnowballAnalyzerType {
    #[doc = "`snowball`"]
    Snowball,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SnowballAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SnowballAnalyzerType::Snowball => "snowball",
            SnowballAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SnowballAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SnowballAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "snowball" => SnowballAnalyzerType::Snowball,
            _ => SnowballAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SnowballLanguage {
    #[doc = "`Armenian`"]
    Armenian,
    #[doc = "`Basque`"]
    Basque,
    #[doc = "`Catalan`"]
    Catalan,
    #[doc = "`Danish`"]
    Danish,
    #[doc = "`Dutch`"]
    Dutch,
    #[doc = "`English`"]
    English,
    #[doc = "`Finnish`"]
    Finnish,
    #[doc = "`French`"]
    French,
    #[doc = "`German`"]
    German,
    #[doc = "`German2`"]
    German2,
    #[doc = "`Hungarian`"]
    Hungarian,
    #[doc = "`Italian`"]
    Italian,
    #[doc = "`Kp`"]
    Kp,
    #[doc = "`Lovins`"]
    Lovins,
    #[doc = "`Norwegian`"]
    Norwegian,
    #[doc = "`Porter`"]
    Porter,
    #[doc = "`Portuguese`"]
    Portuguese,
    #[doc = "`Romanian`"]
    Romanian,
    #[doc = "`Russian`"]
    Russian,
    #[doc = "`Spanish`"]
    Spanish,
    #[doc = "`Swedish`"]
    Swedish,
    #[doc = "`Turkish`"]
    Turkish,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SnowballLanguage {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SnowballLanguage::Armenian => "Armenian",
            SnowballLanguage::Basque => "Basque",
            SnowballLanguage::Catalan => "Catalan",
            SnowballLanguage::Danish => "Danish",
            SnowballLanguage::Dutch => "Dutch",
            SnowballLanguage::English => "English",
            SnowballLanguage::Finnish => "Finnish",
            SnowballLanguage::French => "French",
            SnowballLanguage::German => "German",
            SnowballLanguage::German2 => "German2",
            SnowballLanguage::Hungarian => "Hungarian",
            SnowballLanguage::Italian => "Italian",
            SnowballLanguage::Kp => "Kp",
            SnowballLanguage::Lovins => "Lovins",
            SnowballLanguage::Norwegian => "Norwegian",
            SnowballLanguage::Porter => "Porter",
            SnowballLanguage::Portuguese => "Portuguese",
            SnowballLanguage::Romanian => "Romanian",
            SnowballLanguage::Russian => "Russian",
            SnowballLanguage::Spanish => "Spanish",
            SnowballLanguage::Swedish => "Swedish",
            SnowballLanguage::Turkish => "Turkish",
            SnowballLanguage::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SnowballLanguage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SnowballLanguage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "Armenian" => SnowballLanguage::Armenian,
            "Basque" => SnowballLanguage::Basque,
            "Catalan" => SnowballLanguage::Catalan,
            "Danish" => SnowballLanguage::Danish,
            "Dutch" => SnowballLanguage::Dutch,
            "English" => SnowballLanguage::English,
            "Finnish" => SnowballLanguage::Finnish,
            "French" => SnowballLanguage::French,
            "German" => SnowballLanguage::German,
            "German2" => SnowballLanguage::German2,
            "Hungarian" => SnowballLanguage::Hungarian,
            "Italian" => SnowballLanguage::Italian,
            "Kp" => SnowballLanguage::Kp,
            "Lovins" => SnowballLanguage::Lovins,
            "Norwegian" => SnowballLanguage::Norwegian,
            "Porter" => SnowballLanguage::Porter,
            "Portuguese" => SnowballLanguage::Portuguese,
            "Romanian" => SnowballLanguage::Romanian,
            "Russian" => SnowballLanguage::Russian,
            "Spanish" => SnowballLanguage::Spanish,
            "Swedish" => SnowballLanguage::Swedish,
            "Turkish" => SnowballLanguage::Turkish,
            _ => SnowballLanguage::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SnowballTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub language: crate::types::common_analysis::SnowballLanguage,
    pub r#type: crate::types::common_analysis::SnowballTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SnowballTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SnowballTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SnowballTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_language: Option<crate::types::common_analysis::SnowballLanguage> = None;
                let mut f_type: Option<crate::types::common_analysis::SnowballTokenFilterType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "language" => {
                            if f_language.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            f_language = Some(
                                map.next_value::<crate::types::common_analysis::SnowballLanguage>(
                                )?,
                            );
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::SnowballTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SnowballTokenFilter {
                    version: f_version,
                    language: f_language
                        .ok_or_else(|| serde::de::Error::missing_field("language"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SnowballTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("language", &self.language)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SnowballTokenFilterType {
    #[doc = "`snowball`"]
    Snowball,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SnowballTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SnowballTokenFilterType::Snowball => "snowball",
            SnowballTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SnowballTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SnowballTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "snowball" => SnowballTokenFilterType::Snowball,
            _ => SnowballTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct StandardAnalyzer {
    pub max_token_length: Option<i64>,
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    pub r#type: crate::types::common_analysis::StandardAnalyzerType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StandardAnalyzer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StandardAnalyzer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StandardAnalyzer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_max_token_length: Option<i64> = None;
                let mut seen_max_token_length = false;
                let mut f_stopwords: Option<crate::types::common_analysis::StopWords> = None;
                let mut seen_stopwords = false;
                let mut f_type: Option<crate::types::common_analysis::StandardAnalyzerType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "max_token_length" => {
                            if seen_max_token_length {
                                return Err(serde::de::Error::duplicate_field("max_token_length"));
                            }
                            seen_max_token_length = true;
                            f_max_token_length = map.next_value::<Option<i64>>()?;
                        }
                        "stopwords" => {
                            if seen_stopwords {
                                return Err(serde::de::Error::duplicate_field("stopwords"));
                            }
                            seen_stopwords = true;
                            f_stopwords = map
                                .next_value::<Option<crate::types::common_analysis::StopWords>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::StandardAnalyzerType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StandardAnalyzer {
                    max_token_length: f_max_token_length,
                    stopwords: f_stopwords,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StandardAnalyzer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.max_token_length {
            map.serialize_entry("max_token_length", v)?;
        }
        if let Some(v) = &self.stopwords {
            map.serialize_entry("stopwords", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StandardAnalyzerType {
    #[doc = "`standard`"]
    Standard,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl StandardAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            StandardAnalyzerType::Standard => "standard",
            StandardAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for StandardAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StandardAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "standard" => StandardAnalyzerType::Standard,
            _ => StandardAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct StandardTokenizer {
    pub version: Option<crate::types::common::VersionString>,
    pub max_token_length: Option<i64>,
    pub r#type: crate::types::common_analysis::StandardTokenizerType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StandardTokenizer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StandardTokenizer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StandardTokenizer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_max_token_length: Option<i64> = None;
                let mut seen_max_token_length = false;
                let mut f_type: Option<crate::types::common_analysis::StandardTokenizerType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "max_token_length" => {
                            if seen_max_token_length {
                                return Err(serde::de::Error::duplicate_field("max_token_length"));
                            }
                            seen_max_token_length = true;
                            f_max_token_length = map.next_value::<Option<i64>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::StandardTokenizerType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StandardTokenizer {
                    version: f_version,
                    max_token_length: f_max_token_length,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StandardTokenizer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.max_token_length {
            map.serialize_entry("max_token_length", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StandardTokenizerType {
    #[doc = "`standard`"]
    Standard,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl StandardTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            StandardTokenizerType::Standard => "standard",
            StandardTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for StandardTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StandardTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "standard" => StandardTokenizerType::Standard,
            _ => StandardTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct StemmerOverrideTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub rules: Option<Vec<String>>,
    pub rules_path: Option<String>,
    pub r#type: crate::types::common_analysis::StemmerOverrideTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StemmerOverrideTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StemmerOverrideTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StemmerOverrideTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_rules: Option<Vec<String>> = None;
                let mut seen_rules = false;
                let mut f_rules_path: Option<String> = None;
                let mut seen_rules_path = false;
                let mut f_type: Option<
                    crate::types::common_analysis::StemmerOverrideTokenFilterType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "rules" => {
                            if seen_rules {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            seen_rules = true;
                            f_rules = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "rules_path" => {
                            if seen_rules_path {
                                return Err(serde::de::Error::duplicate_field("rules_path"));
                            }
                            seen_rules_path = true;
                            f_rules_path = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::StemmerOverrideTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StemmerOverrideTokenFilter {
                    version: f_version,
                    rules: f_rules,
                    rules_path: f_rules_path,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StemmerOverrideTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.rules {
            map.serialize_entry("rules", v)?;
        }
        if let Some(v) = &self.rules_path {
            map.serialize_entry("rules_path", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StemmerOverrideTokenFilterType {
    #[doc = "`stemmer_override`"]
    StemmerOverride,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl StemmerOverrideTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            StemmerOverrideTokenFilterType::StemmerOverride => "stemmer_override",
            StemmerOverrideTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for StemmerOverrideTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StemmerOverrideTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "stemmer_override" => StemmerOverrideTokenFilterType::StemmerOverride,
            _ => StemmerOverrideTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct StemmerTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub language: Option<String>,
    pub r#type: crate::types::common_analysis::StemmerTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StemmerTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StemmerTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StemmerTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_language: Option<String> = None;
                let mut seen_language = false;
                let mut f_type: Option<crate::types::common_analysis::StemmerTokenFilterType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "language" => {
                            if seen_language {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            seen_language = true;
                            f_language = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::StemmerTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StemmerTokenFilter {
                    version: f_version,
                    language: f_language,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StemmerTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.language {
            map.serialize_entry("language", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StemmerTokenFilterType {
    #[doc = "`stemmer`"]
    Stemmer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl StemmerTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            StemmerTokenFilterType::Stemmer => "stemmer",
            StemmerTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for StemmerTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StemmerTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "stemmer" => StemmerTokenFilterType::Stemmer,
            _ => StemmerTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct StopAnalyzer {
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    pub stopwords_path: Option<String>,
    pub r#type: crate::types::common_analysis::StopAnalyzerType,
    pub version: Option<crate::types::common::VersionString>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StopAnalyzer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StopAnalyzer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StopAnalyzer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_stopwords: Option<crate::types::common_analysis::StopWords> = None;
                let mut seen_stopwords = false;
                let mut f_stopwords_path: Option<String> = None;
                let mut seen_stopwords_path = false;
                let mut f_type: Option<crate::types::common_analysis::StopAnalyzerType> = None;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "stopwords" => {
                            if seen_stopwords {
                                return Err(serde::de::Error::duplicate_field("stopwords"));
                            }
                            seen_stopwords = true;
                            f_stopwords = map
                                .next_value::<Option<crate::types::common_analysis::StopWords>>()?;
                        }
                        "stopwords_path" => {
                            if seen_stopwords_path {
                                return Err(serde::de::Error::duplicate_field("stopwords_path"));
                            }
                            seen_stopwords_path = true;
                            f_stopwords_path = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(
                                map.next_value::<crate::types::common_analysis::StopAnalyzerType>(
                                )?,
                            );
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StopAnalyzer {
                    stopwords: f_stopwords,
                    stopwords_path: f_stopwords_path,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StopAnalyzer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.stopwords {
            map.serialize_entry("stopwords", v)?;
        }
        if let Some(v) = &self.stopwords_path {
            map.serialize_entry("stopwords_path", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StopAnalyzerType {
    #[doc = "`stop`"]
    Stop,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl StopAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            StopAnalyzerType::Stop => "stop",
            StopAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for StopAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StopAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "stop" => StopAnalyzerType::Stop,
            _ => StopAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct StopTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub ignore_case: Option<bool>,
    pub remove_trailing: Option<bool>,
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    pub stopwords_path: Option<String>,
    pub r#type: crate::types::common_analysis::StopTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StopTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StopTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StopTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_ignore_case: Option<bool> = None;
                let mut seen_ignore_case = false;
                let mut f_remove_trailing: Option<bool> = None;
                let mut seen_remove_trailing = false;
                let mut f_stopwords: Option<crate::types::common_analysis::StopWords> = None;
                let mut seen_stopwords = false;
                let mut f_stopwords_path: Option<String> = None;
                let mut seen_stopwords_path = false;
                let mut f_type: Option<crate::types::common_analysis::StopTokenFilterType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "ignore_case" => {
                            if seen_ignore_case {
                                return Err(serde::de::Error::duplicate_field("ignore_case"));
                            }
                            seen_ignore_case = true;
                            f_ignore_case = map.next_value::<Option<bool>>()?;
                        }
                        "remove_trailing" => {
                            if seen_remove_trailing {
                                return Err(serde::de::Error::duplicate_field("remove_trailing"));
                            }
                            seen_remove_trailing = true;
                            f_remove_trailing = map.next_value::<Option<bool>>()?;
                        }
                        "stopwords" => {
                            if seen_stopwords {
                                return Err(serde::de::Error::duplicate_field("stopwords"));
                            }
                            seen_stopwords = true;
                            f_stopwords = map
                                .next_value::<Option<crate::types::common_analysis::StopWords>>()?;
                        }
                        "stopwords_path" => {
                            if seen_stopwords_path {
                                return Err(serde::de::Error::duplicate_field("stopwords_path"));
                            }
                            seen_stopwords_path = true;
                            f_stopwords_path = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::StopTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StopTokenFilter {
                    version: f_version,
                    ignore_case: f_ignore_case,
                    remove_trailing: f_remove_trailing,
                    stopwords: f_stopwords,
                    stopwords_path: f_stopwords_path,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StopTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.ignore_case {
            map.serialize_entry("ignore_case", v)?;
        }
        if let Some(v) = &self.remove_trailing {
            map.serialize_entry("remove_trailing", v)?;
        }
        if let Some(v) = &self.stopwords {
            map.serialize_entry("stopwords", v)?;
        }
        if let Some(v) = &self.stopwords_path {
            map.serialize_entry("stopwords_path", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StopTokenFilterType {
    #[doc = "`stop`"]
    Stop,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl StopTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            StopTokenFilterType::Stop => "stop",
            StopTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for StopTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StopTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "stop" => StopTokenFilterType::Stop,
            _ => StopTokenFilterType::Unknown(s),
        })
    }
}

#[doc = "Language value, such as `arabic` or `thai`. Defaults to `english`. Each language value corresponds to a predefined list of stop words in Lucene. See Stop words by language for supported language values and their stop words. Also accepts an array of stop words."]
pub type StopWords = crate::types::common::StringOrStringArray;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SynonymFormat {
    #[doc = "`solr`"]
    Solr,
    #[doc = "`wordnet`"]
    Wordnet,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SynonymFormat {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SynonymFormat::Solr => "solr",
            SynonymFormat::Wordnet => "wordnet",
            SynonymFormat::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SynonymFormat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SynonymFormat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "solr" => SynonymFormat::Solr,
            "wordnet" => SynonymFormat::Wordnet,
            _ => SynonymFormat::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SynonymGraphTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub expand: Option<bool>,
    pub format: Option<crate::types::common_analysis::SynonymFormat>,
    pub lenient: Option<bool>,
    pub synonyms: Option<Vec<String>>,
    pub synonyms_path: Option<String>,
    pub tokenizer: Option<String>,
    pub r#type: crate::types::common_analysis::SynonymGraphTokenFilterType,
    pub updateable: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SynonymGraphTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SynonymGraphTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SynonymGraphTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_expand: Option<bool> = None;
                let mut seen_expand = false;
                let mut f_format: Option<crate::types::common_analysis::SynonymFormat> = None;
                let mut seen_format = false;
                let mut f_lenient: Option<bool> = None;
                let mut seen_lenient = false;
                let mut f_synonyms: Option<Vec<String>> = None;
                let mut seen_synonyms = false;
                let mut f_synonyms_path: Option<String> = None;
                let mut seen_synonyms_path = false;
                let mut f_tokenizer: Option<String> = None;
                let mut seen_tokenizer = false;
                let mut f_type: Option<crate::types::common_analysis::SynonymGraphTokenFilterType> =
                    None;
                let mut f_updateable: Option<bool> = None;
                let mut seen_updateable = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "expand" => {
                            if seen_expand {
                                return Err(serde::de::Error::duplicate_field("expand"));
                            }
                            seen_expand = true;
                            f_expand = map.next_value::<Option<bool>>()?;
                        }
                        "format" => {
                            if seen_format {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            seen_format = true;
                            f_format = map
                                .next_value::<Option<crate::types::common_analysis::SynonymFormat>>(
                                )?;
                        }
                        "lenient" => {
                            if seen_lenient {
                                return Err(serde::de::Error::duplicate_field("lenient"));
                            }
                            seen_lenient = true;
                            f_lenient = map.next_value::<Option<bool>>()?;
                        }
                        "synonyms" => {
                            if seen_synonyms {
                                return Err(serde::de::Error::duplicate_field("synonyms"));
                            }
                            seen_synonyms = true;
                            f_synonyms = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "synonyms_path" => {
                            if seen_synonyms_path {
                                return Err(serde::de::Error::duplicate_field("synonyms_path"));
                            }
                            seen_synonyms_path = true;
                            f_synonyms_path = map.next_value::<Option<String>>()?;
                        }
                        "tokenizer" => {
                            if seen_tokenizer {
                                return Err(serde::de::Error::duplicate_field("tokenizer"));
                            }
                            seen_tokenizer = true;
                            f_tokenizer = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::SynonymGraphTokenFilterType>()?);
                        }
                        "updateable" => {
                            if seen_updateable {
                                return Err(serde::de::Error::duplicate_field("updateable"));
                            }
                            seen_updateable = true;
                            f_updateable = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SynonymGraphTokenFilter {
                    version: f_version,
                    expand: f_expand,
                    format: f_format,
                    lenient: f_lenient,
                    synonyms: f_synonyms,
                    synonyms_path: f_synonyms_path,
                    tokenizer: f_tokenizer,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    updateable: f_updateable,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SynonymGraphTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.expand {
            map.serialize_entry("expand", v)?;
        }
        if let Some(v) = &self.format {
            map.serialize_entry("format", v)?;
        }
        if let Some(v) = &self.lenient {
            map.serialize_entry("lenient", v)?;
        }
        if let Some(v) = &self.synonyms {
            map.serialize_entry("synonyms", v)?;
        }
        if let Some(v) = &self.synonyms_path {
            map.serialize_entry("synonyms_path", v)?;
        }
        if let Some(v) = &self.tokenizer {
            map.serialize_entry("tokenizer", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        if let Some(v) = &self.updateable {
            map.serialize_entry("updateable", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SynonymGraphTokenFilterType {
    #[doc = "`synonym_graph`"]
    SynonymGraph,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SynonymGraphTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SynonymGraphTokenFilterType::SynonymGraph => "synonym_graph",
            SynonymGraphTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SynonymGraphTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SynonymGraphTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "synonym_graph" => SynonymGraphTokenFilterType::SynonymGraph,
            _ => SynonymGraphTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SynonymTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub expand: Option<bool>,
    pub format: Option<crate::types::common_analysis::SynonymFormat>,
    pub lenient: Option<bool>,
    pub synonyms: Option<Vec<String>>,
    pub synonyms_path: Option<String>,
    pub tokenizer: Option<String>,
    pub r#type: crate::types::common_analysis::SynonymTokenFilterType,
    pub updateable: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SynonymTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SynonymTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SynonymTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_expand: Option<bool> = None;
                let mut seen_expand = false;
                let mut f_format: Option<crate::types::common_analysis::SynonymFormat> = None;
                let mut seen_format = false;
                let mut f_lenient: Option<bool> = None;
                let mut seen_lenient = false;
                let mut f_synonyms: Option<Vec<String>> = None;
                let mut seen_synonyms = false;
                let mut f_synonyms_path: Option<String> = None;
                let mut seen_synonyms_path = false;
                let mut f_tokenizer: Option<String> = None;
                let mut seen_tokenizer = false;
                let mut f_type: Option<crate::types::common_analysis::SynonymTokenFilterType> =
                    None;
                let mut f_updateable: Option<bool> = None;
                let mut seen_updateable = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "expand" => {
                            if seen_expand {
                                return Err(serde::de::Error::duplicate_field("expand"));
                            }
                            seen_expand = true;
                            f_expand = map.next_value::<Option<bool>>()?;
                        }
                        "format" => {
                            if seen_format {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            seen_format = true;
                            f_format = map
                                .next_value::<Option<crate::types::common_analysis::SynonymFormat>>(
                                )?;
                        }
                        "lenient" => {
                            if seen_lenient {
                                return Err(serde::de::Error::duplicate_field("lenient"));
                            }
                            seen_lenient = true;
                            f_lenient = map.next_value::<Option<bool>>()?;
                        }
                        "synonyms" => {
                            if seen_synonyms {
                                return Err(serde::de::Error::duplicate_field("synonyms"));
                            }
                            seen_synonyms = true;
                            f_synonyms = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "synonyms_path" => {
                            if seen_synonyms_path {
                                return Err(serde::de::Error::duplicate_field("synonyms_path"));
                            }
                            seen_synonyms_path = true;
                            f_synonyms_path = map.next_value::<Option<String>>()?;
                        }
                        "tokenizer" => {
                            if seen_tokenizer {
                                return Err(serde::de::Error::duplicate_field("tokenizer"));
                            }
                            seen_tokenizer = true;
                            f_tokenizer = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::SynonymTokenFilterType>()?);
                        }
                        "updateable" => {
                            if seen_updateable {
                                return Err(serde::de::Error::duplicate_field("updateable"));
                            }
                            seen_updateable = true;
                            f_updateable = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SynonymTokenFilter {
                    version: f_version,
                    expand: f_expand,
                    format: f_format,
                    lenient: f_lenient,
                    synonyms: f_synonyms,
                    synonyms_path: f_synonyms_path,
                    tokenizer: f_tokenizer,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    updateable: f_updateable,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SynonymTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.expand {
            map.serialize_entry("expand", v)?;
        }
        if let Some(v) = &self.format {
            map.serialize_entry("format", v)?;
        }
        if let Some(v) = &self.lenient {
            map.serialize_entry("lenient", v)?;
        }
        if let Some(v) = &self.synonyms {
            map.serialize_entry("synonyms", v)?;
        }
        if let Some(v) = &self.synonyms_path {
            map.serialize_entry("synonyms_path", v)?;
        }
        if let Some(v) = &self.tokenizer {
            map.serialize_entry("tokenizer", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        if let Some(v) = &self.updateable {
            map.serialize_entry("updateable", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SynonymTokenFilterType {
    #[doc = "`synonym`"]
    Synonym,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SynonymTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SynonymTokenFilterType::Synonym => "synonym",
            SynonymTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SynonymTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SynonymTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "synonym" => SynonymTokenFilterType::Synonym,
            _ => SynonymTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenChar {
    #[doc = "`custom`"]
    Custom,
    #[doc = "`digit`"]
    Digit,
    #[doc = "`letter`"]
    Letter,
    #[doc = "`punctuation`"]
    Punctuation,
    #[doc = "`symbol`"]
    Symbol,
    #[doc = "`whitespace`"]
    Whitespace,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TokenChar {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TokenChar::Custom => "custom",
            TokenChar::Digit => "digit",
            TokenChar::Letter => "letter",
            TokenChar::Punctuation => "punctuation",
            TokenChar::Symbol => "symbol",
            TokenChar::Whitespace => "whitespace",
            TokenChar::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TokenChar {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TokenChar {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "custom" => TokenChar::Custom,
            "digit" => TokenChar::Digit,
            "letter" => TokenChar::Letter,
            "punctuation" => TokenChar::Punctuation,
            "symbol" => TokenChar::Symbol,
            "whitespace" => TokenChar::Whitespace,
            _ => TokenChar::Unknown(s),
        })
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TokenFilter {
    raw: Box<serde_json::value::RawValue>,
}
impl TokenFilter {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `name` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_name(&self) -> std::result::Result<String, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "name")
    }
    #[doc = "Parses the value as the `definition` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_definition(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::TokenFilterDefinition,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "definition")
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TokenFilterDefinition {
    raw: Box<serde_json::value::RawValue>,
}
impl TokenFilterDefinition {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `ascii_folding_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_ascii_folding_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::AsciiFoldingTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "ascii_folding_token_filter")
    }
    #[doc = "Parses the value as the `common_grams_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_common_grams_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::CommonGramsTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "common_grams_token_filter")
    }
    #[doc = "Parses the value as the `condition_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_condition_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::ConditionTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "condition_token_filter")
    }
    #[doc = "Parses the value as the `delimited_payload_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_delimited_payload_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::DelimitedPayloadTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "delimited_payload_token_filter")
    }
    #[doc = "Parses the value as the `edge_n_gram_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_edge_n_gram_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::EdgeNGramTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "edge_n_gram_token_filter")
    }
    #[doc = "Parses the value as the `elision_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_elision_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::ElisionTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "elision_token_filter")
    }
    #[doc = "Parses the value as the `fingerprint_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_fingerprint_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::FingerprintTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "fingerprint_token_filter")
    }
    #[doc = "Parses the value as the `hunspell_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_hunspell_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::HunspellTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "hunspell_token_filter")
    }
    #[doc = "Parses the value as the `hyphenation_decompounder_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_hyphenation_decompounder_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::HyphenationDecompounderTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "hyphenation_decompounder_token_filter")
    }
    #[doc = "Parses the value as the `keep_types_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_keep_types_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KeepTypesTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "keep_types_token_filter")
    }
    #[doc = "Parses the value as the `keep_words_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_keep_words_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KeepWordsTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "keep_words_token_filter")
    }
    #[doc = "Parses the value as the `keyword_marker_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_keyword_marker_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KeywordMarkerTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "keyword_marker_token_filter")
    }
    #[doc = "Parses the value as the `k_stem_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_k_stem_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KStemTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "k_stem_token_filter")
    }
    #[doc = "Parses the value as the `length_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_length_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::LengthTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "length_token_filter")
    }
    #[doc = "Parses the value as the `limit_token_count_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_limit_token_count_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::LimitTokenCountTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "limit_token_count_token_filter")
    }
    #[doc = "Parses the value as the `lowercase_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_lowercase_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::LowercaseTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "lowercase_token_filter")
    }
    #[doc = "Parses the value as the `multiplexer_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_multiplexer_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::MultiplexerTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "multiplexer_token_filter")
    }
    #[doc = "Parses the value as the `n_gram_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_n_gram_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::NGramTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "n_gram_token_filter")
    }
    #[doc = "Parses the value as the `nori_part_of_speech_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_nori_part_of_speech_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::NoriPartOfSpeechTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "nori_part_of_speech_token_filter")
    }
    #[doc = "Parses the value as the `pattern_capture_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_pattern_capture_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PatternCaptureTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "pattern_capture_token_filter")
    }
    #[doc = "Parses the value as the `pattern_replace_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_pattern_replace_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PatternReplaceTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "pattern_replace_token_filter")
    }
    #[doc = "Parses the value as the `persian_stem_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_persian_stem_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PersianStemTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "persian_stem_token_filter")
    }
    #[doc = "Parses the value as the `porter_stem_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_porter_stem_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PorterStemTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "porter_stem_token_filter")
    }
    #[doc = "Parses the value as the `predicate_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_predicate_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PredicateTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "predicate_token_filter")
    }
    #[doc = "Parses the value as the `remove_duplicates_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_remove_duplicates_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::RemoveDuplicatesTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "remove_duplicates_token_filter")
    }
    #[doc = "Parses the value as the `reverse_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_reverse_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::ReverseTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "reverse_token_filter")
    }
    #[doc = "Parses the value as the `shingle_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_shingle_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::ShingleTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "shingle_token_filter")
    }
    #[doc = "Parses the value as the `snowball_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_snowball_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::SnowballTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "snowball_token_filter")
    }
    #[doc = "Parses the value as the `stemmer_override_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_stemmer_override_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::StemmerOverrideTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "stemmer_override_token_filter")
    }
    #[doc = "Parses the value as the `stemmer_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_stemmer_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::StemmerTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "stemmer_token_filter")
    }
    #[doc = "Parses the value as the `stop_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_stop_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::StopTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "stop_token_filter")
    }
    #[doc = "Parses the value as the `synonym_graph_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_synonym_graph_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::SynonymGraphTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "synonym_graph_token_filter")
    }
    #[doc = "Parses the value as the `synonym_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_synonym_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::SynonymTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "synonym_token_filter")
    }
    #[doc = "Parses the value as the `trim_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_trim_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::TrimTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "trim_token_filter")
    }
    #[doc = "Parses the value as the `truncate_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_truncate_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::TruncateTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "truncate_token_filter")
    }
    #[doc = "Parses the value as the `unique_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_unique_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::UniqueTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "unique_token_filter")
    }
    #[doc = "Parses the value as the `uppercase_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_uppercase_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::UppercaseTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "uppercase_token_filter")
    }
    #[doc = "Parses the value as the `word_delimiter_graph_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_word_delimiter_graph_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::WordDelimiterGraphTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "word_delimiter_graph_token_filter")
    }
    #[doc = "Parses the value as the `word_delimiter_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_word_delimiter_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::WordDelimiterTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "word_delimiter_token_filter")
    }
    #[doc = "Parses the value as the `kuromoji_stemmer_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_kuromoji_stemmer_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KuromojiStemmerTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "kuromoji_stemmer_token_filter")
    }
    #[doc = "Parses the value as the `kuromoji_reading_form_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_kuromoji_reading_form_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KuromojiReadingFormTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "kuromoji_reading_form_token_filter")
    }
    #[doc = "Parses the value as the `kuromoji_part_of_speech_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_kuromoji_part_of_speech_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KuromojiPartOfSpeechTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "kuromoji_part_of_speech_token_filter")
    }
    #[doc = "Parses the value as the `icu_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_icu_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::IcuTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "icu_tokenizer")
    }
    #[doc = "Parses the value as the `icu_collation_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_icu_collation_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::IcuCollationTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "icu_collation_token_filter")
    }
    #[doc = "Parses the value as the `icu_folding_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_icu_folding_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::IcuFoldingTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "icu_folding_token_filter")
    }
    #[doc = "Parses the value as the `icu_normalization_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_icu_normalization_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::IcuNormalizationTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "icu_normalization_token_filter")
    }
    #[doc = "Parses the value as the `icu_transform_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_icu_transform_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::IcuTransformTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "icu_transform_token_filter")
    }
    #[doc = "Parses the value as the `phonetic_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_phonetic_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PhoneticTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "phonetic_token_filter")
    }
    #[doc = "Parses the value as the `dictionary_decompounder_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_dictionary_decompounder_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::DictionaryDecompounderTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "dictionary_decompounder_token_filter")
    }
    #[doc = "Parses the value as the `smartcn_stop_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_smartcn_stop_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::SmartcnStopTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "smartcn_stop_token_filter")
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Tokenizer {
    raw: Box<serde_json::value::RawValue>,
}
impl Tokenizer {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `name` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_name(&self) -> std::result::Result<String, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "name")
    }
    #[doc = "Parses the value as the `definition` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_definition(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::TokenizerDefinition,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "definition")
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TokenizerDefinition {
    raw: Box<serde_json::value::RawValue>,
}
impl TokenizerDefinition {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `char_group_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_char_group_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::CharGroupTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "char_group_tokenizer")
    }
    #[doc = "Parses the value as the `edge_n_gram_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_edge_n_gram_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::EdgeNGramTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "edge_n_gram_tokenizer")
    }
    #[doc = "Parses the value as the `keyword_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_keyword_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KeywordTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "keyword_tokenizer")
    }
    #[doc = "Parses the value as the `letter_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_letter_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::LetterTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "letter_tokenizer")
    }
    #[doc = "Parses the value as the `lowercase_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_lowercase_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::LowercaseTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "lowercase_tokenizer")
    }
    #[doc = "Parses the value as the `n_gram_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_n_gram_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::NGramTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "n_gram_tokenizer")
    }
    #[doc = "Parses the value as the `nori_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_nori_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::NoriTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "nori_tokenizer")
    }
    #[doc = "Parses the value as the `path_hierarchy_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_path_hierarchy_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PathHierarchyTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "path_hierarchy_tokenizer")
    }
    #[doc = "Parses the value as the `standard_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_standard_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::StandardTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "standard_tokenizer")
    }
    #[doc = "Parses the value as the `uax_email_url_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_uax_email_url_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::UaxEmailUrlTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "uax_email_url_tokenizer")
    }
    #[doc = "Parses the value as the `whitespace_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_whitespace_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::WhitespaceTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "whitespace_tokenizer")
    }
    #[doc = "Parses the value as the `kuromoji_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_kuromoji_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KuromojiTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "kuromoji_tokenizer")
    }
    #[doc = "Parses the value as the `pattern_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_pattern_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PatternTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "pattern_tokenizer")
    }
    #[doc = "Parses the value as the `simple_pattern_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_simple_pattern_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::SimplePatternTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "simple_pattern_tokenizer")
    }
    #[doc = "Parses the value as the `simple_pattern_split_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_simple_pattern_split_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::SimplePatternSplitTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "simple_pattern_split_tokenizer")
    }
    #[doc = "Parses the value as the `icu_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_icu_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::IcuTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "icu_tokenizer")
    }
    #[doc = "Parses the value as the `smartcn_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_smartcn_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::SmartcnTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "smartcn_tokenizer")
    }
}

#[derive(Debug, Clone)]
pub struct TrimTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::TrimTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TrimTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TrimTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TrimTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_type: Option<crate::types::common_analysis::TrimTokenFilterType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::TrimTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TrimTokenFilter {
                    version: f_version,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TrimTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TrimTokenFilterType {
    #[doc = "`trim`"]
    Trim,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TrimTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TrimTokenFilterType::Trim => "trim",
            TrimTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TrimTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TrimTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "trim" => TrimTokenFilterType::Trim,
            _ => TrimTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct TruncateTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub length: Option<i64>,
    pub r#type: crate::types::common_analysis::TruncateTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TruncateTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TruncateTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TruncateTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_length: Option<i64> = None;
                let mut seen_length = false;
                let mut f_type: Option<crate::types::common_analysis::TruncateTokenFilterType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "length" => {
                            if seen_length {
                                return Err(serde::de::Error::duplicate_field("length"));
                            }
                            seen_length = true;
                            f_length = map.next_value::<Option<i64>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::TruncateTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TruncateTokenFilter {
                    version: f_version,
                    length: f_length,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TruncateTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.length {
            map.serialize_entry("length", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TruncateTokenFilterType {
    #[doc = "`truncate`"]
    Truncate,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TruncateTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TruncateTokenFilterType::Truncate => "truncate",
            TruncateTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TruncateTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TruncateTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "truncate" => TruncateTokenFilterType::Truncate,
            _ => TruncateTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct UaxEmailUrlTokenizer {
    pub version: Option<crate::types::common::VersionString>,
    pub max_token_length: Option<i64>,
    pub r#type: crate::types::common_analysis::UaxEmailUrlTokenizerType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UaxEmailUrlTokenizer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UaxEmailUrlTokenizer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UaxEmailUrlTokenizer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_max_token_length: Option<i64> = None;
                let mut seen_max_token_length = false;
                let mut f_type: Option<crate::types::common_analysis::UaxEmailUrlTokenizerType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "max_token_length" => {
                            if seen_max_token_length {
                                return Err(serde::de::Error::duplicate_field("max_token_length"));
                            }
                            seen_max_token_length = true;
                            f_max_token_length = map.next_value::<Option<i64>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::UaxEmailUrlTokenizerType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UaxEmailUrlTokenizer {
                    version: f_version,
                    max_token_length: f_max_token_length,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UaxEmailUrlTokenizer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.max_token_length {
            map.serialize_entry("max_token_length", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UaxEmailUrlTokenizerType {
    #[doc = "`uax_url_email`"]
    UaxUrlEmail,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl UaxEmailUrlTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            UaxEmailUrlTokenizerType::UaxUrlEmail => "uax_url_email",
            UaxEmailUrlTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for UaxEmailUrlTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UaxEmailUrlTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "uax_url_email" => UaxEmailUrlTokenizerType::UaxUrlEmail,
            _ => UaxEmailUrlTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct UniqueTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub only_on_same_position: Option<bool>,
    pub r#type: crate::types::common_analysis::UniqueTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UniqueTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UniqueTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UniqueTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_only_on_same_position: Option<bool> = None;
                let mut seen_only_on_same_position = false;
                let mut f_type: Option<crate::types::common_analysis::UniqueTokenFilterType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "only_on_same_position" => {
                            if seen_only_on_same_position {
                                return Err(serde::de::Error::duplicate_field(
                                    "only_on_same_position",
                                ));
                            }
                            seen_only_on_same_position = true;
                            f_only_on_same_position = map.next_value::<Option<bool>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::UniqueTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UniqueTokenFilter {
                    version: f_version,
                    only_on_same_position: f_only_on_same_position,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UniqueTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.only_on_same_position {
            map.serialize_entry("only_on_same_position", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UniqueTokenFilterType {
    #[doc = "`unique`"]
    Unique,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl UniqueTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            UniqueTokenFilterType::Unique => "unique",
            UniqueTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for UniqueTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UniqueTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "unique" => UniqueTokenFilterType::Unique,
            _ => UniqueTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct UppercaseTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::UppercaseTokenFilterType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UppercaseTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UppercaseTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UppercaseTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_type: Option<crate::types::common_analysis::UppercaseTokenFilterType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::UppercaseTokenFilterType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UppercaseTokenFilter {
                    version: f_version,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UppercaseTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UppercaseTokenFilterType {
    #[doc = "`uppercase`"]
    Uppercase,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl UppercaseTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            UppercaseTokenFilterType::Uppercase => "uppercase",
            UppercaseTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for UppercaseTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UppercaseTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "uppercase" => UppercaseTokenFilterType::Uppercase,
            _ => UppercaseTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct WhitespaceAnalyzer {
    pub r#type: crate::types::common_analysis::WhitespaceAnalyzerType,
    pub version: Option<crate::types::common::VersionString>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for WhitespaceAnalyzer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = WhitespaceAnalyzer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a WhitespaceAnalyzer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_type: Option<crate::types::common_analysis::WhitespaceAnalyzerType> =
                    None;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::WhitespaceAnalyzerType>()?);
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(WhitespaceAnalyzer {
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for WhitespaceAnalyzer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("type", &self.r#type)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WhitespaceAnalyzerType {
    #[doc = "`whitespace`"]
    Whitespace,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl WhitespaceAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            WhitespaceAnalyzerType::Whitespace => "whitespace",
            WhitespaceAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for WhitespaceAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for WhitespaceAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "whitespace" => WhitespaceAnalyzerType::Whitespace,
            _ => WhitespaceAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct WhitespaceTokenizer {
    pub version: Option<crate::types::common::VersionString>,
    pub max_token_length: Option<i64>,
    pub r#type: crate::types::common_analysis::WhitespaceTokenizerType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for WhitespaceTokenizer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = WhitespaceTokenizer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a WhitespaceTokenizer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_max_token_length: Option<i64> = None;
                let mut seen_max_token_length = false;
                let mut f_type: Option<crate::types::common_analysis::WhitespaceTokenizerType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "max_token_length" => {
                            if seen_max_token_length {
                                return Err(serde::de::Error::duplicate_field("max_token_length"));
                            }
                            seen_max_token_length = true;
                            f_max_token_length = map.next_value::<Option<i64>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::WhitespaceTokenizerType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(WhitespaceTokenizer {
                    version: f_version,
                    max_token_length: f_max_token_length,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for WhitespaceTokenizer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.max_token_length {
            map.serialize_entry("max_token_length", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WhitespaceTokenizerType {
    #[doc = "`whitespace`"]
    Whitespace,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl WhitespaceTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            WhitespaceTokenizerType::Whitespace => "whitespace",
            WhitespaceTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for WhitespaceTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for WhitespaceTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "whitespace" => WhitespaceTokenizerType::Whitespace,
            _ => WhitespaceTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct WordDelimiterGraphTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub adjust_offsets: Option<bool>,
    pub catenate_all: Option<bool>,
    pub catenate_numbers: Option<bool>,
    pub catenate_words: Option<bool>,
    pub generate_number_parts: Option<bool>,
    pub generate_word_parts: Option<bool>,
    pub ignore_keywords: Option<bool>,
    pub preserve_original: Option<crate::types::common::StringifiedBoolean>,
    pub protected_words: Option<Vec<String>>,
    pub protected_words_path: Option<String>,
    pub split_on_case_change: Option<bool>,
    pub split_on_numerics: Option<bool>,
    pub stem_english_possessive: Option<bool>,
    pub r#type: crate::types::common_analysis::WordDelimiterGraphTokenFilterType,
    pub type_table: Option<Vec<String>>,
    pub type_table_path: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for WordDelimiterGraphTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = WordDelimiterGraphTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a WordDelimiterGraphTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_adjust_offsets: Option<bool> = None;
                let mut seen_adjust_offsets = false;
                let mut f_catenate_all: Option<bool> = None;
                let mut seen_catenate_all = false;
                let mut f_catenate_numbers: Option<bool> = None;
                let mut seen_catenate_numbers = false;
                let mut f_catenate_words: Option<bool> = None;
                let mut seen_catenate_words = false;
                let mut f_generate_number_parts: Option<bool> = None;
                let mut seen_generate_number_parts = false;
                let mut f_generate_word_parts: Option<bool> = None;
                let mut seen_generate_word_parts = false;
                let mut f_ignore_keywords: Option<bool> = None;
                let mut seen_ignore_keywords = false;
                let mut f_preserve_original: Option<crate::types::common::StringifiedBoolean> =
                    None;
                let mut seen_preserve_original = false;
                let mut f_protected_words: Option<Vec<String>> = None;
                let mut seen_protected_words = false;
                let mut f_protected_words_path: Option<String> = None;
                let mut seen_protected_words_path = false;
                let mut f_split_on_case_change: Option<bool> = None;
                let mut seen_split_on_case_change = false;
                let mut f_split_on_numerics: Option<bool> = None;
                let mut seen_split_on_numerics = false;
                let mut f_stem_english_possessive: Option<bool> = None;
                let mut seen_stem_english_possessive = false;
                let mut f_type: Option<
                    crate::types::common_analysis::WordDelimiterGraphTokenFilterType,
                > = None;
                let mut f_type_table: Option<Vec<String>> = None;
                let mut seen_type_table = false;
                let mut f_type_table_path: Option<String> = None;
                let mut seen_type_table_path = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "adjust_offsets" => {
                            if seen_adjust_offsets {
                                return Err(serde::de::Error::duplicate_field("adjust_offsets"));
                            }
                            seen_adjust_offsets = true;
                            f_adjust_offsets = map.next_value::<Option<bool>>()?;
                        }
                        "catenate_all" => {
                            if seen_catenate_all {
                                return Err(serde::de::Error::duplicate_field("catenate_all"));
                            }
                            seen_catenate_all = true;
                            f_catenate_all = map.next_value::<Option<bool>>()?;
                        }
                        "catenate_numbers" => {
                            if seen_catenate_numbers {
                                return Err(serde::de::Error::duplicate_field("catenate_numbers"));
                            }
                            seen_catenate_numbers = true;
                            f_catenate_numbers = map.next_value::<Option<bool>>()?;
                        }
                        "catenate_words" => {
                            if seen_catenate_words {
                                return Err(serde::de::Error::duplicate_field("catenate_words"));
                            }
                            seen_catenate_words = true;
                            f_catenate_words = map.next_value::<Option<bool>>()?;
                        }
                        "generate_number_parts" => {
                            if seen_generate_number_parts {
                                return Err(serde::de::Error::duplicate_field(
                                    "generate_number_parts",
                                ));
                            }
                            seen_generate_number_parts = true;
                            f_generate_number_parts = map.next_value::<Option<bool>>()?;
                        }
                        "generate_word_parts" => {
                            if seen_generate_word_parts {
                                return Err(serde::de::Error::duplicate_field(
                                    "generate_word_parts",
                                ));
                            }
                            seen_generate_word_parts = true;
                            f_generate_word_parts = map.next_value::<Option<bool>>()?;
                        }
                        "ignore_keywords" => {
                            if seen_ignore_keywords {
                                return Err(serde::de::Error::duplicate_field("ignore_keywords"));
                            }
                            seen_ignore_keywords = true;
                            f_ignore_keywords = map.next_value::<Option<bool>>()?;
                        }
                        "preserve_original" => {
                            if seen_preserve_original {
                                return Err(serde::de::Error::duplicate_field("preserve_original"));
                            }
                            seen_preserve_original = true;
                            f_preserve_original = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "protected_words" => {
                            if seen_protected_words {
                                return Err(serde::de::Error::duplicate_field("protected_words"));
                            }
                            seen_protected_words = true;
                            f_protected_words = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "protected_words_path" => {
                            if seen_protected_words_path {
                                return Err(serde::de::Error::duplicate_field(
                                    "protected_words_path",
                                ));
                            }
                            seen_protected_words_path = true;
                            f_protected_words_path = map.next_value::<Option<String>>()?;
                        }
                        "split_on_case_change" => {
                            if seen_split_on_case_change {
                                return Err(serde::de::Error::duplicate_field(
                                    "split_on_case_change",
                                ));
                            }
                            seen_split_on_case_change = true;
                            f_split_on_case_change = map.next_value::<Option<bool>>()?;
                        }
                        "split_on_numerics" => {
                            if seen_split_on_numerics {
                                return Err(serde::de::Error::duplicate_field("split_on_numerics"));
                            }
                            seen_split_on_numerics = true;
                            f_split_on_numerics = map.next_value::<Option<bool>>()?;
                        }
                        "stem_english_possessive" => {
                            if seen_stem_english_possessive {
                                return Err(serde::de::Error::duplicate_field(
                                    "stem_english_possessive",
                                ));
                            }
                            seen_stem_english_possessive = true;
                            f_stem_english_possessive = map.next_value::<Option<bool>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::WordDelimiterGraphTokenFilterType>()?);
                        }
                        "type_table" => {
                            if seen_type_table {
                                return Err(serde::de::Error::duplicate_field("type_table"));
                            }
                            seen_type_table = true;
                            f_type_table = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "type_table_path" => {
                            if seen_type_table_path {
                                return Err(serde::de::Error::duplicate_field("type_table_path"));
                            }
                            seen_type_table_path = true;
                            f_type_table_path = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(WordDelimiterGraphTokenFilter {
                    version: f_version,
                    adjust_offsets: f_adjust_offsets,
                    catenate_all: f_catenate_all,
                    catenate_numbers: f_catenate_numbers,
                    catenate_words: f_catenate_words,
                    generate_number_parts: f_generate_number_parts,
                    generate_word_parts: f_generate_word_parts,
                    ignore_keywords: f_ignore_keywords,
                    preserve_original: f_preserve_original,
                    protected_words: f_protected_words,
                    protected_words_path: f_protected_words_path,
                    split_on_case_change: f_split_on_case_change,
                    split_on_numerics: f_split_on_numerics,
                    stem_english_possessive: f_stem_english_possessive,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    type_table: f_type_table,
                    type_table_path: f_type_table_path,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for WordDelimiterGraphTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.adjust_offsets {
            map.serialize_entry("adjust_offsets", v)?;
        }
        if let Some(v) = &self.catenate_all {
            map.serialize_entry("catenate_all", v)?;
        }
        if let Some(v) = &self.catenate_numbers {
            map.serialize_entry("catenate_numbers", v)?;
        }
        if let Some(v) = &self.catenate_words {
            map.serialize_entry("catenate_words", v)?;
        }
        if let Some(v) = &self.generate_number_parts {
            map.serialize_entry("generate_number_parts", v)?;
        }
        if let Some(v) = &self.generate_word_parts {
            map.serialize_entry("generate_word_parts", v)?;
        }
        if let Some(v) = &self.ignore_keywords {
            map.serialize_entry("ignore_keywords", v)?;
        }
        if let Some(v) = &self.preserve_original {
            map.serialize_entry("preserve_original", v)?;
        }
        if let Some(v) = &self.protected_words {
            map.serialize_entry("protected_words", v)?;
        }
        if let Some(v) = &self.protected_words_path {
            map.serialize_entry("protected_words_path", v)?;
        }
        if let Some(v) = &self.split_on_case_change {
            map.serialize_entry("split_on_case_change", v)?;
        }
        if let Some(v) = &self.split_on_numerics {
            map.serialize_entry("split_on_numerics", v)?;
        }
        if let Some(v) = &self.stem_english_possessive {
            map.serialize_entry("stem_english_possessive", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        if let Some(v) = &self.type_table {
            map.serialize_entry("type_table", v)?;
        }
        if let Some(v) = &self.type_table_path {
            map.serialize_entry("type_table_path", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WordDelimiterGraphTokenFilterType {
    #[doc = "`word_delimiter_graph`"]
    WordDelimiterGraph,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl WordDelimiterGraphTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            WordDelimiterGraphTokenFilterType::WordDelimiterGraph => "word_delimiter_graph",
            WordDelimiterGraphTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for WordDelimiterGraphTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for WordDelimiterGraphTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "word_delimiter_graph" => WordDelimiterGraphTokenFilterType::WordDelimiterGraph,
            _ => WordDelimiterGraphTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct WordDelimiterTokenFilter {
    pub version: Option<crate::types::common::VersionString>,
    pub catenate_all: Option<bool>,
    pub catenate_numbers: Option<bool>,
    pub catenate_words: Option<bool>,
    pub generate_number_parts: Option<bool>,
    pub generate_word_parts: Option<bool>,
    pub preserve_original: Option<crate::types::common::StringifiedBoolean>,
    pub protected_words: Option<Vec<String>>,
    pub protected_words_path: Option<String>,
    pub split_on_case_change: Option<bool>,
    pub split_on_numerics: Option<bool>,
    pub stem_english_possessive: Option<bool>,
    pub r#type: crate::types::common_analysis::WordDelimiterTokenFilterType,
    pub type_table: Option<Vec<String>>,
    pub type_table_path: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for WordDelimiterTokenFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = WordDelimiterTokenFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a WordDelimiterTokenFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_catenate_all: Option<bool> = None;
                let mut seen_catenate_all = false;
                let mut f_catenate_numbers: Option<bool> = None;
                let mut seen_catenate_numbers = false;
                let mut f_catenate_words: Option<bool> = None;
                let mut seen_catenate_words = false;
                let mut f_generate_number_parts: Option<bool> = None;
                let mut seen_generate_number_parts = false;
                let mut f_generate_word_parts: Option<bool> = None;
                let mut seen_generate_word_parts = false;
                let mut f_preserve_original: Option<crate::types::common::StringifiedBoolean> =
                    None;
                let mut seen_preserve_original = false;
                let mut f_protected_words: Option<Vec<String>> = None;
                let mut seen_protected_words = false;
                let mut f_protected_words_path: Option<String> = None;
                let mut seen_protected_words_path = false;
                let mut f_split_on_case_change: Option<bool> = None;
                let mut seen_split_on_case_change = false;
                let mut f_split_on_numerics: Option<bool> = None;
                let mut seen_split_on_numerics = false;
                let mut f_stem_english_possessive: Option<bool> = None;
                let mut seen_stem_english_possessive = false;
                let mut f_type: Option<
                    crate::types::common_analysis::WordDelimiterTokenFilterType,
                > = None;
                let mut f_type_table: Option<Vec<String>> = None;
                let mut seen_type_table = false;
                let mut f_type_table_path: Option<String> = None;
                let mut seen_type_table_path = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "catenate_all" => {
                            if seen_catenate_all {
                                return Err(serde::de::Error::duplicate_field("catenate_all"));
                            }
                            seen_catenate_all = true;
                            f_catenate_all = map.next_value::<Option<bool>>()?;
                        }
                        "catenate_numbers" => {
                            if seen_catenate_numbers {
                                return Err(serde::de::Error::duplicate_field("catenate_numbers"));
                            }
                            seen_catenate_numbers = true;
                            f_catenate_numbers = map.next_value::<Option<bool>>()?;
                        }
                        "catenate_words" => {
                            if seen_catenate_words {
                                return Err(serde::de::Error::duplicate_field("catenate_words"));
                            }
                            seen_catenate_words = true;
                            f_catenate_words = map.next_value::<Option<bool>>()?;
                        }
                        "generate_number_parts" => {
                            if seen_generate_number_parts {
                                return Err(serde::de::Error::duplicate_field(
                                    "generate_number_parts",
                                ));
                            }
                            seen_generate_number_parts = true;
                            f_generate_number_parts = map.next_value::<Option<bool>>()?;
                        }
                        "generate_word_parts" => {
                            if seen_generate_word_parts {
                                return Err(serde::de::Error::duplicate_field(
                                    "generate_word_parts",
                                ));
                            }
                            seen_generate_word_parts = true;
                            f_generate_word_parts = map.next_value::<Option<bool>>()?;
                        }
                        "preserve_original" => {
                            if seen_preserve_original {
                                return Err(serde::de::Error::duplicate_field("preserve_original"));
                            }
                            seen_preserve_original = true;
                            f_preserve_original = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "protected_words" => {
                            if seen_protected_words {
                                return Err(serde::de::Error::duplicate_field("protected_words"));
                            }
                            seen_protected_words = true;
                            f_protected_words = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "protected_words_path" => {
                            if seen_protected_words_path {
                                return Err(serde::de::Error::duplicate_field(
                                    "protected_words_path",
                                ));
                            }
                            seen_protected_words_path = true;
                            f_protected_words_path = map.next_value::<Option<String>>()?;
                        }
                        "split_on_case_change" => {
                            if seen_split_on_case_change {
                                return Err(serde::de::Error::duplicate_field(
                                    "split_on_case_change",
                                ));
                            }
                            seen_split_on_case_change = true;
                            f_split_on_case_change = map.next_value::<Option<bool>>()?;
                        }
                        "split_on_numerics" => {
                            if seen_split_on_numerics {
                                return Err(serde::de::Error::duplicate_field("split_on_numerics"));
                            }
                            seen_split_on_numerics = true;
                            f_split_on_numerics = map.next_value::<Option<bool>>()?;
                        }
                        "stem_english_possessive" => {
                            if seen_stem_english_possessive {
                                return Err(serde::de::Error::duplicate_field(
                                    "stem_english_possessive",
                                ));
                            }
                            seen_stem_english_possessive = true;
                            f_stem_english_possessive = map.next_value::<Option<bool>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_analysis::WordDelimiterTokenFilterType>()?);
                        }
                        "type_table" => {
                            if seen_type_table {
                                return Err(serde::de::Error::duplicate_field("type_table"));
                            }
                            seen_type_table = true;
                            f_type_table = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "type_table_path" => {
                            if seen_type_table_path {
                                return Err(serde::de::Error::duplicate_field("type_table_path"));
                            }
                            seen_type_table_path = true;
                            f_type_table_path = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(WordDelimiterTokenFilter {
                    version: f_version,
                    catenate_all: f_catenate_all,
                    catenate_numbers: f_catenate_numbers,
                    catenate_words: f_catenate_words,
                    generate_number_parts: f_generate_number_parts,
                    generate_word_parts: f_generate_word_parts,
                    preserve_original: f_preserve_original,
                    protected_words: f_protected_words,
                    protected_words_path: f_protected_words_path,
                    split_on_case_change: f_split_on_case_change,
                    split_on_numerics: f_split_on_numerics,
                    stem_english_possessive: f_stem_english_possessive,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    type_table: f_type_table,
                    type_table_path: f_type_table_path,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for WordDelimiterTokenFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.catenate_all {
            map.serialize_entry("catenate_all", v)?;
        }
        if let Some(v) = &self.catenate_numbers {
            map.serialize_entry("catenate_numbers", v)?;
        }
        if let Some(v) = &self.catenate_words {
            map.serialize_entry("catenate_words", v)?;
        }
        if let Some(v) = &self.generate_number_parts {
            map.serialize_entry("generate_number_parts", v)?;
        }
        if let Some(v) = &self.generate_word_parts {
            map.serialize_entry("generate_word_parts", v)?;
        }
        if let Some(v) = &self.preserve_original {
            map.serialize_entry("preserve_original", v)?;
        }
        if let Some(v) = &self.protected_words {
            map.serialize_entry("protected_words", v)?;
        }
        if let Some(v) = &self.protected_words_path {
            map.serialize_entry("protected_words_path", v)?;
        }
        if let Some(v) = &self.split_on_case_change {
            map.serialize_entry("split_on_case_change", v)?;
        }
        if let Some(v) = &self.split_on_numerics {
            map.serialize_entry("split_on_numerics", v)?;
        }
        if let Some(v) = &self.stem_english_possessive {
            map.serialize_entry("stem_english_possessive", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        if let Some(v) = &self.type_table {
            map.serialize_entry("type_table", v)?;
        }
        if let Some(v) = &self.type_table_path {
            map.serialize_entry("type_table_path", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WordDelimiterTokenFilterType {
    #[doc = "`word_delimiter`"]
    WordDelimiter,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl WordDelimiterTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            WordDelimiterTokenFilterType::WordDelimiter => "word_delimiter",
            WordDelimiterTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for WordDelimiterTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for WordDelimiterTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "word_delimiter" => WordDelimiterTokenFilterType::WordDelimiter,
            _ => WordDelimiterTokenFilterType::Unknown(s),
        })
    }
}
