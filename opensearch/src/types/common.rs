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

#[derive(Debug, Clone)]
pub struct AcknowledgedResponseBase {
    #[doc = "For a successful response, this value is always true. On failure, an exception is returned instead."]
    pub acknowledged: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AcknowledgedResponseBase {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AcknowledgedResponseBase;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AcknowledgedResponseBase object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_acknowledged: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "acknowledged" => {
                            if f_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field("acknowledged"));
                            }
                            f_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AcknowledgedResponseBase {
                    acknowledged: f_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("acknowledged"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AcknowledgedResponseBase {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("acknowledged", &self.acknowledged)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActionStatusOptions {
    #[doc = "`failure`"]
    Failure,
    #[doc = "`simulated`"]
    Simulated,
    #[doc = "`success`"]
    Success,
    #[doc = "`throttled`"]
    Throttled,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ActionStatusOptions {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ActionStatusOptions::Failure => "failure",
            ActionStatusOptions::Simulated => "simulated",
            ActionStatusOptions::Success => "success",
            ActionStatusOptions::Throttled => "throttled",
            ActionStatusOptions::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ActionStatusOptions {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ActionStatusOptions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "failure" => ActionStatusOptions::Failure,
            "simulated" => ActionStatusOptions::Simulated,
            "success" => ActionStatusOptions::Success,
            "throttled" => ActionStatusOptions::Throttled,
            _ => ActionStatusOptions::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuiltinScriptLanguage {
    #[doc = "`expression`"]
    Expression,
    #[doc = "`java`"]
    Java,
    #[doc = "`mustache`"]
    Mustache,
    #[doc = "`painless`"]
    Painless,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl BuiltinScriptLanguage {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            BuiltinScriptLanguage::Expression => "expression",
            BuiltinScriptLanguage::Java => "java",
            BuiltinScriptLanguage::Mustache => "mustache",
            BuiltinScriptLanguage::Painless => "painless",
            BuiltinScriptLanguage::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for BuiltinScriptLanguage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BuiltinScriptLanguage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "expression" => BuiltinScriptLanguage::Expression,
            "java" => BuiltinScriptLanguage::Java,
            "mustache" => BuiltinScriptLanguage::Mustache,
            "painless" => BuiltinScriptLanguage::Painless,
            _ => BuiltinScriptLanguage::Unknown(s),
        })
    }
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

#[derive(Debug, Clone)]
pub struct BulkByScrollResponseBase {
    #[doc = "The number of scroll responses pulled back by the reindex operation."]
    pub batches: i32,
    pub canceled: Option<String>,
    #[doc = "The number of documents that were successfully created."]
    pub created: Option<i64>,
    #[doc = "The number of documents that were successfully deleted."]
    pub deleted: i64,
    #[doc = "The number of documents that were ignored."]
    pub noops: i64,
    #[doc = "The number of requests per second effectively executed during the reindex operation."]
    pub requests_per_second: f64,
    pub retries: crate::types::common::Retries,
    pub slice_id: Option<i32>,
    pub slices: Option<Vec<crate::types::common::BulkByScrollTaskStatusOrException>>,
    pub throttled: Option<crate::types::common::Duration>,
    pub throttled_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The amount time before the throttling will end."]
    pub throttled_until: Option<crate::types::common::Duration>,
    #[doc = "When the throttling will end in milliseconds."]
    pub throttled_until_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The number of documents that were successfully processed."]
    pub total: i64,
    #[doc = "The number of documents that were successfully updated after the reindex operation."]
    pub updated: Option<i64>,
    #[doc = "The number of version conflicts encountered by the reindex operation."]
    pub version_conflicts: i64,
    #[doc = "The list of failures that occurred during the operation."]
    pub failures: Vec<crate::types::common::BulkByScrollFailure>,
    #[doc = "Whether the operation timed out."]
    pub timed_out: bool,
    #[doc = "The time taken by the operation in milliseconds."]
    pub took: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for BulkByScrollResponseBase {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = BulkByScrollResponseBase;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a BulkByScrollResponseBase object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_batches: Option<i32> = None;
                let mut f_canceled: Option<String> = None;
                let mut seen_canceled = false;
                let mut f_created: Option<i64> = None;
                let mut seen_created = false;
                let mut f_deleted: Option<i64> = None;
                let mut f_noops: Option<i64> = None;
                let mut f_requests_per_second: Option<f64> = None;
                let mut f_retries: Option<crate::types::common::Retries> = None;
                let mut f_slice_id: Option<i32> = None;
                let mut seen_slice_id = false;
                let mut f_slices: Option<
                    Vec<crate::types::common::BulkByScrollTaskStatusOrException>,
                > = None;
                let mut seen_slices = false;
                let mut f_throttled: Option<crate::types::common::Duration> = None;
                let mut seen_throttled = false;
                let mut f_throttled_millis: Option<crate::types::common::DurationValueUnitMillis> =
                    None;
                let mut f_throttled_until: Option<crate::types::common::Duration> = None;
                let mut seen_throttled_until = false;
                let mut f_throttled_until_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_total: Option<i64> = None;
                let mut f_updated: Option<i64> = None;
                let mut seen_updated = false;
                let mut f_version_conflicts: Option<i64> = None;
                let mut f_failures: Option<Vec<crate::types::common::BulkByScrollFailure>> = None;
                let mut f_timed_out: Option<bool> = None;
                let mut f_took: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "batches" => {
                            if f_batches.is_some() {
                                return Err(serde::de::Error::duplicate_field("batches"));
                            }
                            f_batches = Some(map.next_value::<i32>()?);
                        }
                        "canceled" => {
                            if seen_canceled {
                                return Err(serde::de::Error::duplicate_field("canceled"));
                            }
                            seen_canceled = true;
                            f_canceled = map.next_value::<Option<String>>()?;
                        }
                        "created" => {
                            if seen_created {
                                return Err(serde::de::Error::duplicate_field("created"));
                            }
                            seen_created = true;
                            f_created = map.next_value::<Option<i64>>()?;
                        }
                        "deleted" => {
                            if f_deleted.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleted"));
                            }
                            f_deleted = Some(map.next_value::<i64>()?);
                        }
                        "noops" => {
                            if f_noops.is_some() {
                                return Err(serde::de::Error::duplicate_field("noops"));
                            }
                            f_noops = Some(map.next_value::<i64>()?);
                        }
                        "requests_per_second" => {
                            if f_requests_per_second.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "requests_per_second",
                                ));
                            }
                            f_requests_per_second = Some(map.next_value::<f64>()?);
                        }
                        "retries" => {
                            if f_retries.is_some() {
                                return Err(serde::de::Error::duplicate_field("retries"));
                            }
                            f_retries = Some(map.next_value::<crate::types::common::Retries>()?);
                        }
                        "slice_id" => {
                            if seen_slice_id {
                                return Err(serde::de::Error::duplicate_field("slice_id"));
                            }
                            seen_slice_id = true;
                            f_slice_id = map.next_value::<Option<i32>>()?;
                        }
                        "slices" => {
                            if seen_slices {
                                return Err(serde::de::Error::duplicate_field("slices"));
                            }
                            seen_slices = true;
                            f_slices = map.next_value::<Option<
                                Vec<crate::types::common::BulkByScrollTaskStatusOrException>,
                            >>()?;
                        }
                        "throttled" => {
                            if seen_throttled {
                                return Err(serde::de::Error::duplicate_field("throttled"));
                            }
                            seen_throttled = true;
                            f_throttled =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "throttled_millis" => {
                            if f_throttled_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field("throttled_millis"));
                            }
                            f_throttled_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "throttled_until" => {
                            if seen_throttled_until {
                                return Err(serde::de::Error::duplicate_field("throttled_until"));
                            }
                            seen_throttled_until = true;
                            f_throttled_until =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "throttled_until_millis" => {
                            if f_throttled_until_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "throttled_until_millis",
                                ));
                            }
                            f_throttled_until_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(map.next_value::<i64>()?);
                        }
                        "updated" => {
                            if seen_updated {
                                return Err(serde::de::Error::duplicate_field("updated"));
                            }
                            seen_updated = true;
                            f_updated = map.next_value::<Option<i64>>()?;
                        }
                        "version_conflicts" => {
                            if f_version_conflicts.is_some() {
                                return Err(serde::de::Error::duplicate_field("version_conflicts"));
                            }
                            f_version_conflicts = Some(map.next_value::<i64>()?);
                        }
                        "failures" => {
                            if f_failures.is_some() {
                                return Err(serde::de::Error::duplicate_field("failures"));
                            }
                            f_failures = Some(
                                map.next_value::<Vec<crate::types::common::BulkByScrollFailure>>()?,
                            );
                        }
                        "timed_out" => {
                            if f_timed_out.is_some() {
                                return Err(serde::de::Error::duplicate_field("timed_out"));
                            }
                            f_timed_out = Some(map.next_value::<bool>()?);
                        }
                        "took" => {
                            if f_took.is_some() {
                                return Err(serde::de::Error::duplicate_field("took"));
                            }
                            f_took = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(BulkByScrollResponseBase {
                    batches: f_batches.ok_or_else(|| serde::de::Error::missing_field("batches"))?,
                    canceled: f_canceled,
                    created: f_created,
                    deleted: f_deleted.ok_or_else(|| serde::de::Error::missing_field("deleted"))?,
                    noops: f_noops.ok_or_else(|| serde::de::Error::missing_field("noops"))?,
                    requests_per_second: f_requests_per_second
                        .ok_or_else(|| serde::de::Error::missing_field("requests_per_second"))?,
                    retries: f_retries.ok_or_else(|| serde::de::Error::missing_field("retries"))?,
                    slice_id: f_slice_id,
                    slices: f_slices,
                    throttled: f_throttled,
                    throttled_millis: f_throttled_millis
                        .ok_or_else(|| serde::de::Error::missing_field("throttled_millis"))?,
                    throttled_until: f_throttled_until,
                    throttled_until_millis: f_throttled_until_millis
                        .ok_or_else(|| serde::de::Error::missing_field("throttled_until_millis"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    updated: f_updated,
                    version_conflicts: f_version_conflicts
                        .ok_or_else(|| serde::de::Error::missing_field("version_conflicts"))?,
                    failures: f_failures
                        .ok_or_else(|| serde::de::Error::missing_field("failures"))?,
                    timed_out: f_timed_out
                        .ok_or_else(|| serde::de::Error::missing_field("timed_out"))?,
                    took: f_took.ok_or_else(|| serde::de::Error::missing_field("took"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for BulkByScrollResponseBase {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("batches", &self.batches)?;
        if let Some(v) = &self.canceled {
            map.serialize_entry("canceled", v)?;
        }
        if let Some(v) = &self.created {
            map.serialize_entry("created", v)?;
        }
        map.serialize_entry("deleted", &self.deleted)?;
        map.serialize_entry("noops", &self.noops)?;
        map.serialize_entry("requests_per_second", &self.requests_per_second)?;
        map.serialize_entry("retries", &self.retries)?;
        if let Some(v) = &self.slice_id {
            map.serialize_entry("slice_id", v)?;
        }
        if let Some(v) = &self.slices {
            map.serialize_entry("slices", v)?;
        }
        if let Some(v) = &self.throttled {
            map.serialize_entry("throttled", v)?;
        }
        map.serialize_entry("throttled_millis", &self.throttled_millis)?;
        if let Some(v) = &self.throttled_until {
            map.serialize_entry("throttled_until", v)?;
        }
        map.serialize_entry("throttled_until_millis", &self.throttled_until_millis)?;
        map.serialize_entry("total", &self.total)?;
        if let Some(v) = &self.updated {
            map.serialize_entry("updated", v)?;
        }
        map.serialize_entry("version_conflicts", &self.version_conflicts)?;
        map.serialize_entry("failures", &self.failures)?;
        map.serialize_entry("timed_out", &self.timed_out)?;
        map.serialize_entry("took", &self.took)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The status of a bulk by scroll task."]
#[derive(Debug, Clone)]
pub struct BulkByScrollTaskStatus {
    #[doc = "The number of scroll responses pulled back by the reindex operation."]
    pub batches: i32,
    pub canceled: Option<String>,
    #[doc = "The number of documents that were successfully created."]
    pub created: Option<i64>,
    #[doc = "The number of documents that were successfully deleted."]
    pub deleted: i64,
    #[doc = "The number of documents that were ignored."]
    pub noops: i64,
    #[doc = "The number of requests per second effectively executed during the reindex operation."]
    pub requests_per_second: f64,
    pub retries: crate::types::common::Retries,
    pub slice_id: Option<i32>,
    pub slices: Option<Vec<crate::types::common::BulkByScrollTaskStatusOrException>>,
    pub throttled: Option<crate::types::common::Duration>,
    pub throttled_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The amount time before the throttling will end."]
    pub throttled_until: Option<crate::types::common::Duration>,
    #[doc = "When the throttling will end in milliseconds."]
    pub throttled_until_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The number of documents that were successfully processed."]
    pub total: i64,
    #[doc = "The number of documents that were successfully updated after the reindex operation."]
    pub updated: Option<i64>,
    #[doc = "The number of version conflicts encountered by the reindex operation."]
    pub version_conflicts: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for BulkByScrollTaskStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = BulkByScrollTaskStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a BulkByScrollTaskStatus object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_batches: Option<i32> = None;
                let mut f_canceled: Option<String> = None;
                let mut seen_canceled = false;
                let mut f_created: Option<i64> = None;
                let mut seen_created = false;
                let mut f_deleted: Option<i64> = None;
                let mut f_noops: Option<i64> = None;
                let mut f_requests_per_second: Option<f64> = None;
                let mut f_retries: Option<crate::types::common::Retries> = None;
                let mut f_slice_id: Option<i32> = None;
                let mut seen_slice_id = false;
                let mut f_slices: Option<
                    Vec<crate::types::common::BulkByScrollTaskStatusOrException>,
                > = None;
                let mut seen_slices = false;
                let mut f_throttled: Option<crate::types::common::Duration> = None;
                let mut seen_throttled = false;
                let mut f_throttled_millis: Option<crate::types::common::DurationValueUnitMillis> =
                    None;
                let mut f_throttled_until: Option<crate::types::common::Duration> = None;
                let mut seen_throttled_until = false;
                let mut f_throttled_until_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_total: Option<i64> = None;
                let mut f_updated: Option<i64> = None;
                let mut seen_updated = false;
                let mut f_version_conflicts: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "batches" => {
                            if f_batches.is_some() {
                                return Err(serde::de::Error::duplicate_field("batches"));
                            }
                            f_batches = Some(map.next_value::<i32>()?);
                        }
                        "canceled" => {
                            if seen_canceled {
                                return Err(serde::de::Error::duplicate_field("canceled"));
                            }
                            seen_canceled = true;
                            f_canceled = map.next_value::<Option<String>>()?;
                        }
                        "created" => {
                            if seen_created {
                                return Err(serde::de::Error::duplicate_field("created"));
                            }
                            seen_created = true;
                            f_created = map.next_value::<Option<i64>>()?;
                        }
                        "deleted" => {
                            if f_deleted.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleted"));
                            }
                            f_deleted = Some(map.next_value::<i64>()?);
                        }
                        "noops" => {
                            if f_noops.is_some() {
                                return Err(serde::de::Error::duplicate_field("noops"));
                            }
                            f_noops = Some(map.next_value::<i64>()?);
                        }
                        "requests_per_second" => {
                            if f_requests_per_second.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "requests_per_second",
                                ));
                            }
                            f_requests_per_second = Some(map.next_value::<f64>()?);
                        }
                        "retries" => {
                            if f_retries.is_some() {
                                return Err(serde::de::Error::duplicate_field("retries"));
                            }
                            f_retries = Some(map.next_value::<crate::types::common::Retries>()?);
                        }
                        "slice_id" => {
                            if seen_slice_id {
                                return Err(serde::de::Error::duplicate_field("slice_id"));
                            }
                            seen_slice_id = true;
                            f_slice_id = map.next_value::<Option<i32>>()?;
                        }
                        "slices" => {
                            if seen_slices {
                                return Err(serde::de::Error::duplicate_field("slices"));
                            }
                            seen_slices = true;
                            f_slices = map.next_value::<Option<
                                Vec<crate::types::common::BulkByScrollTaskStatusOrException>,
                            >>()?;
                        }
                        "throttled" => {
                            if seen_throttled {
                                return Err(serde::de::Error::duplicate_field("throttled"));
                            }
                            seen_throttled = true;
                            f_throttled =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "throttled_millis" => {
                            if f_throttled_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field("throttled_millis"));
                            }
                            f_throttled_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "throttled_until" => {
                            if seen_throttled_until {
                                return Err(serde::de::Error::duplicate_field("throttled_until"));
                            }
                            seen_throttled_until = true;
                            f_throttled_until =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "throttled_until_millis" => {
                            if f_throttled_until_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "throttled_until_millis",
                                ));
                            }
                            f_throttled_until_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(map.next_value::<i64>()?);
                        }
                        "updated" => {
                            if seen_updated {
                                return Err(serde::de::Error::duplicate_field("updated"));
                            }
                            seen_updated = true;
                            f_updated = map.next_value::<Option<i64>>()?;
                        }
                        "version_conflicts" => {
                            if f_version_conflicts.is_some() {
                                return Err(serde::de::Error::duplicate_field("version_conflicts"));
                            }
                            f_version_conflicts = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(BulkByScrollTaskStatus {
                    batches: f_batches.ok_or_else(|| serde::de::Error::missing_field("batches"))?,
                    canceled: f_canceled,
                    created: f_created,
                    deleted: f_deleted.ok_or_else(|| serde::de::Error::missing_field("deleted"))?,
                    noops: f_noops.ok_or_else(|| serde::de::Error::missing_field("noops"))?,
                    requests_per_second: f_requests_per_second
                        .ok_or_else(|| serde::de::Error::missing_field("requests_per_second"))?,
                    retries: f_retries.ok_or_else(|| serde::de::Error::missing_field("retries"))?,
                    slice_id: f_slice_id,
                    slices: f_slices,
                    throttled: f_throttled,
                    throttled_millis: f_throttled_millis
                        .ok_or_else(|| serde::de::Error::missing_field("throttled_millis"))?,
                    throttled_until: f_throttled_until,
                    throttled_until_millis: f_throttled_until_millis
                        .ok_or_else(|| serde::de::Error::missing_field("throttled_until_millis"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    updated: f_updated,
                    version_conflicts: f_version_conflicts
                        .ok_or_else(|| serde::de::Error::missing_field("version_conflicts"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for BulkByScrollTaskStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("batches", &self.batches)?;
        if let Some(v) = &self.canceled {
            map.serialize_entry("canceled", v)?;
        }
        if let Some(v) = &self.created {
            map.serialize_entry("created", v)?;
        }
        map.serialize_entry("deleted", &self.deleted)?;
        map.serialize_entry("noops", &self.noops)?;
        map.serialize_entry("requests_per_second", &self.requests_per_second)?;
        map.serialize_entry("retries", &self.retries)?;
        if let Some(v) = &self.slice_id {
            map.serialize_entry("slice_id", v)?;
        }
        if let Some(v) = &self.slices {
            map.serialize_entry("slices", v)?;
        }
        if let Some(v) = &self.throttled {
            map.serialize_entry("throttled", v)?;
        }
        map.serialize_entry("throttled_millis", &self.throttled_millis)?;
        if let Some(v) = &self.throttled_until {
            map.serialize_entry("throttled_until", v)?;
        }
        map.serialize_entry("throttled_until_millis", &self.throttled_until_millis)?;
        map.serialize_entry("total", &self.total)?;
        if let Some(v) = &self.updated {
            map.serialize_entry("updated", v)?;
        }
        map.serialize_entry("version_conflicts", &self.version_conflicts)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
#[derive(Debug, Clone)]
pub struct BulkItemResponseFailure {
    pub cause: crate::types::common::ErrorCause,
    pub id: Option<crate::types::common::Id>,
    pub index: crate::types::common::IndexName,
    pub status: i32,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for BulkItemResponseFailure {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = BulkItemResponseFailure;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a BulkItemResponseFailure object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cause: Option<crate::types::common::ErrorCause> = None;
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut seen_id = false;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut f_status: Option<i32> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cause" => {
                            if f_cause.is_some() {
                                return Err(serde::de::Error::duplicate_field("cause"));
                            }
                            f_cause = Some(map.next_value::<crate::types::common::ErrorCause>()?);
                        }
                        "id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        "index" => {
                            if f_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            f_index = Some(map.next_value::<crate::types::common::IndexName>()?);
                        }
                        "status" => {
                            if f_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            f_status = Some(map.next_value::<i32>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(BulkItemResponseFailure {
                    cause: f_cause.ok_or_else(|| serde::de::Error::missing_field("cause"))?,
                    id: f_id,
                    index: f_index.ok_or_else(|| serde::de::Error::missing_field("index"))?,
                    status: f_status.ok_or_else(|| serde::de::Error::missing_field("status"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for BulkItemResponseFailure {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("cause", &self.cause)?;
        if let Some(v) = &self.id {
            map.serialize_entry("id", v)?;
        }
        map.serialize_entry("index", &self.index)?;
        map.serialize_entry("status", &self.status)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The byte value."]
pub type Byte = f64;

#[doc = "The size in bytes."]
pub type ByteCount = i64;

#[derive(Debug, Clone)]
pub struct ClusterStatistics {
    pub skipped: i32,
    pub successful: i32,
    pub total: i32,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterStatistics {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterStatistics;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterStatistics object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_skipped: Option<i32> = None;
                let mut f_successful: Option<i32> = None;
                let mut f_total: Option<i32> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "skipped" => {
                            if f_skipped.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipped"));
                            }
                            f_skipped = Some(map.next_value::<i32>()?);
                        }
                        "successful" => {
                            if f_successful.is_some() {
                                return Err(serde::de::Error::duplicate_field("successful"));
                            }
                            f_successful = Some(map.next_value::<i32>()?);
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(map.next_value::<i32>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterStatistics {
                    skipped: f_skipped.ok_or_else(|| serde::de::Error::missing_field("skipped"))?,
                    successful: f_successful
                        .ok_or_else(|| serde::de::Error::missing_field("successful"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterStatistics {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("skipped", &self.skipped)?;
        map.serialize_entry("successful", &self.successful)?;
        map.serialize_entry("total", &self.total)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct CompletionStats {
    #[doc = "The per-field completion statistics."]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common::FieldSizeUsage>>,
    #[doc = "The human-readable size of memory used for completion."]
    pub size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount, in bytes, of memory used for completion across all shards assigned to the selected nodes."]
    pub size_in_bytes: crate::types::common::ByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CompletionStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CompletionStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CompletionStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common::FieldSizeUsage>,
                > = None;
                let mut seen_fields = false;
                let mut f_size: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_size = false;
                let mut f_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common::FieldSizeUsage,
                                >,
                            >>()?;
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
                Ok(CompletionStats {
                    fields: f_fields,
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
impl serde::Serialize for CompletionStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
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

#[doc = "The geographical bounds specified using coordinate values."]
#[derive(Debug, Clone)]
pub struct CoordsGeoBounds {
    #[doc = "The bottom boundary latitude."]
    pub bottom: f64,
    #[doc = "The left boundary longitude."]
    pub left: f64,
    #[doc = "The right boundary longitude."]
    pub right: f64,
    #[doc = "The top boundary latitude."]
    pub top: f64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CoordsGeoBounds {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CoordsGeoBounds;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CoordsGeoBounds object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_bottom: Option<f64> = None;
                let mut f_left: Option<f64> = None;
                let mut f_right: Option<f64> = None;
                let mut f_top: Option<f64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "bottom" => {
                            if f_bottom.is_some() {
                                return Err(serde::de::Error::duplicate_field("bottom"));
                            }
                            f_bottom = Some(map.next_value::<f64>()?);
                        }
                        "left" => {
                            if f_left.is_some() {
                                return Err(serde::de::Error::duplicate_field("left"));
                            }
                            f_left = Some(map.next_value::<f64>()?);
                        }
                        "right" => {
                            if f_right.is_some() {
                                return Err(serde::de::Error::duplicate_field("right"));
                            }
                            f_right = Some(map.next_value::<f64>()?);
                        }
                        "top" => {
                            if f_top.is_some() {
                                return Err(serde::de::Error::duplicate_field("top"));
                            }
                            f_top = Some(map.next_value::<f64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CoordsGeoBounds {
                    bottom: f_bottom.ok_or_else(|| serde::de::Error::missing_field("bottom"))?,
                    left: f_left.ok_or_else(|| serde::de::Error::missing_field("left"))?,
                    right: f_right.ok_or_else(|| serde::de::Error::missing_field("right"))?,
                    top: f_top.ok_or_else(|| serde::de::Error::missing_field("top"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CoordsGeoBounds {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("bottom", &self.bottom)?;
        map.serialize_entry("left", &self.left)?;
        map.serialize_entry("right", &self.right)?;
        map.serialize_entry("top", &self.top)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DFIIndependenceMeasure {
    #[doc = "`chisquared`"]
    Chisquared,
    #[doc = "`saturated`"]
    Saturated,
    #[doc = "`standardized`"]
    Standardized,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DFIIndependenceMeasure {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DFIIndependenceMeasure::Chisquared => "chisquared",
            DFIIndependenceMeasure::Saturated => "saturated",
            DFIIndependenceMeasure::Standardized => "standardized",
            DFIIndependenceMeasure::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DFIIndependenceMeasure {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DFIIndependenceMeasure {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "chisquared" => DFIIndependenceMeasure::Chisquared,
            "saturated" => DFIIndependenceMeasure::Saturated,
            "standardized" => DFIIndependenceMeasure::Standardized,
            _ => DFIIndependenceMeasure::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DFRAfterEffect {
    #[doc = "`b`"]
    B,
    #[doc = "`l`"]
    L,
    #[doc = "`no`"]
    No,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DFRAfterEffect {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DFRAfterEffect::B => "b",
            DFRAfterEffect::L => "l",
            DFRAfterEffect::No => "no",
            DFRAfterEffect::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DFRAfterEffect {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DFRAfterEffect {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "b" => DFRAfterEffect::B,
            "l" => DFRAfterEffect::L,
            "no" => DFRAfterEffect::No,
            _ => DFRAfterEffect::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DFRBasicModel {
    #[doc = "`be`"]
    Be,
    #[doc = "`d`"]
    D,
    #[doc = "`g`"]
    G,
    #[doc = "`if`"]
    If,
    #[doc = "`in`"]
    In,
    #[doc = "`ine`"]
    Ine,
    #[doc = "`p`"]
    P,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DFRBasicModel {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DFRBasicModel::Be => "be",
            DFRBasicModel::D => "d",
            DFRBasicModel::G => "g",
            DFRBasicModel::If => "if",
            DFRBasicModel::In => "in",
            DFRBasicModel::Ine => "ine",
            DFRBasicModel::P => "p",
            DFRBasicModel::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DFRBasicModel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DFRBasicModel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "be" => DFRBasicModel::Be,
            "d" => DFRBasicModel::D,
            "g" => DFRBasicModel::G,
            "if" => DFRBasicModel::If,
            "in" => DFRBasicModel::In,
            "ine" => DFRBasicModel::Ine,
            "p" => DFRBasicModel::P,
            _ => DFRBasicModel::Unknown(s),
        })
    }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'"') => serde_json::from_str(text).map(DateTime::String),
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(DateTime::EpochTimeUnitMillis)
            }
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a DateTime value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

pub type Distance = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DistanceUnit {
    #[doc = "`cm`"]
    Cm,
    #[doc = "`ft`"]
    Ft,
    #[doc = "`in`"]
    In,
    #[doc = "`km`"]
    Km,
    #[doc = "`m`"]
    M,
    #[doc = "`mi`"]
    Mi,
    #[doc = "`mm`"]
    Mm,
    #[doc = "`nmi`"]
    Nmi,
    #[doc = "`yd`"]
    Yd,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DistanceUnit {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DistanceUnit::Cm => "cm",
            DistanceUnit::Ft => "ft",
            DistanceUnit::In => "in",
            DistanceUnit::Km => "km",
            DistanceUnit::M => "m",
            DistanceUnit::Mi => "mi",
            DistanceUnit::Mm => "mm",
            DistanceUnit::Nmi => "nmi",
            DistanceUnit::Yd => "yd",
            DistanceUnit::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DistanceUnit {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DistanceUnit {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "cm" => DistanceUnit::Cm,
            "ft" => DistanceUnit::Ft,
            "in" => DistanceUnit::In,
            "km" => DistanceUnit::Km,
            "m" => DistanceUnit::M,
            "mi" => DistanceUnit::Mi,
            "mm" => DistanceUnit::Mm,
            "nmi" => DistanceUnit::Nmi,
            "yd" => DistanceUnit::Yd,
            _ => DistanceUnit::Unknown(s),
        })
    }
}

#[doc = "The document-level statistics."]
#[derive(Debug, Clone)]
pub struct DocStats {
    #[doc = "The total number of non-deleted documents across all primary shards assigned to the selected nodes. This number is based on documents in Lucene segments and may include documents from nested fields."]
    pub count: i64,
    #[doc = "The total number of deleted documents across all primary shards assigned to the selected nodes. This number is based on the number of documents stored in Lucene segments. OpenSearch reclaims the disk space previously occupied by the deleted Lucene documents when a segment is merged."]
    pub deleted: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DocStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DocStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DocStats object")
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
                let mut f_deleted: Option<i64> = None;
                let mut seen_deleted = false;
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
                        "deleted" => {
                            if seen_deleted {
                                return Err(serde::de::Error::duplicate_field("deleted"));
                            }
                            seen_deleted = true;
                            f_deleted = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DocStats {
                    count: f_count.ok_or_else(|| serde::de::Error::missing_field("count"))?,
                    deleted: f_deleted,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DocStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("count", &self.count)?;
        if let Some(v) = &self.deleted {
            map.serialize_entry("deleted", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The item level HTTP response status code during indexing."]
#[derive(Debug, Clone)]
pub struct DocStatus {
    #[doc = "The number of informational responses."]
    pub _1xx: Option<i64>,
    #[doc = "The number of successful responses."]
    pub _2xx: Option<i64>,
    #[doc = "The number of redirection responses."]
    pub _3xx: Option<i64>,
    #[doc = "The number of client error responses."]
    pub _4xx: Option<i64>,
    #[doc = "The number of server error responses."]
    pub _5xx: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DocStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DocStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DocStatus object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f__1xx: Option<i64> = None;
                let mut seen__1xx = false;
                let mut f__2xx: Option<i64> = None;
                let mut seen__2xx = false;
                let mut f__3xx: Option<i64> = None;
                let mut seen__3xx = false;
                let mut f__4xx: Option<i64> = None;
                let mut seen__4xx = false;
                let mut f__5xx: Option<i64> = None;
                let mut seen__5xx = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "1xx" => {
                            if seen__1xx {
                                return Err(serde::de::Error::duplicate_field("1xx"));
                            }
                            seen__1xx = true;
                            f__1xx = map.next_value::<Option<i64>>()?;
                        }
                        "2xx" => {
                            if seen__2xx {
                                return Err(serde::de::Error::duplicate_field("2xx"));
                            }
                            seen__2xx = true;
                            f__2xx = map.next_value::<Option<i64>>()?;
                        }
                        "3xx" => {
                            if seen__3xx {
                                return Err(serde::de::Error::duplicate_field("3xx"));
                            }
                            seen__3xx = true;
                            f__3xx = map.next_value::<Option<i64>>()?;
                        }
                        "4xx" => {
                            if seen__4xx {
                                return Err(serde::de::Error::duplicate_field("4xx"));
                            }
                            seen__4xx = true;
                            f__4xx = map.next_value::<Option<i64>>()?;
                        }
                        "5xx" => {
                            if seen__5xx {
                                return Err(serde::de::Error::duplicate_field("5xx"));
                            }
                            seen__5xx = true;
                            f__5xx = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DocStatus {
                    _1xx: f__1xx,
                    _2xx: f__2xx,
                    _3xx: f__3xx,
                    _4xx: f__4xx,
                    _5xx: f__5xx,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DocStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self._1xx {
            map.serialize_entry("1xx", v)?;
        }
        if let Some(v) = &self._2xx {
            map.serialize_entry("2xx", v)?;
        }
        if let Some(v) = &self._3xx {
            map.serialize_entry("3xx", v)?;
        }
        if let Some(v) = &self._4xx {
            map.serialize_entry("4xx", v)?;
        }
        if let Some(v) = &self._5xx {
            map.serialize_entry("5xx", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_caused_by: Option<Box<crate::types::common::ErrorCause>> = None;
                let mut seen_caused_by = false;
                let mut f_header: Option<
                    std::collections::BTreeMap<String, crate::types::common::StringOrStringArray>,
                > = None;
                let mut seen_header = false;
                let mut f_reason: Option<String> = None;
                let mut seen_reason = false;
                let mut f_root_cause: Option<Vec<crate::types::common::ErrorCause>> = None;
                let mut seen_root_cause = false;
                let mut f_stack_trace: Option<String> = None;
                let mut seen_stack_trace = false;
                let mut f_suppressed: Option<Vec<crate::types::common::ErrorCause>> = None;
                let mut seen_suppressed = false;
                let mut f_type: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "caused_by" => {
                            if seen_caused_by {
                                return Err(serde::de::Error::duplicate_field("caused_by"));
                            }
                            seen_caused_by = true;
                            f_caused_by =
                                map.next_value::<Option<Box<crate::types::common::ErrorCause>>>()?;
                        }
                        "header" => {
                            if seen_header {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            seen_header = true;
                            f_header = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common::StringOrStringArray,
                                >,
                            >>()?;
                        }
                        "reason" => {
                            if seen_reason {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            seen_reason = true;
                            f_reason = map.next_value::<Option<String>>()?;
                        }
                        "root_cause" => {
                            if seen_root_cause {
                                return Err(serde::de::Error::duplicate_field("root_cause"));
                            }
                            seen_root_cause = true;
                            f_root_cause =
                                map.next_value::<Option<Vec<crate::types::common::ErrorCause>>>()?;
                        }
                        "stack_trace" => {
                            if seen_stack_trace {
                                return Err(serde::de::Error::duplicate_field("stack_trace"));
                            }
                            seen_stack_trace = true;
                            f_stack_trace = map.next_value::<Option<String>>()?;
                        }
                        "suppressed" => {
                            if seen_suppressed {
                                return Err(serde::de::Error::duplicate_field("suppressed"));
                            }
                            seen_suppressed = true;
                            f_suppressed =
                                map.next_value::<Option<Vec<crate::types::common::ErrorCause>>>()?;
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
#[derive(Debug, Clone)]
pub struct ErrorResponseBase {
    pub error: crate::types::common::ErrorCause,
    #[doc = "The HTTP status code."]
    pub status: i32,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ErrorResponseBase {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ErrorResponseBase;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ErrorResponseBase object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_error: Option<crate::types::common::ErrorCause> = None;
                let mut f_status: Option<i32> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "error" => {
                            if f_error.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            f_error = Some(map.next_value::<crate::types::common::ErrorCause>()?);
                        }
                        "status" => {
                            if f_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            f_status = Some(map.next_value::<i32>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ErrorResponseBase {
                    error: f_error.ok_or_else(|| serde::de::Error::missing_field("error"))?,
                    status: f_status.ok_or_else(|| serde::de::Error::missing_field("status"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ErrorResponseBase {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("error", &self.error)?;
        map.serialize_entry("status", &self.status)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The path to a field or an array of paths. Some APIs support wildcards in the path, which allows you to select multiple fields."]
pub type Field = String;

#[doc = "The memory usage statistics for a field."]
#[derive(Debug, Clone)]
pub struct FieldMemoryUsage {
    #[doc = "The human-readable amount of memory used."]
    pub memory_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The amount of memory used in bytes."]
    pub memory_size_in_bytes: crate::types::common::ByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FieldMemoryUsage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FieldMemoryUsage;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FieldMemoryUsage object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_memory_size: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_memory_size = false;
                let mut f_memory_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "memory_size" => {
                            if seen_memory_size {
                                return Err(serde::de::Error::duplicate_field("memory_size"));
                            }
                            seen_memory_size = true;
                            f_memory_size = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "memory_size_in_bytes" => {
                            if f_memory_size_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "memory_size_in_bytes",
                                ));
                            }
                            f_memory_size_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FieldMemoryUsage {
                    memory_size: f_memory_size,
                    memory_size_in_bytes: f_memory_size_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("memory_size_in_bytes"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FieldMemoryUsage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.memory_size {
            map.serialize_entry("memory_size", v)?;
        }
        map.serialize_entry("memory_size_in_bytes", &self.memory_size_in_bytes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The memory usage statistics for a field."]
#[derive(Debug, Clone)]
pub struct FieldSizeUsage {
    #[doc = "The human-readable size of memory used."]
    pub size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The size of memory used in bytes."]
    pub size_in_bytes: crate::types::common::ByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FieldSizeUsage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FieldSizeUsage;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FieldSizeUsage object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_size: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_size = false;
                let mut f_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
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
                Ok(FieldSizeUsage {
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
impl serde::Serialize for FieldSizeUsage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
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

#[doc = "The detailed field sort options."]
#[derive(Debug, Clone)]
pub struct FieldSort {
    #[doc = "The value to use when the field is missing."]
    pub missing: Option<crate::types::common::FieldValue>,
    #[doc = "The mode for sorting on array fields."]
    pub mode: Option<crate::types::common::SortMode>,
    #[doc = "The nested path sort options."]
    pub nested: Option<crate::types::common::NestedSortValue>,
    #[doc = "The numeric type to use for sorting."]
    pub numeric_type: Option<crate::types::common::FieldSortNumericType>,
    #[doc = "The sort order direction."]
    pub order: Option<crate::types::common::SortOrder>,
    #[doc = "The type to use for unmapped fields."]
    pub unmapped_type: Option<crate::types::common_mapping::FieldType>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FieldSort {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FieldSort;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FieldSort object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_missing: Option<crate::types::common::FieldValue> = None;
                let mut seen_missing = false;
                let mut f_mode: Option<crate::types::common::SortMode> = None;
                let mut seen_mode = false;
                let mut f_nested: Option<crate::types::common::NestedSortValue> = None;
                let mut seen_nested = false;
                let mut f_numeric_type: Option<crate::types::common::FieldSortNumericType> = None;
                let mut seen_numeric_type = false;
                let mut f_order: Option<crate::types::common::SortOrder> = None;
                let mut seen_order = false;
                let mut f_unmapped_type: Option<crate::types::common_mapping::FieldType> = None;
                let mut seen_unmapped_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "missing" => {
                            if seen_missing {
                                return Err(serde::de::Error::duplicate_field("missing"));
                            }
                            seen_missing = true;
                            f_missing =
                                map.next_value::<Option<crate::types::common::FieldValue>>()?;
                        }
                        "mode" => {
                            if seen_mode {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            seen_mode = true;
                            f_mode = map.next_value::<Option<crate::types::common::SortMode>>()?;
                        }
                        "nested" => {
                            if seen_nested {
                                return Err(serde::de::Error::duplicate_field("nested"));
                            }
                            seen_nested = true;
                            f_nested =
                                map.next_value::<Option<crate::types::common::NestedSortValue>>()?;
                        }
                        "numeric_type" => {
                            if seen_numeric_type {
                                return Err(serde::de::Error::duplicate_field("numeric_type"));
                            }
                            seen_numeric_type = true;
                            f_numeric_type = map
                                .next_value::<Option<crate::types::common::FieldSortNumericType>>(
                                )?;
                        }
                        "order" => {
                            if seen_order {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            seen_order = true;
                            f_order =
                                map.next_value::<Option<crate::types::common::SortOrder>>()?;
                        }
                        "unmapped_type" => {
                            if seen_unmapped_type {
                                return Err(serde::de::Error::duplicate_field("unmapped_type"));
                            }
                            seen_unmapped_type = true;
                            f_unmapped_type = map
                                .next_value::<Option<crate::types::common_mapping::FieldType>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FieldSort {
                    missing: f_missing,
                    mode: f_mode,
                    nested: f_nested,
                    numeric_type: f_numeric_type,
                    order: f_order,
                    unmapped_type: f_unmapped_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FieldSort {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.missing {
            map.serialize_entry("missing", v)?;
        }
        if let Some(v) = &self.mode {
            map.serialize_entry("mode", v)?;
        }
        if let Some(v) = &self.nested {
            map.serialize_entry("nested", v)?;
        }
        if let Some(v) = &self.numeric_type {
            map.serialize_entry("numeric_type", v)?;
        }
        if let Some(v) = &self.order {
            map.serialize_entry("order", v)?;
        }
        if let Some(v) = &self.unmapped_type {
            map.serialize_entry("unmapped_type", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldSortNumericType {
    #[doc = "`date`"]
    Date,
    #[doc = "`date_nanos`"]
    DateNanos,
    #[doc = "`double`"]
    Double,
    #[doc = "`long`"]
    Long,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl FieldSortNumericType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            FieldSortNumericType::Date => "date",
            FieldSortNumericType::DateNanos => "date_nanos",
            FieldSortNumericType::Double => "double",
            FieldSortNumericType::Long => "long",
            FieldSortNumericType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for FieldSortNumericType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FieldSortNumericType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "date" => FieldSortNumericType::Date,
            "date_nanos" => FieldSortNumericType::DateNanos,
            "double" => FieldSortNumericType::Double,
            "long" => FieldSortNumericType::Long,
            _ => FieldSortNumericType::Unknown(s),
        })
    }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b't') | Some(b'f') => serde_json::from_str(text).map(FieldValue::Boolean),
            Some(b'-') | Some(b'0'..=b'9') => serde_json::from_str(text).map(FieldValue::Number),
            Some(b'"') => serde_json::from_str(text).map(FieldValue::String),
            Some(b'n') => return Ok(FieldValue::Null),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a FieldValue value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

pub type FieldWithDirection = std::collections::BTreeMap<String, crate::types::common::SortOrder>;

pub type FieldWithOrder = std::collections::BTreeMap<String, crate::types::common::FieldSort>;

#[doc = "The statistics about field data memory usage."]
#[derive(Debug, Clone)]
pub struct FielddataStats {
    #[doc = "The number of times field data was evicted from memory."]
    pub evictions: Option<i64>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common::FieldMemoryUsage>>,
    #[doc = "The human-readable amount of memory used for field data."]
    pub memory_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The amount of memory used for field data in bytes."]
    pub memory_size_in_bytes: crate::types::common::ByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FielddataStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FielddataStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FielddataStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_evictions: Option<i64> = None;
                let mut seen_evictions = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common::FieldMemoryUsage>,
                > = None;
                let mut seen_fields = false;
                let mut f_memory_size: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_memory_size = false;
                let mut f_memory_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "evictions" => {
                            if seen_evictions {
                                return Err(serde::de::Error::duplicate_field("evictions"));
                            }
                            seen_evictions = true;
                            f_evictions = map.next_value::<Option<i64>>()?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common::FieldMemoryUsage,
                                >,
                            >>()?;
                        }
                        "memory_size" => {
                            if seen_memory_size {
                                return Err(serde::de::Error::duplicate_field("memory_size"));
                            }
                            seen_memory_size = true;
                            f_memory_size = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "memory_size_in_bytes" => {
                            if f_memory_size_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "memory_size_in_bytes",
                                ));
                            }
                            f_memory_size_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FielddataStats {
                    evictions: f_evictions,
                    fields: f_fields,
                    memory_size: f_memory_size,
                    memory_size_in_bytes: f_memory_size_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("memory_size_in_bytes"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FielddataStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.evictions {
            map.serialize_entry("evictions", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.memory_size {
            map.serialize_entry("memory_size", v)?;
        }
        map.serialize_entry("memory_size_in_bytes", &self.memory_size_in_bytes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'"') => serde_json::from_str(text).map(Fields::Field),
            Some(b'[') => serde_json::from_str(text).map(Fields::Array),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a Fields value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The statistics about flush operations."]
#[derive(Debug, Clone)]
pub struct FlushStats {
    #[doc = "The number of periodic flush operations."]
    pub periodic: i64,
    #[doc = "The total number of flush operations."]
    pub total: i64,
    pub total_time: Option<crate::types::common::Duration>,
    pub total_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FlushStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FlushStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FlushStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_periodic: Option<i64> = None;
                let mut f_total: Option<i64> = None;
                let mut f_total_time: Option<crate::types::common::Duration> = None;
                let mut seen_total_time = false;
                let mut f_total_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "periodic" => {
                            if f_periodic.is_some() {
                                return Err(serde::de::Error::duplicate_field("periodic"));
                            }
                            f_periodic = Some(map.next_value::<i64>()?);
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(map.next_value::<i64>()?);
                        }
                        "total_time" => {
                            if seen_total_time {
                                return Err(serde::de::Error::duplicate_field("total_time"));
                            }
                            seen_total_time = true;
                            f_total_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "total_time_in_millis" => {
                            if f_total_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_time_in_millis",
                                ));
                            }
                            f_total_time_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FlushStats {
                    periodic: f_periodic
                        .ok_or_else(|| serde::de::Error::missing_field("periodic"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    total_time: f_total_time,
                    total_time_in_millis: f_total_time_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("total_time_in_millis"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FlushStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("periodic", &self.periodic)?;
        map.serialize_entry("total", &self.total)?;
        if let Some(v) = &self.total_time {
            map.serialize_entry("total_time", v)?;
        }
        map.serialize_entry("total_time_in_millis", &self.total_time_in_millis)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'"') => serde_json::from_str(text).map(Fuzziness::String),
            Some(b'-') | Some(b'0'..=b'9') => serde_json::from_str(text).map(Fuzziness::Integer),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a Fuzziness value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
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
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_distance_type: Option<crate::types::common::GeoDistanceType> = None;
                let mut seen_distance_type = false;
                let mut f_ignore_unmapped: Option<bool> = None;
                let mut seen_ignore_unmapped = false;
                let mut f_mode: Option<crate::types::common::SortMode> = None;
                let mut seen_mode = false;
                let mut f_nested: Option<crate::types::common::NestedSortValue> = None;
                let mut seen_nested = false;
                let mut f_order: Option<crate::types::common::SortOrder> = None;
                let mut seen_order = false;
                let mut f_unit: Option<crate::types::common::DistanceUnit> = None;
                let mut seen_unit = false;
                let mut f_validation_method: Option<
                    crate::types::common_query_dsl::GeoValidationMethod,
                > = None;
                let mut seen_validation_method = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Vec<crate::types::common::GeoLocation>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "distance_type" => {
                            if seen_distance_type {
                                return Err(serde::de::Error::duplicate_field("distance_type"));
                            }
                            seen_distance_type = true;
                            f_distance_type =
                                map.next_value::<Option<crate::types::common::GeoDistanceType>>()?;
                        }
                        "ignore_unmapped" => {
                            if seen_ignore_unmapped {
                                return Err(serde::de::Error::duplicate_field("ignore_unmapped"));
                            }
                            seen_ignore_unmapped = true;
                            f_ignore_unmapped = map.next_value::<Option<bool>>()?;
                        }
                        "mode" => {
                            if seen_mode {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            seen_mode = true;
                            f_mode = map.next_value::<Option<crate::types::common::SortMode>>()?;
                        }
                        "nested" => {
                            if seen_nested {
                                return Err(serde::de::Error::duplicate_field("nested"));
                            }
                            seen_nested = true;
                            f_nested =
                                map.next_value::<Option<crate::types::common::NestedSortValue>>()?;
                        }
                        "order" => {
                            if seen_order {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            seen_order = true;
                            f_order =
                                map.next_value::<Option<crate::types::common::SortOrder>>()?;
                        }
                        "unit" => {
                            if seen_unit {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            seen_unit = true;
                            f_unit =
                                map.next_value::<Option<crate::types::common::DistanceUnit>>()?;
                        }
                        "validation_method" => {
                            if seen_validation_method {
                                return Err(serde::de::Error::duplicate_field("validation_method"));
                            }
                            seen_validation_method = true;
                            f_validation_method = map.next_value::<Option<crate::types::common_query_dsl::GeoValidationMethod>>()?;
                        }
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GeoDistanceType {
    #[doc = "`arc`"]
    Arc,
    #[doc = "`plane`"]
    Plane,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl GeoDistanceType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            GeoDistanceType::Arc => "arc",
            GeoDistanceType::Plane => "plane",
            GeoDistanceType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for GeoDistanceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GeoDistanceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "arc" => GeoDistanceType::Arc,
            "plane" => GeoDistanceType::Plane,
            _ => GeoDistanceType::Unknown(s),
        })
    }
}

#[doc = "The geohash string representation of a geographical location."]
pub type GeoHash = String;

#[doc = "The geographical location specified using a geohash."]
#[derive(Debug, Clone)]
pub struct GeoHashLocation {
    pub geohash: crate::types::common::GeoHash,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GeoHashLocation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GeoHashLocation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoHashLocation object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_geohash: Option<crate::types::common::GeoHash> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "geohash" => {
                            if f_geohash.is_some() {
                                return Err(serde::de::Error::duplicate_field("geohash"));
                            }
                            f_geohash = Some(map.next_value::<crate::types::common::GeoHash>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GeoHashLocation {
                    geohash: f_geohash.ok_or_else(|| serde::de::Error::missing_field("geohash"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GeoHashLocation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("geohash", &self.geohash)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(GeoHashPrecision::GeohashLength)
            }
            Some(b'"') => serde_json::from_str(text).map(GeoHashPrecision::Distance),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a GeoHashPrecision value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GeoShapeRelation {
    #[doc = "`contains`"]
    Contains,
    #[doc = "`disjoint`"]
    Disjoint,
    #[doc = "`intersects`"]
    Intersects,
    #[doc = "`within`"]
    Within,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl GeoShapeRelation {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            GeoShapeRelation::Contains => "contains",
            GeoShapeRelation::Disjoint => "disjoint",
            GeoShapeRelation::Intersects => "intersects",
            GeoShapeRelation::Within => "within",
            GeoShapeRelation::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for GeoShapeRelation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GeoShapeRelation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "contains" => GeoShapeRelation::Contains,
            "disjoint" => GeoShapeRelation::Disjoint,
            "intersects" => GeoShapeRelation::Intersects,
            "within" => GeoShapeRelation::Within,
            _ => GeoShapeRelation::Unknown(s),
        })
    }
}

#[doc = "A map tile reference, represented as `{zoom}/{x}/{y}`."]
pub type GeoTile = String;

#[doc = "The statistics about get operations."]
#[derive(Debug, Clone)]
pub struct GetStats {
    #[doc = "The number of current get operations."]
    pub current: i64,
    pub exists_time: Option<crate::types::common::Duration>,
    pub exists_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The total number of successful get operations."]
    pub exists_total: i64,
    pub get_time: Option<crate::types::common::Duration>,
    pub missing_time: Option<crate::types::common::Duration>,
    pub missing_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The total number of failed get operations due to missing documents."]
    pub missing_total: i64,
    pub time: Option<crate::types::common::Duration>,
    pub time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The total number of get operations."]
    pub total: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GetStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GetStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GetStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_current: Option<i64> = None;
                let mut f_exists_time: Option<crate::types::common::Duration> = None;
                let mut seen_exists_time = false;
                let mut f_exists_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_exists_total: Option<i64> = None;
                let mut f_get_time: Option<crate::types::common::Duration> = None;
                let mut seen_get_time = false;
                let mut f_missing_time: Option<crate::types::common::Duration> = None;
                let mut seen_missing_time = false;
                let mut f_missing_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_missing_total: Option<i64> = None;
                let mut f_time: Option<crate::types::common::Duration> = None;
                let mut seen_time = false;
                let mut f_time_in_millis: Option<crate::types::common::DurationValueUnitMillis> =
                    None;
                let mut f_total: Option<i64> = None;
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
                            f_current = Some(map.next_value::<i64>()?);
                        }
                        "exists_time" => {
                            if seen_exists_time {
                                return Err(serde::de::Error::duplicate_field("exists_time"));
                            }
                            seen_exists_time = true;
                            f_exists_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "exists_time_in_millis" => {
                            if f_exists_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "exists_time_in_millis",
                                ));
                            }
                            f_exists_time_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "exists_total" => {
                            if f_exists_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("exists_total"));
                            }
                            f_exists_total = Some(map.next_value::<i64>()?);
                        }
                        "getTime" => {
                            if seen_get_time {
                                return Err(serde::de::Error::duplicate_field("getTime"));
                            }
                            seen_get_time = true;
                            f_get_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "missing_time" => {
                            if seen_missing_time {
                                return Err(serde::de::Error::duplicate_field("missing_time"));
                            }
                            seen_missing_time = true;
                            f_missing_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "missing_time_in_millis" => {
                            if f_missing_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "missing_time_in_millis",
                                ));
                            }
                            f_missing_time_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "missing_total" => {
                            if f_missing_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("missing_total"));
                            }
                            f_missing_total = Some(map.next_value::<i64>()?);
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
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GetStats {
                    current: f_current.ok_or_else(|| serde::de::Error::missing_field("current"))?,
                    exists_time: f_exists_time,
                    exists_time_in_millis: f_exists_time_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("exists_time_in_millis"))?,
                    exists_total: f_exists_total
                        .ok_or_else(|| serde::de::Error::missing_field("exists_total"))?,
                    get_time: f_get_time,
                    missing_time: f_missing_time,
                    missing_time_in_millis: f_missing_time_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("missing_time_in_millis"))?,
                    missing_total: f_missing_total
                        .ok_or_else(|| serde::de::Error::missing_field("missing_total"))?,
                    time: f_time,
                    time_in_millis: f_time_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("time_in_millis"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GetStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("current", &self.current)?;
        if let Some(v) = &self.exists_time {
            map.serialize_entry("exists_time", v)?;
        }
        map.serialize_entry("exists_time_in_millis", &self.exists_time_in_millis)?;
        map.serialize_entry("exists_total", &self.exists_total)?;
        if let Some(v) = &self.get_time {
            map.serialize_entry("getTime", v)?;
        }
        if let Some(v) = &self.missing_time {
            map.serialize_entry("missing_time", v)?;
        }
        map.serialize_entry("missing_time_in_millis", &self.missing_time_in_millis)?;
        map.serialize_entry("missing_total", &self.missing_total)?;
        if let Some(v) = &self.time {
            map.serialize_entry("time", v)?;
        }
        map.serialize_entry("time_in_millis", &self.time_in_millis)?;
        map.serialize_entry("total", &self.total)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HealthStatus {
    #[doc = "`green` (also accepted: `GREEN`)"]
    Green,
    #[doc = "`yellow` (also accepted: `YELLOW`)"]
    Yellow,
    #[doc = "`red` (also accepted: `RED`)"]
    Red,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl HealthStatus {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            HealthStatus::Green => "green",
            HealthStatus::Yellow => "yellow",
            HealthStatus::Red => "red",
            HealthStatus::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for HealthStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HealthStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "green" | "GREEN" => HealthStatus::Green,
            "yellow" | "YELLOW" => HealthStatus::Yellow,
            "red" | "RED" => HealthStatus::Red,
            _ => HealthStatus::Unknown(s),
        })
    }
}

#[doc = "The hostname or IP address."]
pub type Host = String;

#[doc = "The HTTP headers."]
pub type HttpHeaders =
    std::collections::BTreeMap<String, crate::types::common::StringOrStringArray>;

#[doc = "The unique identifier of a node."]
pub type HumanReadableByteCount = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IBDistribution {
    #[doc = "`ll`"]
    Ll,
    #[doc = "`spl`"]
    Spl,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IBDistribution {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IBDistribution::Ll => "ll",
            IBDistribution::Spl => "spl",
            IBDistribution::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IBDistribution {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IBDistribution {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ll" => IBDistribution::Ll,
            "spl" => IBDistribution::Spl,
            _ => IBDistribution::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IBLambda {
    #[doc = "`df`"]
    Df,
    #[doc = "`ttf`"]
    Ttf,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IBLambda {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IBLambda::Df => "df",
            IBLambda::Ttf => "ttf",
            IBLambda::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IBLambda {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IBLambda {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "df" => IBLambda::Df,
            "ttf" => IBLambda::Ttf,
            _ => IBLambda::Unknown(s),
        })
    }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'"') => serde_json::from_str(text).map(Ids::Id),
            Some(b'[') => serde_json::from_str(text).map(Ids::Array),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a Ids value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The name of an index alias."]
pub type IndexAlias = String;

pub type IndexName = String;

#[doc = "The statistics about indexing operations."]
#[derive(Debug, Clone)]
pub struct IndexingStats {
    #[doc = "The number of current delete operations."]
    pub delete_current: i64,
    pub delete_time: Option<crate::types::common::Duration>,
    pub delete_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The total number of delete operations."]
    pub delete_total: i64,
    pub doc_status: Option<crate::types::common::DocStatus>,
    #[doc = "The number of current indexing operations."]
    pub index_current: i64,
    #[doc = "The number of failed indexing operations."]
    pub index_failed: i64,
    #[doc = "The total amount of time spent on indexing operations."]
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
    pub throttle_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time spent throttling indexing operations in milliseconds."]
    pub throttle_time_in_millis: crate::types::common::DurationValueUnitMillis,
    pub types: Option<std::collections::BTreeMap<String, crate::types::common::IndexingStats>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexingStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexingStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexingStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_delete_current: Option<i64> = None;
                let mut f_delete_time: Option<crate::types::common::Duration> = None;
                let mut seen_delete_time = false;
                let mut f_delete_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_delete_total: Option<i64> = None;
                let mut f_doc_status: Option<crate::types::common::DocStatus> = None;
                let mut seen_doc_status = false;
                let mut f_index_current: Option<i64> = None;
                let mut f_index_failed: Option<i64> = None;
                let mut f_index_time: Option<crate::types::common::Duration> = None;
                let mut seen_index_time = false;
                let mut f_index_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_index_total: Option<i64> = None;
                let mut f_is_throttled: Option<bool> = None;
                let mut f_noop_update_total: Option<i64> = None;
                let mut f_throttle_time: Option<crate::types::common::Duration> = None;
                let mut seen_throttle_time = false;
                let mut f_throttle_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_types: Option<
                    std::collections::BTreeMap<String, crate::types::common::IndexingStats>,
                > = None;
                let mut seen_types = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "delete_current" => {
                            if f_delete_current.is_some() {
                                return Err(serde::de::Error::duplicate_field("delete_current"));
                            }
                            f_delete_current = Some(map.next_value::<i64>()?);
                        }
                        "delete_time" => {
                            if seen_delete_time {
                                return Err(serde::de::Error::duplicate_field("delete_time"));
                            }
                            seen_delete_time = true;
                            f_delete_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "delete_time_in_millis" => {
                            if f_delete_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "delete_time_in_millis",
                                ));
                            }
                            f_delete_time_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "delete_total" => {
                            if f_delete_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("delete_total"));
                            }
                            f_delete_total = Some(map.next_value::<i64>()?);
                        }
                        "doc_status" => {
                            if seen_doc_status {
                                return Err(serde::de::Error::duplicate_field("doc_status"));
                            }
                            seen_doc_status = true;
                            f_doc_status =
                                map.next_value::<Option<crate::types::common::DocStatus>>()?;
                        }
                        "index_current" => {
                            if f_index_current.is_some() {
                                return Err(serde::de::Error::duplicate_field("index_current"));
                            }
                            f_index_current = Some(map.next_value::<i64>()?);
                        }
                        "index_failed" => {
                            if f_index_failed.is_some() {
                                return Err(serde::de::Error::duplicate_field("index_failed"));
                            }
                            f_index_failed = Some(map.next_value::<i64>()?);
                        }
                        "index_time" => {
                            if seen_index_time {
                                return Err(serde::de::Error::duplicate_field("index_time"));
                            }
                            seen_index_time = true;
                            f_index_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "index_time_in_millis" => {
                            if f_index_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "index_time_in_millis",
                                ));
                            }
                            f_index_time_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "index_total" => {
                            if f_index_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("index_total"));
                            }
                            f_index_total = Some(map.next_value::<i64>()?);
                        }
                        "is_throttled" => {
                            if f_is_throttled.is_some() {
                                return Err(serde::de::Error::duplicate_field("is_throttled"));
                            }
                            f_is_throttled = Some(map.next_value::<bool>()?);
                        }
                        "noop_update_total" => {
                            if f_noop_update_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("noop_update_total"));
                            }
                            f_noop_update_total = Some(map.next_value::<i64>()?);
                        }
                        "throttle_time" => {
                            if seen_throttle_time {
                                return Err(serde::de::Error::duplicate_field("throttle_time"));
                            }
                            seen_throttle_time = true;
                            f_throttle_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "throttle_time_in_millis" => {
                            if f_throttle_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "throttle_time_in_millis",
                                ));
                            }
                            f_throttle_time_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "types" => {
                            if seen_types {
                                return Err(serde::de::Error::duplicate_field("types"));
                            }
                            seen_types = true;
                            f_types = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common::IndexingStats,
                                >,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexingStats {
                    delete_current: f_delete_current
                        .ok_or_else(|| serde::de::Error::missing_field("delete_current"))?,
                    delete_time: f_delete_time,
                    delete_time_in_millis: f_delete_time_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("delete_time_in_millis"))?,
                    delete_total: f_delete_total
                        .ok_or_else(|| serde::de::Error::missing_field("delete_total"))?,
                    doc_status: f_doc_status,
                    index_current: f_index_current
                        .ok_or_else(|| serde::de::Error::missing_field("index_current"))?,
                    index_failed: f_index_failed
                        .ok_or_else(|| serde::de::Error::missing_field("index_failed"))?,
                    index_time: f_index_time,
                    index_time_in_millis: f_index_time_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("index_time_in_millis"))?,
                    index_total: f_index_total
                        .ok_or_else(|| serde::de::Error::missing_field("index_total"))?,
                    is_throttled: f_is_throttled
                        .ok_or_else(|| serde::de::Error::missing_field("is_throttled"))?,
                    noop_update_total: f_noop_update_total
                        .ok_or_else(|| serde::de::Error::missing_field("noop_update_total"))?,
                    throttle_time: f_throttle_time,
                    throttle_time_in_millis: f_throttle_time_in_millis.ok_or_else(|| {
                        serde::de::Error::missing_field("throttle_time_in_millis")
                    })?,
                    types: f_types,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexingStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("delete_current", &self.delete_current)?;
        if let Some(v) = &self.delete_time {
            map.serialize_entry("delete_time", v)?;
        }
        map.serialize_entry("delete_time_in_millis", &self.delete_time_in_millis)?;
        map.serialize_entry("delete_total", &self.delete_total)?;
        if let Some(v) = &self.doc_status {
            map.serialize_entry("doc_status", v)?;
        }
        map.serialize_entry("index_current", &self.index_current)?;
        map.serialize_entry("index_failed", &self.index_failed)?;
        if let Some(v) = &self.index_time {
            map.serialize_entry("index_time", v)?;
        }
        map.serialize_entry("index_time_in_millis", &self.index_time_in_millis)?;
        map.serialize_entry("index_total", &self.index_total)?;
        map.serialize_entry("is_throttled", &self.is_throttled)?;
        map.serialize_entry("noop_update_total", &self.noop_update_total)?;
        if let Some(v) = &self.throttle_time {
            map.serialize_entry("throttle_time", v)?;
        }
        map.serialize_entry("throttle_time_in_millis", &self.throttle_time_in_millis)?;
        if let Some(v) = &self.types {
            map.serialize_entry("types", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'"') => serde_json::from_str(text).map(Indices::IndexName),
            Some(b'[') => serde_json::from_str(text).map(Indices::Array),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a Indices value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct IndicesResponseBase {
    #[doc = "For a successful response, this value is always true. On failure, an exception is returned instead."]
    pub acknowledged: bool,
    pub shards: Option<crate::types::common::ShardStatistics>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndicesResponseBase {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndicesResponseBase;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndicesResponseBase object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_acknowledged: Option<bool> = None;
                let mut f_shards: Option<crate::types::common::ShardStatistics> = None;
                let mut seen_shards = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "acknowledged" => {
                            if f_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field("acknowledged"));
                            }
                            f_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        "_shards" => {
                            if seen_shards {
                                return Err(serde::de::Error::duplicate_field("_shards"));
                            }
                            seen_shards = true;
                            f_shards =
                                map.next_value::<Option<crate::types::common::ShardStatistics>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndicesResponseBase {
                    acknowledged: f_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("acknowledged"))?,
                    shards: f_shards,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndicesResponseBase {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("acknowledged", &self.acknowledged)?;
        if let Some(v) = &self.shards {
            map.serialize_entry("_shards", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_primary_term: Option<i64> = None;
                let mut seen_primary_term = false;
                let mut f_routing: Option<crate::types::common::Routing> = None;
                let mut seen_routing = false;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut seen_seq_no = false;
                let mut f_source: Option<Box<serde_json::value::RawValue>> = None;
                let mut seen_source = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_fields = false;
                let mut f_found: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_primary_term" => {
                            if seen_primary_term {
                                return Err(serde::de::Error::duplicate_field("_primary_term"));
                            }
                            seen_primary_term = true;
                            f_primary_term = map.next_value::<Option<i64>>()?;
                        }
                        "_routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("_routing"));
                            }
                            seen_routing = true;
                            f_routing =
                                map.next_value::<Option<crate::types::common::Routing>>()?;
                        }
                        "_seq_no" => {
                            if seen_seq_no {
                                return Err(serde::de::Error::duplicate_field("_seq_no"));
                            }
                            seen_seq_no = true;
                            f_seq_no =
                                map.next_value::<Option<crate::types::common::SequenceNumber>>()?;
                        }
                        "_source" => {
                            if seen_source {
                                return Err(serde::de::Error::duplicate_field("_source"));
                            }
                            seen_source = true;
                            f_source =
                                map.next_value::<Option<Box<serde_json::value::RawValue>>>()?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "found" => {
                            if f_found.is_some() {
                                return Err(serde::de::Error::duplicate_field("found"));
                            }
                            f_found = Some(map.next_value::<bool>()?);
                        }
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
impl InlineGet {
    #[doc = "Decodes the document body (`_source`) into `T`.\n\nReturns `Ok(None)` when the response carries no `_source` (`_source: false` in the request, or a mapping that disables it), and an error when the document does not match `T`.\n\n`T` may borrow from the response to avoid copying strings: use `#[serde(borrow)] Cow<'a, str>` fields rather than `&'a str`, which cannot hold a string containing JSON escapes."]
    pub fn source_as<'a, T>(&'a self) -> std::result::Result<Option<T>, serde_json::Error>
    where
        T: serde::Deserialize<'a>,
    {
        self.source
            .as_deref()
            .map(|raw| serde_json::from_str(raw.get()))
            .transpose()
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
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_primary_term: Option<i64> = None;
                let mut seen_primary_term = false;
                let mut f_routing: Option<crate::types::common::Routing> = None;
                let mut seen_routing = false;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut seen_seq_no = false;
                let mut f_source: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_source = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<
                        String,
                        std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                    >,
                > = None;
                let mut seen_fields = false;
                let mut f_found: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_primary_term" => {
                            if seen_primary_term {
                                return Err(serde::de::Error::duplicate_field("_primary_term"));
                            }
                            seen_primary_term = true;
                            f_primary_term = map.next_value::<Option<i64>>()?;
                        }
                        "_routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("_routing"));
                            }
                            seen_routing = true;
                            f_routing =
                                map.next_value::<Option<crate::types::common::Routing>>()?;
                        }
                        "_seq_no" => {
                            if seen_seq_no {
                                return Err(serde::de::Error::duplicate_field("_seq_no"));
                            }
                            seen_seq_no = true;
                            f_seq_no =
                                map.next_value::<Option<crate::types::common::SequenceNumber>>()?;
                        }
                        "_source" => {
                            if seen_source {
                                return Err(serde::de::Error::duplicate_field("_source"));
                            }
                            seen_source = true;
                            f_source = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    std::collections::BTreeMap<
                                        String,
                                        Box<serde_json::value::RawValue>,
                                    >,
                                >,
                            >>()?;
                        }
                        "found" => {
                            if f_found.is_some() {
                                return Err(serde::de::Error::duplicate_field("found"));
                            }
                            f_found = Some(map.next_value::<bool>()?);
                        }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'"') => serde_json::from_str(text).map(InlineScript::Source),
            Some(b'{') => serde_json::from_str(text).map(InlineScript::Object),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a InlineScript value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct InlineScriptObject {
    #[doc = "Specifies any named parameters that are passed into the script as variables. Use parameters instead of hard-coded values to decrease compilation time."]
    pub params: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub lang: Option<crate::types::common::ScriptLanguage>,
    pub options: Option<std::collections::BTreeMap<String, String>>,
    #[doc = "The script source."]
    pub source: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for InlineScriptObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = InlineScriptObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a InlineScriptObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_params: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_params = false;
                let mut f_lang: Option<crate::types::common::ScriptLanguage> = None;
                let mut seen_lang = false;
                let mut f_options: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_options = false;
                let mut f_source: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "params" => {
                            if seen_params {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            seen_params = true;
                            f_params = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "lang" => {
                            if seen_lang {
                                return Err(serde::de::Error::duplicate_field("lang"));
                            }
                            seen_lang = true;
                            f_lang =
                                map.next_value::<Option<crate::types::common::ScriptLanguage>>()?;
                        }
                        "options" => {
                            if seen_options {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            seen_options = true;
                            f_options = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "source" => {
                            if f_source.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            f_source = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(InlineScriptObject {
                    params: f_params,
                    lang: f_lang,
                    options: f_options,
                    source: f_source.ok_or_else(|| serde::de::Error::missing_field("source"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for InlineScriptObject {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.params {
            map.serialize_entry("params", v)?;
        }
        if let Some(v) = &self.lang {
            map.serialize_entry("lang", v)?;
        }
        if let Some(v) = &self.options {
            map.serialize_entry("options", v)?;
        }
        map.serialize_entry("source", &self.source)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The IP address."]
pub type Ip = String;

#[doc = "The geographical location specified using latitude and longitude."]
#[derive(Debug, Clone)]
pub struct LatLonGeoLocation {
    #[doc = "The latitude coordinate."]
    pub lat: f64,
    #[doc = "The longitude coordinate."]
    pub lon: f64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for LatLonGeoLocation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = LatLonGeoLocation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a LatLonGeoLocation object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_lat: Option<f64> = None;
                let mut f_lon: Option<f64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "lat" => {
                            if f_lat.is_some() {
                                return Err(serde::de::Error::duplicate_field("lat"));
                            }
                            f_lat = Some(map.next_value::<f64>()?);
                        }
                        "lon" => {
                            if f_lon.is_some() {
                                return Err(serde::de::Error::duplicate_field("lon"));
                            }
                            f_lon = Some(map.next_value::<f64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(LatLonGeoLocation {
                    lat: f_lat.ok_or_else(|| serde::de::Error::missing_field("lat"))?,
                    lon: f_lon.ok_or_else(|| serde::de::Error::missing_field("lon"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for LatLonGeoLocation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("lat", &self.lat)?;
        map.serialize_entry("lon", &self.lon)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The statistics about merge operations."]
#[derive(Debug, Clone)]
pub struct MergesStats {
    #[doc = "The number of current merge operations."]
    pub current: i64,
    #[doc = "The number of documents in current merge operations."]
    pub current_docs: i64,
    #[doc = "The size of current merge operations in a human-readable format."]
    pub current_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The size of current merge operations in bytes."]
    pub current_size_in_bytes: crate::types::common::ByteCount,
    #[doc = "The total number of merge operations."]
    pub total: i64,
    #[doc = "The total auto-throttle size in a human-readable format."]
    pub total_auto_throttle: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total auto-throttle size in bytes."]
    pub total_auto_throttle_in_bytes: crate::types::common::ByteCount,
    #[doc = "The total number of documents that have been merged."]
    pub total_docs: i64,
    #[doc = "The total size of merge operations in a human-readable format."]
    pub total_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total size of merge operations in bytes."]
    pub total_size_in_bytes: crate::types::common::ByteCount,
    #[doc = "The total amount of time merge operations have been stopped."]
    pub total_stopped_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time merge operations have been stopped in milliseconds."]
    pub total_stopped_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The total amount of time merge operations have been throttled."]
    pub total_throttled_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time merge operations have been throttled in milliseconds."]
    pub total_throttled_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The total amount of time spent on merge operations."]
    pub total_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time spent on merge operations in milliseconds."]
    pub total_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The number of unreferenced file cleanups performed."]
    pub unreferenced_file_cleanups_performed: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MergesStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MergesStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MergesStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_current: Option<i64> = None;
                let mut f_current_docs: Option<i64> = None;
                let mut f_current_size: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_current_size = false;
                let mut f_current_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_total: Option<i64> = None;
                let mut f_total_auto_throttle: Option<
                    crate::types::common::HumanReadableByteCount,
                > = None;
                let mut seen_total_auto_throttle = false;
                let mut f_total_auto_throttle_in_bytes: Option<crate::types::common::ByteCount> =
                    None;
                let mut f_total_docs: Option<i64> = None;
                let mut f_total_size: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_total_size = false;
                let mut f_total_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_total_stopped_time: Option<crate::types::common::Duration> = None;
                let mut seen_total_stopped_time = false;
                let mut f_total_stopped_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_total_throttled_time: Option<crate::types::common::Duration> = None;
                let mut seen_total_throttled_time = false;
                let mut f_total_throttled_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_total_time: Option<crate::types::common::Duration> = None;
                let mut seen_total_time = false;
                let mut f_total_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_unreferenced_file_cleanups_performed: Option<i64> = None;
                let mut seen_unreferenced_file_cleanups_performed = false;
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
                            f_current = Some(map.next_value::<i64>()?);
                        }
                        "current_docs" => {
                            if f_current_docs.is_some() {
                                return Err(serde::de::Error::duplicate_field("current_docs"));
                            }
                            f_current_docs = Some(map.next_value::<i64>()?);
                        }
                        "current_size" => {
                            if seen_current_size {
                                return Err(serde::de::Error::duplicate_field("current_size"));
                            }
                            seen_current_size = true;
                            f_current_size = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "current_size_in_bytes" => {
                            if f_current_size_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "current_size_in_bytes",
                                ));
                            }
                            f_current_size_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(map.next_value::<i64>()?);
                        }
                        "total_auto_throttle" => {
                            if seen_total_auto_throttle {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_auto_throttle",
                                ));
                            }
                            seen_total_auto_throttle = true;
                            f_total_auto_throttle = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "total_auto_throttle_in_bytes" => {
                            if f_total_auto_throttle_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_auto_throttle_in_bytes",
                                ));
                            }
                            f_total_auto_throttle_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "total_docs" => {
                            if f_total_docs.is_some() {
                                return Err(serde::de::Error::duplicate_field("total_docs"));
                            }
                            f_total_docs = Some(map.next_value::<i64>()?);
                        }
                        "total_size" => {
                            if seen_total_size {
                                return Err(serde::de::Error::duplicate_field("total_size"));
                            }
                            seen_total_size = true;
                            f_total_size = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "total_size_in_bytes" => {
                            if f_total_size_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_size_in_bytes",
                                ));
                            }
                            f_total_size_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "total_stopped_time" => {
                            if seen_total_stopped_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_stopped_time",
                                ));
                            }
                            seen_total_stopped_time = true;
                            f_total_stopped_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "total_stopped_time_in_millis" => {
                            if f_total_stopped_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_stopped_time_in_millis",
                                ));
                            }
                            f_total_stopped_time_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "total_throttled_time" => {
                            if seen_total_throttled_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_throttled_time",
                                ));
                            }
                            seen_total_throttled_time = true;
                            f_total_throttled_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "total_throttled_time_in_millis" => {
                            if f_total_throttled_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_throttled_time_in_millis",
                                ));
                            }
                            f_total_throttled_time_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "total_time" => {
                            if seen_total_time {
                                return Err(serde::de::Error::duplicate_field("total_time"));
                            }
                            seen_total_time = true;
                            f_total_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "total_time_in_millis" => {
                            if f_total_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_time_in_millis",
                                ));
                            }
                            f_total_time_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "unreferenced_file_cleanups_performed" => {
                            if seen_unreferenced_file_cleanups_performed {
                                return Err(serde::de::Error::duplicate_field(
                                    "unreferenced_file_cleanups_performed",
                                ));
                            }
                            seen_unreferenced_file_cleanups_performed = true;
                            f_unreferenced_file_cleanups_performed =
                                map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MergesStats {
                    current: f_current.ok_or_else(|| serde::de::Error::missing_field("current"))?,
                    current_docs: f_current_docs
                        .ok_or_else(|| serde::de::Error::missing_field("current_docs"))?,
                    current_size: f_current_size,
                    current_size_in_bytes: f_current_size_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("current_size_in_bytes"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    total_auto_throttle: f_total_auto_throttle,
                    total_auto_throttle_in_bytes: f_total_auto_throttle_in_bytes.ok_or_else(
                        || serde::de::Error::missing_field("total_auto_throttle_in_bytes"),
                    )?,
                    total_docs: f_total_docs
                        .ok_or_else(|| serde::de::Error::missing_field("total_docs"))?,
                    total_size: f_total_size,
                    total_size_in_bytes: f_total_size_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("total_size_in_bytes"))?,
                    total_stopped_time: f_total_stopped_time,
                    total_stopped_time_in_millis: f_total_stopped_time_in_millis.ok_or_else(
                        || serde::de::Error::missing_field("total_stopped_time_in_millis"),
                    )?,
                    total_throttled_time: f_total_throttled_time,
                    total_throttled_time_in_millis: f_total_throttled_time_in_millis.ok_or_else(
                        || serde::de::Error::missing_field("total_throttled_time_in_millis"),
                    )?,
                    total_time: f_total_time,
                    total_time_in_millis: f_total_time_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("total_time_in_millis"))?,
                    unreferenced_file_cleanups_performed: f_unreferenced_file_cleanups_performed,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MergesStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("current", &self.current)?;
        map.serialize_entry("current_docs", &self.current_docs)?;
        if let Some(v) = &self.current_size {
            map.serialize_entry("current_size", v)?;
        }
        map.serialize_entry("current_size_in_bytes", &self.current_size_in_bytes)?;
        map.serialize_entry("total", &self.total)?;
        if let Some(v) = &self.total_auto_throttle {
            map.serialize_entry("total_auto_throttle", v)?;
        }
        map.serialize_entry(
            "total_auto_throttle_in_bytes",
            &self.total_auto_throttle_in_bytes,
        )?;
        map.serialize_entry("total_docs", &self.total_docs)?;
        if let Some(v) = &self.total_size {
            map.serialize_entry("total_size", v)?;
        }
        map.serialize_entry("total_size_in_bytes", &self.total_size_in_bytes)?;
        if let Some(v) = &self.total_stopped_time {
            map.serialize_entry("total_stopped_time", v)?;
        }
        map.serialize_entry(
            "total_stopped_time_in_millis",
            &self.total_stopped_time_in_millis,
        )?;
        if let Some(v) = &self.total_throttled_time {
            map.serialize_entry("total_throttled_time", v)?;
        }
        map.serialize_entry(
            "total_throttled_time_in_millis",
            &self.total_throttled_time_in_millis,
        )?;
        if let Some(v) = &self.total_time {
            map.serialize_entry("total_time", v)?;
        }
        map.serialize_entry("total_time_in_millis", &self.total_time_in_millis)?;
        if let Some(v) = &self.unreferenced_file_cleanups_performed {
            map.serialize_entry("unreferenced_file_cleanups_performed", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(MinimumShouldMatch::Integer)
            }
            Some(b'"') => serde_json::from_str(text).map(MinimumShouldMatch::String),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a MinimumShouldMatch value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'"') => serde_json::from_str(text).map(Names::Name),
            Some(b'[') => serde_json::from_str(text).map(Names::Array),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a Names value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct NestedSortValue {
    #[doc = "The filter to apply to nested objects."]
    pub filter: Option<crate::types::common_query_dsl::QueryContainer>,
    #[doc = "The maximum number of children to consider for sorting."]
    pub max_children: Option<i32>,
    #[doc = "How to sort hierarchical nested objects."]
    pub nested: Option<Box<crate::types::common::NestedSortValue>>,
    #[doc = "The path to the nested objects."]
    pub path: crate::types::common::Field,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NestedSortValue {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NestedSortValue;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NestedSortValue object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_filter: Option<crate::types::common_query_dsl::QueryContainer> = None;
                let mut seen_filter = false;
                let mut f_max_children: Option<i32> = None;
                let mut seen_max_children = false;
                let mut f_nested: Option<Box<crate::types::common::NestedSortValue>> = None;
                let mut seen_nested = false;
                let mut f_path: Option<crate::types::common::Field> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "filter" => {
                            if seen_filter {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            seen_filter = true;
                            f_filter = map.next_value::<Option<crate::types::common_query_dsl::QueryContainer>>()?;
                        }
                        "max_children" => {
                            if seen_max_children {
                                return Err(serde::de::Error::duplicate_field("max_children"));
                            }
                            seen_max_children = true;
                            f_max_children = map.next_value::<Option<i32>>()?;
                        }
                        "nested" => {
                            if seen_nested {
                                return Err(serde::de::Error::duplicate_field("nested"));
                            }
                            seen_nested = true;
                            f_nested = map
                                .next_value::<Option<Box<crate::types::common::NestedSortValue>>>(
                                )?;
                        }
                        "path" => {
                            if f_path.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            f_path = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NestedSortValue {
                    filter: f_filter,
                    max_children: f_max_children,
                    nested: f_nested,
                    path: f_path.ok_or_else(|| serde::de::Error::missing_field("path"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NestedSortValue {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.filter {
            map.serialize_entry("filter", v)?;
        }
        if let Some(v) = &self.max_children {
            map.serialize_entry("max_children", v)?;
        }
        if let Some(v) = &self.nested {
            map.serialize_entry("nested", v)?;
        }
        map.serialize_entry("path", &self.path)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NodeAttributes {
    #[doc = "A list of node attributes."]
    pub attributes: std::collections::BTreeMap<String, String>,
    pub ephemeral_id: crate::types::common::Id,
    #[doc = "The roles of the node."]
    pub external_id: Option<String>,
    pub id: Option<crate::types::common::NodeId>,
    pub name: crate::types::common::NodeName,
    pub roles: Option<crate::types::common::NodeRoles>,
    pub transport_address: crate::types::common::TransportAddress,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeAttributes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeAttributes;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeAttributes object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_attributes: Option<std::collections::BTreeMap<String, String>> = None;
                let mut f_ephemeral_id: Option<crate::types::common::Id> = None;
                let mut f_external_id: Option<String> = None;
                let mut seen_external_id = false;
                let mut f_id: Option<crate::types::common::NodeId> = None;
                let mut seen_id = false;
                let mut f_name: Option<crate::types::common::NodeName> = None;
                let mut f_roles: Option<crate::types::common::NodeRoles> = None;
                let mut seen_roles = false;
                let mut f_transport_address: Option<crate::types::common::TransportAddress> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "attributes" => {
                            if f_attributes.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            f_attributes = Some(
                                map.next_value::<std::collections::BTreeMap<String, String>>()?,
                            );
                        }
                        "ephemeral_id" => {
                            if f_ephemeral_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("ephemeral_id"));
                            }
                            f_ephemeral_id = Some(map.next_value::<crate::types::common::Id>()?);
                        }
                        "external_id" => {
                            if seen_external_id {
                                return Err(serde::de::Error::duplicate_field("external_id"));
                            }
                            seen_external_id = true;
                            f_external_id = map.next_value::<Option<String>>()?;
                        }
                        "id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<crate::types::common::NodeId>>()?;
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<crate::types::common::NodeName>()?);
                        }
                        "roles" => {
                            if seen_roles {
                                return Err(serde::de::Error::duplicate_field("roles"));
                            }
                            seen_roles = true;
                            f_roles =
                                map.next_value::<Option<crate::types::common::NodeRoles>>()?;
                        }
                        "transport_address" => {
                            if f_transport_address.is_some() {
                                return Err(serde::de::Error::duplicate_field("transport_address"));
                            }
                            f_transport_address =
                                Some(map.next_value::<crate::types::common::TransportAddress>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeAttributes {
                    attributes: f_attributes
                        .ok_or_else(|| serde::de::Error::missing_field("attributes"))?,
                    ephemeral_id: f_ephemeral_id
                        .ok_or_else(|| serde::de::Error::missing_field("ephemeral_id"))?,
                    external_id: f_external_id,
                    id: f_id,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    roles: f_roles,
                    transport_address: f_transport_address
                        .ok_or_else(|| serde::de::Error::missing_field("transport_address"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeAttributes {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("attributes", &self.attributes)?;
        map.serialize_entry("ephemeral_id", &self.ephemeral_id)?;
        if let Some(v) = &self.external_id {
            map.serialize_entry("external_id", v)?;
        }
        if let Some(v) = &self.id {
            map.serialize_entry("id", v)?;
        }
        map.serialize_entry("name", &self.name)?;
        if let Some(v) = &self.roles {
            map.serialize_entry("roles", v)?;
        }
        map.serialize_entry("transport_address", &self.transport_address)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'"') => serde_json::from_str(text).map(NodeIds::NodeId),
            Some(b'[') => serde_json::from_str(text).map(NodeIds::Array),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a NodeIds value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The name of the node."]
pub type NodeName = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeRole {
    #[doc = "`client`"]
    Client,
    #[doc = "`coordinating_only`"]
    CoordinatingOnly,
    #[doc = "`data`"]
    Data,
    #[doc = "`data_cold`"]
    DataCold,
    #[doc = "`data_content`"]
    DataContent,
    #[doc = "`data_frozen`"]
    DataFrozen,
    #[doc = "`data_hot`"]
    DataHot,
    #[doc = "`data_warm`"]
    DataWarm,
    #[doc = "`ingest`"]
    Ingest,
    #[doc = "`ml`"]
    Ml,
    #[doc = "`remote_cluster_client`"]
    RemoteClusterClient,
    #[doc = "`transform`"]
    Transform,
    #[doc = "`voting_only`"]
    VotingOnly,
    #[doc = "`master`"]
    Master,
    #[doc = "`cluster_manager`"]
    ClusterManager,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl NodeRole {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            NodeRole::Client => "client",
            NodeRole::CoordinatingOnly => "coordinating_only",
            NodeRole::Data => "data",
            NodeRole::DataCold => "data_cold",
            NodeRole::DataContent => "data_content",
            NodeRole::DataFrozen => "data_frozen",
            NodeRole::DataHot => "data_hot",
            NodeRole::DataWarm => "data_warm",
            NodeRole::Ingest => "ingest",
            NodeRole::Ml => "ml",
            NodeRole::RemoteClusterClient => "remote_cluster_client",
            NodeRole::Transform => "transform",
            NodeRole::VotingOnly => "voting_only",
            NodeRole::Master => "master",
            NodeRole::ClusterManager => "cluster_manager",
            NodeRole::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for NodeRole {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NodeRole {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "client" => NodeRole::Client,
            "coordinating_only" => NodeRole::CoordinatingOnly,
            "data" => NodeRole::Data,
            "data_cold" => NodeRole::DataCold,
            "data_content" => NodeRole::DataContent,
            "data_frozen" => NodeRole::DataFrozen,
            "data_hot" => NodeRole::DataHot,
            "data_warm" => NodeRole::DataWarm,
            "ingest" => NodeRole::Ingest,
            "ml" => NodeRole::Ml,
            "remote_cluster_client" => NodeRole::RemoteClusterClient,
            "transform" => NodeRole::Transform,
            "voting_only" => NodeRole::VotingOnly,
            "master" => NodeRole::Master,
            "cluster_manager" => NodeRole::ClusterManager,
            _ => NodeRole::Unknown(s),
        })
    }
}

#[doc = "The role assigned to the node."]
pub type NodeRoles = Vec<crate::types::common::NodeRole>;

#[doc = "The information about a shard on a node."]
#[derive(Debug, Clone)]
pub struct NodeShard {
    pub allocation_id: Option<std::collections::BTreeMap<String, crate::types::common::Id>>,
    pub index: crate::types::common::IndexName,
    pub node: Option<crate::types::common::NodeName>,
    pub primary: bool,
    #[doc = "The recovery source information for this shard."]
    pub recovery_source: Option<std::collections::BTreeMap<String, crate::types::common::Id>>,
    pub relocating_node: Option<crate::types::common::NodeId>,
    #[doc = "Indicates if this shard is configured for search operations only"]
    pub search_only: Option<bool>,
    pub shard: i32,
    pub state: crate::types::indices_stats::ShardRoutingState,
    pub unassigned_info: Option<crate::types::cluster_allocation_explain::UnassignedInformation>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeShard {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeShard;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeShard object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_allocation_id: Option<
                    std::collections::BTreeMap<String, crate::types::common::Id>,
                > = None;
                let mut seen_allocation_id = false;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut f_node: Option<crate::types::common::NodeName> = None;
                let mut seen_node = false;
                let mut f_primary: Option<bool> = None;
                let mut f_recovery_source: Option<
                    std::collections::BTreeMap<String, crate::types::common::Id>,
                > = None;
                let mut seen_recovery_source = false;
                let mut f_relocating_node: Option<crate::types::common::NodeId> = None;
                let mut seen_relocating_node = false;
                let mut f_search_only: Option<bool> = None;
                let mut seen_search_only = false;
                let mut f_shard: Option<i32> = None;
                let mut f_state: Option<crate::types::indices_stats::ShardRoutingState> = None;
                let mut f_unassigned_info: Option<
                    crate::types::cluster_allocation_explain::UnassignedInformation,
                > = None;
                let mut seen_unassigned_info = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "allocation_id" => {
                            if seen_allocation_id {
                                return Err(serde::de::Error::duplicate_field("allocation_id"));
                            }
                            seen_allocation_id = true;
                            f_allocation_id = map.next_value::<Option<
                                std::collections::BTreeMap<String, crate::types::common::Id>,
                            >>()?;
                        }
                        "index" => {
                            if f_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            f_index = Some(map.next_value::<crate::types::common::IndexName>()?);
                        }
                        "node" => {
                            if seen_node {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            seen_node = true;
                            f_node = map.next_value::<Option<crate::types::common::NodeName>>()?;
                        }
                        "primary" => {
                            if f_primary.is_some() {
                                return Err(serde::de::Error::duplicate_field("primary"));
                            }
                            f_primary = Some(map.next_value::<bool>()?);
                        }
                        "recovery_source" => {
                            if seen_recovery_source {
                                return Err(serde::de::Error::duplicate_field("recovery_source"));
                            }
                            seen_recovery_source = true;
                            f_recovery_source = map.next_value::<Option<
                                std::collections::BTreeMap<String, crate::types::common::Id>,
                            >>()?;
                        }
                        "relocating_node" => {
                            if seen_relocating_node {
                                return Err(serde::de::Error::duplicate_field("relocating_node"));
                            }
                            seen_relocating_node = true;
                            f_relocating_node =
                                map.next_value::<Option<crate::types::common::NodeId>>()?;
                        }
                        "searchOnly" => {
                            if seen_search_only {
                                return Err(serde::de::Error::duplicate_field("searchOnly"));
                            }
                            seen_search_only = true;
                            f_search_only = map.next_value::<Option<bool>>()?;
                        }
                        "shard" => {
                            if f_shard.is_some() {
                                return Err(serde::de::Error::duplicate_field("shard"));
                            }
                            f_shard = Some(map.next_value::<i32>()?);
                        }
                        "state" => {
                            if f_state.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            f_state = Some(
                                map.next_value::<crate::types::indices_stats::ShardRoutingState>()?,
                            );
                        }
                        "unassigned_info" => {
                            if seen_unassigned_info {
                                return Err(serde::de::Error::duplicate_field("unassigned_info"));
                            }
                            seen_unassigned_info = true;
                            f_unassigned_info = map.next_value::<Option<
                                crate::types::cluster_allocation_explain::UnassignedInformation,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeShard {
                    allocation_id: f_allocation_id,
                    index: f_index.ok_or_else(|| serde::de::Error::missing_field("index"))?,
                    node: f_node,
                    primary: f_primary.ok_or_else(|| serde::de::Error::missing_field("primary"))?,
                    recovery_source: f_recovery_source,
                    relocating_node: f_relocating_node,
                    search_only: f_search_only,
                    shard: f_shard.ok_or_else(|| serde::de::Error::missing_field("shard"))?,
                    state: f_state.ok_or_else(|| serde::de::Error::missing_field("state"))?,
                    unassigned_info: f_unassigned_info,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeShard {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.allocation_id {
            map.serialize_entry("allocation_id", v)?;
        }
        map.serialize_entry("index", &self.index)?;
        if let Some(v) = &self.node {
            map.serialize_entry("node", v)?;
        }
        map.serialize_entry("primary", &self.primary)?;
        if let Some(v) = &self.recovery_source {
            map.serialize_entry("recovery_source", v)?;
        }
        if let Some(v) = &self.relocating_node {
            map.serialize_entry("relocating_node", v)?;
        }
        if let Some(v) = &self.search_only {
            map.serialize_entry("searchOnly", v)?;
        }
        map.serialize_entry("shard", &self.shard)?;
        map.serialize_entry("state", &self.state)?;
        if let Some(v) = &self.unassigned_info {
            map.serialize_entry("unassigned_info", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The statistics about node operations."]
#[derive(Debug, Clone)]
pub struct NodeStatistics {
    #[doc = "The number of nodes that rejected the request or failed to respond. If this value is not 0, then a reason for the rejection or failure is included in the response."]
    pub failed: i64,
    pub failures: Option<Vec<crate::types::common::ErrorCause>>,
    #[doc = "The number of nodes that responded successfully to the request."]
    pub successful: i64,
    #[doc = "The total number of nodes selected by the request."]
    pub total: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeStatistics {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeStatistics;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeStatistics object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_failed: Option<i64> = None;
                let mut f_failures: Option<Vec<crate::types::common::ErrorCause>> = None;
                let mut seen_failures = false;
                let mut f_successful: Option<i64> = None;
                let mut f_total: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "failed" => {
                            if f_failed.is_some() {
                                return Err(serde::de::Error::duplicate_field("failed"));
                            }
                            f_failed = Some(map.next_value::<i64>()?);
                        }
                        "failures" => {
                            if seen_failures {
                                return Err(serde::de::Error::duplicate_field("failures"));
                            }
                            seen_failures = true;
                            f_failures =
                                map.next_value::<Option<Vec<crate::types::common::ErrorCause>>>()?;
                        }
                        "successful" => {
                            if f_successful.is_some() {
                                return Err(serde::de::Error::duplicate_field("successful"));
                            }
                            f_successful = Some(map.next_value::<i64>()?);
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeStatistics {
                    failed: f_failed.ok_or_else(|| serde::de::Error::missing_field("failed"))?,
                    failures: f_failures,
                    successful: f_successful
                        .ok_or_else(|| serde::de::Error::missing_field("successful"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeStatistics {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("failed", &self.failed)?;
        if let Some(v) = &self.failures {
            map.serialize_entry("failures", v)?;
        }
        map.serialize_entry("successful", &self.successful)?;
        map.serialize_entry("total", &self.total)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The version information for OpenSearch."]
#[derive(Debug, Clone)]
pub struct OpenSearchVersionInfo {
    #[doc = "When the version was built."]
    pub build_date: crate::types::common::DateTime,
    #[doc = "The flavor of the build."]
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
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for OpenSearchVersionInfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = OpenSearchVersionInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a OpenSearchVersionInfo object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_build_date: Option<crate::types::common::DateTime> = None;
                let mut f_build_flavor: Option<String> = None;
                let mut seen_build_flavor = false;
                let mut f_build_hash: Option<String> = None;
                let mut f_build_snapshot: Option<bool> = None;
                let mut f_build_type: Option<String> = None;
                let mut f_distribution: Option<String> = None;
                let mut f_lucene_version: Option<crate::types::common::VersionString> = None;
                let mut f_minimum_index_compatibility_version: Option<
                    crate::types::common::VersionString,
                > = None;
                let mut f_minimum_wire_compatibility_version: Option<
                    crate::types::common::VersionString,
                > = None;
                let mut f_number: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "build_date" => {
                            if f_build_date.is_some() {
                                return Err(serde::de::Error::duplicate_field("build_date"));
                            }
                            f_build_date =
                                Some(map.next_value::<crate::types::common::DateTime>()?);
                        }
                        "build_flavor" => {
                            if seen_build_flavor {
                                return Err(serde::de::Error::duplicate_field("build_flavor"));
                            }
                            seen_build_flavor = true;
                            f_build_flavor = map.next_value::<Option<String>>()?;
                        }
                        "build_hash" => {
                            if f_build_hash.is_some() {
                                return Err(serde::de::Error::duplicate_field("build_hash"));
                            }
                            f_build_hash = Some(map.next_value::<String>()?);
                        }
                        "build_snapshot" => {
                            if f_build_snapshot.is_some() {
                                return Err(serde::de::Error::duplicate_field("build_snapshot"));
                            }
                            f_build_snapshot = Some(map.next_value::<bool>()?);
                        }
                        "build_type" => {
                            if f_build_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("build_type"));
                            }
                            f_build_type = Some(map.next_value::<String>()?);
                        }
                        "distribution" => {
                            if f_distribution.is_some() {
                                return Err(serde::de::Error::duplicate_field("distribution"));
                            }
                            f_distribution = Some(map.next_value::<String>()?);
                        }
                        "lucene_version" => {
                            if f_lucene_version.is_some() {
                                return Err(serde::de::Error::duplicate_field("lucene_version"));
                            }
                            f_lucene_version =
                                Some(map.next_value::<crate::types::common::VersionString>()?);
                        }
                        "minimum_index_compatibility_version" => {
                            if f_minimum_index_compatibility_version.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "minimum_index_compatibility_version",
                                ));
                            }
                            f_minimum_index_compatibility_version =
                                Some(map.next_value::<crate::types::common::VersionString>()?);
                        }
                        "minimum_wire_compatibility_version" => {
                            if f_minimum_wire_compatibility_version.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "minimum_wire_compatibility_version",
                                ));
                            }
                            f_minimum_wire_compatibility_version =
                                Some(map.next_value::<crate::types::common::VersionString>()?);
                        }
                        "number" => {
                            if f_number.is_some() {
                                return Err(serde::de::Error::duplicate_field("number"));
                            }
                            f_number = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(OpenSearchVersionInfo {
                    build_date: f_build_date
                        .ok_or_else(|| serde::de::Error::missing_field("build_date"))?,
                    build_flavor: f_build_flavor,
                    build_hash: f_build_hash
                        .ok_or_else(|| serde::de::Error::missing_field("build_hash"))?,
                    build_snapshot: f_build_snapshot
                        .ok_or_else(|| serde::de::Error::missing_field("build_snapshot"))?,
                    build_type: f_build_type
                        .ok_or_else(|| serde::de::Error::missing_field("build_type"))?,
                    distribution: f_distribution
                        .ok_or_else(|| serde::de::Error::missing_field("distribution"))?,
                    lucene_version: f_lucene_version
                        .ok_or_else(|| serde::de::Error::missing_field("lucene_version"))?,
                    minimum_index_compatibility_version: f_minimum_index_compatibility_version
                        .ok_or_else(|| {
                            serde::de::Error::missing_field("minimum_index_compatibility_version")
                        })?,
                    minimum_wire_compatibility_version: f_minimum_wire_compatibility_version
                        .ok_or_else(|| {
                            serde::de::Error::missing_field("minimum_wire_compatibility_version")
                        })?,
                    number: f_number.ok_or_else(|| serde::de::Error::missing_field("number"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for OpenSearchVersionInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("build_date", &self.build_date)?;
        if let Some(v) = &self.build_flavor {
            map.serialize_entry("build_flavor", v)?;
        }
        map.serialize_entry("build_hash", &self.build_hash)?;
        map.serialize_entry("build_snapshot", &self.build_snapshot)?;
        map.serialize_entry("build_type", &self.build_type)?;
        map.serialize_entry("distribution", &self.distribution)?;
        map.serialize_entry("lucene_version", &self.lucene_version)?;
        map.serialize_entry(
            "minimum_index_compatibility_version",
            &self.minimum_index_compatibility_version,
        )?;
        map.serialize_entry(
            "minimum_wire_compatibility_version",
            &self.minimum_wire_compatibility_version,
        )?;
        map.serialize_entry("number", &self.number)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The percentage value as a number."]
pub type PercentageNumber = f64;

#[doc = "The percentage value as a string."]
pub type PercentageString = String;

#[doc = "The time taken by different phases of the search."]
#[derive(Debug, Clone)]
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
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PhaseTook {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PhaseTook;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PhaseTook object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_can_match: Option<i64> = None;
                let mut f_dfs_pre_query: Option<i64> = None;
                let mut f_dfs_query: Option<i64> = None;
                let mut f_expand: Option<i64> = None;
                let mut f_fetch: Option<i64> = None;
                let mut f_query: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "can_match" => {
                            if f_can_match.is_some() {
                                return Err(serde::de::Error::duplicate_field("can_match"));
                            }
                            f_can_match = Some(map.next_value::<i64>()?);
                        }
                        "dfs_pre_query" => {
                            if f_dfs_pre_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("dfs_pre_query"));
                            }
                            f_dfs_pre_query = Some(map.next_value::<i64>()?);
                        }
                        "dfs_query" => {
                            if f_dfs_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("dfs_query"));
                            }
                            f_dfs_query = Some(map.next_value::<i64>()?);
                        }
                        "expand" => {
                            if f_expand.is_some() {
                                return Err(serde::de::Error::duplicate_field("expand"));
                            }
                            f_expand = Some(map.next_value::<i64>()?);
                        }
                        "fetch" => {
                            if f_fetch.is_some() {
                                return Err(serde::de::Error::duplicate_field("fetch"));
                            }
                            f_fetch = Some(map.next_value::<i64>()?);
                        }
                        "query" => {
                            if f_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            f_query = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PhaseTook {
                    can_match: f_can_match
                        .ok_or_else(|| serde::de::Error::missing_field("can_match"))?,
                    dfs_pre_query: f_dfs_pre_query
                        .ok_or_else(|| serde::de::Error::missing_field("dfs_pre_query"))?,
                    dfs_query: f_dfs_query
                        .ok_or_else(|| serde::de::Error::missing_field("dfs_query"))?,
                    expand: f_expand.ok_or_else(|| serde::de::Error::missing_field("expand"))?,
                    fetch: f_fetch.ok_or_else(|| serde::de::Error::missing_field("fetch"))?,
                    query: f_query.ok_or_else(|| serde::de::Error::missing_field("query"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PhaseTook {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("can_match", &self.can_match)?;
        map.serialize_entry("dfs_pre_query", &self.dfs_pre_query)?;
        map.serialize_entry("dfs_query", &self.dfs_query)?;
        map.serialize_entry("expand", &self.expand)?;
        map.serialize_entry("fetch", &self.fetch)?;
        map.serialize_entry("query", &self.query)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type PipelineName = String;

#[derive(Debug, Clone)]
pub struct PluginStats {
    pub classname: String,
    pub custom_foldername: Option<String>,
    pub description: String,
    pub extended_plugins: Vec<String>,
    pub has_native_controller: bool,
    pub java_version: crate::types::common::VersionString,
    pub licensed: Option<bool>,
    pub name: crate::types::common::Name,
    pub opensearch_version: crate::types::common::VersionString,
    pub optional_extended_plugins: Option<Vec<String>>,
    pub version: crate::types::common::VersionString,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PluginStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PluginStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PluginStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_classname: Option<String> = None;
                let mut f_custom_foldername: Option<String> = None;
                let mut seen_custom_foldername = false;
                let mut f_description: Option<String> = None;
                let mut f_extended_plugins: Option<Vec<String>> = None;
                let mut f_has_native_controller: Option<bool> = None;
                let mut f_java_version: Option<crate::types::common::VersionString> = None;
                let mut f_licensed: Option<bool> = None;
                let mut seen_licensed = false;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut f_opensearch_version: Option<crate::types::common::VersionString> = None;
                let mut f_optional_extended_plugins: Option<Vec<String>> = None;
                let mut seen_optional_extended_plugins = false;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "classname" => {
                            if f_classname.is_some() {
                                return Err(serde::de::Error::duplicate_field("classname"));
                            }
                            f_classname = Some(map.next_value::<String>()?);
                        }
                        "custom_foldername" => {
                            if seen_custom_foldername {
                                return Err(serde::de::Error::duplicate_field("custom_foldername"));
                            }
                            seen_custom_foldername = true;
                            f_custom_foldername = map.next_value::<Option<String>>()?;
                        }
                        "description" => {
                            if f_description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            f_description = Some(map.next_value::<String>()?);
                        }
                        "extended_plugins" => {
                            if f_extended_plugins.is_some() {
                                return Err(serde::de::Error::duplicate_field("extended_plugins"));
                            }
                            f_extended_plugins = Some(map.next_value::<Vec<String>>()?);
                        }
                        "has_native_controller" => {
                            if f_has_native_controller.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "has_native_controller",
                                ));
                            }
                            f_has_native_controller = Some(map.next_value::<bool>()?);
                        }
                        "java_version" => {
                            if f_java_version.is_some() {
                                return Err(serde::de::Error::duplicate_field("java_version"));
                            }
                            f_java_version =
                                Some(map.next_value::<crate::types::common::VersionString>()?);
                        }
                        "licensed" => {
                            if seen_licensed {
                                return Err(serde::de::Error::duplicate_field("licensed"));
                            }
                            seen_licensed = true;
                            f_licensed = map.next_value::<Option<bool>>()?;
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        "opensearch_version" => {
                            if f_opensearch_version.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "opensearch_version",
                                ));
                            }
                            f_opensearch_version =
                                Some(map.next_value::<crate::types::common::VersionString>()?);
                        }
                        "optional_extended_plugins" => {
                            if seen_optional_extended_plugins {
                                return Err(serde::de::Error::duplicate_field(
                                    "optional_extended_plugins",
                                ));
                            }
                            seen_optional_extended_plugins = true;
                            f_optional_extended_plugins =
                                map.next_value::<Option<Vec<String>>>()?;
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
                Ok(PluginStats {
                    classname: f_classname
                        .ok_or_else(|| serde::de::Error::missing_field("classname"))?,
                    custom_foldername: f_custom_foldername,
                    description: f_description
                        .ok_or_else(|| serde::de::Error::missing_field("description"))?,
                    extended_plugins: f_extended_plugins
                        .ok_or_else(|| serde::de::Error::missing_field("extended_plugins"))?,
                    has_native_controller: f_has_native_controller
                        .ok_or_else(|| serde::de::Error::missing_field("has_native_controller"))?,
                    java_version: f_java_version
                        .ok_or_else(|| serde::de::Error::missing_field("java_version"))?,
                    licensed: f_licensed,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    opensearch_version: f_opensearch_version
                        .ok_or_else(|| serde::de::Error::missing_field("opensearch_version"))?,
                    optional_extended_plugins: f_optional_extended_plugins,
                    version: f_version.ok_or_else(|| serde::de::Error::missing_field("version"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PluginStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("classname", &self.classname)?;
        if let Some(v) = &self.custom_foldername {
            map.serialize_entry("custom_foldername", v)?;
        }
        map.serialize_entry("description", &self.description)?;
        map.serialize_entry("extended_plugins", &self.extended_plugins)?;
        map.serialize_entry("has_native_controller", &self.has_native_controller)?;
        map.serialize_entry("java_version", &self.java_version)?;
        if let Some(v) = &self.licensed {
            map.serialize_entry("licensed", v)?;
        }
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("opensearch_version", &self.opensearch_version)?;
        if let Some(v) = &self.optional_extended_plugins {
            map.serialize_entry("optional_extended_plugins", v)?;
        }
        map.serialize_entry("version", &self.version)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The statistics about query cache usage."]
#[derive(Debug, Clone)]
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
    pub memory_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount, in bytes, of memory used for the query cache across all shards assigned to the selected nodes."]
    pub memory_size_in_bytes: crate::types::common::ByteCount,
    #[doc = "The total number of query cache misses across all shards assigned to the selected nodes."]
    pub miss_count: i64,
    #[doc = "The total number of hits and misses stored in the query cache across all shards assigned to the selected nodes."]
    pub total_count: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for QueryCacheStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = QueryCacheStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a QueryCacheStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cache_count: Option<i64> = None;
                let mut f_cache_size: Option<i64> = None;
                let mut f_evictions: Option<i64> = None;
                let mut f_hit_count: Option<i64> = None;
                let mut f_memory_size: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_memory_size = false;
                let mut f_memory_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_miss_count: Option<i64> = None;
                let mut f_total_count: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cache_count" => {
                            if f_cache_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("cache_count"));
                            }
                            f_cache_count = Some(map.next_value::<i64>()?);
                        }
                        "cache_size" => {
                            if f_cache_size.is_some() {
                                return Err(serde::de::Error::duplicate_field("cache_size"));
                            }
                            f_cache_size = Some(map.next_value::<i64>()?);
                        }
                        "evictions" => {
                            if f_evictions.is_some() {
                                return Err(serde::de::Error::duplicate_field("evictions"));
                            }
                            f_evictions = Some(map.next_value::<i64>()?);
                        }
                        "hit_count" => {
                            if f_hit_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("hit_count"));
                            }
                            f_hit_count = Some(map.next_value::<i64>()?);
                        }
                        "memory_size" => {
                            if seen_memory_size {
                                return Err(serde::de::Error::duplicate_field("memory_size"));
                            }
                            seen_memory_size = true;
                            f_memory_size = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "memory_size_in_bytes" => {
                            if f_memory_size_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "memory_size_in_bytes",
                                ));
                            }
                            f_memory_size_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "miss_count" => {
                            if f_miss_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("miss_count"));
                            }
                            f_miss_count = Some(map.next_value::<i64>()?);
                        }
                        "total_count" => {
                            if f_total_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("total_count"));
                            }
                            f_total_count = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(QueryCacheStats {
                    cache_count: f_cache_count
                        .ok_or_else(|| serde::de::Error::missing_field("cache_count"))?,
                    cache_size: f_cache_size
                        .ok_or_else(|| serde::de::Error::missing_field("cache_size"))?,
                    evictions: f_evictions
                        .ok_or_else(|| serde::de::Error::missing_field("evictions"))?,
                    hit_count: f_hit_count
                        .ok_or_else(|| serde::de::Error::missing_field("hit_count"))?,
                    memory_size: f_memory_size,
                    memory_size_in_bytes: f_memory_size_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("memory_size_in_bytes"))?,
                    miss_count: f_miss_count
                        .ok_or_else(|| serde::de::Error::missing_field("miss_count"))?,
                    total_count: f_total_count
                        .ok_or_else(|| serde::de::Error::missing_field("total_count"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for QueryCacheStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("cache_count", &self.cache_count)?;
        map.serialize_entry("cache_size", &self.cache_size)?;
        map.serialize_entry("evictions", &self.evictions)?;
        map.serialize_entry("hit_count", &self.hit_count)?;
        if let Some(v) = &self.memory_size {
            map.serialize_entry("memory_size", v)?;
        }
        map.serialize_entry("memory_size_in_bytes", &self.memory_size_in_bytes)?;
        map.serialize_entry("miss_count", &self.miss_count)?;
        map.serialize_entry("total_count", &self.total_count)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The statistics about recovery operations."]
#[derive(Debug, Clone)]
pub struct RecoveryStats {
    #[doc = "The number of ongoing recoveries for which this node is the source."]
    pub current_as_source: i64,
    #[doc = "The number of ongoing recoveries for which this node is the target."]
    pub current_as_target: i64,
    #[doc = "The total amount of time recovery operations have been throttled."]
    pub throttle_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time recovery operations have been throttled in milliseconds."]
    pub throttle_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RecoveryStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RecoveryStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RecoveryStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_current_as_source: Option<i64> = None;
                let mut f_current_as_target: Option<i64> = None;
                let mut f_throttle_time: Option<crate::types::common::Duration> = None;
                let mut seen_throttle_time = false;
                let mut f_throttle_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "current_as_source" => {
                            if f_current_as_source.is_some() {
                                return Err(serde::de::Error::duplicate_field("current_as_source"));
                            }
                            f_current_as_source = Some(map.next_value::<i64>()?);
                        }
                        "current_as_target" => {
                            if f_current_as_target.is_some() {
                                return Err(serde::de::Error::duplicate_field("current_as_target"));
                            }
                            f_current_as_target = Some(map.next_value::<i64>()?);
                        }
                        "throttle_time" => {
                            if seen_throttle_time {
                                return Err(serde::de::Error::duplicate_field("throttle_time"));
                            }
                            seen_throttle_time = true;
                            f_throttle_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "throttle_time_in_millis" => {
                            if f_throttle_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "throttle_time_in_millis",
                                ));
                            }
                            f_throttle_time_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RecoveryStats {
                    current_as_source: f_current_as_source
                        .ok_or_else(|| serde::de::Error::missing_field("current_as_source"))?,
                    current_as_target: f_current_as_target
                        .ok_or_else(|| serde::de::Error::missing_field("current_as_target"))?,
                    throttle_time: f_throttle_time,
                    throttle_time_in_millis: f_throttle_time_in_millis.ok_or_else(|| {
                        serde::de::Error::missing_field("throttle_time_in_millis")
                    })?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RecoveryStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("current_as_source", &self.current_as_source)?;
        map.serialize_entry("current_as_target", &self.current_as_target)?;
        if let Some(v) = &self.throttle_time {
            map.serialize_entry("throttle_time", v)?;
        }
        map.serialize_entry("throttle_time_in_millis", &self.throttle_time_in_millis)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The statistics about refresh operations."]
#[derive(Debug, Clone)]
pub struct RefreshStats {
    #[doc = "The total number of external refresh operations."]
    pub external_total: i64,
    #[doc = "The total amount of time spent on external refresh operations."]
    pub external_total_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time spent on external refresh operations in milliseconds."]
    pub external_total_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The number of refresh listeners."]
    pub listeners: i64,
    #[doc = "The total number of refresh operations."]
    pub total: i64,
    #[doc = "The total amount of time spent on refresh operations."]
    pub total_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time spent on refresh operations in milliseconds."]
    pub total_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RefreshStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RefreshStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RefreshStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_external_total: Option<i64> = None;
                let mut f_external_total_time: Option<crate::types::common::Duration> = None;
                let mut seen_external_total_time = false;
                let mut f_external_total_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_listeners: Option<i64> = None;
                let mut f_total: Option<i64> = None;
                let mut f_total_time: Option<crate::types::common::Duration> = None;
                let mut seen_total_time = false;
                let mut f_total_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "external_total" => {
                            if f_external_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("external_total"));
                            }
                            f_external_total = Some(map.next_value::<i64>()?);
                        }
                        "external_total_time" => {
                            if seen_external_total_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "external_total_time",
                                ));
                            }
                            seen_external_total_time = true;
                            f_external_total_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "external_total_time_in_millis" => {
                            if f_external_total_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "external_total_time_in_millis",
                                ));
                            }
                            f_external_total_time_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "listeners" => {
                            if f_listeners.is_some() {
                                return Err(serde::de::Error::duplicate_field("listeners"));
                            }
                            f_listeners = Some(map.next_value::<i64>()?);
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(map.next_value::<i64>()?);
                        }
                        "total_time" => {
                            if seen_total_time {
                                return Err(serde::de::Error::duplicate_field("total_time"));
                            }
                            seen_total_time = true;
                            f_total_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "total_time_in_millis" => {
                            if f_total_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_time_in_millis",
                                ));
                            }
                            f_total_time_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RefreshStats {
                    external_total: f_external_total
                        .ok_or_else(|| serde::de::Error::missing_field("external_total"))?,
                    external_total_time: f_external_total_time,
                    external_total_time_in_millis: f_external_total_time_in_millis.ok_or_else(
                        || serde::de::Error::missing_field("external_total_time_in_millis"),
                    )?,
                    listeners: f_listeners
                        .ok_or_else(|| serde::de::Error::missing_field("listeners"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    total_time: f_total_time,
                    total_time_in_millis: f_total_time_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("total_time_in_millis"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RefreshStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("external_total", &self.external_total)?;
        if let Some(v) = &self.external_total_time {
            map.serialize_entry("external_total_time", v)?;
        }
        map.serialize_entry(
            "external_total_time_in_millis",
            &self.external_total_time_in_millis,
        )?;
        map.serialize_entry("listeners", &self.listeners)?;
        map.serialize_entry("total", &self.total)?;
        if let Some(v) = &self.total_time {
            map.serialize_entry("total_time", v)?;
        }
        map.serialize_entry("total_time_in_millis", &self.total_time_in_millis)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The name of a relation in a join field."]
pub type RelationName = String;

#[doc = "Any statistics related to downloads to the remote segment store."]
#[derive(Debug, Clone)]
pub struct RemoteStoreDownloadStats {
    #[doc = "The total amount of data downloaded from the remote segment store."]
    pub total_download_size: crate::types::common::RemoteStoreUploadDownloadStats,
    #[doc = "The total amount of time spent on downloads from the remote segment store."]
    pub total_time_spent: Option<crate::types::common::Duration>,
    #[doc = "The total duration, in milliseconds, spent on downloads from the remote segment store."]
    pub total_time_spent_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RemoteStoreDownloadStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RemoteStoreDownloadStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RemoteStoreDownloadStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_total_download_size: Option<
                    crate::types::common::RemoteStoreUploadDownloadStats,
                > = None;
                let mut f_total_time_spent: Option<crate::types::common::Duration> = None;
                let mut seen_total_time_spent = false;
                let mut f_total_time_spent_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "total_download_size" => {
                            if f_total_download_size.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_download_size",
                                ));
                            }
                            f_total_download_size = Some(map.next_value::<crate::types::common::RemoteStoreUploadDownloadStats>()?);
                        }
                        "total_time_spent" => {
                            if seen_total_time_spent {
                                return Err(serde::de::Error::duplicate_field("total_time_spent"));
                            }
                            seen_total_time_spent = true;
                            f_total_time_spent =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "total_time_spent_in_millis" => {
                            if f_total_time_spent_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_time_spent_in_millis",
                                ));
                            }
                            f_total_time_spent_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RemoteStoreDownloadStats {
                    total_download_size: f_total_download_size
                        .ok_or_else(|| serde::de::Error::missing_field("total_download_size"))?,
                    total_time_spent: f_total_time_spent,
                    total_time_spent_in_millis: f_total_time_spent_in_millis.ok_or_else(|| {
                        serde::de::Error::missing_field("total_time_spent_in_millis")
                    })?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RemoteStoreDownloadStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("total_download_size", &self.total_download_size)?;
        if let Some(v) = &self.total_time_spent {
            map.serialize_entry("total_time_spent", v)?;
        }
        map.serialize_entry(
            "total_time_spent_in_millis",
            &self.total_time_spent_in_millis,
        )?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The statistics related to remote segment store operations."]
#[derive(Debug, Clone)]
pub struct RemoteStoreStats {
    pub download: crate::types::common::RemoteStoreDownloadStats,
    pub upload: crate::types::common::RemoteStoreUploadStats,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RemoteStoreStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RemoteStoreStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RemoteStoreStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_download: Option<crate::types::common::RemoteStoreDownloadStats> = None;
                let mut f_upload: Option<crate::types::common::RemoteStoreUploadStats> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "download" => {
                            if f_download.is_some() {
                                return Err(serde::de::Error::duplicate_field("download"));
                            }
                            f_download = Some(
                                map.next_value::<crate::types::common::RemoteStoreDownloadStats>()?,
                            );
                        }
                        "upload" => {
                            if f_upload.is_some() {
                                return Err(serde::de::Error::duplicate_field("upload"));
                            }
                            f_upload = Some(
                                map.next_value::<crate::types::common::RemoteStoreUploadStats>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RemoteStoreStats {
                    download: f_download
                        .ok_or_else(|| serde::de::Error::missing_field("download"))?,
                    upload: f_upload.ok_or_else(|| serde::de::Error::missing_field("upload"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RemoteStoreStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("download", &self.download)?;
        map.serialize_entry("upload", &self.upload)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The statistics related to remote translog operations."]
#[derive(Debug, Clone)]
pub struct RemoteStoreTranslogStats {
    pub upload: crate::types::common::RemoteStoreTranslogUploadStats,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RemoteStoreTranslogStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RemoteStoreTranslogStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RemoteStoreTranslogStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_upload: Option<crate::types::common::RemoteStoreTranslogUploadStats> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "upload" => {
                            if f_upload.is_some() {
                                return Err(serde::de::Error::duplicate_field("upload"));
                            }
                            f_upload = Some(map.next_value::<crate::types::common::RemoteStoreTranslogUploadStats>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RemoteStoreTranslogStats {
                    upload: f_upload.ok_or_else(|| serde::de::Error::missing_field("upload"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RemoteStoreTranslogStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("upload", &self.upload)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The statistics related to uploads to the remote translog store."]
#[derive(Debug, Clone)]
pub struct RemoteStoreTranslogUploadStats {
    pub total_upload_size: crate::types::common::RemoteStoreTranslogUploadTotalUploadSizeStats,
    pub total_uploads: crate::types::common::RemoteStoreTranslogUploadTotalUploadsStats,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RemoteStoreTranslogUploadStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RemoteStoreTranslogUploadStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RemoteStoreTranslogUploadStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_total_upload_size: Option<
                    crate::types::common::RemoteStoreTranslogUploadTotalUploadSizeStats,
                > = None;
                let mut f_total_uploads: Option<
                    crate::types::common::RemoteStoreTranslogUploadTotalUploadsStats,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "total_upload_size" => {
                            if f_total_upload_size.is_some() {
                                return Err(serde::de::Error::duplicate_field("total_upload_size"));
                            }
                            f_total_upload_size = Some(map.next_value::<crate::types::common::RemoteStoreTranslogUploadTotalUploadSizeStats>()?);
                        }
                        "total_uploads" => {
                            if f_total_uploads.is_some() {
                                return Err(serde::de::Error::duplicate_field("total_uploads"));
                            }
                            f_total_uploads = Some(map.next_value::<crate::types::common::RemoteStoreTranslogUploadTotalUploadsStats>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RemoteStoreTranslogUploadStats {
                    total_upload_size: f_total_upload_size
                        .ok_or_else(|| serde::de::Error::missing_field("total_upload_size"))?,
                    total_uploads: f_total_uploads
                        .ok_or_else(|| serde::de::Error::missing_field("total_uploads"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RemoteStoreTranslogUploadStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("total_upload_size", &self.total_upload_size)?;
        map.serialize_entry("total_uploads", &self.total_uploads)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The total amount of data uploaded to the remote translog store."]
#[derive(Debug, Clone)]
pub struct RemoteStoreTranslogUploadTotalUploadSizeStats {
    #[doc = "The number of bytes that failed to upload to the remote translog store."]
    pub failed: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The number of bytes that failed to upload to the remote translog store."]
    pub failed_bytes: crate::types::common::ByteCount,
    #[doc = "The number of bytes successfully uploaded to the remote translog store."]
    pub started: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The number of bytes successfully uploaded to the remote translog store."]
    pub started_bytes: crate::types::common::ByteCount,
    #[doc = "The number of bytes successfully uploaded to the remote translog store."]
    pub succeeded: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The number of bytes successfully uploaded to the remote translog store."]
    pub succeeded_bytes: crate::types::common::ByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RemoteStoreTranslogUploadTotalUploadSizeStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RemoteStoreTranslogUploadTotalUploadSizeStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RemoteStoreTranslogUploadTotalUploadSizeStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_failed: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_failed = false;
                let mut f_failed_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_started: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_started = false;
                let mut f_started_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_succeeded: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_succeeded = false;
                let mut f_succeeded_bytes: Option<crate::types::common::ByteCount> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "failed" => {
                            if seen_failed {
                                return Err(serde::de::Error::duplicate_field("failed"));
                            }
                            seen_failed = true;
                            f_failed = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "failed_bytes" => {
                            if f_failed_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("failed_bytes"));
                            }
                            f_failed_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "started" => {
                            if seen_started {
                                return Err(serde::de::Error::duplicate_field("started"));
                            }
                            seen_started = true;
                            f_started = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "started_bytes" => {
                            if f_started_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("started_bytes"));
                            }
                            f_started_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "succeeded" => {
                            if seen_succeeded {
                                return Err(serde::de::Error::duplicate_field("succeeded"));
                            }
                            seen_succeeded = true;
                            f_succeeded = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "succeeded_bytes" => {
                            if f_succeeded_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("succeeded_bytes"));
                            }
                            f_succeeded_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RemoteStoreTranslogUploadTotalUploadSizeStats {
                    failed: f_failed,
                    failed_bytes: f_failed_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("failed_bytes"))?,
                    started: f_started,
                    started_bytes: f_started_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("started_bytes"))?,
                    succeeded: f_succeeded,
                    succeeded_bytes: f_succeeded_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("succeeded_bytes"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RemoteStoreTranslogUploadTotalUploadSizeStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.failed {
            map.serialize_entry("failed", v)?;
        }
        map.serialize_entry("failed_bytes", &self.failed_bytes)?;
        if let Some(v) = &self.started {
            map.serialize_entry("started", v)?;
        }
        map.serialize_entry("started_bytes", &self.started_bytes)?;
        if let Some(v) = &self.succeeded {
            map.serialize_entry("succeeded", v)?;
        }
        map.serialize_entry("succeeded_bytes", &self.succeeded_bytes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The number of syncs to the remote translog store."]
#[derive(Debug, Clone)]
pub struct RemoteStoreTranslogUploadTotalUploadsStats {
    #[doc = "The number of failed upload syncs to the remote translog store."]
    pub failed: i64,
    #[doc = "The number of upload syncs to the remote translog store that have started."]
    pub started: i64,
    #[doc = "The number of successful upload syncs to the remote translog store."]
    pub succeeded: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RemoteStoreTranslogUploadTotalUploadsStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RemoteStoreTranslogUploadTotalUploadsStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RemoteStoreTranslogUploadTotalUploadsStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_failed: Option<i64> = None;
                let mut f_started: Option<i64> = None;
                let mut f_succeeded: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "failed" => {
                            if f_failed.is_some() {
                                return Err(serde::de::Error::duplicate_field("failed"));
                            }
                            f_failed = Some(map.next_value::<i64>()?);
                        }
                        "started" => {
                            if f_started.is_some() {
                                return Err(serde::de::Error::duplicate_field("started"));
                            }
                            f_started = Some(map.next_value::<i64>()?);
                        }
                        "succeeded" => {
                            if f_succeeded.is_some() {
                                return Err(serde::de::Error::duplicate_field("succeeded"));
                            }
                            f_succeeded = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RemoteStoreTranslogUploadTotalUploadsStats {
                    failed: f_failed.ok_or_else(|| serde::de::Error::missing_field("failed"))?,
                    started: f_started.ok_or_else(|| serde::de::Error::missing_field("started"))?,
                    succeeded: f_succeeded
                        .ok_or_else(|| serde::de::Error::missing_field("succeeded"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RemoteStoreTranslogUploadTotalUploadsStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("failed", &self.failed)?;
        map.serialize_entry("started", &self.started)?;
        map.serialize_entry("succeeded", &self.succeeded)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The amount of data, in bytes, uploaded or downloaded to/from the remote segment store."]
#[derive(Debug, Clone)]
pub struct RemoteStoreUploadDownloadStats {
    #[doc = "The number of bytes that failed to upload to/from the remote segment store."]
    pub failed: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The number of bytes that failed to upload to/from the remote segment store."]
    pub failed_bytes: crate::types::common::ByteCount,
    #[doc = "The number of bytes to upload/download to/from the remote segment store after the upload/download has started."]
    pub started: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The number of bytes to upload/download to/from the remote segment store after the upload/download has started."]
    pub started_bytes: crate::types::common::ByteCount,
    #[doc = "The number of bytes successfully uploaded/downloaded to/from the remote segment store."]
    pub succeeded: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The number of bytes successfully uploaded/downloaded to/from the remote segment store."]
    pub succeeded_bytes: crate::types::common::ByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RemoteStoreUploadDownloadStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RemoteStoreUploadDownloadStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RemoteStoreUploadDownloadStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_failed: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_failed = false;
                let mut f_failed_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_started: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_started = false;
                let mut f_started_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_succeeded: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_succeeded = false;
                let mut f_succeeded_bytes: Option<crate::types::common::ByteCount> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "failed" => {
                            if seen_failed {
                                return Err(serde::de::Error::duplicate_field("failed"));
                            }
                            seen_failed = true;
                            f_failed = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "failed_bytes" => {
                            if f_failed_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("failed_bytes"));
                            }
                            f_failed_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "started" => {
                            if seen_started {
                                return Err(serde::de::Error::duplicate_field("started"));
                            }
                            seen_started = true;
                            f_started = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "started_bytes" => {
                            if f_started_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("started_bytes"));
                            }
                            f_started_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "succeeded" => {
                            if seen_succeeded {
                                return Err(serde::de::Error::duplicate_field("succeeded"));
                            }
                            seen_succeeded = true;
                            f_succeeded = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "succeeded_bytes" => {
                            if f_succeeded_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("succeeded_bytes"));
                            }
                            f_succeeded_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RemoteStoreUploadDownloadStats {
                    failed: f_failed,
                    failed_bytes: f_failed_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("failed_bytes"))?,
                    started: f_started,
                    started_bytes: f_started_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("started_bytes"))?,
                    succeeded: f_succeeded,
                    succeeded_bytes: f_succeeded_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("succeeded_bytes"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RemoteStoreUploadDownloadStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.failed {
            map.serialize_entry("failed", v)?;
        }
        map.serialize_entry("failed_bytes", &self.failed_bytes)?;
        if let Some(v) = &self.started {
            map.serialize_entry("started", v)?;
        }
        map.serialize_entry("started_bytes", &self.started_bytes)?;
        if let Some(v) = &self.succeeded {
            map.serialize_entry("succeeded", v)?;
        }
        map.serialize_entry("succeeded_bytes", &self.succeeded_bytes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Statistics related to segment store upload backpressure."]
#[derive(Debug, Clone)]
pub struct RemoteStoreUploadPressureStats {
    #[doc = "The total number of requests rejected due to segment store upload backpressure."]
    pub total_rejections: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RemoteStoreUploadPressureStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RemoteStoreUploadPressureStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RemoteStoreUploadPressureStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_total_rejections: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "total_rejections" => {
                            if f_total_rejections.is_some() {
                                return Err(serde::de::Error::duplicate_field("total_rejections"));
                            }
                            f_total_rejections = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RemoteStoreUploadPressureStats {
                    total_rejections: f_total_rejections
                        .ok_or_else(|| serde::de::Error::missing_field("total_rejections"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RemoteStoreUploadPressureStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("total_rejections", &self.total_rejections)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The amount of lag during upload between the remote segment store and the local store."]
#[derive(Debug, Clone)]
pub struct RemoteStoreUploadRefreshSizeLagStats {
    #[doc = "The maximum amount of lag, in bytes, during the upload refresh between the remote segment store and the local store."]
    pub max: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The maximum amount of lag, in bytes, during the upload refresh between the remote segment store and the local store."]
    pub max_bytes: crate::types::common::ByteCount,
    #[doc = "The total number of bytes that lagged during the upload refresh between the remote segment store and the local store."]
    pub total: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total number of bytes that lagged during the upload refresh between the remote segment store and the local store."]
    pub total_bytes: crate::types::common::ByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RemoteStoreUploadRefreshSizeLagStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RemoteStoreUploadRefreshSizeLagStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RemoteStoreUploadRefreshSizeLagStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_max: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_max = false;
                let mut f_max_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_total: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_total = false;
                let mut f_total_bytes: Option<crate::types::common::ByteCount> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "max" => {
                            if seen_max {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            seen_max = true;
                            f_max = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "max_bytes" => {
                            if f_max_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("max_bytes"));
                            }
                            f_max_bytes =
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
                        "total_bytes" => {
                            if f_total_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("total_bytes"));
                            }
                            f_total_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RemoteStoreUploadRefreshSizeLagStats {
                    max: f_max,
                    max_bytes: f_max_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("max_bytes"))?,
                    total: f_total,
                    total_bytes: f_total_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("total_bytes"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RemoteStoreUploadRefreshSizeLagStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.max {
            map.serialize_entry("max", v)?;
        }
        map.serialize_entry("max_bytes", &self.max_bytes)?;
        if let Some(v) = &self.total {
            map.serialize_entry("total", v)?;
        }
        map.serialize_entry("total_bytes", &self.total_bytes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Any statistics related to uploads to the remote segment store."]
#[derive(Debug, Clone)]
pub struct RemoteStoreUploadStats {
    #[doc = "The maximum duration that the remote refresh is behind the local refresh."]
    pub max_refresh_time_lag: Option<crate::types::common::Duration>,
    #[doc = "The maximum duration, in milliseconds, that the remote refresh is behind the local refresh."]
    pub max_refresh_time_lag_in_millis: crate::types::common::DurationValueUnitMillis,
    pub pressure: Option<crate::types::common::RemoteStoreUploadPressureStats>,
    pub refresh_size_lag: crate::types::common::RemoteStoreUploadRefreshSizeLagStats,
    #[doc = "The total amount of time spent on uploads to the remote segment store."]
    pub total_time_spent: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time, in milliseconds, spent on uploads to the remote segment store."]
    pub total_time_spent_in_millis: crate::types::common::DurationValueUnitMillis,
    pub total_upload_size: crate::types::common::RemoteStoreUploadDownloadStats,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RemoteStoreUploadStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RemoteStoreUploadStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RemoteStoreUploadStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_max_refresh_time_lag: Option<crate::types::common::Duration> = None;
                let mut seen_max_refresh_time_lag = false;
                let mut f_max_refresh_time_lag_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_pressure: Option<crate::types::common::RemoteStoreUploadPressureStats> =
                    None;
                let mut seen_pressure = false;
                let mut f_refresh_size_lag: Option<
                    crate::types::common::RemoteStoreUploadRefreshSizeLagStats,
                > = None;
                let mut f_total_time_spent: Option<crate::types::common::Duration> = None;
                let mut seen_total_time_spent = false;
                let mut f_total_time_spent_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_total_upload_size: Option<
                    crate::types::common::RemoteStoreUploadDownloadStats,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "max_refresh_time_lag" => {
                            if seen_max_refresh_time_lag {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_refresh_time_lag",
                                ));
                            }
                            seen_max_refresh_time_lag = true;
                            f_max_refresh_time_lag =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "max_refresh_time_lag_in_millis" => {
                            if f_max_refresh_time_lag_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_refresh_time_lag_in_millis",
                                ));
                            }
                            f_max_refresh_time_lag_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "pressure" => {
                            if seen_pressure {
                                return Err(serde::de::Error::duplicate_field("pressure"));
                            }
                            seen_pressure = true;
                            f_pressure = map.next_value::<Option<crate::types::common::RemoteStoreUploadPressureStats>>()?;
                        }
                        "refresh_size_lag" => {
                            if f_refresh_size_lag.is_some() {
                                return Err(serde::de::Error::duplicate_field("refresh_size_lag"));
                            }
                            f_refresh_size_lag = Some(map.next_value::<crate::types::common::RemoteStoreUploadRefreshSizeLagStats>()?);
                        }
                        "total_time_spent" => {
                            if seen_total_time_spent {
                                return Err(serde::de::Error::duplicate_field("total_time_spent"));
                            }
                            seen_total_time_spent = true;
                            f_total_time_spent =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "total_time_spent_in_millis" => {
                            if f_total_time_spent_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_time_spent_in_millis",
                                ));
                            }
                            f_total_time_spent_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "total_upload_size" => {
                            if f_total_upload_size.is_some() {
                                return Err(serde::de::Error::duplicate_field("total_upload_size"));
                            }
                            f_total_upload_size = Some(map.next_value::<crate::types::common::RemoteStoreUploadDownloadStats>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RemoteStoreUploadStats {
                    max_refresh_time_lag: f_max_refresh_time_lag,
                    max_refresh_time_lag_in_millis: f_max_refresh_time_lag_in_millis.ok_or_else(
                        || serde::de::Error::missing_field("max_refresh_time_lag_in_millis"),
                    )?,
                    pressure: f_pressure,
                    refresh_size_lag: f_refresh_size_lag
                        .ok_or_else(|| serde::de::Error::missing_field("refresh_size_lag"))?,
                    total_time_spent: f_total_time_spent,
                    total_time_spent_in_millis: f_total_time_spent_in_millis.ok_or_else(|| {
                        serde::de::Error::missing_field("total_time_spent_in_millis")
                    })?,
                    total_upload_size: f_total_upload_size
                        .ok_or_else(|| serde::de::Error::missing_field("total_upload_size"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RemoteStoreUploadStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.max_refresh_time_lag {
            map.serialize_entry("max_refresh_time_lag", v)?;
        }
        map.serialize_entry(
            "max_refresh_time_lag_in_millis",
            &self.max_refresh_time_lag_in_millis,
        )?;
        if let Some(v) = &self.pressure {
            map.serialize_entry("pressure", v)?;
        }
        map.serialize_entry("refresh_size_lag", &self.refresh_size_lag)?;
        if let Some(v) = &self.total_time_spent {
            map.serialize_entry("total_time_spent", v)?;
        }
        map.serialize_entry(
            "total_time_spent_in_millis",
            &self.total_time_spent_in_millis,
        )?;
        map.serialize_entry("total_upload_size", &self.total_upload_size)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The statistics about request cache operations."]
#[derive(Debug, Clone)]
pub struct RequestCacheStats {
    #[doc = "The number of request cache evictions."]
    pub evictions: i64,
    #[doc = "The number of request cache hits."]
    pub hit_count: i64,
    #[doc = "The size of the request cache in a human-readable format."]
    pub memory_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The size of the request cache in bytes."]
    pub memory_size_in_bytes: crate::types::common::ByteCount,
    #[doc = "The number of request cache misses."]
    pub miss_count: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RequestCacheStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RequestCacheStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RequestCacheStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_evictions: Option<i64> = None;
                let mut f_hit_count: Option<i64> = None;
                let mut f_memory_size: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_memory_size = false;
                let mut f_memory_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_miss_count: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "evictions" => {
                            if f_evictions.is_some() {
                                return Err(serde::de::Error::duplicate_field("evictions"));
                            }
                            f_evictions = Some(map.next_value::<i64>()?);
                        }
                        "hit_count" => {
                            if f_hit_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("hit_count"));
                            }
                            f_hit_count = Some(map.next_value::<i64>()?);
                        }
                        "memory_size" => {
                            if seen_memory_size {
                                return Err(serde::de::Error::duplicate_field("memory_size"));
                            }
                            seen_memory_size = true;
                            f_memory_size = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "memory_size_in_bytes" => {
                            if f_memory_size_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "memory_size_in_bytes",
                                ));
                            }
                            f_memory_size_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "miss_count" => {
                            if f_miss_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("miss_count"));
                            }
                            f_miss_count = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RequestCacheStats {
                    evictions: f_evictions
                        .ok_or_else(|| serde::de::Error::missing_field("evictions"))?,
                    hit_count: f_hit_count
                        .ok_or_else(|| serde::de::Error::missing_field("hit_count"))?,
                    memory_size: f_memory_size,
                    memory_size_in_bytes: f_memory_size_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("memory_size_in_bytes"))?,
                    miss_count: f_miss_count
                        .ok_or_else(|| serde::de::Error::missing_field("miss_count"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RequestCacheStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("evictions", &self.evictions)?;
        map.serialize_entry("hit_count", &self.hit_count)?;
        if let Some(v) = &self.memory_size {
            map.serialize_entry("memory_size", v)?;
        }
        map.serialize_entry("memory_size_in_bytes", &self.memory_size_in_bytes)?;
        map.serialize_entry("miss_count", &self.miss_count)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The statistics about request operations."]
#[derive(Debug, Clone)]
pub struct RequestStats {
    #[doc = "The number of current request operations."]
    pub current: Option<i64>,
    #[doc = "The total time spent on request operations."]
    pub time: Option<crate::types::common::Duration>,
    #[doc = "The total time spent on request operations in milliseconds."]
    pub time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "The total number of request operations."]
    pub total: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RequestStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RequestStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RequestStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_current: Option<i64> = None;
                let mut seen_current = false;
                let mut f_time: Option<crate::types::common::Duration> = None;
                let mut seen_time = false;
                let mut f_time_in_millis: Option<crate::types::common::DurationValueUnitMillis> =
                    None;
                let mut seen_time_in_millis = false;
                let mut f_total: Option<i64> = None;
                let mut seen_total = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "current" => {
                            if seen_current {
                                return Err(serde::de::Error::duplicate_field("current"));
                            }
                            seen_current = true;
                            f_current = map.next_value::<Option<i64>>()?;
                        }
                        "time" => {
                            if seen_time {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            seen_time = true;
                            f_time = map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "time_in_millis" => {
                            if seen_time_in_millis {
                                return Err(serde::de::Error::duplicate_field("time_in_millis"));
                            }
                            seen_time_in_millis = true;
                            f_time_in_millis = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RequestStats {
                    current: f_current,
                    time: f_time,
                    time_in_millis: f_time_in_millis,
                    total: f_total,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RequestStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.current {
            map.serialize_entry("current", v)?;
        }
        if let Some(v) = &self.time {
            map.serialize_entry("time", v)?;
        }
        if let Some(v) = &self.time_in_millis {
            map.serialize_entry("time_in_millis", v)?;
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

#[doc = "Any statistics about CPU and memory usage."]
#[derive(Debug, Clone)]
pub struct ResourceStat {
    pub cpu_time_in_nanos: crate::types::common::DurationValueUnitNanos,
    pub memory_in_bytes: crate::types::common::ByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ResourceStat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ResourceStat;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ResourceStat object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cpu_time_in_nanos: Option<crate::types::common::DurationValueUnitNanos> =
                    None;
                let mut f_memory_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cpu_time_in_nanos" => {
                            if f_cpu_time_in_nanos.is_some() {
                                return Err(serde::de::Error::duplicate_field("cpu_time_in_nanos"));
                            }
                            f_cpu_time_in_nanos = Some(
                                map.next_value::<crate::types::common::DurationValueUnitNanos>()?,
                            );
                        }
                        "memory_in_bytes" => {
                            if f_memory_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("memory_in_bytes"));
                            }
                            f_memory_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ResourceStat {
                    cpu_time_in_nanos: f_cpu_time_in_nanos
                        .ok_or_else(|| serde::de::Error::missing_field("cpu_time_in_nanos"))?,
                    memory_in_bytes: f_memory_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("memory_in_bytes"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ResourceStat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("cpu_time_in_nanos", &self.cpu_time_in_nanos)?;
        map.serialize_entry("memory_in_bytes", &self.memory_in_bytes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The statistics about resource usage."]
#[derive(Debug, Clone)]
pub struct ResourceStats {
    pub average: crate::types::common::ResourceStat,
    pub max: crate::types::common::ResourceStat,
    pub min: crate::types::common::ResourceStat,
    pub thread_info: crate::types::common::ThreadInfo,
    pub total: crate::types::common::ResourceStat,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ResourceStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ResourceStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ResourceStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_average: Option<crate::types::common::ResourceStat> = None;
                let mut f_max: Option<crate::types::common::ResourceStat> = None;
                let mut f_min: Option<crate::types::common::ResourceStat> = None;
                let mut f_thread_info: Option<crate::types::common::ThreadInfo> = None;
                let mut f_total: Option<crate::types::common::ResourceStat> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "average" => {
                            if f_average.is_some() {
                                return Err(serde::de::Error::duplicate_field("average"));
                            }
                            f_average =
                                Some(map.next_value::<crate::types::common::ResourceStat>()?);
                        }
                        "max" => {
                            if f_max.is_some() {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            f_max = Some(map.next_value::<crate::types::common::ResourceStat>()?);
                        }
                        "min" => {
                            if f_min.is_some() {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            f_min = Some(map.next_value::<crate::types::common::ResourceStat>()?);
                        }
                        "thread_info" => {
                            if f_thread_info.is_some() {
                                return Err(serde::de::Error::duplicate_field("thread_info"));
                            }
                            f_thread_info =
                                Some(map.next_value::<crate::types::common::ThreadInfo>()?);
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(map.next_value::<crate::types::common::ResourceStat>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ResourceStats {
                    average: f_average.ok_or_else(|| serde::de::Error::missing_field("average"))?,
                    max: f_max.ok_or_else(|| serde::de::Error::missing_field("max"))?,
                    min: f_min.ok_or_else(|| serde::de::Error::missing_field("min"))?,
                    thread_info: f_thread_info
                        .ok_or_else(|| serde::de::Error::missing_field("thread_info"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ResourceStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("average", &self.average)?;
        map.serialize_entry("max", &self.max)?;
        map.serialize_entry("min", &self.min)?;
        map.serialize_entry("thread_info", &self.thread_info)?;
        map.serialize_entry("total", &self.total)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Result {
    #[doc = "`created`"]
    Created,
    #[doc = "`deleted`"]
    Deleted,
    #[doc = "`noop`"]
    Noop,
    #[doc = "`not_found`"]
    NotFound,
    #[doc = "`updated`"]
    Updated,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl Result {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            Result::Created => "created",
            Result::Deleted => "deleted",
            Result::Noop => "noop",
            Result::NotFound => "not_found",
            Result::Updated => "updated",
            Result::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for Result {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for Result {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "created" => Result::Created,
            "deleted" => Result::Deleted,
            "noop" => Result::Noop,
            "not_found" => Result::NotFound,
            "updated" => Result::Updated,
            _ => Result::Unknown(s),
        })
    }
}

#[doc = "The retry statistics for bulk and search operations."]
#[derive(Debug, Clone)]
pub struct Retries {
    #[doc = "The number of bulk operation retries."]
    pub bulk: i64,
    #[doc = "The number of search operation retries."]
    pub search: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Retries {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Retries;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Retries object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_bulk: Option<i64> = None;
                let mut f_search: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "bulk" => {
                            if f_bulk.is_some() {
                                return Err(serde::de::Error::duplicate_field("bulk"));
                            }
                            f_bulk = Some(map.next_value::<i64>()?);
                        }
                        "search" => {
                            if f_search.is_some() {
                                return Err(serde::de::Error::duplicate_field("search"));
                            }
                            f_search = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Retries {
                    bulk: f_bulk.ok_or_else(|| serde::de::Error::missing_field("bulk"))?,
                    search: f_search.ok_or_else(|| serde::de::Error::missing_field("search"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Retries {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("bulk", &self.bulk)?;
        map.serialize_entry("search", &self.search)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The routing value for the document."]
pub type Routing = String;

#[derive(Debug, Clone)]
pub struct ScoreSort {
    pub order: Option<crate::types::common::SortOrder>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ScoreSort {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ScoreSort;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ScoreSort object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_order: Option<crate::types::common::SortOrder> = None;
                let mut seen_order = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "order" => {
                            if seen_order {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            seen_order = true;
                            f_order =
                                map.next_value::<Option<crate::types::common::SortOrder>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ScoreSort {
                    order: f_order,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ScoreSort {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.order {
            map.serialize_entry("order", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
#[derive(Debug, Clone)]
pub struct ScriptField {
    #[doc = "Whether to ignore failures during script execution."]
    pub ignore_failure: Option<bool>,
    #[doc = "The script to execute for this field."]
    pub script: crate::types::common::Script,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ScriptField {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ScriptField;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ScriptField object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_script: Option<crate::types::common::Script> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "script" => {
                            if f_script.is_some() {
                                return Err(serde::de::Error::duplicate_field("script"));
                            }
                            f_script = Some(map.next_value::<crate::types::common::Script>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ScriptField {
                    ignore_failure: f_ignore_failure,
                    script: f_script.ok_or_else(|| serde::de::Error::missing_field("script"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ScriptField {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        map.serialize_entry("script", &self.script)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct ScriptSort {
    #[doc = "The mode for sorting on array or multi-valued fields."]
    pub mode: Option<crate::types::common::SortMode>,
    #[doc = "The nested path and filter for nested sorting."]
    pub nested: Option<crate::types::common::NestedSortValue>,
    pub order: Option<crate::types::common::SortOrder>,
    #[doc = "The script to use for sorting."]
    pub script: crate::types::common::Script,
    #[doc = "The type of the script sort value."]
    pub r#type: Option<crate::types::common::ScriptSortType>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ScriptSort {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ScriptSort;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ScriptSort object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_mode: Option<crate::types::common::SortMode> = None;
                let mut seen_mode = false;
                let mut f_nested: Option<crate::types::common::NestedSortValue> = None;
                let mut seen_nested = false;
                let mut f_order: Option<crate::types::common::SortOrder> = None;
                let mut seen_order = false;
                let mut f_script: Option<crate::types::common::Script> = None;
                let mut f_type: Option<crate::types::common::ScriptSortType> = None;
                let mut seen_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "mode" => {
                            if seen_mode {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            seen_mode = true;
                            f_mode = map.next_value::<Option<crate::types::common::SortMode>>()?;
                        }
                        "nested" => {
                            if seen_nested {
                                return Err(serde::de::Error::duplicate_field("nested"));
                            }
                            seen_nested = true;
                            f_nested =
                                map.next_value::<Option<crate::types::common::NestedSortValue>>()?;
                        }
                        "order" => {
                            if seen_order {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            seen_order = true;
                            f_order =
                                map.next_value::<Option<crate::types::common::SortOrder>>()?;
                        }
                        "script" => {
                            if f_script.is_some() {
                                return Err(serde::de::Error::duplicate_field("script"));
                            }
                            f_script = Some(map.next_value::<crate::types::common::Script>()?);
                        }
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type =
                                map.next_value::<Option<crate::types::common::ScriptSortType>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ScriptSort {
                    mode: f_mode,
                    nested: f_nested,
                    order: f_order,
                    script: f_script.ok_or_else(|| serde::de::Error::missing_field("script"))?,
                    r#type: f_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ScriptSort {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.mode {
            map.serialize_entry("mode", v)?;
        }
        if let Some(v) = &self.nested {
            map.serialize_entry("nested", v)?;
        }
        if let Some(v) = &self.order {
            map.serialize_entry("order", v)?;
        }
        map.serialize_entry("script", &self.script)?;
        if let Some(v) = &self.r#type {
            map.serialize_entry("type", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScriptSortType {
    #[doc = "`number`"]
    Number,
    #[doc = "`string`"]
    String,
    #[doc = "`version`"]
    Version,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ScriptSortType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ScriptSortType::Number => "number",
            ScriptSortType::String => "string",
            ScriptSortType::Version => "version",
            ScriptSortType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ScriptSortType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScriptSortType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "number" => ScriptSortType::Number,
            "string" => ScriptSortType::String,
            "version" => ScriptSortType::Version,
            _ => ScriptSortType::Unknown(s),
        })
    }
}

pub type ScrollId = String;

#[doc = "The details of a search failure during scrolling."]
#[derive(Debug, Clone)]
pub struct ScrollableHitSourceSearchFailure {
    pub index: Option<crate::types::common::IndexName>,
    pub node: Option<String>,
    pub reason: crate::types::common::ErrorCause,
    pub shard: Option<i32>,
    pub status: i32,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ScrollableHitSourceSearchFailure {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ScrollableHitSourceSearchFailure;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ScrollableHitSourceSearchFailure object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut seen_index = false;
                let mut f_node: Option<String> = None;
                let mut seen_node = false;
                let mut f_reason: Option<crate::types::common::ErrorCause> = None;
                let mut f_shard: Option<i32> = None;
                let mut seen_shard = false;
                let mut f_status: Option<i32> = None;
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
                            f_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "node" => {
                            if seen_node {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            seen_node = true;
                            f_node = map.next_value::<Option<String>>()?;
                        }
                        "reason" => {
                            if f_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            f_reason = Some(map.next_value::<crate::types::common::ErrorCause>()?);
                        }
                        "shard" => {
                            if seen_shard {
                                return Err(serde::de::Error::duplicate_field("shard"));
                            }
                            seen_shard = true;
                            f_shard = map.next_value::<Option<i32>>()?;
                        }
                        "status" => {
                            if f_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            f_status = Some(map.next_value::<i32>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ScrollableHitSourceSearchFailure {
                    index: f_index,
                    node: f_node,
                    reason: f_reason.ok_or_else(|| serde::de::Error::missing_field("reason"))?,
                    shard: f_shard,
                    status: f_status.ok_or_else(|| serde::de::Error::missing_field("status"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ScrollableHitSourceSearchFailure {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.node {
            map.serialize_entry("node", v)?;
        }
        map.serialize_entry("reason", &self.reason)?;
        if let Some(v) = &self.shard {
            map.serialize_entry("shard", v)?;
        }
        map.serialize_entry("status", &self.status)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The statistics about search operations."]
#[derive(Debug, Clone)]
pub struct SearchStats {
    #[doc = "The average slice count of all search requests. This is computed as the total slice count divided by the total number of concurrent search requests."]
    pub concurrent_avg_slice_count: Option<f64>,
    #[doc = "The number of currently running query operations using concurrent segment search."]
    pub concurrent_query_current: Option<i64>,
    pub concurrent_query_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time taken by all query operations using concurrent segment search, in milliseconds."]
    pub concurrent_query_time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "The total number of query operations using concurrent segment search."]
    pub concurrent_query_total: Option<i64>,
    #[doc = "The number of currently running shard fetch operations."]
    pub fetch_current: i64,
    #[doc = "The total amount of time taken to complete all shard fetch operations."]
    pub fetch_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time taken to complete all shard fetch operations, in milliseconds."]
    pub fetch_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The total number of shard fetch operations."]
    pub fetch_total: i64,
    #[doc = "The search statistics grouped by search groups."]
    pub groups: Option<std::collections::BTreeMap<String, crate::types::common::SearchStats>>,
    #[doc = "The number of open search contexts."]
    pub open_contexts: Option<i64>,
    #[doc = "The number of currently open shard PIT contexts."]
    pub point_in_time_current: Option<i64>,
    pub point_in_time_time: Option<crate::types::common::Duration>,
    #[doc = "The amount of time that shard PIT contexts have been held open since the node last restarted, in milliseconds."]
    pub point_in_time_time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "The total number of shard Point in Time (PIT) contexts created (completed and active) since the node last restarted."]
    pub point_in_time_total: Option<i64>,
    #[doc = "The number of currently running shard query operations."]
    pub query_current: i64,
    #[doc = "The total amount of time taken to complete all shard query operations."]
    pub query_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time taken to complete all shard query operations, in milliseconds."]
    pub query_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The total number of shard query operations."]
    pub query_total: i64,
    #[doc = "Statistics related to coordinator search operations for the node."]
    pub request: Option<std::collections::BTreeMap<String, crate::types::common::RequestStats>>,
    #[doc = "The number of shard scroll operations that are currently running."]
    pub scroll_current: i64,
    #[doc = "The total amount of time taken to complete all shard scroll operations."]
    pub scroll_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time taken to complete all shard scroll operations, in milliseconds."]
    pub scroll_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The total number of shard scroll operations."]
    pub scroll_total: i64,
    #[doc = "The total number of idle search reactivations."]
    pub search_idle_reactivate_count_total: Option<i64>,
    #[doc = "The number of currently running shard suggest operations."]
    pub suggest_current: i64,
    #[doc = "The total amount of time take to complete all shard suggest operations."]
    pub suggest_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time taken to complete all shard suggest operations, in milliseconds."]
    pub suggest_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The total number of shard suggest operations."]
    pub suggest_total: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SearchStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SearchStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_concurrent_avg_slice_count: Option<f64> = None;
                let mut seen_concurrent_avg_slice_count = false;
                let mut f_concurrent_query_current: Option<i64> = None;
                let mut seen_concurrent_query_current = false;
                let mut f_concurrent_query_time: Option<crate::types::common::Duration> = None;
                let mut seen_concurrent_query_time = false;
                let mut f_concurrent_query_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut seen_concurrent_query_time_in_millis = false;
                let mut f_concurrent_query_total: Option<i64> = None;
                let mut seen_concurrent_query_total = false;
                let mut f_fetch_current: Option<i64> = None;
                let mut f_fetch_time: Option<crate::types::common::Duration> = None;
                let mut seen_fetch_time = false;
                let mut f_fetch_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_fetch_total: Option<i64> = None;
                let mut f_groups: Option<
                    std::collections::BTreeMap<String, crate::types::common::SearchStats>,
                > = None;
                let mut seen_groups = false;
                let mut f_open_contexts: Option<i64> = None;
                let mut seen_open_contexts = false;
                let mut f_point_in_time_current: Option<i64> = None;
                let mut seen_point_in_time_current = false;
                let mut f_point_in_time_time: Option<crate::types::common::Duration> = None;
                let mut seen_point_in_time_time = false;
                let mut f_point_in_time_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut seen_point_in_time_time_in_millis = false;
                let mut f_point_in_time_total: Option<i64> = None;
                let mut seen_point_in_time_total = false;
                let mut f_query_current: Option<i64> = None;
                let mut f_query_time: Option<crate::types::common::Duration> = None;
                let mut seen_query_time = false;
                let mut f_query_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_query_total: Option<i64> = None;
                let mut f_request: Option<
                    std::collections::BTreeMap<String, crate::types::common::RequestStats>,
                > = None;
                let mut seen_request = false;
                let mut f_scroll_current: Option<i64> = None;
                let mut f_scroll_time: Option<crate::types::common::Duration> = None;
                let mut seen_scroll_time = false;
                let mut f_scroll_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_scroll_total: Option<i64> = None;
                let mut f_search_idle_reactivate_count_total: Option<i64> = None;
                let mut seen_search_idle_reactivate_count_total = false;
                let mut f_suggest_current: Option<i64> = None;
                let mut f_suggest_time: Option<crate::types::common::Duration> = None;
                let mut seen_suggest_time = false;
                let mut f_suggest_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_suggest_total: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "concurrent_avg_slice_count" => {
                            if seen_concurrent_avg_slice_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "concurrent_avg_slice_count",
                                ));
                            }
                            seen_concurrent_avg_slice_count = true;
                            f_concurrent_avg_slice_count = map.next_value::<Option<f64>>()?;
                        }
                        "concurrent_query_current" => {
                            if seen_concurrent_query_current {
                                return Err(serde::de::Error::duplicate_field(
                                    "concurrent_query_current",
                                ));
                            }
                            seen_concurrent_query_current = true;
                            f_concurrent_query_current = map.next_value::<Option<i64>>()?;
                        }
                        "concurrent_query_time" => {
                            if seen_concurrent_query_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "concurrent_query_time",
                                ));
                            }
                            seen_concurrent_query_time = true;
                            f_concurrent_query_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "concurrent_query_time_in_millis" => {
                            if seen_concurrent_query_time_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "concurrent_query_time_in_millis",
                                ));
                            }
                            seen_concurrent_query_time_in_millis = true;
                            f_concurrent_query_time_in_millis = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        "concurrent_query_total" => {
                            if seen_concurrent_query_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "concurrent_query_total",
                                ));
                            }
                            seen_concurrent_query_total = true;
                            f_concurrent_query_total = map.next_value::<Option<i64>>()?;
                        }
                        "fetch_current" => {
                            if f_fetch_current.is_some() {
                                return Err(serde::de::Error::duplicate_field("fetch_current"));
                            }
                            f_fetch_current = Some(map.next_value::<i64>()?);
                        }
                        "fetch_time" => {
                            if seen_fetch_time {
                                return Err(serde::de::Error::duplicate_field("fetch_time"));
                            }
                            seen_fetch_time = true;
                            f_fetch_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "fetch_time_in_millis" => {
                            if f_fetch_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fetch_time_in_millis",
                                ));
                            }
                            f_fetch_time_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "fetch_total" => {
                            if f_fetch_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("fetch_total"));
                            }
                            f_fetch_total = Some(map.next_value::<i64>()?);
                        }
                        "groups" => {
                            if seen_groups {
                                return Err(serde::de::Error::duplicate_field("groups"));
                            }
                            seen_groups = true;
                            f_groups = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common::SearchStats,
                                >,
                            >>()?;
                        }
                        "open_contexts" => {
                            if seen_open_contexts {
                                return Err(serde::de::Error::duplicate_field("open_contexts"));
                            }
                            seen_open_contexts = true;
                            f_open_contexts = map.next_value::<Option<i64>>()?;
                        }
                        "point_in_time_current" => {
                            if seen_point_in_time_current {
                                return Err(serde::de::Error::duplicate_field(
                                    "point_in_time_current",
                                ));
                            }
                            seen_point_in_time_current = true;
                            f_point_in_time_current = map.next_value::<Option<i64>>()?;
                        }
                        "point_in_time_time" => {
                            if seen_point_in_time_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "point_in_time_time",
                                ));
                            }
                            seen_point_in_time_time = true;
                            f_point_in_time_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "point_in_time_time_in_millis" => {
                            if seen_point_in_time_time_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "point_in_time_time_in_millis",
                                ));
                            }
                            seen_point_in_time_time_in_millis = true;
                            f_point_in_time_time_in_millis = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        "point_in_time_total" => {
                            if seen_point_in_time_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "point_in_time_total",
                                ));
                            }
                            seen_point_in_time_total = true;
                            f_point_in_time_total = map.next_value::<Option<i64>>()?;
                        }
                        "query_current" => {
                            if f_query_current.is_some() {
                                return Err(serde::de::Error::duplicate_field("query_current"));
                            }
                            f_query_current = Some(map.next_value::<i64>()?);
                        }
                        "query_time" => {
                            if seen_query_time {
                                return Err(serde::de::Error::duplicate_field("query_time"));
                            }
                            seen_query_time = true;
                            f_query_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "query_time_in_millis" => {
                            if f_query_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "query_time_in_millis",
                                ));
                            }
                            f_query_time_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "query_total" => {
                            if f_query_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("query_total"));
                            }
                            f_query_total = Some(map.next_value::<i64>()?);
                        }
                        "request" => {
                            if seen_request {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            seen_request = true;
                            f_request = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common::RequestStats,
                                >,
                            >>()?;
                        }
                        "scroll_current" => {
                            if f_scroll_current.is_some() {
                                return Err(serde::de::Error::duplicate_field("scroll_current"));
                            }
                            f_scroll_current = Some(map.next_value::<i64>()?);
                        }
                        "scroll_time" => {
                            if seen_scroll_time {
                                return Err(serde::de::Error::duplicate_field("scroll_time"));
                            }
                            seen_scroll_time = true;
                            f_scroll_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "scroll_time_in_millis" => {
                            if f_scroll_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "scroll_time_in_millis",
                                ));
                            }
                            f_scroll_time_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "scroll_total" => {
                            if f_scroll_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("scroll_total"));
                            }
                            f_scroll_total = Some(map.next_value::<i64>()?);
                        }
                        "search_idle_reactivate_count_total" => {
                            if seen_search_idle_reactivate_count_total {
                                return Err(serde::de::Error::duplicate_field(
                                    "search_idle_reactivate_count_total",
                                ));
                            }
                            seen_search_idle_reactivate_count_total = true;
                            f_search_idle_reactivate_count_total =
                                map.next_value::<Option<i64>>()?;
                        }
                        "suggest_current" => {
                            if f_suggest_current.is_some() {
                                return Err(serde::de::Error::duplicate_field("suggest_current"));
                            }
                            f_suggest_current = Some(map.next_value::<i64>()?);
                        }
                        "suggest_time" => {
                            if seen_suggest_time {
                                return Err(serde::de::Error::duplicate_field("suggest_time"));
                            }
                            seen_suggest_time = true;
                            f_suggest_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "suggest_time_in_millis" => {
                            if f_suggest_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "suggest_time_in_millis",
                                ));
                            }
                            f_suggest_time_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "suggest_total" => {
                            if f_suggest_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("suggest_total"));
                            }
                            f_suggest_total = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SearchStats {
                    concurrent_avg_slice_count: f_concurrent_avg_slice_count,
                    concurrent_query_current: f_concurrent_query_current,
                    concurrent_query_time: f_concurrent_query_time,
                    concurrent_query_time_in_millis: f_concurrent_query_time_in_millis,
                    concurrent_query_total: f_concurrent_query_total,
                    fetch_current: f_fetch_current
                        .ok_or_else(|| serde::de::Error::missing_field("fetch_current"))?,
                    fetch_time: f_fetch_time,
                    fetch_time_in_millis: f_fetch_time_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("fetch_time_in_millis"))?,
                    fetch_total: f_fetch_total
                        .ok_or_else(|| serde::de::Error::missing_field("fetch_total"))?,
                    groups: f_groups,
                    open_contexts: f_open_contexts,
                    point_in_time_current: f_point_in_time_current,
                    point_in_time_time: f_point_in_time_time,
                    point_in_time_time_in_millis: f_point_in_time_time_in_millis,
                    point_in_time_total: f_point_in_time_total,
                    query_current: f_query_current
                        .ok_or_else(|| serde::de::Error::missing_field("query_current"))?,
                    query_time: f_query_time,
                    query_time_in_millis: f_query_time_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("query_time_in_millis"))?,
                    query_total: f_query_total
                        .ok_or_else(|| serde::de::Error::missing_field("query_total"))?,
                    request: f_request,
                    scroll_current: f_scroll_current
                        .ok_or_else(|| serde::de::Error::missing_field("scroll_current"))?,
                    scroll_time: f_scroll_time,
                    scroll_time_in_millis: f_scroll_time_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("scroll_time_in_millis"))?,
                    scroll_total: f_scroll_total
                        .ok_or_else(|| serde::de::Error::missing_field("scroll_total"))?,
                    search_idle_reactivate_count_total: f_search_idle_reactivate_count_total,
                    suggest_current: f_suggest_current
                        .ok_or_else(|| serde::de::Error::missing_field("suggest_current"))?,
                    suggest_time: f_suggest_time,
                    suggest_time_in_millis: f_suggest_time_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("suggest_time_in_millis"))?,
                    suggest_total: f_suggest_total
                        .ok_or_else(|| serde::de::Error::missing_field("suggest_total"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SearchStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.concurrent_avg_slice_count {
            map.serialize_entry("concurrent_avg_slice_count", v)?;
        }
        if let Some(v) = &self.concurrent_query_current {
            map.serialize_entry("concurrent_query_current", v)?;
        }
        if let Some(v) = &self.concurrent_query_time {
            map.serialize_entry("concurrent_query_time", v)?;
        }
        if let Some(v) = &self.concurrent_query_time_in_millis {
            map.serialize_entry("concurrent_query_time_in_millis", v)?;
        }
        if let Some(v) = &self.concurrent_query_total {
            map.serialize_entry("concurrent_query_total", v)?;
        }
        map.serialize_entry("fetch_current", &self.fetch_current)?;
        if let Some(v) = &self.fetch_time {
            map.serialize_entry("fetch_time", v)?;
        }
        map.serialize_entry("fetch_time_in_millis", &self.fetch_time_in_millis)?;
        map.serialize_entry("fetch_total", &self.fetch_total)?;
        if let Some(v) = &self.groups {
            map.serialize_entry("groups", v)?;
        }
        if let Some(v) = &self.open_contexts {
            map.serialize_entry("open_contexts", v)?;
        }
        if let Some(v) = &self.point_in_time_current {
            map.serialize_entry("point_in_time_current", v)?;
        }
        if let Some(v) = &self.point_in_time_time {
            map.serialize_entry("point_in_time_time", v)?;
        }
        if let Some(v) = &self.point_in_time_time_in_millis {
            map.serialize_entry("point_in_time_time_in_millis", v)?;
        }
        if let Some(v) = &self.point_in_time_total {
            map.serialize_entry("point_in_time_total", v)?;
        }
        map.serialize_entry("query_current", &self.query_current)?;
        if let Some(v) = &self.query_time {
            map.serialize_entry("query_time", v)?;
        }
        map.serialize_entry("query_time_in_millis", &self.query_time_in_millis)?;
        map.serialize_entry("query_total", &self.query_total)?;
        if let Some(v) = &self.request {
            map.serialize_entry("request", v)?;
        }
        map.serialize_entry("scroll_current", &self.scroll_current)?;
        if let Some(v) = &self.scroll_time {
            map.serialize_entry("scroll_time", v)?;
        }
        map.serialize_entry("scroll_time_in_millis", &self.scroll_time_in_millis)?;
        map.serialize_entry("scroll_total", &self.scroll_total)?;
        if let Some(v) = &self.search_idle_reactivate_count_total {
            map.serialize_entry("search_idle_reactivate_count_total", v)?;
        }
        map.serialize_entry("suggest_current", &self.suggest_current)?;
        if let Some(v) = &self.suggest_time {
            map.serialize_entry("suggest_time", v)?;
        }
        map.serialize_entry("suggest_time_in_millis", &self.suggest_time_in_millis)?;
        map.serialize_entry("suggest_total", &self.suggest_total)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
#[derive(Debug, Clone)]
pub struct SegmentReplicationStatsObject {
    #[doc = "The maximum number of bytes the replica is behind the primary."]
    pub max_bytes_behind: crate::types::common::HumanReadableByteCount,
    #[doc = "The maximum replication lag between primary and replica shards."]
    pub max_replication_lag: crate::types::common::Duration,
    #[doc = "The total number of bytes all replicas are behind their primaries."]
    pub total_bytes_behind: crate::types::common::HumanReadableByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SegmentReplicationStatsObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SegmentReplicationStatsObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SegmentReplicationStatsObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_max_bytes_behind: Option<crate::types::common::HumanReadableByteCount> =
                    None;
                let mut f_max_replication_lag: Option<crate::types::common::Duration> = None;
                let mut f_total_bytes_behind: Option<crate::types::common::HumanReadableByteCount> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "max_bytes_behind" => {
                            if f_max_bytes_behind.is_some() {
                                return Err(serde::de::Error::duplicate_field("max_bytes_behind"));
                            }
                            f_max_bytes_behind = Some(
                                map.next_value::<crate::types::common::HumanReadableByteCount>()?,
                            );
                        }
                        "max_replication_lag" => {
                            if f_max_replication_lag.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_replication_lag",
                                ));
                            }
                            f_max_replication_lag =
                                Some(map.next_value::<crate::types::common::Duration>()?);
                        }
                        "total_bytes_behind" => {
                            if f_total_bytes_behind.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_bytes_behind",
                                ));
                            }
                            f_total_bytes_behind = Some(
                                map.next_value::<crate::types::common::HumanReadableByteCount>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SegmentReplicationStatsObject {
                    max_bytes_behind: f_max_bytes_behind
                        .ok_or_else(|| serde::de::Error::missing_field("max_bytes_behind"))?,
                    max_replication_lag: f_max_replication_lag
                        .ok_or_else(|| serde::de::Error::missing_field("max_replication_lag"))?,
                    total_bytes_behind: f_total_bytes_behind
                        .ok_or_else(|| serde::de::Error::missing_field("total_bytes_behind"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SegmentReplicationStatsObject {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("max_bytes_behind", &self.max_bytes_behind)?;
        map.serialize_entry("max_replication_lag", &self.max_replication_lag)?;
        map.serialize_entry("total_bytes_behind", &self.total_bytes_behind)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The segment replication statistics for version 2.12.0 and later."]
#[derive(Debug, Clone)]
pub struct SegmentReplicationStatsObjectObject {
    #[doc = "The maximum number of bytes the replica is behind the primary."]
    pub max_bytes_behind: crate::types::common::ByteCount,
    #[doc = "The maximum replication lag between primary and replica shards in milliseconds."]
    pub max_replication_lag: crate::types::common::DurationValueUnitMillis,
    #[doc = "The total number of bytes all replicas are behind their primaries."]
    pub total_bytes_behind: crate::types::common::ByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SegmentReplicationStatsObjectObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SegmentReplicationStatsObjectObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SegmentReplicationStatsObjectObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_max_bytes_behind: Option<crate::types::common::ByteCount> = None;
                let mut f_max_replication_lag: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_total_bytes_behind: Option<crate::types::common::ByteCount> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "max_bytes_behind" => {
                            if f_max_bytes_behind.is_some() {
                                return Err(serde::de::Error::duplicate_field("max_bytes_behind"));
                            }
                            f_max_bytes_behind =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "max_replication_lag" => {
                            if f_max_replication_lag.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_replication_lag",
                                ));
                            }
                            f_max_replication_lag = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "total_bytes_behind" => {
                            if f_total_bytes_behind.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_bytes_behind",
                                ));
                            }
                            f_total_bytes_behind =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SegmentReplicationStatsObjectObject {
                    max_bytes_behind: f_max_bytes_behind
                        .ok_or_else(|| serde::de::Error::missing_field("max_bytes_behind"))?,
                    max_replication_lag: f_max_replication_lag
                        .ok_or_else(|| serde::de::Error::missing_field("max_replication_lag"))?,
                    total_bytes_behind: f_total_bytes_behind
                        .ok_or_else(|| serde::de::Error::missing_field("total_bytes_behind"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SegmentReplicationStatsObjectObject {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("max_bytes_behind", &self.max_bytes_behind)?;
        map.serialize_entry("max_replication_lag", &self.max_replication_lag)?;
        map.serialize_entry("total_bytes_behind", &self.total_bytes_behind)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The statistics about segments."]
#[derive(Debug, Clone)]
pub struct SegmentsStats {
    #[doc = "The total number of segments across all shards assigned to the selected nodes."]
    pub count: i32,
    #[doc = "The human-readable amount of memory used for doc values."]
    pub doc_values_memory: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount, in bytes, of memory used for document values across all shards assigned to the selected nodes."]
    pub doc_values_memory_in_bytes: crate::types::common::ByteCount,
    #[doc = "This object is not populated by the cluster stats API. To get information on segment files, use the node stats API."]
    pub file_sizes:
        std::collections::BTreeMap<String, crate::types::indices_stats::ShardFileSizeInfo>,
    pub fixed_bit_set: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount of memory, in bytes, used by fixed bit sets across all shards assigned to the selected nodes."]
    pub fixed_bit_set_memory_in_bytes: crate::types::common::ByteCount,
    #[doc = "The maximum amount of memory, in bytes, used by index writers."]
    pub index_writer_max_memory_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "The human-readable amount of memory used by index writers."]
    pub index_writer_memory: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount, in bytes, of memory used by all index writers across all shards assigned to the selected nodes."]
    pub index_writer_memory_in_bytes: crate::types::common::ByteCount,
    #[doc = "The Unix timestamp, in milliseconds, of the most recently retried indexing request."]
    pub max_unsafe_auto_id_timestamp: crate::types::common::EpochTimeUnitMillis,
    #[doc = "The total amount of memory used for segments across all shards assigned to the selected nodes."]
    pub memory: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount, in bytes, of memory used for segments across all shards assigned to the selected nodes."]
    pub memory_in_bytes: crate::types::common::ByteCount,
    #[doc = "The total amount of memory used for normalization factors across all shards assigned to the selected nodes."]
    pub norms_memory: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount, in bytes, of memory used for normalization factors across all shards assigned to the selected nodes."]
    pub norms_memory_in_bytes: crate::types::common::ByteCount,
    #[doc = "The total amount of memory used for points across all shards assigned to the selected nodes."]
    pub points_memory: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount, in bytes, of memory used for points across all shards assigned to the selected nodes."]
    pub points_memory_in_bytes: crate::types::common::ByteCount,
    pub remote_store: Option<crate::types::common::RemoteStoreStats>,
    pub segment_replication: Option<crate::types::common::SegmentReplicationStats>,
    #[doc = "The total amount of memory used for stored fields across all shards assigned to the selected nodes."]
    pub stored_fields_memory: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount, in bytes, of memory used for stored fields across all shards assigned to the selected nodes."]
    pub stored_fields_memory_in_bytes: crate::types::common::ByteCount,
    #[doc = "The total amount of memory used for term vectors across all shards assigned to the selected nodes."]
    pub term_vectors_memory: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount, in bytes, of memory used for term vectors across all shards assigned to the selected nodes."]
    pub term_vectors_memory_in_bytes: crate::types::common::ByteCount,
    #[doc = "The total amount of memory used for terms across all shards assigned to the selected nodes."]
    pub terms_memory: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount, in bytes, of memory used for terms across all shards assigned to the selected nodes."]
    pub terms_memory_in_bytes: crate::types::common::ByteCount,
    #[doc = "The total amount of memory used by all version maps across all shards assigned to the selected nodes."]
    pub version_map_memory: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total amount, in bytes, of memory used by all version maps across all shards assigned to the selected nodes."]
    pub version_map_memory_in_bytes: crate::types::common::ByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SegmentsStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SegmentsStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SegmentsStats object")
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
                let mut f_doc_values_memory: Option<crate::types::common::HumanReadableByteCount> =
                    None;
                let mut seen_doc_values_memory = false;
                let mut f_doc_values_memory_in_bytes: Option<crate::types::common::ByteCount> =
                    None;
                let mut f_file_sizes: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::indices_stats::ShardFileSizeInfo,
                    >,
                > = None;
                let mut f_fixed_bit_set: Option<crate::types::common::HumanReadableByteCount> =
                    None;
                let mut seen_fixed_bit_set = false;
                let mut f_fixed_bit_set_memory_in_bytes: Option<crate::types::common::ByteCount> =
                    None;
                let mut f_index_writer_max_memory_in_bytes: Option<
                    crate::types::common::ByteCount,
                > = None;
                let mut seen_index_writer_max_memory_in_bytes = false;
                let mut f_index_writer_memory: Option<
                    crate::types::common::HumanReadableByteCount,
                > = None;
                let mut seen_index_writer_memory = false;
                let mut f_index_writer_memory_in_bytes: Option<crate::types::common::ByteCount> =
                    None;
                let mut f_max_unsafe_auto_id_timestamp: Option<
                    crate::types::common::EpochTimeUnitMillis,
                > = None;
                let mut f_memory: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_memory = false;
                let mut f_memory_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_norms_memory: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_norms_memory = false;
                let mut f_norms_memory_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_points_memory: Option<crate::types::common::HumanReadableByteCount> =
                    None;
                let mut seen_points_memory = false;
                let mut f_points_memory_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_remote_store: Option<crate::types::common::RemoteStoreStats> = None;
                let mut seen_remote_store = false;
                let mut f_segment_replication: Option<
                    crate::types::common::SegmentReplicationStats,
                > = None;
                let mut seen_segment_replication = false;
                let mut f_stored_fields_memory: Option<
                    crate::types::common::HumanReadableByteCount,
                > = None;
                let mut seen_stored_fields_memory = false;
                let mut f_stored_fields_memory_in_bytes: Option<crate::types::common::ByteCount> =
                    None;
                let mut f_term_vectors_memory: Option<
                    crate::types::common::HumanReadableByteCount,
                > = None;
                let mut seen_term_vectors_memory = false;
                let mut f_term_vectors_memory_in_bytes: Option<crate::types::common::ByteCount> =
                    None;
                let mut f_terms_memory: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_terms_memory = false;
                let mut f_terms_memory_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_version_map_memory: Option<crate::types::common::HumanReadableByteCount> =
                    None;
                let mut seen_version_map_memory = false;
                let mut f_version_map_memory_in_bytes: Option<crate::types::common::ByteCount> =
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
                            f_count = Some(map.next_value::<i32>()?);
                        }
                        "doc_values_memory" => {
                            if seen_doc_values_memory {
                                return Err(serde::de::Error::duplicate_field("doc_values_memory"));
                            }
                            seen_doc_values_memory = true;
                            f_doc_values_memory = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "doc_values_memory_in_bytes" => {
                            if f_doc_values_memory_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "doc_values_memory_in_bytes",
                                ));
                            }
                            f_doc_values_memory_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "file_sizes" => {
                            if f_file_sizes.is_some() {
                                return Err(serde::de::Error::duplicate_field("file_sizes"));
                            }
                            f_file_sizes = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                crate::types::indices_stats::ShardFileSizeInfo,
                            >>()?);
                        }
                        "fixed_bit_set" => {
                            if seen_fixed_bit_set {
                                return Err(serde::de::Error::duplicate_field("fixed_bit_set"));
                            }
                            seen_fixed_bit_set = true;
                            f_fixed_bit_set = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "fixed_bit_set_memory_in_bytes" => {
                            if f_fixed_bit_set_memory_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fixed_bit_set_memory_in_bytes",
                                ));
                            }
                            f_fixed_bit_set_memory_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "index_writer_max_memory_in_bytes" => {
                            if seen_index_writer_max_memory_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "index_writer_max_memory_in_bytes",
                                ));
                            }
                            seen_index_writer_max_memory_in_bytes = true;
                            f_index_writer_max_memory_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "index_writer_memory" => {
                            if seen_index_writer_memory {
                                return Err(serde::de::Error::duplicate_field(
                                    "index_writer_memory",
                                ));
                            }
                            seen_index_writer_memory = true;
                            f_index_writer_memory = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "index_writer_memory_in_bytes" => {
                            if f_index_writer_memory_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "index_writer_memory_in_bytes",
                                ));
                            }
                            f_index_writer_memory_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "max_unsafe_auto_id_timestamp" => {
                            if f_max_unsafe_auto_id_timestamp.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_unsafe_auto_id_timestamp",
                                ));
                            }
                            f_max_unsafe_auto_id_timestamp = Some(
                                map.next_value::<crate::types::common::EpochTimeUnitMillis>()?,
                            );
                        }
                        "memory" => {
                            if seen_memory {
                                return Err(serde::de::Error::duplicate_field("memory"));
                            }
                            seen_memory = true;
                            f_memory = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "memory_in_bytes" => {
                            if f_memory_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("memory_in_bytes"));
                            }
                            f_memory_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "norms_memory" => {
                            if seen_norms_memory {
                                return Err(serde::de::Error::duplicate_field("norms_memory"));
                            }
                            seen_norms_memory = true;
                            f_norms_memory = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "norms_memory_in_bytes" => {
                            if f_norms_memory_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "norms_memory_in_bytes",
                                ));
                            }
                            f_norms_memory_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "points_memory" => {
                            if seen_points_memory {
                                return Err(serde::de::Error::duplicate_field("points_memory"));
                            }
                            seen_points_memory = true;
                            f_points_memory = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "points_memory_in_bytes" => {
                            if f_points_memory_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "points_memory_in_bytes",
                                ));
                            }
                            f_points_memory_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "remote_store" => {
                            if seen_remote_store {
                                return Err(serde::de::Error::duplicate_field("remote_store"));
                            }
                            seen_remote_store = true;
                            f_remote_store =
                                map.next_value::<Option<crate::types::common::RemoteStoreStats>>()?;
                        }
                        "segment_replication" => {
                            if seen_segment_replication {
                                return Err(serde::de::Error::duplicate_field(
                                    "segment_replication",
                                ));
                            }
                            seen_segment_replication = true;
                            f_segment_replication = map.next_value::<Option<crate::types::common::SegmentReplicationStats>>()?;
                        }
                        "stored_fields_memory" => {
                            if seen_stored_fields_memory {
                                return Err(serde::de::Error::duplicate_field(
                                    "stored_fields_memory",
                                ));
                            }
                            seen_stored_fields_memory = true;
                            f_stored_fields_memory = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "stored_fields_memory_in_bytes" => {
                            if f_stored_fields_memory_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "stored_fields_memory_in_bytes",
                                ));
                            }
                            f_stored_fields_memory_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "term_vectors_memory" => {
                            if seen_term_vectors_memory {
                                return Err(serde::de::Error::duplicate_field(
                                    "term_vectors_memory",
                                ));
                            }
                            seen_term_vectors_memory = true;
                            f_term_vectors_memory = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "term_vectors_memory_in_bytes" => {
                            if f_term_vectors_memory_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "term_vectors_memory_in_bytes",
                                ));
                            }
                            f_term_vectors_memory_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "terms_memory" => {
                            if seen_terms_memory {
                                return Err(serde::de::Error::duplicate_field("terms_memory"));
                            }
                            seen_terms_memory = true;
                            f_terms_memory = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "terms_memory_in_bytes" => {
                            if f_terms_memory_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "terms_memory_in_bytes",
                                ));
                            }
                            f_terms_memory_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "version_map_memory" => {
                            if seen_version_map_memory {
                                return Err(serde::de::Error::duplicate_field(
                                    "version_map_memory",
                                ));
                            }
                            seen_version_map_memory = true;
                            f_version_map_memory = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "version_map_memory_in_bytes" => {
                            if f_version_map_memory_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "version_map_memory_in_bytes",
                                ));
                            }
                            f_version_map_memory_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SegmentsStats {
                    count: f_count.ok_or_else(|| serde::de::Error::missing_field("count"))?,
                    doc_values_memory: f_doc_values_memory,
                    doc_values_memory_in_bytes: f_doc_values_memory_in_bytes.ok_or_else(|| {
                        serde::de::Error::missing_field("doc_values_memory_in_bytes")
                    })?,
                    file_sizes: f_file_sizes
                        .ok_or_else(|| serde::de::Error::missing_field("file_sizes"))?,
                    fixed_bit_set: f_fixed_bit_set,
                    fixed_bit_set_memory_in_bytes: f_fixed_bit_set_memory_in_bytes.ok_or_else(
                        || serde::de::Error::missing_field("fixed_bit_set_memory_in_bytes"),
                    )?,
                    index_writer_max_memory_in_bytes: f_index_writer_max_memory_in_bytes,
                    index_writer_memory: f_index_writer_memory,
                    index_writer_memory_in_bytes: f_index_writer_memory_in_bytes.ok_or_else(
                        || serde::de::Error::missing_field("index_writer_memory_in_bytes"),
                    )?,
                    max_unsafe_auto_id_timestamp: f_max_unsafe_auto_id_timestamp.ok_or_else(
                        || serde::de::Error::missing_field("max_unsafe_auto_id_timestamp"),
                    )?,
                    memory: f_memory,
                    memory_in_bytes: f_memory_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("memory_in_bytes"))?,
                    norms_memory: f_norms_memory,
                    norms_memory_in_bytes: f_norms_memory_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("norms_memory_in_bytes"))?,
                    points_memory: f_points_memory,
                    points_memory_in_bytes: f_points_memory_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("points_memory_in_bytes"))?,
                    remote_store: f_remote_store,
                    segment_replication: f_segment_replication,
                    stored_fields_memory: f_stored_fields_memory,
                    stored_fields_memory_in_bytes: f_stored_fields_memory_in_bytes.ok_or_else(
                        || serde::de::Error::missing_field("stored_fields_memory_in_bytes"),
                    )?,
                    term_vectors_memory: f_term_vectors_memory,
                    term_vectors_memory_in_bytes: f_term_vectors_memory_in_bytes.ok_or_else(
                        || serde::de::Error::missing_field("term_vectors_memory_in_bytes"),
                    )?,
                    terms_memory: f_terms_memory,
                    terms_memory_in_bytes: f_terms_memory_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("terms_memory_in_bytes"))?,
                    version_map_memory: f_version_map_memory,
                    version_map_memory_in_bytes: f_version_map_memory_in_bytes.ok_or_else(
                        || serde::de::Error::missing_field("version_map_memory_in_bytes"),
                    )?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SegmentsStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("count", &self.count)?;
        if let Some(v) = &self.doc_values_memory {
            map.serialize_entry("doc_values_memory", v)?;
        }
        map.serialize_entry(
            "doc_values_memory_in_bytes",
            &self.doc_values_memory_in_bytes,
        )?;
        map.serialize_entry("file_sizes", &self.file_sizes)?;
        if let Some(v) = &self.fixed_bit_set {
            map.serialize_entry("fixed_bit_set", v)?;
        }
        map.serialize_entry(
            "fixed_bit_set_memory_in_bytes",
            &self.fixed_bit_set_memory_in_bytes,
        )?;
        if let Some(v) = &self.index_writer_max_memory_in_bytes {
            map.serialize_entry("index_writer_max_memory_in_bytes", v)?;
        }
        if let Some(v) = &self.index_writer_memory {
            map.serialize_entry("index_writer_memory", v)?;
        }
        map.serialize_entry(
            "index_writer_memory_in_bytes",
            &self.index_writer_memory_in_bytes,
        )?;
        map.serialize_entry(
            "max_unsafe_auto_id_timestamp",
            &self.max_unsafe_auto_id_timestamp,
        )?;
        if let Some(v) = &self.memory {
            map.serialize_entry("memory", v)?;
        }
        map.serialize_entry("memory_in_bytes", &self.memory_in_bytes)?;
        if let Some(v) = &self.norms_memory {
            map.serialize_entry("norms_memory", v)?;
        }
        map.serialize_entry("norms_memory_in_bytes", &self.norms_memory_in_bytes)?;
        if let Some(v) = &self.points_memory {
            map.serialize_entry("points_memory", v)?;
        }
        map.serialize_entry("points_memory_in_bytes", &self.points_memory_in_bytes)?;
        if let Some(v) = &self.remote_store {
            map.serialize_entry("remote_store", v)?;
        }
        if let Some(v) = &self.segment_replication {
            map.serialize_entry("segment_replication", v)?;
        }
        if let Some(v) = &self.stored_fields_memory {
            map.serialize_entry("stored_fields_memory", v)?;
        }
        map.serialize_entry(
            "stored_fields_memory_in_bytes",
            &self.stored_fields_memory_in_bytes,
        )?;
        if let Some(v) = &self.term_vectors_memory {
            map.serialize_entry("term_vectors_memory", v)?;
        }
        map.serialize_entry(
            "term_vectors_memory_in_bytes",
            &self.term_vectors_memory_in_bytes,
        )?;
        if let Some(v) = &self.terms_memory {
            map.serialize_entry("terms_memory", v)?;
        }
        map.serialize_entry("terms_memory_in_bytes", &self.terms_memory_in_bytes)?;
        if let Some(v) = &self.version_map_memory {
            map.serialize_entry("version_map_memory", v)?;
        }
        map.serialize_entry(
            "version_map_memory_in_bytes",
            &self.version_map_memory_in_bytes,
        )?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The sequence number of the document."]
pub type SequenceNumber = i64;

#[derive(Debug, Clone)]
pub struct ShardFailure {
    pub index: Option<crate::types::common::IndexName>,
    pub node: Option<String>,
    pub primary: bool,
    pub reason: crate::types::common::ErrorCause,
    pub shard: i64,
    pub status: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardFailure {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardFailure;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardFailure object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut seen_index = false;
                let mut f_node: Option<String> = None;
                let mut seen_node = false;
                let mut f_primary: Option<bool> = None;
                let mut f_reason: Option<crate::types::common::ErrorCause> = None;
                let mut f_shard: Option<i64> = None;
                let mut f_status: Option<String> = None;
                let mut seen_status = false;
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
                            f_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "node" => {
                            if seen_node {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            seen_node = true;
                            f_node = map.next_value::<Option<String>>()?;
                        }
                        "primary" => {
                            if f_primary.is_some() {
                                return Err(serde::de::Error::duplicate_field("primary"));
                            }
                            f_primary = Some(map.next_value::<bool>()?);
                        }
                        "reason" => {
                            if f_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            f_reason = Some(map.next_value::<crate::types::common::ErrorCause>()?);
                        }
                        "shard" => {
                            if f_shard.is_some() {
                                return Err(serde::de::Error::duplicate_field("shard"));
                            }
                            f_shard = Some(map.next_value::<i64>()?);
                        }
                        "status" => {
                            if seen_status {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            seen_status = true;
                            f_status = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardFailure {
                    index: f_index,
                    node: f_node,
                    primary: f_primary.ok_or_else(|| serde::de::Error::missing_field("primary"))?,
                    reason: f_reason.ok_or_else(|| serde::de::Error::missing_field("reason"))?,
                    shard: f_shard.ok_or_else(|| serde::de::Error::missing_field("shard"))?,
                    status: f_status,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardFailure {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.node {
            map.serialize_entry("node", v)?;
        }
        map.serialize_entry("primary", &self.primary)?;
        map.serialize_entry("reason", &self.reason)?;
        map.serialize_entry("shard", &self.shard)?;
        if let Some(v) = &self.status {
            map.serialize_entry("status", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardInfo {
    pub failed: crate::types::common::Uint,
    pub failures: Option<Vec<crate::types::common::ShardFailure>>,
    pub successful: crate::types::common::Uint,
    pub total: crate::types::common::Uint,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardInfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardInfo object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_failed: Option<crate::types::common::Uint> = None;
                let mut f_failures: Option<Vec<crate::types::common::ShardFailure>> = None;
                let mut seen_failures = false;
                let mut f_successful: Option<crate::types::common::Uint> = None;
                let mut f_total: Option<crate::types::common::Uint> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "failed" => {
                            if f_failed.is_some() {
                                return Err(serde::de::Error::duplicate_field("failed"));
                            }
                            f_failed = Some(map.next_value::<crate::types::common::Uint>()?);
                        }
                        "failures" => {
                            if seen_failures {
                                return Err(serde::de::Error::duplicate_field("failures"));
                            }
                            seen_failures = true;
                            f_failures = map
                                .next_value::<Option<Vec<crate::types::common::ShardFailure>>>()?;
                        }
                        "successful" => {
                            if f_successful.is_some() {
                                return Err(serde::de::Error::duplicate_field("successful"));
                            }
                            f_successful = Some(map.next_value::<crate::types::common::Uint>()?);
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(map.next_value::<crate::types::common::Uint>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardInfo {
                    failed: f_failed.ok_or_else(|| serde::de::Error::missing_field("failed"))?,
                    failures: f_failures,
                    successful: f_successful
                        .ok_or_else(|| serde::de::Error::missing_field("successful"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("failed", &self.failed)?;
        if let Some(v) = &self.failures {
            map.serialize_entry("failures", v)?;
        }
        map.serialize_entry("successful", &self.successful)?;
        map.serialize_entry("total", &self.total)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Represents a failure to search on a specific shard. Used in search responses."]
#[derive(Debug, Clone)]
pub struct ShardSearchFailure {
    #[doc = "The index name where the failure occurred."]
    pub index: Option<String>,
    #[doc = "The node id where the failure occurred."]
    pub node: Option<String>,
    pub reason: crate::types::common::ErrorCause,
    #[doc = "The shard id where the failure occurred."]
    pub shard: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardSearchFailure {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardSearchFailure;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardSearchFailure object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_index: Option<String> = None;
                let mut seen_index = false;
                let mut f_node: Option<String> = None;
                let mut seen_node = false;
                let mut f_reason: Option<crate::types::common::ErrorCause> = None;
                let mut f_shard: Option<i64> = None;
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
                            f_index = map.next_value::<Option<String>>()?;
                        }
                        "node" => {
                            if seen_node {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            seen_node = true;
                            f_node = map.next_value::<Option<String>>()?;
                        }
                        "reason" => {
                            if f_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            f_reason = Some(map.next_value::<crate::types::common::ErrorCause>()?);
                        }
                        "shard" => {
                            if f_shard.is_some() {
                                return Err(serde::de::Error::duplicate_field("shard"));
                            }
                            f_shard = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardSearchFailure {
                    index: f_index,
                    node: f_node,
                    reason: f_reason.ok_or_else(|| serde::de::Error::missing_field("reason"))?,
                    shard: f_shard.ok_or_else(|| serde::de::Error::missing_field("shard"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardSearchFailure {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.node {
            map.serialize_entry("node", v)?;
        }
        map.serialize_entry("reason", &self.reason)?;
        map.serialize_entry("shard", &self.shard)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardStatistics {
    pub failed: crate::types::common::Uint,
    pub failures: Option<Vec<crate::types::common::ShardSearchFailure>>,
    pub skipped: Option<crate::types::common::Uint>,
    pub successful: crate::types::common::Uint,
    pub total: crate::types::common::Uint,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardStatistics {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardStatistics;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardStatistics object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_failed: Option<crate::types::common::Uint> = None;
                let mut f_failures: Option<Vec<crate::types::common::ShardSearchFailure>> = None;
                let mut seen_failures = false;
                let mut f_skipped: Option<crate::types::common::Uint> = None;
                let mut seen_skipped = false;
                let mut f_successful: Option<crate::types::common::Uint> = None;
                let mut f_total: Option<crate::types::common::Uint> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "failed" => {
                            if f_failed.is_some() {
                                return Err(serde::de::Error::duplicate_field("failed"));
                            }
                            f_failed = Some(map.next_value::<crate::types::common::Uint>()?);
                        }
                        "failures" => {
                            if seen_failures {
                                return Err(serde::de::Error::duplicate_field("failures"));
                            }
                            seen_failures = true;
                            f_failures = map.next_value::<Option<Vec<crate::types::common::ShardSearchFailure>>>()?;
                        }
                        "skipped" => {
                            if seen_skipped {
                                return Err(serde::de::Error::duplicate_field("skipped"));
                            }
                            seen_skipped = true;
                            f_skipped = map.next_value::<Option<crate::types::common::Uint>>()?;
                        }
                        "successful" => {
                            if f_successful.is_some() {
                                return Err(serde::de::Error::duplicate_field("successful"));
                            }
                            f_successful = Some(map.next_value::<crate::types::common::Uint>()?);
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(map.next_value::<crate::types::common::Uint>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardStatistics {
                    failed: f_failed.ok_or_else(|| serde::de::Error::missing_field("failed"))?,
                    failures: f_failures,
                    skipped: f_skipped,
                    successful: f_successful
                        .ok_or_else(|| serde::de::Error::missing_field("successful"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardStatistics {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("failed", &self.failed)?;
        if let Some(v) = &self.failures {
            map.serialize_entry("failures", v)?;
        }
        if let Some(v) = &self.skipped {
            map.serialize_entry("skipped", v)?;
        }
        map.serialize_entry("successful", &self.successful)?;
        map.serialize_entry("total", &self.total)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The base response for operations involving shards."]
#[derive(Debug, Clone)]
pub struct ShardsOperationResponseBase {
    pub shards: crate::types::common::ShardStatistics,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardsOperationResponseBase {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardsOperationResponseBase;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardsOperationResponseBase object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_shards: Option<crate::types::common::ShardStatistics> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_shards" => {
                            if f_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("_shards"));
                            }
                            f_shards =
                                Some(map.next_value::<crate::types::common::ShardStatistics>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardsOperationResponseBase {
                    shards: f_shards.ok_or_else(|| serde::de::Error::missing_field("_shards"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardsOperationResponseBase {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("_shards", &self.shards)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The short integer value."]
pub type Short = f64;

#[doc = "The configuration for a sliced scroll request."]
#[derive(Debug, Clone)]
pub struct SlicedScroll {
    #[doc = "The field to slice on."]
    pub field: Option<crate::types::common::Field>,
    #[doc = "The ID of the slice."]
    pub id: i32,
    #[doc = "The maximum number of slices."]
    pub max: i32,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SlicedScroll {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SlicedScroll;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SlicedScroll object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_field: Option<crate::types::common::Field> = None;
                let mut seen_field = false;
                let mut f_id: Option<i32> = None;
                let mut f_max: Option<i32> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "field" => {
                            if seen_field {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            seen_field = true;
                            f_field = map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        "id" => {
                            if f_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            f_id = Some(map.next_value::<i32>()?);
                        }
                        "max" => {
                            if f_max.is_some() {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            f_max = Some(map.next_value::<i32>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SlicedScroll {
                    field: f_field,
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("id"))?,
                    max: f_max.ok_or_else(|| serde::de::Error::missing_field("max"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SlicedScroll {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.field {
            map.serialize_entry("field", v)?;
        }
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("max", &self.max)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SortMode {
    #[doc = "`avg`"]
    Avg,
    #[doc = "`max`"]
    Max,
    #[doc = "`median`"]
    Median,
    #[doc = "`min`"]
    Min,
    #[doc = "`sum`"]
    Sum,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SortMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SortMode::Avg => "avg",
            SortMode::Max => "max",
            SortMode::Median => "median",
            SortMode::Min => "min",
            SortMode::Sum => "sum",
            SortMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SortMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SortMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "avg" => SortMode::Avg,
            "max" => SortMode::Max,
            "median" => SortMode::Median,
            "min" => SortMode::Min,
            "sum" => SortMode::Sum,
            _ => SortMode::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SortOptions {
    pub geo_distance: Option<crate::types::common::GeoDistanceSort>,
    pub score: Option<crate::types::common::ScoreSort>,
    pub script: Option<crate::types::common::ScriptSort>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SortOptions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SortOptions;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SortOptions object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_geo_distance: Option<crate::types::common::GeoDistanceSort> = None;
                let mut seen_geo_distance = false;
                let mut f_score: Option<crate::types::common::ScoreSort> = None;
                let mut seen_score = false;
                let mut f_script: Option<crate::types::common::ScriptSort> = None;
                let mut seen_script = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_geo_distance" => {
                            if seen_geo_distance {
                                return Err(serde::de::Error::duplicate_field("_geo_distance"));
                            }
                            seen_geo_distance = true;
                            f_geo_distance =
                                map.next_value::<Option<crate::types::common::GeoDistanceSort>>()?;
                        }
                        "_score" => {
                            if seen_score {
                                return Err(serde::de::Error::duplicate_field("_score"));
                            }
                            seen_score = true;
                            f_score =
                                map.next_value::<Option<crate::types::common::ScoreSort>>()?;
                        }
                        "_script" => {
                            if seen_script {
                                return Err(serde::de::Error::duplicate_field("_script"));
                            }
                            seen_script = true;
                            f_script =
                                map.next_value::<Option<crate::types::common::ScriptSort>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SortOptions {
                    geo_distance: f_geo_distance,
                    score: f_score,
                    script: f_script,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SortOptions {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.geo_distance {
            map.serialize_entry("_geo_distance", v)?;
        }
        if let Some(v) = &self.score {
            map.serialize_entry("_score", v)?;
        }
        if let Some(v) = &self.script {
            map.serialize_entry("_script", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The direction of the sort order."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SortOrder {
    #[doc = "`asc`"]
    Asc,
    #[doc = "`desc`"]
    Desc,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SortOrder {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SortOrder::Asc => "asc",
            SortOrder::Desc => "desc",
            SortOrder::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SortOrder {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SortOrder {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "asc" => SortOrder::Asc,
            "desc" => SortOrder::Desc,
            _ => SortOrder::Unknown(s),
        })
    }
}

pub type SortResults = Vec<crate::types::common::FieldValue>;

#[derive(Debug, Clone)]
pub struct StoreStats {
    pub reserved: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "A prediction, in bytes, of how much larger the shard stores will eventually grow due to ongoing peer recoveries, restoring snapshots, and similar activities."]
    pub reserved_in_bytes: crate::types::common::ByteCount,
    pub size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total size, in bytes, of all shards assigned to the selected nodes."]
    pub size_in_bytes: crate::types::common::ByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StoreStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StoreStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StoreStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_reserved: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_reserved = false;
                let mut f_reserved_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_size: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_size = false;
                let mut f_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "reserved" => {
                            if seen_reserved {
                                return Err(serde::de::Error::duplicate_field("reserved"));
                            }
                            seen_reserved = true;
                            f_reserved = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "reserved_in_bytes" => {
                            if f_reserved_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("reserved_in_bytes"));
                            }
                            f_reserved_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
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
                Ok(StoreStats {
                    reserved: f_reserved,
                    reserved_in_bytes: f_reserved_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("reserved_in_bytes"))?,
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
impl serde::Serialize for StoreStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.reserved {
            map.serialize_entry("reserved", v)?;
        }
        map.serialize_entry("reserved_in_bytes", &self.reserved_in_bytes)?;
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

#[doc = "The definition of a stored script."]
#[derive(Debug, Clone)]
pub struct StoredScript {
    pub lang: crate::types::common::ScriptLanguage,
    #[doc = "The options for the script."]
    pub options: Option<std::collections::BTreeMap<String, String>>,
    #[doc = "The location of the source code for the script."]
    pub source: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StoredScript {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StoredScript;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StoredScript object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_lang: Option<crate::types::common::ScriptLanguage> = None;
                let mut f_options: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_options = false;
                let mut f_source: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "lang" => {
                            if f_lang.is_some() {
                                return Err(serde::de::Error::duplicate_field("lang"));
                            }
                            f_lang =
                                Some(map.next_value::<crate::types::common::ScriptLanguage>()?);
                        }
                        "options" => {
                            if seen_options {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            seen_options = true;
                            f_options = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "source" => {
                            if f_source.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            f_source = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StoredScript {
                    lang: f_lang.ok_or_else(|| serde::de::Error::missing_field("lang"))?,
                    options: f_options,
                    source: f_source.ok_or_else(|| serde::de::Error::missing_field("source"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StoredScript {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("lang", &self.lang)?;
        if let Some(v) = &self.options {
            map.serialize_entry("options", v)?;
        }
        map.serialize_entry("source", &self.source)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct StoredScriptId {
    #[doc = "Specifies any named parameters that are passed into the script as variables. Use parameters instead of hard-coded values to decrease compilation time."]
    pub params: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub id: crate::types::common::Id,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StoredScriptId {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StoredScriptId;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StoredScriptId object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_params: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_params = false;
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "params" => {
                            if seen_params {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            seen_params = true;
                            f_params = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "id" => {
                            if f_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            f_id = Some(map.next_value::<crate::types::common::Id>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StoredScriptId {
                    params: f_params,
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("id"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StoredScriptId {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.params {
            map.serialize_entry("params", v)?;
        }
        map.serialize_entry("id", &self.id)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'"') => serde_json::from_str(text).map(StringOrStringArray::String),
            Some(b'[') => serde_json::from_str(text).map(StringOrStringArray::Array),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a StringOrStringArray value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b't') | Some(b'f') => serde_json::from_str(text).map(StringifiedBoolean::Boolean),
            Some(b'"') => serde_json::from_str(text).map(StringifiedBoolean::String),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a StringifiedBoolean value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(StringifiedDouble::Number)
            }
            Some(b'"') => serde_json::from_str(text).map(StringifiedDouble::String),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a StringifiedDouble value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(StringifiedEpochTimeUnitMillis::EpochTimeUnitMillis)
            }
            Some(b'"') => serde_json::from_str(text).map(StringifiedEpochTimeUnitMillis::String),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a StringifiedEpochTimeUnitMillis value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'-') | Some(b'0'..=b'9') => serde_json::from_str(text)
                .map(StringifiedEpochTimeUnitSeconds::EpochTimeUnitSeconds),
            Some(b'"') => serde_json::from_str(text).map(StringifiedEpochTimeUnitSeconds::String),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a StringifiedEpochTimeUnitSeconds value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "Certain APIs may return values, including numbers such as epoch timestamps, as strings. This setting captures this behavior while keeping the semantics of the field type.  Depending on the target language, code generators can keep the union or remove it and leniently parse strings to the target type."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum StringifiedInteger {
    Integer(i64),
    String(String),
}
impl<'de> serde::Deserialize<'de> for StringifiedInteger {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(StringifiedInteger::Integer)
            }
            Some(b'"') => serde_json::from_str(text).map(StringifiedInteger::String),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a StringifiedInteger value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(StringifiedLong::Integer)
            }
            Some(b'"') => serde_json::from_str(text).map(StringifiedLong::String),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a StringifiedLong value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(StringifiedVersionNumber::VersionNumber)
            }
            Some(b'"') => serde_json::from_str(text).map(StringifiedVersionNumber::String),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a StringifiedVersionNumber value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SuggestMode {
    #[doc = "`always`"]
    Always,
    #[doc = "`missing`"]
    Missing,
    #[doc = "`popular`"]
    Popular,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SuggestMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SuggestMode::Always => "always",
            SuggestMode::Missing => "missing",
            SuggestMode::Popular => "popular",
            SuggestMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SuggestMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SuggestMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "always" => SuggestMode::Always,
            "missing" => SuggestMode::Missing,
            "popular" => SuggestMode::Popular,
            _ => SuggestMode::Unknown(s),
        })
    }
}

pub type TDocument = Box<serde_json::value::RawValue>;

#[doc = "The details of a task failure."]
#[derive(Debug, Clone)]
pub struct TaskFailure {
    pub node_id: crate::types::common::NodeId,
    pub reason: crate::types::common::ErrorCause,
    #[doc = "The status of the failed task."]
    pub status: String,
    #[doc = "The ID of the failed task."]
    pub task_id: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TaskFailure {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TaskFailure;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TaskFailure object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_node_id: Option<crate::types::common::NodeId> = None;
                let mut f_reason: Option<crate::types::common::ErrorCause> = None;
                let mut f_status: Option<String> = None;
                let mut f_task_id: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "node_id" => {
                            if f_node_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("node_id"));
                            }
                            f_node_id = Some(map.next_value::<crate::types::common::NodeId>()?);
                        }
                        "reason" => {
                            if f_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            f_reason = Some(map.next_value::<crate::types::common::ErrorCause>()?);
                        }
                        "status" => {
                            if f_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            f_status = Some(map.next_value::<String>()?);
                        }
                        "task_id" => {
                            if f_task_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("task_id"));
                            }
                            f_task_id = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TaskFailure {
                    node_id: f_node_id.ok_or_else(|| serde::de::Error::missing_field("node_id"))?,
                    reason: f_reason.ok_or_else(|| serde::de::Error::missing_field("reason"))?,
                    status: f_status.ok_or_else(|| serde::de::Error::missing_field("status"))?,
                    task_id: f_task_id.ok_or_else(|| serde::de::Error::missing_field("task_id"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TaskFailure {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("node_id", &self.node_id)?;
        map.serialize_entry("reason", &self.reason)?;
        map.serialize_entry("status", &self.status)?;
        map.serialize_entry("task_id", &self.task_id)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The unique identifier of a task."]
pub type TaskId = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TermFrequencyNormalization {
    #[doc = "`h1`"]
    H1,
    #[doc = "`h2`"]
    H2,
    #[doc = "`h3`"]
    H3,
    #[doc = "`no`"]
    No,
    #[doc = "`z`"]
    Z,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TermFrequencyNormalization {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TermFrequencyNormalization::H1 => "h1",
            TermFrequencyNormalization::H2 => "h2",
            TermFrequencyNormalization::H3 => "h3",
            TermFrequencyNormalization::No => "no",
            TermFrequencyNormalization::Z => "z",
            TermFrequencyNormalization::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TermFrequencyNormalization {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TermFrequencyNormalization {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "h1" => TermFrequencyNormalization::H1,
            "h2" => TermFrequencyNormalization::H2,
            "h3" => TermFrequencyNormalization::H3,
            "no" => TermFrequencyNormalization::No,
            "z" => TermFrequencyNormalization::Z,
            _ => TermFrequencyNormalization::Unknown(s),
        })
    }
}

#[doc = "The information about thread usage."]
#[derive(Debug, Clone)]
pub struct ThreadInfo {
    #[doc = "The number of currently active threads."]
    pub active_threads: f64,
    #[doc = "The number of thread executions."]
    pub thread_executions: f64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ThreadInfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ThreadInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ThreadInfo object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_active_threads: Option<f64> = None;
                let mut f_thread_executions: Option<f64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "active_threads" => {
                            if f_active_threads.is_some() {
                                return Err(serde::de::Error::duplicate_field("active_threads"));
                            }
                            f_active_threads = Some(map.next_value::<f64>()?);
                        }
                        "thread_executions" => {
                            if f_thread_executions.is_some() {
                                return Err(serde::de::Error::duplicate_field("thread_executions"));
                            }
                            f_thread_executions = Some(map.next_value::<f64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ThreadInfo {
                    active_threads: f_active_threads
                        .ok_or_else(|| serde::de::Error::missing_field("active_threads"))?,
                    thread_executions: f_thread_executions
                        .ok_or_else(|| serde::de::Error::missing_field("thread_executions"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ThreadInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("active_threads", &self.active_threads)?;
        map.serialize_entry("thread_executions", &self.thread_executions)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Time of day, expressed as HH:MM:SS."]
pub type TimeOfDay = String;

#[doc = "The time zone identifier."]
pub type TimeZone = String;

#[doc = "The geographical bounds specified using upper-left and lower-right points."]
#[derive(Debug, Clone)]
pub struct TopLeftBottomRightGeoBounds {
    #[doc = "The lower-right corner coordinates."]
    pub bottom_right: crate::types::common::GeoLocation,
    #[doc = "The upper-left corner coordinates."]
    pub top_left: crate::types::common::GeoLocation,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TopLeftBottomRightGeoBounds {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TopLeftBottomRightGeoBounds;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TopLeftBottomRightGeoBounds object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_bottom_right: Option<crate::types::common::GeoLocation> = None;
                let mut f_top_left: Option<crate::types::common::GeoLocation> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "bottom_right" => {
                            if f_bottom_right.is_some() {
                                return Err(serde::de::Error::duplicate_field("bottom_right"));
                            }
                            f_bottom_right =
                                Some(map.next_value::<crate::types::common::GeoLocation>()?);
                        }
                        "top_left" => {
                            if f_top_left.is_some() {
                                return Err(serde::de::Error::duplicate_field("top_left"));
                            }
                            f_top_left =
                                Some(map.next_value::<crate::types::common::GeoLocation>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TopLeftBottomRightGeoBounds {
                    bottom_right: f_bottom_right
                        .ok_or_else(|| serde::de::Error::missing_field("bottom_right"))?,
                    top_left: f_top_left
                        .ok_or_else(|| serde::de::Error::missing_field("top_left"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TopLeftBottomRightGeoBounds {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("bottom_right", &self.bottom_right)?;
        map.serialize_entry("top_left", &self.top_left)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The geographical bounds specified using upper-right and lower-left points."]
#[derive(Debug, Clone)]
pub struct TopRightBottomLeftGeoBounds {
    #[doc = "The lower-left corner coordinates."]
    pub bottom_left: crate::types::common::GeoLocation,
    #[doc = "The upper-right corner coordinates."]
    pub top_right: crate::types::common::GeoLocation,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TopRightBottomLeftGeoBounds {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TopRightBottomLeftGeoBounds;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TopRightBottomLeftGeoBounds object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_bottom_left: Option<crate::types::common::GeoLocation> = None;
                let mut f_top_right: Option<crate::types::common::GeoLocation> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "bottom_left" => {
                            if f_bottom_left.is_some() {
                                return Err(serde::de::Error::duplicate_field("bottom_left"));
                            }
                            f_bottom_left =
                                Some(map.next_value::<crate::types::common::GeoLocation>()?);
                        }
                        "top_right" => {
                            if f_top_right.is_some() {
                                return Err(serde::de::Error::duplicate_field("top_right"));
                            }
                            f_top_right =
                                Some(map.next_value::<crate::types::common::GeoLocation>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TopRightBottomLeftGeoBounds {
                    bottom_left: f_bottom_left
                        .ok_or_else(|| serde::de::Error::missing_field("bottom_left"))?,
                    top_right: f_top_right
                        .ok_or_else(|| serde::de::Error::missing_field("top_right"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TopRightBottomLeftGeoBounds {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("bottom_left", &self.bottom_left)?;
        map.serialize_entry("top_right", &self.top_right)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The statistics about translog operations."]
#[derive(Debug, Clone)]
pub struct TranslogStats {
    #[doc = "The age of the oldest translog entry."]
    pub earliest_last_modified_age: i64,
    #[doc = "The number of operations in the translog."]
    pub operations: i32,
    pub remote_store: Option<crate::types::common::RemoteStoreTranslogStats>,
    #[doc = "The size of the translog in a human-readable format."]
    pub size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The size of the translog in bytes."]
    pub size_in_bytes: crate::types::common::ByteCount,
    #[doc = "The number of uncommitted operations in the translog."]
    pub uncommitted_operations: i32,
    #[doc = "The size of uncommitted operations in a human-readable format."]
    pub uncommitted_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The size of uncommitted operations in bytes."]
    pub uncommitted_size_in_bytes: crate::types::common::ByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TranslogStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TranslogStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TranslogStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_earliest_last_modified_age: Option<i64> = None;
                let mut f_operations: Option<i32> = None;
                let mut f_remote_store: Option<crate::types::common::RemoteStoreTranslogStats> =
                    None;
                let mut seen_remote_store = false;
                let mut f_size: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_size = false;
                let mut f_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_uncommitted_operations: Option<i32> = None;
                let mut f_uncommitted_size: Option<crate::types::common::HumanReadableByteCount> =
                    None;
                let mut seen_uncommitted_size = false;
                let mut f_uncommitted_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "earliest_last_modified_age" => {
                            if f_earliest_last_modified_age.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "earliest_last_modified_age",
                                ));
                            }
                            f_earliest_last_modified_age = Some(map.next_value::<i64>()?);
                        }
                        "operations" => {
                            if f_operations.is_some() {
                                return Err(serde::de::Error::duplicate_field("operations"));
                            }
                            f_operations = Some(map.next_value::<i32>()?);
                        }
                        "remote_store" => {
                            if seen_remote_store {
                                return Err(serde::de::Error::duplicate_field("remote_store"));
                            }
                            seen_remote_store = true;
                            f_remote_store = map.next_value::<Option<crate::types::common::RemoteStoreTranslogStats>>()?;
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
                        "uncommitted_operations" => {
                            if f_uncommitted_operations.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "uncommitted_operations",
                                ));
                            }
                            f_uncommitted_operations = Some(map.next_value::<i32>()?);
                        }
                        "uncommitted_size" => {
                            if seen_uncommitted_size {
                                return Err(serde::de::Error::duplicate_field("uncommitted_size"));
                            }
                            seen_uncommitted_size = true;
                            f_uncommitted_size = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "uncommitted_size_in_bytes" => {
                            if f_uncommitted_size_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "uncommitted_size_in_bytes",
                                ));
                            }
                            f_uncommitted_size_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TranslogStats {
                    earliest_last_modified_age: f_earliest_last_modified_age.ok_or_else(|| {
                        serde::de::Error::missing_field("earliest_last_modified_age")
                    })?,
                    operations: f_operations
                        .ok_or_else(|| serde::de::Error::missing_field("operations"))?,
                    remote_store: f_remote_store,
                    size: f_size,
                    size_in_bytes: f_size_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("size_in_bytes"))?,
                    uncommitted_operations: f_uncommitted_operations
                        .ok_or_else(|| serde::de::Error::missing_field("uncommitted_operations"))?,
                    uncommitted_size: f_uncommitted_size,
                    uncommitted_size_in_bytes: f_uncommitted_size_in_bytes.ok_or_else(|| {
                        serde::de::Error::missing_field("uncommitted_size_in_bytes")
                    })?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TranslogStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry(
            "earliest_last_modified_age",
            &self.earliest_last_modified_age,
        )?;
        map.serialize_entry("operations", &self.operations)?;
        if let Some(v) = &self.remote_store {
            map.serialize_entry("remote_store", v)?;
        }
        if let Some(v) = &self.size {
            map.serialize_entry("size", v)?;
        }
        map.serialize_entry("size_in_bytes", &self.size_in_bytes)?;
        map.serialize_entry("uncommitted_operations", &self.uncommitted_operations)?;
        if let Some(v) = &self.uncommitted_size {
            map.serialize_entry("uncommitted_size", v)?;
        }
        map.serialize_entry("uncommitted_size_in_bytes", &self.uncommitted_size_in_bytes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The transport address of a node."]
pub type TransportAddress = String;

#[doc = "The type of document or resource."]
pub type Type = String;

pub type Uint = i64;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VersionType {
    #[doc = "`external`"]
    External,
    #[doc = "`external_gte`"]
    ExternalGte,
    #[doc = "`internal`"]
    Internal,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl VersionType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            VersionType::External => "external",
            VersionType::ExternalGte => "external_gte",
            VersionType::Internal => "internal",
            VersionType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for VersionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VersionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "external" => VersionType::External,
            "external_gte" => VersionType::ExternalGte,
            "internal" => VersionType::Internal,
            _ => VersionType::Unknown(s),
        })
    }
}

#[doc = "The absence of any type. This is commonly used in APIs that don't return a body.  Although \"void\" is generally used for a unit type that has only one value, this is interpreted as the bottom type, which has no value. Most languages have a unit type, but few have a bottom type.  See https://en.m.wikipedia.org/wiki/Unit_type and https://en.m.wikipedia.org/wiki/Bottom_type."]
pub type Void = std::collections::BTreeMap<String, serde_json::Value>;

#[doc = "The statistics about index warmer operations."]
#[derive(Debug, Clone)]
pub struct WarmerStats {
    #[doc = "The number of current warmer operations."]
    pub current: i64,
    #[doc = "The total number of warmer operations."]
    pub total: i64,
    #[doc = "The total time spent on warmer operations."]
    pub total_time: Option<crate::types::common::Duration>,
    #[doc = "The total time spent on warmer operations in milliseconds."]
    pub total_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for WarmerStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = WarmerStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a WarmerStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_current: Option<i64> = None;
                let mut f_total: Option<i64> = None;
                let mut f_total_time: Option<crate::types::common::Duration> = None;
                let mut seen_total_time = false;
                let mut f_total_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
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
                            f_current = Some(map.next_value::<i64>()?);
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(map.next_value::<i64>()?);
                        }
                        "total_time" => {
                            if seen_total_time {
                                return Err(serde::de::Error::duplicate_field("total_time"));
                            }
                            seen_total_time = true;
                            f_total_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "total_time_in_millis" => {
                            if f_total_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_time_in_millis",
                                ));
                            }
                            f_total_time_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(WarmerStats {
                    current: f_current.ok_or_else(|| serde::de::Error::missing_field("current"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    total_time: f_total_time,
                    total_time_in_millis: f_total_time_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("total_time_in_millis"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for WarmerStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("current", &self.current)?;
        map.serialize_entry("total", &self.total)?;
        if let Some(v) = &self.total_time {
            map.serialize_entry("total_time", v)?;
        }
        map.serialize_entry("total_time_in_millis", &self.total_time_in_millis)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The geographical bounds specified using Well-Known Text (WKT) format."]
#[derive(Debug, Clone)]
pub struct WktGeoBounds {
    #[doc = "The WKT string representation of the geographical bounds."]
    pub wkt: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for WktGeoBounds {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = WktGeoBounds;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a WktGeoBounds object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_wkt: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "wkt" => {
                            if f_wkt.is_some() {
                                return Err(serde::de::Error::duplicate_field("wkt"));
                            }
                            f_wkt = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(WktGeoBounds {
                    wkt: f_wkt.ok_or_else(|| serde::de::Error::missing_field("wkt"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for WktGeoBounds {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("wkt", &self.wkt)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The base response for write operations."]
#[derive(Debug, Clone)]
pub struct WriteResponseBase {
    pub id: crate::types::common::Id,
    pub index: crate::types::common::IndexName,
    #[doc = "The primary term of the document."]
    pub primary_term: i64,
    pub seq_no: crate::types::common::SequenceNumber,
    pub shards: crate::types::common::ShardStatistics,
    pub r#type: Option<crate::types::common::Type>,
    pub version: crate::types::common::VersionNumber,
    pub forced_refresh: Option<bool>,
    pub result: crate::types::common::Result,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for WriteResponseBase {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = WriteResponseBase;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a WriteResponseBase object")
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
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut f_primary_term: Option<i64> = None;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut f_shards: Option<crate::types::common::ShardStatistics> = None;
                let mut f_type: Option<crate::types::common::Type> = None;
                let mut seen_type = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut f_forced_refresh: Option<bool> = None;
                let mut seen_forced_refresh = false;
                let mut f_result: Option<crate::types::common::Result> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_id" => {
                            if f_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("_id"));
                            }
                            f_id = Some(map.next_value::<crate::types::common::Id>()?);
                        }
                        "_index" => {
                            if f_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("_index"));
                            }
                            f_index = Some(map.next_value::<crate::types::common::IndexName>()?);
                        }
                        "_primary_term" => {
                            if f_primary_term.is_some() {
                                return Err(serde::de::Error::duplicate_field("_primary_term"));
                            }
                            f_primary_term = Some(map.next_value::<i64>()?);
                        }
                        "_seq_no" => {
                            if f_seq_no.is_some() {
                                return Err(serde::de::Error::duplicate_field("_seq_no"));
                            }
                            f_seq_no =
                                Some(map.next_value::<crate::types::common::SequenceNumber>()?);
                        }
                        "_shards" => {
                            if f_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("_shards"));
                            }
                            f_shards =
                                Some(map.next_value::<crate::types::common::ShardStatistics>()?);
                        }
                        "_type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<crate::types::common::Type>>()?;
                        }
                        "_version" => {
                            if f_version.is_some() {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            f_version =
                                Some(map.next_value::<crate::types::common::VersionNumber>()?);
                        }
                        "forced_refresh" => {
                            if seen_forced_refresh {
                                return Err(serde::de::Error::duplicate_field("forced_refresh"));
                            }
                            seen_forced_refresh = true;
                            f_forced_refresh = map.next_value::<Option<bool>>()?;
                        }
                        "result" => {
                            if f_result.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            f_result = Some(map.next_value::<crate::types::common::Result>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(WriteResponseBase {
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("_id"))?,
                    index: f_index.ok_or_else(|| serde::de::Error::missing_field("_index"))?,
                    primary_term: f_primary_term
                        .ok_or_else(|| serde::de::Error::missing_field("_primary_term"))?,
                    seq_no: f_seq_no.ok_or_else(|| serde::de::Error::missing_field("_seq_no"))?,
                    shards: f_shards.ok_or_else(|| serde::de::Error::missing_field("_shards"))?,
                    r#type: f_type,
                    version: f_version
                        .ok_or_else(|| serde::de::Error::missing_field("_version"))?,
                    forced_refresh: f_forced_refresh,
                    result: f_result.ok_or_else(|| serde::de::Error::missing_field("result"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for WriteResponseBase {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("_id", &self.id)?;
        map.serialize_entry("_index", &self.index)?;
        map.serialize_entry("_primary_term", &self.primary_term)?;
        map.serialize_entry("_seq_no", &self.seq_no)?;
        map.serialize_entry("_shards", &self.shards)?;
        if let Some(v) = &self.r#type {
            map.serialize_entry("_type", v)?;
        }
        map.serialize_entry("_version", &self.version)?;
        if let Some(v) = &self.forced_refresh {
            map.serialize_entry("forced_refresh", v)?;
        }
        map.serialize_entry("result", &self.result)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The Cartesian coordinates specified using x and y values."]
#[derive(Debug, Clone)]
pub struct XyCartesianCoordinates {
    #[doc = "The x coordinate."]
    pub x: f64,
    #[doc = "The y coordinate."]
    pub y: f64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for XyCartesianCoordinates {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = XyCartesianCoordinates;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a XyCartesianCoordinates object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_x: Option<f64> = None;
                let mut f_y: Option<f64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "x" => {
                            if f_x.is_some() {
                                return Err(serde::de::Error::duplicate_field("x"));
                            }
                            f_x = Some(map.next_value::<f64>()?);
                        }
                        "y" => {
                            if f_y.is_some() {
                                return Err(serde::de::Error::duplicate_field("y"));
                            }
                            f_y = Some(map.next_value::<f64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(XyCartesianCoordinates {
                    x: f_x.ok_or_else(|| serde::de::Error::missing_field("x"))?,
                    y: f_y.ok_or_else(|| serde::de::Error::missing_field("y"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for XyCartesianCoordinates {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("x", &self.x)?;
        map.serialize_entry("y", &self.y)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'{') => serde_json::from_str(text).map(XyLocation::Cartesian),
            Some(b'[') => serde_json::from_str(text).map(XyLocation::Coords),
            Some(b'"') => serde_json::from_str(text).map(XyLocation::Text),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a XyLocation value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}
