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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AsciiFoldingTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_original: Option<crate::types::common::StringifiedBoolean>,
    pub r#type: crate::types::common_analysis::AsciiFoldingTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AsciiFoldingTokenFilterType {
    #[serde(rename = "asciifolding")]
    Asciifolding,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CharGroupTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_token_length: Option<i32>,
    pub tokenize_on_chars: Vec<String>,
    pub r#type: crate::types::common_analysis::CharGroupTokenizerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CharGroupTokenizerType {
    #[serde(rename = "char_group")]
    CharGroup,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CjkAnalyzer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::types::common_analysis::CjkAnalyzerType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CjkAnalyzerType {
    #[serde(rename = "cjk")]
    Cjk,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CommonGramsTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common_words: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common_words_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_case: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_mode: Option<bool>,
    pub r#type: crate::types::common_analysis::CommonGramsTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CommonGramsTokenFilterType {
    #[serde(rename = "common_grams")]
    CommonGrams,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConditionTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub filter: Vec<String>,
    pub script: crate::types::common::Script,
    pub r#type: crate::types::common_analysis::ConditionTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ConditionTokenFilterType {
    #[serde(rename = "condition")]
    Condition,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CustomAnalyzer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub char_filter: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_increment_gap: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_offset_gap: Option<i32>,
    pub tokenizer: String,
    pub r#type: crate::types::common_analysis::CustomAnalyzerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CustomAnalyzerType {
    #[serde(rename = "custom")]
    Custom,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CustomNormalizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub char_filter: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<String>>,
    pub r#type: crate::types::common_analysis::CustomNormalizerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CustomNormalizerType {
    #[serde(rename = "custom")]
    Custom,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DelimitedPayloadEncoding {
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "identity")]
    Identity,
    #[serde(rename = "int")]
    Int,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DelimitedPayloadTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<crate::types::common_analysis::DelimitedPayloadEncoding>,
    pub r#type: crate::types::common_analysis::DelimitedPayloadTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DelimitedPayloadTokenFilterType {
    #[serde(rename = "delimited_payload")]
    DelimitedPayload,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DictionaryDecompounderTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hyphenation_patterns_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_subword_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_subword_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_word_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub only_longest_match: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub word_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub word_list_path: Option<String>,
    pub r#type: crate::types::common_analysis::DictionaryDecompounderTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DictionaryDecompounderTokenFilterType {
    #[serde(rename = "dictionary_decompounder")]
    DictionaryDecompounder,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DutchAnalyzer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    pub r#type: crate::types::common_analysis::DutchAnalyzerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DutchAnalyzerType {
    #[serde(rename = "dutch")]
    Dutch,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EdgeNGramSide {
    #[serde(rename = "back")]
    Back,
    #[serde(rename = "front")]
    Front,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EdgeNGramTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_gram: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_gram: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_original: Option<crate::types::common::StringifiedBoolean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<crate::types::common_analysis::EdgeNGramSide>,
    pub r#type: crate::types::common_analysis::EdgeNGramTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EdgeNGramTokenFilterType {
    #[serde(rename = "edge_ngram")]
    EdgeNgram,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EdgeNGramTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_token_chars: Option<String>,
    pub max_gram: i32,
    pub min_gram: i32,
    pub token_chars: Vec<crate::types::common_analysis::TokenChar>,
    pub r#type: crate::types::common_analysis::EdgeNGramTokenizerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EdgeNGramTokenizerType {
    #[serde(rename = "edge_ngram")]
    EdgeNgram,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ElisionTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub articles: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub articles_case: Option<crate::types::common::StringifiedBoolean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub articles_path: Option<String>,
    pub r#type: crate::types::common_analysis::ElisionTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ElisionTokenFilterType {
    #[serde(rename = "elision")]
    Elision,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FingerprintAnalyzer {
    pub max_output_size: i32,
    pub preserve_original: bool,
    pub separator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords_path: Option<String>,
    pub r#type: crate::types::common_analysis::FingerprintAnalyzerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FingerprintAnalyzerType {
    #[serde(rename = "fingerprint")]
    Fingerprint,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FingerprintTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_output_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
    pub r#type: crate::types::common_analysis::FingerprintTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FingerprintTokenFilterType {
    #[serde(rename = "fingerprint")]
    Fingerprint,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HtmlStripCharFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::HtmlStripCharFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum HtmlStripCharFilterType {
    #[serde(rename = "html_strip")]
    HtmlStrip,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HunspellTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dedup: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dictionary: Option<String>,
    pub locale: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longest_only: Option<bool>,
    pub r#type: crate::types::common_analysis::HunspellTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum HunspellTokenFilterType {
    #[serde(rename = "hunspell")]
    Hunspell,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HyphenationDecompounderTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hyphenation_patterns_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_subword_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_subword_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_word_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub only_longest_match: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub word_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub word_list_path: Option<String>,
    pub r#type: crate::types::common_analysis::HyphenationDecompounderTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum HyphenationDecompounderTokenFilterType {
    #[serde(rename = "hyphenation_decompounder")]
    HyphenationDecompounder,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IcuAnalyzer {
    pub method: crate::types::common_analysis::IcuNormalizationType,
    pub mode: crate::types::common_analysis::IcuNormalizationMode,
    pub r#type: crate::types::common_analysis::IcuAnalyzerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IcuAnalyzerType {
    #[serde(rename = "icu_analyzer")]
    IcuAnalyzer,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IcuCollationAlternate {
    #[serde(rename = "non-ignorable")]
    NonIgnorable,
    #[serde(rename = "shifted")]
    Shifted,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IcuCollationCaseFirst {
    #[serde(rename = "lower")]
    Lower,
    #[serde(rename = "upper")]
    Upper,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IcuCollationDecomposition {
    #[serde(rename = "canonical")]
    Canonical,
    #[serde(rename = "no")]
    No,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IcuCollationStrength {
    #[serde(rename = "identical")]
    Identical,
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "quaternary")]
    Quaternary,
    #[serde(rename = "secondary")]
    Secondary,
    #[serde(rename = "tertiary")]
    Tertiary,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IcuCollationTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alternate: Option<crate::types::common_analysis::IcuCollationAlternate>,
    #[serde(rename = "caseFirst")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub case_first: Option<crate::types::common_analysis::IcuCollationCaseFirst>,
    #[serde(rename = "caseLevel")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub case_level: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decomposition: Option<crate::types::common_analysis::IcuCollationDecomposition>,
    #[serde(rename = "hiraganaQuaternaryMode")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hiragana_quaternary_mode: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numeric: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strength: Option<crate::types::common_analysis::IcuCollationStrength>,
    pub r#type: crate::types::common_analysis::IcuCollationTokenFilterType,
    #[serde(rename = "variableTop")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable_top: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IcuCollationTokenFilterType {
    #[serde(rename = "icu_collation")]
    IcuCollation,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IcuFoldingTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::IcuFoldingTokenFilterType,
    pub unicode_set_filter: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IcuFoldingTokenFilterType {
    #[serde(rename = "icu_folding")]
    IcuFolding,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IcuNormalizationCharFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<crate::types::common_analysis::IcuNormalizationMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<crate::types::common_analysis::IcuNormalizationType>,
    pub r#type: crate::types::common_analysis::IcuNormalizationCharFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IcuNormalizationCharFilterType {
    #[serde(rename = "icu_normalizer")]
    IcuNormalizer,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IcuNormalizationMode {
    #[serde(rename = "compose")]
    Compose,
    #[serde(rename = "decompose")]
    Decompose,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IcuNormalizationTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub name: crate::types::common_analysis::IcuNormalizationType,
    pub r#type: crate::types::common_analysis::IcuNormalizationTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IcuNormalizationTokenFilterType {
    #[serde(rename = "icu_normalizer")]
    IcuNormalizer,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IcuNormalizationType {
    #[serde(rename = "nfc")]
    Nfc,
    #[serde(rename = "nfkc")]
    Nfkc,
    #[serde(rename = "nfkc_cf")]
    NfkcCf,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IcuTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub rule_files: String,
    pub r#type: crate::types::common_analysis::IcuTokenizerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IcuTokenizerType {
    #[serde(rename = "icu_tokenizer")]
    IcuTokenizer,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IcuTransformDirection {
    #[serde(rename = "forward")]
    Forward,
    #[serde(rename = "reverse")]
    Reverse,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IcuTransformTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dir: Option<crate::types::common_analysis::IcuTransformDirection>,
    pub id: String,
    pub r#type: crate::types::common_analysis::IcuTransformTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IcuTransformTokenFilterType {
    #[serde(rename = "icu_transform")]
    IcuTransform,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KStemTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::KStemTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KStemTokenFilterType {
    #[serde(rename = "kstem")]
    Kstem,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KeepTypesMode {
    #[serde(rename = "exclude")]
    Exclude,
    #[serde(rename = "include")]
    Include,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeepTypesTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<crate::types::common_analysis::KeepTypesMode>,
    pub r#type: crate::types::common_analysis::KeepTypesTokenFilterType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KeepTypesTokenFilterType {
    #[serde(rename = "keep_types")]
    KeepTypes,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeepWordsTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keep_words: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keep_words_case: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keep_words_path: Option<String>,
    pub r#type: crate::types::common_analysis::KeepWordsTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KeepWordsTokenFilterType {
    #[serde(rename = "keep")]
    Keep,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeywordAnalyzer {
    pub r#type: crate::types::common_analysis::KeywordAnalyzerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KeywordAnalyzerType {
    #[serde(rename = "keyword")]
    Keyword,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeywordMarkerTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_case: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords_pattern: Option<String>,
    pub r#type: crate::types::common_analysis::KeywordMarkerTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KeywordMarkerTokenFilterType {
    #[serde(rename = "keyword_marker")]
    KeywordMarker,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeywordTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub buffer_size: i32,
    pub r#type: crate::types::common_analysis::KeywordTokenizerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KeywordTokenizerType {
    #[serde(rename = "keyword")]
    Keyword,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KuromojiAnalyzer {
    pub mode: crate::types::common_analysis::KuromojiTokenizationMode,
    pub r#type: crate::types::common_analysis::KuromojiAnalyzerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_dictionary: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KuromojiAnalyzerType {
    #[serde(rename = "kuromoji")]
    Kuromoji,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KuromojiIterationMarkCharFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub normalize_kana: bool,
    pub normalize_kanji: bool,
    pub r#type: crate::types::common_analysis::KuromojiIterationMarkCharFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KuromojiIterationMarkCharFilterType {
    #[serde(rename = "kuromoji_iteration_mark")]
    KuromojiIterationMark,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KuromojiPartOfSpeechTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub stoptags: Vec<String>,
    pub r#type: crate::types::common_analysis::KuromojiPartOfSpeechTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KuromojiPartOfSpeechTokenFilterType {
    #[serde(rename = "kuromoji_part_of_speech")]
    KuromojiPartOfSpeech,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KuromojiReadingFormTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::KuromojiReadingFormTokenFilterType,
    pub use_romaji: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KuromojiReadingFormTokenFilterType {
    #[serde(rename = "kuromoji_readingform")]
    KuromojiReadingform,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KuromojiStemmerTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub minimum_length: i32,
    pub r#type: crate::types::common_analysis::KuromojiStemmerTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KuromojiStemmerTokenFilterType {
    #[serde(rename = "kuromoji_stemmer")]
    KuromojiStemmer,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KuromojiTokenizationMode {
    #[serde(rename = "extended")]
    Extended,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "search")]
    Search,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KuromojiTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discard_compound_token: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discard_punctuation: Option<bool>,
    pub mode: crate::types::common_analysis::KuromojiTokenizationMode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nbest_cost: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nbest_examples: Option<String>,
    pub r#type: crate::types::common_analysis::KuromojiTokenizerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_dictionary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_dictionary_rules: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KuromojiTokenizerType {
    #[serde(rename = "kuromoji_tokenizer")]
    KuromojiTokenizer,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Language {
    #[serde(rename = "Arabic")]
    Arabic,
    #[serde(rename = "Armenian")]
    Armenian,
    #[serde(rename = "Basque")]
    Basque,
    #[serde(rename = "Brazilian")]
    Brazilian,
    #[serde(rename = "Bulgarian")]
    Bulgarian,
    #[serde(rename = "Catalan")]
    Catalan,
    #[serde(rename = "Chinese")]
    Chinese,
    #[serde(rename = "Cjk")]
    Cjk,
    #[serde(rename = "Czech")]
    Czech,
    #[serde(rename = "Danish")]
    Danish,
    #[serde(rename = "Dutch")]
    Dutch,
    #[serde(rename = "English")]
    English,
    #[serde(rename = "Estonian")]
    Estonian,
    #[serde(rename = "Finnish")]
    Finnish,
    #[serde(rename = "French")]
    French,
    #[serde(rename = "Galician")]
    Galician,
    #[serde(rename = "German")]
    German,
    #[serde(rename = "Greek")]
    Greek,
    #[serde(rename = "Hindi")]
    Hindi,
    #[serde(rename = "Hungarian")]
    Hungarian,
    #[serde(rename = "Indonesian")]
    Indonesian,
    #[serde(rename = "Irish")]
    Irish,
    #[serde(rename = "Italian")]
    Italian,
    #[serde(rename = "Latvian")]
    Latvian,
    #[serde(rename = "Norwegian")]
    Norwegian,
    #[serde(rename = "Persian")]
    Persian,
    #[serde(rename = "Portuguese")]
    Portuguese,
    #[serde(rename = "Romanian")]
    Romanian,
    #[serde(rename = "Russian")]
    Russian,
    #[serde(rename = "Sorani")]
    Sorani,
    #[serde(rename = "Spanish")]
    Spanish,
    #[serde(rename = "Swedish")]
    Swedish,
    #[serde(rename = "Thai")]
    Thai,
    #[serde(rename = "Turkish")]
    Turkish,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LanguageAnalyzer {
    pub language: crate::types::common_analysis::Language,
    pub stem_exclusion: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords_path: Option<String>,
    pub r#type: crate::types::common_analysis::LanguageAnalyzerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LanguageAnalyzerType {
    #[serde(rename = "language")]
    Language,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LengthTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
    pub r#type: crate::types::common_analysis::LengthTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LengthTokenFilterType {
    #[serde(rename = "length")]
    Length,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LetterTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::LetterTokenizerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LetterTokenizerType {
    #[serde(rename = "letter")]
    Letter,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LimitTokenCountTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consume_all_tokens: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_token_count: Option<crate::types::common::StringifiedInteger>,
    pub r#type: crate::types::common_analysis::LimitTokenCountTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LimitTokenCountTokenFilterType {
    #[serde(rename = "limit")]
    Limit,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LowercaseNormalizer {
    pub r#type: crate::types::common_analysis::LowercaseNormalizerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LowercaseNormalizerType {
    #[serde(rename = "lowercase")]
    Lowercase,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LowercaseTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    pub r#type: crate::types::common_analysis::LowercaseTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LowercaseTokenFilterType {
    #[serde(rename = "lowercase")]
    Lowercase,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LowercaseTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::LowercaseTokenizerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LowercaseTokenizerType {
    #[serde(rename = "lowercase")]
    Lowercase,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MappingCharFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mappings: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mappings_path: Option<String>,
    pub r#type: crate::types::common_analysis::MappingCharFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MappingCharFilterType {
    #[serde(rename = "mapping")]
    Mapping,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MultiplexerTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub filters: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_original: Option<crate::types::common::StringifiedBoolean>,
    pub r#type: crate::types::common_analysis::MultiplexerTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MultiplexerTokenFilterType {
    #[serde(rename = "multiplexer")]
    Multiplexer,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NGramTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_gram: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_gram: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_original: Option<crate::types::common::StringifiedBoolean>,
    pub r#type: crate::types::common_analysis::NGramTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NGramTokenFilterType {
    #[serde(rename = "ngram")]
    Ngram,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NGramTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_token_chars: Option<String>,
    pub max_gram: i32,
    pub min_gram: i32,
    pub token_chars: Vec<crate::types::common_analysis::TokenChar>,
    pub r#type: crate::types::common_analysis::NGramTokenizerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NGramTokenizerType {
    #[serde(rename = "ngram")]
    Ngram,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NoriAnalyzer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decompound_mode: Option<crate::types::common_analysis::NoriDecompoundMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stoptags: Option<Vec<String>>,
    pub r#type: crate::types::common_analysis::NoriAnalyzerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_dictionary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NoriAnalyzerType {
    #[serde(rename = "nori")]
    Nori,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NoriDecompoundMode {
    #[serde(rename = "discard")]
    Discard,
    #[serde(rename = "mixed")]
    Mixed,
    #[serde(rename = "none")]
    None,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NoriPartOfSpeechTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stoptags: Option<Vec<String>>,
    pub r#type: crate::types::common_analysis::NoriPartOfSpeechTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NoriPartOfSpeechTokenFilterType {
    #[serde(rename = "nori_part_of_speech")]
    NoriPartOfSpeech,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NoriTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decompound_mode: Option<crate::types::common_analysis::NoriDecompoundMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discard_punctuation: Option<bool>,
    pub r#type: crate::types::common_analysis::NoriTokenizerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_dictionary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_dictionary_rules: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NoriTokenizerType {
    #[serde(rename = "nori_tokenizer")]
    NoriTokenizer,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PathHierarchyTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub buffer_size: crate::types::common::StringifiedInteger,
    pub delimiter: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,
    pub reverse: crate::types::common::StringifiedBoolean,
    pub skip: crate::types::common::StringifiedInteger,
    pub r#type: crate::types::common_analysis::PathHierarchyTokenizerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PathHierarchyTokenizerType {
    #[serde(rename = "path_hierarchy")]
    PathHierarchy,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PatternAnalyzer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lowercase: Option<bool>,
    pub pattern: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    pub r#type: crate::types::common_analysis::PatternAnalyzerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PatternAnalyzerType {
    #[serde(rename = "pattern")]
    Pattern,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PatternCaptureTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub patterns: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_original: Option<crate::types::common::StringifiedBoolean>,
    pub r#type: crate::types::common_analysis::PatternCaptureTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PatternCaptureTokenFilterType {
    #[serde(rename = "pattern_capture")]
    PatternCapture,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PatternReplaceCharFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
    pub pattern: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,
    pub r#type: crate::types::common_analysis::PatternReplaceCharFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PatternReplaceCharFilterType {
    #[serde(rename = "pattern_replace")]
    PatternReplace,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PatternReplaceTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
    pub pattern: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,
    pub r#type: crate::types::common_analysis::PatternReplaceTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PatternReplaceTokenFilterType {
    #[serde(rename = "pattern_replace")]
    PatternReplace,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PatternTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    pub r#type: crate::types::common_analysis::PatternTokenizerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PatternTokenizerType {
    #[serde(rename = "pattern")]
    Pattern,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PersianStemTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::PersianStemTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PersianStemTokenFilterType {
    #[serde(rename = "persian_stem")]
    PersianStem,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhoneAnalyzer {
    #[doc = "Optional ISO 3166 country code, defaults to \"ZZ\" (unknown region)."]
    #[serde(rename = "phone-region")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_region: Option<String>,
    pub r#type: crate::types::common_analysis::PhoneAnalyzerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PhoneAnalyzerType {
    #[serde(rename = "phone")]
    Phone,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhoneSearchAnalyzer {
    #[doc = "Optional ISO 3166 country code, defaults to \"ZZ\" (unknown region)."]
    #[serde(rename = "phone-region")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_region: Option<String>,
    pub r#type: crate::types::common_analysis::PhoneSearchAnalyzerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PhoneSearchAnalyzerType {
    #[serde(rename = "phone-search")]
    PhoneSearch,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PhoneticEncoder {
    #[serde(rename = "beider_morse")]
    BeiderMorse,
    #[serde(rename = "caverphone1")]
    Caverphone1,
    #[serde(rename = "caverphone2")]
    Caverphone2,
    #[serde(rename = "cologne")]
    Cologne,
    #[serde(rename = "daitch_mokotoff")]
    DaitchMokotoff,
    #[serde(rename = "double_metaphone")]
    DoubleMetaphone,
    #[serde(rename = "haasephonetik")]
    Haasephonetik,
    #[serde(rename = "koelnerphonetik")]
    Koelnerphonetik,
    #[serde(rename = "metaphone")]
    Metaphone,
    #[serde(rename = "nysiis")]
    Nysiis,
    #[serde(rename = "refined_soundex")]
    RefinedSoundex,
    #[serde(rename = "soundex")]
    Soundex,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PhoneticLanguage {
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "common")]
    Common,
    #[serde(rename = "cyrillic")]
    Cyrillic,
    #[serde(rename = "english")]
    English,
    #[serde(rename = "french")]
    French,
    #[serde(rename = "german")]
    German,
    #[serde(rename = "hebrew")]
    Hebrew,
    #[serde(rename = "hungarian")]
    Hungarian,
    #[serde(rename = "polish")]
    Polish,
    #[serde(rename = "romanian")]
    Romanian,
    #[serde(rename = "russian")]
    Russian,
    #[serde(rename = "spanish")]
    Spanish,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PhoneticNameType {
    #[serde(rename = "ashkenazi")]
    Ashkenazi,
    #[serde(rename = "generic")]
    Generic,
    #[serde(rename = "sephardic")]
    Sephardic,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PhoneticRuleType {
    #[serde(rename = "approx")]
    Approx,
    #[serde(rename = "exact")]
    Exact,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhoneticTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub encoder: crate::types::common_analysis::PhoneticEncoder,
    pub languageset: Vec<crate::types::common_analysis::PhoneticLanguage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_code_len: Option<i32>,
    pub name_type: crate::types::common_analysis::PhoneticNameType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
    pub rule_type: crate::types::common_analysis::PhoneticRuleType,
    pub r#type: crate::types::common_analysis::PhoneticTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PhoneticTokenFilterType {
    #[serde(rename = "phonetic")]
    Phonetic,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PorterStemTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::PorterStemTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PorterStemTokenFilterType {
    #[serde(rename = "porter_stem")]
    PorterStem,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PredicateTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub script: crate::types::common::Script,
    pub r#type: crate::types::common_analysis::PredicateTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PredicateTokenFilterType {
    #[serde(rename = "predicate_token_filter")]
    PredicateTokenFilter,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoveDuplicatesTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::RemoveDuplicatesTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RemoveDuplicatesTokenFilterType {
    #[serde(rename = "remove_duplicates")]
    RemoveDuplicates,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReverseTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::ReverseTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ReverseTokenFilterType {
    #[serde(rename = "reverse")]
    Reverse,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShingleTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filler_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_shingle_size: Option<crate::types::common::StringifiedInteger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_shingle_size: Option<crate::types::common::StringifiedInteger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_unigrams: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_unigrams_if_no_shingles: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_separator: Option<String>,
    pub r#type: crate::types::common_analysis::ShingleTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ShingleTokenFilterType {
    #[serde(rename = "shingle")]
    Shingle,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimpleAnalyzer {
    pub r#type: crate::types::common_analysis::SimpleAnalyzerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SimpleAnalyzerType {
    #[serde(rename = "simple")]
    Simple,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimplePatternSplitTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    pub r#type: crate::types::common_analysis::SimplePatternSplitTokenizerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SimplePatternSplitTokenizerType {
    #[serde(rename = "simple_pattern_split")]
    SimplePatternSplit,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimplePatternTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    pub r#type: crate::types::common_analysis::SimplePatternTokenizerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SimplePatternTokenizerType {
    #[serde(rename = "simple_pattern")]
    SimplePattern,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SmartcnAnalyzer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::types::common_analysis::SmartcnAnalyzerType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SmartcnAnalyzerType {
    #[serde(rename = "smartcn")]
    Smartcn,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SmartcnStopTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::SmartcnStopTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SmartcnStopTokenFilterType {
    #[serde(rename = "smartcn_stop")]
    SmartcnStop,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SmartcnTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::SmartcnTokenizerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SmartcnTokenizerType {
    #[serde(rename = "smartcn_tokenizer")]
    SmartcnTokenizer,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SnowballAnalyzer {
    pub language: crate::types::common_analysis::SnowballLanguage,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    pub r#type: crate::types::common_analysis::SnowballAnalyzerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SnowballAnalyzerType {
    #[serde(rename = "snowball")]
    Snowball,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SnowballLanguage {
    #[serde(rename = "Armenian")]
    Armenian,
    #[serde(rename = "Basque")]
    Basque,
    #[serde(rename = "Catalan")]
    Catalan,
    #[serde(rename = "Danish")]
    Danish,
    #[serde(rename = "Dutch")]
    Dutch,
    #[serde(rename = "English")]
    English,
    #[serde(rename = "Finnish")]
    Finnish,
    #[serde(rename = "French")]
    French,
    #[serde(rename = "German")]
    German,
    #[serde(rename = "German2")]
    German2,
    #[serde(rename = "Hungarian")]
    Hungarian,
    #[serde(rename = "Italian")]
    Italian,
    #[serde(rename = "Kp")]
    Kp,
    #[serde(rename = "Lovins")]
    Lovins,
    #[serde(rename = "Norwegian")]
    Norwegian,
    #[serde(rename = "Porter")]
    Porter,
    #[serde(rename = "Portuguese")]
    Portuguese,
    #[serde(rename = "Romanian")]
    Romanian,
    #[serde(rename = "Russian")]
    Russian,
    #[serde(rename = "Spanish")]
    Spanish,
    #[serde(rename = "Swedish")]
    Swedish,
    #[serde(rename = "Turkish")]
    Turkish,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SnowballTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub language: crate::types::common_analysis::SnowballLanguage,
    pub r#type: crate::types::common_analysis::SnowballTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SnowballTokenFilterType {
    #[serde(rename = "snowball")]
    Snowball,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StandardAnalyzer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_token_length: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    pub r#type: crate::types::common_analysis::StandardAnalyzerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StandardAnalyzerType {
    #[serde(rename = "standard")]
    Standard,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StandardTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_token_length: Option<i32>,
    pub r#type: crate::types::common_analysis::StandardTokenizerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StandardTokenizerType {
    #[serde(rename = "standard")]
    Standard,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StemmerOverrideTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules_path: Option<String>,
    pub r#type: crate::types::common_analysis::StemmerOverrideTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StemmerOverrideTokenFilterType {
    #[serde(rename = "stemmer_override")]
    StemmerOverride,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StemmerTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    pub r#type: crate::types::common_analysis::StemmerTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StemmerTokenFilterType {
    #[serde(rename = "stemmer")]
    Stemmer,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StopAnalyzer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords_path: Option<String>,
    pub r#type: crate::types::common_analysis::StopAnalyzerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StopAnalyzerType {
    #[serde(rename = "stop")]
    Stop,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StopTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_case: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_trailing: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords_path: Option<String>,
    pub r#type: crate::types::common_analysis::StopTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StopTokenFilterType {
    #[serde(rename = "stop")]
    Stop,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[doc = "Language value, such as `arabic` or `thai`. Defaults to `english`. Each language value corresponds to a predefined list of stop words in Lucene. See Stop words by language for supported language values and their stop words. Also accepts an array of stop words."]
pub type StopWords = crate::types::common::StringOrStringArray;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SynonymFormat {
    #[serde(rename = "solr")]
    Solr,
    #[serde(rename = "wordnet")]
    Wordnet,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SynonymGraphTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<crate::types::common_analysis::SynonymFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lenient: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synonyms_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tokenizer: Option<String>,
    pub r#type: crate::types::common_analysis::SynonymGraphTokenFilterType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updateable: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SynonymGraphTokenFilterType {
    #[serde(rename = "synonym_graph")]
    SynonymGraph,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SynonymTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<crate::types::common_analysis::SynonymFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lenient: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synonyms_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tokenizer: Option<String>,
    pub r#type: crate::types::common_analysis::SynonymTokenFilterType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updateable: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SynonymTokenFilterType {
    #[serde(rename = "synonym")]
    Synonym,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TokenChar {
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "digit")]
    Digit,
    #[serde(rename = "letter")]
    Letter,
    #[serde(rename = "punctuation")]
    Punctuation,
    #[serde(rename = "symbol")]
    Symbol,
    #[serde(rename = "whitespace")]
    Whitespace,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TrimTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::TrimTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TrimTokenFilterType {
    #[serde(rename = "trim")]
    Trim,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TruncateTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    pub r#type: crate::types::common_analysis::TruncateTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TruncateTokenFilterType {
    #[serde(rename = "truncate")]
    Truncate,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UaxEmailUrlTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_token_length: Option<i32>,
    pub r#type: crate::types::common_analysis::UaxEmailUrlTokenizerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UaxEmailUrlTokenizerType {
    #[serde(rename = "uax_url_email")]
    UaxUrlEmail,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UniqueTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub only_on_same_position: Option<bool>,
    pub r#type: crate::types::common_analysis::UniqueTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UniqueTokenFilterType {
    #[serde(rename = "unique")]
    Unique,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UppercaseTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::UppercaseTokenFilterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UppercaseTokenFilterType {
    #[serde(rename = "uppercase")]
    Uppercase,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WhitespaceAnalyzer {
    pub r#type: crate::types::common_analysis::WhitespaceAnalyzerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum WhitespaceAnalyzerType {
    #[serde(rename = "whitespace")]
    Whitespace,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WhitespaceTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_token_length: Option<i32>,
    pub r#type: crate::types::common_analysis::WhitespaceTokenizerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum WhitespaceTokenizerType {
    #[serde(rename = "whitespace")]
    Whitespace,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WordDelimiterGraphTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adjust_offsets: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catenate_all: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catenate_numbers: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catenate_words: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generate_number_parts: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generate_word_parts: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_keywords: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_original: Option<crate::types::common::StringifiedBoolean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protected_words: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protected_words_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split_on_case_change: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split_on_numerics: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stem_english_possessive: Option<bool>,
    pub r#type: crate::types::common_analysis::WordDelimiterGraphTokenFilterType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_table: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_table_path: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum WordDelimiterGraphTokenFilterType {
    #[serde(rename = "word_delimiter_graph")]
    WordDelimiterGraph,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WordDelimiterTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catenate_all: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catenate_numbers: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catenate_words: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generate_number_parts: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generate_word_parts: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_original: Option<crate::types::common::StringifiedBoolean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protected_words: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protected_words_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split_on_case_change: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split_on_numerics: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stem_english_possessive: Option<bool>,
    pub r#type: crate::types::common_analysis::WordDelimiterTokenFilterType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_table: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_table_path: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum WordDelimiterTokenFilterType {
    #[serde(rename = "word_delimiter")]
    WordDelimiter,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}
