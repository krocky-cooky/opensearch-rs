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

//! Response types of the `common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AcknowledgedResponseBase {
    #[doc = "For a successful response, this value is always true. On failure, an exception is returned instead."]
    pub acknowledged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ActionStatusOptions {
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "simulated")]
    Simulated,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "throttled")]
    Throttled,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BuiltinScriptLanguage {
    #[serde(rename = "expression")]
    Expression,
    #[serde(rename = "java")]
    Java,
    #[serde(rename = "mustache")]
    Mustache,
    #[serde(rename = "painless")]
    Painless,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct BulkByScrollFailure {
    raw: Box<serde_json::value::RawValue>,
}
impl BulkByScrollFailure {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `bulk_item_response_failure` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_bulk_item_response_failure(
        &self,
    ) -> std::result::Result<
        crate::types::common::BulkItemResponseFailure,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "bulk_item_response_failure")
    }
    #[doc = "Parses the value as the `scrollable_hit_source_search_failure` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_scrollable_hit_source_search_failure(
        &self,
    ) -> std::result::Result<
        crate::types::common::ScrollableHitSourceSearchFailure,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "scrollable_hit_source_search_failure")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BulkByScrollResponseBase {
    #[doc = "The number of scroll responses pulled back by the reindex operation."]
    pub batches: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canceled: Option<String>,
    #[doc = "The number of documents that were successfully created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[doc = "The number of documents that were successfully deleted."]
    pub deleted: i64,
    #[doc = "The number of documents that were ignored."]
    pub noops: i64,
    #[doc = "The number of requests per second effectively executed during the reindex operation."]
    pub requests_per_second: f64,
    pub retries: crate::types::common::Retries,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slice_id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slices: Option<Vec<crate::types::common::BulkByScrollTaskStatusOrException>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throttled: Option<crate::types::common::Duration>,
    pub throttled_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The amount time before the throttling will end."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throttled_until: Option<crate::types::common::Duration>,
    #[doc = "When the throttling will end in milliseconds."]
    pub throttled_until_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The number of documents that were successfully processed."]
    pub total: i64,
    #[doc = "The number of documents that were successfully updated after the reindex operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<i64>,
    #[doc = "The number of version conflicts encountered by the reindex operation."]
    pub version_conflicts: i64,
    #[doc = "The list of failures that occurred during the operation."]
    pub failures: Vec<crate::types::common::BulkByScrollFailure>,
    #[doc = "Whether the operation timed out."]
    pub timed_out: bool,
    #[doc = "The time taken by the operation in milliseconds."]
    pub took: i64,
}

#[doc = "The status of a bulk by scroll task."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BulkByScrollTaskStatus {
    #[doc = "The number of scroll responses pulled back by the reindex operation."]
    pub batches: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canceled: Option<String>,
    #[doc = "The number of documents that were successfully created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[doc = "The number of documents that were successfully deleted."]
    pub deleted: i64,
    #[doc = "The number of documents that were ignored."]
    pub noops: i64,
    #[doc = "The number of requests per second effectively executed during the reindex operation."]
    pub requests_per_second: f64,
    pub retries: crate::types::common::Retries,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slice_id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slices: Option<Vec<crate::types::common::BulkByScrollTaskStatusOrException>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throttled: Option<crate::types::common::Duration>,
    pub throttled_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The amount time before the throttling will end."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throttled_until: Option<crate::types::common::Duration>,
    #[doc = "When the throttling will end in milliseconds."]
    pub throttled_until_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The number of documents that were successfully processed."]
    pub total: i64,
    #[doc = "The number of documents that were successfully updated after the reindex operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<i64>,
    #[doc = "The number of version conflicts encountered by the reindex operation."]
    pub version_conflicts: i64,
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct BulkByScrollTaskStatusOrException {
    raw: Box<serde_json::value::RawValue>,
}
impl BulkByScrollTaskStatusOrException {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `status` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_status(
        &self,
    ) -> std::result::Result<
        crate::types::common::BulkByScrollTaskStatus,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "status")
    }
    #[doc = "Parses the value as the `exception` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_exception(
        &self,
    ) -> std::result::Result<crate::types::common::ErrorCause, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "exception")
    }
}

#[doc = "The details of a bulk item operation failure."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BulkItemResponseFailure {
    pub cause: crate::types::common::ErrorCause,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::types::common::Id>,
    pub index: crate::types::common::IndexName,
    pub status: i32,
}

#[doc = "The byte value."]
pub type Byte = f64;

#[doc = "The size in bytes."]
pub type ByteCount = i64;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterStatistics {
    pub skipped: i32,
    pub successful: i32,
    pub total: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompletionStats {
    #[doc = "The per-field completion statistics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common::FieldSizeUsage>>,
    #[doc = "The human-readable size of memory used for completion."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount, in bytes, of memory used for completion across all shards assigned to the selected nodes."]
    pub size_in_bytes: crate::types::common::ByteCount,
}

#[doc = "The geographical bounds specified using coordinate values."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoordsGeoBounds {
    #[doc = "The bottom boundary latitude."]
    pub bottom: f64,
    #[doc = "The left boundary longitude."]
    pub left: f64,
    #[doc = "The right boundary longitude."]
    pub right: f64,
    #[doc = "The top boundary latitude."]
    pub top: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DFIIndependenceMeasure {
    #[serde(rename = "chisquared")]
    Chisquared,
    #[serde(rename = "saturated")]
    Saturated,
    #[serde(rename = "standardized")]
    Standardized,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DFRAfterEffect {
    #[serde(rename = "b")]
    B,
    #[serde(rename = "l")]
    L,
    #[serde(rename = "no")]
    No,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DFRBasicModel {
    #[serde(rename = "be")]
    Be,
    #[serde(rename = "d")]
    D,
    #[serde(rename = "g")]
    G,
    #[serde(rename = "if")]
    If,
    #[serde(rename = "in")]
    In,
    #[serde(rename = "ine")]
    Ine,
    #[serde(rename = "p")]
    P,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

pub type DataStreamName = String;

#[doc = "The date format pattern."]
pub type DateFormat = String;

pub type DateMath = String;

#[doc = "A date and time, either as a string whose format depends on the context (defaulting to ISO_8601) or the number of milliseconds since the epoch. OpenSearch accepts both as an input but will generally output a string. representation."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum DateTime {
    String(String),
    EpochTimeUnitMillis(crate::types::common::EpochTimeUnitMillis),
}
impl<'de> serde::Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = DateTime;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DateTime value")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(DateTime::String)
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::I64Deserializer::new(v))
                    .map(DateTime::EpochTimeUnitMillis)
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::U64Deserializer::new(v))
                    .map(DateTime::EpochTimeUnitMillis)
            }
            fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::F64Deserializer::new(v))
                    .map(DateTime::EpochTimeUnitMillis)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

