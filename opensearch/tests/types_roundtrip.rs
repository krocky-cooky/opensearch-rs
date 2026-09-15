/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! Round-trip tests for the generated response types: realistic server
//! response fixtures must deserialize into the typed models and serialize
//! back to the same JSON. No server is required.

use opensearch::types::{
    bulk::BulkResponse, common::WriteResponseBase, get::GetResponse, search::SearchResponse,
};
use serde_json::{json, Value};

/// Deserializes the fixture as `T`, serializes it back, and asserts the
/// JSON survived unchanged
fn round_trip<T>(fixture: &Value) -> T
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    let typed: T = serde_json::from_value(fixture.clone())
        .unwrap_or_else(|e| panic!("deserialization failed: {}", e));
    let back = serde_json::to_value(&typed).unwrap();
    assert_eq!(fixture, &back, "round-trip changed the JSON");
    typed
}

#[test]
fn search_response_round_trips() {
    let fixture = json!({
        "took": 5,
        "timed_out": false,
        "_shards": { "total": 2, "successful": 2, "skipped": 0, "failed": 0 },
        "hits": {
            "total": { "value": 42, "relation": "eq" },
            "max_score": 1.3,
            "hits": [
                {
                    "_index": "movies",
                    "_id": "1",
                    "_score": 1.3,
                    "_source": { "title": "Moneyball", "year": 2011 }
                }
            ]
        }
    });
    let resp: SearchResponse = round_trip(&fixture);
    assert_eq!(resp.took, 5);
    assert!(!resp.timed_out);
    assert_eq!(resp.shards.total, 2);
    assert_eq!(resp.hits.hits.len(), 1);

    let hit = &resp.hits.hits[0];
    assert_eq!(hit.index.as_deref(), Some("movies"));
    // _source is intentionally untyped: the caller decides the type
    let source = hit.source.as_ref().unwrap();
    let doc: Value = serde_json::from_str(source.get()).unwrap();
    assert_eq!(doc["title"], "Moneyball");
}

#[test]
fn total_hits_dispatches_on_json_kind() {
    // rest_total_hits_as_int=true collapses total to a bare integer
    let as_object = json!({ "value": 42, "relation": "eq" });
    let as_int = json!(42);

    use opensearch::types::search::SearchResponseHitsTotal;
    let object: SearchResponseHitsTotal = round_trip(&as_object);
    match object {
        SearchResponseHitsTotal::TotalHits(t) => assert_eq!(t.value, 42),
        other => panic!("expected the object branch, got {:?}", other),
    }
    let int: SearchResponseHitsTotal = round_trip(&as_int);
    match int {
        SearchResponseHitsTotal::Integer(v) => assert_eq!(v, 42),
        other => panic!("expected the integer branch, got {:?}", other),
    }
}

#[test]
fn aggregations_decode_lazily() {
    let fixture = json!({
        "took": 3,
        "timed_out": false,
        "_shards": { "total": 1, "successful": 1, "skipped": 0, "failed": 0 },
        "hits": { "hits": [] },
        "aggregations": {
            "my_genres": {
                "doc_count_error_upper_bound": 0,
                "sum_other_doc_count": 0,
                "buckets": [
                    {
                        "key": "rock",
                        "doc_count": 12,
                        "avg_year": { "value": 1998.5 }
                    },
                    {
                        "key": "jazz",
                        "doc_count": 3
                    }
                ]
            },
            "avg_year": { "value": 2001.25 }
        }
    });
    let resp: SearchResponse = round_trip(&fixture);
    let aggregations = resp.aggregations.as_ref().unwrap();

    // top level: parse each aggregation as the branch the request asked for
    let genres = aggregations["my_genres"].as_sterms().unwrap();
    let avg = aggregations["avg_year"].as_avg().unwrap();
    assert_eq!(avg.value, Some(2001.25));

    // string terms buckets are declared as the array form by the spec
    let buckets = &genres.buckets;
    assert_eq!(buckets.len(), 2);
    assert_eq!(buckets[0].key, "rock");
    assert_eq!(buckets[0].doc_count, 12);

    // sub-aggregations: residual keys of the bucket, decoded lazily again
    let sub = buckets[0].aggregations["avg_year"].as_avg().unwrap();
    assert_eq!(sub.value, Some(1998.5));
    assert!(buckets[1].aggregations.is_empty());
}

