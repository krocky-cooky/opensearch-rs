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

//! Response types of the `transforms_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct ContinuousStats {
    pub documents_behind: Option<std::collections::BTreeMap<String, f64>>,
    pub last_timestamp: Option<f64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ContinuousStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ContinuousStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ContinuousStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_documents_behind: Option<std::collections::BTreeMap<String, f64>> = None;
                let mut seen_documents_behind = false;
                let mut f_last_timestamp: Option<f64> = None;
                let mut seen_last_timestamp = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "documents_behind" => {
                            if seen_documents_behind {
                                return Err(serde::de::Error::duplicate_field("documents_behind"));
                            }
                            seen_documents_behind = true;
                            f_documents_behind = map
                                .next_value::<Option<std::collections::BTreeMap<String, f64>>>()?;
                        }
                        "last_timestamp" => {
                            if seen_last_timestamp {
                                return Err(serde::de::Error::duplicate_field("last_timestamp"));
                            }
                            seen_last_timestamp = true;
                            f_last_timestamp = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ContinuousStats {
                    documents_behind: f_documents_behind,
                    last_timestamp: f_last_timestamp,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ContinuousStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.documents_behind {
            map.serialize_entry("documents_behind", v)?;
        }
        if let Some(v) = &self.last_timestamp {
            map.serialize_entry("last_timestamp", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct DateHistogramGroup {
    pub calendar_interval: Option<String>,
    pub fixed_interval: Option<String>,
    pub source_field: Option<String>,
    pub target_field: Option<String>,
    pub timezone: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DateHistogramGroup {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DateHistogramGroup;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DateHistogramGroup object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_calendar_interval: Option<String> = None;
                let mut seen_calendar_interval = false;
                let mut f_fixed_interval: Option<String> = None;
                let mut seen_fixed_interval = false;
                let mut f_source_field: Option<String> = None;
                let mut seen_source_field = false;
                let mut f_target_field: Option<String> = None;
                let mut seen_target_field = false;
                let mut f_timezone: Option<String> = None;
                let mut seen_timezone = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "calendar_interval" => {
                            if seen_calendar_interval {
                                return Err(serde::de::Error::duplicate_field("calendar_interval"));
                            }
                            seen_calendar_interval = true;
                            f_calendar_interval = map.next_value::<Option<String>>()?;
                        }
                        "fixed_interval" => {
                            if seen_fixed_interval {
                                return Err(serde::de::Error::duplicate_field("fixed_interval"));
                            }
                            seen_fixed_interval = true;
                            f_fixed_interval = map.next_value::<Option<String>>()?;
                        }
                        "source_field" => {
                            if seen_source_field {
                                return Err(serde::de::Error::duplicate_field("source_field"));
                            }
                            seen_source_field = true;
                            f_source_field = map.next_value::<Option<String>>()?;
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field = map.next_value::<Option<String>>()?;
                        }
                        "timezone" => {
                            if seen_timezone {
                                return Err(serde::de::Error::duplicate_field("timezone"));
                            }
                            seen_timezone = true;
                            f_timezone = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DateHistogramGroup {
                    calendar_interval: f_calendar_interval,
                    fixed_interval: f_fixed_interval,
                    source_field: f_source_field,
                    target_field: f_target_field,
                    timezone: f_timezone,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DateHistogramGroup {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.calendar_interval {
            map.serialize_entry("calendar_interval", v)?;
        }
        if let Some(v) = &self.fixed_interval {
            map.serialize_entry("fixed_interval", v)?;
        }
        if let Some(v) = &self.source_field {
            map.serialize_entry("source_field", v)?;
        }
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        if let Some(v) = &self.timezone {
            map.serialize_entry("timezone", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Explain {
    pub metadata_id: Option<String>,
    pub transform_metadata: Option<crate::types::transforms_common::TransformMetadata>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Explain {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Explain;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Explain object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_metadata_id: Option<String> = None;
                let mut seen_metadata_id = false;
                let mut f_transform_metadata: Option<
                    crate::types::transforms_common::TransformMetadata,
                > = None;
                let mut seen_transform_metadata = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "metadata_id" => {
                            if seen_metadata_id {
                                return Err(serde::de::Error::duplicate_field("metadata_id"));
                            }
                            seen_metadata_id = true;
                            f_metadata_id = map.next_value::<Option<String>>()?;
                        }
                        "transform_metadata" => {
                            if seen_transform_metadata {
                                return Err(serde::de::Error::duplicate_field(
                                    "transform_metadata",
                                ));
                            }
                            seen_transform_metadata = true;
                            f_transform_metadata = map.next_value::<Option<crate::types::transforms_common::TransformMetadata>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Explain {
                    metadata_id: f_metadata_id,
                    transform_metadata: f_transform_metadata,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Explain {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.metadata_id {
            map.serialize_entry("metadata_id", v)?;
        }
        if let Some(v) = &self.transform_metadata {
            map.serialize_entry("transform_metadata", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type ExplainResponse =
    std::collections::BTreeMap<String, crate::types::transforms_common::Explain>;

#[derive(Debug, Clone)]
pub struct ExplainStats {
    pub documents_indexed: Option<f64>,
    pub documents_processed: Option<f64>,
    pub index_time_in_millis: Option<f64>,
    pub pages_processed: Option<f64>,
    pub search_time_in_millis: Option<f64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ExplainStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ExplainStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ExplainStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_documents_indexed: Option<f64> = None;
                let mut seen_documents_indexed = false;
                let mut f_documents_processed: Option<f64> = None;
                let mut seen_documents_processed = false;
                let mut f_index_time_in_millis: Option<f64> = None;
                let mut seen_index_time_in_millis = false;
                let mut f_pages_processed: Option<f64> = None;
                let mut seen_pages_processed = false;
                let mut f_search_time_in_millis: Option<f64> = None;
                let mut seen_search_time_in_millis = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "documents_indexed" => {
                            if seen_documents_indexed {
                                return Err(serde::de::Error::duplicate_field("documents_indexed"));
                            }
                            seen_documents_indexed = true;
                            f_documents_indexed = map.next_value::<Option<f64>>()?;
                        }
                        "documents_processed" => {
                            if seen_documents_processed {
                                return Err(serde::de::Error::duplicate_field(
                                    "documents_processed",
                                ));
                            }
                            seen_documents_processed = true;
                            f_documents_processed = map.next_value::<Option<f64>>()?;
                        }
                        "index_time_in_millis" => {
                            if seen_index_time_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "index_time_in_millis",
                                ));
                            }
                            seen_index_time_in_millis = true;
                            f_index_time_in_millis = map.next_value::<Option<f64>>()?;
                        }
                        "pages_processed" => {
                            if seen_pages_processed {
                                return Err(serde::de::Error::duplicate_field("pages_processed"));
                            }
                            seen_pages_processed = true;
                            f_pages_processed = map.next_value::<Option<f64>>()?;
                        }
                        "search_time_in_millis" => {
                            if seen_search_time_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "search_time_in_millis",
                                ));
                            }
                            seen_search_time_in_millis = true;
                            f_search_time_in_millis = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ExplainStats {
                    documents_indexed: f_documents_indexed,
                    documents_processed: f_documents_processed,
                    index_time_in_millis: f_index_time_in_millis,
                    pages_processed: f_pages_processed,
                    search_time_in_millis: f_search_time_in_millis,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ExplainStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.documents_indexed {
            map.serialize_entry("documents_indexed", v)?;
        }
        if let Some(v) = &self.documents_processed {
            map.serialize_entry("documents_processed", v)?;
        }
        if let Some(v) = &self.index_time_in_millis {
            map.serialize_entry("index_time_in_millis", v)?;
        }
        if let Some(v) = &self.pages_processed {
            map.serialize_entry("pages_processed", v)?;
        }
        if let Some(v) = &self.search_time_in_millis {
            map.serialize_entry("search_time_in_millis", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct GroupsConfigItem {
    pub date_histogram: Option<crate::types::transforms_common::DateHistogramGroup>,
    pub histogram: Option<crate::types::transforms_common::HistogramGroup>,
    pub terms: Option<crate::types::transforms_common::TermsGroup>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GroupsConfigItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GroupsConfigItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GroupsConfigItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_date_histogram: Option<
                    crate::types::transforms_common::DateHistogramGroup,
                > = None;
                let mut seen_date_histogram = false;
                let mut f_histogram: Option<crate::types::transforms_common::HistogramGroup> = None;
                let mut seen_histogram = false;
                let mut f_terms: Option<crate::types::transforms_common::TermsGroup> = None;
                let mut seen_terms = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "date_histogram" => {
                            if seen_date_histogram {
                                return Err(serde::de::Error::duplicate_field("date_histogram"));
                            }
                            seen_date_histogram = true;
                            f_date_histogram = map.next_value::<Option<crate::types::transforms_common::DateHistogramGroup>>()?;
                        }
                        "histogram" => {
                            if seen_histogram {
                                return Err(serde::de::Error::duplicate_field("histogram"));
                            }
                            seen_histogram = true;
                            f_histogram = map.next_value::<Option<crate::types::transforms_common::HistogramGroup>>()?;
                        }
                        "terms" => {
                            if seen_terms {
                                return Err(serde::de::Error::duplicate_field("terms"));
                            }
                            seen_terms = true;
                            f_terms = map
                                .next_value::<Option<crate::types::transforms_common::TermsGroup>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GroupsConfigItem {
                    date_histogram: f_date_histogram,
                    histogram: f_histogram,
                    terms: f_terms,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GroupsConfigItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.date_histogram {
            map.serialize_entry("date_histogram", v)?;
        }
        if let Some(v) = &self.histogram {
            map.serialize_entry("histogram", v)?;
        }
        if let Some(v) = &self.terms {
            map.serialize_entry("terms", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct HistogramGroup {
    pub interval: Option<String>,
    pub source_field: Option<String>,
    pub target_field: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HistogramGroup {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HistogramGroup;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HistogramGroup object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_interval: Option<String> = None;
                let mut seen_interval = false;
                let mut f_source_field: Option<String> = None;
                let mut seen_source_field = false;
                let mut f_target_field: Option<String> = None;
                let mut seen_target_field = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "interval" => {
                            if seen_interval {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            seen_interval = true;
                            f_interval = map.next_value::<Option<String>>()?;
                        }
                        "source_field" => {
                            if seen_source_field {
                                return Err(serde::de::Error::duplicate_field("source_field"));
                            }
                            seen_source_field = true;
                            f_source_field = map.next_value::<Option<String>>()?;
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HistogramGroup {
                    interval: f_interval,
                    source_field: f_source_field,
                    target_field: f_target_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HistogramGroup {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.interval {
            map.serialize_entry("interval", v)?;
        }
        if let Some(v) = &self.source_field {
            map.serialize_entry("source_field", v)?;
        }
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Preview {
    pub documents:
        Option<Vec<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Preview {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Preview;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Preview object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_documents: Option<
                    Vec<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
                > = None;
                let mut seen_documents = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "documents" => {
                            if seen_documents {
                                return Err(serde::de::Error::duplicate_field("documents"));
                            }
                            seen_documents = true;
                            f_documents = map.next_value::<Option<
                                Vec<
                                    std::collections::BTreeMap<
                                        String,
                                        Box<serde_json::value::RawValue>,
                                    >,
                                >,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Preview {
                    documents: f_documents,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Preview {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.documents {
            map.serialize_entry("documents", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Schedule {
    pub interval: crate::types::transforms_common::ScheduleInterval,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Schedule {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Schedule;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Schedule object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_interval: Option<crate::types::transforms_common::ScheduleInterval> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "interval" => {
                            if f_interval.is_some() {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            f_interval = Some(map.next_value::<crate::types::transforms_common::ScheduleInterval>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Schedule {
                    interval: f_interval
                        .ok_or_else(|| serde::de::Error::missing_field("interval"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Schedule {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("interval", &self.interval)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ScheduleInterval {
    pub period: Option<f64>,
    pub start_time: Option<f64>,
    pub unit: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ScheduleInterval {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ScheduleInterval;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ScheduleInterval object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_period: Option<f64> = None;
                let mut seen_period = false;
                let mut f_start_time: Option<f64> = None;
                let mut seen_start_time = false;
                let mut f_unit: Option<String> = None;
                let mut seen_unit = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "period" => {
                            if seen_period {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            seen_period = true;
                            f_period = map.next_value::<Option<f64>>()?;
                        }
                        "start_time" => {
                            if seen_start_time {
                                return Err(serde::de::Error::duplicate_field("start_time"));
                            }
                            seen_start_time = true;
                            f_start_time = map.next_value::<Option<f64>>()?;
                        }
                        "unit" => {
                            if seen_unit {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            seen_unit = true;
                            f_unit = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ScheduleInterval {
                    period: f_period,
                    start_time: f_start_time,
                    unit: f_unit,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ScheduleInterval {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.period {
            map.serialize_entry("period", v)?;
        }
        if let Some(v) = &self.start_time {
            map.serialize_entry("start_time", v)?;
        }
        if let Some(v) = &self.unit {
            map.serialize_entry("unit", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TermsGroup {
    pub source_field: Option<String>,
    pub target_field: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TermsGroup {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TermsGroup;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TermsGroup object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_source_field: Option<String> = None;
                let mut seen_source_field = false;
                let mut f_target_field: Option<String> = None;
                let mut seen_target_field = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "source_field" => {
                            if seen_source_field {
                                return Err(serde::de::Error::duplicate_field("source_field"));
                            }
                            seen_source_field = true;
                            f_source_field = map.next_value::<Option<String>>()?;
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TermsGroup {
                    source_field: f_source_field,
                    target_field: f_target_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TermsGroup {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.source_field {
            map.serialize_entry("source_field", v)?;
        }
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Transform {
    pub aggregations: Option<
        std::collections::BTreeMap<String, crate::types::transforms_common::GroupsConfigItem>,
    >,
    pub continuous: Option<bool>,
    pub data_selection_query: Option<crate::types::common_query_dsl::QueryContainer>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
    pub enabled_at: Option<f64>,
    pub groups: Option<Vec<crate::types::transforms_common::GroupsConfigItem>>,
    pub metadata_id: Option<String>,
    pub page_size: Option<f64>,
    pub roles: Option<Vec<String>>,
    pub schedule: Option<crate::types::transforms_common::Schedule>,
    pub schema_version: Option<f64>,
    pub source_index: Option<String>,
    pub target_index: Option<String>,
    pub transform_id: Option<String>,
    pub updated_at: Option<f64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Transform {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Transform;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Transform object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_aggregations: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::transforms_common::GroupsConfigItem,
                    >,
                > = None;
                let mut seen_aggregations = false;
                let mut f_continuous: Option<bool> = None;
                let mut seen_continuous = false;
                let mut f_data_selection_query: Option<
                    crate::types::common_query_dsl::QueryContainer,
                > = None;
                let mut seen_data_selection_query = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_enabled: Option<bool> = None;
                let mut seen_enabled = false;
                let mut f_enabled_at: Option<f64> = None;
                let mut seen_enabled_at = false;
                let mut f_groups: Option<Vec<crate::types::transforms_common::GroupsConfigItem>> =
                    None;
                let mut seen_groups = false;
                let mut f_metadata_id: Option<String> = None;
                let mut seen_metadata_id = false;
                let mut f_page_size: Option<f64> = None;
                let mut seen_page_size = false;
                let mut f_roles: Option<Vec<String>> = None;
                let mut seen_roles = false;
                let mut f_schedule: Option<crate::types::transforms_common::Schedule> = None;
                let mut seen_schedule = false;
                let mut f_schema_version: Option<f64> = None;
                let mut seen_schema_version = false;
                let mut f_source_index: Option<String> = None;
                let mut seen_source_index = false;
                let mut f_target_index: Option<String> = None;
                let mut seen_target_index = false;
                let mut f_transform_id: Option<String> = None;
                let mut seen_transform_id = false;
                let mut f_updated_at: Option<f64> = None;
                let mut seen_updated_at = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "aggregations" => {
                            if seen_aggregations {
                                return Err(serde::de::Error::duplicate_field("aggregations"));
                            }
                            seen_aggregations = true;
                            f_aggregations = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::transforms_common::GroupsConfigItem,
                                >,
                            >>()?;
                        }
                        "continuous" => {
                            if seen_continuous {
                                return Err(serde::de::Error::duplicate_field("continuous"));
                            }
                            seen_continuous = true;
                            f_continuous = map.next_value::<Option<bool>>()?;
                        }
                        "data_selection_query" => {
                            if seen_data_selection_query {
                                return Err(serde::de::Error::duplicate_field(
                                    "data_selection_query",
                                ));
                            }
                            seen_data_selection_query = true;
                            f_data_selection_query = map.next_value::<Option<crate::types::common_query_dsl::QueryContainer>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "enabled" => {
                            if seen_enabled {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            seen_enabled = true;
                            f_enabled = map.next_value::<Option<bool>>()?;
                        }
                        "enabled_at" => {
                            if seen_enabled_at {
                                return Err(serde::de::Error::duplicate_field("enabled_at"));
                            }
                            seen_enabled_at = true;
                            f_enabled_at = map.next_value::<Option<f64>>()?;
                        }
                        "groups" => {
                            if seen_groups {
                                return Err(serde::de::Error::duplicate_field("groups"));
                            }
                            seen_groups = true;
                            f_groups = map.next_value::<Option<Vec<crate::types::transforms_common::GroupsConfigItem>>>()?;
                        }
                        "metadata_id" => {
                            if seen_metadata_id {
                                return Err(serde::de::Error::duplicate_field("metadata_id"));
                            }
                            seen_metadata_id = true;
                            f_metadata_id = map.next_value::<Option<String>>()?;
                        }
                        "page_size" => {
                            if seen_page_size {
                                return Err(serde::de::Error::duplicate_field("page_size"));
                            }
                            seen_page_size = true;
                            f_page_size = map.next_value::<Option<f64>>()?;
                        }
                        "roles" => {
                            if seen_roles {
                                return Err(serde::de::Error::duplicate_field("roles"));
                            }
                            seen_roles = true;
                            f_roles = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "schedule" => {
                            if seen_schedule {
                                return Err(serde::de::Error::duplicate_field("schedule"));
                            }
                            seen_schedule = true;
                            f_schedule = map
                                .next_value::<Option<crate::types::transforms_common::Schedule>>(
                                )?;
                        }
                        "schema_version" => {
                            if seen_schema_version {
                                return Err(serde::de::Error::duplicate_field("schema_version"));
                            }
                            seen_schema_version = true;
                            f_schema_version = map.next_value::<Option<f64>>()?;
                        }
                        "source_index" => {
                            if seen_source_index {
                                return Err(serde::de::Error::duplicate_field("source_index"));
                            }
                            seen_source_index = true;
                            f_source_index = map.next_value::<Option<String>>()?;
                        }
                        "target_index" => {
                            if seen_target_index {
                                return Err(serde::de::Error::duplicate_field("target_index"));
                            }
                            seen_target_index = true;
                            f_target_index = map.next_value::<Option<String>>()?;
                        }
                        "transform_id" => {
                            if seen_transform_id {
                                return Err(serde::de::Error::duplicate_field("transform_id"));
                            }
                            seen_transform_id = true;
                            f_transform_id = map.next_value::<Option<String>>()?;
                        }
                        "updated_at" => {
                            if seen_updated_at {
                                return Err(serde::de::Error::duplicate_field("updated_at"));
                            }
                            seen_updated_at = true;
                            f_updated_at = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Transform {
                    aggregations: f_aggregations,
                    continuous: f_continuous,
                    data_selection_query: f_data_selection_query,
                    description: f_description,
                    enabled: f_enabled,
                    enabled_at: f_enabled_at,
                    groups: f_groups,
                    metadata_id: f_metadata_id,
                    page_size: f_page_size,
                    roles: f_roles,
                    schedule: f_schedule,
                    schema_version: f_schema_version,
                    source_index: f_source_index,
                    target_index: f_target_index,
                    transform_id: f_transform_id,
                    updated_at: f_updated_at,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Transform {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.aggregations {
            map.serialize_entry("aggregations", v)?;
        }
        if let Some(v) = &self.continuous {
            map.serialize_entry("continuous", v)?;
        }
        if let Some(v) = &self.data_selection_query {
            map.serialize_entry("data_selection_query", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.enabled {
            map.serialize_entry("enabled", v)?;
        }
        if let Some(v) = &self.enabled_at {
            map.serialize_entry("enabled_at", v)?;
        }
        if let Some(v) = &self.groups {
            map.serialize_entry("groups", v)?;
        }
        if let Some(v) = &self.metadata_id {
            map.serialize_entry("metadata_id", v)?;
        }
        if let Some(v) = &self.page_size {
            map.serialize_entry("page_size", v)?;
        }
        if let Some(v) = &self.roles {
            map.serialize_entry("roles", v)?;
        }
        if let Some(v) = &self.schedule {
            map.serialize_entry("schedule", v)?;
        }
        if let Some(v) = &self.schema_version {
            map.serialize_entry("schema_version", v)?;
        }
        if let Some(v) = &self.source_index {
            map.serialize_entry("source_index", v)?;
        }
        if let Some(v) = &self.target_index {
            map.serialize_entry("target_index", v)?;
        }
        if let Some(v) = &self.transform_id {
            map.serialize_entry("transform_id", v)?;
        }
        if let Some(v) = &self.updated_at {
            map.serialize_entry("updated_at", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TransformMetadata {
    pub continuous_stats: Option<crate::types::transforms_common::ContinuousStats>,
    pub failure_reason: Option<String>,
    pub last_updated_at: Option<f64>,
    pub stats: Option<crate::types::transforms_common::ExplainStats>,
    pub status: Option<String>,
    pub transform_id: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TransformMetadata {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TransformMetadata;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TransformMetadata object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_continuous_stats: Option<
                    crate::types::transforms_common::ContinuousStats,
                > = None;
                let mut seen_continuous_stats = false;
                let mut f_failure_reason: Option<String> = None;
                let mut seen_failure_reason = false;
                let mut f_last_updated_at: Option<f64> = None;
                let mut seen_last_updated_at = false;
                let mut f_stats: Option<crate::types::transforms_common::ExplainStats> = None;
                let mut seen_stats = false;
                let mut f_status: Option<String> = None;
                let mut seen_status = false;
                let mut f_transform_id: Option<String> = None;
                let mut seen_transform_id = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "continuous_stats" => {
                            if seen_continuous_stats {
                                return Err(serde::de::Error::duplicate_field("continuous_stats"));
                            }
                            seen_continuous_stats = true;
                            f_continuous_stats = map.next_value::<Option<crate::types::transforms_common::ContinuousStats>>()?;
                        }
                        "failure_reason" => {
                            if seen_failure_reason {
                                return Err(serde::de::Error::duplicate_field("failure_reason"));
                            }
                            seen_failure_reason = true;
                            f_failure_reason = map.next_value::<Option<String>>()?;
                        }
                        "last_updated_at" => {
                            if seen_last_updated_at {
                                return Err(serde::de::Error::duplicate_field("last_updated_at"));
                            }
                            seen_last_updated_at = true;
                            f_last_updated_at = map.next_value::<Option<f64>>()?;
                        }
                        "stats" => {
                            if seen_stats {
                                return Err(serde::de::Error::duplicate_field("stats"));
                            }
                            seen_stats = true;
                            f_stats = map.next_value::<Option<crate::types::transforms_common::ExplainStats>>()?;
                        }
                        "status" => {
                            if seen_status {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            seen_status = true;
                            f_status = map.next_value::<Option<String>>()?;
                        }
                        "transform_id" => {
                            if seen_transform_id {
                                return Err(serde::de::Error::duplicate_field("transform_id"));
                            }
                            seen_transform_id = true;
                            f_transform_id = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TransformMetadata {
                    continuous_stats: f_continuous_stats,
                    failure_reason: f_failure_reason,
                    last_updated_at: f_last_updated_at,
                    stats: f_stats,
                    status: f_status,
                    transform_id: f_transform_id,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TransformMetadata {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.continuous_stats {
            map.serialize_entry("continuous_stats", v)?;
        }
        if let Some(v) = &self.failure_reason {
            map.serialize_entry("failure_reason", v)?;
        }
        if let Some(v) = &self.last_updated_at {
            map.serialize_entry("last_updated_at", v)?;
        }
        if let Some(v) = &self.stats {
            map.serialize_entry("stats", v)?;
        }
        if let Some(v) = &self.status {
            map.serialize_entry("status", v)?;
        }
        if let Some(v) = &self.transform_id {
            map.serialize_entry("transform_id", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TransformResponse {
    pub id: Option<crate::types::common::Id>,
    pub primary_term: Option<i64>,
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    pub version: Option<crate::types::common::VersionNumber>,
    pub transform: Option<crate::types::transforms_common::Transform>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TransformResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TransformResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TransformResponse object")
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
                let mut seen_id = false;
                let mut f_primary_term: Option<i64> = None;
                let mut seen_primary_term = false;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut seen_seq_no = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version = false;
                let mut f_transform: Option<crate::types::transforms_common::Transform> = None;
                let mut seen_transform = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("_id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        "_primary_term" => {
                            if seen_primary_term {
                                return Err(serde::de::Error::duplicate_field("_primary_term"));
                            }
                            seen_primary_term = true;
                            f_primary_term = map.next_value::<Option<i64>>()?;
                        }
                        "_seq_no" => {
                            if seen_seq_no {
                                return Err(serde::de::Error::duplicate_field("_seq_no"));
                            }
                            seen_seq_no = true;
                            f_seq_no =
                                map.next_value::<Option<crate::types::common::SequenceNumber>>()?;
                        }
                        "_version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionNumber>>()?;
                        }
                        "transform" => {
                            if seen_transform {
                                return Err(serde::de::Error::duplicate_field("transform"));
                            }
                            seen_transform = true;
                            f_transform = map
                                .next_value::<Option<crate::types::transforms_common::Transform>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TransformResponse {
                    id: f_id,
                    primary_term: f_primary_term,
                    seq_no: f_seq_no,
                    version: f_version,
                    transform: f_transform,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TransformResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.id {
            map.serialize_entry("_id", v)?;
        }
        if let Some(v) = &self.primary_term {
            map.serialize_entry("_primary_term", v)?;
        }
        if let Some(v) = &self.seq_no {
            map.serialize_entry("_seq_no", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("_version", v)?;
        }
        if let Some(v) = &self.transform {
            map.serialize_entry("transform", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TransformsResponse {
    pub total_transforms: Option<f64>,
    pub transforms: Option<Vec<crate::types::transforms_common::TransformResponse>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TransformsResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TransformsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TransformsResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_total_transforms: Option<f64> = None;
                let mut seen_total_transforms = false;
                let mut f_transforms: Option<
                    Vec<crate::types::transforms_common::TransformResponse>,
                > = None;
                let mut seen_transforms = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "total_transforms" => {
                            if seen_total_transforms {
                                return Err(serde::de::Error::duplicate_field("total_transforms"));
                            }
                            seen_total_transforms = true;
                            f_total_transforms = map.next_value::<Option<f64>>()?;
                        }
                        "transforms" => {
                            if seen_transforms {
                                return Err(serde::de::Error::duplicate_field("transforms"));
                            }
                            seen_transforms = true;
                            f_transforms = map.next_value::<Option<Vec<crate::types::transforms_common::TransformResponse>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TransformsResponse {
                    total_transforms: f_total_transforms,
                    transforms: f_transforms,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TransformsResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.total_transforms {
            map.serialize_entry("total_transforms", v)?;
        }
        if let Some(v) = &self.transforms {
            map.serialize_entry("transforms", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