pub type Distance = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DistanceUnit {
    #[serde(rename = "cm")]
    Cm,
    #[serde(rename = "ft")]
    Ft,
    #[serde(rename = "in")]
    In,
    #[serde(rename = "km")]
    Km,
    #[serde(rename = "m")]
    M,
    #[serde(rename = "mi")]
    Mi,
    #[serde(rename = "mm")]
    Mm,
    #[serde(rename = "nmi")]
    Nmi,
    #[serde(rename = "yd")]
    Yd,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[doc = "The document-level statistics."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DocStats {
    #[doc = "The total number of non-deleted documents across all primary shards assigned to the selected nodes. This number is based on documents in Lucene segments and may include documents from nested fields."]
    pub count: i64,
    #[doc = "The total number of deleted documents across all primary shards assigned to the selected nodes. This number is based on the number of documents stored in Lucene segments. OpenSearch reclaims the disk space previously occupied by the deleted Lucene documents when a segment is merged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<i64>,
}

#[doc = "The item level HTTP response status code during indexing."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DocStatus {
    #[doc = "The number of informational responses."]
    #[serde(rename = "1xx")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _1xx: Option<i64>,
    #[doc = "The number of successful responses."]
    #[serde(rename = "2xx")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _2xx: Option<i64>,
    #[doc = "The number of redirection responses."]
    #[serde(rename = "3xx")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _3xx: Option<i64>,
    #[doc = "The number of client error responses."]
    #[serde(rename = "4xx")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _4xx: Option<i64>,
    #[doc = "The number of server error responses."]
    #[serde(rename = "5xx")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _5xx: Option<i64>,
}

#[doc = "A duration. Units can be `nanos`, `micros`, `ms` (milliseconds), `s` (seconds), `m` (minutes), `h` (hours) and `d` (days). Also accepts `0` without a unit and `-1` to indicate an unspecified value."]
pub type Duration = String;

#[doc = "A date histogram interval, similar to `Duration`, with support for additional units: `w` (week), `M` (month), `q` (quarter), and `y` (year)."]
pub type DurationLarge = String;

pub type DurationValueUnitMicros = crate::types::common::UnitMicros;

pub type DurationValueUnitMillis = crate::types::common::UnitMillis;

pub type DurationValueUnitNanos = crate::types::common::UnitNanos;

pub type EpochTimeUnitMillis = crate::types::common::UnitMillis;

pub type EpochTimeUnitSeconds = crate::types::common::UnitSeconds;

#[derive(Debug, Clone)]
pub struct ErrorCause {
    pub caused_by: Option<Box<crate::types::common::ErrorCause>>,
    pub header:
        Option<std::collections::BTreeMap<String, crate::types::common::StringOrStringArray>>,
    #[doc = "A human-readable explanation of the error, in English."]
    pub reason: Option<String>,
    pub root_cause: Option<Vec<crate::types::common::ErrorCause>>,
    #[doc = "The server stack trace, present only if the `error_trace=true` parameter was sent with the request."]
    pub stack_trace: Option<String>,
    pub suppressed: Option<Vec<crate::types::common::ErrorCause>>,
    #[doc = "The type of error."]
    pub r#type: String,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ErrorCause {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ErrorCause;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ErrorCause object")
            }
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_caused_by: Option<Box<crate::types::common::ErrorCause>> = None;
                let mut f_header: Option<
                    std::collections::BTreeMap<String, crate::types::common::StringOrStringArray>,
                > = None;
                let mut f_reason: Option<String> = None;
                let mut f_root_cause: Option<Vec<crate::types::common::ErrorCause>> = None;
                let mut f_stack_trace: Option<String> = None;
                let mut f_suppressed: Option<Vec<crate::types::common::ErrorCause>> = None;
                let mut f_type: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "caused_by" => {
                            f_caused_by =
                                map.next_value::<Option<Box<crate::types::common::ErrorCause>>>()?
                        }
                        "header" => {
                            f_header = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common::StringOrStringArray,
                                >,
                            >>()?
                        }
                        "reason" => f_reason = map.next_value::<Option<String>>()?,
                        "root_cause" => {
                            f_root_cause =
                                map.next_value::<Option<Vec<crate::types::common::ErrorCause>>>()?
                        }
                        "stack_trace" => f_stack_trace = map.next_value::<Option<String>>()?,
                        "suppressed" => {
                            f_suppressed =
                                map.next_value::<Option<Vec<crate::types::common::ErrorCause>>>()?
                        }
                        "type" => f_type = Some(map.next_value::<String>()?),
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ErrorCause {
                    caused_by: f_caused_by,
                    header: f_header,
                    reason: f_reason,
                    root_cause: f_root_cause,
                    stack_trace: f_stack_trace,
                    suppressed: f_suppressed,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ErrorCause {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.caused_by {
            map.serialize_entry("caused_by", v)?;
        }
        if let Some(v) = &self.header {
            map.serialize_entry("header", v)?;
        }
        if let Some(v) = &self.reason {
            map.serialize_entry("reason", v)?;
        }
        if let Some(v) = &self.root_cause {
            map.serialize_entry("root_cause", v)?;
        }
        if let Some(v) = &self.stack_trace {
            map.serialize_entry("stack_trace", v)?;
        }
        if let Some(v) = &self.suppressed {
            map.serialize_entry("suppressed", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The base structure for error responses."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorResponseBase {
    pub error: crate::types::common::ErrorCause,
    #[doc = "The HTTP status code."]
    pub status: i32,
}

#[doc = "The path to a field or an array of paths. Some APIs support wildcards in the path, which allows you to select multiple fields."]
pub type Field = String;

#[doc = "The memory usage statistics for a field."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FieldMemoryUsage {
    #[doc = "The human-readable amount of memory used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The amount of memory used in bytes."]
    pub memory_size_in_bytes: crate::types::common::ByteCount,
}

#[doc = "The memory usage statistics for a field."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FieldSizeUsage {
    #[doc = "The human-readable size of memory used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The size of memory used in bytes."]
    pub size_in_bytes: crate::types::common::ByteCount,
}

#[doc = "The detailed field sort options."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FieldSort {
    #[doc = "The value to use when the field is missing."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing: Option<crate::types::common::FieldValue>,
    #[doc = "The mode for sorting on array fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<crate::types::common::SortMode>,
    #[doc = "The nested path sort options."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nested: Option<crate::types::common::NestedSortValue>,
    #[doc = "The numeric type to use for sorting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numeric_type: Option<crate::types::common::FieldSortNumericType>,
    #[doc = "The sort order direction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<crate::types::common::SortOrder>,
    #[doc = "The type to use for unmapped fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unmapped_type: Option<crate::types::common_mapping::FieldType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FieldSortNumericType {
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "date_nanos")]
    DateNanos,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "long")]
    Long,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[doc = "A field value."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum FieldValue {
    Boolean(bool),
    Number(f64),
    String(String),
    #[doc = "An explicit null value."]
    Null,
}
impl<'de> serde::Deserialize<'de> for FieldValue {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = FieldValue;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FieldValue value")
            }
            fn visit_bool<E>(self, v: bool) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::BoolDeserializer::new(v))
                    .map(FieldValue::Boolean)
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::I64Deserializer::new(v))
                    .map(FieldValue::Number)
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::U64Deserializer::new(v))
                    .map(FieldValue::Number)
            }
            fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::F64Deserializer::new(v))
                    .map(FieldValue::Number)
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(FieldValue::String)
            }
            fn visit_unit<E>(self) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(FieldValue::Null)
            }
            fn visit_none<E>(self) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(FieldValue::Null)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

pub type FieldWithDirection = std::collections::BTreeMap<String, crate::types::common::SortOrder>;

pub type FieldWithOrder = std::collections::BTreeMap<String, crate::types::common::FieldSort>;

#[doc = "The statistics about field data memory usage."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FielddataStats {
    #[doc = "The number of times field data was evicted from memory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evictions: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common::FieldMemoryUsage>>,
    #[doc = "The human-readable amount of memory used for field data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The amount of memory used for field data in bytes."]
    pub memory_size_in_bytes: crate::types::common::ByteCount,
}

#[doc = "A comma-separated list or a wildcard expression specifying the fields to include in the statistics. Used as the default list unless a specific field list is provided in the `completion_fields` or `fielddata_fields` parameters."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum Fields {
    Field(crate::types::common::Field),
    Array(Vec<crate::types::common::Field>),
}
impl<'de> serde::Deserialize<'de> for Fields {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = Fields;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Fields value")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(Fields::Field)
            }
            fn visit_seq<A>(self, seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
                    .map(Fields::Array)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[doc = "The statistics about flush operations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FlushStats {
    #[doc = "The number of periodic flush operations."]
    pub periodic: i64,
    #[doc = "The total number of flush operations."]
    pub total: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_time: Option<crate::types::common::Duration>,
    pub total_time_in_millis: crate::types::common::DurationValueUnitMillis,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum Fuzziness {
    String(String),
    Integer(i32),
}
impl<'de> serde::Deserialize<'de> for Fuzziness {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = Fuzziness;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Fuzziness value")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(Fuzziness::String)
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::I64Deserializer::new(v))
                    .map(Fuzziness::Integer)
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::U64Deserializer::new(v))
                    .map(Fuzziness::Integer)
            }
            fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::F64Deserializer::new(v))
                    .map(Fuzziness::Integer)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[doc = "A geo-bounding box. It can be represented in the following ways:  - As 4 top/bottom/left/right coordinates.  - As 2 top_left/bottom_right points.  - As 2 top_right/bottom_left points.  - As a Well Known Text (WKT) bounding box."]
#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct GeoBounds {
    raw: Box<serde_json::value::RawValue>,
}
impl GeoBounds {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `coords` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_coords(
        &self,
    ) -> std::result::Result<crate::types::common::CoordsGeoBounds, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "coords")
    }
    #[doc = "Parses the value as the `tlbr` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_tlbr(
        &self,
    ) -> std::result::Result<
        crate::types::common::TopLeftBottomRightGeoBounds,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "tlbr")
    }
    #[doc = "Parses the value as the `trbl` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_trbl(
        &self,
    ) -> std::result::Result<
        crate::types::common::TopRightBottomLeftGeoBounds,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "trbl")
    }
    #[doc = "Parses the value as the `wkt` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_wkt(
        &self,
    ) -> std::result::Result<crate::types::common::WktGeoBounds, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "wkt")
    }
}

