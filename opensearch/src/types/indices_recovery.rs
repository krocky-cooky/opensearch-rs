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

//! Response types of the `indices_recovery` specification namespace

#![allow(missing_docs)]

#[doc = "The details of a recovered file."]
#[derive(Debug, Clone)]
pub struct FileDetails {
    #[doc = "The length of the file in bytes."]
    pub length: i64,
    #[doc = "The name of the file."]
    pub name: String,
    #[doc = "The number of bytes recovered."]
    pub recovered: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FileDetails {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FileDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FileDetails object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_length: Option<i64> = None;
                let mut f_name: Option<String> = None;
                let mut f_recovered: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "length" => {
                            if f_length.is_some() {
                                return Err(serde::de::Error::duplicate_field("length"));
                            }
                            f_length = Some(map.next_value::<i64>()?);
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<String>()?);
                        }
                        "recovered" => {
                            if f_recovered.is_some() {
                                return Err(serde::de::Error::duplicate_field("recovered"));
                            }
                            f_recovered = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FileDetails {
                    length: f_length.ok_or_else(|| serde::de::Error::missing_field("length"))?,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    recovered: f_recovered
                        .ok_or_else(|| serde::de::Error::missing_field("recovered"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FileDetails {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("length", &self.length)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("recovered", &self.recovered)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The byte-level recovery statistics."]
#[derive(Debug, Clone)]
pub struct RecoveryBytes {
    pub percent: crate::types::common::PercentageString,
    pub recovered: Option<crate::types::common::HumanReadableByteCount>,
    pub recovered_from_snapshot: Option<crate::types::common::HumanReadableByteCount>,
    pub recovered_from_snapshot_in_bytes: Option<crate::types::common::ByteCount>,
    pub recovered_in_bytes: crate::types::common::ByteCount,
    pub reused: Option<crate::types::common::HumanReadableByteCount>,
    pub reused_in_bytes: crate::types::common::ByteCount,
    pub total: Option<crate::types::common::HumanReadableByteCount>,
    pub total_in_bytes: crate::types::common::ByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RecoveryBytes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RecoveryBytes;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RecoveryBytes object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_percent: Option<crate::types::common::PercentageString> = None;
                let mut f_recovered: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_recovered = false;
                let mut f_recovered_from_snapshot: Option<
                    crate::types::common::HumanReadableByteCount,
                > = None;
                let mut seen_recovered_from_snapshot = false;
                let mut f_recovered_from_snapshot_in_bytes: Option<
                    crate::types::common::ByteCount,
                > = None;
                let mut seen_recovered_from_snapshot_in_bytes = false;
                let mut f_recovered_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_reused: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_reused = false;
                let mut f_reused_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_total: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_total = false;
                let mut f_total_in_bytes: Option<crate::types::common::ByteCount> = None;
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
                                Some(map.next_value::<crate::types::common::PercentageString>()?);
                        }
                        "recovered" => {
                            if seen_recovered {
                                return Err(serde::de::Error::duplicate_field("recovered"));
                            }
                            seen_recovered = true;
                            f_recovered = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "recovered_from_snapshot" => {
                            if seen_recovered_from_snapshot {
                                return Err(serde::de::Error::duplicate_field(
                                    "recovered_from_snapshot",
                                ));
                            }
                            seen_recovered_from_snapshot = true;
                            f_recovered_from_snapshot = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "recovered_from_snapshot_in_bytes" => {
                            if seen_recovered_from_snapshot_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "recovered_from_snapshot_in_bytes",
                                ));
                            }
                            seen_recovered_from_snapshot_in_bytes = true;
                            f_recovered_from_snapshot_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "recovered_in_bytes" => {
                            if f_recovered_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "recovered_in_bytes",
                                ));
                            }
                            f_recovered_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "reused" => {
                            if seen_reused {
                                return Err(serde::de::Error::duplicate_field("reused"));
                            }
                            seen_reused = true;
                            f_reused = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "reused_in_bytes" => {
                            if f_reused_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("reused_in_bytes"));
                            }
                            f_reused_in_bytes =
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
                Ok(RecoveryBytes {
                    percent: f_percent.ok_or_else(|| serde::de::Error::missing_field("percent"))?,
                    recovered: f_recovered,
                    recovered_from_snapshot: f_recovered_from_snapshot,
                    recovered_from_snapshot_in_bytes: f_recovered_from_snapshot_in_bytes,
                    recovered_in_bytes: f_recovered_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("recovered_in_bytes"))?,
                    reused: f_reused,
                    reused_in_bytes: f_reused_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("reused_in_bytes"))?,
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
impl serde::Serialize for RecoveryBytes {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("percent", &self.percent)?;
        if let Some(v) = &self.recovered {
            map.serialize_entry("recovered", v)?;
        }
        if let Some(v) = &self.recovered_from_snapshot {
            map.serialize_entry("recovered_from_snapshot", v)?;
        }
        if let Some(v) = &self.recovered_from_snapshot_in_bytes {
            map.serialize_entry("recovered_from_snapshot_in_bytes", v)?;
        }
        map.serialize_entry("recovered_in_bytes", &self.recovered_in_bytes)?;
        if let Some(v) = &self.reused {
            map.serialize_entry("reused", v)?;
        }
        map.serialize_entry("reused_in_bytes", &self.reused_in_bytes)?;
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

#[doc = "The file-level recovery statistics."]
#[derive(Debug, Clone)]
pub struct RecoveryFiles {
    #[doc = "The detailed information about recovered files."]
    pub details: Option<Vec<crate::types::indices_recovery::FileDetails>>,
    pub percent: crate::types::common::PercentageString,
    #[doc = "The number of recovered files."]
    pub recovered: i64,
    #[doc = "The number of reused files."]
    pub reused: i64,
    #[doc = "The total number of files."]
    pub total: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RecoveryFiles {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RecoveryFiles;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RecoveryFiles object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_details: Option<Vec<crate::types::indices_recovery::FileDetails>> = None;
                let mut seen_details = false;
                let mut f_percent: Option<crate::types::common::PercentageString> = None;
                let mut f_recovered: Option<i64> = None;
                let mut f_reused: Option<i64> = None;
                let mut f_total: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "details" => {
                            if seen_details {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            seen_details = true;
                            f_details = map.next_value::<Option<Vec<crate::types::indices_recovery::FileDetails>>>()?;
                        }
                        "percent" => {
                            if f_percent.is_some() {
                                return Err(serde::de::Error::duplicate_field("percent"));
                            }
                            f_percent =
                                Some(map.next_value::<crate::types::common::PercentageString>()?);
                        }
                        "recovered" => {
                            if f_recovered.is_some() {
                                return Err(serde::de::Error::duplicate_field("recovered"));
                            }
                            f_recovered = Some(map.next_value::<i64>()?);
                        }
                        "reused" => {
                            if f_reused.is_some() {
                                return Err(serde::de::Error::duplicate_field("reused"));
                            }
                            f_reused = Some(map.next_value::<i64>()?);
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
                Ok(RecoveryFiles {
                    details: f_details,
                    percent: f_percent.ok_or_else(|| serde::de::Error::missing_field("percent"))?,
                    recovered: f_recovered
                        .ok_or_else(|| serde::de::Error::missing_field("recovered"))?,
                    reused: f_reused.ok_or_else(|| serde::de::Error::missing_field("reused"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RecoveryFiles {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.details {
            map.serialize_entry("details", v)?;
        }
        map.serialize_entry("percent", &self.percent)?;
        map.serialize_entry("recovered", &self.recovered)?;
        map.serialize_entry("reused", &self.reused)?;
        map.serialize_entry("total", &self.total)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The index recovery status."]
#[derive(Debug, Clone)]
pub struct RecoveryIndexStatus {
    pub bytes: Option<crate::types::indices_recovery::RecoveryBytes>,
    pub files: crate::types::indices_recovery::RecoveryFiles,
    pub size: crate::types::indices_recovery::RecoveryBytes,
    pub source_throttle_time: Option<crate::types::common::Duration>,
    pub source_throttle_time_in_millis: crate::types::common::DurationValueUnitMillis,
    pub target_throttle_time: Option<crate::types::common::Duration>,
    pub target_throttle_time_in_millis: crate::types::common::DurationValueUnitMillis,
    pub total_time: Option<crate::types::common::Duration>,
    pub total_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RecoveryIndexStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RecoveryIndexStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RecoveryIndexStatus object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_bytes: Option<crate::types::indices_recovery::RecoveryBytes> = None;
                let mut seen_bytes = false;
                let mut f_files: Option<crate::types::indices_recovery::RecoveryFiles> = None;
                let mut f_size: Option<crate::types::indices_recovery::RecoveryBytes> = None;
                let mut f_source_throttle_time: Option<crate::types::common::Duration> = None;
                let mut seen_source_throttle_time = false;
                let mut f_source_throttle_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_target_throttle_time: Option<crate::types::common::Duration> = None;
                let mut seen_target_throttle_time = false;
                let mut f_target_throttle_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
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
                        "bytes" => {
                            if seen_bytes {
                                return Err(serde::de::Error::duplicate_field("bytes"));
                            }
                            seen_bytes = true;
                            f_bytes = map.next_value::<Option<crate::types::indices_recovery::RecoveryBytes>>()?;
                        }
                        "files" => {
                            if f_files.is_some() {
                                return Err(serde::de::Error::duplicate_field("files"));
                            }
                            f_files = Some(
                                map.next_value::<crate::types::indices_recovery::RecoveryFiles>()?,
                            );
                        }
                        "size" => {
                            if f_size.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            f_size = Some(
                                map.next_value::<crate::types::indices_recovery::RecoveryBytes>()?,
                            );
                        }
                        "source_throttle_time" => {
                            if seen_source_throttle_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "source_throttle_time",
                                ));
                            }
                            seen_source_throttle_time = true;
                            f_source_throttle_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "source_throttle_time_in_millis" => {
                            if f_source_throttle_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "source_throttle_time_in_millis",
                                ));
                            }
                            f_source_throttle_time_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "target_throttle_time" => {
                            if seen_target_throttle_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "target_throttle_time",
                                ));
                            }
                            seen_target_throttle_time = true;
                            f_target_throttle_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "target_throttle_time_in_millis" => {
                            if f_target_throttle_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "target_throttle_time_in_millis",
                                ));
                            }
                            f_target_throttle_time_in_millis = Some(
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
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RecoveryIndexStatus {
                    bytes: f_bytes,
                    files: f_files.ok_or_else(|| serde::de::Error::missing_field("files"))?,
                    size: f_size.ok_or_else(|| serde::de::Error::missing_field("size"))?,
                    source_throttle_time: f_source_throttle_time,
                    source_throttle_time_in_millis: f_source_throttle_time_in_millis.ok_or_else(
                        || serde::de::Error::missing_field("source_throttle_time_in_millis"),
                    )?,
                    target_throttle_time: f_target_throttle_time,
                    target_throttle_time_in_millis: f_target_throttle_time_in_millis.ok_or_else(
                        || serde::de::Error::missing_field("target_throttle_time_in_millis"),
                    )?,
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
impl serde::Serialize for RecoveryIndexStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.bytes {
            map.serialize_entry("bytes", v)?;
        }
        map.serialize_entry("files", &self.files)?;
        map.serialize_entry("size", &self.size)?;
        if let Some(v) = &self.source_throttle_time {
            map.serialize_entry("source_throttle_time", v)?;
        }
        map.serialize_entry(
            "source_throttle_time_in_millis",
            &self.source_throttle_time_in_millis,
        )?;
        if let Some(v) = &self.target_throttle_time {
            map.serialize_entry("target_throttle_time", v)?;
        }
        map.serialize_entry(
            "target_throttle_time_in_millis",
            &self.target_throttle_time_in_millis,
        )?;
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

#[doc = "The source or target of a recovery operation."]
#[derive(Debug, Clone)]
pub struct RecoveryOrigin {
    #[doc = "Whether a new history UUID was bootstrapped."]
    pub bootstrap_new_history_uuid: Option<bool>,
    pub host: Option<crate::types::common::Host>,
    #[doc = "The hostname of the node."]
    pub hostname: Option<String>,
    pub id: Option<crate::types::common::Id>,
    pub index: Option<crate::types::common::IndexName>,
    pub ip: Option<crate::types::common::Ip>,
    #[doc = "Whether this is a searchable snapshot."]
    pub is_searchable_snapshot: Option<bool>,
    pub name: Option<crate::types::common::Name>,
    #[doc = "Whether this is a shallow copy from a remote store."]
    pub remote_store_index_shallow_copy: Option<bool>,
    pub repository: Option<crate::types::common::Name>,
    pub restore_uuid: Option<crate::types::common::Uuid>,
    pub snapshot: Option<crate::types::common::Name>,
    #[doc = "The source remote store repository."]
    pub source_remote_store_repository: Option<String>,
    #[doc = "The source remote translog repository."]
    pub source_remote_translog_repository: Option<String>,
    pub transport_address: Option<crate::types::common::TransportAddress>,
    pub version: Option<crate::types::common::VersionString>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RecoveryOrigin {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RecoveryOrigin;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RecoveryOrigin object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_bootstrap_new_history_uuid: Option<bool> = None;
                let mut seen_bootstrap_new_history_uuid = false;
                let mut f_host: Option<crate::types::common::Host> = None;
                let mut seen_host = false;
                let mut f_hostname: Option<String> = None;
                let mut seen_hostname = false;
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut seen_id = false;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut seen_index = false;
                let mut f_ip: Option<crate::types::common::Ip> = None;
                let mut seen_ip = false;
                let mut f_is_searchable_snapshot: Option<bool> = None;
                let mut seen_is_searchable_snapshot = false;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut seen_name = false;
                let mut f_remote_store_index_shallow_copy: Option<bool> = None;
                let mut seen_remote_store_index_shallow_copy = false;
                let mut f_repository: Option<crate::types::common::Name> = None;
                let mut seen_repository = false;
                let mut f_restore_uuid: Option<crate::types::common::Uuid> = None;
                let mut seen_restore_uuid = false;
                let mut f_snapshot: Option<crate::types::common::Name> = None;
                let mut seen_snapshot = false;
                let mut f_source_remote_store_repository: Option<String> = None;
                let mut seen_source_remote_store_repository = false;
                let mut f_source_remote_translog_repository: Option<String> = None;
                let mut seen_source_remote_translog_repository = false;
                let mut f_transport_address: Option<crate::types::common::TransportAddress> = None;
                let mut seen_transport_address = false;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "bootstrap_new_history_uuid" => {
                            if seen_bootstrap_new_history_uuid {
                                return Err(serde::de::Error::duplicate_field(
                                    "bootstrap_new_history_uuid",
                                ));
                            }
                            seen_bootstrap_new_history_uuid = true;
                            f_bootstrap_new_history_uuid = map.next_value::<Option<bool>>()?;
                        }
                        "host" => {
                            if seen_host {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            seen_host = true;
                            f_host = map.next_value::<Option<crate::types::common::Host>>()?;
                        }
                        "hostname" => {
                            if seen_hostname {
                                return Err(serde::de::Error::duplicate_field("hostname"));
                            }
                            seen_hostname = true;
                            f_hostname = map.next_value::<Option<String>>()?;
                        }
                        "id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "ip" => {
                            if seen_ip {
                                return Err(serde::de::Error::duplicate_field("ip"));
                            }
                            seen_ip = true;
                            f_ip = map.next_value::<Option<crate::types::common::Ip>>()?;
                        }
                        "isSearchableSnapshot" => {
                            if seen_is_searchable_snapshot {
                                return Err(serde::de::Error::duplicate_field(
                                    "isSearchableSnapshot",
                                ));
                            }
                            seen_is_searchable_snapshot = true;
                            f_is_searchable_snapshot = map.next_value::<Option<bool>>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "remoteStoreIndexShallowCopy" => {
                            if seen_remote_store_index_shallow_copy {
                                return Err(serde::de::Error::duplicate_field(
                                    "remoteStoreIndexShallowCopy",
                                ));
                            }
                            seen_remote_store_index_shallow_copy = true;
                            f_remote_store_index_shallow_copy = map.next_value::<Option<bool>>()?;
                        }
                        "repository" => {
                            if seen_repository {
                                return Err(serde::de::Error::duplicate_field("repository"));
                            }
                            seen_repository = true;
                            f_repository =
                                map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "restoreUUID" => {
                            if seen_restore_uuid {
                                return Err(serde::de::Error::duplicate_field("restoreUUID"));
                            }
                            seen_restore_uuid = true;
                            f_restore_uuid =
                                map.next_value::<Option<crate::types::common::Uuid>>()?;
                        }
                        "snapshot" => {
                            if seen_snapshot {
                                return Err(serde::de::Error::duplicate_field("snapshot"));
                            }
                            seen_snapshot = true;
                            f_snapshot = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "sourceRemoteStoreRepository" => {
                            if seen_source_remote_store_repository {
                                return Err(serde::de::Error::duplicate_field(
                                    "sourceRemoteStoreRepository",
                                ));
                            }
                            seen_source_remote_store_repository = true;
                            f_source_remote_store_repository =
                                map.next_value::<Option<String>>()?;
                        }
                        "sourceRemoteTranslogRepository" => {
                            if seen_source_remote_translog_repository {
                                return Err(serde::de::Error::duplicate_field(
                                    "sourceRemoteTranslogRepository",
                                ));
                            }
                            seen_source_remote_translog_repository = true;
                            f_source_remote_translog_repository =
                                map.next_value::<Option<String>>()?;
                        }
                        "transport_address" => {
                            if seen_transport_address {
                                return Err(serde::de::Error::duplicate_field("transport_address"));
                            }
                            seen_transport_address = true;
                            f_transport_address =
                                map.next_value::<Option<crate::types::common::TransportAddress>>()?;
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
                Ok(RecoveryOrigin {
                    bootstrap_new_history_uuid: f_bootstrap_new_history_uuid,
                    host: f_host,
                    hostname: f_hostname,
                    id: f_id,
                    index: f_index,
                    ip: f_ip,
                    is_searchable_snapshot: f_is_searchable_snapshot,
                    name: f_name,
                    remote_store_index_shallow_copy: f_remote_store_index_shallow_copy,
                    repository: f_repository,
                    restore_uuid: f_restore_uuid,
                    snapshot: f_snapshot,
                    source_remote_store_repository: f_source_remote_store_repository,
                    source_remote_translog_repository: f_source_remote_translog_repository,
                    transport_address: f_transport_address,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RecoveryOrigin {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.bootstrap_new_history_uuid {
            map.serialize_entry("bootstrap_new_history_uuid", v)?;
        }
        if let Some(v) = &self.host {
            map.serialize_entry("host", v)?;
        }
        if let Some(v) = &self.hostname {
            map.serialize_entry("hostname", v)?;
        }
        if let Some(v) = &self.id {
            map.serialize_entry("id", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.ip {
            map.serialize_entry("ip", v)?;
        }
        if let Some(v) = &self.is_searchable_snapshot {
            map.serialize_entry("isSearchableSnapshot", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.remote_store_index_shallow_copy {
            map.serialize_entry("remoteStoreIndexShallowCopy", v)?;
        }
        if let Some(v) = &self.repository {
            map.serialize_entry("repository", v)?;
        }
        if let Some(v) = &self.restore_uuid {
            map.serialize_entry("restoreUUID", v)?;
        }
        if let Some(v) = &self.snapshot {
            map.serialize_entry("snapshot", v)?;
        }
        if let Some(v) = &self.source_remote_store_repository {
            map.serialize_entry("sourceRemoteStoreRepository", v)?;
        }
        if let Some(v) = &self.source_remote_translog_repository {
            map.serialize_entry("sourceRemoteTranslogRepository", v)?;
        }
        if let Some(v) = &self.transport_address {
            map.serialize_entry("transport_address", v)?;
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

#[doc = "The response of the `indices.recovery` API"]
pub type RecoveryResponse = crate::types::indices::RecoveryResponse;

#[doc = "The status at the start of recovery."]
#[derive(Debug, Clone)]
pub struct RecoveryStartStatus {
    pub check_index_time: Option<crate::types::common::Duration>,
    pub check_index_time_in_millis: crate::types::common::DurationValueUnitMillis,
    pub total_time: Option<crate::types::common::Duration>,
    pub total_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RecoveryStartStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RecoveryStartStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RecoveryStartStatus object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_check_index_time: Option<crate::types::common::Duration> = None;
                let mut seen_check_index_time = false;
                let mut f_check_index_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
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
                        "check_index_time" => {
                            if seen_check_index_time {
                                return Err(serde::de::Error::duplicate_field("check_index_time"));
                            }
                            seen_check_index_time = true;
                            f_check_index_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "check_index_time_in_millis" => {
                            if f_check_index_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "check_index_time_in_millis",
                                ));
                            }
                            f_check_index_time_in_millis = Some(
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
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RecoveryStartStatus {
                    check_index_time: f_check_index_time,
                    check_index_time_in_millis: f_check_index_time_in_millis.ok_or_else(|| {
                        serde::de::Error::missing_field("check_index_time_in_millis")
                    })?,
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
impl serde::Serialize for RecoveryStartStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.check_index_time {
            map.serialize_entry("check_index_time", v)?;
        }
        map.serialize_entry(
            "check_index_time_in_millis",
            &self.check_index_time_in_millis,
        )?;
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

#[doc = "The status of recovery operations."]
#[derive(Debug, Clone)]
pub struct RecoveryStatus {
    #[doc = "The list of shard recovery statuses."]
    pub shards: Vec<crate::types::indices_recovery::ShardRecovery>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RecoveryStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RecoveryStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RecoveryStatus object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_shards: Option<Vec<crate::types::indices_recovery::ShardRecovery>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "shards" => {
                            if f_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("shards"));
                            }
                            f_shards = Some(map.next_value::<Vec<crate::types::indices_recovery::ShardRecovery>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RecoveryStatus {
                    shards: f_shards.ok_or_else(|| serde::de::Error::missing_field("shards"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RecoveryStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("shards", &self.shards)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The recovery status of a shard."]
#[derive(Debug, Clone)]
pub struct ShardRecovery {
    #[doc = "The shard ID."]
    pub id: i32,
    pub index: crate::types::indices_recovery::RecoveryIndexStatus,
    #[doc = "Whether this is a primary shard."]
    pub primary: bool,
    pub source: crate::types::indices_recovery::RecoveryOrigin,
    #[doc = "The current recovery stage."]
    pub stage: String,
    pub start: Option<crate::types::indices_recovery::RecoveryStartStatus>,
    pub start_time: Option<crate::types::common::DateTime>,
    pub start_time_in_millis: crate::types::common::EpochTimeUnitMillis,
    pub stop_time: Option<crate::types::common::DateTime>,
    pub stop_time_in_millis: Option<crate::types::common::EpochTimeUnitMillis>,
    pub target: crate::types::indices_recovery::RecoveryOrigin,
    pub total_time: Option<crate::types::common::Duration>,
    pub total_time_in_millis: crate::types::common::DurationValueUnitMillis,
    pub translog: crate::types::indices_recovery::TranslogStatus,
    #[doc = "The type of recovery operation."]
    pub r#type: String,
    pub verify_index: crate::types::indices_recovery::VerifyIndex,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardRecovery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardRecovery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardRecovery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_id: Option<i32> = None;
                let mut f_index: Option<crate::types::indices_recovery::RecoveryIndexStatus> = None;
                let mut f_primary: Option<bool> = None;
                let mut f_source: Option<crate::types::indices_recovery::RecoveryOrigin> = None;
                let mut f_stage: Option<String> = None;
                let mut f_start: Option<crate::types::indices_recovery::RecoveryStartStatus> = None;
                let mut seen_start = false;
                let mut f_start_time: Option<crate::types::common::DateTime> = None;
                let mut seen_start_time = false;
                let mut f_start_time_in_millis: Option<crate::types::common::EpochTimeUnitMillis> =
                    None;
                let mut f_stop_time: Option<crate::types::common::DateTime> = None;
                let mut seen_stop_time = false;
                let mut f_stop_time_in_millis: Option<crate::types::common::EpochTimeUnitMillis> =
                    None;
                let mut seen_stop_time_in_millis = false;
                let mut f_target: Option<crate::types::indices_recovery::RecoveryOrigin> = None;
                let mut f_total_time: Option<crate::types::common::Duration> = None;
                let mut seen_total_time = false;
                let mut f_total_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_translog: Option<crate::types::indices_recovery::TranslogStatus> = None;
                let mut f_type: Option<String> = None;
                let mut f_verify_index: Option<crate::types::indices_recovery::VerifyIndex> = None;
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
                            f_id = Some(map.next_value::<i32>()?);
                        }
                        "index" => {
                            if f_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            f_index = Some(map.next_value::<crate::types::indices_recovery::RecoveryIndexStatus>()?);
                        }
                        "primary" => {
                            if f_primary.is_some() {
                                return Err(serde::de::Error::duplicate_field("primary"));
                            }
                            f_primary = Some(map.next_value::<bool>()?);
                        }
                        "source" => {
                            if f_source.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            f_source = Some(
                                map.next_value::<crate::types::indices_recovery::RecoveryOrigin>()?,
                            );
                        }
                        "stage" => {
                            if f_stage.is_some() {
                                return Err(serde::de::Error::duplicate_field("stage"));
                            }
                            f_stage = Some(map.next_value::<String>()?);
                        }
                        "start" => {
                            if seen_start {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            seen_start = true;
                            f_start = map.next_value::<Option<crate::types::indices_recovery::RecoveryStartStatus>>()?;
                        }
                        "start_time" => {
                            if seen_start_time {
                                return Err(serde::de::Error::duplicate_field("start_time"));
                            }
                            seen_start_time = true;
                            f_start_time =
                                map.next_value::<Option<crate::types::common::DateTime>>()?;
                        }
                        "start_time_in_millis" => {
                            if f_start_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "start_time_in_millis",
                                ));
                            }
                            f_start_time_in_millis = Some(
                                map.next_value::<crate::types::common::EpochTimeUnitMillis>()?,
                            );
                        }
                        "stop_time" => {
                            if seen_stop_time {
                                return Err(serde::de::Error::duplicate_field("stop_time"));
                            }
                            seen_stop_time = true;
                            f_stop_time =
                                map.next_value::<Option<crate::types::common::DateTime>>()?;
                        }
                        "stop_time_in_millis" => {
                            if seen_stop_time_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "stop_time_in_millis",
                                ));
                            }
                            seen_stop_time_in_millis = true;
                            f_stop_time_in_millis = map
                                .next_value::<Option<crate::types::common::EpochTimeUnitMillis>>(
                                )?;
                        }
                        "target" => {
                            if f_target.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            f_target = Some(
                                map.next_value::<crate::types::indices_recovery::RecoveryOrigin>()?,
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
                        "translog" => {
                            if f_translog.is_some() {
                                return Err(serde::de::Error::duplicate_field("translog"));
                            }
                            f_translog = Some(
                                map.next_value::<crate::types::indices_recovery::TranslogStatus>()?,
                            );
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<String>()?);
                        }
                        "verify_index" => {
                            if f_verify_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("verify_index"));
                            }
                            f_verify_index = Some(
                                map.next_value::<crate::types::indices_recovery::VerifyIndex>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardRecovery {
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("id"))?,
                    index: f_index.ok_or_else(|| serde::de::Error::missing_field("index"))?,
                    primary: f_primary.ok_or_else(|| serde::de::Error::missing_field("primary"))?,
                    source: f_source.ok_or_else(|| serde::de::Error::missing_field("source"))?,
                    stage: f_stage.ok_or_else(|| serde::de::Error::missing_field("stage"))?,
                    start: f_start,
                    start_time: f_start_time,
                    start_time_in_millis: f_start_time_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("start_time_in_millis"))?,
                    stop_time: f_stop_time,
                    stop_time_in_millis: f_stop_time_in_millis,
                    target: f_target.ok_or_else(|| serde::de::Error::missing_field("target"))?,
                    total_time: f_total_time,
                    total_time_in_millis: f_total_time_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("total_time_in_millis"))?,
                    translog: f_translog
                        .ok_or_else(|| serde::de::Error::missing_field("translog"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    verify_index: f_verify_index
                        .ok_or_else(|| serde::de::Error::missing_field("verify_index"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardRecovery {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("index", &self.index)?;
        map.serialize_entry("primary", &self.primary)?;
        map.serialize_entry("source", &self.source)?;
        map.serialize_entry("stage", &self.stage)?;
        if let Some(v) = &self.start {
            map.serialize_entry("start", v)?;
        }
        if let Some(v) = &self.start_time {
            map.serialize_entry("start_time", v)?;
        }
        map.serialize_entry("start_time_in_millis", &self.start_time_in_millis)?;
        if let Some(v) = &self.stop_time {
            map.serialize_entry("stop_time", v)?;
        }
        if let Some(v) = &self.stop_time_in_millis {
            map.serialize_entry("stop_time_in_millis", v)?;
        }
        map.serialize_entry("target", &self.target)?;
        if let Some(v) = &self.total_time {
            map.serialize_entry("total_time", v)?;
        }
        map.serialize_entry("total_time_in_millis", &self.total_time_in_millis)?;
        map.serialize_entry("translog", &self.translog)?;
        map.serialize_entry("type", &self.r#type)?;
        map.serialize_entry("verify_index", &self.verify_index)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The transaction log recovery status."]
#[derive(Debug, Clone)]
pub struct TranslogStatus {
    pub percent: crate::types::common::PercentageString,
    #[doc = "The number of recovered transaction log operations."]
    pub recovered: i32,
    #[doc = "The total number of transaction log operations."]
    pub total: i32,
    #[doc = "The number of transaction log operations at start."]
    pub total_on_start: i32,
    pub total_time: Option<crate::types::common::Duration>,
    pub total_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TranslogStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TranslogStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TranslogStatus object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_percent: Option<crate::types::common::PercentageString> = None;
                let mut f_recovered: Option<i32> = None;
                let mut f_total: Option<i32> = None;
                let mut f_total_on_start: Option<i32> = None;
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
                        "percent" => {
                            if f_percent.is_some() {
                                return Err(serde::de::Error::duplicate_field("percent"));
                            }
                            f_percent =
                                Some(map.next_value::<crate::types::common::PercentageString>()?);
                        }
                        "recovered" => {
                            if f_recovered.is_some() {
                                return Err(serde::de::Error::duplicate_field("recovered"));
                            }
                            f_recovered = Some(map.next_value::<i32>()?);
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(map.next_value::<i32>()?);
                        }
                        "total_on_start" => {
                            if f_total_on_start.is_some() {
                                return Err(serde::de::Error::duplicate_field("total_on_start"));
                            }
                            f_total_on_start = Some(map.next_value::<i32>()?);
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
                Ok(TranslogStatus {
                    percent: f_percent.ok_or_else(|| serde::de::Error::missing_field("percent"))?,
                    recovered: f_recovered
                        .ok_or_else(|| serde::de::Error::missing_field("recovered"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    total_on_start: f_total_on_start
                        .ok_or_else(|| serde::de::Error::missing_field("total_on_start"))?,
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
impl serde::Serialize for TranslogStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("percent", &self.percent)?;
        map.serialize_entry("recovered", &self.recovered)?;
        map.serialize_entry("total", &self.total)?;
        map.serialize_entry("total_on_start", &self.total_on_start)?;
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

#[doc = "The index verification status."]
#[derive(Debug, Clone)]
pub struct VerifyIndex {
    pub check_index_time: Option<crate::types::common::Duration>,
    pub check_index_time_in_millis: crate::types::common::DurationValueUnitMillis,
    pub total_time: Option<crate::types::common::Duration>,
    pub total_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for VerifyIndex {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = VerifyIndex;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a VerifyIndex object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_check_index_time: Option<crate::types::common::Duration> = None;
                let mut seen_check_index_time = false;
                let mut f_check_index_time_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
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
                        "check_index_time" => {
                            if seen_check_index_time {
                                return Err(serde::de::Error::duplicate_field("check_index_time"));
                            }
                            seen_check_index_time = true;
                            f_check_index_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "check_index_time_in_millis" => {
                            if f_check_index_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "check_index_time_in_millis",
                                ));
                            }
                            f_check_index_time_in_millis = Some(
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
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(VerifyIndex {
                    check_index_time: f_check_index_time,
                    check_index_time_in_millis: f_check_index_time_in_millis.ok_or_else(|| {
                        serde::de::Error::missing_field("check_index_time_in_millis")
                    })?,
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
impl serde::Serialize for VerifyIndex {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.check_index_time {
            map.serialize_entry("check_index_time", v)?;
        }
        map.serialize_entry(
            "check_index_time_in_millis",
            &self.check_index_time_in_millis,
        )?;
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
