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

//! Response types of the `geospatial_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct DataSource {
    pub database: crate::types::geospatial_common::Database,
    #[doc = "URL endpoint for the data source."]
    pub endpoint: String,
    pub name: crate::types::common::Name,
    pub next_update_at_in_epoch_millis: crate::types::common::EpochTimeUnitMillis,
    #[doc = "State of the data source."]
    pub state: crate::types::geospatial_common::DataSourceState,
    #[doc = "Update interval."]
    pub update_interval_in_days: i64,
    pub update_stats: crate::types::geospatial_common::UpdateStats,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DataSource {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DataSource;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DataSource object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_database: Option<crate::types::geospatial_common::Database> = None;
                let mut f_endpoint: Option<String> = None;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut f_next_update_at_in_epoch_millis: Option<
                    crate::types::common::EpochTimeUnitMillis,
                > = None;
                let mut f_state: Option<crate::types::geospatial_common::DataSourceState> = None;
                let mut f_update_interval_in_days: Option<i64> = None;
                let mut f_update_stats: Option<crate::types::geospatial_common::UpdateStats> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "database" => {
                            if f_database.is_some() {
                                return Err(serde::de::Error::duplicate_field("database"));
                            }
                            f_database = Some(
                                map.next_value::<crate::types::geospatial_common::Database>()?,
                            );
                        }
                        "endpoint" => {
                            if f_endpoint.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            f_endpoint = Some(map.next_value::<String>()?);
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        "next_update_at_in_epoch_millis" => {
                            if f_next_update_at_in_epoch_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "next_update_at_in_epoch_millis",
                                ));
                            }
                            f_next_update_at_in_epoch_millis = Some(
                                map.next_value::<crate::types::common::EpochTimeUnitMillis>()?,
                            );
                        }
                        "state" => {
                            if f_state.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            f_state = Some(
                                map.next_value::<crate::types::geospatial_common::DataSourceState>(
                                )?,
                            );
                        }
                        "update_interval_in_days" => {
                            if f_update_interval_in_days.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "update_interval_in_days",
                                ));
                            }
                            f_update_interval_in_days = Some(map.next_value::<i64>()?);
                        }
                        "update_stats" => {
                            if f_update_stats.is_some() {
                                return Err(serde::de::Error::duplicate_field("update_stats"));
                            }
                            f_update_stats = Some(
                                map.next_value::<crate::types::geospatial_common::UpdateStats>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DataSource {
                    database: f_database
                        .ok_or_else(|| serde::de::Error::missing_field("database"))?,
                    endpoint: f_endpoint
                        .ok_or_else(|| serde::de::Error::missing_field("endpoint"))?,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    next_update_at_in_epoch_millis: f_next_update_at_in_epoch_millis.ok_or_else(
                        || serde::de::Error::missing_field("next_update_at_in_epoch_millis"),
                    )?,
                    state: f_state.ok_or_else(|| serde::de::Error::missing_field("state"))?,
                    update_interval_in_days: f_update_interval_in_days.ok_or_else(|| {
                        serde::de::Error::missing_field("update_interval_in_days")
                    })?,
                    update_stats: f_update_stats
                        .ok_or_else(|| serde::de::Error::missing_field("update_stats"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DataSource {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("database", &self.database)?;
        map.serialize_entry("endpoint", &self.endpoint)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry(
            "next_update_at_in_epoch_millis",
            &self.next_update_at_in_epoch_millis,
        )?;
        map.serialize_entry("state", &self.state)?;
        map.serialize_entry("update_interval_in_days", &self.update_interval_in_days)?;
        map.serialize_entry("update_stats", &self.update_stats)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataSourceState {
    #[doc = "`AVAILABLE`"]
    Available,
    #[doc = "`CREATE_FAILED`"]
    CreateFailed,
    #[doc = "`CREATING`"]
    Creating,
    #[doc = "`DELETING`"]
    Deleting,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DataSourceState {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DataSourceState::Available => "AVAILABLE",
            DataSourceState::CreateFailed => "CREATE_FAILED",
            DataSourceState::Creating => "CREATING",
            DataSourceState::Deleting => "DELETING",
            DataSourceState::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DataSourceState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DataSourceState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "AVAILABLE" => DataSourceState::Available,
            "CREATE_FAILED" => DataSourceState::CreateFailed,
            "CREATING" => DataSourceState::Creating,
            "DELETING" => DataSourceState::Deleting,
            _ => DataSourceState::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Database {
    pub fields: Option<Vec<String>>,
    pub provider: Option<String>,
    pub sha_256_hash: Option<String>,
    pub updated_at_in_epoch_millis: Option<crate::types::common::EpochTimeUnitMillis>,
    pub valid_for_in_days: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Database {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Database;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Database object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_fields: Option<Vec<String>> = None;
                let mut seen_fields = false;
                let mut f_provider: Option<String> = None;
                let mut seen_provider = false;
                let mut f_sha_256_hash: Option<String> = None;
                let mut seen_sha_256_hash = false;
                let mut f_updated_at_in_epoch_millis: Option<
                    crate::types::common::EpochTimeUnitMillis,
                > = None;
                let mut seen_updated_at_in_epoch_millis = false;
                let mut f_valid_for_in_days: Option<i64> = None;
                let mut seen_valid_for_in_days = false;
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
                            f_fields = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "provider" => {
                            if seen_provider {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            seen_provider = true;
                            f_provider = map.next_value::<Option<String>>()?;
                        }
                        "sha256_hash" => {
                            if seen_sha_256_hash {
                                return Err(serde::de::Error::duplicate_field("sha256_hash"));
                            }
                            seen_sha_256_hash = true;
                            f_sha_256_hash = map.next_value::<Option<String>>()?;
                        }
                        "updated_at_in_epoch_millis" => {
                            if seen_updated_at_in_epoch_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "updated_at_in_epoch_millis",
                                ));
                            }
                            seen_updated_at_in_epoch_millis = true;
                            f_updated_at_in_epoch_millis = map
                                .next_value::<Option<crate::types::common::EpochTimeUnitMillis>>(
                                )?;
                        }
                        "valid_for_in_days" => {
                            if seen_valid_for_in_days {
                                return Err(serde::de::Error::duplicate_field("valid_for_in_days"));
                            }
                            seen_valid_for_in_days = true;
                            f_valid_for_in_days = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Database {
                    fields: f_fields,
                    provider: f_provider,
                    sha_256_hash: f_sha_256_hash,
                    updated_at_in_epoch_millis: f_updated_at_in_epoch_millis,
                    valid_for_in_days: f_valid_for_in_days,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Database {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.provider {
            map.serialize_entry("provider", v)?;
        }
        if let Some(v) = &self.sha_256_hash {
            map.serialize_entry("sha256_hash", v)?;
        }
        if let Some(v) = &self.updated_at_in_epoch_millis {
            map.serialize_entry("updated_at_in_epoch_millis", v)?;
        }
        if let Some(v) = &self.valid_for_in_days {
            map.serialize_entry("valid_for_in_days", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct GeoSpatialGeojsonUploadResponse {
    #[doc = "Whether there were any errors."]
    pub errors: bool,
    #[doc = "Number of features that failed to upload."]
    pub failure: i64,
    #[doc = "Number of features successfully uploaded."]
    pub success: i64,
    pub took: crate::types::common::DurationValueUnitMillis,
    #[doc = "Total number of features processed."]
    pub total: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GeoSpatialGeojsonUploadResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GeoSpatialGeojsonUploadResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoSpatialGeojsonUploadResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_errors: Option<bool> = None;
                let mut f_failure: Option<i64> = None;
                let mut f_success: Option<i64> = None;
                let mut f_took: Option<crate::types::common::DurationValueUnitMillis> = None;
                let mut f_total: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "errors" => {
                            if f_errors.is_some() {
                                return Err(serde::de::Error::duplicate_field("errors"));
                            }
                            f_errors = Some(map.next_value::<bool>()?);
                        }
                        "failure" => {
                            if f_failure.is_some() {
                                return Err(serde::de::Error::duplicate_field("failure"));
                            }
                            f_failure = Some(map.next_value::<i64>()?);
                        }
                        "success" => {
                            if f_success.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            f_success = Some(map.next_value::<i64>()?);
                        }
                        "took" => {
                            if f_took.is_some() {
                                return Err(serde::de::Error::duplicate_field("took"));
                            }
                            f_took = Some(
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
                Ok(GeoSpatialGeojsonUploadResponse {
                    errors: f_errors.ok_or_else(|| serde::de::Error::missing_field("errors"))?,
                    failure: f_failure.ok_or_else(|| serde::de::Error::missing_field("failure"))?,
                    success: f_success.ok_or_else(|| serde::de::Error::missing_field("success"))?,
                    took: f_took.ok_or_else(|| serde::de::Error::missing_field("took"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GeoSpatialGeojsonUploadResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("errors", &self.errors)?;
        map.serialize_entry("failure", &self.failure)?;
        map.serialize_entry("success", &self.success)?;
        map.serialize_entry("took", &self.took)?;
        map.serialize_entry("total", &self.total)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct GeoSpatialUploadStats {
    pub metrics: Vec<crate::types::geospatial_common::UploadStatsMetric>,
    pub total: crate::types::geospatial_common::UploadStatsTotal,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GeoSpatialUploadStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GeoSpatialUploadStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoSpatialUploadStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_metrics: Option<Vec<crate::types::geospatial_common::UploadStatsMetric>> =
                    None;
                let mut f_total: Option<crate::types::geospatial_common::UploadStatsTotal> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "metrics" => {
                            if f_metrics.is_some() {
                                return Err(serde::de::Error::duplicate_field("metrics"));
                            }
                            f_metrics = Some(map.next_value::<Vec<crate::types::geospatial_common::UploadStatsMetric>>()?);
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(map.next_value::<crate::types::geospatial_common::UploadStatsTotal>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GeoSpatialUploadStats {
                    metrics: f_metrics.ok_or_else(|| serde::de::Error::missing_field("metrics"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GeoSpatialUploadStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("metrics", &self.metrics)?;
        map.serialize_entry("total", &self.total)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct GetDataSourceResponse {
    pub datasources: Vec<crate::types::geospatial_common::DataSource>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GetDataSourceResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GetDataSourceResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GetDataSourceResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_datasources: Option<Vec<crate::types::geospatial_common::DataSource>> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "datasources" => {
                            if f_datasources.is_some() {
                                return Err(serde::de::Error::duplicate_field("datasources"));
                            }
                            f_datasources = Some(
                                map.next_value::<Vec<crate::types::geospatial_common::DataSource>>(
                                )?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GetDataSourceResponse {
                    datasources: f_datasources
                        .ok_or_else(|| serde::de::Error::missing_field("datasources"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GetDataSourceResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("datasources", &self.datasources)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct UpdateStats {
    pub last_processing_time_in_millis: Option<crate::types::common::EpochTimeUnitMillis>,
    pub last_succeeded_at_in_epoch_millis: Option<crate::types::common::EpochTimeUnitMillis>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UpdateStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UpdateStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UpdateStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_last_processing_time_in_millis: Option<
                    crate::types::common::EpochTimeUnitMillis,
                > = None;
                let mut seen_last_processing_time_in_millis = false;
                let mut f_last_succeeded_at_in_epoch_millis: Option<
                    crate::types::common::EpochTimeUnitMillis,
                > = None;
                let mut seen_last_succeeded_at_in_epoch_millis = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "last_processing_time_in_millis" => {
                            if seen_last_processing_time_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "last_processing_time_in_millis",
                                ));
                            }
                            seen_last_processing_time_in_millis = true;
                            f_last_processing_time_in_millis = map
                                .next_value::<Option<crate::types::common::EpochTimeUnitMillis>>(
                                )?;
                        }
                        "last_succeeded_at_in_epoch_millis" => {
                            if seen_last_succeeded_at_in_epoch_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "last_succeeded_at_in_epoch_millis",
                                ));
                            }
                            seen_last_succeeded_at_in_epoch_millis = true;
                            f_last_succeeded_at_in_epoch_millis = map
                                .next_value::<Option<crate::types::common::EpochTimeUnitMillis>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UpdateStats {
                    last_processing_time_in_millis: f_last_processing_time_in_millis,
                    last_succeeded_at_in_epoch_millis: f_last_succeeded_at_in_epoch_millis,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UpdateStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.last_processing_time_in_millis {
            map.serialize_entry("last_processing_time_in_millis", v)?;
        }
        if let Some(v) = &self.last_succeeded_at_in_epoch_millis {
            map.serialize_entry("last_succeeded_at_in_epoch_millis", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct UploadStatsMetric {
    pub duration: crate::types::common::DurationValueUnitMillis,
    #[doc = "Number of failed features in this upload."]
    pub failed: i64,
    pub id: crate::types::common::Id,
    pub node_id: crate::types::common::NodeId,
    #[doc = "Number of successful features in this upload."]
    pub success: i64,
    #[doc = "Type of upload (such as \"GeoJSON\")."]
    pub r#type: String,
    #[doc = "Number of features in this upload."]
    pub upload: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UploadStatsMetric {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UploadStatsMetric;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UploadStatsMetric object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_duration: Option<crate::types::common::DurationValueUnitMillis> = None;
                let mut f_failed: Option<i64> = None;
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut f_node_id: Option<crate::types::common::NodeId> = None;
                let mut f_success: Option<i64> = None;
                let mut f_type: Option<String> = None;
                let mut f_upload: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "duration" => {
                            if f_duration.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            f_duration = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "failed" => {
                            if f_failed.is_some() {
                                return Err(serde::de::Error::duplicate_field("failed"));
                            }
                            f_failed = Some(map.next_value::<i64>()?);
                        }
                        "id" => {
                            if f_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            f_id = Some(map.next_value::<crate::types::common::Id>()?);
                        }
                        "node_id" => {
                            if f_node_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("node_id"));
                            }
                            f_node_id = Some(map.next_value::<crate::types::common::NodeId>()?);
                        }
                        "success" => {
                            if f_success.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            f_success = Some(map.next_value::<i64>()?);
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<String>()?);
                        }
                        "upload" => {
                            if f_upload.is_some() {
                                return Err(serde::de::Error::duplicate_field("upload"));
                            }
                            f_upload = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UploadStatsMetric {
                    duration: f_duration
                        .ok_or_else(|| serde::de::Error::missing_field("duration"))?,
                    failed: f_failed.ok_or_else(|| serde::de::Error::missing_field("failed"))?,
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("id"))?,
                    node_id: f_node_id.ok_or_else(|| serde::de::Error::missing_field("node_id"))?,
                    success: f_success.ok_or_else(|| serde::de::Error::missing_field("success"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    upload: f_upload.ok_or_else(|| serde::de::Error::missing_field("upload"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UploadStatsMetric {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("duration", &self.duration)?;
        map.serialize_entry("failed", &self.failed)?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("node_id", &self.node_id)?;
        map.serialize_entry("success", &self.success)?;
        map.serialize_entry("type", &self.r#type)?;
        map.serialize_entry("upload", &self.upload)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct UploadStatsTotal {
    pub duration: crate::types::common::DurationValueUnitMillis,
    #[doc = "Total failed uploads."]
    pub failed: i64,
    #[doc = "Total number of upload requests."]
    pub request_count: i64,
    #[doc = "Total successful uploads."]
    pub success: i64,
    #[doc = "Total number of uploads."]
    pub upload: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UploadStatsTotal {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UploadStatsTotal;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UploadStatsTotal object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_duration: Option<crate::types::common::DurationValueUnitMillis> = None;
                let mut f_failed: Option<i64> = None;
                let mut f_request_count: Option<i64> = None;
                let mut f_success: Option<i64> = None;
                let mut f_upload: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "duration" => {
                            if f_duration.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            f_duration = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "failed" => {
                            if f_failed.is_some() {
                                return Err(serde::de::Error::duplicate_field("failed"));
                            }
                            f_failed = Some(map.next_value::<i64>()?);
                        }
                        "request_count" => {
                            if f_request_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("request_count"));
                            }
                            f_request_count = Some(map.next_value::<i64>()?);
                        }
                        "success" => {
                            if f_success.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            f_success = Some(map.next_value::<i64>()?);
                        }
                        "upload" => {
                            if f_upload.is_some() {
                                return Err(serde::de::Error::duplicate_field("upload"));
                            }
                            f_upload = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UploadStatsTotal {
                    duration: f_duration
                        .ok_or_else(|| serde::de::Error::missing_field("duration"))?,
                    failed: f_failed.ok_or_else(|| serde::de::Error::missing_field("failed"))?,
                    request_count: f_request_count
                        .ok_or_else(|| serde::de::Error::missing_field("request_count"))?,
                    success: f_success.ok_or_else(|| serde::de::Error::missing_field("success"))?,
                    upload: f_upload.ok_or_else(|| serde::de::Error::missing_field("upload"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UploadStatsTotal {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("duration", &self.duration)?;
        map.serialize_entry("failed", &self.failed)?;
        map.serialize_entry("request_count", &self.request_count)?;
        map.serialize_entry("success", &self.success)?;
        map.serialize_entry("upload", &self.upload)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
