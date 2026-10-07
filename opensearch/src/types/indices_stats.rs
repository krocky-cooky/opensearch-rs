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

//! Response types of the `indices_stats` specification namespace

#![allow(missing_docs)]

#[doc = "The statistics for all indexes."]
#[derive(Debug, Clone)]
pub struct AllIndicesStats {
    pub primaries: crate::types::indices_stats::IndexStats,
    pub total: crate::types::indices_stats::IndexStats,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AllIndicesStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AllIndicesStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AllIndicesStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_primaries: Option<crate::types::indices_stats::IndexStats> = None;
                let mut f_total: Option<crate::types::indices_stats::IndexStats> = None;
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
                            f_primaries =
                                Some(map.next_value::<crate::types::indices_stats::IndexStats>()?);
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total =
                                Some(map.next_value::<crate::types::indices_stats::IndexStats>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AllIndicesStats {
                    primaries: f_primaries
                        .ok_or_else(|| serde::de::Error::missing_field("primaries"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AllIndicesStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("primaries", &self.primaries)?;
        map.serialize_entry("total", &self.total)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IndexShardStats {
    pub completion: Option<crate::types::common::CompletionStats>,
    pub docs: Option<crate::types::common::DocStats>,
    pub fielddata: Option<crate::types::common::FielddataStats>,
    pub flush: Option<crate::types::common::FlushStats>,
    pub get: Option<crate::types::common::GetStats>,
    pub indexing: Option<crate::types::common::IndexingStats>,
    pub merges: Option<crate::types::common::MergesStats>,
    pub query_cache: Option<crate::types::common::QueryCacheStats>,
    pub recovery: Option<crate::types::common::RecoveryStats>,
    pub refresh: Option<crate::types::common::RefreshStats>,
    pub request_cache: Option<crate::types::common::RequestCacheStats>,
    pub search: Option<crate::types::common::SearchStats>,
    pub segments: Option<crate::types::common::SegmentsStats>,
    pub store: Option<crate::types::common::StoreStats>,
    pub translog: Option<crate::types::common::TranslogStats>,
    pub warmer: Option<crate::types::common::WarmerStats>,
    pub commit: Option<crate::types::indices_stats::ShardCommit>,
    pub retention_leases: Option<crate::types::indices_stats::ShardRetentionLeases>,
    pub routing: Option<crate::types::indices_stats::ShardRouting>,
    pub seq_no: Option<crate::types::indices_stats::ShardSequenceNumber>,
    pub shard_path: Option<crate::types::indices_stats::ShardPath>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexShardStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexShardStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexShardStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_completion: Option<crate::types::common::CompletionStats> = None;
                let mut seen_completion = false;
                let mut f_docs: Option<crate::types::common::DocStats> = None;
                let mut seen_docs = false;
                let mut f_fielddata: Option<crate::types::common::FielddataStats> = None;
                let mut seen_fielddata = false;
                let mut f_flush: Option<crate::types::common::FlushStats> = None;
                let mut seen_flush = false;
                let mut f_get: Option<crate::types::common::GetStats> = None;
                let mut seen_get = false;
                let mut f_indexing: Option<crate::types::common::IndexingStats> = None;
                let mut seen_indexing = false;
                let mut f_merges: Option<crate::types::common::MergesStats> = None;
                let mut seen_merges = false;
                let mut f_query_cache: Option<crate::types::common::QueryCacheStats> = None;
                let mut seen_query_cache = false;
                let mut f_recovery: Option<crate::types::common::RecoveryStats> = None;
                let mut seen_recovery = false;
                let mut f_refresh: Option<crate::types::common::RefreshStats> = None;
                let mut seen_refresh = false;
                let mut f_request_cache: Option<crate::types::common::RequestCacheStats> = None;
                let mut seen_request_cache = false;
                let mut f_search: Option<crate::types::common::SearchStats> = None;
                let mut seen_search = false;
                let mut f_segments: Option<crate::types::common::SegmentsStats> = None;
                let mut seen_segments = false;
                let mut f_store: Option<crate::types::common::StoreStats> = None;
                let mut seen_store = false;
                let mut f_translog: Option<crate::types::common::TranslogStats> = None;
                let mut seen_translog = false;
                let mut f_warmer: Option<crate::types::common::WarmerStats> = None;
                let mut seen_warmer = false;
                let mut f_commit: Option<crate::types::indices_stats::ShardCommit> = None;
                let mut seen_commit = false;
                let mut f_retention_leases: Option<
                    crate::types::indices_stats::ShardRetentionLeases,
                > = None;
                let mut seen_retention_leases = false;
                let mut f_routing: Option<crate::types::indices_stats::ShardRouting> = None;
                let mut seen_routing = false;
                let mut f_seq_no: Option<crate::types::indices_stats::ShardSequenceNumber> = None;
                let mut seen_seq_no = false;
                let mut f_shard_path: Option<crate::types::indices_stats::ShardPath> = None;
                let mut seen_shard_path = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "completion" => {
                            if seen_completion {
                                return Err(serde::de::Error::duplicate_field("completion"));
                            }
                            seen_completion = true;
                            f_completion =
                                map.next_value::<Option<crate::types::common::CompletionStats>>()?;
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
                        "flush" => {
                            if seen_flush {
                                return Err(serde::de::Error::duplicate_field("flush"));
                            }
                            seen_flush = true;
                            f_flush =
                                map.next_value::<Option<crate::types::common::FlushStats>>()?;
                        }
                        "get" => {
                            if seen_get {
                                return Err(serde::de::Error::duplicate_field("get"));
                            }
                            seen_get = true;
                            f_get = map.next_value::<Option<crate::types::common::GetStats>>()?;
                        }
                        "indexing" => {
                            if seen_indexing {
                                return Err(serde::de::Error::duplicate_field("indexing"));
                            }
                            seen_indexing = true;
                            f_indexing =
                                map.next_value::<Option<crate::types::common::IndexingStats>>()?;
                        }
                        "merges" => {
                            if seen_merges {
                                return Err(serde::de::Error::duplicate_field("merges"));
                            }
                            seen_merges = true;
                            f_merges =
                                map.next_value::<Option<crate::types::common::MergesStats>>()?;
                        }
                        "query_cache" => {
                            if seen_query_cache {
                                return Err(serde::de::Error::duplicate_field("query_cache"));
                            }
                            seen_query_cache = true;
                            f_query_cache =
                                map.next_value::<Option<crate::types::common::QueryCacheStats>>()?;
                        }
                        "recovery" => {
                            if seen_recovery {
                                return Err(serde::de::Error::duplicate_field("recovery"));
                            }
                            seen_recovery = true;
                            f_recovery =
                                map.next_value::<Option<crate::types::common::RecoveryStats>>()?;
                        }
                        "refresh" => {
                            if seen_refresh {
                                return Err(serde::de::Error::duplicate_field("refresh"));
                            }
                            seen_refresh = true;
                            f_refresh =
                                map.next_value::<Option<crate::types::common::RefreshStats>>()?;
                        }
                        "request_cache" => {
                            if seen_request_cache {
                                return Err(serde::de::Error::duplicate_field("request_cache"));
                            }
                            seen_request_cache = true;
                            f_request_cache = map
                                .next_value::<Option<crate::types::common::RequestCacheStats>>()?;
                        }
                        "search" => {
                            if seen_search {
                                return Err(serde::de::Error::duplicate_field("search"));
                            }
                            seen_search = true;
                            f_search =
                                map.next_value::<Option<crate::types::common::SearchStats>>()?;
                        }
                        "segments" => {
                            if seen_segments {
                                return Err(serde::de::Error::duplicate_field("segments"));
                            }
                            seen_segments = true;
                            f_segments =
                                map.next_value::<Option<crate::types::common::SegmentsStats>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store =
                                map.next_value::<Option<crate::types::common::StoreStats>>()?;
                        }
                        "translog" => {
                            if seen_translog {
                                return Err(serde::de::Error::duplicate_field("translog"));
                            }
                            seen_translog = true;
                            f_translog =
                                map.next_value::<Option<crate::types::common::TranslogStats>>()?;
                        }
                        "warmer" => {
                            if seen_warmer {
                                return Err(serde::de::Error::duplicate_field("warmer"));
                            }
                            seen_warmer = true;
                            f_warmer =
                                map.next_value::<Option<crate::types::common::WarmerStats>>()?;
                        }
                        "commit" => {
                            if seen_commit {
                                return Err(serde::de::Error::duplicate_field("commit"));
                            }
                            seen_commit = true;
                            f_commit = map
                                .next_value::<Option<crate::types::indices_stats::ShardCommit>>()?;
                        }
                        "retention_leases" => {
                            if seen_retention_leases {
                                return Err(serde::de::Error::duplicate_field("retention_leases"));
                            }
                            seen_retention_leases = true;
                            f_retention_leases = map.next_value::<Option<crate::types::indices_stats::ShardRetentionLeases>>()?;
                        }
                        "routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("routing"));
                            }
                            seen_routing = true;
                            f_routing = map
                                .next_value::<Option<crate::types::indices_stats::ShardRouting>>(
                                )?;
                        }
                        "seq_no" => {
                            if seen_seq_no {
                                return Err(serde::de::Error::duplicate_field("seq_no"));
                            }
                            seen_seq_no = true;
                            f_seq_no = map.next_value::<Option<crate::types::indices_stats::ShardSequenceNumber>>()?;
                        }
                        "shard_path" => {
                            if seen_shard_path {
                                return Err(serde::de::Error::duplicate_field("shard_path"));
                            }
                            seen_shard_path = true;
                            f_shard_path =
                                map.next_value::<Option<crate::types::indices_stats::ShardPath>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexShardStats {
                    completion: f_completion,
                    docs: f_docs,
                    fielddata: f_fielddata,
                    flush: f_flush,
                    get: f_get,
                    indexing: f_indexing,
                    merges: f_merges,
                    query_cache: f_query_cache,
                    recovery: f_recovery,
                    refresh: f_refresh,
                    request_cache: f_request_cache,
                    search: f_search,
                    segments: f_segments,
                    store: f_store,
                    translog: f_translog,
                    warmer: f_warmer,
                    commit: f_commit,
                    retention_leases: f_retention_leases,
                    routing: f_routing,
                    seq_no: f_seq_no,
                    shard_path: f_shard_path,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexShardStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.completion {
            map.serialize_entry("completion", v)?;
        }
        if let Some(v) = &self.docs {
            map.serialize_entry("docs", v)?;
        }
        if let Some(v) = &self.fielddata {
            map.serialize_entry("fielddata", v)?;
        }
        if let Some(v) = &self.flush {
            map.serialize_entry("flush", v)?;
        }
        if let Some(v) = &self.get {
            map.serialize_entry("get", v)?;
        }
        if let Some(v) = &self.indexing {
            map.serialize_entry("indexing", v)?;
        }
        if let Some(v) = &self.merges {
            map.serialize_entry("merges", v)?;
        }
        if let Some(v) = &self.query_cache {
            map.serialize_entry("query_cache", v)?;
        }
        if let Some(v) = &self.recovery {
            map.serialize_entry("recovery", v)?;
        }
        if let Some(v) = &self.refresh {
            map.serialize_entry("refresh", v)?;
        }
        if let Some(v) = &self.request_cache {
            map.serialize_entry("request_cache", v)?;
        }
        if let Some(v) = &self.search {
            map.serialize_entry("search", v)?;
        }
        if let Some(v) = &self.segments {
            map.serialize_entry("segments", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.translog {
            map.serialize_entry("translog", v)?;
        }
        if let Some(v) = &self.warmer {
            map.serialize_entry("warmer", v)?;
        }
        if let Some(v) = &self.commit {
            map.serialize_entry("commit", v)?;
        }
        if let Some(v) = &self.retention_leases {
            map.serialize_entry("retention_leases", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("routing", v)?;
        }
        if let Some(v) = &self.seq_no {
            map.serialize_entry("seq_no", v)?;
        }
        if let Some(v) = &self.shard_path {
            map.serialize_entry("shard_path", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IndexStats {
    pub completion: Option<crate::types::common::CompletionStats>,
    pub docs: Option<crate::types::common::DocStats>,
    pub fielddata: Option<crate::types::common::FielddataStats>,
    pub flush: Option<crate::types::common::FlushStats>,
    pub get: Option<crate::types::common::GetStats>,
    pub indexing: Option<crate::types::common::IndexingStats>,
    pub merges: Option<crate::types::common::MergesStats>,
    pub query_cache: Option<crate::types::common::QueryCacheStats>,
    pub recovery: Option<crate::types::common::RecoveryStats>,
    pub refresh: Option<crate::types::common::RefreshStats>,
    pub request_cache: Option<crate::types::common::RequestCacheStats>,
    pub search: Option<crate::types::common::SearchStats>,
    pub segments: Option<crate::types::common::SegmentsStats>,
    pub store: Option<crate::types::common::StoreStats>,
    pub translog: Option<crate::types::common::TranslogStats>,
    pub warmer: Option<crate::types::common::WarmerStats>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_completion: Option<crate::types::common::CompletionStats> = None;
                let mut seen_completion = false;
                let mut f_docs: Option<crate::types::common::DocStats> = None;
                let mut seen_docs = false;
                let mut f_fielddata: Option<crate::types::common::FielddataStats> = None;
                let mut seen_fielddata = false;
                let mut f_flush: Option<crate::types::common::FlushStats> = None;
                let mut seen_flush = false;
                let mut f_get: Option<crate::types::common::GetStats> = None;
                let mut seen_get = false;
                let mut f_indexing: Option<crate::types::common::IndexingStats> = None;
                let mut seen_indexing = false;
                let mut f_merges: Option<crate::types::common::MergesStats> = None;
                let mut seen_merges = false;
                let mut f_query_cache: Option<crate::types::common::QueryCacheStats> = None;
                let mut seen_query_cache = false;
                let mut f_recovery: Option<crate::types::common::RecoveryStats> = None;
                let mut seen_recovery = false;
                let mut f_refresh: Option<crate::types::common::RefreshStats> = None;
                let mut seen_refresh = false;
                let mut f_request_cache: Option<crate::types::common::RequestCacheStats> = None;
                let mut seen_request_cache = false;
                let mut f_search: Option<crate::types::common::SearchStats> = None;
                let mut seen_search = false;
                let mut f_segments: Option<crate::types::common::SegmentsStats> = None;
                let mut seen_segments = false;
                let mut f_store: Option<crate::types::common::StoreStats> = None;
                let mut seen_store = false;
                let mut f_translog: Option<crate::types::common::TranslogStats> = None;
                let mut seen_translog = false;
                let mut f_warmer: Option<crate::types::common::WarmerStats> = None;
                let mut seen_warmer = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "completion" => {
                            if seen_completion {
                                return Err(serde::de::Error::duplicate_field("completion"));
                            }
                            seen_completion = true;
                            f_completion =
                                map.next_value::<Option<crate::types::common::CompletionStats>>()?;
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
                        "flush" => {
                            if seen_flush {
                                return Err(serde::de::Error::duplicate_field("flush"));
                            }
                            seen_flush = true;
                            f_flush =
                                map.next_value::<Option<crate::types::common::FlushStats>>()?;
                        }
                        "get" => {
                            if seen_get {
                                return Err(serde::de::Error::duplicate_field("get"));
                            }
                            seen_get = true;
                            f_get = map.next_value::<Option<crate::types::common::GetStats>>()?;
                        }
                        "indexing" => {
                            if seen_indexing {
                                return Err(serde::de::Error::duplicate_field("indexing"));
                            }
                            seen_indexing = true;
                            f_indexing =
                                map.next_value::<Option<crate::types::common::IndexingStats>>()?;
                        }
                        "merges" => {
                            if seen_merges {
                                return Err(serde::de::Error::duplicate_field("merges"));
                            }
                            seen_merges = true;
                            f_merges =
                                map.next_value::<Option<crate::types::common::MergesStats>>()?;
                        }
                        "query_cache" => {
                            if seen_query_cache {
                                return Err(serde::de::Error::duplicate_field("query_cache"));
                            }
                            seen_query_cache = true;
                            f_query_cache =
                                map.next_value::<Option<crate::types::common::QueryCacheStats>>()?;
                        }
                        "recovery" => {
                            if seen_recovery {
                                return Err(serde::de::Error::duplicate_field("recovery"));
                            }
                            seen_recovery = true;
                            f_recovery =
                                map.next_value::<Option<crate::types::common::RecoveryStats>>()?;
                        }
                        "refresh" => {
                            if seen_refresh {
                                return Err(serde::de::Error::duplicate_field("refresh"));
                            }
                            seen_refresh = true;
                            f_refresh =
                                map.next_value::<Option<crate::types::common::RefreshStats>>()?;
                        }
                        "request_cache" => {
                            if seen_request_cache {
                                return Err(serde::de::Error::duplicate_field("request_cache"));
                            }
                            seen_request_cache = true;
                            f_request_cache = map
                                .next_value::<Option<crate::types::common::RequestCacheStats>>()?;
                        }
                        "search" => {
                            if seen_search {
                                return Err(serde::de::Error::duplicate_field("search"));
                            }
                            seen_search = true;
                            f_search =
                                map.next_value::<Option<crate::types::common::SearchStats>>()?;
                        }
                        "segments" => {
                            if seen_segments {
                                return Err(serde::de::Error::duplicate_field("segments"));
                            }
                            seen_segments = true;
                            f_segments =
                                map.next_value::<Option<crate::types::common::SegmentsStats>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store =
                                map.next_value::<Option<crate::types::common::StoreStats>>()?;
                        }
                        "translog" => {
                            if seen_translog {
                                return Err(serde::de::Error::duplicate_field("translog"));
                            }
                            seen_translog = true;
                            f_translog =
                                map.next_value::<Option<crate::types::common::TranslogStats>>()?;
                        }
                        "warmer" => {
                            if seen_warmer {
                                return Err(serde::de::Error::duplicate_field("warmer"));
                            }
                            seen_warmer = true;
                            f_warmer =
                                map.next_value::<Option<crate::types::common::WarmerStats>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexStats {
                    completion: f_completion,
                    docs: f_docs,
                    fielddata: f_fielddata,
                    flush: f_flush,
                    get: f_get,
                    indexing: f_indexing,
                    merges: f_merges,
                    query_cache: f_query_cache,
                    recovery: f_recovery,
                    refresh: f_refresh,
                    request_cache: f_request_cache,
                    search: f_search,
                    segments: f_segments,
                    store: f_store,
                    translog: f_translog,
                    warmer: f_warmer,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.completion {
            map.serialize_entry("completion", v)?;
        }
        if let Some(v) = &self.docs {
            map.serialize_entry("docs", v)?;
        }
        if let Some(v) = &self.fielddata {
            map.serialize_entry("fielddata", v)?;
        }
        if let Some(v) = &self.flush {
            map.serialize_entry("flush", v)?;
        }
        if let Some(v) = &self.get {
            map.serialize_entry("get", v)?;
        }
        if let Some(v) = &self.indexing {
            map.serialize_entry("indexing", v)?;
        }
        if let Some(v) = &self.merges {
            map.serialize_entry("merges", v)?;
        }
        if let Some(v) = &self.query_cache {
            map.serialize_entry("query_cache", v)?;
        }
        if let Some(v) = &self.recovery {
            map.serialize_entry("recovery", v)?;
        }
        if let Some(v) = &self.refresh {
            map.serialize_entry("refresh", v)?;
        }
        if let Some(v) = &self.request_cache {
            map.serialize_entry("request_cache", v)?;
        }
        if let Some(v) = &self.search {
            map.serialize_entry("search", v)?;
        }
        if let Some(v) = &self.segments {
            map.serialize_entry("segments", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.translog {
            map.serialize_entry("translog", v)?;
        }
        if let Some(v) = &self.warmer {
            map.serialize_entry("warmer", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The statistics for specific indexes."]
#[derive(Debug, Clone)]
pub struct IndicesStats {
    pub primaries: crate::types::indices_stats::IndexStats,
    #[doc = "The statistics for individual shards."]
    pub shards: Option<
        std::collections::BTreeMap<String, Vec<crate::types::indices_stats::IndexShardStats>>,
    >,
    pub total: crate::types::indices_stats::IndexStats,
    pub uuid: crate::types::common::Uuid,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndicesStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndicesStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndicesStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_primaries: Option<crate::types::indices_stats::IndexStats> = None;
                let mut f_shards: Option<
                    std::collections::BTreeMap<
                        String,
                        Vec<crate::types::indices_stats::IndexShardStats>,
                    >,
                > = None;
                let mut seen_shards = false;
                let mut f_total: Option<crate::types::indices_stats::IndexStats> = None;
                let mut f_uuid: Option<crate::types::common::Uuid> = None;
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
                            f_primaries =
                                Some(map.next_value::<crate::types::indices_stats::IndexStats>()?);
                        }
                        "shards" => {
                            if seen_shards {
                                return Err(serde::de::Error::duplicate_field("shards"));
                            }
                            seen_shards = true;
                            f_shards = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Vec<crate::types::indices_stats::IndexShardStats>,
                                >,
                            >>()?;
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total =
                                Some(map.next_value::<crate::types::indices_stats::IndexStats>()?);
                        }
                        "uuid" => {
                            if f_uuid.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            f_uuid = Some(map.next_value::<crate::types::common::Uuid>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndicesStats {
                    primaries: f_primaries
                        .ok_or_else(|| serde::de::Error::missing_field("primaries"))?,
                    shards: f_shards,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    uuid: f_uuid.ok_or_else(|| serde::de::Error::missing_field("uuid"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndicesStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("primaries", &self.primaries)?;
        if let Some(v) = &self.shards {
            map.serialize_entry("shards", v)?;
        }
        map.serialize_entry("total", &self.total)?;
        map.serialize_entry("uuid", &self.uuid)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The commit information for a shard."]
#[derive(Debug, Clone)]
pub struct ShardCommit {
    #[doc = "The generation number of the commit."]
    pub generation: i32,
    pub id: crate::types::common::Id,
    #[doc = "The number of documents in the commit."]
    pub num_docs: i64,
    #[doc = "The user-defined data associated with the commit."]
    pub user_data: std::collections::BTreeMap<String, String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardCommit {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardCommit;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardCommit object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_generation: Option<i32> = None;
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut f_num_docs: Option<i64> = None;
                let mut f_user_data: Option<std::collections::BTreeMap<String, String>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "generation" => {
                            if f_generation.is_some() {
                                return Err(serde::de::Error::duplicate_field("generation"));
                            }
                            f_generation = Some(map.next_value::<i32>()?);
                        }
                        "id" => {
                            if f_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            f_id = Some(map.next_value::<crate::types::common::Id>()?);
                        }
                        "num_docs" => {
                            if f_num_docs.is_some() {
                                return Err(serde::de::Error::duplicate_field("num_docs"));
                            }
                            f_num_docs = Some(map.next_value::<i64>()?);
                        }
                        "user_data" => {
                            if f_user_data.is_some() {
                                return Err(serde::de::Error::duplicate_field("user_data"));
                            }
                            f_user_data = Some(
                                map.next_value::<std::collections::BTreeMap<String, String>>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardCommit {
                    generation: f_generation
                        .ok_or_else(|| serde::de::Error::missing_field("generation"))?,
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("id"))?,
                    num_docs: f_num_docs
                        .ok_or_else(|| serde::de::Error::missing_field("num_docs"))?,
                    user_data: f_user_data
                        .ok_or_else(|| serde::de::Error::missing_field("user_data"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardCommit {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("generation", &self.generation)?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("num_docs", &self.num_docs)?;
        map.serialize_entry("user_data", &self.user_data)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The size information for shard files."]
#[derive(Debug, Clone)]
pub struct ShardFileSizeInfo {
    pub average_size_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "The number of files."]
    pub count: Option<i64>,
    #[doc = "The description of the file type."]
    pub description: String,
    pub max_size_in_bytes: Option<crate::types::common::ByteCount>,
    pub min_size_in_bytes: Option<crate::types::common::ByteCount>,
    pub size: Option<crate::types::common::HumanReadableByteCount>,
    pub size_in_bytes: crate::types::common::ByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardFileSizeInfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardFileSizeInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardFileSizeInfo object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_average_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_average_size_in_bytes = false;
                let mut f_count: Option<i64> = None;
                let mut seen_count = false;
                let mut f_description: Option<String> = None;
                let mut f_max_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_max_size_in_bytes = false;
                let mut f_min_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_min_size_in_bytes = false;
                let mut f_size: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_size = false;
                let mut f_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "average_size_in_bytes" => {
                            if seen_average_size_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "average_size_in_bytes",
                                ));
                            }
                            seen_average_size_in_bytes = true;
                            f_average_size_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "count" => {
                            if seen_count {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            seen_count = true;
                            f_count = map.next_value::<Option<i64>>()?;
                        }
                        "description" => {
                            if f_description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            f_description = Some(map.next_value::<String>()?);
                        }
                        "max_size_in_bytes" => {
                            if seen_max_size_in_bytes {
                                return Err(serde::de::Error::duplicate_field("max_size_in_bytes"));
                            }
                            seen_max_size_in_bytes = true;
                            f_max_size_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "min_size_in_bytes" => {
                            if seen_min_size_in_bytes {
                                return Err(serde::de::Error::duplicate_field("min_size_in_bytes"));
                            }
                            seen_min_size_in_bytes = true;
                            f_min_size_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "size" => {
                            if seen_size {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            seen_size = true;
                            f_size = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "size_in_bytes" => {
                            if f_size_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("size_in_bytes"));
                            }
                            f_size_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardFileSizeInfo {
                    average_size_in_bytes: f_average_size_in_bytes,
                    count: f_count,
                    description: f_description
                        .ok_or_else(|| serde::de::Error::missing_field("description"))?,
                    max_size_in_bytes: f_max_size_in_bytes,
                    min_size_in_bytes: f_min_size_in_bytes,
                    size: f_size,
                    size_in_bytes: f_size_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("size_in_bytes"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardFileSizeInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.average_size_in_bytes {
            map.serialize_entry("average_size_in_bytes", v)?;
        }
        if let Some(v) = &self.count {
            map.serialize_entry("count", v)?;
        }
        map.serialize_entry("description", &self.description)?;
        if let Some(v) = &self.max_size_in_bytes {
            map.serialize_entry("max_size_in_bytes", v)?;
        }
        if let Some(v) = &self.min_size_in_bytes {
            map.serialize_entry("min_size_in_bytes", v)?;
        }
        if let Some(v) = &self.size {
            map.serialize_entry("size", v)?;
        }
        map.serialize_entry("size_in_bytes", &self.size_in_bytes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The retention lease details."]
#[derive(Debug, Clone)]
pub struct ShardLease {
    pub id: crate::types::common::Id,
    pub retaining_seq_no: crate::types::common::SequenceNumber,
    pub source: String,
    pub timestamp: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardLease {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardLease;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardLease object")
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
                let mut f_retaining_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut f_source: Option<String> = None;
                let mut f_timestamp: Option<i64> = None;
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
                        "retaining_seq_no" => {
                            if f_retaining_seq_no.is_some() {
                                return Err(serde::de::Error::duplicate_field("retaining_seq_no"));
                            }
                            f_retaining_seq_no =
                                Some(map.next_value::<crate::types::common::SequenceNumber>()?);
                        }
                        "source" => {
                            if f_source.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            f_source = Some(map.next_value::<String>()?);
                        }
                        "timestamp" => {
                            if f_timestamp.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            f_timestamp = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardLease {
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("id"))?,
                    retaining_seq_no: f_retaining_seq_no
                        .ok_or_else(|| serde::de::Error::missing_field("retaining_seq_no"))?,
                    source: f_source.ok_or_else(|| serde::de::Error::missing_field("source"))?,
                    timestamp: f_timestamp
                        .ok_or_else(|| serde::de::Error::missing_field("timestamp"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardLease {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("retaining_seq_no", &self.retaining_seq_no)?;
        map.serialize_entry("source", &self.source)?;
        map.serialize_entry("timestamp", &self.timestamp)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The path information for a shard."]
#[derive(Debug, Clone)]
pub struct ShardPath {
    #[doc = "The path to the shard's data."]
    pub data_path: String,
    #[doc = "Whether the shard uses a custom data path."]
    pub is_custom_data_path: bool,
    #[doc = "The path to the shard's state."]
    pub state_path: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardPath {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardPath;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardPath object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_data_path: Option<String> = None;
                let mut f_is_custom_data_path: Option<bool> = None;
                let mut f_state_path: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "data_path" => {
                            if f_data_path.is_some() {
                                return Err(serde::de::Error::duplicate_field("data_path"));
                            }
                            f_data_path = Some(map.next_value::<String>()?);
                        }
                        "is_custom_data_path" => {
                            if f_is_custom_data_path.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "is_custom_data_path",
                                ));
                            }
                            f_is_custom_data_path = Some(map.next_value::<bool>()?);
                        }
                        "state_path" => {
                            if f_state_path.is_some() {
                                return Err(serde::de::Error::duplicate_field("state_path"));
                            }
                            f_state_path = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardPath {
                    data_path: f_data_path
                        .ok_or_else(|| serde::de::Error::missing_field("data_path"))?,
                    is_custom_data_path: f_is_custom_data_path
                        .ok_or_else(|| serde::de::Error::missing_field("is_custom_data_path"))?,
                    state_path: f_state_path
                        .ok_or_else(|| serde::de::Error::missing_field("state_path"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardPath {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("data_path", &self.data_path)?;
        map.serialize_entry("is_custom_data_path", &self.is_custom_data_path)?;
        map.serialize_entry("state_path", &self.state_path)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The retention lease information for a shard."]
#[derive(Debug, Clone)]
pub struct ShardRetentionLeases {
    #[doc = "The list of retention leases."]
    pub leases: Vec<crate::types::indices_stats::ShardLease>,
    #[doc = "The primary term of the retention leases."]
    pub primary_term: i64,
    pub version: crate::types::common::VersionNumber,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardRetentionLeases {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardRetentionLeases;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardRetentionLeases object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_leases: Option<Vec<crate::types::indices_stats::ShardLease>> = None;
                let mut f_primary_term: Option<i64> = None;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "leases" => {
                            if f_leases.is_some() {
                                return Err(serde::de::Error::duplicate_field("leases"));
                            }
                            f_leases = Some(
                                map.next_value::<Vec<crate::types::indices_stats::ShardLease>>()?,
                            );
                        }
                        "primary_term" => {
                            if f_primary_term.is_some() {
                                return Err(serde::de::Error::duplicate_field("primary_term"));
                            }
                            f_primary_term = Some(map.next_value::<i64>()?);
                        }
                        "version" => {
                            if f_version.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            f_version =
                                Some(map.next_value::<crate::types::common::VersionNumber>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardRetentionLeases {
                    leases: f_leases.ok_or_else(|| serde::de::Error::missing_field("leases"))?,
                    primary_term: f_primary_term
                        .ok_or_else(|| serde::de::Error::missing_field("primary_term"))?,
                    version: f_version.ok_or_else(|| serde::de::Error::missing_field("version"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardRetentionLeases {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("leases", &self.leases)?;
        map.serialize_entry("primary_term", &self.primary_term)?;
        map.serialize_entry("version", &self.version)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The routing information for a shard."]
#[derive(Debug, Clone)]
pub struct ShardRouting {
    #[doc = "The node containing the shard."]
    pub node: String,
    #[doc = "Whether this is a primary shard."]
    pub primary: bool,
    #[doc = "The node where the shard is relocating to."]
    pub relocating_node: Option<String>,
    pub state: crate::types::indices_stats::ShardRoutingState,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardRouting {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardRouting;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardRouting object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_node: Option<String> = None;
                let mut f_primary: Option<bool> = None;
                let mut f_relocating_node: Option<String> = None;
                let mut seen_relocating_node = false;
                let mut f_state: Option<crate::types::indices_stats::ShardRoutingState> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "node" => {
                            if f_node.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            f_node = Some(map.next_value::<String>()?);
                        }
                        "primary" => {
                            if f_primary.is_some() {
                                return Err(serde::de::Error::duplicate_field("primary"));
                            }
                            f_primary = Some(map.next_value::<bool>()?);
                        }
                        "relocating_node" => {
                            if seen_relocating_node {
                                return Err(serde::de::Error::duplicate_field("relocating_node"));
                            }
                            seen_relocating_node = true;
                            f_relocating_node = map.next_value::<Option<String>>()?;
                        }
                        "state" => {
                            if f_state.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            f_state = Some(
                                map.next_value::<crate::types::indices_stats::ShardRoutingState>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardRouting {
                    node: f_node.ok_or_else(|| serde::de::Error::missing_field("node"))?,
                    primary: f_primary.ok_or_else(|| serde::de::Error::missing_field("primary"))?,
                    relocating_node: f_relocating_node,
                    state: f_state.ok_or_else(|| serde::de::Error::missing_field("state"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardRouting {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("node", &self.node)?;
        map.serialize_entry("primary", &self.primary)?;
        if let Some(v) = &self.relocating_node {
            map.serialize_entry("relocating_node", v)?;
        }
        map.serialize_entry("state", &self.state)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The state of shard routing."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShardRoutingState {
    #[doc = "`INITIALIZING`"]
    Initializing,
    #[doc = "`RELOCATING`"]
    Relocating,
    #[doc = "`STARTED`"]
    Started,
    #[doc = "`UNASSIGNED`"]
    Unassigned,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ShardRoutingState {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ShardRoutingState::Initializing => "INITIALIZING",
            ShardRoutingState::Relocating => "RELOCATING",
            ShardRoutingState::Started => "STARTED",
            ShardRoutingState::Unassigned => "UNASSIGNED",
            ShardRoutingState::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ShardRoutingState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShardRoutingState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "INITIALIZING" => ShardRoutingState::Initializing,
            "RELOCATING" => ShardRoutingState::Relocating,
            "STARTED" => ShardRoutingState::Started,
            "UNASSIGNED" => ShardRoutingState::Unassigned,
            _ => ShardRoutingState::Unknown(s),
        })
    }
}

#[doc = "The sequence number information for a shard."]
#[derive(Debug, Clone)]
pub struct ShardSequenceNumber {
    #[doc = "The global checkpoint for the shard."]
    pub global_checkpoint: i64,
    #[doc = "The local checkpoint for the shard."]
    pub local_checkpoint: i64,
    pub max_seq_no: crate::types::common::SequenceNumber,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardSequenceNumber {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardSequenceNumber;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardSequenceNumber object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_global_checkpoint: Option<i64> = None;
                let mut f_local_checkpoint: Option<i64> = None;
                let mut f_max_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "global_checkpoint" => {
                            if f_global_checkpoint.is_some() {
                                return Err(serde::de::Error::duplicate_field("global_checkpoint"));
                            }
                            f_global_checkpoint = Some(map.next_value::<i64>()?);
                        }
                        "local_checkpoint" => {
                            if f_local_checkpoint.is_some() {
                                return Err(serde::de::Error::duplicate_field("local_checkpoint"));
                            }
                            f_local_checkpoint = Some(map.next_value::<i64>()?);
                        }
                        "max_seq_no" => {
                            if f_max_seq_no.is_some() {
                                return Err(serde::de::Error::duplicate_field("max_seq_no"));
                            }
                            f_max_seq_no =
                                Some(map.next_value::<crate::types::common::SequenceNumber>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardSequenceNumber {
                    global_checkpoint: f_global_checkpoint
                        .ok_or_else(|| serde::de::Error::missing_field("global_checkpoint"))?,
                    local_checkpoint: f_local_checkpoint
                        .ok_or_else(|| serde::de::Error::missing_field("local_checkpoint"))?,
                    max_seq_no: f_max_seq_no
                        .ok_or_else(|| serde::de::Error::missing_field("max_seq_no"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardSequenceNumber {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("global_checkpoint", &self.global_checkpoint)?;
        map.serialize_entry("local_checkpoint", &self.local_checkpoint)?;
        map.serialize_entry("max_seq_no", &self.max_seq_no)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `indices.stats` API"]
pub type StatsResponse = crate::types::indices::StatsResponse;
