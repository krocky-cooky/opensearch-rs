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

//! Response types of the `cat_indices` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct IndicesRecord {
    #[doc = "size of completion"]
    pub completion_size: Option<String>,
    #[doc = "index creation date (millisecond value)"]
    pub creation_date: Option<String>,
    #[doc = "index creation date (as string)"]
    pub creation_date_string: Option<String>,
    #[doc = "available docs"]
    pub docs_count: Option<String>,
    #[doc = "deleted docs"]
    pub docs_deleted: Option<String>,
    #[doc = "field data evictions"]
    pub fielddata_evictions: Option<String>,
    #[doc = "used field data cache"]
    pub fielddata_memory_size: Option<String>,
    #[doc = "number of flushes"]
    pub flush_total: Option<String>,
    #[doc = "time spent in flush"]
    pub flush_total_time: Option<String>,
    #[doc = "number of current get ops"]
    pub get_current: Option<String>,
    #[doc = "time spent in successful gets"]
    pub get_exists_time: Option<String>,
    #[doc = "number of successful gets"]
    pub get_exists_total: Option<String>,
    #[doc = "time spent in failed gets"]
    pub get_missing_time: Option<String>,
    #[doc = "number of failed gets"]
    pub get_missing_total: Option<String>,
    #[doc = "time spent in get"]
    pub get_time: Option<String>,
    #[doc = "number of get ops"]
    pub get_total: Option<String>,
    #[doc = "current health status"]
    pub health: Option<String>,
    #[doc = "index name"]
    pub index: Option<String>,
    #[doc = "number of current deletions"]
    pub indexing_delete_current: Option<String>,
    #[doc = "time spent in deletions"]
    pub indexing_delete_time: Option<String>,
    #[doc = "number of delete ops"]
    pub indexing_delete_total: Option<String>,
    #[doc = "number of current indexing ops"]
    pub indexing_index_current: Option<String>,
    #[doc = "number of failed indexing ops"]
    pub indexing_index_failed: Option<String>,
    #[doc = "time spent in indexing"]
    pub indexing_index_time: Option<String>,
    #[doc = "number of indexing ops"]
    pub indexing_index_total: Option<String>,
    #[doc = "total used memory"]
    pub memory_total: Option<String>,
    #[doc = "number of current merges"]
    pub merges_current: Option<String>,
    #[doc = "number of current merging docs"]
    pub merges_current_docs: Option<String>,
    #[doc = "size of current merges"]
    pub merges_current_size: Option<String>,
    #[doc = "number of completed merge ops"]
    pub merges_total: Option<String>,
    #[doc = "docs merged"]
    pub merges_total_docs: Option<String>,
    #[doc = "size merged"]
    pub merges_total_size: Option<String>,
    #[doc = "time spent in merges"]
    pub merges_total_time: Option<String>,
    #[doc = "number of primary shards"]
    pub pri: Option<String>,
    #[doc = "size of completion"]
    pub pri_completion_size: Option<String>,
    #[doc = "field data evictions"]
    pub pri_fielddata_evictions: Option<String>,
    #[doc = "used field data cache"]
    pub pri_fielddata_memory_size: Option<String>,
    #[doc = "number of flushes"]
    pub pri_flush_total: Option<String>,
    #[doc = "time spent in flush"]
    pub pri_flush_total_time: Option<String>,
    #[doc = "number of current get ops"]
    pub pri_get_current: Option<String>,
    #[doc = "time spent in successful gets"]
    pub pri_get_exists_time: Option<String>,
    #[doc = "number of successful gets"]
    pub pri_get_exists_total: Option<String>,
    #[doc = "time spent in failed gets"]
    pub pri_get_missing_time: Option<String>,
    #[doc = "number of failed gets"]
    pub pri_get_missing_total: Option<String>,
    #[doc = "time spent in get"]
    pub pri_get_time: Option<String>,
    #[doc = "number of get ops"]
    pub pri_get_total: Option<String>,
    #[doc = "number of current deletions"]
    pub pri_indexing_delete_current: Option<String>,
    #[doc = "time spent in deletions"]
    pub pri_indexing_delete_time: Option<String>,
    #[doc = "number of delete ops"]
    pub pri_indexing_delete_total: Option<String>,
    #[doc = "number of current indexing ops"]
    pub pri_indexing_index_current: Option<String>,
    #[doc = "number of failed indexing ops"]
    pub pri_indexing_index_failed: Option<String>,
    #[doc = "time spent in indexing"]
    pub pri_indexing_index_time: Option<String>,
    #[doc = "number of indexing ops"]
    pub pri_indexing_index_total: Option<String>,
    #[doc = "total user memory"]
    pub pri_memory_total: Option<String>,
    #[doc = "number of current merges"]
    pub pri_merges_current: Option<String>,
    #[doc = "number of current merging docs"]
    pub pri_merges_current_docs: Option<String>,
    #[doc = "size of current merges"]
    pub pri_merges_current_size: Option<String>,
    #[doc = "number of completed merge ops"]
    pub pri_merges_total: Option<String>,
    #[doc = "docs merged"]
    pub pri_merges_total_docs: Option<String>,
    #[doc = "size merged"]
    pub pri_merges_total_size: Option<String>,
    #[doc = "time spent in merges"]
    pub pri_merges_total_time: Option<String>,
    #[doc = "query cache evictions"]
    pub pri_query_cache_evictions: Option<String>,
    #[doc = "used query cache"]
    pub pri_query_cache_memory_size: Option<String>,
    #[doc = "time spent in external refreshes"]
    pub pri_refresh_external_time: Option<String>,
    #[doc = "total external refreshes"]
    pub pri_refresh_external_total: Option<String>,
    #[doc = "number of pending refresh listeners"]
    pub pri_refresh_listeners: Option<String>,
    #[doc = "time spent in refreshes"]
    pub pri_refresh_time: Option<String>,
    #[doc = "total refreshes"]
    pub pri_refresh_total: Option<String>,
    #[doc = "request cache evictions"]
    pub pri_request_cache_evictions: Option<String>,
    #[doc = "request cache hit count"]
    pub pri_request_cache_hit_count: Option<String>,
    #[doc = "used request cache"]
    pub pri_request_cache_memory_size: Option<String>,
    #[doc = "request cache miss count"]
    pub pri_request_cache_miss_count: Option<String>,
    pub pri_search_concurrent_avg_slice_count: Option<String>,
    pub pri_search_concurrent_query_current: Option<String>,
    pub pri_search_concurrent_query_time: Option<String>,
    pub pri_search_concurrent_query_total: Option<String>,
    #[doc = "current fetch phase ops"]
    pub pri_search_fetch_current: Option<String>,
    #[doc = "time spent in fetch phase"]
    pub pri_search_fetch_time: Option<String>,
    #[doc = "total fetch ops"]
    pub pri_search_fetch_total: Option<String>,
    #[doc = "open search contexts"]
    pub pri_search_open_contexts: Option<String>,
    pub pri_search_point_in_time_current: Option<String>,
    pub pri_search_point_in_time_time: Option<String>,
    pub pri_search_point_in_time_total: Option<String>,
    #[doc = "current query phase ops"]
    pub pri_search_query_current: Option<String>,
    #[doc = "time spent in query phase"]
    pub pri_search_query_time: Option<String>,
    #[doc = "total query phase ops"]
    pub pri_search_query_total: Option<String>,
    #[doc = "open scroll contexts"]
    pub pri_search_scroll_current: Option<String>,
    #[doc = "time scroll contexts held open"]
    pub pri_search_scroll_time: Option<String>,
    #[doc = "completed scroll contexts"]
    pub pri_search_scroll_total: Option<String>,
    #[doc = "number of segments"]
    pub pri_segments_count: Option<String>,
    #[doc = "memory used by fixed bit sets for nested object field types and export type filters for types referred in `_parent` fields"]
    pub pri_segments_fixed_bitset_memory: Option<String>,
    #[doc = "memory used by index writer"]
    pub pri_segments_index_writer_memory: Option<String>,
    #[doc = "memory used by segments"]
    pub pri_segments_memory: Option<String>,
    #[doc = "memory used by version map"]
    pub pri_segments_version_map_memory: Option<String>,
    #[doc = "store size of primaries"]
    pub pri_store_size: Option<String>,
    #[doc = "number of current suggest ops"]
    pub pri_suggest_current: Option<String>,
    #[doc = "time spend in suggest"]
    pub pri_suggest_time: Option<String>,
    #[doc = "number of suggest ops"]
    pub pri_suggest_total: Option<String>,
    #[doc = "current warmer ops"]
    pub pri_warmer_current: Option<String>,
    #[doc = "total warmer ops"]
    pub pri_warmer_total: Option<String>,
    #[doc = "time spent in warmers"]
    pub pri_warmer_total_time: Option<String>,
    #[doc = "query cache evictions"]
    pub query_cache_evictions: Option<String>,
    #[doc = "used query cache"]
    pub query_cache_memory_size: Option<String>,
    #[doc = "time spent in external refreshes"]
    pub refresh_external_time: Option<String>,
    #[doc = "total external refreshes"]
    pub refresh_external_total: Option<String>,
    #[doc = "number of pending refresh listeners"]
    pub refresh_listeners: Option<String>,
    #[doc = "time spent in refreshes"]
    pub refresh_time: Option<String>,
    #[doc = "total refreshes"]
    pub refresh_total: Option<String>,
    #[doc = "number of replica shards"]
    pub rep: Option<String>,
    #[doc = "request cache evictions"]
    pub request_cache_evictions: Option<String>,
    #[doc = "request cache hit count"]
    pub request_cache_hit_count: Option<String>,
    #[doc = "used request cache"]
    pub request_cache_memory_size: Option<String>,
    #[doc = "request cache miss count"]
    pub request_cache_miss_count: Option<String>,
    pub search_concurrent_avg_slice_count: Option<String>,
    pub search_concurrent_query_current: Option<String>,
    pub search_concurrent_query_time: Option<String>,
    pub search_concurrent_query_total: Option<String>,
    #[doc = "current fetch phase ops"]
    pub search_fetch_current: Option<String>,
    #[doc = "time spent in fetch phase"]
    pub search_fetch_time: Option<String>,
    #[doc = "total fetch ops"]
    pub search_fetch_total: Option<String>,
    #[doc = "open search contexts"]
    pub search_open_contexts: Option<String>,
    pub search_point_in_time_current: Option<String>,
    pub search_point_in_time_time: Option<String>,
    pub search_point_in_time_total: Option<String>,
    #[doc = "current query phase ops"]
    pub search_query_current: Option<String>,
    #[doc = "time spent in query phase"]
    pub search_query_time: Option<String>,
    #[doc = "total query phase ops"]
    pub search_query_total: Option<String>,
    #[doc = "open scroll contexts"]
    pub search_scroll_current: Option<String>,
    #[doc = "time scroll contexts held open"]
    pub search_scroll_time: Option<String>,
    #[doc = "completed scroll contexts"]
    pub search_scroll_total: Option<String>,
    #[doc = "indicates if the index is search throttled"]
    pub search_throttled: Option<String>,
    #[doc = "number of segments"]
    pub segments_count: Option<String>,
    #[doc = "memory used by fixed bit sets for nested object field types and export type filters for types referred in `_parent` fields"]
    pub segments_fixed_bitset_memory: Option<String>,
    #[doc = "memory used by index writer"]
    pub segments_index_writer_memory: Option<String>,
    #[doc = "memory used by segments"]
    pub segments_memory: Option<String>,
    #[doc = "memory used by version map"]
    pub segments_version_map_memory: Option<String>,
    #[doc = "open/close status"]
    pub status: Option<String>,
    #[doc = "store size of primaries and replicas"]
    pub store_size: Option<String>,
    #[doc = "number of current suggest ops"]
    pub suggest_current: Option<String>,
    #[doc = "time spend in suggest"]
    pub suggest_time: Option<String>,
    #[doc = "number of suggest ops"]
    pub suggest_total: Option<String>,
    #[doc = "index UUID"]
    pub uuid: Option<String>,
    #[doc = "current warmer ops"]
    pub warmer_current: Option<String>,
    #[doc = "total warmer ops"]
    pub warmer_total: Option<String>,
    #[doc = "time spent in warmers"]
    pub warmer_total_time: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndicesRecord {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndicesRecord;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndicesRecord object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_completion_size: Option<String> = None;
                let mut seen_completion_size = false;
                let mut f_creation_date: Option<String> = None;
                let mut seen_creation_date = false;
                let mut f_creation_date_string: Option<String> = None;
                let mut seen_creation_date_string = false;
                let mut f_docs_count: Option<String> = None;
                let mut seen_docs_count = false;
                let mut f_docs_deleted: Option<String> = None;
                let mut seen_docs_deleted = false;
                let mut f_fielddata_evictions: Option<String> = None;
                let mut seen_fielddata_evictions = false;
                let mut f_fielddata_memory_size: Option<String> = None;
                let mut seen_fielddata_memory_size = false;
                let mut f_flush_total: Option<String> = None;
                let mut seen_flush_total = false;
                let mut f_flush_total_time: Option<String> = None;
                let mut seen_flush_total_time = false;
                let mut f_get_current: Option<String> = None;
                let mut seen_get_current = false;
                let mut f_get_exists_time: Option<String> = None;
                let mut seen_get_exists_time = false;
                let mut f_get_exists_total: Option<String> = None;
                let mut seen_get_exists_total = false;
                let mut f_get_missing_time: Option<String> = None;
                let mut seen_get_missing_time = false;
                let mut f_get_missing_total: Option<String> = None;
                let mut seen_get_missing_total = false;
                let mut f_get_time: Option<String> = None;
                let mut seen_get_time = false;
                let mut f_get_total: Option<String> = None;
                let mut seen_get_total = false;
                let mut f_health: Option<String> = None;
                let mut seen_health = false;
                let mut f_index: Option<String> = None;
                let mut seen_index = false;
                let mut f_indexing_delete_current: Option<String> = None;
                let mut seen_indexing_delete_current = false;
                let mut f_indexing_delete_time: Option<String> = None;
                let mut seen_indexing_delete_time = false;
                let mut f_indexing_delete_total: Option<String> = None;
                let mut seen_indexing_delete_total = false;
                let mut f_indexing_index_current: Option<String> = None;
                let mut seen_indexing_index_current = false;
                let mut f_indexing_index_failed: Option<String> = None;
                let mut seen_indexing_index_failed = false;
                let mut f_indexing_index_time: Option<String> = None;
                let mut seen_indexing_index_time = false;
                let mut f_indexing_index_total: Option<String> = None;
                let mut seen_indexing_index_total = false;
                let mut f_memory_total: Option<String> = None;
                let mut seen_memory_total = false;
                let mut f_merges_current: Option<String> = None;
                let mut seen_merges_current = false;
                let mut f_merges_current_docs: Option<String> = None;
                let mut seen_merges_current_docs = false;
                let mut f_merges_current_size: Option<String> = None;
                let mut seen_merges_current_size = false;
                let mut f_merges_total: Option<String> = None;
                let mut seen_merges_total = false;
                let mut f_merges_total_docs: Option<String> = None;
                let mut seen_merges_total_docs = false;
                let mut f_merges_total_size: Option<String> = None;
                let mut seen_merges_total_size = false;
                let mut f_merges_total_time: Option<String> = None;
                let mut seen_merges_total_time = false;
                let mut f_pri: Option<String> = None;
                let mut seen_pri = false;
                let mut f_pri_completion_size: Option<String> = None;
                let mut seen_pri_completion_size = false;
                let mut f_pri_fielddata_evictions: Option<String> = None;
                let mut seen_pri_fielddata_evictions = false;
                let mut f_pri_fielddata_memory_size: Option<String> = None;
                let mut seen_pri_fielddata_memory_size = false;
                let mut f_pri_flush_total: Option<String> = None;
                let mut seen_pri_flush_total = false;
                let mut f_pri_flush_total_time: Option<String> = None;
                let mut seen_pri_flush_total_time = false;
                let mut f_pri_get_current: Option<String> = None;
                let mut seen_pri_get_current = false;
                let mut f_pri_get_exists_time: Option<String> = None;
                let mut seen_pri_get_exists_time = false;
                let mut f_pri_get_exists_total: Option<String> = None;
                let mut seen_pri_get_exists_total = false;
                let mut f_pri_get_missing_time: Option<String> = None;
                let mut seen_pri_get_missing_time = false;
                let mut f_pri_get_missing_total: Option<String> = None;
                let mut seen_pri_get_missing_total = false;
                let mut f_pri_get_time: Option<String> = None;
                let mut seen_pri_get_time = false;
                let mut f_pri_get_total: Option<String> = None;
                let mut seen_pri_get_total = false;
                let mut f_pri_indexing_delete_current: Option<String> = None;
                let mut seen_pri_indexing_delete_current = false;
                let mut f_pri_indexing_delete_time: Option<String> = None;
                let mut seen_pri_indexing_delete_time = false;
                let mut f_pri_indexing_delete_total: Option<String> = None;
                let mut seen_pri_indexing_delete_total = false;
                let mut f_pri_indexing_index_current: Option<String> = None;
                let mut seen_pri_indexing_index_current = false;
                let mut f_pri_indexing_index_failed: Option<String> = None;
                let mut seen_pri_indexing_index_failed = false;
                let mut f_pri_indexing_index_time: Option<String> = None;
                let mut seen_pri_indexing_index_time = false;
                let mut f_pri_indexing_index_total: Option<String> = None;
                let mut seen_pri_indexing_index_total = false;
                let mut f_pri_memory_total: Option<String> = None;
                let mut seen_pri_memory_total = false;
                let mut f_pri_merges_current: Option<String> = None;
                let mut seen_pri_merges_current = false;
                let mut f_pri_merges_current_docs: Option<String> = None;
                let mut seen_pri_merges_current_docs = false;
                let mut f_pri_merges_current_size: Option<String> = None;
                let mut seen_pri_merges_current_size = false;
                let mut f_pri_merges_total: Option<String> = None;
                let mut seen_pri_merges_total = false;
                let mut f_pri_merges_total_docs: Option<String> = None;
                let mut seen_pri_merges_total_docs = false;
                let mut f_pri_merges_total_size: Option<String> = None;
                let mut seen_pri_merges_total_size = false;
                let mut f_pri_merges_total_time: Option<String> = None;
                let mut seen_pri_merges_total_time = false;
                let mut f_pri_query_cache_evictions: Option<String> = None;
                let mut seen_pri_query_cache_evictions = false;
                let mut f_pri_query_cache_memory_size: Option<String> = None;
                let mut seen_pri_query_cache_memory_size = false;
                let mut f_pri_refresh_external_time: Option<String> = None;
                let mut seen_pri_refresh_external_time = false;
                let mut f_pri_refresh_external_total: Option<String> = None;
                let mut seen_pri_refresh_external_total = false;
                let mut f_pri_refresh_listeners: Option<String> = None;
                let mut seen_pri_refresh_listeners = false;
                let mut f_pri_refresh_time: Option<String> = None;
                let mut seen_pri_refresh_time = false;
                let mut f_pri_refresh_total: Option<String> = None;
                let mut seen_pri_refresh_total = false;
                let mut f_pri_request_cache_evictions: Option<String> = None;
                let mut seen_pri_request_cache_evictions = false;
                let mut f_pri_request_cache_hit_count: Option<String> = None;
                let mut seen_pri_request_cache_hit_count = false;
                let mut f_pri_request_cache_memory_size: Option<String> = None;
                let mut seen_pri_request_cache_memory_size = false;
                let mut f_pri_request_cache_miss_count: Option<String> = None;
                let mut seen_pri_request_cache_miss_count = false;
                let mut f_pri_search_concurrent_avg_slice_count: Option<String> = None;
                let mut seen_pri_search_concurrent_avg_slice_count = false;
                let mut f_pri_search_concurrent_query_current: Option<String> = None;
                let mut seen_pri_search_concurrent_query_current = false;
                let mut f_pri_search_concurrent_query_time: Option<String> = None;
                let mut seen_pri_search_concurrent_query_time = false;
                let mut f_pri_search_concurrent_query_total: Option<String> = None;
                let mut seen_pri_search_concurrent_query_total = false;
                let mut f_pri_search_fetch_current: Option<String> = None;
                let mut seen_pri_search_fetch_current = false;
                let mut f_pri_search_fetch_time: Option<String> = None;
                let mut seen_pri_search_fetch_time = false;
                let mut f_pri_search_fetch_total: Option<String> = None;
                let mut seen_pri_search_fetch_total = false;
                let mut f_pri_search_open_contexts: Option<String> = None;
                let mut seen_pri_search_open_contexts = false;
                let mut f_pri_search_point_in_time_current: Option<String> = None;
                let mut seen_pri_search_point_in_time_current = false;
                let mut f_pri_search_point_in_time_time: Option<String> = None;
                let mut seen_pri_search_point_in_time_time = false;
                let mut f_pri_search_point_in_time_total: Option<String> = None;
                let mut seen_pri_search_point_in_time_total = false;
                let mut f_pri_search_query_current: Option<String> = None;
                let mut seen_pri_search_query_current = false;
                let mut f_pri_search_query_time: Option<String> = None;
                let mut seen_pri_search_query_time = false;
                let mut f_pri_search_query_total: Option<String> = None;
                let mut seen_pri_search_query_total = false;
                let mut f_pri_search_scroll_current: Option<String> = None;
                let mut seen_pri_search_scroll_current = false;
                let mut f_pri_search_scroll_time: Option<String> = None;
                let mut seen_pri_search_scroll_time = false;
                let mut f_pri_search_scroll_total: Option<String> = None;
                let mut seen_pri_search_scroll_total = false;
                let mut f_pri_segments_count: Option<String> = None;
                let mut seen_pri_segments_count = false;
                let mut f_pri_segments_fixed_bitset_memory: Option<String> = None;
                let mut seen_pri_segments_fixed_bitset_memory = false;
                let mut f_pri_segments_index_writer_memory: Option<String> = None;
                let mut seen_pri_segments_index_writer_memory = false;
                let mut f_pri_segments_memory: Option<String> = None;
                let mut seen_pri_segments_memory = false;
                let mut f_pri_segments_version_map_memory: Option<String> = None;
                let mut seen_pri_segments_version_map_memory = false;
                let mut f_pri_store_size: Option<String> = None;
                let mut seen_pri_store_size = false;
                let mut f_pri_suggest_current: Option<String> = None;
                let mut seen_pri_suggest_current = false;
                let mut f_pri_suggest_time: Option<String> = None;
                let mut seen_pri_suggest_time = false;
                let mut f_pri_suggest_total: Option<String> = None;
                let mut seen_pri_suggest_total = false;
                let mut f_pri_warmer_current: Option<String> = None;
                let mut seen_pri_warmer_current = false;
                let mut f_pri_warmer_total: Option<String> = None;
                let mut seen_pri_warmer_total = false;
                let mut f_pri_warmer_total_time: Option<String> = None;
                let mut seen_pri_warmer_total_time = false;
                let mut f_query_cache_evictions: Option<String> = None;
                let mut seen_query_cache_evictions = false;
                let mut f_query_cache_memory_size: Option<String> = None;
                let mut seen_query_cache_memory_size = false;
                let mut f_refresh_external_time: Option<String> = None;
                let mut seen_refresh_external_time = false;
                let mut f_refresh_external_total: Option<String> = None;
                let mut seen_refresh_external_total = false;
                let mut f_refresh_listeners: Option<String> = None;
                let mut seen_refresh_listeners = false;
                let mut f_refresh_time: Option<String> = None;
                let mut seen_refresh_time = false;
                let mut f_refresh_total: Option<String> = None;
                let mut seen_refresh_total = false;
                let mut f_rep: Option<String> = None;
                let mut seen_rep = false;
                let mut f_request_cache_evictions: Option<String> = None;
                let mut seen_request_cache_evictions = false;
                let mut f_request_cache_hit_count: Option<String> = None;
                let mut seen_request_cache_hit_count = false;
                let mut f_request_cache_memory_size: Option<String> = None;
                let mut seen_request_cache_memory_size = false;
                let mut f_request_cache_miss_count: Option<String> = None;
                let mut seen_request_cache_miss_count = false;
                let mut f_search_concurrent_avg_slice_count: Option<String> = None;
                let mut seen_search_concurrent_avg_slice_count = false;
                let mut f_search_concurrent_query_current: Option<String> = None;
                let mut seen_search_concurrent_query_current = false;
                let mut f_search_concurrent_query_time: Option<String> = None;
                let mut seen_search_concurrent_query_time = false;
                let mut f_search_concurrent_query_total: Option<String> = None;
                let mut seen_search_concurrent_query_total = false;
                let mut f_search_fetch_current: Option<String> = None;
                let mut seen_search_fetch_current = false;
                let mut f_search_fetch_time: Option<String> = None;
                let mut seen_search_fetch_time = false;
                let mut f_search_fetch_total: Option<String> = None;
                let mut seen_search_fetch_total = false;
                let mut f_search_open_contexts: Option<String> = None;
                let mut seen_search_open_contexts = false;
                let mut f_search_point_in_time_current: Option<String> = None;
                let mut seen_search_point_in_time_current = false;
                let mut f_search_point_in_time_time: Option<String> = None;
                let mut seen_search_point_in_time_time = false;
                let mut f_search_point_in_time_total: Option<String> = None;
                let mut seen_search_point_in_time_total = false;
                let mut f_search_query_current: Option<String> = None;
                let mut seen_search_query_current = false;
                let mut f_search_query_time: Option<String> = None;
                let mut seen_search_query_time = false;
                let mut f_search_query_total: Option<String> = None;
                let mut seen_search_query_total = false;
                let mut f_search_scroll_current: Option<String> = None;
                let mut seen_search_scroll_current = false;
                let mut f_search_scroll_time: Option<String> = None;
                let mut seen_search_scroll_time = false;
                let mut f_search_scroll_total: Option<String> = None;
                let mut seen_search_scroll_total = false;
                let mut f_search_throttled: Option<String> = None;
                let mut seen_search_throttled = false;
                let mut f_segments_count: Option<String> = None;
                let mut seen_segments_count = false;
                let mut f_segments_fixed_bitset_memory: Option<String> = None;
                let mut seen_segments_fixed_bitset_memory = false;
                let mut f_segments_index_writer_memory: Option<String> = None;
                let mut seen_segments_index_writer_memory = false;
                let mut f_segments_memory: Option<String> = None;
                let mut seen_segments_memory = false;
                let mut f_segments_version_map_memory: Option<String> = None;
                let mut seen_segments_version_map_memory = false;
                let mut f_status: Option<String> = None;
                let mut seen_status = false;
                let mut f_store_size: Option<String> = None;
                let mut seen_store_size = false;
                let mut f_suggest_current: Option<String> = None;
                let mut seen_suggest_current = false;
                let mut f_suggest_time: Option<String> = None;
                let mut seen_suggest_time = false;
                let mut f_suggest_total: Option<String> = None;
                let mut seen_suggest_total = false;
                let mut f_uuid: Option<String> = None;
                let mut seen_uuid = false;
                let mut f_warmer_current: Option<String> = None;
                let mut seen_warmer_current = false;
                let mut f_warmer_total: Option<String> = None;
                let mut seen_warmer_total = false;
                let mut f_warmer_total_time: Option<String> = None;
                let mut seen_warmer_total_time = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "completion.size" => {
                            if seen_completion_size {
                                return Err(serde::de::Error::duplicate_field("completion.size"));
                            }
                            seen_completion_size = true;
                            f_completion_size = map.next_value::<Option<String>>()?;
                        }
                        "creation.date" => {
                            if seen_creation_date {
                                return Err(serde::de::Error::duplicate_field("creation.date"));
                            }
                            seen_creation_date = true;
                            f_creation_date = map.next_value::<Option<String>>()?;
                        }
                        "creation.date.string" => {
                            if seen_creation_date_string {
                                return Err(serde::de::Error::duplicate_field(
                                    "creation.date.string",
                                ));
                            }
                            seen_creation_date_string = true;
                            f_creation_date_string = map.next_value::<Option<String>>()?;
                        }
                        "docs.count" => {
                            if seen_docs_count {
                                return Err(serde::de::Error::duplicate_field("docs.count"));
                            }
                            seen_docs_count = true;
                            f_docs_count = map.next_value::<Option<String>>()?;
                        }
                        "docs.deleted" => {
                            if seen_docs_deleted {
                                return Err(serde::de::Error::duplicate_field("docs.deleted"));
                            }
                            seen_docs_deleted = true;
                            f_docs_deleted = map.next_value::<Option<String>>()?;
                        }
                        "fielddata.evictions" => {
                            if seen_fielddata_evictions {
                                return Err(serde::de::Error::duplicate_field(
                                    "fielddata.evictions",
                                ));
                            }
                            seen_fielddata_evictions = true;
                            f_fielddata_evictions = map.next_value::<Option<String>>()?;
                        }
                        "fielddata.memory_size" => {
                            if seen_fielddata_memory_size {
                                return Err(serde::de::Error::duplicate_field(
                                    "fielddata.memory_size",
                                ));
                            }
                            seen_fielddata_memory_size = true;
                            f_fielddata_memory_size = map.next_value::<Option<String>>()?;
                        }
                        "flush.total" => {
                            if seen_flush_total {
                                return Err(serde::de::Error::duplicate_field("flush.total"));
                            }
                            seen_flush_total = true;
                            f_flush_total = map.next_value::<Option<String>>()?;
                        }
                        "flush.total_time" => {
                            if seen_flush_total_time {
                                return Err(serde::de::Error::duplicate_field("flush.total_time"));
                            }
                            seen_flush_total_time = true;
                            f_flush_total_time = map.next_value::<Option<String>>()?;
                        }
                        "get.current" => {
                            if seen_get_current {
                                return Err(serde::de::Error::duplicate_field("get.current"));
                            }
                            seen_get_current = true;
                            f_get_current = map.next_value::<Option<String>>()?;
                        }
                        "get.exists_time" => {
                            if seen_get_exists_time {
                                return Err(serde::de::Error::duplicate_field("get.exists_time"));
                            }
                            seen_get_exists_time = true;
                            f_get_exists_time = map.next_value::<Option<String>>()?;
                        }
                        "get.exists_total" => {
                            if seen_get_exists_total {
                                return Err(serde::de::Error::duplicate_field("get.exists_total"));
                            }
                            seen_get_exists_total = true;
                            f_get_exists_total = map.next_value::<Option<String>>()?;
                        }
                        "get.missing_time" => {
                            if seen_get_missing_time {
                                return Err(serde::de::Error::duplicate_field("get.missing_time"));
                            }
                            seen_get_missing_time = true;
                            f_get_missing_time = map.next_value::<Option<String>>()?;
                        }
                        "get.missing_total" => {
                            if seen_get_missing_total {
                                return Err(serde::de::Error::duplicate_field("get.missing_total"));
                            }
                            seen_get_missing_total = true;
                            f_get_missing_total = map.next_value::<Option<String>>()?;
                        }
                        "get.time" => {
                            if seen_get_time {
                                return Err(serde::de::Error::duplicate_field("get.time"));
                            }
                            seen_get_time = true;
                            f_get_time = map.next_value::<Option<String>>()?;
                        }
                        "get.total" => {
                            if seen_get_total {
                                return Err(serde::de::Error::duplicate_field("get.total"));
                            }
                            seen_get_total = true;
                            f_get_total = map.next_value::<Option<String>>()?;
                        }
                        "health" => {
                            if seen_health {
                                return Err(serde::de::Error::duplicate_field("health"));
                            }
                            seen_health = true;
                            f_health = map.next_value::<Option<String>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<String>>()?;
                        }
                        "indexing.delete_current" => {
                            if seen_indexing_delete_current {
                                return Err(serde::de::Error::duplicate_field(
                                    "indexing.delete_current",
                                ));
                            }
                            seen_indexing_delete_current = true;
                            f_indexing_delete_current = map.next_value::<Option<String>>()?;
                        }
                        "indexing.delete_time" => {
                            if seen_indexing_delete_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "indexing.delete_time",
                                ));
                            }
                            seen_indexing_delete_time = true;
                            f_indexing_delete_time = map.next_value::<Option<String>>()?;
                        }
                        "indexing.delete_total" => {
                            if seen_indexing_delete_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "indexing.delete_total",
                                ));
                            }
                            seen_indexing_delete_total = true;
                            f_indexing_delete_total = map.next_value::<Option<String>>()?;
                        }
                        "indexing.index_current" => {
                            if seen_indexing_index_current {
                                return Err(serde::de::Error::duplicate_field(
                                    "indexing.index_current",
                                ));
                            }
                            seen_indexing_index_current = true;
                            f_indexing_index_current = map.next_value::<Option<String>>()?;
                        }
                        "indexing.index_failed" => {
                            if seen_indexing_index_failed {
                                return Err(serde::de::Error::duplicate_field(
                                    "indexing.index_failed",
                                ));
                            }
                            seen_indexing_index_failed = true;
                            f_indexing_index_failed = map.next_value::<Option<String>>()?;
                        }
                        "indexing.index_time" => {
                            if seen_indexing_index_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "indexing.index_time",
                                ));
                            }
                            seen_indexing_index_time = true;
                            f_indexing_index_time = map.next_value::<Option<String>>()?;
                        }
                        "indexing.index_total" => {
                            if seen_indexing_index_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "indexing.index_total",
                                ));
                            }
                            seen_indexing_index_total = true;
                            f_indexing_index_total = map.next_value::<Option<String>>()?;
                        }
                        "memory.total" => {
                            if seen_memory_total {
                                return Err(serde::de::Error::duplicate_field("memory.total"));
                            }
                            seen_memory_total = true;
                            f_memory_total = map.next_value::<Option<String>>()?;
                        }
                        "merges.current" => {
                            if seen_merges_current {
                                return Err(serde::de::Error::duplicate_field("merges.current"));
                            }
                            seen_merges_current = true;
                            f_merges_current = map.next_value::<Option<String>>()?;
                        }
                        "merges.current_docs" => {
                            if seen_merges_current_docs {
                                return Err(serde::de::Error::duplicate_field(
                                    "merges.current_docs",
                                ));
                            }
                            seen_merges_current_docs = true;
                            f_merges_current_docs = map.next_value::<Option<String>>()?;
                        }
                        "merges.current_size" => {
                            if seen_merges_current_size {
                                return Err(serde::de::Error::duplicate_field(
                                    "merges.current_size",
                                ));
                            }
                            seen_merges_current_size = true;
                            f_merges_current_size = map.next_value::<Option<String>>()?;
                        }
                        "merges.total" => {
                            if seen_merges_total {
                                return Err(serde::de::Error::duplicate_field("merges.total"));
                            }
                            seen_merges_total = true;
                            f_merges_total = map.next_value::<Option<String>>()?;
                        }
                        "merges.total_docs" => {
                            if seen_merges_total_docs {
                                return Err(serde::de::Error::duplicate_field("merges.total_docs"));
                            }
                            seen_merges_total_docs = true;
                            f_merges_total_docs = map.next_value::<Option<String>>()?;
                        }
                        "merges.total_size" => {
                            if seen_merges_total_size {
                                return Err(serde::de::Error::duplicate_field("merges.total_size"));
                            }
                            seen_merges_total_size = true;
                            f_merges_total_size = map.next_value::<Option<String>>()?;
                        }
                        "merges.total_time" => {
                            if seen_merges_total_time {
                                return Err(serde::de::Error::duplicate_field("merges.total_time"));
                            }
                            seen_merges_total_time = true;
                            f_merges_total_time = map.next_value::<Option<String>>()?;
                        }
                        "pri" => {
                            if seen_pri {
                                return Err(serde::de::Error::duplicate_field("pri"));
                            }
                            seen_pri = true;
                            f_pri = map.next_value::<Option<String>>()?;
                        }
                        "pri.completion.size" => {
                            if seen_pri_completion_size {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.completion.size",
                                ));
                            }
                            seen_pri_completion_size = true;
                            f_pri_completion_size = map.next_value::<Option<String>>()?;
                        }
                        "pri.fielddata.evictions" => {
                            if seen_pri_fielddata_evictions {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.fielddata.evictions",
                                ));
                            }
                            seen_pri_fielddata_evictions = true;
                            f_pri_fielddata_evictions = map.next_value::<Option<String>>()?;
                        }
                        "pri.fielddata.memory_size" => {
                            if seen_pri_fielddata_memory_size {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.fielddata.memory_size",
                                ));
                            }
                            seen_pri_fielddata_memory_size = true;
                            f_pri_fielddata_memory_size = map.next_value::<Option<String>>()?;
                        }
                        "pri.flush.total" => {
                            if seen_pri_flush_total {
                                return Err(serde::de::Error::duplicate_field("pri.flush.total"));
                            }
                            seen_pri_flush_total = true;
                            f_pri_flush_total = map.next_value::<Option<String>>()?;
                        }
                        "pri.flush.total_time" => {
                            if seen_pri_flush_total_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.flush.total_time",
                                ));
                            }
                            seen_pri_flush_total_time = true;
                            f_pri_flush_total_time = map.next_value::<Option<String>>()?;
                        }
                        "pri.get.current" => {
                            if seen_pri_get_current {
                                return Err(serde::de::Error::duplicate_field("pri.get.current"));
                            }
                            seen_pri_get_current = true;
                            f_pri_get_current = map.next_value::<Option<String>>()?;
                        }
                        "pri.get.exists_time" => {
                            if seen_pri_get_exists_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.get.exists_time",
                                ));
                            }
                            seen_pri_get_exists_time = true;
                            f_pri_get_exists_time = map.next_value::<Option<String>>()?;
                        }
                        "pri.get.exists_total" => {
                            if seen_pri_get_exists_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.get.exists_total",
                                ));
                            }
                            seen_pri_get_exists_total = true;
                            f_pri_get_exists_total = map.next_value::<Option<String>>()?;
                        }
                        "pri.get.missing_time" => {
                            if seen_pri_get_missing_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.get.missing_time",
                                ));
                            }
                            seen_pri_get_missing_time = true;
                            f_pri_get_missing_time = map.next_value::<Option<String>>()?;
                        }
                        "pri.get.missing_total" => {
                            if seen_pri_get_missing_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.get.missing_total",
                                ));
                            }
                            seen_pri_get_missing_total = true;
                            f_pri_get_missing_total = map.next_value::<Option<String>>()?;
                        }
                        "pri.get.time" => {
                            if seen_pri_get_time {
                                return Err(serde::de::Error::duplicate_field("pri.get.time"));
                            }
                            seen_pri_get_time = true;
                            f_pri_get_time = map.next_value::<Option<String>>()?;
                        }
                        "pri.get.total" => {
                            if seen_pri_get_total {
                                return Err(serde::de::Error::duplicate_field("pri.get.total"));
                            }
                            seen_pri_get_total = true;
                            f_pri_get_total = map.next_value::<Option<String>>()?;
                        }
                        "pri.indexing.delete_current" => {
                            if seen_pri_indexing_delete_current {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.indexing.delete_current",
                                ));
                            }
                            seen_pri_indexing_delete_current = true;
                            f_pri_indexing_delete_current = map.next_value::<Option<String>>()?;
                        }
                        "pri.indexing.delete_time" => {
                            if seen_pri_indexing_delete_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.indexing.delete_time",
                                ));
                            }
                            seen_pri_indexing_delete_time = true;
                            f_pri_indexing_delete_time = map.next_value::<Option<String>>()?;
                        }
                        "pri.indexing.delete_total" => {
                            if seen_pri_indexing_delete_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.indexing.delete_total",
                                ));
                            }
                            seen_pri_indexing_delete_total = true;
                            f_pri_indexing_delete_total = map.next_value::<Option<String>>()?;
                        }
                        "pri.indexing.index_current" => {
                            if seen_pri_indexing_index_current {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.indexing.index_current",
                                ));
                            }
                            seen_pri_indexing_index_current = true;
                            f_pri_indexing_index_current = map.next_value::<Option<String>>()?;
                        }
                        "pri.indexing.index_failed" => {
                            if seen_pri_indexing_index_failed {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.indexing.index_failed",
                                ));
                            }
                            seen_pri_indexing_index_failed = true;
                            f_pri_indexing_index_failed = map.next_value::<Option<String>>()?;
                        }
                        "pri.indexing.index_time" => {
                            if seen_pri_indexing_index_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.indexing.index_time",
                                ));
                            }
                            seen_pri_indexing_index_time = true;
                            f_pri_indexing_index_time = map.next_value::<Option<String>>()?;
                        }
                        "pri.indexing.index_total" => {
                            if seen_pri_indexing_index_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.indexing.index_total",
                                ));
                            }
                            seen_pri_indexing_index_total = true;
                            f_pri_indexing_index_total = map.next_value::<Option<String>>()?;
                        }
                        "pri.memory.total" => {
                            if seen_pri_memory_total {
                                return Err(serde::de::Error::duplicate_field("pri.memory.total"));
                            }
                            seen_pri_memory_total = true;
                            f_pri_memory_total = map.next_value::<Option<String>>()?;
                        }
                        "pri.merges.current" => {
                            if seen_pri_merges_current {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.merges.current",
                                ));
                            }
                            seen_pri_merges_current = true;
                            f_pri_merges_current = map.next_value::<Option<String>>()?;
                        }
                        "pri.merges.current_docs" => {
                            if seen_pri_merges_current_docs {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.merges.current_docs",
                                ));
                            }
                            seen_pri_merges_current_docs = true;
                            f_pri_merges_current_docs = map.next_value::<Option<String>>()?;
                        }
                        "pri.merges.current_size" => {
                            if seen_pri_merges_current_size {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.merges.current_size",
                                ));
                            }
                            seen_pri_merges_current_size = true;
                            f_pri_merges_current_size = map.next_value::<Option<String>>()?;
                        }
                        "pri.merges.total" => {
                            if seen_pri_merges_total {
                                return Err(serde::de::Error::duplicate_field("pri.merges.total"));
                            }
                            seen_pri_merges_total = true;
                            f_pri_merges_total = map.next_value::<Option<String>>()?;
                        }
                        "pri.merges.total_docs" => {
                            if seen_pri_merges_total_docs {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.merges.total_docs",
                                ));
                            }
                            seen_pri_merges_total_docs = true;
                            f_pri_merges_total_docs = map.next_value::<Option<String>>()?;
                        }
                        "pri.merges.total_size" => {
                            if seen_pri_merges_total_size {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.merges.total_size",
                                ));
                            }
                            seen_pri_merges_total_size = true;
                            f_pri_merges_total_size = map.next_value::<Option<String>>()?;
                        }
                        "pri.merges.total_time" => {
                            if seen_pri_merges_total_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.merges.total_time",
                                ));
                            }
                            seen_pri_merges_total_time = true;
                            f_pri_merges_total_time = map.next_value::<Option<String>>()?;
                        }
                        "pri.query_cache.evictions" => {
                            if seen_pri_query_cache_evictions {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.query_cache.evictions",
                                ));
                            }
                            seen_pri_query_cache_evictions = true;
                            f_pri_query_cache_evictions = map.next_value::<Option<String>>()?;
                        }
                        "pri.query_cache.memory_size" => {
                            if seen_pri_query_cache_memory_size {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.query_cache.memory_size",
                                ));
                            }
                            seen_pri_query_cache_memory_size = true;
                            f_pri_query_cache_memory_size = map.next_value::<Option<String>>()?;
                        }
                        "pri.refresh.external_time" => {
                            if seen_pri_refresh_external_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.refresh.external_time",
                                ));
                            }
                            seen_pri_refresh_external_time = true;
                            f_pri_refresh_external_time = map.next_value::<Option<String>>()?;
                        }
                        "pri.refresh.external_total" => {
                            if seen_pri_refresh_external_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.refresh.external_total",
                                ));
                            }
                            seen_pri_refresh_external_total = true;
                            f_pri_refresh_external_total = map.next_value::<Option<String>>()?;
                        }
                        "pri.refresh.listeners" => {
                            if seen_pri_refresh_listeners {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.refresh.listeners",
                                ));
                            }
                            seen_pri_refresh_listeners = true;
                            f_pri_refresh_listeners = map.next_value::<Option<String>>()?;
                        }
                        "pri.refresh.time" => {
                            if seen_pri_refresh_time {
                                return Err(serde::de::Error::duplicate_field("pri.refresh.time"));
                            }
                            seen_pri_refresh_time = true;
                            f_pri_refresh_time = map.next_value::<Option<String>>()?;
                        }
                        "pri.refresh.total" => {
                            if seen_pri_refresh_total {
                                return Err(serde::de::Error::duplicate_field("pri.refresh.total"));
                            }
                            seen_pri_refresh_total = true;
                            f_pri_refresh_total = map.next_value::<Option<String>>()?;
                        }
                        "pri.request_cache.evictions" => {
                            if seen_pri_request_cache_evictions {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.request_cache.evictions",
                                ));
                            }
                            seen_pri_request_cache_evictions = true;
                            f_pri_request_cache_evictions = map.next_value::<Option<String>>()?;
                        }
                        "pri.request_cache.hit_count" => {
                            if seen_pri_request_cache_hit_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.request_cache.hit_count",
                                ));
                            }
                            seen_pri_request_cache_hit_count = true;
                            f_pri_request_cache_hit_count = map.next_value::<Option<String>>()?;
                        }
                        "pri.request_cache.memory_size" => {
                            if seen_pri_request_cache_memory_size {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.request_cache.memory_size",
                                ));
                            }
                            seen_pri_request_cache_memory_size = true;
                            f_pri_request_cache_memory_size = map.next_value::<Option<String>>()?;
                        }
                        "pri.request_cache.miss_count" => {
                            if seen_pri_request_cache_miss_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.request_cache.miss_count",
                                ));
                            }
                            seen_pri_request_cache_miss_count = true;
                            f_pri_request_cache_miss_count = map.next_value::<Option<String>>()?;
                        }
                        "pri.search.concurrent_avg_slice_count" => {
                            if seen_pri_search_concurrent_avg_slice_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.search.concurrent_avg_slice_count",
                                ));
                            }
                            seen_pri_search_concurrent_avg_slice_count = true;
                            f_pri_search_concurrent_avg_slice_count =
                                map.next_value::<Option<String>>()?;
                        }
                        "pri.search.concurrent_query_current" => {
                            if seen_pri_search_concurrent_query_current {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.search.concurrent_query_current",
                                ));
                            }
                            seen_pri_search_concurrent_query_current = true;
                            f_pri_search_concurrent_query_current =
                                map.next_value::<Option<String>>()?;
                        }
                        "pri.search.concurrent_query_time" => {
                            if seen_pri_search_concurrent_query_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.search.concurrent_query_time",
                                ));
                            }
                            seen_pri_search_concurrent_query_time = true;
                            f_pri_search_concurrent_query_time =
                                map.next_value::<Option<String>>()?;
                        }
                        "pri.search.concurrent_query_total" => {
                            if seen_pri_search_concurrent_query_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.search.concurrent_query_total",
                                ));
                            }
                            seen_pri_search_concurrent_query_total = true;
                            f_pri_search_concurrent_query_total =
                                map.next_value::<Option<String>>()?;
                        }
                        "pri.search.fetch_current" => {
                            if seen_pri_search_fetch_current {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.search.fetch_current",
                                ));
                            }
                            seen_pri_search_fetch_current = true;
                            f_pri_search_fetch_current = map.next_value::<Option<String>>()?;
                        }
                        "pri.search.fetch_time" => {
                            if seen_pri_search_fetch_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.search.fetch_time",
                                ));
                            }
                            seen_pri_search_fetch_time = true;
                            f_pri_search_fetch_time = map.next_value::<Option<String>>()?;
                        }
                        "pri.search.fetch_total" => {
                            if seen_pri_search_fetch_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.search.fetch_total",
                                ));
                            }
                            seen_pri_search_fetch_total = true;
                            f_pri_search_fetch_total = map.next_value::<Option<String>>()?;
                        }
                        "pri.search.open_contexts" => {
                            if seen_pri_search_open_contexts {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.search.open_contexts",
                                ));
                            }
                            seen_pri_search_open_contexts = true;
                            f_pri_search_open_contexts = map.next_value::<Option<String>>()?;
                        }
                        "pri.search.point_in_time_current" => {
                            if seen_pri_search_point_in_time_current {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.search.point_in_time_current",
                                ));
                            }
                            seen_pri_search_point_in_time_current = true;
                            f_pri_search_point_in_time_current =
                                map.next_value::<Option<String>>()?;
                        }
                        "pri.search.point_in_time_time" => {
                            if seen_pri_search_point_in_time_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.search.point_in_time_time",
                                ));
                            }
                            seen_pri_search_point_in_time_time = true;
                            f_pri_search_point_in_time_time = map.next_value::<Option<String>>()?;
                        }
                        "pri.search.point_in_time_total" => {
                            if seen_pri_search_point_in_time_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.search.point_in_time_total",
                                ));
                            }
                            seen_pri_search_point_in_time_total = true;
                            f_pri_search_point_in_time_total =
                                map.next_value::<Option<String>>()?;
                        }
                        "pri.search.query_current" => {
                            if seen_pri_search_query_current {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.search.query_current",
                                ));
                            }
                            seen_pri_search_query_current = true;
                            f_pri_search_query_current = map.next_value::<Option<String>>()?;
                        }
                        "pri.search.query_time" => {
                            if seen_pri_search_query_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.search.query_time",
                                ));
                            }
                            seen_pri_search_query_time = true;
                            f_pri_search_query_time = map.next_value::<Option<String>>()?;
                        }
                        "pri.search.query_total" => {
                            if seen_pri_search_query_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.search.query_total",
                                ));
                            }
                            seen_pri_search_query_total = true;
                            f_pri_search_query_total = map.next_value::<Option<String>>()?;
                        }
                        "pri.search.scroll_current" => {
                            if seen_pri_search_scroll_current {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.search.scroll_current",
                                ));
                            }
                            seen_pri_search_scroll_current = true;
                            f_pri_search_scroll_current = map.next_value::<Option<String>>()?;
                        }
                        "pri.search.scroll_time" => {
                            if seen_pri_search_scroll_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.search.scroll_time",
                                ));
                            }
                            seen_pri_search_scroll_time = true;
                            f_pri_search_scroll_time = map.next_value::<Option<String>>()?;
                        }
                        "pri.search.scroll_total" => {
                            if seen_pri_search_scroll_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.search.scroll_total",
                                ));
                            }
                            seen_pri_search_scroll_total = true;
                            f_pri_search_scroll_total = map.next_value::<Option<String>>()?;
                        }
                        "pri.segments.count" => {
                            if seen_pri_segments_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.segments.count",
                                ));
                            }
                            seen_pri_segments_count = true;
                            f_pri_segments_count = map.next_value::<Option<String>>()?;
                        }
                        "pri.segments.fixed_bitset_memory" => {
                            if seen_pri_segments_fixed_bitset_memory {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.segments.fixed_bitset_memory",
                                ));
                            }
                            seen_pri_segments_fixed_bitset_memory = true;
                            f_pri_segments_fixed_bitset_memory =
                                map.next_value::<Option<String>>()?;
                        }
                        "pri.segments.index_writer_memory" => {
                            if seen_pri_segments_index_writer_memory {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.segments.index_writer_memory",
                                ));
                            }
                            seen_pri_segments_index_writer_memory = true;
                            f_pri_segments_index_writer_memory =
                                map.next_value::<Option<String>>()?;
                        }
                        "pri.segments.memory" => {
                            if seen_pri_segments_memory {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.segments.memory",
                                ));
                            }
                            seen_pri_segments_memory = true;
                            f_pri_segments_memory = map.next_value::<Option<String>>()?;
                        }
                        "pri.segments.version_map_memory" => {
                            if seen_pri_segments_version_map_memory {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.segments.version_map_memory",
                                ));
                            }
                            seen_pri_segments_version_map_memory = true;
                            f_pri_segments_version_map_memory =
                                map.next_value::<Option<String>>()?;
                        }
                        "pri.store.size" => {
                            if seen_pri_store_size {
                                return Err(serde::de::Error::duplicate_field("pri.store.size"));
                            }
                            seen_pri_store_size = true;
                            f_pri_store_size = map.next_value::<Option<String>>()?;
                        }
                        "pri.suggest.current" => {
                            if seen_pri_suggest_current {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.suggest.current",
                                ));
                            }
                            seen_pri_suggest_current = true;
                            f_pri_suggest_current = map.next_value::<Option<String>>()?;
                        }
                        "pri.suggest.time" => {
                            if seen_pri_suggest_time {
                                return Err(serde::de::Error::duplicate_field("pri.suggest.time"));
                            }
                            seen_pri_suggest_time = true;
                            f_pri_suggest_time = map.next_value::<Option<String>>()?;
                        }
                        "pri.suggest.total" => {
                            if seen_pri_suggest_total {
                                return Err(serde::de::Error::duplicate_field("pri.suggest.total"));
                            }
                            seen_pri_suggest_total = true;
                            f_pri_suggest_total = map.next_value::<Option<String>>()?;
                        }
                        "pri.warmer.current" => {
                            if seen_pri_warmer_current {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.warmer.current",
                                ));
                            }
                            seen_pri_warmer_current = true;
                            f_pri_warmer_current = map.next_value::<Option<String>>()?;
                        }
                        "pri.warmer.total" => {
                            if seen_pri_warmer_total {
                                return Err(serde::de::Error::duplicate_field("pri.warmer.total"));
                            }
                            seen_pri_warmer_total = true;
                            f_pri_warmer_total = map.next_value::<Option<String>>()?;
                        }
                        "pri.warmer.total_time" => {
                            if seen_pri_warmer_total_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "pri.warmer.total_time",
                                ));
                            }
                            seen_pri_warmer_total_time = true;
                            f_pri_warmer_total_time = map.next_value::<Option<String>>()?;
                        }
                        "query_cache.evictions" => {
                            if seen_query_cache_evictions {
                                return Err(serde::de::Error::duplicate_field(
                                    "query_cache.evictions",
                                ));
                            }
                            seen_query_cache_evictions = true;
                            f_query_cache_evictions = map.next_value::<Option<String>>()?;
                        }
                        "query_cache.memory_size" => {
                            if seen_query_cache_memory_size {
                                return Err(serde::de::Error::duplicate_field(
                                    "query_cache.memory_size",
                                ));
                            }
                            seen_query_cache_memory_size = true;
                            f_query_cache_memory_size = map.next_value::<Option<String>>()?;
                        }
                        "refresh.external_time" => {
                            if seen_refresh_external_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "refresh.external_time",
                                ));
                            }
                            seen_refresh_external_time = true;
                            f_refresh_external_time = map.next_value::<Option<String>>()?;
                        }
                        "refresh.external_total" => {
                            if seen_refresh_external_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "refresh.external_total",
                                ));
                            }
                            seen_refresh_external_total = true;
                            f_refresh_external_total = map.next_value::<Option<String>>()?;
                        }
                        "refresh.listeners" => {
                            if seen_refresh_listeners {
                                return Err(serde::de::Error::duplicate_field("refresh.listeners"));
                            }
                            seen_refresh_listeners = true;
                            f_refresh_listeners = map.next_value::<Option<String>>()?;
                        }
                        "refresh.time" => {
                            if seen_refresh_time {
                                return Err(serde::de::Error::duplicate_field("refresh.time"));
                            }
                            seen_refresh_time = true;
                            f_refresh_time = map.next_value::<Option<String>>()?;
                        }
                        "refresh.total" => {
                            if seen_refresh_total {
                                return Err(serde::de::Error::duplicate_field("refresh.total"));
                            }
                            seen_refresh_total = true;
                            f_refresh_total = map.next_value::<Option<String>>()?;
                        }
                        "rep" => {
                            if seen_rep {
                                return Err(serde::de::Error::duplicate_field("rep"));
                            }
                            seen_rep = true;
                            f_rep = map.next_value::<Option<String>>()?;
                        }
                        "request_cache.evictions" => {
                            if seen_request_cache_evictions {
                                return Err(serde::de::Error::duplicate_field(
                                    "request_cache.evictions",
                                ));
                            }
                            seen_request_cache_evictions = true;
                            f_request_cache_evictions = map.next_value::<Option<String>>()?;
                        }
                        "request_cache.hit_count" => {
                            if seen_request_cache_hit_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "request_cache.hit_count",
                                ));
                            }
                            seen_request_cache_hit_count = true;
                            f_request_cache_hit_count = map.next_value::<Option<String>>()?;
                        }
                        "request_cache.memory_size" => {
                            if seen_request_cache_memory_size {
                                return Err(serde::de::Error::duplicate_field(
                                    "request_cache.memory_size",
                                ));
                            }
                            seen_request_cache_memory_size = true;
                            f_request_cache_memory_size = map.next_value::<Option<String>>()?;
                        }
                        "request_cache.miss_count" => {
                            if seen_request_cache_miss_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "request_cache.miss_count",
                                ));
                            }
                            seen_request_cache_miss_count = true;
                            f_request_cache_miss_count = map.next_value::<Option<String>>()?;
                        }
                        "search.concurrent_avg_slice_count" => {
                            if seen_search_concurrent_avg_slice_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "search.concurrent_avg_slice_count",
                                ));
                            }
                            seen_search_concurrent_avg_slice_count = true;
                            f_search_concurrent_avg_slice_count =
                                map.next_value::<Option<String>>()?;
                        }
                        "search.concurrent_query_current" => {
                            if seen_search_concurrent_query_current {
                                return Err(serde::de::Error::duplicate_field(
                                    "search.concurrent_query_current",
                                ));
                            }
                            seen_search_concurrent_query_current = true;
                            f_search_concurrent_query_current =
                                map.next_value::<Option<String>>()?;
                        }
                        "search.concurrent_query_time" => {
                            if seen_search_concurrent_query_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "search.concurrent_query_time",
                                ));
                            }
                            seen_search_concurrent_query_time = true;
                            f_search_concurrent_query_time = map.next_value::<Option<String>>()?;
                        }
                        "search.concurrent_query_total" => {
                            if seen_search_concurrent_query_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "search.concurrent_query_total",
                                ));
                            }
                            seen_search_concurrent_query_total = true;
                            f_search_concurrent_query_total = map.next_value::<Option<String>>()?;
                        }
                        "search.fetch_current" => {
                            if seen_search_fetch_current {
                                return Err(serde::de::Error::duplicate_field(
                                    "search.fetch_current",
                                ));
                            }
                            seen_search_fetch_current = true;
                            f_search_fetch_current = map.next_value::<Option<String>>()?;
                        }
                        "search.fetch_time" => {
                            if seen_search_fetch_time {
                                return Err(serde::de::Error::duplicate_field("search.fetch_time"));
                            }
                            seen_search_fetch_time = true;
                            f_search_fetch_time = map.next_value::<Option<String>>()?;
                        }
                        "search.fetch_total" => {
                            if seen_search_fetch_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "search.fetch_total",
                                ));
                            }
                            seen_search_fetch_total = true;
                            f_search_fetch_total = map.next_value::<Option<String>>()?;
                        }
                        "search.open_contexts" => {
                            if seen_search_open_contexts {
                                return Err(serde::de::Error::duplicate_field(
                                    "search.open_contexts",
                                ));
                            }
                            seen_search_open_contexts = true;
                            f_search_open_contexts = map.next_value::<Option<String>>()?;
                        }
                        "search.point_in_time_current" => {
                            if seen_search_point_in_time_current {
                                return Err(serde::de::Error::duplicate_field(
                                    "search.point_in_time_current",
                                ));
                            }
                            seen_search_point_in_time_current = true;
                            f_search_point_in_time_current = map.next_value::<Option<String>>()?;
                        }
                        "search.point_in_time_time" => {
                            if seen_search_point_in_time_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "search.point_in_time_time",
                                ));
                            }
                            seen_search_point_in_time_time = true;
                            f_search_point_in_time_time = map.next_value::<Option<String>>()?;
                        }
                        "search.point_in_time_total" => {
                            if seen_search_point_in_time_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "search.point_in_time_total",
                                ));
                            }
                            seen_search_point_in_time_total = true;
                            f_search_point_in_time_total = map.next_value::<Option<String>>()?;
                        }
                        "search.query_current" => {
                            if seen_search_query_current {
                                return Err(serde::de::Error::duplicate_field(
                                    "search.query_current",
                                ));
                            }
                            seen_search_query_current = true;
                            f_search_query_current = map.next_value::<Option<String>>()?;
                        }
                        "search.query_time" => {
                            if seen_search_query_time {
                                return Err(serde::de::Error::duplicate_field("search.query_time"));
                            }
                            seen_search_query_time = true;
                            f_search_query_time = map.next_value::<Option<String>>()?;
                        }
                        "search.query_total" => {
                            if seen_search_query_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "search.query_total",
                                ));
                            }
                            seen_search_query_total = true;
                            f_search_query_total = map.next_value::<Option<String>>()?;
                        }
                        "search.scroll_current" => {
                            if seen_search_scroll_current {
                                return Err(serde::de::Error::duplicate_field(
                                    "search.scroll_current",
                                ));
                            }
                            seen_search_scroll_current = true;
                            f_search_scroll_current = map.next_value::<Option<String>>()?;
                        }
                        "search.scroll_time" => {
                            if seen_search_scroll_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "search.scroll_time",
                                ));
                            }
                            seen_search_scroll_time = true;
                            f_search_scroll_time = map.next_value::<Option<String>>()?;
                        }
                        "search.scroll_total" => {
                            if seen_search_scroll_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "search.scroll_total",
                                ));
                            }
                            seen_search_scroll_total = true;
                            f_search_scroll_total = map.next_value::<Option<String>>()?;
                        }
                        "search.throttled" => {
                            if seen_search_throttled {
                                return Err(serde::de::Error::duplicate_field("search.throttled"));
                            }
                            seen_search_throttled = true;
                            f_search_throttled = map.next_value::<Option<String>>()?;
                        }
                        "segments.count" => {
                            if seen_segments_count {
                                return Err(serde::de::Error::duplicate_field("segments.count"));
                            }
                            seen_segments_count = true;
                            f_segments_count = map.next_value::<Option<String>>()?;
                        }
                        "segments.fixed_bitset_memory" => {
                            if seen_segments_fixed_bitset_memory {
                                return Err(serde::de::Error::duplicate_field(
                                    "segments.fixed_bitset_memory",
                                ));
                            }
                            seen_segments_fixed_bitset_memory = true;
                            f_segments_fixed_bitset_memory = map.next_value::<Option<String>>()?;
                        }
                        "segments.index_writer_memory" => {
                            if seen_segments_index_writer_memory {
                                return Err(serde::de::Error::duplicate_field(
                                    "segments.index_writer_memory",
                                ));
                            }
                            seen_segments_index_writer_memory = true;
                            f_segments_index_writer_memory = map.next_value::<Option<String>>()?;
                        }
                        "segments.memory" => {
                            if seen_segments_memory {
                                return Err(serde::de::Error::duplicate_field("segments.memory"));
                            }
                            seen_segments_memory = true;
                            f_segments_memory = map.next_value::<Option<String>>()?;
                        }
                        "segments.version_map_memory" => {
                            if seen_segments_version_map_memory {
                                return Err(serde::de::Error::duplicate_field(
                                    "segments.version_map_memory",
                                ));
                            }
                            seen_segments_version_map_memory = true;
                            f_segments_version_map_memory = map.next_value::<Option<String>>()?;
                        }
                        "status" => {
                            if seen_status {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            seen_status = true;
                            f_status = map.next_value::<Option<String>>()?;
                        }
                        "store.size" => {
                            if seen_store_size {
                                return Err(serde::de::Error::duplicate_field("store.size"));
                            }
                            seen_store_size = true;
                            f_store_size = map.next_value::<Option<String>>()?;
                        }
                        "suggest.current" => {
                            if seen_suggest_current {
                                return Err(serde::de::Error::duplicate_field("suggest.current"));
                            }
                            seen_suggest_current = true;
                            f_suggest_current = map.next_value::<Option<String>>()?;
                        }
                        "suggest.time" => {
                            if seen_suggest_time {
                                return Err(serde::de::Error::duplicate_field("suggest.time"));
                            }
                            seen_suggest_time = true;
                            f_suggest_time = map.next_value::<Option<String>>()?;
                        }
                        "suggest.total" => {
                            if seen_suggest_total {
                                return Err(serde::de::Error::duplicate_field("suggest.total"));
                            }
                            seen_suggest_total = true;
                            f_suggest_total = map.next_value::<Option<String>>()?;
                        }
                        "uuid" => {
                            if seen_uuid {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            seen_uuid = true;
                            f_uuid = map.next_value::<Option<String>>()?;
                        }
                        "warmer.current" => {
                            if seen_warmer_current {
                                return Err(serde::de::Error::duplicate_field("warmer.current"));
                            }
                            seen_warmer_current = true;
                            f_warmer_current = map.next_value::<Option<String>>()?;
                        }
                        "warmer.total" => {
                            if seen_warmer_total {
                                return Err(serde::de::Error::duplicate_field("warmer.total"));
                            }
                            seen_warmer_total = true;
                            f_warmer_total = map.next_value::<Option<String>>()?;
                        }
                        "warmer.total_time" => {
                            if seen_warmer_total_time {
                                return Err(serde::de::Error::duplicate_field("warmer.total_time"));
                            }
                            seen_warmer_total_time = true;
                            f_warmer_total_time = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndicesRecord {
                    completion_size: f_completion_size,
                    creation_date: f_creation_date,
                    creation_date_string: f_creation_date_string,
                    docs_count: f_docs_count,
                    docs_deleted: f_docs_deleted,
                    fielddata_evictions: f_fielddata_evictions,
                    fielddata_memory_size: f_fielddata_memory_size,
                    flush_total: f_flush_total,
                    flush_total_time: f_flush_total_time,
                    get_current: f_get_current,
                    get_exists_time: f_get_exists_time,
                    get_exists_total: f_get_exists_total,
                    get_missing_time: f_get_missing_time,
                    get_missing_total: f_get_missing_total,
                    get_time: f_get_time,
                    get_total: f_get_total,
                    health: f_health,
                    index: f_index,
                    indexing_delete_current: f_indexing_delete_current,
                    indexing_delete_time: f_indexing_delete_time,
                    indexing_delete_total: f_indexing_delete_total,
                    indexing_index_current: f_indexing_index_current,
                    indexing_index_failed: f_indexing_index_failed,
                    indexing_index_time: f_indexing_index_time,
                    indexing_index_total: f_indexing_index_total,
                    memory_total: f_memory_total,
                    merges_current: f_merges_current,
                    merges_current_docs: f_merges_current_docs,
                    merges_current_size: f_merges_current_size,
                    merges_total: f_merges_total,
                    merges_total_docs: f_merges_total_docs,
                    merges_total_size: f_merges_total_size,
                    merges_total_time: f_merges_total_time,
                    pri: f_pri,
                    pri_completion_size: f_pri_completion_size,
                    pri_fielddata_evictions: f_pri_fielddata_evictions,
                    pri_fielddata_memory_size: f_pri_fielddata_memory_size,
                    pri_flush_total: f_pri_flush_total,
                    pri_flush_total_time: f_pri_flush_total_time,
                    pri_get_current: f_pri_get_current,
                    pri_get_exists_time: f_pri_get_exists_time,
                    pri_get_exists_total: f_pri_get_exists_total,
                    pri_get_missing_time: f_pri_get_missing_time,
                    pri_get_missing_total: f_pri_get_missing_total,
                    pri_get_time: f_pri_get_time,
                    pri_get_total: f_pri_get_total,
                    pri_indexing_delete_current: f_pri_indexing_delete_current,
                    pri_indexing_delete_time: f_pri_indexing_delete_time,
                    pri_indexing_delete_total: f_pri_indexing_delete_total,
                    pri_indexing_index_current: f_pri_indexing_index_current,
                    pri_indexing_index_failed: f_pri_indexing_index_failed,
                    pri_indexing_index_time: f_pri_indexing_index_time,
                    pri_indexing_index_total: f_pri_indexing_index_total,
                    pri_memory_total: f_pri_memory_total,
                    pri_merges_current: f_pri_merges_current,
                    pri_merges_current_docs: f_pri_merges_current_docs,
                    pri_merges_current_size: f_pri_merges_current_size,
                    pri_merges_total: f_pri_merges_total,
                    pri_merges_total_docs: f_pri_merges_total_docs,
                    pri_merges_total_size: f_pri_merges_total_size,
                    pri_merges_total_time: f_pri_merges_total_time,
                    pri_query_cache_evictions: f_pri_query_cache_evictions,
                    pri_query_cache_memory_size: f_pri_query_cache_memory_size,
                    pri_refresh_external_time: f_pri_refresh_external_time,
                    pri_refresh_external_total: f_pri_refresh_external_total,
                    pri_refresh_listeners: f_pri_refresh_listeners,
                    pri_refresh_time: f_pri_refresh_time,
                    pri_refresh_total: f_pri_refresh_total,
                    pri_request_cache_evictions: f_pri_request_cache_evictions,
                    pri_request_cache_hit_count: f_pri_request_cache_hit_count,
                    pri_request_cache_memory_size: f_pri_request_cache_memory_size,
                    pri_request_cache_miss_count: f_pri_request_cache_miss_count,
                    pri_search_concurrent_avg_slice_count: f_pri_search_concurrent_avg_slice_count,
                    pri_search_concurrent_query_current: f_pri_search_concurrent_query_current,
                    pri_search_concurrent_query_time: f_pri_search_concurrent_query_time,
                    pri_search_concurrent_query_total: f_pri_search_concurrent_query_total,
                    pri_search_fetch_current: f_pri_search_fetch_current,
                    pri_search_fetch_time: f_pri_search_fetch_time,
                    pri_search_fetch_total: f_pri_search_fetch_total,
                    pri_search_open_contexts: f_pri_search_open_contexts,
                    pri_search_point_in_time_current: f_pri_search_point_in_time_current,
                    pri_search_point_in_time_time: f_pri_search_point_in_time_time,
                    pri_search_point_in_time_total: f_pri_search_point_in_time_total,
                    pri_search_query_current: f_pri_search_query_current,
                    pri_search_query_time: f_pri_search_query_time,
                    pri_search_query_total: f_pri_search_query_total,
                    pri_search_scroll_current: f_pri_search_scroll_current,
                    pri_search_scroll_time: f_pri_search_scroll_time,
                    pri_search_scroll_total: f_pri_search_scroll_total,
                    pri_segments_count: f_pri_segments_count,
                    pri_segments_fixed_bitset_memory: f_pri_segments_fixed_bitset_memory,
                    pri_segments_index_writer_memory: f_pri_segments_index_writer_memory,
                    pri_segments_memory: f_pri_segments_memory,
                    pri_segments_version_map_memory: f_pri_segments_version_map_memory,
                    pri_store_size: f_pri_store_size,
                    pri_suggest_current: f_pri_suggest_current,
                    pri_suggest_time: f_pri_suggest_time,
                    pri_suggest_total: f_pri_suggest_total,
                    pri_warmer_current: f_pri_warmer_current,
                    pri_warmer_total: f_pri_warmer_total,
                    pri_warmer_total_time: f_pri_warmer_total_time,
                    query_cache_evictions: f_query_cache_evictions,
                    query_cache_memory_size: f_query_cache_memory_size,
                    refresh_external_time: f_refresh_external_time,
                    refresh_external_total: f_refresh_external_total,
                    refresh_listeners: f_refresh_listeners,
                    refresh_time: f_refresh_time,
                    refresh_total: f_refresh_total,
                    rep: f_rep,
                    request_cache_evictions: f_request_cache_evictions,
                    request_cache_hit_count: f_request_cache_hit_count,
                    request_cache_memory_size: f_request_cache_memory_size,
                    request_cache_miss_count: f_request_cache_miss_count,
                    search_concurrent_avg_slice_count: f_search_concurrent_avg_slice_count,
                    search_concurrent_query_current: f_search_concurrent_query_current,
                    search_concurrent_query_time: f_search_concurrent_query_time,
                    search_concurrent_query_total: f_search_concurrent_query_total,
                    search_fetch_current: f_search_fetch_current,
                    search_fetch_time: f_search_fetch_time,
                    search_fetch_total: f_search_fetch_total,
                    search_open_contexts: f_search_open_contexts,
                    search_point_in_time_current: f_search_point_in_time_current,
                    search_point_in_time_time: f_search_point_in_time_time,
                    search_point_in_time_total: f_search_point_in_time_total,
                    search_query_current: f_search_query_current,
                    search_query_time: f_search_query_time,
                    search_query_total: f_search_query_total,
                    search_scroll_current: f_search_scroll_current,
                    search_scroll_time: f_search_scroll_time,
                    search_scroll_total: f_search_scroll_total,
                    search_throttled: f_search_throttled,
                    segments_count: f_segments_count,
                    segments_fixed_bitset_memory: f_segments_fixed_bitset_memory,
                    segments_index_writer_memory: f_segments_index_writer_memory,
                    segments_memory: f_segments_memory,
                    segments_version_map_memory: f_segments_version_map_memory,
                    status: f_status,
                    store_size: f_store_size,
                    suggest_current: f_suggest_current,
                    suggest_time: f_suggest_time,
                    suggest_total: f_suggest_total,
                    uuid: f_uuid,
                    warmer_current: f_warmer_current,
                    warmer_total: f_warmer_total,
                    warmer_total_time: f_warmer_total_time,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndicesRecord {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.completion_size {
            map.serialize_entry("completion.size", v)?;
        }
        if let Some(v) = &self.creation_date {
            map.serialize_entry("creation.date", v)?;
        }
        if let Some(v) = &self.creation_date_string {
            map.serialize_entry("creation.date.string", v)?;
        }
        if let Some(v) = &self.docs_count {
            map.serialize_entry("docs.count", v)?;
        }
        if let Some(v) = &self.docs_deleted {
            map.serialize_entry("docs.deleted", v)?;
        }
        if let Some(v) = &self.fielddata_evictions {
            map.serialize_entry("fielddata.evictions", v)?;
        }
        if let Some(v) = &self.fielddata_memory_size {
            map.serialize_entry("fielddata.memory_size", v)?;
        }
        if let Some(v) = &self.flush_total {
            map.serialize_entry("flush.total", v)?;
        }
        if let Some(v) = &self.flush_total_time {
            map.serialize_entry("flush.total_time", v)?;
        }
        if let Some(v) = &self.get_current {
            map.serialize_entry("get.current", v)?;
        }
        if let Some(v) = &self.get_exists_time {
            map.serialize_entry("get.exists_time", v)?;
        }
        if let Some(v) = &self.get_exists_total {
            map.serialize_entry("get.exists_total", v)?;
        }
        if let Some(v) = &self.get_missing_time {
            map.serialize_entry("get.missing_time", v)?;
        }
        if let Some(v) = &self.get_missing_total {
            map.serialize_entry("get.missing_total", v)?;
        }
        if let Some(v) = &self.get_time {
            map.serialize_entry("get.time", v)?;
        }
        if let Some(v) = &self.get_total {
            map.serialize_entry("get.total", v)?;
        }
        if let Some(v) = &self.health {
            map.serialize_entry("health", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.indexing_delete_current {
            map.serialize_entry("indexing.delete_current", v)?;
        }
        if let Some(v) = &self.indexing_delete_time {
            map.serialize_entry("indexing.delete_time", v)?;
        }
        if let Some(v) = &self.indexing_delete_total {
            map.serialize_entry("indexing.delete_total", v)?;
        }
        if let Some(v) = &self.indexing_index_current {
            map.serialize_entry("indexing.index_current", v)?;
        }
        if let Some(v) = &self.indexing_index_failed {
            map.serialize_entry("indexing.index_failed", v)?;
        }
        if let Some(v) = &self.indexing_index_time {
            map.serialize_entry("indexing.index_time", v)?;
        }
        if let Some(v) = &self.indexing_index_total {
            map.serialize_entry("indexing.index_total", v)?;
        }
        if let Some(v) = &self.memory_total {
            map.serialize_entry("memory.total", v)?;
        }
        if let Some(v) = &self.merges_current {
            map.serialize_entry("merges.current", v)?;
        }
        if let Some(v) = &self.merges_current_docs {
            map.serialize_entry("merges.current_docs", v)?;
        }
        if let Some(v) = &self.merges_current_size {
            map.serialize_entry("merges.current_size", v)?;
        }
        if let Some(v) = &self.merges_total {
            map.serialize_entry("merges.total", v)?;
        }
        if let Some(v) = &self.merges_total_docs {
            map.serialize_entry("merges.total_docs", v)?;
        }
        if let Some(v) = &self.merges_total_size {
            map.serialize_entry("merges.total_size", v)?;
        }
        if let Some(v) = &self.merges_total_time {
            map.serialize_entry("merges.total_time", v)?;
        }
        if let Some(v) = &self.pri {
            map.serialize_entry("pri", v)?;
        }
        if let Some(v) = &self.pri_completion_size {
            map.serialize_entry("pri.completion.size", v)?;
        }
        if let Some(v) = &self.pri_fielddata_evictions {
            map.serialize_entry("pri.fielddata.evictions", v)?;
        }
        if let Some(v) = &self.pri_fielddata_memory_size {
            map.serialize_entry("pri.fielddata.memory_size", v)?;
        }
        if let Some(v) = &self.pri_flush_total {
            map.serialize_entry("pri.flush.total", v)?;
        }
        if let Some(v) = &self.pri_flush_total_time {
            map.serialize_entry("pri.flush.total_time", v)?;
        }
        if let Some(v) = &self.pri_get_current {
            map.serialize_entry("pri.get.current", v)?;
        }
        if let Some(v) = &self.pri_get_exists_time {
            map.serialize_entry("pri.get.exists_time", v)?;
        }
        if let Some(v) = &self.pri_get_exists_total {
            map.serialize_entry("pri.get.exists_total", v)?;
        }
        if let Some(v) = &self.pri_get_missing_time {
            map.serialize_entry("pri.get.missing_time", v)?;
        }
        if let Some(v) = &self.pri_get_missing_total {
            map.serialize_entry("pri.get.missing_total", v)?;
        }
        if let Some(v) = &self.pri_get_time {
            map.serialize_entry("pri.get.time", v)?;
        }
        if let Some(v) = &self.pri_get_total {
            map.serialize_entry("pri.get.total", v)?;
        }
        if let Some(v) = &self.pri_indexing_delete_current {
            map.serialize_entry("pri.indexing.delete_current", v)?;
        }
        if let Some(v) = &self.pri_indexing_delete_time {
            map.serialize_entry("pri.indexing.delete_time", v)?;
        }
        if let Some(v) = &self.pri_indexing_delete_total {
            map.serialize_entry("pri.indexing.delete_total", v)?;
        }
        if let Some(v) = &self.pri_indexing_index_current {
            map.serialize_entry("pri.indexing.index_current", v)?;
        }
        if let Some(v) = &self.pri_indexing_index_failed {
            map.serialize_entry("pri.indexing.index_failed", v)?;
        }
        if let Some(v) = &self.pri_indexing_index_time {
            map.serialize_entry("pri.indexing.index_time", v)?;
        }
        if let Some(v) = &self.pri_indexing_index_total {
            map.serialize_entry("pri.indexing.index_total", v)?;
        }
        if let Some(v) = &self.pri_memory_total {
            map.serialize_entry("pri.memory.total", v)?;
        }
        if let Some(v) = &self.pri_merges_current {
            map.serialize_entry("pri.merges.current", v)?;
        }
        if let Some(v) = &self.pri_merges_current_docs {
            map.serialize_entry("pri.merges.current_docs", v)?;
        }
        if let Some(v) = &self.pri_merges_current_size {
            map.serialize_entry("pri.merges.current_size", v)?;
        }
        if let Some(v) = &self.pri_merges_total {
            map.serialize_entry("pri.merges.total", v)?;
        }
        if let Some(v) = &self.pri_merges_total_docs {
            map.serialize_entry("pri.merges.total_docs", v)?;
        }
        if let Some(v) = &self.pri_merges_total_size {
            map.serialize_entry("pri.merges.total_size", v)?;
        }
        if let Some(v) = &self.pri_merges_total_time {
            map.serialize_entry("pri.merges.total_time", v)?;
        }
        if let Some(v) = &self.pri_query_cache_evictions {
            map.serialize_entry("pri.query_cache.evictions", v)?;
        }
        if let Some(v) = &self.pri_query_cache_memory_size {
            map.serialize_entry("pri.query_cache.memory_size", v)?;
        }
        if let Some(v) = &self.pri_refresh_external_time {
            map.serialize_entry("pri.refresh.external_time", v)?;
        }
        if let Some(v) = &self.pri_refresh_external_total {
            map.serialize_entry("pri.refresh.external_total", v)?;
        }
        if let Some(v) = &self.pri_refresh_listeners {
            map.serialize_entry("pri.refresh.listeners", v)?;
        }
        if let Some(v) = &self.pri_refresh_time {
            map.serialize_entry("pri.refresh.time", v)?;
        }
        if let Some(v) = &self.pri_refresh_total {
            map.serialize_entry("pri.refresh.total", v)?;
        }
        if let Some(v) = &self.pri_request_cache_evictions {
            map.serialize_entry("pri.request_cache.evictions", v)?;
        }
        if let Some(v) = &self.pri_request_cache_hit_count {
            map.serialize_entry("pri.request_cache.hit_count", v)?;
        }
        if let Some(v) = &self.pri_request_cache_memory_size {
            map.serialize_entry("pri.request_cache.memory_size", v)?;
        }
        if let Some(v) = &self.pri_request_cache_miss_count {
            map.serialize_entry("pri.request_cache.miss_count", v)?;
        }
        if let Some(v) = &self.pri_search_concurrent_avg_slice_count {
            map.serialize_entry("pri.search.concurrent_avg_slice_count", v)?;
        }
        if let Some(v) = &self.pri_search_concurrent_query_current {
            map.serialize_entry("pri.search.concurrent_query_current", v)?;
        }
        if let Some(v) = &self.pri_search_concurrent_query_time {
            map.serialize_entry("pri.search.concurrent_query_time", v)?;
        }
        if let Some(v) = &self.pri_search_concurrent_query_total {
            map.serialize_entry("pri.search.concurrent_query_total", v)?;
        }
        if let Some(v) = &self.pri_search_fetch_current {
            map.serialize_entry("pri.search.fetch_current", v)?;
        }
        if let Some(v) = &self.pri_search_fetch_time {
            map.serialize_entry("pri.search.fetch_time", v)?;
        }
        if let Some(v) = &self.pri_search_fetch_total {
            map.serialize_entry("pri.search.fetch_total", v)?;
        }
        if let Some(v) = &self.pri_search_open_contexts {
            map.serialize_entry("pri.search.open_contexts", v)?;
        }
        if let Some(v) = &self.pri_search_point_in_time_current {
            map.serialize_entry("pri.search.point_in_time_current", v)?;
        }
        if let Some(v) = &self.pri_search_point_in_time_time {
            map.serialize_entry("pri.search.point_in_time_time", v)?;
        }
        if let Some(v) = &self.pri_search_point_in_time_total {
            map.serialize_entry("pri.search.point_in_time_total", v)?;
        }
        if let Some(v) = &self.pri_search_query_current {
            map.serialize_entry("pri.search.query_current", v)?;
        }
        if let Some(v) = &self.pri_search_query_time {
            map.serialize_entry("pri.search.query_time", v)?;
        }
        if let Some(v) = &self.pri_search_query_total {
            map.serialize_entry("pri.search.query_total", v)?;
        }
        if let Some(v) = &self.pri_search_scroll_current {
            map.serialize_entry("pri.search.scroll_current", v)?;
        }
        if let Some(v) = &self.pri_search_scroll_time {
            map.serialize_entry("pri.search.scroll_time", v)?;
        }
        if let Some(v) = &self.pri_search_scroll_total {
            map.serialize_entry("pri.search.scroll_total", v)?;
        }
        if let Some(v) = &self.pri_segments_count {
            map.serialize_entry("pri.segments.count", v)?;
        }
        if let Some(v) = &self.pri_segments_fixed_bitset_memory {
            map.serialize_entry("pri.segments.fixed_bitset_memory", v)?;
        }
        if let Some(v) = &self.pri_segments_index_writer_memory {
            map.serialize_entry("pri.segments.index_writer_memory", v)?;
        }
        if let Some(v) = &self.pri_segments_memory {
            map.serialize_entry("pri.segments.memory", v)?;
        }
        if let Some(v) = &self.pri_segments_version_map_memory {
            map.serialize_entry("pri.segments.version_map_memory", v)?;
        }
        if let Some(v) = &self.pri_store_size {
            map.serialize_entry("pri.store.size", v)?;
        }
        if let Some(v) = &self.pri_suggest_current {
            map.serialize_entry("pri.suggest.current", v)?;
        }
        if let Some(v) = &self.pri_suggest_time {
            map.serialize_entry("pri.suggest.time", v)?;
        }
        if let Some(v) = &self.pri_suggest_total {
            map.serialize_entry("pri.suggest.total", v)?;
        }
        if let Some(v) = &self.pri_warmer_current {
            map.serialize_entry("pri.warmer.current", v)?;
        }
        if let Some(v) = &self.pri_warmer_total {
            map.serialize_entry("pri.warmer.total", v)?;
        }
        if let Some(v) = &self.pri_warmer_total_time {
            map.serialize_entry("pri.warmer.total_time", v)?;
        }
        if let Some(v) = &self.query_cache_evictions {
            map.serialize_entry("query_cache.evictions", v)?;
        }
        if let Some(v) = &self.query_cache_memory_size {
            map.serialize_entry("query_cache.memory_size", v)?;
        }
        if let Some(v) = &self.refresh_external_time {
            map.serialize_entry("refresh.external_time", v)?;
        }
        if let Some(v) = &self.refresh_external_total {
            map.serialize_entry("refresh.external_total", v)?;
        }
        if let Some(v) = &self.refresh_listeners {
            map.serialize_entry("refresh.listeners", v)?;
        }
        if let Some(v) = &self.refresh_time {
            map.serialize_entry("refresh.time", v)?;
        }
        if let Some(v) = &self.refresh_total {
            map.serialize_entry("refresh.total", v)?;
        }
        if let Some(v) = &self.rep {
            map.serialize_entry("rep", v)?;
        }
        if let Some(v) = &self.request_cache_evictions {
            map.serialize_entry("request_cache.evictions", v)?;
        }
        if let Some(v) = &self.request_cache_hit_count {
            map.serialize_entry("request_cache.hit_count", v)?;
        }
        if let Some(v) = &self.request_cache_memory_size {
            map.serialize_entry("request_cache.memory_size", v)?;
        }
        if let Some(v) = &self.request_cache_miss_count {
            map.serialize_entry("request_cache.miss_count", v)?;
        }
        if let Some(v) = &self.search_concurrent_avg_slice_count {
            map.serialize_entry("search.concurrent_avg_slice_count", v)?;
        }
        if let Some(v) = &self.search_concurrent_query_current {
            map.serialize_entry("search.concurrent_query_current", v)?;
        }
        if let Some(v) = &self.search_concurrent_query_time {
            map.serialize_entry("search.concurrent_query_time", v)?;
        }
        if let Some(v) = &self.search_concurrent_query_total {
            map.serialize_entry("search.concurrent_query_total", v)?;
        }
        if let Some(v) = &self.search_fetch_current {
            map.serialize_entry("search.fetch_current", v)?;
        }
        if let Some(v) = &self.search_fetch_time {
            map.serialize_entry("search.fetch_time", v)?;
        }
        if let Some(v) = &self.search_fetch_total {
            map.serialize_entry("search.fetch_total", v)?;
        }
        if let Some(v) = &self.search_open_contexts {
            map.serialize_entry("search.open_contexts", v)?;
        }
        if let Some(v) = &self.search_point_in_time_current {
            map.serialize_entry("search.point_in_time_current", v)?;
        }
        if let Some(v) = &self.search_point_in_time_time {
            map.serialize_entry("search.point_in_time_time", v)?;
        }
        if let Some(v) = &self.search_point_in_time_total {
            map.serialize_entry("search.point_in_time_total", v)?;
        }
        if let Some(v) = &self.search_query_current {
            map.serialize_entry("search.query_current", v)?;
        }
        if let Some(v) = &self.search_query_time {
            map.serialize_entry("search.query_time", v)?;
        }
        if let Some(v) = &self.search_query_total {
            map.serialize_entry("search.query_total", v)?;
        }
        if let Some(v) = &self.search_scroll_current {
            map.serialize_entry("search.scroll_current", v)?;
        }
        if let Some(v) = &self.search_scroll_time {
            map.serialize_entry("search.scroll_time", v)?;
        }
        if let Some(v) = &self.search_scroll_total {
            map.serialize_entry("search.scroll_total", v)?;
        }
        if let Some(v) = &self.search_throttled {
            map.serialize_entry("search.throttled", v)?;
        }
        if let Some(v) = &self.segments_count {
            map.serialize_entry("segments.count", v)?;
        }
        if let Some(v) = &self.segments_fixed_bitset_memory {
            map.serialize_entry("segments.fixed_bitset_memory", v)?;
        }
        if let Some(v) = &self.segments_index_writer_memory {
            map.serialize_entry("segments.index_writer_memory", v)?;
        }
        if let Some(v) = &self.segments_memory {
            map.serialize_entry("segments.memory", v)?;
        }
        if let Some(v) = &self.segments_version_map_memory {
            map.serialize_entry("segments.version_map_memory", v)?;
        }
        if let Some(v) = &self.status {
            map.serialize_entry("status", v)?;
        }
        if let Some(v) = &self.store_size {
            map.serialize_entry("store.size", v)?;
        }
        if let Some(v) = &self.suggest_current {
            map.serialize_entry("suggest.current", v)?;
        }
        if let Some(v) = &self.suggest_time {
            map.serialize_entry("suggest.time", v)?;
        }
        if let Some(v) = &self.suggest_total {
            map.serialize_entry("suggest.total", v)?;
        }
        if let Some(v) = &self.uuid {
            map.serialize_entry("uuid", v)?;
        }
        if let Some(v) = &self.warmer_current {
            map.serialize_entry("warmer.current", v)?;
        }
        if let Some(v) = &self.warmer_total {
            map.serialize_entry("warmer.total", v)?;
        }
        if let Some(v) = &self.warmer_total_time {
            map.serialize_entry("warmer.total_time", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `cat.indices` API"]
pub type IndicesResponse = crate::types::cat::IndicesResponse;
