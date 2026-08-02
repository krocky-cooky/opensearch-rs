# Upgrading

- [Upgrading to >= 3.0.0](#upgrading-to--300)
  - [Removed APIs](#removed-apis)
  - [Removed query parameters](#removed-query-parameters)
  - [OpenSearch 1.x support](#opensearch-1x-support)
  - [New APIs](#new-apis)

## Upgrading to >= 3.0.0

Version 3.0.0 regenerates the client from the OpenSearch `main` branch REST API
specification (previously generated from the `1.x` branch). This aligns the client's API
surface with OpenSearch 3.x.

### Removed APIs

The following APIs were removed from OpenSearch and are no longer available in the
client. Code calling them will fail to compile:

| Removed | Replacement |
| --- | --- |
| `client.cat().master()` | `client.cat().cluster_manager()` |
| `client.indices().unfreeze()` | None. Index freezing was never supported by OpenSearch |
| `client.indices().reload_search_analyzers()` | None |
| `client.indices().migrate_to_data_stream()` | None |
| `client.indices().promote_data_stream()` | None |
| `client.snapshot().get_features()` | None |
| `client.text_structure().find_structure()` (the whole `text_structure` namespace) | None |

If you still need to call one of these endpoints against an older cluster, use the raw
HTTP API via `client.send()`.

### Removed query parameters

Legacy parameters that no longer exist in OpenSearch were removed from the generated
builders, for example:

- `min_compatible_shard_node` on the Search API

Additionally, `master_timeout` parameters remain available but are deprecated; use
`cluster_manager_timeout` instead.

### OpenSearch 1.x support

OpenSearch 1.x is no longer part of the integration test matrix for the 3.x client. The
client remains wire-compatible with 1.x clusters for APIs that exist in both versions,
but this is not verified by CI. If you run OpenSearch 1.x, consider staying on a 2.x
client release (see [COMPATIBILITY.md](COMPATIBILITY.md)).

### New APIs

The following APIs introduced in OpenSearch 2.x/3.x are now available:

- Data streams: `client.indices().create_data_stream()`, `get_data_stream()`,
  `data_streams_stats()`, `modify_data_stream()`
- Search pipelines: `client.search_pipeline().put()`, `get()`, `delete()`
- Cluster weighted routing: `client.cluster().put_weighted_routing()`,
  `get_weighted_routing()`, `delete_weighted_routing()`
- Cluster decommission awareness: `client.cluster().put_decommission_awareness()`,
  `get_decommission_awareness()`, `delete_decommission_awareness()`
- Tasks: `client.tasks().delete()`
- Workload management: `client.wlm_stats_list()`
- Remote store (requires the `experimental-apis` feature):
  `client.remote_store().restore()`, `stats()`
- Cat segment replication (requires the `experimental-apis` feature):
  `client.cat().segment_replication()`

New query parameters are also exposed on existing APIs, e.g. `search_pipeline` and
`include_named_queries_score` on the Search API.