#[doc = "The options for sorting by geo distance."]
#[derive(Debug, Clone)]
pub struct GeoDistanceSort {
    #[doc = "The algorithm to use for distance calculation."]
    pub distance_type: Option<crate::types::common::GeoDistanceType>,
    #[doc = "Whether to ignore unmapped fields and not sort based on them."]
    pub ignore_unmapped: Option<bool>,
    #[doc = "The mode for sorting on array or multi-valued fields."]
    pub mode: Option<crate::types::common::SortMode>,
    pub nested: Option<crate::types::common::NestedSortValue>,
    #[doc = "The direction of the sort order."]
    pub order: Option<crate::types::common::SortOrder>,
    #[doc = "The unit of distance measurement."]
    pub unit: Option<crate::types::common::DistanceUnit>,
    pub validation_method: Option<crate::types::common_query_dsl::GeoValidationMethod>,
    pub additional: std::collections::BTreeMap<String, Vec<crate::types::common::GeoLocation>>,
}
impl<'de> serde::Deserialize<'de> for GeoDistanceSort {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GeoDistanceSort;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoDistanceSort object")
            }
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_distance_type: Option<crate::types::common::GeoDistanceType> = None;
                let mut f_ignore_unmapped: Option<bool> = None;
                let mut f_mode: Option<crate::types::common::SortMode> = None;
                let mut f_nested: Option<crate::types::common::NestedSortValue> = None;
                let mut f_order: Option<crate::types::common::SortOrder> = None;
                let mut f_unit: Option<crate::types::common::DistanceUnit> = None;
                let mut f_validation_method: Option<
                    crate::types::common_query_dsl::GeoValidationMethod,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Vec<crate::types::common::GeoLocation>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "distance_type" => f_distance_type = map.next_value::<Option<crate::types::common::GeoDistanceType>>()?,
                        "ignore_unmapped" => f_ignore_unmapped = map.next_value::<Option<bool>>()?,
                        "mode" => f_mode = map.next_value::<Option<crate::types::common::SortMode>>()?,
                        "nested" => f_nested = map.next_value::<Option<crate::types::common::NestedSortValue>>()?,
                        "order" => f_order = map.next_value::<Option<crate::types::common::SortOrder>>()?,
                        "unit" => f_unit = map.next_value::<Option<crate::types::common::DistanceUnit>>()?,
                        "validation_method" => f_validation_method = map.next_value::<Option<crate::types::common_query_dsl::GeoValidationMethod>>()?,
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GeoDistanceSort {
                    distance_type: f_distance_type,
                    ignore_unmapped: f_ignore_unmapped,
                    mode: f_mode,
                    nested: f_nested,
                    order: f_order,
                    unit: f_unit,
                    validation_method: f_validation_method,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GeoDistanceSort {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.distance_type {
            map.serialize_entry("distance_type", v)?;
        }
        if let Some(v) = &self.ignore_unmapped {
            map.serialize_entry("ignore_unmapped", v)?;
        }
        if let Some(v) = &self.mode {
            map.serialize_entry("mode", v)?;
        }
        if let Some(v) = &self.nested {
            map.serialize_entry("nested", v)?;
        }
        if let Some(v) = &self.order {
            map.serialize_entry("order", v)?;
        }
        if let Some(v) = &self.unit {
            map.serialize_entry("unit", v)?;
        }
        if let Some(v) = &self.validation_method {
            map.serialize_entry("validation_method", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GeoDistanceType {
    #[serde(rename = "arc")]
    Arc,
    #[serde(rename = "plane")]
    Plane,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[doc = "The geohash string representation of a geographical location."]
pub type GeoHash = String;

#[doc = "The geographical location specified using a geohash."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeoHashLocation {
    pub geohash: crate::types::common::GeoHash,
}

#[doc = "The level of geohash precision, which can be expressed as a geohash length between 1 and 12 or as a distance measure, such as \"1km\" or \"10m\"."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum GeoHashPrecision {
    GeohashLength(i32),
    Distance(String),
}
impl<'de> serde::Deserialize<'de> for GeoHashPrecision {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = GeoHashPrecision;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoHashPrecision value")
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::I64Deserializer::new(v))
                    .map(GeoHashPrecision::GeohashLength)
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::U64Deserializer::new(v))
                    .map(GeoHashPrecision::GeohashLength)
            }
            fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::F64Deserializer::new(v))
                    .map(GeoHashPrecision::GeohashLength)
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(GeoHashPrecision::Distance)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[doc = "A latitude/longitude as a two-dimensional point. It can be represented in the following ways:  - As a `{lat, long}` object.  - As a geohash value.  - As a `[lon, lat]` array.  - As a string in `<lat>, <lon>` or WKT point format."]
#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct GeoLocation {
    raw: Box<serde_json::value::RawValue>,
}
impl GeoLocation {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `latlon` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_latlon(
        &self,
    ) -> std::result::Result<crate::types::common::LatLonGeoLocation, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "latlon")
    }
    #[doc = "Parses the value as the `geohash` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_geohash(
        &self,
    ) -> std::result::Result<crate::types::common::GeoHashLocation, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "geohash")
    }
    #[doc = "Parses the value as the `coords` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_coords(&self) -> std::result::Result<Vec<f64>, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "coords")
    }
    #[doc = "Parses the value as the `text` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_text(&self) -> std::result::Result<String, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "text")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GeoShapeRelation {
    #[serde(rename = "contains")]
    Contains,
    #[serde(rename = "disjoint")]
    Disjoint,
    #[serde(rename = "intersects")]
    Intersects,
    #[serde(rename = "within")]
    Within,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[doc = "A map tile reference, represented as `{zoom}/{x}/{y}`."]
pub type GeoTile = String;

#[doc = "The statistics about get operations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetStats {
    #[doc = "The number of current get operations."]
    pub current: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exists_time: Option<crate::types::common::Duration>,
    pub exists_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The total number of successful get operations."]
    pub exists_total: i64,
    #[serde(rename = "getTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_time: Option<crate::types::common::Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing_time: Option<crate::types::common::Duration>,
    pub missing_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The total number of failed get operations due to missing documents."]
    pub missing_total: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<crate::types::common::Duration>,
    pub time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The total number of get operations."]
    pub total: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum HealthStatus {
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "red")]
    Red,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[doc = "The hostname or IP address."]
pub type Host = String;

#[doc = "The HTTP headers."]
pub type HttpHeaders =
    std::collections::BTreeMap<String, crate::types::common::StringOrStringArray>;

