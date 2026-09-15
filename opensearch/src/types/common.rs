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
pub struct ClusterStatistics {
    pub skipped: i32,
    pub successful: i32,
    pub total: i32,
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

#[doc = "A date histogram interval, similar to `Duration`, with support for additional units: `w` (week), `M` (month), `q` (quarter), and `y` (year)."]
pub type DurationLarge = String;

pub type DurationValueUnitNanos = crate::types::common::UnitNanos;

pub type EpochTimeUnitMillis = crate::types::common::UnitMillis;

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

#[doc = "The path to a field or an array of paths. Some APIs support wildcards in the path, which allows you to select multiple fields."]
pub type Field = String;

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

#[doc = "The geohash string representation of a geographical location."]
pub type GeoHash = String;

#[doc = "The geographical location specified using a geohash."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeoHashLocation {
    pub geohash: crate::types::common::GeoHash,
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

#[doc = "A map tile reference, represented as `{zoom}/{x}/{y}`."]
pub type GeoTile = String;

#[doc = "The unique identifier for a resource."]
pub type Id = String;

pub type IndexName = String;

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

#[doc = "The geographical location specified using latitude and longitude."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LatLonGeoLocation {
    #[doc = "The latitude coordinate."]
    pub lat: f64,
    #[doc = "The longitude coordinate."]
    pub lon: f64,
}

#[doc = "The custom metadata attached to a resource."]
pub type Metadata = std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

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

#[doc = "The routing value for the document."]
pub type Routing = String;

pub type ScrollId = String;

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

pub type SortResults = Vec<crate::types::common::FieldValue>;

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

#[doc = "The type of document or resource."]
pub type Type = String;

pub type Uint = i32;

#[doc = "The time unit for milliseconds."]
pub type UnitMillis = i64;

#[doc = "Time unit for nanoseconds."]
pub type UnitNanos = i64;

pub type VersionNumber = i64;

#[doc = "The absence of any type. This is commonly used in APIs that don't return a body.  Although \"void\" is generally used for a unit type that has only one value, this is interpreted as the bottom type, which has no value. Most languages have a unit type, but few have a bottom type.  See https://en.m.wikipedia.org/wiki/Unit_type and https://en.m.wikipedia.org/wiki/Bottom_type."]
pub type Void = std::collections::BTreeMap<String, serde_json::Value>;

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
