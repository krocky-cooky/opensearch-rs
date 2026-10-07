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

//! Response types of the `cluster_stats` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct CharFilterTypes {
    #[doc = "Contains statistics about analyzer types used in selected nodes."]
    pub analyzer_types: Vec<crate::types::cluster_stats::FieldTypes>,
    #[doc = "Contains statistics about built-in analyzers used in selected nodes."]
    pub built_in_analyzers: Vec<crate::types::cluster_stats::FieldTypes>,
    #[doc = "Contains statistics about built-in character filters used in selected nodes."]
    pub built_in_char_filters: Vec<crate::types::cluster_stats::FieldTypes>,
    #[doc = "Contains statistics about built-in token filters used in selected nodes."]
    pub built_in_filters: Vec<crate::types::cluster_stats::FieldTypes>,
    #[doc = "Contains statistics about built-in tokenizers used in selected nodes."]
    pub built_in_tokenizers: Vec<crate::types::cluster_stats::FieldTypes>,
    #[doc = "Contains statistics about character filter types used in selected nodes."]
    pub char_filter_types: Vec<crate::types::cluster_stats::FieldTypes>,
    #[doc = "Contains statistics about token filter types used in selected nodes."]
    pub filter_types: Vec<crate::types::cluster_stats::FieldTypes>,
    #[doc = "Contains statistics about tokenizer types used in selected nodes."]
    pub tokenizer_types: Vec<crate::types::cluster_stats::FieldTypes>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CharFilterTypes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CharFilterTypes;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CharFilterTypes object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_analyzer_types: Option<Vec<crate::types::cluster_stats::FieldTypes>> =
                    None;
                let mut f_built_in_analyzers: Option<Vec<crate::types::cluster_stats::FieldTypes>> =
                    None;
                let mut f_built_in_char_filters: Option<
                    Vec<crate::types::cluster_stats::FieldTypes>,
                > = None;
                let mut f_built_in_filters: Option<Vec<crate::types::cluster_stats::FieldTypes>> =
                    None;
                let mut f_built_in_tokenizers: Option<
                    Vec<crate::types::cluster_stats::FieldTypes>,
                > = None;
                let mut f_char_filter_types: Option<Vec<crate::types::cluster_stats::FieldTypes>> =
                    None;
                let mut f_filter_types: Option<Vec<crate::types::cluster_stats::FieldTypes>> = None;
                let mut f_tokenizer_types: Option<Vec<crate::types::cluster_stats::FieldTypes>> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "analyzer_types" => {
                            if f_analyzer_types.is_some() {
                                return Err(serde::de::Error::duplicate_field("analyzer_types"));
                            }
                            f_analyzer_types = Some(
                                map.next_value::<Vec<crate::types::cluster_stats::FieldTypes>>()?,
                            );
                        }
                        "built_in_analyzers" => {
                            if f_built_in_analyzers.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "built_in_analyzers",
                                ));
                            }
                            f_built_in_analyzers = Some(
                                map.next_value::<Vec<crate::types::cluster_stats::FieldTypes>>()?,
                            );
                        }
                        "built_in_char_filters" => {
                            if f_built_in_char_filters.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "built_in_char_filters",
                                ));
                            }
                            f_built_in_char_filters = Some(
                                map.next_value::<Vec<crate::types::cluster_stats::FieldTypes>>()?,
                            );
                        }
                        "built_in_filters" => {
                            if f_built_in_filters.is_some() {
                                return Err(serde::de::Error::duplicate_field("built_in_filters"));
                            }
                            f_built_in_filters = Some(
                                map.next_value::<Vec<crate::types::cluster_stats::FieldTypes>>()?,
                            );
                        }
                        "built_in_tokenizers" => {
                            if f_built_in_tokenizers.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "built_in_tokenizers",
                                ));
                            }
                            f_built_in_tokenizers = Some(
                                map.next_value::<Vec<crate::types::cluster_stats::FieldTypes>>()?,
                            );
                        }
                        "char_filter_types" => {
                            if f_char_filter_types.is_some() {
                                return Err(serde::de::Error::duplicate_field("char_filter_types"));
                            }
                            f_char_filter_types = Some(
                                map.next_value::<Vec<crate::types::cluster_stats::FieldTypes>>()?,
                            );
                        }
                        "filter_types" => {
                            if f_filter_types.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter_types"));
                            }
                            f_filter_types = Some(
                                map.next_value::<Vec<crate::types::cluster_stats::FieldTypes>>()?,
                            );
                        }
                        "tokenizer_types" => {
                            if f_tokenizer_types.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenizer_types"));
                            }
                            f_tokenizer_types = Some(
                                map.next_value::<Vec<crate::types::cluster_stats::FieldTypes>>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CharFilterTypes {
                    analyzer_types: f_analyzer_types
                        .ok_or_else(|| serde::de::Error::missing_field("analyzer_types"))?,
                    built_in_analyzers: f_built_in_analyzers
                        .ok_or_else(|| serde::de::Error::missing_field("built_in_analyzers"))?,
                    built_in_char_filters: f_built_in_char_filters
                        .ok_or_else(|| serde::de::Error::missing_field("built_in_char_filters"))?,
                    built_in_filters: f_built_in_filters
                        .ok_or_else(|| serde::de::Error::missing_field("built_in_filters"))?,
                    built_in_tokenizers: f_built_in_tokenizers
                        .ok_or_else(|| serde::de::Error::missing_field("built_in_tokenizers"))?,
                    char_filter_types: f_char_filter_types
                        .ok_or_else(|| serde::de::Error::missing_field("char_filter_types"))?,
                    filter_types: f_filter_types
                        .ok_or_else(|| serde::de::Error::missing_field("filter_types"))?,
                    tokenizer_types: f_tokenizer_types
                        .ok_or_else(|| serde::de::Error::missing_field("tokenizer_types"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CharFilterTypes {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("analyzer_types", &self.analyzer_types)?;
        map.serialize_entry("built_in_analyzers", &self.built_in_analyzers)?;
        map.serialize_entry("built_in_char_filters", &self.built_in_char_filters)?;
        map.serialize_entry("built_in_filters", &self.built_in_filters)?;
        map.serialize_entry("built_in_tokenizers", &self.built_in_tokenizers)?;
        map.serialize_entry("char_filter_types", &self.char_filter_types)?;
        map.serialize_entry("filter_types", &self.filter_types)?;
        map.serialize_entry("tokenizer_types", &self.tokenizer_types)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ClusterFileSystem {
    #[doc = "Total number of bytes available to JVM in file stores across all selected nodes. Depending on operating system or process-level restrictions, this number may be less than `nodes.fs.free_in_byes`. This is the actual amount of free disk space the selected OpenSearch nodes can use."]
    pub available: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total number of bytes available to JVM in file stores across all selected nodes. Depending on operating system or process-level restrictions, this number may be less than `nodes.fs.free_in_byes`. This is the actual amount of free disk space the selected OpenSearch nodes can use."]
    pub available_in_bytes: crate::types::common::ByteCount,
    #[doc = "Total size, of all cache reserved across all selected nodes."]
    pub cache_reserved: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total size, in bytes, of all cache reserved across all selected nodes."]
    pub cache_reserved_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Total number of unallocated bytes in file stores across all selected nodes."]
    pub free: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total number of unallocated bytes in file stores across all selected nodes."]
    pub free_in_bytes: crate::types::common::ByteCount,
    #[doc = "Total size, of all file stores across all selected nodes."]
    pub total: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total size, in bytes, of all file stores across all selected nodes."]
    pub total_in_bytes: crate::types::common::ByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterFileSystem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterFileSystem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterFileSystem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_available: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_available = false;
                let mut f_available_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_cache_reserved: Option<crate::types::common::HumanReadableByteCount> =
                    None;
                let mut seen_cache_reserved = false;
                let mut f_cache_reserved_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_cache_reserved_in_bytes = false;
                let mut f_free: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_free = false;
                let mut f_free_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_total: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_total = false;
                let mut f_total_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "available" => {
                            if seen_available {
                                return Err(serde::de::Error::duplicate_field("available"));
                            }
                            seen_available = true;
                            f_available = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "available_in_bytes" => {
                            if f_available_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "available_in_bytes",
                                ));
                            }
                            f_available_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "cache_reserved" => {
                            if seen_cache_reserved {
                                return Err(serde::de::Error::duplicate_field("cache_reserved"));
                            }
                            seen_cache_reserved = true;
                            f_cache_reserved = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "cache_reserved_in_bytes" => {
                            if seen_cache_reserved_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "cache_reserved_in_bytes",
                                ));
                            }
                            seen_cache_reserved_in_bytes = true;
                            f_cache_reserved_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "free" => {
                            if seen_free {
                                return Err(serde::de::Error::duplicate_field("free"));
                            }
                            seen_free = true;
                            f_free = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "free_in_bytes" => {
                            if f_free_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("free_in_bytes"));
                            }
                            f_free_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "total_in_bytes" => {
                            if f_total_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("total_in_bytes"));
                            }
                            f_total_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterFileSystem {
                    available: f_available,
                    available_in_bytes: f_available_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("available_in_bytes"))?,
                    cache_reserved: f_cache_reserved,
                    cache_reserved_in_bytes: f_cache_reserved_in_bytes,
                    free: f_free,
                    free_in_bytes: f_free_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("free_in_bytes"))?,
                    total: f_total,
                    total_in_bytes: f_total_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("total_in_bytes"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterFileSystem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.available {
            map.serialize_entry("available", v)?;
        }
        map.serialize_entry("available_in_bytes", &self.available_in_bytes)?;
        if let Some(v) = &self.cache_reserved {
            map.serialize_entry("cache_reserved", v)?;
        }
        if let Some(v) = &self.cache_reserved_in_bytes {
            map.serialize_entry("cache_reserved_in_bytes", v)?;
        }
        if let Some(v) = &self.free {
            map.serialize_entry("free", v)?;
        }
        map.serialize_entry("free_in_bytes", &self.free_in_bytes)?;
        if let Some(v) = &self.total {
            map.serialize_entry("total", v)?;
        }
        map.serialize_entry("total_in_bytes", &self.total_in_bytes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ClusterIndices {
    pub analysis: Option<crate::types::cluster_stats::CharFilterTypes>,
    pub completion: Option<crate::types::common::CompletionStats>,
    #[doc = "Total number of indexes with shards assigned to selected nodes."]
    pub count: Option<i64>,
    pub docs: Option<crate::types::common::DocStats>,
    pub fielddata: Option<crate::types::common::FielddataStats>,
    pub mappings: Option<crate::types::cluster_stats::FieldTypesMappings>,
    pub query_cache: Option<crate::types::common::QueryCacheStats>,
    pub segments: Option<crate::types::common::SegmentsStats>,
    pub shards: Option<crate::types::cluster_stats::ClusterIndicesShards>,
    pub store: Option<crate::types::common::StoreStats>,
    #[doc = "Contains statistics about analyzers and analyzer components used in selected nodes."]
    pub versions: Option<Vec<crate::types::cluster_stats::IndicesVersions>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterIndices {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterIndices;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterIndices object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_analysis: Option<crate::types::cluster_stats::CharFilterTypes> = None;
                let mut seen_analysis = false;
                let mut f_completion: Option<crate::types::common::CompletionStats> = None;
                let mut seen_completion = false;
                let mut f_count: Option<i64> = None;
                let mut seen_count = false;
                let mut f_docs: Option<crate::types::common::DocStats> = None;
                let mut seen_docs = false;
                let mut f_fielddata: Option<crate::types::common::FielddataStats> = None;
                let mut seen_fielddata = false;
                let mut f_mappings: Option<crate::types::cluster_stats::FieldTypesMappings> = None;
                let mut seen_mappings = false;
                let mut f_query_cache: Option<crate::types::common::QueryCacheStats> = None;
                let mut seen_query_cache = false;
                let mut f_segments: Option<crate::types::common::SegmentsStats> = None;
                let mut seen_segments = false;
                let mut f_shards: Option<crate::types::cluster_stats::ClusterIndicesShards> = None;
                let mut seen_shards = false;
                let mut f_store: Option<crate::types::common::StoreStats> = None;
                let mut seen_store = false;
                let mut f_versions: Option<Vec<crate::types::cluster_stats::IndicesVersions>> =
                    None;
                let mut seen_versions = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "analysis" => {
                            if seen_analysis {
                                return Err(serde::de::Error::duplicate_field("analysis"));
                            }
                            seen_analysis = true;
                            f_analysis = map
                                .next_value::<Option<crate::types::cluster_stats::CharFilterTypes>>(
                                )?;
                        }
                        "completion" => {
                            if seen_completion {
                                return Err(serde::de::Error::duplicate_field("completion"));
                            }
                            seen_completion = true;
                            f_completion =
                                map.next_value::<Option<crate::types::common::CompletionStats>>()?;
                        }
                        "count" => {
                            if seen_count {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            seen_count = true;
                            f_count = map.next_value::<Option<i64>>()?;
                        }
                        "docs" => {
                            if seen_docs {
                                return Err(serde::de::Error::duplicate_field("docs"));
                            }
                            seen_docs = true;
                            f_docs = map.next_value::<Option<crate::types::common::DocStats>>()?;
                        }
                        "fielddata" => {
                            if seen_fielddata {
                                return Err(serde::de::Error::duplicate_field("fielddata"));
                            }
                            seen_fielddata = true;
                            f_fielddata =
                                map.next_value::<Option<crate::types::common::FielddataStats>>()?;
                        }
                        "mappings" => {
                            if seen_mappings {
                                return Err(serde::de::Error::duplicate_field("mappings"));
                            }
                            seen_mappings = true;
                            f_mappings = map.next_value::<Option<crate::types::cluster_stats::FieldTypesMappings>>()?;
                        }
                        "query_cache" => {
                            if seen_query_cache {
                                return Err(serde::de::Error::duplicate_field("query_cache"));
                            }
                            seen_query_cache = true;
                            f_query_cache =
                                map.next_value::<Option<crate::types::common::QueryCacheStats>>()?;
                        }
                        "segments" => {
                            if seen_segments {
                                return Err(serde::de::Error::duplicate_field("segments"));
                            }
                            seen_segments = true;
                            f_segments =
                                map.next_value::<Option<crate::types::common::SegmentsStats>>()?;
                        }
                        "shards" => {
                            if seen_shards {
                                return Err(serde::de::Error::duplicate_field("shards"));
                            }
                            seen_shards = true;
                            f_shards = map.next_value::<Option<crate::types::cluster_stats::ClusterIndicesShards>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store =
                                map.next_value::<Option<crate::types::common::StoreStats>>()?;
                        }
                        "versions" => {
                            if seen_versions {
                                return Err(serde::de::Error::duplicate_field("versions"));
                            }
                            seen_versions = true;
                            f_versions = map.next_value::<Option<Vec<crate::types::cluster_stats::IndicesVersions>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterIndices {
                    analysis: f_analysis,
                    completion: f_completion,
                    count: f_count,
                    docs: f_docs,
                    fielddata: f_fielddata,
                    mappings: f_mappings,
                    query_cache: f_query_cache,
                    segments: f_segments,
                    shards: f_shards,
                    store: f_store,
                    versions: f_versions,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterIndices {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.analysis {
            map.serialize_entry("analysis", v)?;
        }
        if let Some(v) = &self.completion {
            map.serialize_entry("completion", v)?;
        }
        if let Some(v) = &self.count {
            map.serialize_entry("count", v)?;
        }
        if let Some(v) = &self.docs {
            map.serialize_entry("docs", v)?;
        }
        if let Some(v) = &self.fielddata {
            map.serialize_entry("fielddata", v)?;
        }
        if let Some(v) = &self.mappings {
            map.serialize_entry("mappings", v)?;
        }
        if let Some(v) = &self.query_cache {
            map.serialize_entry("query_cache", v)?;
        }
        if let Some(v) = &self.segments {
            map.serialize_entry("segments", v)?;
        }
        if let Some(v) = &self.shards {
            map.serialize_entry("shards", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.versions {
            map.serialize_entry("versions", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ClusterIndicesShards {
    pub index: Option<crate::types::cluster_stats::ClusterIndicesShardsIndex>,
    #[doc = "Number of primary shards assigned to selected nodes."]
    pub primaries: Option<i32>,
    #[doc = "Ratio of replica shards to primary shards across all selected nodes."]
    pub replication: Option<i32>,
    #[doc = "Total number of shards assigned to selected nodes."]
    pub total: Option<i32>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterIndicesShards {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterIndicesShards;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterIndicesShards object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_index: Option<crate::types::cluster_stats::ClusterIndicesShardsIndex> =
                    None;
                let mut seen_index = false;
                let mut f_primaries: Option<i32> = None;
                let mut seen_primaries = false;
                let mut f_replication: Option<i32> = None;
                let mut seen_replication = false;
                let mut f_total: Option<i32> = None;
                let mut seen_total = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<crate::types::cluster_stats::ClusterIndicesShardsIndex>>()?;
                        }
                        "primaries" => {
                            if seen_primaries {
                                return Err(serde::de::Error::duplicate_field("primaries"));
                            }
                            seen_primaries = true;
                            f_primaries = map.next_value::<Option<i32>>()?;
                        }
                        "replication" => {
                            if seen_replication {
                                return Err(serde::de::Error::duplicate_field("replication"));
                            }
                            seen_replication = true;
                            f_replication = map.next_value::<Option<i32>>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map.next_value::<Option<i32>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterIndicesShards {
                    index: f_index,
                    primaries: f_primaries,
                    replication: f_replication,
                    total: f_total,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterIndicesShards {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.primaries {
            map.serialize_entry("primaries", v)?;
        }
        if let Some(v) = &self.replication {
            map.serialize_entry("replication", v)?;
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

#[derive(Debug, Clone)]
pub struct ClusterIndicesShardsIndex {
    #[doc = "Contains statistics about the number of primary shards assigned to selected nodes."]
    pub primaries: crate::types::cluster_stats::ClusterShardMetrics,
    #[doc = "Contains statistics about the number of replication shards assigned to selected nodes."]
    pub replication: crate::types::cluster_stats::ClusterShardMetrics,
    #[doc = "Contains statistics about the number of shards assigned to selected nodes."]
    pub shards: crate::types::cluster_stats::ClusterShardMetrics,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterIndicesShardsIndex {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterIndicesShardsIndex;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterIndicesShardsIndex object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_primaries: Option<crate::types::cluster_stats::ClusterShardMetrics> =
                    None;
                let mut f_replication: Option<crate::types::cluster_stats::ClusterShardMetrics> =
                    None;
                let mut f_shards: Option<crate::types::cluster_stats::ClusterShardMetrics> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "primaries" => {
                            if f_primaries.is_some() {
                                return Err(serde::de::Error::duplicate_field("primaries"));
                            }
                            f_primaries = Some(
                                map.next_value::<crate::types::cluster_stats::ClusterShardMetrics>(
                                )?,
                            );
                        }
                        "replication" => {
                            if f_replication.is_some() {
                                return Err(serde::de::Error::duplicate_field("replication"));
                            }
                            f_replication = Some(
                                map.next_value::<crate::types::cluster_stats::ClusterShardMetrics>(
                                )?,
                            );
                        }
                        "shards" => {
                            if f_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("shards"));
                            }
                            f_shards = Some(
                                map.next_value::<crate::types::cluster_stats::ClusterShardMetrics>(
                                )?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterIndicesShardsIndex {
                    primaries: f_primaries
                        .ok_or_else(|| serde::de::Error::missing_field("primaries"))?,
                    replication: f_replication
                        .ok_or_else(|| serde::de::Error::missing_field("replication"))?,
                    shards: f_shards.ok_or_else(|| serde::de::Error::missing_field("shards"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterIndicesShardsIndex {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("primaries", &self.primaries)?;
        map.serialize_entry("replication", &self.replication)?;
        map.serialize_entry("shards", &self.shards)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ClusterIngest {
    pub number_of_pipelines: i32,
    pub processor_stats:
        std::collections::BTreeMap<String, crate::types::cluster_stats::ClusterProcessor>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterIngest {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterIngest;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterIngest object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_number_of_pipelines: Option<i32> = None;
                let mut f_processor_stats: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::cluster_stats::ClusterProcessor,
                    >,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "number_of_pipelines" => {
                            if f_number_of_pipelines.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "number_of_pipelines",
                                ));
                            }
                            f_number_of_pipelines = Some(map.next_value::<i32>()?);
                        }
                        "processor_stats" => {
                            if f_processor_stats.is_some() {
                                return Err(serde::de::Error::duplicate_field("processor_stats"));
                            }
                            f_processor_stats = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                crate::types::cluster_stats::ClusterProcessor,
                            >>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterIngest {
                    number_of_pipelines: f_number_of_pipelines
                        .ok_or_else(|| serde::de::Error::missing_field("number_of_pipelines"))?,
                    processor_stats: f_processor_stats
                        .ok_or_else(|| serde::de::Error::missing_field("processor_stats"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterIngest {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("number_of_pipelines", &self.number_of_pipelines)?;
        map.serialize_entry("processor_stats", &self.processor_stats)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ClusterJvm {
    pub max_uptime: Option<crate::types::common::Duration>,
    pub max_uptime_in_millis: crate::types::common::DurationValueUnitMillis,
    pub mem: crate::types::cluster_stats::ClusterJvmMemory,
    #[doc = "Number of active threads in use by JVM across all selected nodes."]
    pub threads: i64,
    #[doc = "Contains statistics about the JVM versions used by selected nodes."]
    pub versions: Option<Vec<crate::types::cluster_stats::ClusterJvmVersion>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterJvm {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterJvm;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterJvm object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_max_uptime: Option<crate::types::common::Duration> = None;
                let mut seen_max_uptime = false;
                let mut f_max_uptime_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_mem: Option<crate::types::cluster_stats::ClusterJvmMemory> = None;
                let mut f_threads: Option<i64> = None;
                let mut f_versions: Option<Vec<crate::types::cluster_stats::ClusterJvmVersion>> =
                    None;
                let mut seen_versions = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "max_uptime" => {
                            if seen_max_uptime {
                                return Err(serde::de::Error::duplicate_field("max_uptime"));
                            }
                            seen_max_uptime = true;
                            f_max_uptime =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "max_uptime_in_millis" => {
                            if f_max_uptime_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_uptime_in_millis",
                                ));
                            }
                            f_max_uptime_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "mem" => {
                            if f_mem.is_some() {
                                return Err(serde::de::Error::duplicate_field("mem"));
                            }
                            f_mem = Some(
                                map.next_value::<crate::types::cluster_stats::ClusterJvmMemory>()?,
                            );
                        }
                        "threads" => {
                            if f_threads.is_some() {
                                return Err(serde::de::Error::duplicate_field("threads"));
                            }
                            f_threads = Some(map.next_value::<i64>()?);
                        }
                        "versions" => {
                            if seen_versions {
                                return Err(serde::de::Error::duplicate_field("versions"));
                            }
                            seen_versions = true;
                            f_versions = map.next_value::<Option<Vec<crate::types::cluster_stats::ClusterJvmVersion>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterJvm {
                    max_uptime: f_max_uptime,
                    max_uptime_in_millis: f_max_uptime_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("max_uptime_in_millis"))?,
                    mem: f_mem.ok_or_else(|| serde::de::Error::missing_field("mem"))?,
                    threads: f_threads.ok_or_else(|| serde::de::Error::missing_field("threads"))?,
                    versions: f_versions,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterJvm {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.max_uptime {
            map.serialize_entry("max_uptime", v)?;
        }
        map.serialize_entry("max_uptime_in_millis", &self.max_uptime_in_millis)?;
        map.serialize_entry("mem", &self.mem)?;
        map.serialize_entry("threads", &self.threads)?;
        if let Some(v) = &self.versions {
            map.serialize_entry("versions", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ClusterJvmMemory {
    #[doc = "Maximum amount of memory available for use by the heap across all selected nodes."]
    pub heap_max: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Maximum amount of memory, in bytes, available for use by the heap across all selected nodes."]
    pub heap_max_in_bytes: crate::types::common::ByteCount,
    #[doc = "Memory currently in use by the heap across all selected nodes."]
    pub heap_used: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Memory, in bytes, currently in use by the heap across all selected nodes."]
    pub heap_used_in_bytes: crate::types::common::ByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterJvmMemory {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterJvmMemory;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterJvmMemory object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_heap_max: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_heap_max = false;
                let mut f_heap_max_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_heap_used: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_heap_used = false;
                let mut f_heap_used_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "heap_max" => {
                            if seen_heap_max {
                                return Err(serde::de::Error::duplicate_field("heap_max"));
                            }
                            seen_heap_max = true;
                            f_heap_max = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "heap_max_in_bytes" => {
                            if f_heap_max_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("heap_max_in_bytes"));
                            }
                            f_heap_max_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "heap_used" => {
                            if seen_heap_used {
                                return Err(serde::de::Error::duplicate_field("heap_used"));
                            }
                            seen_heap_used = true;
                            f_heap_used = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "heap_used_in_bytes" => {
                            if f_heap_used_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "heap_used_in_bytes",
                                ));
                            }
                            f_heap_used_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterJvmMemory {
                    heap_max: f_heap_max,
                    heap_max_in_bytes: f_heap_max_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("heap_max_in_bytes"))?,
                    heap_used: f_heap_used,
                    heap_used_in_bytes: f_heap_used_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("heap_used_in_bytes"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterJvmMemory {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.heap_max {
            map.serialize_entry("heap_max", v)?;
        }
        map.serialize_entry("heap_max_in_bytes", &self.heap_max_in_bytes)?;
        if let Some(v) = &self.heap_used {
            map.serialize_entry("heap_used", v)?;
        }
        map.serialize_entry("heap_used_in_bytes", &self.heap_used_in_bytes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ClusterJvmVersion {
    #[doc = "Always `true`. All distributions come with a bundled Java Development Kit (JDK)."]
    pub bundled_jdk: bool,
    #[doc = "Total number of selected nodes using JVM."]
    pub count: i32,
    #[doc = "If `true`, a bundled JDK is in use by JVM."]
    pub using_bundled_jdk: bool,
    pub version: crate::types::common::VersionString,
    #[doc = "Name of the JVM."]
    pub vm_name: String,
    #[doc = "Vendor of the JVM."]
    pub vm_vendor: String,
    pub vm_version: crate::types::common::VersionString,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterJvmVersion {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterJvmVersion;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterJvmVersion object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_bundled_jdk: Option<bool> = None;
                let mut f_count: Option<i32> = None;
                let mut f_using_bundled_jdk: Option<bool> = None;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut f_vm_name: Option<String> = None;
                let mut f_vm_vendor: Option<String> = None;
                let mut f_vm_version: Option<crate::types::common::VersionString> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "bundled_jdk" => {
                            if f_bundled_jdk.is_some() {
                                return Err(serde::de::Error::duplicate_field("bundled_jdk"));
                            }
                            f_bundled_jdk = Some(map.next_value::<bool>()?);
                        }
                        "count" => {
                            if f_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            f_count = Some(map.next_value::<i32>()?);
                        }
                        "using_bundled_jdk" => {
                            if f_using_bundled_jdk.is_some() {
                                return Err(serde::de::Error::duplicate_field("using_bundled_jdk"));
                            }
                            f_using_bundled_jdk = Some(map.next_value::<bool>()?);
                        }
                        "version" => {
                            if f_version.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            f_version =
                                Some(map.next_value::<crate::types::common::VersionString>()?);
                        }
                        "vm_name" => {
                            if f_vm_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("vm_name"));
                            }
                            f_vm_name = Some(map.next_value::<String>()?);
                        }
                        "vm_vendor" => {
                            if f_vm_vendor.is_some() {
                                return Err(serde::de::Error::duplicate_field("vm_vendor"));
                            }
                            f_vm_vendor = Some(map.next_value::<String>()?);
                        }
                        "vm_version" => {
                            if f_vm_version.is_some() {
                                return Err(serde::de::Error::duplicate_field("vm_version"));
                            }
                            f_vm_version =
                                Some(map.next_value::<crate::types::common::VersionString>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterJvmVersion {
                    bundled_jdk: f_bundled_jdk
                        .ok_or_else(|| serde::de::Error::missing_field("bundled_jdk"))?,
                    count: f_count.ok_or_else(|| serde::de::Error::missing_field("count"))?,
                    using_bundled_jdk: f_using_bundled_jdk
                        .ok_or_else(|| serde::de::Error::missing_field("using_bundled_jdk"))?,
                    version: f_version.ok_or_else(|| serde::de::Error::missing_field("version"))?,
                    vm_name: f_vm_name.ok_or_else(|| serde::de::Error::missing_field("vm_name"))?,
                    vm_vendor: f_vm_vendor
                        .ok_or_else(|| serde::de::Error::missing_field("vm_vendor"))?,
                    vm_version: f_vm_version
                        .ok_or_else(|| serde::de::Error::missing_field("vm_version"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterJvmVersion {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("bundled_jdk", &self.bundled_jdk)?;
        map.serialize_entry("count", &self.count)?;
        map.serialize_entry("using_bundled_jdk", &self.using_bundled_jdk)?;
        map.serialize_entry("version", &self.version)?;
        map.serialize_entry("vm_name", &self.vm_name)?;
        map.serialize_entry("vm_vendor", &self.vm_vendor)?;
        map.serialize_entry("vm_version", &self.vm_version)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ClusterNetworkTypes {
    #[doc = "Contains statistics about the HTTP network types used by selected nodes."]
    pub http_types: std::collections::BTreeMap<String, i32>,
    #[doc = "Contains statistics about the transport network types used by selected nodes."]
    pub transport_types: std::collections::BTreeMap<String, i32>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterNetworkTypes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterNetworkTypes;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterNetworkTypes object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_http_types: Option<std::collections::BTreeMap<String, i32>> = None;
                let mut f_transport_types: Option<std::collections::BTreeMap<String, i32>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "http_types" => {
                            if f_http_types.is_some() {
                                return Err(serde::de::Error::duplicate_field("http_types"));
                            }
                            f_http_types =
                                Some(map.next_value::<std::collections::BTreeMap<String, i32>>()?);
                        }
                        "transport_types" => {
                            if f_transport_types.is_some() {
                                return Err(serde::de::Error::duplicate_field("transport_types"));
                            }
                            f_transport_types =
                                Some(map.next_value::<std::collections::BTreeMap<String, i32>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterNetworkTypes {
                    http_types: f_http_types
                        .ok_or_else(|| serde::de::Error::missing_field("http_types"))?,
                    transport_types: f_transport_types
                        .ok_or_else(|| serde::de::Error::missing_field("transport_types"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterNetworkTypes {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("http_types", &self.http_types)?;
        map.serialize_entry("transport_types", &self.transport_types)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ClusterNodeCount {
    pub cluster_manager: Option<i32>,
    pub coordinating_only: i32,
    pub data: i32,
    pub data_cold: Option<i32>,
    pub data_content: Option<i32>,
    pub data_frozen: Option<i32>,
    pub data_hot: Option<i32>,
    pub data_warm: Option<i32>,
    pub ingest: i32,
    pub master: i32,
    pub ml: Option<i32>,
    pub remote_cluster_client: i32,
    pub search: Option<i32>,
    pub total: i32,
    pub transform: Option<i32>,
    pub voting_only: Option<i32>,
    pub warm: Option<i32>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterNodeCount {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterNodeCount;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterNodeCount object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cluster_manager: Option<i32> = None;
                let mut seen_cluster_manager = false;
                let mut f_coordinating_only: Option<i32> = None;
                let mut f_data: Option<i32> = None;
                let mut f_data_cold: Option<i32> = None;
                let mut seen_data_cold = false;
                let mut f_data_content: Option<i32> = None;
                let mut seen_data_content = false;
                let mut f_data_frozen: Option<i32> = None;
                let mut seen_data_frozen = false;
                let mut f_data_hot: Option<i32> = None;
                let mut seen_data_hot = false;
                let mut f_data_warm: Option<i32> = None;
                let mut seen_data_warm = false;
                let mut f_ingest: Option<i32> = None;
                let mut f_master: Option<i32> = None;
                let mut f_ml: Option<i32> = None;
                let mut seen_ml = false;
                let mut f_remote_cluster_client: Option<i32> = None;
                let mut f_search: Option<i32> = None;
                let mut seen_search = false;
                let mut f_total: Option<i32> = None;
                let mut f_transform: Option<i32> = None;
                let mut seen_transform = false;
                let mut f_voting_only: Option<i32> = None;
                let mut seen_voting_only = false;
                let mut f_warm: Option<i32> = None;
                let mut seen_warm = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cluster_manager" => {
                            if seen_cluster_manager {
                                return Err(serde::de::Error::duplicate_field("cluster_manager"));
                            }
                            seen_cluster_manager = true;
                            f_cluster_manager = map.next_value::<Option<i32>>()?;
                        }
                        "coordinating_only" => {
                            if f_coordinating_only.is_some() {
                                return Err(serde::de::Error::duplicate_field("coordinating_only"));
                            }
                            f_coordinating_only = Some(map.next_value::<i32>()?);
                        }
                        "data" => {
                            if f_data.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            f_data = Some(map.next_value::<i32>()?);
                        }
                        "data_cold" => {
                            if seen_data_cold {
                                return Err(serde::de::Error::duplicate_field("data_cold"));
                            }
                            seen_data_cold = true;
                            f_data_cold = map.next_value::<Option<i32>>()?;
                        }
                        "data_content" => {
                            if seen_data_content {
                                return Err(serde::de::Error::duplicate_field("data_content"));
                            }
                            seen_data_content = true;
                            f_data_content = map.next_value::<Option<i32>>()?;
                        }
                        "data_frozen" => {
                            if seen_data_frozen {
                                return Err(serde::de::Error::duplicate_field("data_frozen"));
                            }
                            seen_data_frozen = true;
                            f_data_frozen = map.next_value::<Option<i32>>()?;
                        }
                        "data_hot" => {
                            if seen_data_hot {
                                return Err(serde::de::Error::duplicate_field("data_hot"));
                            }
                            seen_data_hot = true;
                            f_data_hot = map.next_value::<Option<i32>>()?;
                        }
                        "data_warm" => {
                            if seen_data_warm {
                                return Err(serde::de::Error::duplicate_field("data_warm"));
                            }
                            seen_data_warm = true;
                            f_data_warm = map.next_value::<Option<i32>>()?;
                        }
                        "ingest" => {
                            if f_ingest.is_some() {
                                return Err(serde::de::Error::duplicate_field("ingest"));
                            }
                            f_ingest = Some(map.next_value::<i32>()?);
                        }
                        "master" => {
                            if f_master.is_some() {
                                return Err(serde::de::Error::duplicate_field("master"));
                            }
                            f_master = Some(map.next_value::<i32>()?);
                        }
                        "ml" => {
                            if seen_ml {
                                return Err(serde::de::Error::duplicate_field("ml"));
                            }
                            seen_ml = true;
                            f_ml = map.next_value::<Option<i32>>()?;
                        }
                        "remote_cluster_client" => {
                            if f_remote_cluster_client.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "remote_cluster_client",
                                ));
                            }
                            f_remote_cluster_client = Some(map.next_value::<i32>()?);
                        }
                        "search" => {
                            if seen_search {
                                return Err(serde::de::Error::duplicate_field("search"));
                            }
                            seen_search = true;
                            f_search = map.next_value::<Option<i32>>()?;
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(map.next_value::<i32>()?);
                        }
                        "transform" => {
                            if seen_transform {
                                return Err(serde::de::Error::duplicate_field("transform"));
                            }
                            seen_transform = true;
                            f_transform = map.next_value::<Option<i32>>()?;
                        }
                        "voting_only" => {
                            if seen_voting_only {
                                return Err(serde::de::Error::duplicate_field("voting_only"));
                            }
                            seen_voting_only = true;
                            f_voting_only = map.next_value::<Option<i32>>()?;
                        }
                        "warm" => {
                            if seen_warm {
                                return Err(serde::de::Error::duplicate_field("warm"));
                            }
                            seen_warm = true;
                            f_warm = map.next_value::<Option<i32>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterNodeCount {
                    cluster_manager: f_cluster_manager,
                    coordinating_only: f_coordinating_only
                        .ok_or_else(|| serde::de::Error::missing_field("coordinating_only"))?,
                    data: f_data.ok_or_else(|| serde::de::Error::missing_field("data"))?,
                    data_cold: f_data_cold,
                    data_content: f_data_content,
                    data_frozen: f_data_frozen,
                    data_hot: f_data_hot,
                    data_warm: f_data_warm,
                    ingest: f_ingest.ok_or_else(|| serde::de::Error::missing_field("ingest"))?,
                    master: f_master.ok_or_else(|| serde::de::Error::missing_field("master"))?,
                    ml: f_ml,
                    remote_cluster_client: f_remote_cluster_client
                        .ok_or_else(|| serde::de::Error::missing_field("remote_cluster_client"))?,
                    search: f_search,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    transform: f_transform,
                    voting_only: f_voting_only,
                    warm: f_warm,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterNodeCount {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cluster_manager {
            map.serialize_entry("cluster_manager", v)?;
        }
        map.serialize_entry("coordinating_only", &self.coordinating_only)?;
        map.serialize_entry("data", &self.data)?;
        if let Some(v) = &self.data_cold {
            map.serialize_entry("data_cold", v)?;
        }
        if let Some(v) = &self.data_content {
            map.serialize_entry("data_content", v)?;
        }
        if let Some(v) = &self.data_frozen {
            map.serialize_entry("data_frozen", v)?;
        }
        if let Some(v) = &self.data_hot {
            map.serialize_entry("data_hot", v)?;
        }
        if let Some(v) = &self.data_warm {
            map.serialize_entry("data_warm", v)?;
        }
        map.serialize_entry("ingest", &self.ingest)?;
        map.serialize_entry("master", &self.master)?;
        if let Some(v) = &self.ml {
            map.serialize_entry("ml", v)?;
        }
        map.serialize_entry("remote_cluster_client", &self.remote_cluster_client)?;
        if let Some(v) = &self.search {
            map.serialize_entry("search", v)?;
        }
        map.serialize_entry("total", &self.total)?;
        if let Some(v) = &self.transform {
            map.serialize_entry("transform", v)?;
        }
        if let Some(v) = &self.voting_only {
            map.serialize_entry("voting_only", v)?;
        }
        if let Some(v) = &self.warm {
            map.serialize_entry("warm", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ClusterNodes {
    #[doc = "Contains counts for nodes selected by the request's node filters."]
    pub count: Option<crate::types::cluster_stats::ClusterNodeCount>,
    #[doc = "Contains statistics about the discovery types used by selected nodes."]
    pub discovery_types: Option<std::collections::BTreeMap<String, i32>>,
    #[doc = "Contains statistics about file stores by selected nodes."]
    pub fs: Option<crate::types::cluster_stats::ClusterFileSystem>,
    pub indexing_pressure: Option<crate::types::cluster_stats::IndexingPressure>,
    pub ingest: Option<crate::types::cluster_stats::ClusterIngest>,
    #[doc = "Contains statistics about the Java Virtual Machines (JVMs) used by selected nodes."]
    pub jvm: Option<crate::types::cluster_stats::ClusterJvm>,
    #[doc = "Contains statistics about the transport and HTTP networks used by selected nodes."]
    pub network_types: Option<crate::types::cluster_stats::ClusterNetworkTypes>,
    #[doc = "Contains statistics about the operating systems used by selected nodes."]
    pub os: Option<crate::types::cluster_stats::ClusterOperatingSystem>,
    #[doc = "Contains statistics about OpenSearch distributions installed on selected nodes."]
    pub packaging_types: Option<Vec<crate::types::cluster_stats::NodePackagingType>>,
    #[doc = "Contains statistics about installed plugins and modules by selected nodes. If no plugins or modules are installed, this array is empty."]
    pub plugins: Option<Vec<crate::types::common::PluginStats>>,
    #[doc = "Contains statistics about processes used by selected nodes."]
    pub process: Option<crate::types::cluster_stats::ClusterProcess>,
    #[doc = "Array of OpenSearch versions used on selected nodes."]
    pub versions: Option<Vec<crate::types::common::VersionString>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterNodes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterNodes;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterNodes object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_count: Option<crate::types::cluster_stats::ClusterNodeCount> = None;
                let mut seen_count = false;
                let mut f_discovery_types: Option<std::collections::BTreeMap<String, i32>> = None;
                let mut seen_discovery_types = false;
                let mut f_fs: Option<crate::types::cluster_stats::ClusterFileSystem> = None;
                let mut seen_fs = false;
                let mut f_indexing_pressure: Option<crate::types::cluster_stats::IndexingPressure> =
                    None;
                let mut seen_indexing_pressure = false;
                let mut f_ingest: Option<crate::types::cluster_stats::ClusterIngest> = None;
                let mut seen_ingest = false;
                let mut f_jvm: Option<crate::types::cluster_stats::ClusterJvm> = None;
                let mut seen_jvm = false;
                let mut f_network_types: Option<crate::types::cluster_stats::ClusterNetworkTypes> =
                    None;
                let mut seen_network_types = false;
                let mut f_os: Option<crate::types::cluster_stats::ClusterOperatingSystem> = None;
                let mut seen_os = false;
                let mut f_packaging_types: Option<
                    Vec<crate::types::cluster_stats::NodePackagingType>,
                > = None;
                let mut seen_packaging_types = false;
                let mut f_plugins: Option<Vec<crate::types::common::PluginStats>> = None;
                let mut seen_plugins = false;
                let mut f_process: Option<crate::types::cluster_stats::ClusterProcess> = None;
                let mut seen_process = false;
                let mut f_versions: Option<Vec<crate::types::common::VersionString>> = None;
                let mut seen_versions = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "count" => {
                            if seen_count {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            seen_count = true;
                            f_count = map.next_value::<Option<crate::types::cluster_stats::ClusterNodeCount>>()?;
                        }
                        "discovery_types" => {
                            if seen_discovery_types {
                                return Err(serde::de::Error::duplicate_field("discovery_types"));
                            }
                            seen_discovery_types = true;
                            f_discovery_types = map
                                .next_value::<Option<std::collections::BTreeMap<String, i32>>>()?;
                        }
                        "fs" => {
                            if seen_fs {
                                return Err(serde::de::Error::duplicate_field("fs"));
                            }
                            seen_fs = true;
                            f_fs = map.next_value::<Option<crate::types::cluster_stats::ClusterFileSystem>>()?;
                        }
                        "indexing_pressure" => {
                            if seen_indexing_pressure {
                                return Err(serde::de::Error::duplicate_field("indexing_pressure"));
                            }
                            seen_indexing_pressure = true;
                            f_indexing_pressure = map.next_value::<Option<crate::types::cluster_stats::IndexingPressure>>()?;
                        }
                        "ingest" => {
                            if seen_ingest {
                                return Err(serde::de::Error::duplicate_field("ingest"));
                            }
                            seen_ingest = true;
                            f_ingest = map
                                .next_value::<Option<crate::types::cluster_stats::ClusterIngest>>(
                                )?;
                        }
                        "jvm" => {
                            if seen_jvm {
                                return Err(serde::de::Error::duplicate_field("jvm"));
                            }
                            seen_jvm = true;
                            f_jvm = map
                                .next_value::<Option<crate::types::cluster_stats::ClusterJvm>>()?;
                        }
                        "network_types" => {
                            if seen_network_types {
                                return Err(serde::de::Error::duplicate_field("network_types"));
                            }
                            seen_network_types = true;
                            f_network_types = map.next_value::<Option<crate::types::cluster_stats::ClusterNetworkTypes>>()?;
                        }
                        "os" => {
                            if seen_os {
                                return Err(serde::de::Error::duplicate_field("os"));
                            }
                            seen_os = true;
                            f_os = map.next_value::<Option<crate::types::cluster_stats::ClusterOperatingSystem>>()?;
                        }
                        "packaging_types" => {
                            if seen_packaging_types {
                                return Err(serde::de::Error::duplicate_field("packaging_types"));
                            }
                            seen_packaging_types = true;
                            f_packaging_types = map.next_value::<Option<Vec<crate::types::cluster_stats::NodePackagingType>>>()?;
                        }
                        "plugins" => {
                            if seen_plugins {
                                return Err(serde::de::Error::duplicate_field("plugins"));
                            }
                            seen_plugins = true;
                            f_plugins =
                                map.next_value::<Option<Vec<crate::types::common::PluginStats>>>()?;
                        }
                        "process" => {
                            if seen_process {
                                return Err(serde::de::Error::duplicate_field("process"));
                            }
                            seen_process = true;
                            f_process = map
                                .next_value::<Option<crate::types::cluster_stats::ClusterProcess>>(
                                )?;
                        }
                        "versions" => {
                            if seen_versions {
                                return Err(serde::de::Error::duplicate_field("versions"));
                            }
                            seen_versions = true;
                            f_versions = map
                                .next_value::<Option<Vec<crate::types::common::VersionString>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterNodes {
                    count: f_count,
                    discovery_types: f_discovery_types,
                    fs: f_fs,
                    indexing_pressure: f_indexing_pressure,
                    ingest: f_ingest,
                    jvm: f_jvm,
                    network_types: f_network_types,
                    os: f_os,
                    packaging_types: f_packaging_types,
                    plugins: f_plugins,
                    process: f_process,
                    versions: f_versions,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterNodes {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.count {
            map.serialize_entry("count", v)?;
        }
        if let Some(v) = &self.discovery_types {
            map.serialize_entry("discovery_types", v)?;
        }
        if let Some(v) = &self.fs {
            map.serialize_entry("fs", v)?;
        }
        if let Some(v) = &self.indexing_pressure {
            map.serialize_entry("indexing_pressure", v)?;
        }
        if let Some(v) = &self.ingest {
            map.serialize_entry("ingest", v)?;
        }
        if let Some(v) = &self.jvm {
            map.serialize_entry("jvm", v)?;
        }
        if let Some(v) = &self.network_types {
            map.serialize_entry("network_types", v)?;
        }
        if let Some(v) = &self.os {
            map.serialize_entry("os", v)?;
        }
        if let Some(v) = &self.packaging_types {
            map.serialize_entry("packaging_types", v)?;
        }
        if let Some(v) = &self.plugins {
            map.serialize_entry("plugins", v)?;
        }
        if let Some(v) = &self.process {
            map.serialize_entry("process", v)?;
        }
        if let Some(v) = &self.versions {
            map.serialize_entry("versions", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ClusterOperatingSystem {
    #[doc = "Number of processors used to calculate thread pool size across all selected nodes. This number can be set with the processors setting of a node and defaults to the number of processors reported by the operating system. In both cases, this number will never be larger than 32."]
    pub allocated_processors: i32,
    #[doc = "Contains statistics about processor architectures (for example, x86_64 or aarch64) used by selected nodes."]
    pub architectures: Option<Vec<crate::types::cluster_stats::ClusterOperatingSystemArchitecture>>,
    #[doc = "Number of processors available to JVM across all selected nodes."]
    pub available_processors: i32,
    pub mem: crate::types::cluster_stats::OperatingSystemMemoryInfo,
    #[doc = "Contains statistics about operating systems used by selected nodes."]
    pub names: Vec<crate::types::cluster_stats::ClusterOperatingSystemName>,
    #[doc = "Contains statistics about operating systems used by selected nodes."]
    pub pretty_names: Vec<crate::types::cluster_stats::ClusterOperatingSystemPrettyName>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterOperatingSystem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterOperatingSystem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterOperatingSystem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_allocated_processors: Option<i32> = None;
                let mut f_architectures: Option<
                    Vec<crate::types::cluster_stats::ClusterOperatingSystemArchitecture>,
                > = None;
                let mut seen_architectures = false;
                let mut f_available_processors: Option<i32> = None;
                let mut f_mem: Option<crate::types::cluster_stats::OperatingSystemMemoryInfo> =
                    None;
                let mut f_names: Option<
                    Vec<crate::types::cluster_stats::ClusterOperatingSystemName>,
                > = None;
                let mut f_pretty_names: Option<
                    Vec<crate::types::cluster_stats::ClusterOperatingSystemPrettyName>,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "allocated_processors" => {
                            if f_allocated_processors.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "allocated_processors",
                                ));
                            }
                            f_allocated_processors = Some(map.next_value::<i32>()?);
                        }
                        "architectures" => {
                            if seen_architectures {
                                return Err(serde::de::Error::duplicate_field("architectures"));
                            }
                            seen_architectures = true;
                            f_architectures = map.next_value::<Option<
                                Vec<
                                    crate::types::cluster_stats::ClusterOperatingSystemArchitecture,
                                >,
                            >>()?;
                        }
                        "available_processors" => {
                            if f_available_processors.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "available_processors",
                                ));
                            }
                            f_available_processors = Some(map.next_value::<i32>()?);
                        }
                        "mem" => {
                            if f_mem.is_some() {
                                return Err(serde::de::Error::duplicate_field("mem"));
                            }
                            f_mem = Some(map.next_value::<crate::types::cluster_stats::OperatingSystemMemoryInfo>()?);
                        }
                        "names" => {
                            if f_names.is_some() {
                                return Err(serde::de::Error::duplicate_field("names"));
                            }
                            f_names =
                                Some(
                                    map.next_value::<Vec<
                                        crate::types::cluster_stats::ClusterOperatingSystemName,
                                    >>()?,
                                );
                        }
                        "pretty_names" => {
                            if f_pretty_names.is_some() {
                                return Err(serde::de::Error::duplicate_field("pretty_names"));
                            }
                            f_pretty_names = Some(map.next_value::<Vec<
                                crate::types::cluster_stats::ClusterOperatingSystemPrettyName,
                            >>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterOperatingSystem {
                    allocated_processors: f_allocated_processors
                        .ok_or_else(|| serde::de::Error::missing_field("allocated_processors"))?,
                    architectures: f_architectures,
                    available_processors: f_available_processors
                        .ok_or_else(|| serde::de::Error::missing_field("available_processors"))?,
                    mem: f_mem.ok_or_else(|| serde::de::Error::missing_field("mem"))?,
                    names: f_names.ok_or_else(|| serde::de::Error::missing_field("names"))?,
                    pretty_names: f_pretty_names
                        .ok_or_else(|| serde::de::Error::missing_field("pretty_names"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterOperatingSystem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("allocated_processors", &self.allocated_processors)?;
        if let Some(v) = &self.architectures {
            map.serialize_entry("architectures", v)?;
        }
        map.serialize_entry("available_processors", &self.available_processors)?;
        map.serialize_entry("mem", &self.mem)?;
        map.serialize_entry("names", &self.names)?;
        map.serialize_entry("pretty_names", &self.pretty_names)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ClusterOperatingSystemArchitecture {
    #[doc = "Name of an architecture used by one or more selected nodes."]
    pub arch: String,
    #[doc = "Number of selected nodes using the architecture."]
    pub count: i32,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterOperatingSystemArchitecture {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterOperatingSystemArchitecture;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterOperatingSystemArchitecture object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_arch: Option<String> = None;
                let mut f_count: Option<i32> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "arch" => {
                            if f_arch.is_some() {
                                return Err(serde::de::Error::duplicate_field("arch"));
                            }
                            f_arch = Some(map.next_value::<String>()?);
                        }
                        "count" => {
                            if f_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            f_count = Some(map.next_value::<i32>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterOperatingSystemArchitecture {
                    arch: f_arch.ok_or_else(|| serde::de::Error::missing_field("arch"))?,
                    count: f_count.ok_or_else(|| serde::de::Error::missing_field("count"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterOperatingSystemArchitecture {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("arch", &self.arch)?;
        map.serialize_entry("count", &self.count)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ClusterOperatingSystemName {
    #[doc = "Number of selected nodes using the operating system."]
    pub count: i32,
    pub name: Option<crate::types::common::Name>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterOperatingSystemName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterOperatingSystemName;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterOperatingSystemName object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_count: Option<i32> = None;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut seen_name = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "count" => {
                            if f_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            f_count = Some(map.next_value::<i32>()?);
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterOperatingSystemName {
                    count: f_count.ok_or_else(|| serde::de::Error::missing_field("count"))?,
                    name: f_name,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterOperatingSystemName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("count", &self.count)?;
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ClusterOperatingSystemPrettyName {
    #[doc = "Number of selected nodes using the operating system."]
    pub count: i32,
    pub pretty_name: Option<crate::types::common::Name>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterOperatingSystemPrettyName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterOperatingSystemPrettyName;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterOperatingSystemPrettyName object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_count: Option<i32> = None;
                let mut f_pretty_name: Option<crate::types::common::Name> = None;
                let mut seen_pretty_name = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "count" => {
                            if f_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            f_count = Some(map.next_value::<i32>()?);
                        }
                        "pretty_name" => {
                            if seen_pretty_name {
                                return Err(serde::de::Error::duplicate_field("pretty_name"));
                            }
                            seen_pretty_name = true;
                            f_pretty_name =
                                map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterOperatingSystemPrettyName {
                    count: f_count.ok_or_else(|| serde::de::Error::missing_field("count"))?,
                    pretty_name: f_pretty_name,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterOperatingSystemPrettyName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("count", &self.count)?;
        if let Some(v) = &self.pretty_name {
            map.serialize_entry("pretty_name", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ClusterProcess {
    pub cpu: crate::types::cluster_stats::ClusterProcessCpu,
    pub open_file_descriptors: crate::types::cluster_stats::ClusterProcessOpenFileDescriptors,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterProcess {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterProcess;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterProcess object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cpu: Option<crate::types::cluster_stats::ClusterProcessCpu> = None;
                let mut f_open_file_descriptors: Option<
                    crate::types::cluster_stats::ClusterProcessOpenFileDescriptors,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cpu" => {
                            if f_cpu.is_some() {
                                return Err(serde::de::Error::duplicate_field("cpu"));
                            }
                            f_cpu = Some(
                                map.next_value::<crate::types::cluster_stats::ClusterProcessCpu>()?,
                            );
                        }
                        "open_file_descriptors" => {
                            if f_open_file_descriptors.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "open_file_descriptors",
                                ));
                            }
                            f_open_file_descriptors = Some(map.next_value::<crate::types::cluster_stats::ClusterProcessOpenFileDescriptors>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterProcess {
                    cpu: f_cpu.ok_or_else(|| serde::de::Error::missing_field("cpu"))?,
                    open_file_descriptors: f_open_file_descriptors
                        .ok_or_else(|| serde::de::Error::missing_field("open_file_descriptors"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterProcess {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("cpu", &self.cpu)?;
        map.serialize_entry("open_file_descriptors", &self.open_file_descriptors)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ClusterProcessCpu {
    #[doc = "Percentage of CPU used across all selected nodes. Returns `-1` if not supported."]
    pub percent: crate::types::common::PercentageNumber,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterProcessCpu {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterProcessCpu;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterProcessCpu object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_percent: Option<crate::types::common::PercentageNumber> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "percent" => {
                            if f_percent.is_some() {
                                return Err(serde::de::Error::duplicate_field("percent"));
                            }
                            f_percent =
                                Some(map.next_value::<crate::types::common::PercentageNumber>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterProcessCpu {
                    percent: f_percent.ok_or_else(|| serde::de::Error::missing_field("percent"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterProcessCpu {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("percent", &self.percent)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ClusterProcessOpenFileDescriptors {
    #[doc = "Average number of concurrently open file descriptors. Returns `-1` if not supported."]
    pub avg: i64,
    #[doc = "Maximum number of concurrently open file descriptors allowed across all selected nodes. Returns `-1` if not supported."]
    pub max: i64,
    #[doc = "Minimum number of concurrently open file descriptors across all selected nodes. Returns -1 if not supported."]
    pub min: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterProcessOpenFileDescriptors {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterProcessOpenFileDescriptors;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterProcessOpenFileDescriptors object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_avg: Option<i64> = None;
                let mut f_max: Option<i64> = None;
                let mut f_min: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "avg" => {
                            if f_avg.is_some() {
                                return Err(serde::de::Error::duplicate_field("avg"));
                            }
                            f_avg = Some(map.next_value::<i64>()?);
                        }
                        "max" => {
                            if f_max.is_some() {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            f_max = Some(map.next_value::<i64>()?);
                        }
                        "min" => {
                            if f_min.is_some() {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            f_min = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterProcessOpenFileDescriptors {
                    avg: f_avg.ok_or_else(|| serde::de::Error::missing_field("avg"))?,
                    max: f_max.ok_or_else(|| serde::de::Error::missing_field("max"))?,
                    min: f_min.ok_or_else(|| serde::de::Error::missing_field("min"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterProcessOpenFileDescriptors {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("avg", &self.avg)?;
        map.serialize_entry("max", &self.max)?;
        map.serialize_entry("min", &self.min)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ClusterProcessor {
    pub count: i64,
    pub current: i64,
    pub failed: i64,
    pub time: Option<crate::types::common::Duration>,
    pub time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_count: Option<i64> = None;
                let mut f_current: Option<i64> = None;
                let mut f_failed: Option<i64> = None;
                let mut f_time: Option<crate::types::common::Duration> = None;
                let mut seen_time = false;
                let mut f_time_in_millis: Option<crate::types::common::DurationValueUnitMillis> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "count" => {
                            if f_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            f_count = Some(map.next_value::<i64>()?);
                        }
                        "current" => {
                            if f_current.is_some() {
                                return Err(serde::de::Error::duplicate_field("current"));
                            }
                            f_current = Some(map.next_value::<i64>()?);
                        }
                        "failed" => {
                            if f_failed.is_some() {
                                return Err(serde::de::Error::duplicate_field("failed"));
                            }
                            f_failed = Some(map.next_value::<i64>()?);
                        }
                        "time" => {
                            if seen_time {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            seen_time = true;
                            f_time = map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "time_in_millis" => {
                            if f_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field("time_in_millis"));
                            }
                            f_time_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterProcessor {
                    count: f_count.ok_or_else(|| serde::de::Error::missing_field("count"))?,
                    current: f_current.ok_or_else(|| serde::de::Error::missing_field("current"))?,
                    failed: f_failed.ok_or_else(|| serde::de::Error::missing_field("failed"))?,
                    time: f_time,
                    time_in_millis: f_time_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("time_in_millis"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("count", &self.count)?;
        map.serialize_entry("current", &self.current)?;
        map.serialize_entry("failed", &self.failed)?;
        if let Some(v) = &self.time {
            map.serialize_entry("time", v)?;
        }
        map.serialize_entry("time_in_millis", &self.time_in_millis)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ClusterShardMetrics {
    #[doc = "Mean number of shards in an index, counting only shards assigned to selected nodes."]
    pub avg: f64,
    #[doc = "Maximum number of shards in an index, counting only shards assigned to selected nodes."]
    pub max: f64,
    #[doc = "Minimum number of shards in an index, counting only shards assigned to selected nodes."]
    pub min: f64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterShardMetrics {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterShardMetrics;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterShardMetrics object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_avg: Option<f64> = None;
                let mut f_max: Option<f64> = None;
                let mut f_min: Option<f64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "avg" => {
                            if f_avg.is_some() {
                                return Err(serde::de::Error::duplicate_field("avg"));
                            }
                            f_avg = Some(map.next_value::<f64>()?);
                        }
                        "max" => {
                            if f_max.is_some() {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            f_max = Some(map.next_value::<f64>()?);
                        }
                        "min" => {
                            if f_min.is_some() {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            f_min = Some(map.next_value::<f64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterShardMetrics {
                    avg: f_avg.ok_or_else(|| serde::de::Error::missing_field("avg"))?,
                    max: f_max.ok_or_else(|| serde::de::Error::missing_field("max"))?,
                    min: f_min.ok_or_else(|| serde::de::Error::missing_field("min"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterShardMetrics {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("avg", &self.avg)?;
        map.serialize_entry("max", &self.max)?;
        map.serialize_entry("min", &self.min)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct FieldTypes {
    #[doc = "The number of occurrences of the field type in selected nodes."]
    pub count: i32,
    #[doc = "The number of indexes containing the field type in selected nodes."]
    pub index_count: i32,
    pub name: crate::types::common::Name,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FieldTypes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FieldTypes;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FieldTypes object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_count: Option<i32> = None;
                let mut f_index_count: Option<i32> = None;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "count" => {
                            if f_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            f_count = Some(map.next_value::<i32>()?);
                        }
                        "index_count" => {
                            if f_index_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("index_count"));
                            }
                            f_index_count = Some(map.next_value::<i32>()?);
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FieldTypes {
                    count: f_count.ok_or_else(|| serde::de::Error::missing_field("count"))?,
                    index_count: f_index_count
                        .ok_or_else(|| serde::de::Error::missing_field("index_count"))?,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FieldTypes {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("count", &self.count)?;
        map.serialize_entry("index_count", &self.index_count)?;
        map.serialize_entry("name", &self.name)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct FieldTypesMappings {
    #[doc = "Contains statistics about field data types used in selected nodes."]
    pub field_types: Vec<crate::types::cluster_stats::FieldTypes>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FieldTypesMappings {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FieldTypesMappings;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FieldTypesMappings object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_field_types: Option<Vec<crate::types::cluster_stats::FieldTypes>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "field_types" => {
                            if f_field_types.is_some() {
                                return Err(serde::de::Error::duplicate_field("field_types"));
                            }
                            f_field_types = Some(
                                map.next_value::<Vec<crate::types::cluster_stats::FieldTypes>>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FieldTypesMappings {
                    field_types: f_field_types
                        .ok_or_else(|| serde::de::Error::missing_field("field_types"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FieldTypesMappings {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("field_types", &self.field_types)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IndexingPressure {
    pub memory: crate::types::cluster_stats::IndexingPressureMemory,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexingPressure {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexingPressure;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexingPressure object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_memory: Option<crate::types::cluster_stats::IndexingPressureMemory> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "memory" => {
                            if f_memory.is_some() {
                                return Err(serde::de::Error::duplicate_field("memory"));
                            }
                            f_memory = Some(map.next_value::<crate::types::cluster_stats::IndexingPressureMemory>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexingPressure {
                    memory: f_memory.ok_or_else(|| serde::de::Error::missing_field("memory"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexingPressure {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("memory", &self.memory)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IndexingPressureMemory {
    pub current: crate::types::cluster_stats::IndexingPressureMemorySummary,
    pub limit_in_bytes: crate::types::common::ByteCount,
    pub total: crate::types::cluster_stats::IndexingPressureMemorySummary,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexingPressureMemory {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexingPressureMemory;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexingPressureMemory object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_current: Option<
                    crate::types::cluster_stats::IndexingPressureMemorySummary,
                > = None;
                let mut f_limit_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_total: Option<
                    crate::types::cluster_stats::IndexingPressureMemorySummary,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "current" => {
                            if f_current.is_some() {
                                return Err(serde::de::Error::duplicate_field("current"));
                            }
                            f_current = Some(map.next_value::<crate::types::cluster_stats::IndexingPressureMemorySummary>()?);
                        }
                        "limit_in_bytes" => {
                            if f_limit_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit_in_bytes"));
                            }
                            f_limit_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(map.next_value::<crate::types::cluster_stats::IndexingPressureMemorySummary>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexingPressureMemory {
                    current: f_current.ok_or_else(|| serde::de::Error::missing_field("current"))?,
                    limit_in_bytes: f_limit_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("limit_in_bytes"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexingPressureMemory {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("current", &self.current)?;
        map.serialize_entry("limit_in_bytes", &self.limit_in_bytes)?;
        map.serialize_entry("total", &self.total)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IndexingPressureMemorySummary {
    pub all_in_bytes: crate::types::common::ByteCount,
    pub combined_coordinating_and_primary_in_bytes: crate::types::common::ByteCount,
    pub coordinating_in_bytes: crate::types::common::ByteCount,
    pub coordinating_rejections: Option<f64>,
    pub primary_in_bytes: crate::types::common::ByteCount,
    pub primary_rejections: Option<f64>,
    pub replica_in_bytes: crate::types::common::ByteCount,
    pub replica_rejections: Option<f64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexingPressureMemorySummary {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexingPressureMemorySummary;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexingPressureMemorySummary object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_all_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_combined_coordinating_and_primary_in_bytes: Option<
                    crate::types::common::ByteCount,
                > = None;
                let mut f_coordinating_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_coordinating_rejections: Option<f64> = None;
                let mut seen_coordinating_rejections = false;
                let mut f_primary_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_primary_rejections: Option<f64> = None;
                let mut seen_primary_rejections = false;
                let mut f_replica_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_replica_rejections: Option<f64> = None;
                let mut seen_replica_rejections = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "all_in_bytes" => {
                            if f_all_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("all_in_bytes"));
                            }
                            f_all_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "combined_coordinating_and_primary_in_bytes" => {
                            if f_combined_coordinating_and_primary_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "combined_coordinating_and_primary_in_bytes",
                                ));
                            }
                            f_combined_coordinating_and_primary_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "coordinating_in_bytes" => {
                            if f_coordinating_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "coordinating_in_bytes",
                                ));
                            }
                            f_coordinating_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "coordinating_rejections" => {
                            if seen_coordinating_rejections {
                                return Err(serde::de::Error::duplicate_field(
                                    "coordinating_rejections",
                                ));
                            }
                            seen_coordinating_rejections = true;
                            f_coordinating_rejections = map.next_value::<Option<f64>>()?;
                        }
                        "primary_in_bytes" => {
                            if f_primary_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("primary_in_bytes"));
                            }
                            f_primary_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "primary_rejections" => {
                            if seen_primary_rejections {
                                return Err(serde::de::Error::duplicate_field(
                                    "primary_rejections",
                                ));
                            }
                            seen_primary_rejections = true;
                            f_primary_rejections = map.next_value::<Option<f64>>()?;
                        }
                        "replica_in_bytes" => {
                            if f_replica_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("replica_in_bytes"));
                            }
                            f_replica_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "replica_rejections" => {
                            if seen_replica_rejections {
                                return Err(serde::de::Error::duplicate_field(
                                    "replica_rejections",
                                ));
                            }
                            seen_replica_rejections = true;
                            f_replica_rejections = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexingPressureMemorySummary {
                    all_in_bytes: f_all_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("all_in_bytes"))?,
                    combined_coordinating_and_primary_in_bytes:
                        f_combined_coordinating_and_primary_in_bytes.ok_or_else(|| {
                            serde::de::Error::missing_field(
                                "combined_coordinating_and_primary_in_bytes",
                            )
                        })?,
                    coordinating_in_bytes: f_coordinating_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("coordinating_in_bytes"))?,
                    coordinating_rejections: f_coordinating_rejections,
                    primary_in_bytes: f_primary_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("primary_in_bytes"))?,
                    primary_rejections: f_primary_rejections,
                    replica_in_bytes: f_replica_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("replica_in_bytes"))?,
                    replica_rejections: f_replica_rejections,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexingPressureMemorySummary {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("all_in_bytes", &self.all_in_bytes)?;
        map.serialize_entry(
            "combined_coordinating_and_primary_in_bytes",
            &self.combined_coordinating_and_primary_in_bytes,
        )?;
        map.serialize_entry("coordinating_in_bytes", &self.coordinating_in_bytes)?;
        if let Some(v) = &self.coordinating_rejections {
            map.serialize_entry("coordinating_rejections", v)?;
        }
        map.serialize_entry("primary_in_bytes", &self.primary_in_bytes)?;
        if let Some(v) = &self.primary_rejections {
            map.serialize_entry("primary_rejections", v)?;
        }
        map.serialize_entry("replica_in_bytes", &self.replica_in_bytes)?;
        if let Some(v) = &self.replica_rejections {
            map.serialize_entry("replica_rejections", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IndicesVersions {
    pub index_count: i32,
    pub primary_shard_count: i32,
    pub total_primary_bytes: crate::types::common::ByteCount,
    pub version: crate::types::common::VersionString,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndicesVersions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndicesVersions;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndicesVersions object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_index_count: Option<i32> = None;
                let mut f_primary_shard_count: Option<i32> = None;
                let mut f_total_primary_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "index_count" => {
                            if f_index_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("index_count"));
                            }
                            f_index_count = Some(map.next_value::<i32>()?);
                        }
                        "primary_shard_count" => {
                            if f_primary_shard_count.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "primary_shard_count",
                                ));
                            }
                            f_primary_shard_count = Some(map.next_value::<i32>()?);
                        }
                        "total_primary_bytes" => {
                            if f_total_primary_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_primary_bytes",
                                ));
                            }
                            f_total_primary_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "version" => {
                            if f_version.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            f_version =
                                Some(map.next_value::<crate::types::common::VersionString>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndicesVersions {
                    index_count: f_index_count
                        .ok_or_else(|| serde::de::Error::missing_field("index_count"))?,
                    primary_shard_count: f_primary_shard_count
                        .ok_or_else(|| serde::de::Error::missing_field("primary_shard_count"))?,
                    total_primary_bytes: f_total_primary_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("total_primary_bytes"))?,
                    version: f_version.ok_or_else(|| serde::de::Error::missing_field("version"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndicesVersions {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("index_count", &self.index_count)?;
        map.serialize_entry("primary_shard_count", &self.primary_shard_count)?;
        map.serialize_entry("total_primary_bytes", &self.total_primary_bytes)?;
        map.serialize_entry("version", &self.version)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NodePackagingType {
    #[doc = "Number of selected nodes using the distribution flavor and file type."]
    pub count: i32,
    #[doc = "Type of OpenSearch distribution. This is always `default`."]
    pub flavor: Option<String>,
    #[doc = "File type (such as `tar` or `zip`) used for the distribution package."]
    pub r#type: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodePackagingType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodePackagingType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodePackagingType object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_count: Option<i32> = None;
                let mut f_flavor: Option<String> = None;
                let mut seen_flavor = false;
                let mut f_type: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "count" => {
                            if f_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            f_count = Some(map.next_value::<i32>()?);
                        }
                        "flavor" => {
                            if seen_flavor {
                                return Err(serde::de::Error::duplicate_field("flavor"));
                            }
                            seen_flavor = true;
                            f_flavor = map.next_value::<Option<String>>()?;
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
                Ok(NodePackagingType {
                    count: f_count.ok_or_else(|| serde::de::Error::missing_field("count"))?,
                    flavor: f_flavor,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodePackagingType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("count", &self.count)?;
        if let Some(v) = &self.flavor {
            map.serialize_entry("flavor", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct OperatingSystemMemoryInfo {
    #[doc = "Total amount, in bytes, of memory across all selected nodes, but using the value specified using the `es.total_memory_bytes` system property instead of measured total memory for those nodes where that system property was set."]
    pub adjusted_total_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Amount, in bytes, of free physical memory across all selected nodes."]
    pub free: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Amount, in bytes, of free physical memory across all selected nodes."]
    pub free_in_bytes: crate::types::common::ByteCount,
    #[doc = "Percentage of free physical memory across all selected nodes."]
    pub free_percent: crate::types::common::PercentageNumber,
    #[doc = "Total amount of physical memory across all selected nodes."]
    pub total: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total amount, in bytes, of physical memory across all selected nodes."]
    pub total_in_bytes: crate::types::common::ByteCount,
    #[doc = "Amount of physical memory in use across all selected nodes."]
    pub used: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Amount, in bytes, of physical memory in use across all selected nodes."]
    pub used_in_bytes: crate::types::common::ByteCount,
    #[doc = "Percentage of physical memory in use across all selected nodes."]
    pub used_percent: crate::types::common::PercentageNumber,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for OperatingSystemMemoryInfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = OperatingSystemMemoryInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a OperatingSystemMemoryInfo object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_adjusted_total_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_adjusted_total_in_bytes = false;
                let mut f_free: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_free = false;
                let mut f_free_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_free_percent: Option<crate::types::common::PercentageNumber> = None;
                let mut f_total: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_total = false;
                let mut f_total_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_used: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_used = false;
                let mut f_used_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_used_percent: Option<crate::types::common::PercentageNumber> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "adjusted_total_in_bytes" => {
                            if seen_adjusted_total_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "adjusted_total_in_bytes",
                                ));
                            }
                            seen_adjusted_total_in_bytes = true;
                            f_adjusted_total_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "free" => {
                            if seen_free {
                                return Err(serde::de::Error::duplicate_field("free"));
                            }
                            seen_free = true;
                            f_free = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "free_in_bytes" => {
                            if f_free_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("free_in_bytes"));
                            }
                            f_free_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "free_percent" => {
                            if f_free_percent.is_some() {
                                return Err(serde::de::Error::duplicate_field("free_percent"));
                            }
                            f_free_percent =
                                Some(map.next_value::<crate::types::common::PercentageNumber>()?);
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "total_in_bytes" => {
                            if f_total_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("total_in_bytes"));
                            }
                            f_total_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "used" => {
                            if seen_used {
                                return Err(serde::de::Error::duplicate_field("used"));
                            }
                            seen_used = true;
                            f_used = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "used_in_bytes" => {
                            if f_used_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("used_in_bytes"));
                            }
                            f_used_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "used_percent" => {
                            if f_used_percent.is_some() {
                                return Err(serde::de::Error::duplicate_field("used_percent"));
                            }
                            f_used_percent =
                                Some(map.next_value::<crate::types::common::PercentageNumber>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(OperatingSystemMemoryInfo {
                    adjusted_total_in_bytes: f_adjusted_total_in_bytes,
                    free: f_free,
                    free_in_bytes: f_free_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("free_in_bytes"))?,
                    free_percent: f_free_percent
                        .ok_or_else(|| serde::de::Error::missing_field("free_percent"))?,
                    total: f_total,
                    total_in_bytes: f_total_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("total_in_bytes"))?,
                    used: f_used,
                    used_in_bytes: f_used_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("used_in_bytes"))?,
                    used_percent: f_used_percent
                        .ok_or_else(|| serde::de::Error::missing_field("used_percent"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for OperatingSystemMemoryInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.adjusted_total_in_bytes {
            map.serialize_entry("adjusted_total_in_bytes", v)?;
        }
        if let Some(v) = &self.free {
            map.serialize_entry("free", v)?;
        }
        map.serialize_entry("free_in_bytes", &self.free_in_bytes)?;
        map.serialize_entry("free_percent", &self.free_percent)?;
        if let Some(v) = &self.total {
            map.serialize_entry("total", v)?;
        }
        map.serialize_entry("total_in_bytes", &self.total_in_bytes)?;
        if let Some(v) = &self.used {
            map.serialize_entry("used", v)?;
        }
        map.serialize_entry("used_in_bytes", &self.used_in_bytes)?;
        map.serialize_entry("used_percent", &self.used_percent)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `cluster.stats` API"]
pub type StatsResponse = crate::types::cluster::StatsResponse;