#[doc = "The unique identifier of a node."]
pub type HumanReadableByteCount = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IBDistribution {
    #[serde(rename = "ll")]
    Ll,
    #[serde(rename = "spl")]
    Spl,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IBLambda {
    #[serde(rename = "df")]
    Df,
    #[serde(rename = "ttf")]
    Ttf,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[doc = "The unique identifier for a resource."]
pub type Id = String;

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum Ids {
    Id(crate::types::common::Id),
    Array(Vec<crate::types::common::Id>),
}
impl<'de> serde::Deserialize<'de> for Ids {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = Ids;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Ids value")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(Ids::Id)
            }
            fn visit_seq<A>(self, seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
                    .map(Ids::Array)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[doc = "The name of an index alias."]
pub type IndexAlias = String;

pub type IndexName = String;

#[doc = "The statistics about indexing operations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexingStats {
    #[doc = "The number of current delete operations."]
    pub delete_current: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<crate::types::common::Duration>,
    pub delete_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The total number of delete operations."]
    pub delete_total: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_status: Option<crate::types::common::DocStatus>,
    #[doc = "The number of current indexing operations."]
    pub index_current: i64,
    #[doc = "The number of failed indexing operations."]
    pub index_failed: i64,
    #[doc = "The total amount of time spent on indexing operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time spent on indexing operations in milliseconds."]
    pub index_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The total number of indexing operations."]
    pub index_total: i64,
    #[doc = "Whether indexing is currently throttled."]
    pub is_throttled: bool,
    #[doc = "The total number of noop update operations."]
    pub noop_update_total: i64,
    #[doc = "The total amount of time spent throttling indexing operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throttle_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time spent throttling indexing operations in milliseconds."]
    pub throttle_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub types: Option<std::collections::BTreeMap<String, crate::types::common::IndexingStats>>,
}

#[doc = "A comma-separated list of data streams, indexes, and aliases used to limit the request. Supports wildcards (`*`). To target all data streams and indexes, omit this parameter or use `*` or `_all`."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum Indices {
    IndexName(crate::types::common::IndexName),
    Array(Vec<crate::types::common::IndexName>),
}
impl<'de> serde::Deserialize<'de> for Indices {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = Indices;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Indices value")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(Indices::IndexName)
            }
            fn visit_seq<A>(self, seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
                    .map(Indices::Array)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndicesResponseBase {
    #[doc = "For a successful response, this value is always true. On failure, an exception is returned instead."]
    pub acknowledged: bool,
    #[serde(rename = "_shards")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shards: Option<crate::types::common::ShardStatistics>,
}

#[doc = "The result of an inline get operation."]
#[derive(Debug, Clone)]
pub struct InlineGet {
    #[doc = "The primary term of the document."]
    pub primary_term: Option<i64>,
    pub routing: Option<crate::types::common::Routing>,
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    #[doc = "The source of the document."]
    pub source: Option<Box<serde_json::value::RawValue>>,
    #[doc = "The fields retrieved from the document."]
    pub fields: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "Whether the document was found."]
    pub found: bool,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for InlineGet {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = InlineGet;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a InlineGet object")
            }
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_primary_term: Option<i64> = None;
                let mut f_routing: Option<crate::types::common::Routing> = None;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut f_source: Option<Box<serde_json::value::RawValue>> = None;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut f_found: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_primary_term" => f_primary_term = map.next_value::<Option<i64>>()?,
                        "_routing" => {
                            f_routing = map.next_value::<Option<crate::types::common::Routing>>()?
                        }
                        "_seq_no" => {
                            f_seq_no =
                                map.next_value::<Option<crate::types::common::SequenceNumber>>()?
                        }
                        "_source" => {
                            f_source =
                                map.next_value::<Option<Box<serde_json::value::RawValue>>>()?
                        }
                        "fields" => {
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?
                        }
                        "found" => f_found = Some(map.next_value::<bool>()?),
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(InlineGet {
                    primary_term: f_primary_term,
                    routing: f_routing,
                    seq_no: f_seq_no,
                    source: f_source,
                    fields: f_fields,
                    found: f_found.ok_or_else(|| serde::de::Error::missing_field("found"))?,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for InlineGet {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.primary_term {
            map.serialize_entry("_primary_term", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("_routing", v)?;
        }
        if let Some(v) = &self.seq_no {
            map.serialize_entry("_seq_no", v)?;
        }
        if let Some(v) = &self.source {
            map.serialize_entry("_source", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        map.serialize_entry("found", &self.found)?;
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The inline get operation results for user-defined dictionaries."]
#[derive(Debug, Clone)]
pub struct InlineGetDictUserDefined {
    #[doc = "The primary term of the document."]
    pub primary_term: Option<i64>,
    pub routing: Option<crate::types::common::Routing>,
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    #[doc = "The source of the document."]
    pub source: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The fields retrieved from the document."]
    pub fields: Option<
        std::collections::BTreeMap<
            String,
            std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
        >,
    >,
    #[doc = "Whether the document was found."]
    pub found: bool,
    pub additional: std::collections::BTreeMap<
        String,
        std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
    >,
}
impl<'de> serde::Deserialize<'de> for InlineGetDictUserDefined {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = InlineGetDictUserDefined;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a InlineGetDictUserDefined object")
            }
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_primary_term: Option<i64> = None;
                let mut f_routing: Option<crate::types::common::Routing> = None;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut f_source: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut f_fields: Option<
                    std::collections::BTreeMap<
                        String,
                        std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                    >,
                > = None;
                let mut f_found: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_primary_term" => f_primary_term = map.next_value::<Option<i64>>()?,
                        "_routing" => {
                            f_routing = map.next_value::<Option<crate::types::common::Routing>>()?
                        }
                        "_seq_no" => {
                            f_seq_no =
                                map.next_value::<Option<crate::types::common::SequenceNumber>>()?
                        }
                        "_source" => {
                            f_source = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?
                        }
                        "fields" => {
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    std::collections::BTreeMap<
                                        String,
                                        Box<serde_json::value::RawValue>,
                                    >,
                                >,
                            >>()?
                        }
                        "found" => f_found = Some(map.next_value::<bool>()?),
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(InlineGetDictUserDefined {
                    primary_term: f_primary_term,
                    routing: f_routing,
                    seq_no: f_seq_no,
                    source: f_source,
                    fields: f_fields,
                    found: f_found.ok_or_else(|| serde::de::Error::missing_field("found"))?,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for InlineGetDictUserDefined {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.primary_term {
            map.serialize_entry("_primary_term", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("_routing", v)?;
        }
        if let Some(v) = &self.seq_no {
            map.serialize_entry("_seq_no", v)?;
        }
        if let Some(v) = &self.source {
            map.serialize_entry("_source", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        map.serialize_entry("found", &self.found)?;
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum InlineScript {
    Source(String),
    Object(crate::types::common::InlineScriptObject),
}
impl<'de> serde::Deserialize<'de> for InlineScript {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = InlineScript;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a InlineScript value")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(InlineScript::Source)
            }
            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(InlineScript::Object)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InlineScriptObject {
    #[doc = "Specifies any named parameters that are passed into the script as variables. Use parameters instead of hard-coded values to decrease compilation time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<crate::types::common::ScriptLanguage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::BTreeMap<String, String>>,
    #[doc = "The script source."]
    pub source: String,
}

#[doc = "The IP address."]
pub type Ip = String;

#[doc = "The geographical location specified using latitude and longitude."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LatLonGeoLocation {
    #[doc = "The latitude coordinate."]
    pub lat: f64,
    #[doc = "The longitude coordinate."]
    pub lon: f64,
}

#[doc = "The statistics about merge operations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MergesStats {
    #[doc = "The number of current merge operations."]
    pub current: i64,
    #[doc = "The number of documents in current merge operations."]
    pub current_docs: i64,
    #[doc = "The size of current merge operations in a human-readable format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The size of current merge operations in bytes."]
    pub current_size_in_bytes: crate::types::common::ByteCount,
    #[doc = "The total number of merge operations."]
    pub total: i64,
    #[doc = "The total auto-throttle size in a human-readable format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_auto_throttle: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total auto-throttle size in bytes."]
    pub total_auto_throttle_in_bytes: crate::types::common::ByteCount,
    #[doc = "The total number of documents that have been merged."]
    pub total_docs: i64,
    #[doc = "The total size of merge operations in a human-readable format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total size of merge operations in bytes."]
    pub total_size_in_bytes: crate::types::common::ByteCount,
    #[doc = "The total amount of time merge operations have been stopped."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_stopped_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time merge operations have been stopped in milliseconds."]
    pub total_stopped_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The total amount of time merge operations have been throttled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_throttled_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time merge operations have been throttled in milliseconds."]
    pub total_throttled_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The total amount of time spent on merge operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time spent on merge operations in milliseconds."]
    pub total_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The number of unreferenced file cleanups performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unreferenced_file_cleanups_performed: Option<i64>,
}

#[doc = "The custom metadata attached to a resource."]
pub type Metadata = std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

#[doc = "The minimum number of terms that should match as an integer, percentage, or range."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum MinimumShouldMatch {
    Integer(i32),
    String(String),
}
impl<'de> serde::Deserialize<'de> for MinimumShouldMatch {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = MinimumShouldMatch;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MinimumShouldMatch value")
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::I64Deserializer::new(v))
                    .map(MinimumShouldMatch::Integer)
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::U64Deserializer::new(v))
                    .map(MinimumShouldMatch::Integer)
            }
            fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::F64Deserializer::new(v))
                    .map(MinimumShouldMatch::Integer)
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(MinimumShouldMatch::String)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

pub type MultiTermQueryRewrite = String;

#[doc = "The name of a resource or configuration element."]
pub type Name = String;

#[doc = "A comma-separated list of aliases to retrieve. Supports wildcards (`*`).  To retrieve all aliases, omit this parameter or use `*` or `_all`."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum Names {
    Name(crate::types::common::Name),
    Array(Vec<crate::types::common::Name>),
}
impl<'de> serde::Deserialize<'de> for Names {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = Names;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Names value")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(Names::Name)
            }
            fn visit_seq<A>(self, seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
                    .map(Names::Array)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedSortValue {
    #[doc = "The filter to apply to nested objects."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<crate::types::common_query_dsl::QueryContainer>,
    #[doc = "The maximum number of children to consider for sorting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_children: Option<i32>,
    #[doc = "How to sort hierarchical nested objects."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nested: Option<Box<crate::types::common::NestedSortValue>>,
    #[doc = "The path to the nested objects."]
    pub path: crate::types::common::Field,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeAttributes {
    #[doc = "A list of node attributes."]
    pub attributes: std::collections::BTreeMap<String, String>,
    pub ephemeral_id: crate::types::common::Id,
    #[doc = "The roles of the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::types::common::NodeId>,
    pub name: crate::types::common::NodeName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<crate::types::common::NodeRoles>,
    pub transport_address: crate::types::common::TransportAddress,
}

#[doc = "The unique identifier of a node."]
pub type NodeId = String;

#[doc = "A comma-separated list of node IDs used to filter results. Supports [node filters](https://opensearch.org/docs/latest/api-reference/nodes-apis/index/#node-filters)."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum NodeIds {
    NodeId(crate::types::common::NodeId),
    Array(Vec<crate::types::common::NodeId>),
}
impl<'de> serde::Deserialize<'de> for NodeIds {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = NodeIds;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeIds value")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(NodeIds::NodeId)
            }
            fn visit_seq<A>(self, seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
                    .map(NodeIds::Array)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[doc = "The name of the node."]
pub type NodeName = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NodeRole {
    #[serde(rename = "client")]
    Client,
    #[serde(rename = "coordinating_only")]
    CoordinatingOnly,
    #[serde(rename = "data")]
    Data,
    #[serde(rename = "data_cold")]
    DataCold,
    #[serde(rename = "data_content")]
    DataContent,
    #[serde(rename = "data_frozen")]
    DataFrozen,
    #[serde(rename = "data_hot")]
    DataHot,
    #[serde(rename = "data_warm")]
    DataWarm,
    #[serde(rename = "ingest")]
    Ingest,
    #[serde(rename = "ml")]
    Ml,
    #[serde(rename = "remote_cluster_client")]
    RemoteClusterClient,
    #[serde(rename = "transform")]
    Transform,
    #[serde(rename = "voting_only")]
    VotingOnly,
    #[serde(rename = "master")]
    Master,
    #[serde(rename = "cluster_manager")]
    ClusterManager,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[doc = "The role assigned to the node."]
pub type NodeRoles = Vec<crate::types::common::NodeRole>;

#[doc = "The information about a shard on a node."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeShard {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocation_id: Option<std::collections::BTreeMap<String, crate::types::common::Id>>,
    pub index: crate::types::common::IndexName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<crate::types::common::NodeName>,
    pub primary: bool,
    #[doc = "The recovery source information for this shard."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_source: Option<std::collections::BTreeMap<String, crate::types::common::Id>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relocating_node: Option<crate::types::common::NodeId>,
    #[doc = "Indicates if this shard is configured for search operations only"]
    #[serde(rename = "searchOnly")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_only: Option<bool>,
    pub shard: i32,
    pub state: crate::types::indices_stats::ShardRoutingState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unassigned_info: Option<crate::types::cluster_allocation_explain::UnassignedInformation>,
}

#[doc = "The statistics about node operations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeStatistics {
    #[doc = "The number of nodes that rejected the request or failed to respond. If this value is not 0, then a reason for the rejection or failure is included in the response."]
    pub failed: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<crate::types::common::ErrorCause>>,
    #[doc = "The number of nodes that responded successfully to the request."]
    pub successful: i32,
    #[doc = "The total number of nodes selected by the request."]
    pub total: i32,
}

#[doc = "The version information for OpenSearch."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OpenSearchVersionInfo {
    #[doc = "When the version was built."]
    pub build_date: crate::types::common::DateTime,
    #[doc = "The flavor of the build."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_flavor: Option<String>,
    #[doc = "The hash of the build."]
    pub build_hash: String,
    #[doc = "Whether this is a snapshot build."]
    pub build_snapshot: bool,
    #[doc = "The type of the build."]
    pub build_type: String,
    #[doc = "The distribution name."]
    pub distribution: String,
    #[doc = "The version of Lucene being used."]
    pub lucene_version: crate::types::common::VersionString,
    #[doc = "The minimum compatible index version."]
    pub minimum_index_compatibility_version: crate::types::common::VersionString,
    #[doc = "The minimum compatible wire protocol version."]
    pub minimum_wire_compatibility_version: crate::types::common::VersionString,
    #[doc = "The version number."]
    pub number: String,
}

#[doc = "The percentage value as a number."]
pub type PercentageNumber = f64;

#[doc = "The percentage value as a string."]
pub type PercentageString = String;

#[doc = "The time taken by different phases of the search."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhaseTook {
    #[doc = "The time taken for the `can_match` phase."]
    pub can_match: i64,
    #[doc = "The time taken for the distributed frequency search `pre-query` phase."]
    pub dfs_pre_query: i64,
    #[doc = "The time taken for the distributed frequency search query phase."]
    pub dfs_query: i64,
    #[doc = "The time taken for the `expand` phase."]
    pub expand: i64,
    #[doc = "The time taken for the `fetch` phase."]
    pub fetch: i64,
    #[doc = "The time taken for the `query` phase."]
    pub query: i64,
}

pub type PipelineName = String;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PluginStats {
    pub classname: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_foldername: Option<String>,
    pub description: String,
    pub extended_plugins: Vec<String>,
    pub has_native_controller: bool,
    pub java_version: crate::types::common::VersionString,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub licensed: Option<bool>,
    pub name: crate::types::common::Name,
    pub opensearch_version: crate::types::common::VersionString,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional_extended_plugins: Option<Vec<String>>,
    pub version: crate::types::common::VersionString,
}

#[doc = "The statistics about query cache usage."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryCacheStats {
    #[doc = "The total number of entries added to the query cache across all shards assigned to the selected nodes. This number includes all current and evicted entries."]
    pub cache_count: i64,
    #[doc = "The total number of entries currently stored in the query cache across all shards assigned to the selected nodes."]
    pub cache_size: i64,
    #[doc = "The total number of query cache evictions across all shards assigned to the selected nodes."]
    pub evictions: i64,
    #[doc = "The total number of query cache hits across all shards assigned to the selected nodes."]
    pub hit_count: i64,
    #[doc = "The human-readable amount of memory used for the query cache."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount, in bytes, of memory used for the query cache across all shards assigned to the selected nodes."]
    pub memory_size_in_bytes: crate::types::common::ByteCount,
    #[doc = "The total number of query cache misses across all shards assigned to the selected nodes."]
    pub miss_count: i64,
    #[doc = "The total number of hits and misses stored in the query cache across all shards assigned to the selected nodes."]
    pub total_count: i64,
}

#[doc = "The statistics about recovery operations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecoveryStats {
    #[doc = "The number of ongoing recoveries for which this node is the source."]
    pub current_as_source: i64,
    #[doc = "The number of ongoing recoveries for which this node is the target."]
    pub current_as_target: i64,
    #[doc = "The total amount of time recovery operations have been throttled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throttle_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time recovery operations have been throttled in milliseconds."]
    pub throttle_time_in_millis: crate::types::common::DurationValueUnitMillis,
}

#[doc = "The statistics about refresh operations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RefreshStats {
    #[doc = "The total number of external refresh operations."]
    pub external_total: i64,
    #[doc = "The total amount of time spent on external refresh operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_total_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time spent on external refresh operations in milliseconds."]
    pub external_total_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The number of refresh listeners."]
    pub listeners: i64,
    #[doc = "The total number of refresh operations."]
    pub total: i64,
    #[doc = "The total amount of time spent on refresh operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time spent on refresh operations in milliseconds."]
    pub total_time_in_millis: crate::types::common::DurationValueUnitMillis,
}

#[doc = "The name of a relation in a join field."]
pub type RelationName = String;

#[doc = "Any statistics related to downloads to the remote segment store."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteStoreDownloadStats {
    #[doc = "The total amount of data downloaded from the remote segment store."]
    pub total_download_size: crate::types::common::RemoteStoreUploadDownloadStats,
    #[doc = "The total amount of time spent on downloads from the remote segment store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_time_spent: Option<crate::types::common::Duration>,
    #[doc = "The total duration, in milliseconds, spent on downloads from the remote segment store."]
    pub total_time_spent_in_millis: crate::types::common::DurationValueUnitMillis,
}

#[doc = "The statistics related to remote segment store operations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteStoreStats {
    pub download: crate::types::common::RemoteStoreDownloadStats,
    pub upload: crate::types::common::RemoteStoreUploadStats,
}

#[doc = "The statistics related to remote translog operations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteStoreTranslogStats {
    pub upload: crate::types::common::RemoteStoreTranslogUploadStats,
}

