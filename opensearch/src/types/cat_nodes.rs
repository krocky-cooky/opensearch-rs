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

//! Response types of the `cat_nodes` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct NodesRecord {
    #[doc = "The OpenSearch build hash."]
    pub build: Option<String>,
    #[doc = "The average size in bytes of shard bulk."]
    pub bulk_avg_size_in_bytes: Option<String>,
    #[doc = "The average time spend in shard bulk."]
    pub bulk_avg_time: Option<String>,
    #[doc = "The number of bulk shard operations."]
    pub bulk_total_operations: Option<String>,
    #[doc = "The total size in bytes of shard bulk."]
    pub bulk_total_size_in_bytes: Option<String>,
    #[doc = "The time spend in shard bulk."]
    pub bulk_total_time: Option<String>,
    #[doc = "Indicates whether the node is the elected cluster-manager node. Returned values include `*`(elected cluster-manager) and `-`(not elected cluster-manager)."]
    pub cluster_manager: Option<String>,
    #[doc = "The size of completion."]
    pub completion_size: Option<String>,
    #[doc = "The recent system CPU usage as a percentage."]
    pub cpu: Option<String>,
    pub disk_avail: Option<String>,
    pub disk_total: Option<String>,
    pub disk_used: Option<String>,
    pub disk_used_percent: Option<crate::types::common::PercentageString>,
    #[doc = "The field data evictions."]
    pub fielddata_evictions: Option<String>,
    #[doc = "The used field data cache."]
    pub fielddata_memory_size: Option<String>,
    #[doc = "The used file descriptors."]
    pub file_desc_current: Option<String>,
    #[doc = "The maximum number of file descriptors."]
    pub file_desc_max: Option<String>,
    pub file_desc_percent: Option<crate::types::common::PercentageString>,
    #[doc = "The OpenSearch distribution flavor."]
    pub flavor: Option<String>,
    #[doc = "The number of flushes."]
    pub flush_total: Option<String>,
    #[doc = "The time spent in flush."]
    pub flush_total_time: Option<String>,
    #[doc = "The number of current get ops."]
    pub get_current: Option<String>,
    #[doc = "The time spent in successful gets."]
    pub get_exists_time: Option<String>,
    #[doc = "The number of successful get operations."]
    pub get_exists_total: Option<String>,
    #[doc = "The time spent in failed gets."]
    pub get_missing_time: Option<String>,
    #[doc = "The number of failed gets."]
    pub get_missing_total: Option<String>,
    #[doc = "The time spent in get."]
    pub get_time: Option<String>,
    #[doc = "The number of get ops."]
    pub get_total: Option<String>,
    #[doc = "The used heap."]
    pub heap_current: Option<String>,
    #[doc = "The maximum configured heap."]
    pub heap_max: Option<String>,
    pub heap_percent: Option<crate::types::common::PercentageString>,
    #[doc = "The bound HTTP address."]
    pub http_address: Option<String>,
    pub id: Option<crate::types::common::Id>,
    #[doc = "The number of current deletions."]
    pub indexing_delete_current: Option<String>,
    #[doc = "The time spent in deletions."]
    pub indexing_delete_time: Option<String>,
    #[doc = "The number of delete operations."]
    pub indexing_delete_total: Option<String>,
    #[doc = "The number of current indexing operations."]
    pub indexing_index_current: Option<String>,
    #[doc = "The number of failed indexing operations."]
    pub indexing_index_failed: Option<String>,
    #[doc = "The time spent in indexing."]
    pub indexing_index_time: Option<String>,
    #[doc = "The number of indexing operations."]
    pub indexing_index_total: Option<String>,
    #[doc = "The IP address."]
    pub ip: Option<String>,
    #[doc = "The Java version."]
    pub jdk: Option<String>,
    #[doc = "The load average for the last fifteen minutes."]
    pub load_1_5m: Option<String>,
    #[doc = "The load average for the most recent minute."]
    pub load_1m: Option<String>,
    #[doc = "The load average for the last five minutes."]
    pub load_5m: Option<String>,
    #[doc = "Indicates whether the node is the elected cluster-manager node. Returned values include `*`(elected cluster manager) and `-`(not elected cluster manager)."]
    pub master: Option<String>,
    #[doc = "The number of current merges."]
    pub merges_current: Option<String>,
    #[doc = "The number of current merging docs."]
    pub merges_current_docs: Option<String>,
    #[doc = "The size of current merges."]
    pub merges_current_size: Option<String>,
    #[doc = "The number of completed merge operations."]
    pub merges_total: Option<String>,
    #[doc = "The docs merged."]
    pub merges_total_docs: Option<String>,
    #[doc = "The size merged."]
    pub merges_total_size: Option<String>,
    #[doc = "The time spent in merges."]
    pub merges_total_time: Option<String>,
    pub name: Option<crate::types::common::Name>,
    #[doc = "The roles of the node. Returned values include `c`(cold node), `d`(data node), `f`(frozen node), `h`(hot node), `i`(ingest node), `l`(machine learning node), `m` (cluster-manager eligible node), `r`(remote cluster client node), `s`(content node), `t`(transform node), `v`(voting-only node), `w`(warm node),and `-`(coordinating node only)."]
    pub node_role: Option<String>,
    #[doc = "A comma-separated list of the roles of the node. Returned values may include but are not limited to: `data`, `ingest`, `master`, `cluster_manager`, `remote_cluster_client`, `search`, and `-`(coordinating node only)."]
    pub node_roles: Option<String>,
    #[doc = "The process identifier."]
    pub pid: Option<String>,
    #[doc = "The bound transport port."]
    pub port: Option<String>,
    #[doc = "The query cache evictions."]
    pub query_cache_evictions: Option<String>,
    #[doc = "The query cache hit counts."]
    pub query_cache_hit_count: Option<String>,
    #[doc = "The used query cache."]
    pub query_cache_memory_size: Option<String>,
    #[doc = "The query cache miss counts."]
    pub query_cache_miss_count: Option<String>,
    #[doc = "The used machine memory."]
    pub ram_current: Option<String>,
    #[doc = "The total machine memory."]
    pub ram_max: Option<String>,
    pub ram_percent: Option<crate::types::common::PercentageString>,
    #[doc = "The time spent in external refreshes."]
    pub refresh_external_time: Option<String>,
    #[doc = "The total external refreshes."]
    pub refresh_external_total: Option<String>,
    #[doc = "The number of pending refresh listeners."]
    pub refresh_listeners: Option<String>,
    #[doc = "The time spent in refreshes."]
    pub refresh_time: Option<String>,
    #[doc = "The total refreshes."]
    pub refresh_total: Option<String>,
    #[doc = "The request cache evictions."]
    pub request_cache_evictions: Option<String>,
    #[doc = "The request cache hit counts."]
    pub request_cache_hit_count: Option<String>,
    #[doc = "The used request cache."]
    pub request_cache_memory_size: Option<String>,
    #[doc = "The request cache miss counts."]
    pub request_cache_miss_count: Option<String>,
    #[doc = "The total compiled scripts evicted from the cache."]
    pub script_cache_evictions: Option<String>,
    #[doc = "The script cache compilation limit triggered."]
    pub script_compilation_limit_triggered: Option<String>,
    #[doc = "The total script compilations."]
    pub script_compilations: Option<String>,
    pub search_concurrent_avg_slice_count: Option<String>,
    pub search_concurrent_query_current: Option<String>,
    pub search_concurrent_query_time: Option<String>,
    pub search_concurrent_query_total: Option<String>,
    #[doc = "The current fetch phase operations."]
    pub search_fetch_current: Option<String>,
    #[doc = "The time spent in fetch phase."]
    pub search_fetch_time: Option<String>,
    #[doc = "The total fetch operations."]
    pub search_fetch_total: Option<String>,
    #[doc = "The open search contexts."]
    pub search_open_contexts: Option<String>,
    pub search_point_in_time_current: Option<String>,
    pub search_point_in_time_time: Option<String>,
    pub search_point_in_time_total: Option<String>,
    #[doc = "The current query phase operations."]
    pub search_query_current: Option<String>,
    #[doc = "The time spent in query phase."]
    pub search_query_time: Option<String>,
    #[doc = "The total query phase operations."]
    pub search_query_total: Option<String>,
    #[doc = "The open scroll contexts."]
    pub search_scroll_current: Option<String>,
    #[doc = "The time scroll contexts held open."]
    pub search_scroll_time: Option<String>,
    #[doc = "The completed scroll contexts."]
    pub search_scroll_total: Option<String>,
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
    #[doc = "The number of current suggest operations."]
    pub suggest_current: Option<String>,
    #[doc = "The time spend in suggest."]
    pub suggest_time: Option<String>,
    #[doc = "The number of suggest operations."]
    pub suggest_total: Option<String>,
    #[doc = "The OpenSearch distribution type."]
    pub r#type: Option<String>,
    #[doc = "The node uptime."]
    pub uptime: Option<String>,
    pub version: Option<crate::types::common::VersionString>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodesRecord {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodesRecord;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodesRecord object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_build: Option<String> = None;
                let mut seen_build = false;
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
                let mut f_cluster_manager: Option<String> = None;
                let mut seen_cluster_manager = false;
                let mut f_completion_size: Option<String> = None;
                let mut seen_completion_size = false;
                let mut f_cpu: Option<String> = None;
                let mut seen_cpu = false;
                let mut f_disk_avail: Option<String> = None;
                let mut seen_disk_avail = false;
                let mut f_disk_total: Option<String> = None;
                let mut seen_disk_total = false;
                let mut f_disk_used: Option<String> = None;
                let mut seen_disk_used = false;
                let mut f_disk_used_percent: Option<crate::types::common::PercentageString> = None;
                let mut seen_disk_used_percent = false;
                let mut f_fielddata_evictions: Option<String> = None;
                let mut seen_fielddata_evictions = false;
                let mut f_fielddata_memory_size: Option<String> = None;
                let mut seen_fielddata_memory_size = false;
                let mut f_file_desc_current: Option<String> = None;
                let mut seen_file_desc_current = false;
                let mut f_file_desc_max: Option<String> = None;
                let mut seen_file_desc_max = false;
                let mut f_file_desc_percent: Option<crate::types::common::PercentageString> = None;
                let mut seen_file_desc_percent = false;
                let mut f_flavor: Option<String> = None;
                let mut seen_flavor = false;
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
                let mut f_heap_current: Option<String> = None;
                let mut seen_heap_current = false;
                let mut f_heap_max: Option<String> = None;
                let mut seen_heap_max = false;
                let mut f_heap_percent: Option<crate::types::common::PercentageString> = None;
                let mut seen_heap_percent = false;
                let mut f_http_address: Option<String> = None;
                let mut seen_http_address = false;
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut seen_id = false;
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
                let mut f_jdk: Option<String> = None;
                let mut seen_jdk = false;
                let mut f_load_1_5m: Option<String> = None;
                let mut seen_load_1_5m = false;
                let mut f_load_1m: Option<String> = None;
                let mut seen_load_1m = false;
                let mut f_load_5m: Option<String> = None;
                let mut seen_load_5m = false;
                let mut f_master: Option<String> = None;
                let mut seen_master = false;
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
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut seen_name = false;
                let mut f_node_role: Option<String> = None;
                let mut seen_node_role = false;
                let mut f_node_roles: Option<String> = None;
                let mut seen_node_roles = false;
                let mut f_pid: Option<String> = None;
                let mut seen_pid = false;
                let mut f_port: Option<String> = None;
                let mut seen_port = false;
                let mut f_query_cache_evictions: Option<String> = None;
                let mut seen_query_cache_evictions = false;
                let mut f_query_cache_hit_count: Option<String> = None;
                let mut seen_query_cache_hit_count = false;
                let mut f_query_cache_memory_size: Option<String> = None;
                let mut seen_query_cache_memory_size = false;
                let mut f_query_cache_miss_count: Option<String> = None;
                let mut seen_query_cache_miss_count = false;
                let mut f_ram_current: Option<String> = None;
                let mut seen_ram_current = false;
                let mut f_ram_max: Option<String> = None;
                let mut seen_ram_max = false;
                let mut f_ram_percent: Option<crate::types::common::PercentageString> = None;
                let mut seen_ram_percent = false;
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
                let mut f_request_cache_evictions: Option<String> = None;
                let mut seen_request_cache_evictions = false;
                let mut f_request_cache_hit_count: Option<String> = None;
                let mut seen_request_cache_hit_count = false;
                let mut f_request_cache_memory_size: Option<String> = None;
                let mut seen_request_cache_memory_size = false;
                let mut f_request_cache_miss_count: Option<String> = None;
                let mut seen_request_cache_miss_count = false;
                let mut f_script_cache_evictions: Option<String> = None;
                let mut seen_script_cache_evictions = false;
                let mut f_script_compilation_limit_triggered: Option<String> = None;
                let mut seen_script_compilation_limit_triggered = false;
                let mut f_script_compilations: Option<String> = None;
                let mut seen_script_compilations = false;
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
                let mut f_suggest_current: Option<String> = None;
                let mut seen_suggest_current = false;
                let mut f_suggest_time: Option<String> = None;
                let mut seen_suggest_time = false;
                let mut f_suggest_total: Option<String> = None;
                let mut seen_suggest_total = false;
                let mut f_type: Option<String> = None;
                let mut seen_type = false;
                let mut f_uptime: Option<String> = None;
                let mut seen_uptime = false;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "build" => {
                            if seen_build {
                                return Err(serde::de::Error::duplicate_field("build"));
                            }
                            seen_build = true;
                            f_build = map.next_value::<Option<String>>()?;
                        }
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
                        "cluster_manager" => {
                            if seen_cluster_manager {
                                return Err(serde::de::Error::duplicate_field("cluster_manager"));
                            }
                            seen_cluster_manager = true;
                            f_cluster_manager = map.next_value::<Option<String>>()?;
                        }
                        "completion.size" => {
                            if seen_completion_size {
                                return Err(serde::de::Error::duplicate_field("completion.size"));
                            }
                            seen_completion_size = true;
                            f_completion_size = map.next_value::<Option<String>>()?;
                        }
                        "cpu" => {
                            if seen_cpu {
                                return Err(serde::de::Error::duplicate_field("cpu"));
                            }
                            seen_cpu = true;
                            f_cpu = map.next_value::<Option<String>>()?;
                        }
                        "disk.avail" => {
                            if seen_disk_avail {
                                return Err(serde::de::Error::duplicate_field("disk.avail"));
                            }
                            seen_disk_avail = true;
                            f_disk_avail = map.next_value::<Option<String>>()?;
                        }
                        "disk.total" => {
                            if seen_disk_total {
                                return Err(serde::de::Error::duplicate_field("disk.total"));
                            }
                            seen_disk_total = true;
                            f_disk_total = map.next_value::<Option<String>>()?;
                        }
                        "disk.used" => {
                            if seen_disk_used {
                                return Err(serde::de::Error::duplicate_field("disk.used"));
                            }
                            seen_disk_used = true;
                            f_disk_used = map.next_value::<Option<String>>()?;
                        }
                        "disk.used_percent" => {
                            if seen_disk_used_percent {
                                return Err(serde::de::Error::duplicate_field("disk.used_percent"));
                            }
                            seen_disk_used_percent = true;
                            f_disk_used_percent =
                                map.next_value::<Option<crate::types::common::PercentageString>>()?;
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
                        "file_desc.current" => {
                            if seen_file_desc_current {
                                return Err(serde::de::Error::duplicate_field("file_desc.current"));
                            }
                            seen_file_desc_current = true;
                            f_file_desc_current = map.next_value::<Option<String>>()?;
                        }
                        "file_desc.max" => {
                            if seen_file_desc_max {
                                return Err(serde::de::Error::duplicate_field("file_desc.max"));
                            }
                            seen_file_desc_max = true;
                            f_file_desc_max = map.next_value::<Option<String>>()?;
                        }
                        "file_desc.percent" => {
                            if seen_file_desc_percent {
                                return Err(serde::de::Error::duplicate_field("file_desc.percent"));
                            }
                            seen_file_desc_percent = true;
                            f_file_desc_percent =
                                map.next_value::<Option<crate::types::common::PercentageString>>()?;
                        }
                        "flavor" => {
                            if seen_flavor {
                                return Err(serde::de::Error::duplicate_field("flavor"));
                            }
                            seen_flavor = true;
                            f_flavor = map.next_value::<Option<String>>()?;
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
                        "heap.current" => {
                            if seen_heap_current {
                                return Err(serde::de::Error::duplicate_field("heap.current"));
                            }
                            seen_heap_current = true;
                            f_heap_current = map.next_value::<Option<String>>()?;
                        }
                        "heap.max" => {
                            if seen_heap_max {
                                return Err(serde::de::Error::duplicate_field("heap.max"));
                            }
                            seen_heap_max = true;
                            f_heap_max = map.next_value::<Option<String>>()?;
                        }
                        "heap.percent" => {
                            if seen_heap_percent {
                                return Err(serde::de::Error::duplicate_field("heap.percent"));
                            }
                            seen_heap_percent = true;
                            f_heap_percent =
                                map.next_value::<Option<crate::types::common::PercentageString>>()?;
                        }
                        "http_address" => {
                            if seen_http_address {
                                return Err(serde::de::Error::duplicate_field("http_address"));
                            }
                            seen_http_address = true;
                            f_http_address = map.next_value::<Option<String>>()?;
                        }
                        "id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<crate::types::common::Id>>()?;
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
                        "jdk" => {
                            if seen_jdk {
                                return Err(serde::de::Error::duplicate_field("jdk"));
                            }
                            seen_jdk = true;
                            f_jdk = map.next_value::<Option<String>>()?;
                        }
                        "load_15m" => {
                            if seen_load_1_5m {
                                return Err(serde::de::Error::duplicate_field("load_15m"));
                            }
                            seen_load_1_5m = true;
                            f_load_1_5m = map.next_value::<Option<String>>()?;
                        }
                        "load_1m" => {
                            if seen_load_1m {
                                return Err(serde::de::Error::duplicate_field("load_1m"));
                            }
                            seen_load_1m = true;
                            f_load_1m = map.next_value::<Option<String>>()?;
                        }
                        "load_5m" => {
                            if seen_load_5m {
                                return Err(serde::de::Error::duplicate_field("load_5m"));
                            }
                            seen_load_5m = true;
                            f_load_5m = map.next_value::<Option<String>>()?;
                        }
                        "master" => {
                            if seen_master {
                                return Err(serde::de::Error::duplicate_field("master"));
                            }
                            seen_master = true;
                            f_master = map.next_value::<Option<String>>()?;
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
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "node.role" => {
                            if seen_node_role {
                                return Err(serde::de::Error::duplicate_field("node.role"));
                            }
                            seen_node_role = true;
                            f_node_role = map.next_value::<Option<String>>()?;
                        }
                        "node.roles" => {
                            if seen_node_roles {
                                return Err(serde::de::Error::duplicate_field("node.roles"));
                            }
                            seen_node_roles = true;
                            f_node_roles = map.next_value::<Option<String>>()?;
                        }
                        "pid" => {
                            if seen_pid {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            seen_pid = true;
                            f_pid = map.next_value::<Option<String>>()?;
                        }
                        "port" => {
                            if seen_port {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            seen_port = true;
                            f_port = map.next_value::<Option<String>>()?;
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
                        "query_cache.hit_count" => {
                            if seen_query_cache_hit_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "query_cache.hit_count",
                                ));
                            }
                            seen_query_cache_hit_count = true;
                            f_query_cache_hit_count = map.next_value::<Option<String>>()?;
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
                        "query_cache.miss_count" => {
                            if seen_query_cache_miss_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "query_cache.miss_count",
                                ));
                            }
                            seen_query_cache_miss_count = true;
                            f_query_cache_miss_count = map.next_value::<Option<String>>()?;
                        }
                        "ram.current" => {
                            if seen_ram_current {
                                return Err(serde::de::Error::duplicate_field("ram.current"));
                            }
                            seen_ram_current = true;
                            f_ram_current = map.next_value::<Option<String>>()?;
                        }
                        "ram.max" => {
                            if seen_ram_max {
                                return Err(serde::de::Error::duplicate_field("ram.max"));
                            }
                            seen_ram_max = true;
                            f_ram_max = map.next_value::<Option<String>>()?;
                        }
                        "ram.percent" => {
                            if seen_ram_percent {
                                return Err(serde::de::Error::duplicate_field("ram.percent"));
                            }
                            seen_ram_percent = true;
                            f_ram_percent =
                                map.next_value::<Option<crate::types::common::PercentageString>>()?;
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
                        "script.cache_evictions" => {
                            if seen_script_cache_evictions {
                                return Err(serde::de::Error::duplicate_field(
                                    "script.cache_evictions",
                                ));
                            }
                            seen_script_cache_evictions = true;
                            f_script_cache_evictions = map.next_value::<Option<String>>()?;
                        }
                        "script.compilation_limit_triggered" => {
                            if seen_script_compilation_limit_triggered {
                                return Err(serde::de::Error::duplicate_field(
                                    "script.compilation_limit_triggered",
                                ));
                            }
                            seen_script_compilation_limit_triggered = true;
                            f_script_compilation_limit_triggered =
                                map.next_value::<Option<String>>()?;
                        }
                        "script.compilations" => {
                            if seen_script_compilations {
                                return Err(serde::de::Error::duplicate_field(
                                    "script.compilations",
                                ));
                            }
                            seen_script_compilations = true;
                            f_script_compilations = map.next_value::<Option<String>>()?;
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
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<String>>()?;
                        }
                        "uptime" => {
                            if seen_uptime {
                                return Err(serde::de::Error::duplicate_field("uptime"));
                            }
                            seen_uptime = true;
                            f_uptime = map.next_value::<Option<String>>()?;
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
                Ok(NodesRecord {
                    build: f_build,
                    bulk_avg_size_in_bytes: f_bulk_avg_size_in_bytes,
                    bulk_avg_time: f_bulk_avg_time,
                    bulk_total_operations: f_bulk_total_operations,
                    bulk_total_size_in_bytes: f_bulk_total_size_in_bytes,
                    bulk_total_time: f_bulk_total_time,
                    cluster_manager: f_cluster_manager,
                    completion_size: f_completion_size,
                    cpu: f_cpu,
                    disk_avail: f_disk_avail,
                    disk_total: f_disk_total,
                    disk_used: f_disk_used,
                    disk_used_percent: f_disk_used_percent,
                    fielddata_evictions: f_fielddata_evictions,
                    fielddata_memory_size: f_fielddata_memory_size,
                    file_desc_current: f_file_desc_current,
                    file_desc_max: f_file_desc_max,
                    file_desc_percent: f_file_desc_percent,
                    flavor: f_flavor,
                    flush_total: f_flush_total,
                    flush_total_time: f_flush_total_time,
                    get_current: f_get_current,
                    get_exists_time: f_get_exists_time,
                    get_exists_total: f_get_exists_total,
                    get_missing_time: f_get_missing_time,
                    get_missing_total: f_get_missing_total,
                    get_time: f_get_time,
                    get_total: f_get_total,
                    heap_current: f_heap_current,
                    heap_max: f_heap_max,
                    heap_percent: f_heap_percent,
                    http_address: f_http_address,
                    id: f_id,
                    indexing_delete_current: f_indexing_delete_current,
                    indexing_delete_time: f_indexing_delete_time,
                    indexing_delete_total: f_indexing_delete_total,
                    indexing_index_current: f_indexing_index_current,
                    indexing_index_failed: f_indexing_index_failed,
                    indexing_index_time: f_indexing_index_time,
                    indexing_index_total: f_indexing_index_total,
                    ip: f_ip,
                    jdk: f_jdk,
                    load_1_5m: f_load_1_5m,
                    load_1m: f_load_1m,
                    load_5m: f_load_5m,
                    master: f_master,
                    merges_current: f_merges_current,
                    merges_current_docs: f_merges_current_docs,
                    merges_current_size: f_merges_current_size,
                    merges_total: f_merges_total,
                    merges_total_docs: f_merges_total_docs,
                    merges_total_size: f_merges_total_size,
                    merges_total_time: f_merges_total_time,
                    name: f_name,
                    node_role: f_node_role,
                    node_roles: f_node_roles,
                    pid: f_pid,
                    port: f_port,
                    query_cache_evictions: f_query_cache_evictions,
                    query_cache_hit_count: f_query_cache_hit_count,
                    query_cache_memory_size: f_query_cache_memory_size,
                    query_cache_miss_count: f_query_cache_miss_count,
                    ram_current: f_ram_current,
                    ram_max: f_ram_max,
                    ram_percent: f_ram_percent,
                    refresh_external_time: f_refresh_external_time,
                    refresh_external_total: f_refresh_external_total,
                    refresh_listeners: f_refresh_listeners,
                    refresh_time: f_refresh_time,
                    refresh_total: f_refresh_total,
                    request_cache_evictions: f_request_cache_evictions,
                    request_cache_hit_count: f_request_cache_hit_count,
                    request_cache_memory_size: f_request_cache_memory_size,
                    request_cache_miss_count: f_request_cache_miss_count,
                    script_cache_evictions: f_script_cache_evictions,
                    script_compilation_limit_triggered: f_script_compilation_limit_triggered,
                    script_compilations: f_script_compilations,
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
                    segments_count: f_segments_count,
                    segments_fixed_bitset_memory: f_segments_fixed_bitset_memory,
                    segments_index_writer_memory: f_segments_index_writer_memory,
                    segments_memory: f_segments_memory,
                    segments_version_map_memory: f_segments_version_map_memory,
                    suggest_current: f_suggest_current,
                    suggest_time: f_suggest_time,
                    suggest_total: f_suggest_total,
                    r#type: f_type,
                    uptime: f_uptime,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodesRecord {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.build {
            map.serialize_entry("build", v)?;
        }
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
        if let Some(v) = &self.cluster_manager {
            map.serialize_entry("cluster_manager", v)?;
        }
        if let Some(v) = &self.completion_size {
            map.serialize_entry("completion.size", v)?;
        }
        if let Some(v) = &self.cpu {
            map.serialize_entry("cpu", v)?;
        }
        if let Some(v) = &self.disk_avail {
            map.serialize_entry("disk.avail", v)?;
        }
        if let Some(v) = &self.disk_total {
            map.serialize_entry("disk.total", v)?;
        }
        if let Some(v) = &self.disk_used {
            map.serialize_entry("disk.used", v)?;
        }
        if let Some(v) = &self.disk_used_percent {
            map.serialize_entry("disk.used_percent", v)?;
        }
        if let Some(v) = &self.fielddata_evictions {
            map.serialize_entry("fielddata.evictions", v)?;
        }
        if let Some(v) = &self.fielddata_memory_size {
            map.serialize_entry("fielddata.memory_size", v)?;
        }
        if let Some(v) = &self.file_desc_current {
            map.serialize_entry("file_desc.current", v)?;
        }
        if let Some(v) = &self.file_desc_max {
            map.serialize_entry("file_desc.max", v)?;
        }
        if let Some(v) = &self.file_desc_percent {
            map.serialize_entry("file_desc.percent", v)?;
        }
        if let Some(v) = &self.flavor {
            map.serialize_entry("flavor", v)?;
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
        if let Some(v) = &self.heap_current {
            map.serialize_entry("heap.current", v)?;
        }
        if let Some(v) = &self.heap_max {
            map.serialize_entry("heap.max", v)?;
        }
        if let Some(v) = &self.heap_percent {
            map.serialize_entry("heap.percent", v)?;
        }
        if let Some(v) = &self.http_address {
            map.serialize_entry("http_address", v)?;
        }
        if let Some(v) = &self.id {
            map.serialize_entry("id", v)?;
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
        if let Some(v) = &self.jdk {
            map.serialize_entry("jdk", v)?;
        }
        if let Some(v) = &self.load_1_5m {
            map.serialize_entry("load_15m", v)?;
        }
        if let Some(v) = &self.load_1m {
            map.serialize_entry("load_1m", v)?;
        }
        if let Some(v) = &self.load_5m {
            map.serialize_entry("load_5m", v)?;
        }
        if let Some(v) = &self.master {
            map.serialize_entry("master", v)?;
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
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.node_role {
            map.serialize_entry("node.role", v)?;
        }
        if let Some(v) = &self.node_roles {
            map.serialize_entry("node.roles", v)?;
        }
        if let Some(v) = &self.pid {
            map.serialize_entry("pid", v)?;
        }
        if let Some(v) = &self.port {
            map.serialize_entry("port", v)?;
        }
        if let Some(v) = &self.query_cache_evictions {
            map.serialize_entry("query_cache.evictions", v)?;
        }
        if let Some(v) = &self.query_cache_hit_count {
            map.serialize_entry("query_cache.hit_count", v)?;
        }
        if let Some(v) = &self.query_cache_memory_size {
            map.serialize_entry("query_cache.memory_size", v)?;
        }
        if let Some(v) = &self.query_cache_miss_count {
            map.serialize_entry("query_cache.miss_count", v)?;
        }
        if let Some(v) = &self.ram_current {
            map.serialize_entry("ram.current", v)?;
        }
        if let Some(v) = &self.ram_max {
            map.serialize_entry("ram.max", v)?;
        }
        if let Some(v) = &self.ram_percent {
            map.serialize_entry("ram.percent", v)?;
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
        if let Some(v) = &self.script_cache_evictions {
            map.serialize_entry("script.cache_evictions", v)?;
        }
        if let Some(v) = &self.script_compilation_limit_triggered {
            map.serialize_entry("script.compilation_limit_triggered", v)?;
        }
        if let Some(v) = &self.script_compilations {
            map.serialize_entry("script.compilations", v)?;
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
        if let Some(v) = &self.suggest_current {
            map.serialize_entry("suggest.current", v)?;
        }
        if let Some(v) = &self.suggest_time {
            map.serialize_entry("suggest.time", v)?;
        }
        if let Some(v) = &self.suggest_total {
            map.serialize_entry("suggest.total", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("type", v)?;
        }
        if let Some(v) = &self.uptime {
            map.serialize_entry("uptime", v)?;
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

#[doc = "The response of the `cat.nodes` API"]
pub type NodesResponse = crate::types::cat::NodesResponse;
