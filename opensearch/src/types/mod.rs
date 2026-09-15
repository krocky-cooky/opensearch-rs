/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! Typed response models generated from the
//! [OpenSearch API specification](https://github.com/opensearch-project/opensearch-api-specification),
//! together with the hand-written runtime support they rely on.
//!
//! # Design notes
//!
//! 100% typing is a non-goal: fields whose shape is decided at runtime
//! (documents, aggregation values, ...) intentionally hold raw JSON and
//! expose it through explicit escape hatches. Additions of such untyped
//! fields are guarded by an allowlist at generation time.

pub mod union;

pub use union::UnionBranchError;

// GENERATED-BEGIN:response-modules
// Generated code - do not edit until the next GENERATED-END marker

pub mod bulk;
pub mod common;
pub mod common_aggregations;
pub mod explain;
pub mod get;
pub mod index;
pub mod search;

// GENERATED-END