#[doc = "The statistics related to uploads to the remote translog store."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteStoreTranslogUploadStats {
    pub total_upload_size: crate::types::common::RemoteStoreTranslogUploadTotalUploadSizeStats,
    pub total_uploads: crate::types::common::RemoteStoreTranslogUploadTotalUploadsStats,
}

#[doc = "The total amount of data uploaded to the remote translog store."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteStoreTranslogUploadTotalUploadSizeStats {
    #[doc = "The number of bytes that failed to upload to the remote translog store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The number of bytes that failed to upload to the remote translog store."]
    pub failed_bytes: crate::types::common::ByteCount,
    #[doc = "The number of bytes successfully uploaded to the remote translog store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The number of bytes successfully uploaded to the remote translog store."]
    pub started_bytes: crate::types::common::ByteCount,
    #[doc = "The number of bytes successfully uploaded to the remote translog store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The number of bytes successfully uploaded to the remote translog store."]
    pub succeeded_bytes: crate::types::common::ByteCount,
}

#[doc = "The number of syncs to the remote translog store."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteStoreTranslogUploadTotalUploadsStats {
    #[doc = "The number of failed upload syncs to the remote translog store."]
    pub failed: i64,
    #[doc = "The number of upload syncs to the remote translog store that have started."]
    pub started: i64,
    #[doc = "The number of successful upload syncs to the remote translog store."]
    pub succeeded: i64,
}

#[doc = "The amount of data, in bytes, uploaded or downloaded to/from the remote segment store."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteStoreUploadDownloadStats {
    #[doc = "The number of bytes that failed to upload to/from the remote segment store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The number of bytes that failed to upload to/from the remote segment store."]
    pub failed_bytes: crate::types::common::ByteCount,
    #[doc = "The number of bytes to upload/download to/from the remote segment store after the upload/download has started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The number of bytes to upload/download to/from the remote segment store after the upload/download has started."]
    pub started_bytes: crate::types::common::ByteCount,
    #[doc = "The number of bytes successfully uploaded/downloaded to/from the remote segment store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The number of bytes successfully uploaded/downloaded to/from the remote segment store."]
    pub succeeded_bytes: crate::types::common::ByteCount,
}

#[doc = "Statistics related to segment store upload backpressure."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteStoreUploadPressureStats {
    #[doc = "The total number of requests rejected due to segment store upload backpressure."]
    pub total_rejections: i64,
}

#[doc = "The amount of lag during upload between the remote segment store and the local store."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteStoreUploadRefreshSizeLagStats {
    #[doc = "The maximum amount of lag, in bytes, during the upload refresh between the remote segment store and the local store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The maximum amount of lag, in bytes, during the upload refresh between the remote segment store and the local store."]
    pub max_bytes: crate::types::common::ByteCount,
    #[doc = "The total number of bytes that lagged during the upload refresh between the remote segment store and the local store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total number of bytes that lagged during the upload refresh between the remote segment store and the local store."]
    pub total_bytes: crate::types::common::ByteCount,
}