#[test]
fn wrong_branch_reports_union_error() {
    let fixture = json!({
        "took": 1,
        "timed_out": false,
        "_shards": { "total": 1, "successful": 1, "skipped": 0, "failed": 0 },
        "hits": { "hits": [] },
        "aggregations": {
            "avg_year": { "value": 2001.25 }
        }
    });
    let resp: SearchResponse = round_trip(&fixture);
    let aggregations = resp.aggregations.as_ref().unwrap();

    // asking for the wrong branch fails without poisoning the value
    let err = aggregations["avg_year"].as_sterms().unwrap_err();
    assert_eq!(err.attempted(), "sterms");
    // the raw JSON stays accessible
    assert!(aggregations["avg_year"].raw().get().contains("2001.25"));
    // and the right branch still works
    assert!(aggregations["avg_year"].as_avg().is_ok());
}

#[test]
fn unknown_enum_value_does_not_fail_the_response() {
    use opensearch::types::search::TotalHitsRelation;
    let known: TotalHitsRelation = serde_json::from_value(json!("gte")).unwrap();
    assert_eq!(known, TotalHitsRelation::Gte);
    // a value the specification does not know must not fail deserialization
    let unknown: TotalHitsRelation = serde_json::from_value(json!("approximate")).unwrap();
    assert_eq!(unknown, TotalHitsRelation::Unknown);
}

#[test]
fn index_response_round_trips() {
    // the index API returns the shared write response
    let fixture = json!({
        "_index": "movies",
        "_id": "1",
        "_version": 1,
        "result": "created",
        "_shards": { "total": 2, "successful": 1, "failed": 0 },
        "_seq_no": 0,
        "_primary_term": 1
    });
    let resp: WriteResponseBase = round_trip(&fixture);
    assert_eq!(resp.index, "movies");
    assert_eq!(resp.version, 1);

    // IndexResponse is an alias of the shared type
    let _alias: opensearch::types::index::IndexResponse = round_trip(&fixture);
}

#[test]
fn bulk_response_round_trips() {
    let fixture = json!({
        "took": 11,
        "errors": true,
        "items": [
            {
                "index": {
                    "_index": "movies",
                    "_id": "1",
                    "_version": 1,
                    "result": "created",
                    "status": 201,
                    "_shards": { "total": 2, "successful": 1, "failed": 0 },
                    "_seq_no": 0,
                    "_primary_term": 1
                }
            },
            {
                "delete": {
                    "_index": "movies",
                    "_id": "404",
                    "status": 404,
                    "error": {
                        "type": "document_missing_exception",
                        "reason": "[404]: document missing"
                    }
                }
            }
        ]
    });
    let resp: BulkResponse = round_trip(&fixture);
    assert_eq!(resp.took, 11);
    assert!(resp.errors);
    assert_eq!(resp.items.len(), 2);
}

#[test]
fn get_response_round_trips() {
    let fixture = json!({
        "_index": "movies",
        "_id": "1",
        "_version": 1,
        "_seq_no": 0,
        "_primary_term": 1,
        "found": true,
        "_source": { "title": "Moneyball", "year": 2011 }
    });
    let resp: GetResponse = round_trip(&fixture);
    assert_eq!(resp.index, "movies");
    assert!(resp.found);
    let source = resp.source.as_ref().unwrap();
    let doc: Value = serde_json::from_str(source.get()).unwrap();
    assert_eq!(doc["year"], 2011);

    // a get miss: found=false and no source
    let miss = json!({
        "_index": "movies",
        "_id": "999",
        "found": false
    });
    let resp: GetResponse = round_trip(&miss);
    assert!(!resp.found);
    assert!(resp.source.is_none());
}
