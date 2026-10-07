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

//! Response types of the `cat_shards` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct ShardsRecord {
    #[doc = "The average size in bytes of shard bulk operations."]
    pub bulk_avg_size_in_bytes: Option<String>,
    #[doc = "The average time spent in shard bulk operations."]
    pub bulk_avg_time: Option<String>,
    #[doc = "The number of bulk shard operations."]
    pub bulk_total_operations: Option<String>,
    #[doc = "The total size in bytes of shard bulk operations."]
    pub bulk_total_size_in_bytes: Option<String>,
    #[doc = "The time spent in shard bulk operations."]
    pub bulk_total_time: Option<String>,
    #[doc = "The size of completion."]
    pub completion_size: Option<String>,
    #[doc = "The number of documents in the shard."]
    pub docs: Option<String>,
    pub docs_deleted: Option<String>,
    #[doc = "The field data cache evictions."]
    pub fielddata_evictions: Option<String>,
    #[doc = "The used field data cache memory."]
    pub fielddata_memory_size: Option<String>,
    #[doc = "The number of flushes."]
    pub flush_total: Option<String>,
    #[doc = "The time spent in flush."]
    pub flush_total_time: Option<String>,
    #[doc = "The number of current get operations."]
    pub get_current: Option<String>,
    #[doc = "The time spent in successful get operations."]
    pub get_exists_time: Option<String>,
    #[doc = "The number of successful get operations."]
    pub get_exists_total: Option<String>,
    #[doc = "The time spent in failed get operations."]
    pub get_missing_time: Option<String>,
    #[doc = "The number of failed get operations."]
    pub get_missing_total: Option<String>,
    #[doc = "The time spent in get operations."]
    pub get_time: Option<String>,
    #[doc = "The number of get operations."]
    pub get_total: Option<String>,
    #[doc = "The unique identifier for the node."]
    pub id: Option<String>,
    #[doc = "The index name."]
    pub index: Option<String>,
    #[doc = "The number of current deletion operations."]
    pub indexing_delete_current: Option<String>,
    #[doc = "The time spent in deletion operations."]
    pub indexing_delete_time: Option<String>,
    #[doc = "The number of delete operations."]
    pub indexing_delete_total: Option<String>,
    #[doc = "The number of current indexing operations."]
    pub indexing_index_current: Option<String>,
    #[doc = "The number of failed indexing operations."]
    pub indexing_index_failed: Option<String>,
    #[doc = "The time spent in indexing operations."]
    pub indexing_index_time: Option<String>,
    #[doc = "The number of indexing operations."]
    pub indexing_index_total: Option<String>,
    #[doc = "The IP address of the node."]
    pub ip: Option<String>,
    #[doc = "The number of current merge operations."]
    pub merges_current: Option<String>,
    #[doc = "The number of current merging documents."]
    pub merges_current_docs: Option<String>,
    #[doc = "The size of current merge operations."]
    pub merges_current_size: Option<String>,
    #[doc = "The number of completed merge operations."]
    pub merges_total: Option<String>,
    #[doc = "The number of merged documents."]
    pub merges_total_docs: Option<String>,
    #[doc = "The size of current merges."]
    pub merges_total_size: Option<String>,
    #[doc = "The time spent merging documents."]
    pub merges_total_time: Option<String>,
    #[doc = "The name of node."]
    pub node: Option<String>,
    #[doc = "The shard data path."]
    pub path_data: Option<String>,
    #[doc = "The shard state path."]
    pub path_state: Option<String>,
    #[doc = "The shard type: `primary` or `replica`."]
    pub prirep: Option<String>,
    #[doc = "The query cache evictions."]
    pub query_cache_evictions: Option<String>,
    #[doc = "The used query cache memory."]
    pub query_cache_memory_size: Option<String>,
    #[doc = "The type of recovery source."]
    pub recoverysource_type: Option<String>,
    #[doc = "The time spent in external refreshes."]
    pub refresh_external_time: Option<String>,
    #[doc = "The total number of external refreshes."]
    pub refresh_external_total: Option<String>,
    #[doc = "The number of pending refresh listeners."]
    pub refresh_listeners: Option<String>,
    #[doc = "The time spent in refreshes."]
    pub refresh_time: Option<String>,
    #[doc = "The total number of refreshes."]
    pub refresh_total: Option<String>,
    pub search_concurrent_avg_slice_count: Option<String>,
    pub search_concurrent_query_current: Option<String>,
    pub search_concurrent_query_time: Option<String>,
    pub search_concurrent_query_total: Option<String>,
    #[doc = "The current fetch phase operations."]
    pub search_fetch_current: Option<String>,
    #[doc = "The time spent in fetch phase."]
    pub search_fetch_time: Option<String>,
    #[doc = "The total number of fetch operations."]
    pub search_fetch_total: Option<String>,
    #[doc = "The number of open search contexts."]
    pub search_open_contexts: Option<String>,
    pub search_point_in_time_current: Option<String>,
    pub search_point_in_time_time: Option<String>,
    pub search_point_in_time_total: Option<String>,
    #[doc = "The current query phase operations."]
    pub search_query_current: Option<String>,
    #[doc = "The time spent in query phase."]
    pub search_query_time: Option<String>,
    #[doc = "The total number of query phase operations."]
    pub search_query_total: Option<String>,
    #[doc = "The open scroll contexts."]
    pub search_scroll_current: Option<String>,
    #[doc = "The time scroll contexts were held open."]
    pub search_scroll_time: Option<String>,
    #[doc = "The number of completed scroll contexts."]
    pub search_scroll_total: Option<String>,
    pub search_search_idle_reactivate_count_total: Option<String>,
    #[doc = "The number of segments."]
    pub segments_count: Option<String>,
    #[doc = "The memory used by fixed bit sets for nested object field types and export type filters for types referred in `_parent` fields."]
    pub segments_fixed_bitset_memory: Option<String>,
    #[doc = "The memory used by the index writer."]
    pub segments_index_writer_memory: Option<String>,
    #[doc = "The memory used by segments."]
    pub segments_memory: Option<String>,
    #[doc = "The memory used by the version map."]
    pub segments_version_map_memory: Option<String>,
    #[doc = "The global checkpoint."]
    pub seq_no_global_checkpoint: Option<String>,
    #[doc = "The local checkpoint."]
    pub seq_no_local_checkpoint: Option<String>,
    #[doc = "The maximum sequence number."]
    pub seq_no_max: Option<String>,
    #[doc = "The shard name."]
    pub shard: Option<String>,
    #[doc = "The shard state. Returned values include: `INITIALIZING`: The shard is recovering from a peer shard or gateway. `RELOCATING`: The shard is relocating. `STARTED`: The shard has started. `UNASSIGNED`: The shard is not assigned to any node."]
    pub state: Option<String>,
    #[doc = "The disk space used by the shard."]
    pub store: Option<String>,
    #[doc = "The sync identifier."]
    pub sync_id: Option<String>,
    #[doc = "The time at which the shard became unassigned in Coordinated Universal Time (UTC)."]
    pub unassigned_at: Option<String>,
    #[doc = "Additional details as to why the shard became unassigned. It does not explain why the shard is not assigned; use the cluster allocation explain API for that information."]
    pub unassigned_details: Option<String>,
    #[doc = "The time at which the shard was requested to be unassigned in Coordinated Universal Time (UTC)."]
    pub unassigned_for: Option<String>,
    #[doc = "The reason for the last change to the state of an unassigned shard. It does not explain why the shard is currently unassigned; use the cluster allocation explain API for that information. Returned values include: `ALLOCATION_FAILED`: Unassigned as a result of a failed allocation of the shard. `CLUSTER_RECOVERED`: Unassigned as a result of a full cluster recovery. `DANGLING_INDEX_IMPORTED`: Unassigned as a result of importing a dangling index. `EXISTING_INDEX_RESTORED`: Unassigned as a result of restoring into a closed index. `FORCED_EMPTY_PRIMARY`: The shard's allocation was last modified by forcing an empty primary using the cluster reroute API. `INDEX_CLOSED`: Unassigned because the index was closed. `INDEX_CREATED`: Unassigned as a result of an API creation of an index. `INDEX_REOPENED`: Unassigned as a result of opening a closed index. `MANUAL_ALLOCATION`: The shard's allocation was last modified by the cluster reroute API. `NEW_INDEX_RESTORED`: Unassigned as a result of restoring into a new index. `NODE_LEFT`: Unassigned as a result of the node hosting it leaving the cluster. `NODE_RESTARTING`: Similar to `NODE_LEFT`, except that the node was registered as restarting using the node shutdown API. `PRIMARY_FAILED`: The shard was initializing as a replica, but the primary shard failed before the initialization completed. `REALLOCATED_REPLICA`: A better replica location is identified and causes the existing replica allocation to be cancelled. `REINITIALIZED`: When a shard moves from started back to initializing. `REPLICA_ADDED`: Unassigned as a result of explicit addition of a replica. `REROUTE_CANCELLED`: Unassigned as a result of explicit cancel reroute command."]
    pub unassigned_reason: Option<String>,
    #[doc = "The number of current warmer operations."]
    pub warmer_current: Option<String>,
    #[doc = "The total number of warmer operations."]
    pub warmer_total: Option<String>,
    #[doc = "The time spent in warmer operations."]
    pub warmer_total_time: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardsRecord {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardsRecord;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardsRecord object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_bulk_avg_size_in_bytes: Option<String> = None;
                let mut seen_bulk_avg_size_in_bytes = false;
                let mut f_bulk_avg_time: Option<String> = None;
                let mut seen_bulk_avg_time = false;
                let mut f_bulk_total_operations: Option<String> = None;
                let mut seen_bulk_total_operations = false;
                let mut f_bulk_total_size_in_bytes: Option<String> = None;
                let mut seen_bulk_total_size_in_bytes = false;
                let mut f_bulk_total_time: Option<String> = None;
                let mut seen_bulk_total_time = false;
                let mut f_completion_size: Option<String> = None;
                let mut seen_completion_size = false;
                let mut f_docs: Option<String> = None;
                let mut seen_docs = false;
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
                let mut f_id: Option<String> = None;
                let mut seen_id = false;
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
                let mut f_ip: Option<String> = None;
                let mut seen_ip = false;
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
                let mut f_node: Option<String> = None;
                let mut seen_node = false;
                let mut f_path_data: Option<String> = None;
                let mut seen_path_data = false;
                let mut f_path_state: Option<String> = None;
                let mut seen_path_state = false;
                let mut f_prirep: Option<String> = None;
                let mut seen_prirep = false;
                let mut f_query_cache_evictions: Option<String> = None;
                let mut seen_query_cache_evictions = false;
                let mut f_query_cache_memory_size: Option<String> = None;
                let mut seen_query_cache_memory_size = false;
                let mut f_recoverysource_type: Option<String> = None;
                let mut seen_recoverysource_type = false;
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
                let mut f_search_search_idle_reactivate_count_total: Option<String> = None;
                let mut seen_search_search_idle_reactivate_count_total = false;
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
                let mut f_seq_no_global_checkpoint: Option<String> = None;
                let mut seen_seq_no_global_checkpoint = false;
                let mut f_seq_no_local_checkpoint: Option<String> = None;
                let mut seen_seq_no_local_checkpoint = false;
                let mut f_seq_no_max: Option<String> = None;
                let mut seen_seq_no_max = false;
                let mut f_shard: Option<String> = None;
                let mut seen_shard = false;
                let mut f_state: Option<String> = None;
                let mut seen_state = false;
                let mut f_store: Option<String> = None;
                let mut seen_store = false;
                let mut f_sync_id: Option<String> = None;
                let mut seen_sync_id = false;
                let mut f_unassigned_at: Option<String> = None;
                let mut seen_unassigned_at = false;
                let mut f_unassigned_details: Option<String> = None;
                let mut seen_unassigned_details = false;
                let mut f_unassigned_for: Option<String> = None;
                let mut seen_unassigned_for = false;
                let mut f_unassigned_reason: Option<String> = None;
                let mut seen_unassigned_reason = false;
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
                        "bulk.avg_size_in_bytes" => {
                            if seen_bulk_avg_size_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "bulk.avg_size_in_bytes",
                                ));
                            }
                            seen_bulk_avg_size_in_bytes = true;
                            f_bulk_avg_size_in_bytes = map.next_value::<Option<String>>()?;
                        }
                        "bulk.avg_time" => {
                            if seen_bulk_avg_time {
                                return Err(serde::de::Error::duplicate_field("bulk.avg_time"));
                            }
                            seen_bulk_avg_time = true;
                            f_bulk_avg_time = map.next_value::<Option<String>>()?;
                        }
                        "bulk.total_operations" => {
                            if seen_bulk_total_operations {
                                return Err(serde::de::Error::duplicate_field(
                                    "bulk.total_operations",
                                ));
                            }
                            seen_bulk_total_operations = true;
                            f_bulk_total_operations = map.next_value::<Option<String>>()?;
                        }
                        "bulk.total_size_in_bytes" => {
                            if seen_bulk_total_size_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "bulk.total_size_in_bytes",
                                ));
                            }
                            seen_bulk_total_size_in_bytes = true;
                            f_bulk_total_size_in_bytes = map.next_value::<Option<String>>()?;
                        }
                        "bulk.total_time" => {
                            if seen_bulk_total_time {
                                return Err(serde::de::Error::duplicate_field("bulk.total_time"));
                            }
                            seen_bulk_total_time = true;
                            f_bulk_total_time = map.next_value::<Option<String>>()?;
                        }
                        "completion.size" => {
                            if seen_completion_size {
                                return Err(serde::de::Error::duplicate_field("completion.size"));
                            }
                            seen_completion_size = true;
                            f_completion_size = map.next_value::<Option<String>>()?;
                        }
                        "docs" => {
                            if seen_docs {
                                return Err(serde::de::Error::duplicate_field("docs"));
                            }
                            seen_docs = true;
                            f_docs = map.next_value::<Option<String>>()?;
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
                        "id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<String>>()?;
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
                        "ip" => {
                            if seen_ip {
                                return Err(serde::de::Error::duplicate_field("ip"));
                            }
                            seen_ip = true;
                            f_ip = map.next_value::<Option<String>>()?;
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
                        "node" => {
                            if seen_node {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            seen_node = true;
                            f_node = map.next_value::<Option<String>>()?;
                        }
                        "path.data" => {
                            if seen_path_data {
                                return Err(serde::de::Error::duplicate_field("path.data"));
                            }
                            seen_path_data = true;
                            f_path_data = map.next_value::<Option<String>>()?;
                        }
                        "path.state" => {
                            if seen_path_state {
                                return Err(serde::de::Error::duplicate_field("path.state"));
                            }
                            seen_path_state = true;
                            f_path_state = map.next_value::<Option<String>>()?;
                        }
                        "prirep" => {
                            if seen_prirep {
                                return Err(serde::de::Error::duplicate_field("prirep"));
                            }
                            seen_prirep = true;
                            f_prirep = map.next_value::<Option<String>>()?;
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
                        "recoverysource.type" => {
                            if seen_recoverysource_type {
                                return Err(serde::de::Error::duplicate_field(
                                    "recoverysource.type",
                                ));
                            }
                            seen_recoverysource_type = true;
                            f_recoverysource_type = map.next_value::<Option<String>>()?;
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
                        "search.search_idle_reactivate_count_total" => {
                            if seen_search_search_idle_reactivate_count_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "search.search_idle_reactivate_count_total",
                                ));
                            }
                            seen_search_search_idle_reactivate_count_total = true;
                            f_search_search_idle_reactivate_count_total =
                                map.next_value::<Option<String>>()?;
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
                        "seq_no.global_checkpoint" => {
                            if seen_seq_no_global_checkpoint {
                                return Err(serde::de::Error::duplicate_field(
                                    "seq_no.global_checkpoint",
                                ));
                            }
                            seen_seq_no_global_checkpoint = true;
                            f_seq_no_global_checkpoint = map.next_value::<Option<String>>()?;
                        }
                        "seq_no.local_checkpoint" => {
                            if seen_seq_no_local_checkpoint {
                                return Err(serde::de::Error::duplicate_field(
                                    "seq_no.local_checkpoint",
                                ));
                            }
                            seen_seq_no_local_checkpoint = true;
                            f_seq_no_local_checkpoint = map.next_value::<Option<String>>()?;
                        }
                        "seq_no.max" => {
                            if seen_seq_no_max {
                                return Err(serde::de::Error::duplicate_field("seq_no.max"));
                            }
                            seen_seq_no_max = true;
                            f_seq_no_max = map.next_value::<Option<String>>()?;
                        }
                        "shard" => {
                            if seen_shard {
                                return Err(serde::de::Error::duplicate_field("shard"));
                            }
                            seen_shard = true;
                            f_shard = map.next_value::<Option<String>>()?;
                        }
                        "state" => {
                            if seen_state {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            seen_state = true;
                            f_state = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<String>>()?;
                        }
                        "sync_id" => {
                            if seen_sync_id {
                                return Err(serde::de::Error::duplicate_field("sync_id"));
                            }
                            seen_sync_id = true;
                            f_sync_id = map.next_value::<Option<String>>()?;
                        }
                        "unassigned.at" => {
                            if seen_unassigned_at {
                                return Err(serde::de::Error::duplicate_field("unassigned.at"));
                            }
                            seen_unassigned_at = true;
                            f_unassigned_at = map.next_value::<Option<String>>()?;
                        }
                        "unassigned.details" => {
                            if seen_unassigned_details {
                                return Err(serde::de::Error::duplicate_field(
                                    "unassigned.details",
                                ));
                            }
                            seen_unassigned_details = true;
                            f_unassigned_details = map.next_value::<Option<String>>()?;
                        }
                        "unassigned.for" => {
                            if seen_unassigned_for {
                                return Err(serde::de::Error::duplicate_field("unassigned.for"));
                            }
                            seen_unassigned_for = true;
                            f_unassigned_for = map.next_value::<Option<String>>()?;
                        }
                        "unassigned.reason" => {
                            if seen_unassigned_reason {
                                return Err(serde::de::Error::duplicate_field("unassigned.reason"));
                            }
                            seen_unassigned_reason = true;
                            f_unassigned_reason = map.next_value::<Option<String>>()?;
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
                Ok(ShardsRecord {
                    bulk_avg_size_in_bytes: f_bulk_avg_size_in_bytes,
                    bulk_avg_time: f_bulk_avg_time,
                    bulk_total_operations: f_bulk_total_operations,
                    bulk_total_size_in_bytes: f_bulk_total_size_in_bytes,
                    bulk_total_time: f_bulk_total_time,
                    completion_size: f_completion_size,
                    docs: f_docs,
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
                    id: f_id,
                    index: f_index,
                    indexing_delete_current: f_indexing_delete_current,
                    indexing_delete_time: f_indexing_delete_time,
                    indexing_delete_total: f_indexing_delete_total,
                    indexing_index_current: f_indexing_index_current,
                    indexing_index_failed: f_indexing_index_failed,
                    indexing_index_time: f_indexing_index_time,
                    indexing_index_total: f_indexing_index_total,
                    ip: f_ip,
                    merges_current: f_merges_current,
                    merges_current_docs: f_merges_current_docs,
                    merges_current_size: f_merges_current_size,
                    merges_total: f_merges_total,
                    merges_total_docs: f_merges_total_docs,
                    merges_total_size: f_merges_total_size,
                    merges_total_time: f_merges_total_time,
                    node: f_node,
                    path_data: f_path_data,
                    path_state: f_path_state,
                    prirep: f_prirep,
                    query_cache_evictions: f_query_cache_evictions,
                    query_cache_memory_size: f_query_cache_memory_size,
                    recoverysource_type: f_recoverysource_type,
                    refresh_external_time: f_refresh_external_time,
                    refresh_external_total: f_refresh_external_total,
                    refresh_listeners: f_refresh_listeners,
                    refresh_time: f_refresh_time,
                    refresh_total: f_refresh_total,
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
                    search_search_idle_reactivate_count_total:
                        f_search_search_idle_reactivate_count_total,
                    segments_count: f_segments_count,
                    segments_fixed_bitset_memory: f_segments_fixed_bitset_memory,
                    segments_index_writer_memory: f_segments_index_writer_memory,
                    segments_memory: f_segments_memory,
                    segments_version_map_memory: f_segments_version_map_memory,
                    seq_no_global_checkpoint: f_seq_no_global_checkpoint,
                    seq_no_local_checkpoint: f_seq_no_local_checkpoint,
                    seq_no_max: f_seq_no_max,
                    shard: f_shard,
                    state: f_state,
                    store: f_store,
                    sync_id: f_sync_id,
                    unassigned_at: f_unassigned_at,
                    unassigned_details: f_unassigned_details,
                    unassigned_for: f_unassigned_for,
                    unassigned_reason: f_unassigned_reason,
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
impl serde::Serialize for ShardsRecord {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.bulk_avg_size_in_bytes {
            map.serialize_entry("bulk.avg_size_in_bytes", v)?;
        }
        if let Some(v) = &self.bulk_avg_time {
            map.serialize_entry("bulk.avg_time", v)?;
        }
        if let Some(v) = &self.bulk_total_operations {
            map.serialize_entry("bulk.total_operations", v)?;
        }
        if let Some(v) = &self.bulk_total_size_in_bytes {
            map.serialize_entry("bulk.total_size_in_bytes", v)?;
        }
        if let Some(v) = &self.bulk_total_time {
            map.serialize_entry("bulk.total_time", v)?;
        }
        if let Some(v) = &self.completion_size {
            map.serialize_entry("completion.size", v)?;
        }
        if let Some(v) = &self.docs {
            map.serialize_entry("docs", v)?;
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
        if let Some(v) = &self.id {
            map.serialize_entry("id", v)?;
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
        if let Some(v) = &self.ip {
            map.serialize_entry("ip", v)?;
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
        if let Some(v) = &self.node {
            map.serialize_entry("node", v)?;
        }
        if let Some(v) = &self.path_data {
            map.serialize_entry("path.data", v)?;
        }
        if let Some(v) = &self.path_state {
            map.serialize_entry("path.state", v)?;
        }
        if let Some(v) = &self.prirep {
            map.serialize_entry("prirep", v)?;
        }
        if let Some(v) = &self.query_cache_evictions {
            map.serialize_entry("query_cache.evictions", v)?;
        }
        if let Some(v) = &self.query_cache_memory_size {
            map.serialize_entry("query_cache.memory_size", v)?;
        }
        if let Some(v) = &self.recoverysource_type {
            map.serialize_entry("recoverysource.type", v)?;
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
        if let Some(v) = &self.search_search_idle_reactivate_count_total {
            map.serialize_entry("search.search_idle_reactivate_count_total", v)?;
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
        if let Some(v) = &self.seq_no_global_checkpoint {
            map.serialize_entry("seq_no.global_checkpoint", v)?;
        }
        if let Some(v) = &self.seq_no_local_checkpoint {
            map.serialize_entry("seq_no.local_checkpoint", v)?;
        }
        if let Some(v) = &self.seq_no_max {
            map.serialize_entry("seq_no.max", v)?;
        }
        if let Some(v) = &self.shard {
            map.serialize_entry("shard", v)?;
        }
        if let Some(v) = &self.state {
            map.serialize_entry("state", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.sync_id {
            map.serialize_entry("sync_id", v)?;
        }
        if let Some(v) = &self.unassigned_at {
            map.serialize_entry("unassigned.at", v)?;
        }
        if let Some(v) = &self.unassigned_details {
            map.serialize_entry("unassigned.details", v)?;
        }
        if let Some(v) = &self.unassigned_for {
            map.serialize_entry("unassigned.for", v)?;
        }
        if let Some(v) = &self.unassigned_reason {
            map.serialize_entry("unassigned.reason", v)?;
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

#[doc = "The response of the `cat.shards` API"]
pub type ShardsResponse = crate::types::cat::ShardsResponse;