#[doc = "Any statistics related to uploads to the remote segment store."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteStoreUploadStats {
    #[doc = "The maximum duration that the remote refresh is behind the local refresh."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_refresh_time_lag: Option<crate::types::common::Duration>,
    #[doc = "The maximum duration, in milliseconds, that the remote refresh is behind the local refresh."]
    pub max_refresh_time_lag_in_millis: crate::types::common::DurationValueUnitMillis,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pressure: Option<crate::types::common::RemoteStoreUploadPressureStats>,
    pub refresh_size_lag: crate::types::common::RemoteStoreUploadRefreshSizeLagStats,
    #[doc = "The total amount of time spent on uploads to the remote segment store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_time_spent: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time, in milliseconds, spent on uploads to the remote segment store."]
    pub total_time_spent_in_millis: crate::types::common::DurationValueUnitMillis,
    pub total_upload_size: crate::types::common::RemoteStoreUploadDownloadStats,
}

#[doc = "The statistics about request cache operations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RequestCacheStats {
    #[doc = "The number of request cache evictions."]
    pub evictions: i64,
    #[doc = "The number of request cache hits."]
    pub hit_count: i64,
    #[doc = "The size of the request cache in a human-readable format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The size of the request cache in bytes."]
    pub memory_size_in_bytes: crate::types::common::ByteCount,
    #[doc = "The number of request cache misses."]
    pub miss_count: i64,
}

#[doc = "The statistics about request operations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RequestStats {
    #[doc = "The number of current request operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current: Option<i64>,
    #[doc = "The total time spent on request operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<crate::types::common::Duration>,
    #[doc = "The total time spent on request operations in milliseconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "The total number of request operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

#[doc = "Any statistics about CPU and memory usage."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResourceStat {
    pub cpu_time_in_nanos: crate::types::common::DurationValueUnitNanos,
    pub memory_in_bytes: crate::types::common::ByteCount,
}

#[doc = "The statistics about resource usage."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResourceStats {
    pub average: crate::types::common::ResourceStat,
    pub max: crate::types::common::ResourceStat,
    pub min: crate::types::common::ResourceStat,
    pub thread_info: crate::types::common::ThreadInfo,
    pub total: crate::types::common::ResourceStat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Result {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "noop")]
    Noop,
    #[serde(rename = "not_found")]
    NotFound,
    #[serde(rename = "updated")]
    Updated,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[doc = "The retry statistics for bulk and search operations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Retries {
    #[doc = "The number of bulk operation retries."]
    pub bulk: i64,
    #[doc = "The number of search operation retries."]
    pub search: i64,
}

#[doc = "The routing value for the document."]
pub type Routing = String;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScoreSort {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<crate::types::common::SortOrder>,
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Script {
    raw: Box<serde_json::value::RawValue>,
}
impl Script {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `inline` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_inline(
        &self,
    ) -> std::result::Result<crate::types::common::InlineScript, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "inline")
    }
    #[doc = "Parses the value as the `stored` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_stored(
        &self,
    ) -> std::result::Result<crate::types::common::StoredScriptId, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "stored")
    }
}

#[doc = "The configuration for a script field."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScriptField {
    #[doc = "Whether to ignore failures during script execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "The script to execute for this field."]
    pub script: crate::types::common::Script,
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ScriptLanguage {
    raw: Box<serde_json::value::RawValue>,
}
impl ScriptLanguage {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `builtin` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_builtin(
        &self,
    ) -> std::result::Result<
        crate::types::common::BuiltinScriptLanguage,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "builtin")
    }
    #[doc = "Parses the value as the `custom` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_custom(&self) -> std::result::Result<String, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "custom")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScriptSort {
    #[doc = "The mode for sorting on array or multi-valued fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<crate::types::common::SortMode>,
    #[doc = "The nested path and filter for nested sorting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nested: Option<crate::types::common::NestedSortValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<crate::types::common::SortOrder>,
    #[doc = "The script to use for sorting."]
    pub script: crate::types::common::Script,
    #[doc = "The type of the script sort value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::types::common::ScriptSortType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ScriptSortType {
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "version")]
    Version,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

pub type ScrollId = String;

#[doc = "The details of a search failure during scrolling."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScrollableHitSourceSearchFailure {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<crate::types::common::IndexName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
    pub reason: crate::types::common::ErrorCause,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shard: Option<i32>,
    pub status: i32,
}

#[doc = "The statistics about search operations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchStats {
    #[doc = "The average slice count of all search requests. This is computed as the total slice count divided by the total number of concurrent search requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrent_avg_slice_count: Option<f64>,
    #[doc = "The number of currently running query operations using concurrent segment search."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrent_query_current: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrent_query_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time taken by all query operations using concurrent segment search, in milliseconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrent_query_time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "The total number of query operations using concurrent segment search."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrent_query_total: Option<i64>,
    #[doc = "The number of currently running shard fetch operations."]
    pub fetch_current: i64,
    #[doc = "The total amount of time taken to complete all shard fetch operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fetch_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time taken to complete all shard fetch operations, in milliseconds."]
    pub fetch_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The total number of shard fetch operations."]
    pub fetch_total: i64,
    #[doc = "The search statistics grouped by search groups."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<std::collections::BTreeMap<String, crate::types::common::SearchStats>>,
    #[doc = "The number of open search contexts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_contexts: Option<i64>,
    #[doc = "The number of currently open shard PIT contexts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub point_in_time_current: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub point_in_time_time: Option<crate::types::common::Duration>,
    #[doc = "The amount of time that shard PIT contexts have been held open since the node last restarted, in milliseconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub point_in_time_time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "The total number of shard Point in Time (PIT) contexts created (completed and active) since the node last restarted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub point_in_time_total: Option<i64>,
    #[doc = "The number of currently running shard query operations."]
    pub query_current: i64,
    #[doc = "The total amount of time taken to complete all shard query operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time taken to complete all shard query operations, in milliseconds."]
    pub query_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The total number of shard query operations."]
    pub query_total: i64,
    #[doc = "Statistics related to coordinator search operations for the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<std::collections::BTreeMap<String, crate::types::common::RequestStats>>,
    #[doc = "The number of shard scroll operations that are currently running."]
    pub scroll_current: i64,
    #[doc = "The total amount of time taken to complete all shard scroll operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scroll_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time taken to complete all shard scroll operations, in milliseconds."]
    pub scroll_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The total number of shard scroll operations."]
    pub scroll_total: i64,
    #[doc = "The total number of idle search reactivations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_idle_reactivate_count_total: Option<i64>,
    #[doc = "The number of currently running shard suggest operations."]
    pub suggest_current: i64,
    #[doc = "The total amount of time take to complete all shard suggest operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggest_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time taken to complete all shard suggest operations, in milliseconds."]
    pub suggest_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The total number of shard suggest operations."]
    pub suggest_total: i64,
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SegmentReplicationStats {
    raw: Box<serde_json::value::RawValue>,
}
impl SegmentReplicationStats {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `object` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_object(
        &self,
    ) -> std::result::Result<
        crate::types::common::SegmentReplicationStatsObject,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "object")
    }
    #[doc = "Parses the value as the `object_object` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_object_object(
        &self,
    ) -> std::result::Result<
        crate::types::common::SegmentReplicationStatsObjectObject,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "object_object")
    }
}

#[doc = "The segment replication statistics for versions 2.10.0 to 2.12.0."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SegmentReplicationStatsObject {
    #[doc = "The maximum number of bytes the replica is behind the primary."]
    pub max_bytes_behind: crate::types::common::HumanReadableByteCount,
    #[doc = "The maximum replication lag between primary and replica shards."]
    pub max_replication_lag: crate::types::common::Duration,
    #[doc = "The total number of bytes all replicas are behind their primaries."]
    pub total_bytes_behind: crate::types::common::HumanReadableByteCount,
}

#[doc = "The segment replication statistics for version 2.12.0 and later."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SegmentReplicationStatsObjectObject {
    #[doc = "The maximum number of bytes the replica is behind the primary."]
    pub max_bytes_behind: crate::types::common::ByteCount,
    #[doc = "The maximum replication lag between primary and replica shards in milliseconds."]
    pub max_replication_lag: crate::types::common::DurationValueUnitMillis,
    #[doc = "The total number of bytes all replicas are behind their primaries."]
    pub total_bytes_behind: crate::types::common::ByteCount,
}

#[doc = "The statistics about segments."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SegmentsStats {
    #[doc = "The total number of segments across all shards assigned to the selected nodes."]
    pub count: i32,
    #[doc = "The human-readable amount of memory used for doc values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values_memory: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount, in bytes, of memory used for document values across all shards assigned to the selected nodes."]
    pub doc_values_memory_in_bytes: crate::types::common::ByteCount,
    #[doc = "This object is not populated by the cluster stats API. To get information on segment files, use the node stats API."]
    pub file_sizes:
        std::collections::BTreeMap<String, crate::types::indices_stats::ShardFileSizeInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_bit_set: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount of memory, in bytes, used by fixed bit sets across all shards assigned to the selected nodes."]
    pub fixed_bit_set_memory_in_bytes: crate::types::common::ByteCount,
    #[doc = "The maximum amount of memory, in bytes, used by index writers."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_writer_max_memory_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "The human-readable amount of memory used by index writers."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_writer_memory: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount, in bytes, of memory used by all index writers across all shards assigned to the selected nodes."]
    pub index_writer_memory_in_bytes: crate::types::common::ByteCount,
    #[doc = "The Unix timestamp, in milliseconds, of the most recently retried indexing request."]
    pub max_unsafe_auto_id_timestamp: crate::types::common::EpochTimeUnitMillis,
    #[doc = "The total amount of memory used for segments across all shards assigned to the selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount, in bytes, of memory used for segments across all shards assigned to the selected nodes."]
    pub memory_in_bytes: crate::types::common::ByteCount,
    #[doc = "The total amount of memory used for normalization factors across all shards assigned to the selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub norms_memory: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount, in bytes, of memory used for normalization factors across all shards assigned to the selected nodes."]
    pub norms_memory_in_bytes: crate::types::common::ByteCount,
    #[doc = "The total amount of memory used for points across all shards assigned to the selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub points_memory: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount, in bytes, of memory used for points across all shards assigned to the selected nodes."]
    pub points_memory_in_bytes: crate::types::common::ByteCount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_store: Option<crate::types::common::RemoteStoreStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_replication: Option<crate::types::common::SegmentReplicationStats>,
    #[doc = "The total amount of memory used for stored fields across all shards assigned to the selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stored_fields_memory: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount, in bytes, of memory used for stored fields across all shards assigned to the selected nodes."]
    pub stored_fields_memory_in_bytes: crate::types::common::ByteCount,
    #[doc = "The total amount of memory used for term vectors across all shards assigned to the selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term_vectors_memory: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount, in bytes, of memory used for term vectors across all shards assigned to the selected nodes."]
    pub term_vectors_memory_in_bytes: crate::types::common::ByteCount,
    #[doc = "The total amount of memory used for terms across all shards assigned to the selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terms_memory: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount, in bytes, of memory used for terms across all shards assigned to the selected nodes."]
    pub terms_memory_in_bytes: crate::types::common::ByteCount,
    #[doc = "The total amount of memory used by all version maps across all shards assigned to the selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_map_memory: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount, in bytes, of memory used by all version maps across all shards assigned to the selected nodes."]
    pub version_map_memory_in_bytes: crate::types::common::ByteCount,
}

#[doc = "The sequence number of the document."]
pub type SequenceNumber = i64;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardFailure {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<crate::types::common::IndexName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
    pub primary: bool,
    pub reason: crate::types::common::ErrorCause,
    pub shard: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardInfo {
    pub failed: crate::types::common::Uint,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<crate::types::common::ShardFailure>>,
    pub successful: crate::types::common::Uint,
    pub total: crate::types::common::Uint,
}

#[doc = "Represents a failure to search on a specific shard. Used in search responses."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardSearchFailure {
    #[doc = "The index name where the failure occurred."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    #[doc = "The node id where the failure occurred."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
    pub reason: crate::types::common::ErrorCause,
    #[doc = "The shard id where the failure occurred."]
    pub shard: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardStatistics {
    pub failed: crate::types::common::Uint,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<crate::types::common::ShardSearchFailure>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skipped: Option<crate::types::common::Uint>,
    pub successful: crate::types::common::Uint,
    pub total: crate::types::common::Uint,
}

#[doc = "The base response for operations involving shards."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardsOperationResponseBase {
    #[serde(rename = "_shards")]
    pub shards: crate::types::common::ShardStatistics,
}

#[doc = "The short integer value."]
pub type Short = f64;

#[doc = "The configuration for a sliced scroll request."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SlicedScroll {
    #[doc = "The field to slice on."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<crate::types::common::Field>,
    #[doc = "The ID of the slice."]
    pub id: i32,
    #[doc = "The maximum number of slices."]
    pub max: i32,
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Sort {
    raw: Box<serde_json::value::RawValue>,
}
impl Sort {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `sort_combinations` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_sort_combinations(
        &self,
    ) -> std::result::Result<crate::types::common::SortCombinations, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "sort_combinations")
    }
    #[doc = "Parses the value as the `array` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_array(
        &self,
    ) -> std::result::Result<
        Vec<crate::types::common::SortCombinations>,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "array")
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SortCombinations {
    raw: Box<serde_json::value::RawValue>,
}
impl SortCombinations {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `field` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_field(
        &self,
    ) -> std::result::Result<crate::types::common::Field, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "field")
    }
    #[doc = "Parses the value as the `field_with_direction` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_field_with_direction(
        &self,
    ) -> std::result::Result<crate::types::common::FieldWithDirection, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "field_with_direction")
    }
    #[doc = "Parses the value as the `field_with_order` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_field_with_order(
        &self,
    ) -> std::result::Result<crate::types::common::FieldWithOrder, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "field_with_order")
    }
    #[doc = "Parses the value as the `options` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_options(
        &self,
    ) -> std::result::Result<crate::types::common::SortOptions, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "options")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SortMode {
    #[serde(rename = "avg")]
    Avg,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "median")]
    Median,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "sum")]
    Sum,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SortOptions {
    #[serde(rename = "_geo_distance")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geo_distance: Option<crate::types::common::GeoDistanceSort>,
    #[serde(rename = "_score")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<crate::types::common::ScoreSort>,
    #[serde(rename = "_script")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<crate::types::common::ScriptSort>,
}

#[doc = "The direction of the sort order."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

pub type SortResults = Vec<crate::types::common::FieldValue>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StoreStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserved: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "A prediction, in bytes, of how much larger the shard stores will eventually grow due to ongoing peer recoveries, restoring snapshots, and similar activities."]
    pub reserved_in_bytes: crate::types::common::ByteCount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total size, in bytes, of all shards assigned to the selected nodes."]
    pub size_in_bytes: crate::types::common::ByteCount,
}

#[doc = "The definition of a stored script."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StoredScript {
    pub lang: crate::types::common::ScriptLanguage,
    #[doc = "The options for the script."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::BTreeMap<String, String>>,
    #[doc = "The location of the source code for the script."]
    pub source: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StoredScriptId {
    #[doc = "Specifies any named parameters that are passed into the script as variables. Use parameters instead of hard-coded values to decrease compilation time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub id: crate::types::common::Id,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum StringOrStringArray {
    String(String),
    Array(Vec<String>),
}
impl<'de> serde::Deserialize<'de> for StringOrStringArray {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = StringOrStringArray;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StringOrStringArray value")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(StringOrStringArray::String)
            }
            fn visit_seq<A>(self, seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
                    .map(StringOrStringArray::Array)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[doc = "Certain APIs may return values, including numbers such as epoch timestamps, as strings. This setting captures this behavior while keeping the semantics of the field type.  Depending on the target language, code generators can keep the union or remove it and leniently parse strings to the target type."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum StringifiedBoolean {
    Boolean(bool),
    String(String),
}
impl<'de> serde::Deserialize<'de> for StringifiedBoolean {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = StringifiedBoolean;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StringifiedBoolean value")
            }
            fn visit_bool<E>(self, v: bool) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::BoolDeserializer::new(v))
                    .map(StringifiedBoolean::Boolean)
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(StringifiedBoolean::String)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum StringifiedDouble {
    Number(f64),
    String(String),
}
impl<'de> serde::Deserialize<'de> for StringifiedDouble {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = StringifiedDouble;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StringifiedDouble value")
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::I64Deserializer::new(v))
                    .map(StringifiedDouble::Number)
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::U64Deserializer::new(v))
                    .map(StringifiedDouble::Number)
            }
            fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::F64Deserializer::new(v))
                    .map(StringifiedDouble::Number)
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(StringifiedDouble::String)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[doc = "Certain APIs may return values, including numbers such as epoch timestamps, as strings. This setting captures this behavior while keeping the semantics of the field type.  Depending on the target language, code generators can keep the union or remove it and leniently parse strings to the target type."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum StringifiedEpochTimeUnitMillis {
    EpochTimeUnitMillis(crate::types::common::EpochTimeUnitMillis),
    String(String),
}
impl<'de> serde::Deserialize<'de> for StringifiedEpochTimeUnitMillis {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = StringifiedEpochTimeUnitMillis;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StringifiedEpochTimeUnitMillis value")
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::I64Deserializer::new(v))
                    .map(StringifiedEpochTimeUnitMillis::EpochTimeUnitMillis)
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::U64Deserializer::new(v))
                    .map(StringifiedEpochTimeUnitMillis::EpochTimeUnitMillis)
            }
            fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::F64Deserializer::new(v))
                    .map(StringifiedEpochTimeUnitMillis::EpochTimeUnitMillis)
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(StringifiedEpochTimeUnitMillis::String)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[doc = "Certain APIs may return values, including numbers such as epoch timestamps, as strings. This setting captures this behavior while keeping the semantics of the field type.  Depending on the target language, code generators can keep the union or remove it and leniently parse strings to the target type."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum StringifiedEpochTimeUnitSeconds {
    EpochTimeUnitSeconds(crate::types::common::EpochTimeUnitSeconds),
    String(String),
}
impl<'de> serde::Deserialize<'de> for StringifiedEpochTimeUnitSeconds {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = StringifiedEpochTimeUnitSeconds;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StringifiedEpochTimeUnitSeconds value")
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::I64Deserializer::new(v))
                    .map(StringifiedEpochTimeUnitSeconds::EpochTimeUnitSeconds)
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::U64Deserializer::new(v))
                    .map(StringifiedEpochTimeUnitSeconds::EpochTimeUnitSeconds)
            }
            fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::F64Deserializer::new(v))
                    .map(StringifiedEpochTimeUnitSeconds::EpochTimeUnitSeconds)
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(StringifiedEpochTimeUnitSeconds::String)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[doc = "Certain APIs may return values, including numbers such as epoch timestamps, as strings. This setting captures this behavior while keeping the semantics of the field type.  Depending on the target language, code generators can keep the union or remove it and leniently parse strings to the target type."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum StringifiedInteger {
    Integer(i32),
    String(String),
}
impl<'de> serde::Deserialize<'de> for StringifiedInteger {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = StringifiedInteger;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StringifiedInteger value")
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::I64Deserializer::new(v))
                    .map(StringifiedInteger::Integer)
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::U64Deserializer::new(v))
                    .map(StringifiedInteger::Integer)
            }
            fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::F64Deserializer::new(v))
                    .map(StringifiedInteger::Integer)
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(StringifiedInteger::String)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum StringifiedLong {
    Integer(i64),
    String(String),
}
impl<'de> serde::Deserialize<'de> for StringifiedLong {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = StringifiedLong;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StringifiedLong value")
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::I64Deserializer::new(v))
                    .map(StringifiedLong::Integer)
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::U64Deserializer::new(v))
                    .map(StringifiedLong::Integer)
            }
            fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::F64Deserializer::new(v))
                    .map(StringifiedLong::Integer)
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(StringifiedLong::String)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[doc = "Certain APIs may return values, including numbers such as epoch timestamps, as strings. This setting captures this behavior while keeping the semantics of the field type.  Depending on the target language, code generators can keep the union or remove it and leniently parse strings to the target type."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum StringifiedVersionNumber {
    VersionNumber(crate::types::common::VersionNumber),
    String(String),
}
impl<'de> serde::Deserialize<'de> for StringifiedVersionNumber {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = StringifiedVersionNumber;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StringifiedVersionNumber value")
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::I64Deserializer::new(v))
                    .map(StringifiedVersionNumber::VersionNumber)
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::U64Deserializer::new(v))
                    .map(StringifiedVersionNumber::VersionNumber)
            }
            fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::F64Deserializer::new(v))
                    .map(StringifiedVersionNumber::VersionNumber)
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(StringifiedVersionNumber::String)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SuggestMode {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "popular")]
    Popular,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

pub type TDocument = Box<serde_json::value::RawValue>;

#[doc = "The details of a task failure."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TaskFailure {
    pub node_id: crate::types::common::NodeId,
    pub reason: crate::types::common::ErrorCause,
    #[doc = "The status of the failed task."]
    pub status: String,
    #[doc = "The ID of the failed task."]
    pub task_id: i32,
}

#[doc = "The unique identifier of a task."]
pub type TaskId = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TermFrequencyNormalization {
    #[serde(rename = "h1")]
    H1,
    #[serde(rename = "h2")]
    H2,
    #[serde(rename = "h3")]
    H3,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "z")]
    Z,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[doc = "The information about thread usage."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ThreadInfo {
    #[doc = "The number of currently active threads."]
    pub active_threads: f64,
    #[doc = "The number of thread executions."]
    pub thread_executions: f64,
}

#[doc = "Time of day, expressed as HH:MM:SS."]
pub type TimeOfDay = String;

#[doc = "The time zone identifier."]
pub type TimeZone = String;

#[doc = "The geographical bounds specified using upper-left and lower-right points."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TopLeftBottomRightGeoBounds {
    #[doc = "The lower-right corner coordinates."]
    pub bottom_right: crate::types::common::GeoLocation,
    #[doc = "The upper-left corner coordinates."]
    pub top_left: crate::types::common::GeoLocation,
}

#[doc = "The geographical bounds specified using upper-right and lower-left points."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TopRightBottomLeftGeoBounds {
    #[doc = "The lower-left corner coordinates."]
    pub bottom_left: crate::types::common::GeoLocation,
    #[doc = "The upper-right corner coordinates."]
    pub top_right: crate::types::common::GeoLocation,
}

#[doc = "The statistics about translog operations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TranslogStats {
    #[doc = "The age of the oldest translog entry."]
    pub earliest_last_modified_age: i64,
    #[doc = "The number of operations in the translog."]
    pub operations: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_store: Option<crate::types::common::RemoteStoreTranslogStats>,
    #[doc = "The size of the translog in a human-readable format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The size of the translog in bytes."]
    pub size_in_bytes: crate::types::common::ByteCount,
    #[doc = "The number of uncommitted operations in the translog."]
    pub uncommitted_operations: i32,
    #[doc = "The size of uncommitted operations in a human-readable format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uncommitted_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The size of uncommitted operations in bytes."]
    pub uncommitted_size_in_bytes: crate::types::common::ByteCount,
}

#[doc = "The transport address of a node."]
pub type TransportAddress = String;

#[doc = "The type of document or resource."]
pub type Type = String;

pub type Uint = i32;

#[doc = "The unsigned long integer value."]
pub type Ulong = f64;

#[doc = "Time unit for microseconds."]
pub type UnitMicros = i64;

#[doc = "The time unit for milliseconds."]
pub type UnitMillis = i64;

#[doc = "Time unit for nanoseconds."]
pub type UnitNanos = i64;

#[doc = "Time unit for seconds."]
pub type UnitSeconds = i64;

#[doc = "The universally unique identifier."]
pub type Uuid = String;

pub type VersionNumber = i64;

pub type VersionString = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum VersionType {
    #[serde(rename = "external")]
    External,
    #[serde(rename = "external_gte")]
    ExternalGte,
    #[serde(rename = "internal")]
    Internal,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[doc = "The absence of any type. This is commonly used in APIs that don't return a body.  Although \"void\" is generally used for a unit type that has only one value, this is interpreted as the bottom type, which has no value. Most languages have a unit type, but few have a bottom type.  See https://en.m.wikipedia.org/wiki/Unit_type and https://en.m.wikipedia.org/wiki/Bottom_type."]
pub type Void = std::collections::BTreeMap<String, serde_json::Value>;

#[doc = "The statistics about index warmer operations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WarmerStats {
    #[doc = "The number of current warmer operations."]
    pub current: i64,
    #[doc = "The total number of warmer operations."]
    pub total: i64,
    #[doc = "The total time spent on warmer operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_time: Option<crate::types::common::Duration>,
    #[doc = "The total time spent on warmer operations in milliseconds."]
    pub total_time_in_millis: crate::types::common::DurationValueUnitMillis,
}

#[doc = "The geographical bounds specified using Well-Known Text (WKT) format."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WktGeoBounds {
    #[doc = "The WKT string representation of the geographical bounds."]
    pub wkt: String,
}

#[doc = "The base response for write operations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WriteResponseBase {
    #[serde(rename = "_id")]
    pub id: crate::types::common::Id,
    #[serde(rename = "_index")]
    pub index: crate::types::common::IndexName,
    #[doc = "The primary term of the document."]
    #[serde(rename = "_primary_term")]
    pub primary_term: i64,
    #[serde(rename = "_seq_no")]
    pub seq_no: crate::types::common::SequenceNumber,
    #[serde(rename = "_shards")]
    pub shards: crate::types::common::ShardStatistics,
    #[serde(rename = "_type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::types::common::Type>,
    #[serde(rename = "_version")]
    pub version: crate::types::common::VersionNumber,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forced_refresh: Option<bool>,
    pub result: crate::types::common::Result,
}

#[doc = "The Cartesian coordinates specified using x and y values."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct XyCartesianCoordinates {
    #[doc = "The x coordinate."]
    pub x: f64,
    #[doc = "The y coordinate."]
    pub y: f64,
}

#[doc = "A two-dimensional Cartesian point specified by x and y coordinates. It can be represented in the following ways:  - As an `{x, y}` object.  - As an `[x, y]` array.  - As a string in `\"x, y\"` or WKT point format."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum XyLocation {
    Cartesian(crate::types::common::XyCartesianCoordinates),
    Coords(Vec<f64>),
    Text(String),
}
impl<'de> serde::Deserialize<'de> for XyLocation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = XyLocation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a XyLocation value")
            }
            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(XyLocation::Cartesian)
            }
            fn visit_seq<A>(self, seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
                    .map(XyLocation::Coords)
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(XyLocation::Text)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}
