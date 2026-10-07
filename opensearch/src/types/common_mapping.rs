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

//! Response types of the `common_mapping` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct AggregateMetricDoubleProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub default_metric: String,
    pub metrics: Vec<String>,
    pub r#type: crate::types::common_mapping::AggregateMetricDoublePropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AggregateMetricDoubleProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AggregateMetricDoubleProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AggregateMetricDoubleProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_default_metric: Option<String> = None;
                let mut f_metrics: Option<Vec<String>> = None;
                let mut f_type: Option<
                    crate::types::common_mapping::AggregateMetricDoublePropertyType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "default_metric" => {
                            if f_default_metric.is_some() {
                                return Err(serde::de::Error::duplicate_field("default_metric"));
                            }
                            f_default_metric = Some(map.next_value::<String>()?);
                        }
                        "metrics" => {
                            if f_metrics.is_some() {
                                return Err(serde::de::Error::duplicate_field("metrics"));
                            }
                            f_metrics = Some(map.next_value::<Vec<String>>()?);
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::AggregateMetricDoublePropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AggregateMetricDoubleProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    default_metric: f_default_metric
                        .ok_or_else(|| serde::de::Error::missing_field("default_metric"))?,
                    metrics: f_metrics.ok_or_else(|| serde::de::Error::missing_field("metrics"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AggregateMetricDoubleProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        map.serialize_entry("default_metric", &self.default_metric)?;
        map.serialize_entry("metrics", &self.metrics)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AggregateMetricDoublePropertyType {
    #[doc = "`aggregate_metric_double`"]
    AggregateMetricDouble,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl AggregateMetricDoublePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            AggregateMetricDoublePropertyType::AggregateMetricDouble => "aggregate_metric_double",
            AggregateMetricDoublePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for AggregateMetricDoublePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AggregateMetricDoublePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "aggregate_metric_double" => AggregateMetricDoublePropertyType::AggregateMetricDouble,
            _ => AggregateMetricDoublePropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct AllField {
    pub analyzer: String,
    pub enabled: bool,
    pub omit_norms: bool,
    pub search_analyzer: String,
    pub similarity: String,
    pub store: bool,
    pub store_term_vector_offsets: bool,
    pub store_term_vector_payloads: bool,
    pub store_term_vector_positions: bool,
    pub store_term_vectors: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AllField {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AllField;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AllField object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_analyzer: Option<String> = None;
                let mut f_enabled: Option<bool> = None;
                let mut f_omit_norms: Option<bool> = None;
                let mut f_search_analyzer: Option<String> = None;
                let mut f_similarity: Option<String> = None;
                let mut f_store: Option<bool> = None;
                let mut f_store_term_vector_offsets: Option<bool> = None;
                let mut f_store_term_vector_payloads: Option<bool> = None;
                let mut f_store_term_vector_positions: Option<bool> = None;
                let mut f_store_term_vectors: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "analyzer" => {
                            if f_analyzer.is_some() {
                                return Err(serde::de::Error::duplicate_field("analyzer"));
                            }
                            f_analyzer = Some(map.next_value::<String>()?);
                        }
                        "enabled" => {
                            if f_enabled.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            f_enabled = Some(map.next_value::<bool>()?);
                        }
                        "omit_norms" => {
                            if f_omit_norms.is_some() {
                                return Err(serde::de::Error::duplicate_field("omit_norms"));
                            }
                            f_omit_norms = Some(map.next_value::<bool>()?);
                        }
                        "search_analyzer" => {
                            if f_search_analyzer.is_some() {
                                return Err(serde::de::Error::duplicate_field("search_analyzer"));
                            }
                            f_search_analyzer = Some(map.next_value::<String>()?);
                        }
                        "similarity" => {
                            if f_similarity.is_some() {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            f_similarity = Some(map.next_value::<String>()?);
                        }
                        "store" => {
                            if f_store.is_some() {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            f_store = Some(map.next_value::<bool>()?);
                        }
                        "store_term_vector_offsets" => {
                            if f_store_term_vector_offsets.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "store_term_vector_offsets",
                                ));
                            }
                            f_store_term_vector_offsets = Some(map.next_value::<bool>()?);
                        }
                        "store_term_vector_payloads" => {
                            if f_store_term_vector_payloads.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "store_term_vector_payloads",
                                ));
                            }
                            f_store_term_vector_payloads = Some(map.next_value::<bool>()?);
                        }
                        "store_term_vector_positions" => {
                            if f_store_term_vector_positions.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "store_term_vector_positions",
                                ));
                            }
                            f_store_term_vector_positions = Some(map.next_value::<bool>()?);
                        }
                        "store_term_vectors" => {
                            if f_store_term_vectors.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "store_term_vectors",
                                ));
                            }
                            f_store_term_vectors = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AllField {
                    analyzer: f_analyzer
                        .ok_or_else(|| serde::de::Error::missing_field("analyzer"))?,
                    enabled: f_enabled.ok_or_else(|| serde::de::Error::missing_field("enabled"))?,
                    omit_norms: f_omit_norms
                        .ok_or_else(|| serde::de::Error::missing_field("omit_norms"))?,
                    search_analyzer: f_search_analyzer
                        .ok_or_else(|| serde::de::Error::missing_field("search_analyzer"))?,
                    similarity: f_similarity
                        .ok_or_else(|| serde::de::Error::missing_field("similarity"))?,
                    store: f_store.ok_or_else(|| serde::de::Error::missing_field("store"))?,
                    store_term_vector_offsets: f_store_term_vector_offsets.ok_or_else(|| {
                        serde::de::Error::missing_field("store_term_vector_offsets")
                    })?,
                    store_term_vector_payloads: f_store_term_vector_payloads.ok_or_else(|| {
                        serde::de::Error::missing_field("store_term_vector_payloads")
                    })?,
                    store_term_vector_positions: f_store_term_vector_positions.ok_or_else(
                        || serde::de::Error::missing_field("store_term_vector_positions"),
                    )?,
                    store_term_vectors: f_store_term_vectors
                        .ok_or_else(|| serde::de::Error::missing_field("store_term_vectors"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AllField {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("analyzer", &self.analyzer)?;
        map.serialize_entry("enabled", &self.enabled)?;
        map.serialize_entry("omit_norms", &self.omit_norms)?;
        map.serialize_entry("search_analyzer", &self.search_analyzer)?;
        map.serialize_entry("similarity", &self.similarity)?;
        map.serialize_entry("store", &self.store)?;
        map.serialize_entry("store_term_vector_offsets", &self.store_term_vector_offsets)?;
        map.serialize_entry(
            "store_term_vector_payloads",
            &self.store_term_vector_payloads,
        )?;
        map.serialize_entry(
            "store_term_vector_positions",
            &self.store_term_vector_positions,
        )?;
        map.serialize_entry("store_term_vectors", &self.store_term_vectors)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct BinaryProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub r#type: crate::types::common_mapping::BinaryPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for BinaryProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = BinaryProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a BinaryProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_type: Option<crate::types::common_mapping::BinaryPropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(
                                map.next_value::<crate::types::common_mapping::BinaryPropertyType>(
                                )?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(BinaryProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for BinaryProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryPropertyType {
    #[doc = "`binary`"]
    Binary,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl BinaryPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            BinaryPropertyType::Binary => "binary",
            BinaryPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for BinaryPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BinaryPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "binary" => BinaryPropertyType::Binary,
            _ => BinaryPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct BooleanProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub boost: Option<f64>,
    pub fielddata: Option<crate::types::indices_common::NumericFielddata>,
    pub index: Option<bool>,
    pub null_value: Option<bool>,
    pub r#type: crate::types::common_mapping::BooleanPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for BooleanProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = BooleanProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a BooleanProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_fielddata: Option<crate::types::indices_common::NumericFielddata> = None;
                let mut seen_fielddata = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_null_value: Option<bool> = None;
                let mut seen_null_value = false;
                let mut f_type: Option<crate::types::common_mapping::BooleanPropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "fielddata" => {
                            if seen_fielddata {
                                return Err(serde::de::Error::duplicate_field("fielddata"));
                            }
                            seen_fielddata = true;
                            f_fielddata = map.next_value::<Option<crate::types::indices_common::NumericFielddata>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "null_value" => {
                            if seen_null_value {
                                return Err(serde::de::Error::duplicate_field("null_value"));
                            }
                            seen_null_value = true;
                            f_null_value = map.next_value::<Option<bool>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::BooleanPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(BooleanProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    boost: f_boost,
                    fielddata: f_fielddata,
                    index: f_index,
                    null_value: f_null_value,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for BooleanProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.fielddata {
            map.serialize_entry("fielddata", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.null_value {
            map.serialize_entry("null_value", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BooleanPropertyType {
    #[doc = "`boolean`"]
    Boolean,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl BooleanPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            BooleanPropertyType::Boolean => "boolean",
            BooleanPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for BooleanPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BooleanPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "boolean" => BooleanPropertyType::Boolean,
            _ => BooleanPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ByteNumberProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub boost: Option<f64>,
    pub coerce: Option<bool>,
    pub ignore_malformed: Option<bool>,
    pub index: Option<bool>,
    pub null_value: Option<crate::types::common::Byte>,
    pub r#type: crate::types::common_mapping::ByteNumberPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ByteNumberProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ByteNumberProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ByteNumberProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_coerce: Option<bool> = None;
                let mut seen_coerce = false;
                let mut f_ignore_malformed: Option<bool> = None;
                let mut seen_ignore_malformed = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_null_value: Option<crate::types::common::Byte> = None;
                let mut seen_null_value = false;
                let mut f_type: Option<crate::types::common_mapping::ByteNumberPropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "coerce" => {
                            if seen_coerce {
                                return Err(serde::de::Error::duplicate_field("coerce"));
                            }
                            seen_coerce = true;
                            f_coerce = map.next_value::<Option<bool>>()?;
                        }
                        "ignore_malformed" => {
                            if seen_ignore_malformed {
                                return Err(serde::de::Error::duplicate_field("ignore_malformed"));
                            }
                            seen_ignore_malformed = true;
                            f_ignore_malformed = map.next_value::<Option<bool>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "null_value" => {
                            if seen_null_value {
                                return Err(serde::de::Error::duplicate_field("null_value"));
                            }
                            seen_null_value = true;
                            f_null_value =
                                map.next_value::<Option<crate::types::common::Byte>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::ByteNumberPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ByteNumberProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    boost: f_boost,
                    coerce: f_coerce,
                    ignore_malformed: f_ignore_malformed,
                    index: f_index,
                    null_value: f_null_value,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ByteNumberProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.coerce {
            map.serialize_entry("coerce", v)?;
        }
        if let Some(v) = &self.ignore_malformed {
            map.serialize_entry("ignore_malformed", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.null_value {
            map.serialize_entry("null_value", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ByteNumberPropertyType {
    #[doc = "`byte`"]
    Byte,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ByteNumberPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ByteNumberPropertyType::Byte => "byte",
            ByteNumberPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ByteNumberPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ByteNumberPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "byte" => ByteNumberPropertyType::Byte,
            _ => ByteNumberPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct CompletionProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub analyzer: Option<String>,
    pub contexts: Option<Vec<crate::types::common_mapping::SuggestContext>>,
    pub max_input_length: Option<i64>,
    pub preserve_position_increments: Option<bool>,
    pub preserve_separators: Option<bool>,
    pub search_analyzer: Option<String>,
    pub r#type: crate::types::common_mapping::CompletionPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CompletionProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CompletionProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CompletionProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_analyzer: Option<String> = None;
                let mut seen_analyzer = false;
                let mut f_contexts: Option<Vec<crate::types::common_mapping::SuggestContext>> =
                    None;
                let mut seen_contexts = false;
                let mut f_max_input_length: Option<i64> = None;
                let mut seen_max_input_length = false;
                let mut f_preserve_position_increments: Option<bool> = None;
                let mut seen_preserve_position_increments = false;
                let mut f_preserve_separators: Option<bool> = None;
                let mut seen_preserve_separators = false;
                let mut f_search_analyzer: Option<String> = None;
                let mut seen_search_analyzer = false;
                let mut f_type: Option<crate::types::common_mapping::CompletionPropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "analyzer" => {
                            if seen_analyzer {
                                return Err(serde::de::Error::duplicate_field("analyzer"));
                            }
                            seen_analyzer = true;
                            f_analyzer = map.next_value::<Option<String>>()?;
                        }
                        "contexts" => {
                            if seen_contexts {
                                return Err(serde::de::Error::duplicate_field("contexts"));
                            }
                            seen_contexts = true;
                            f_contexts = map.next_value::<Option<Vec<crate::types::common_mapping::SuggestContext>>>()?;
                        }
                        "max_input_length" => {
                            if seen_max_input_length {
                                return Err(serde::de::Error::duplicate_field("max_input_length"));
                            }
                            seen_max_input_length = true;
                            f_max_input_length = map.next_value::<Option<i64>>()?;
                        }
                        "preserve_position_increments" => {
                            if seen_preserve_position_increments {
                                return Err(serde::de::Error::duplicate_field(
                                    "preserve_position_increments",
                                ));
                            }
                            seen_preserve_position_increments = true;
                            f_preserve_position_increments = map.next_value::<Option<bool>>()?;
                        }
                        "preserve_separators" => {
                            if seen_preserve_separators {
                                return Err(serde::de::Error::duplicate_field(
                                    "preserve_separators",
                                ));
                            }
                            seen_preserve_separators = true;
                            f_preserve_separators = map.next_value::<Option<bool>>()?;
                        }
                        "search_analyzer" => {
                            if seen_search_analyzer {
                                return Err(serde::de::Error::duplicate_field("search_analyzer"));
                            }
                            seen_search_analyzer = true;
                            f_search_analyzer = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::CompletionPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CompletionProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    analyzer: f_analyzer,
                    contexts: f_contexts,
                    max_input_length: f_max_input_length,
                    preserve_position_increments: f_preserve_position_increments,
                    preserve_separators: f_preserve_separators,
                    search_analyzer: f_search_analyzer,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CompletionProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.analyzer {
            map.serialize_entry("analyzer", v)?;
        }
        if let Some(v) = &self.contexts {
            map.serialize_entry("contexts", v)?;
        }
        if let Some(v) = &self.max_input_length {
            map.serialize_entry("max_input_length", v)?;
        }
        if let Some(v) = &self.preserve_position_increments {
            map.serialize_entry("preserve_position_increments", v)?;
        }
        if let Some(v) = &self.preserve_separators {
            map.serialize_entry("preserve_separators", v)?;
        }
        if let Some(v) = &self.search_analyzer {
            map.serialize_entry("search_analyzer", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompletionPropertyType {
    #[doc = "`completion`"]
    Completion,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl CompletionPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            CompletionPropertyType::Completion => "completion",
            CompletionPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for CompletionPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CompletionPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "completion" => CompletionPropertyType::Completion,
            _ => CompletionPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ConstantKeywordProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub r#type: crate::types::common_mapping::ConstantKeywordPropertyType,
    pub value: Option<Box<serde_json::value::RawValue>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ConstantKeywordProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ConstantKeywordProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ConstantKeywordProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_type: Option<crate::types::common_mapping::ConstantKeywordPropertyType> =
                    None;
                let mut f_value: Option<Box<serde_json::value::RawValue>> = None;
                let mut seen_value = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::ConstantKeywordPropertyType>()?);
                        }
                        "value" => {
                            if seen_value {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            seen_value = true;
                            f_value =
                                map.next_value::<Option<Box<serde_json::value::RawValue>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ConstantKeywordProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    value: f_value,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ConstantKeywordProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        if let Some(v) = &self.value {
            map.serialize_entry("value", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstantKeywordPropertyType {
    #[doc = "`constant_keyword`"]
    ConstantKeyword,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ConstantKeywordPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ConstantKeywordPropertyType::ConstantKeyword => "constant_keyword",
            ConstantKeywordPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ConstantKeywordPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ConstantKeywordPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "constant_keyword" => ConstantKeywordPropertyType::ConstantKeyword,
            _ => ConstantKeywordPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct DataStreamTimestamp {
    pub enabled: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DataStreamTimestamp {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DataStreamTimestamp;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DataStreamTimestamp object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_enabled: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "enabled" => {
                            if f_enabled.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            f_enabled = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DataStreamTimestamp {
                    enabled: f_enabled.ok_or_else(|| serde::de::Error::missing_field("enabled"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DataStreamTimestamp {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("enabled", &self.enabled)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct DateNanosProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub boost: Option<f64>,
    pub format: Option<String>,
    pub ignore_malformed: Option<bool>,
    pub index: Option<bool>,
    pub null_value: Option<crate::types::common::DateTime>,
    pub precision_step: Option<i64>,
    pub r#type: crate::types::common_mapping::DateNanosPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DateNanosProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DateNanosProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DateNanosProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_format: Option<String> = None;
                let mut seen_format = false;
                let mut f_ignore_malformed: Option<bool> = None;
                let mut seen_ignore_malformed = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_null_value: Option<crate::types::common::DateTime> = None;
                let mut seen_null_value = false;
                let mut f_precision_step: Option<i64> = None;
                let mut seen_precision_step = false;
                let mut f_type: Option<crate::types::common_mapping::DateNanosPropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "format" => {
                            if seen_format {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            seen_format = true;
                            f_format = map.next_value::<Option<String>>()?;
                        }
                        "ignore_malformed" => {
                            if seen_ignore_malformed {
                                return Err(serde::de::Error::duplicate_field("ignore_malformed"));
                            }
                            seen_ignore_malformed = true;
                            f_ignore_malformed = map.next_value::<Option<bool>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "null_value" => {
                            if seen_null_value {
                                return Err(serde::de::Error::duplicate_field("null_value"));
                            }
                            seen_null_value = true;
                            f_null_value =
                                map.next_value::<Option<crate::types::common::DateTime>>()?;
                        }
                        "precision_step" => {
                            if seen_precision_step {
                                return Err(serde::de::Error::duplicate_field("precision_step"));
                            }
                            seen_precision_step = true;
                            f_precision_step = map.next_value::<Option<i64>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::DateNanosPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DateNanosProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    boost: f_boost,
                    format: f_format,
                    ignore_malformed: f_ignore_malformed,
                    index: f_index,
                    null_value: f_null_value,
                    precision_step: f_precision_step,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DateNanosProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.format {
            map.serialize_entry("format", v)?;
        }
        if let Some(v) = &self.ignore_malformed {
            map.serialize_entry("ignore_malformed", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.null_value {
            map.serialize_entry("null_value", v)?;
        }
        if let Some(v) = &self.precision_step {
            map.serialize_entry("precision_step", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DateNanosPropertyType {
    #[doc = "`date_nanos`"]
    DateNanos,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DateNanosPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DateNanosPropertyType::DateNanos => "date_nanos",
            DateNanosPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DateNanosPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DateNanosPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "date_nanos" => DateNanosPropertyType::DateNanos,
            _ => DateNanosPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct DateProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub boost: Option<f64>,
    pub fielddata: Option<crate::types::indices_common::NumericFielddata>,
    pub format: Option<String>,
    pub ignore_malformed: Option<bool>,
    pub index: Option<bool>,
    pub locale: Option<String>,
    pub null_value: Option<crate::types::common::DateTime>,
    pub precision_step: Option<i64>,
    pub r#type: crate::types::common_mapping::DatePropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DateProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DateProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DateProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_fielddata: Option<crate::types::indices_common::NumericFielddata> = None;
                let mut seen_fielddata = false;
                let mut f_format: Option<String> = None;
                let mut seen_format = false;
                let mut f_ignore_malformed: Option<bool> = None;
                let mut seen_ignore_malformed = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_locale: Option<String> = None;
                let mut seen_locale = false;
                let mut f_null_value: Option<crate::types::common::DateTime> = None;
                let mut seen_null_value = false;
                let mut f_precision_step: Option<i64> = None;
                let mut seen_precision_step = false;
                let mut f_type: Option<crate::types::common_mapping::DatePropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "fielddata" => {
                            if seen_fielddata {
                                return Err(serde::de::Error::duplicate_field("fielddata"));
                            }
                            seen_fielddata = true;
                            f_fielddata = map.next_value::<Option<crate::types::indices_common::NumericFielddata>>()?;
                        }
                        "format" => {
                            if seen_format {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            seen_format = true;
                            f_format = map.next_value::<Option<String>>()?;
                        }
                        "ignore_malformed" => {
                            if seen_ignore_malformed {
                                return Err(serde::de::Error::duplicate_field("ignore_malformed"));
                            }
                            seen_ignore_malformed = true;
                            f_ignore_malformed = map.next_value::<Option<bool>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "locale" => {
                            if seen_locale {
                                return Err(serde::de::Error::duplicate_field("locale"));
                            }
                            seen_locale = true;
                            f_locale = map.next_value::<Option<String>>()?;
                        }
                        "null_value" => {
                            if seen_null_value {
                                return Err(serde::de::Error::duplicate_field("null_value"));
                            }
                            seen_null_value = true;
                            f_null_value =
                                map.next_value::<Option<crate::types::common::DateTime>>()?;
                        }
                        "precision_step" => {
                            if seen_precision_step {
                                return Err(serde::de::Error::duplicate_field("precision_step"));
                            }
                            seen_precision_step = true;
                            f_precision_step = map.next_value::<Option<i64>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(
                                map.next_value::<crate::types::common_mapping::DatePropertyType>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DateProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    boost: f_boost,
                    fielddata: f_fielddata,
                    format: f_format,
                    ignore_malformed: f_ignore_malformed,
                    index: f_index,
                    locale: f_locale,
                    null_value: f_null_value,
                    precision_step: f_precision_step,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DateProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.fielddata {
            map.serialize_entry("fielddata", v)?;
        }
        if let Some(v) = &self.format {
            map.serialize_entry("format", v)?;
        }
        if let Some(v) = &self.ignore_malformed {
            map.serialize_entry("ignore_malformed", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.locale {
            map.serialize_entry("locale", v)?;
        }
        if let Some(v) = &self.null_value {
            map.serialize_entry("null_value", v)?;
        }
        if let Some(v) = &self.precision_step {
            map.serialize_entry("precision_step", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DatePropertyType {
    #[doc = "`date`"]
    Date,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DatePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DatePropertyType::Date => "date",
            DatePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DatePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DatePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "date" => DatePropertyType::Date,
            _ => DatePropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct DateRangeProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub boost: Option<f64>,
    pub coerce: Option<bool>,
    pub index: Option<bool>,
    pub format: Option<String>,
    pub r#type: crate::types::common_mapping::DateRangePropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DateRangeProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DateRangeProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DateRangeProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_coerce: Option<bool> = None;
                let mut seen_coerce = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_format: Option<String> = None;
                let mut seen_format = false;
                let mut f_type: Option<crate::types::common_mapping::DateRangePropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "coerce" => {
                            if seen_coerce {
                                return Err(serde::de::Error::duplicate_field("coerce"));
                            }
                            seen_coerce = true;
                            f_coerce = map.next_value::<Option<bool>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "format" => {
                            if seen_format {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            seen_format = true;
                            f_format = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::DateRangePropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DateRangeProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    boost: f_boost,
                    coerce: f_coerce,
                    index: f_index,
                    format: f_format,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DateRangeProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.coerce {
            map.serialize_entry("coerce", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.format {
            map.serialize_entry("format", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DateRangePropertyType {
    #[doc = "`date_range`"]
    DateRange,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DateRangePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DateRangePropertyType::DateRange => "date_range",
            DateRangePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DateRangePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DateRangePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "date_range" => DateRangePropertyType::DateRange,
            _ => DateRangePropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct DoubleNumberProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub boost: Option<f64>,
    pub coerce: Option<bool>,
    pub ignore_malformed: Option<bool>,
    pub index: Option<bool>,
    pub null_value: Option<f64>,
    pub r#type: crate::types::common_mapping::DoubleNumberPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DoubleNumberProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DoubleNumberProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DoubleNumberProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_coerce: Option<bool> = None;
                let mut seen_coerce = false;
                let mut f_ignore_malformed: Option<bool> = None;
                let mut seen_ignore_malformed = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_null_value: Option<f64> = None;
                let mut seen_null_value = false;
                let mut f_type: Option<crate::types::common_mapping::DoubleNumberPropertyType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "coerce" => {
                            if seen_coerce {
                                return Err(serde::de::Error::duplicate_field("coerce"));
                            }
                            seen_coerce = true;
                            f_coerce = map.next_value::<Option<bool>>()?;
                        }
                        "ignore_malformed" => {
                            if seen_ignore_malformed {
                                return Err(serde::de::Error::duplicate_field("ignore_malformed"));
                            }
                            seen_ignore_malformed = true;
                            f_ignore_malformed = map.next_value::<Option<bool>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "null_value" => {
                            if seen_null_value {
                                return Err(serde::de::Error::duplicate_field("null_value"));
                            }
                            seen_null_value = true;
                            f_null_value = map.next_value::<Option<f64>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::DoubleNumberPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DoubleNumberProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    boost: f_boost,
                    coerce: f_coerce,
                    ignore_malformed: f_ignore_malformed,
                    index: f_index,
                    null_value: f_null_value,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DoubleNumberProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.coerce {
            map.serialize_entry("coerce", v)?;
        }
        if let Some(v) = &self.ignore_malformed {
            map.serialize_entry("ignore_malformed", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.null_value {
            map.serialize_entry("null_value", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DoubleNumberPropertyType {
    #[doc = "`double`"]
    Double,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DoubleNumberPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DoubleNumberPropertyType::Double => "double",
            DoubleNumberPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DoubleNumberPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DoubleNumberPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "double" => DoubleNumberPropertyType::Double,
            _ => DoubleNumberPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct DoubleRangeProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub boost: Option<f64>,
    pub coerce: Option<bool>,
    pub index: Option<bool>,
    pub r#type: crate::types::common_mapping::DoubleRangePropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DoubleRangeProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DoubleRangeProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DoubleRangeProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_coerce: Option<bool> = None;
                let mut seen_coerce = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_type: Option<crate::types::common_mapping::DoubleRangePropertyType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "coerce" => {
                            if seen_coerce {
                                return Err(serde::de::Error::duplicate_field("coerce"));
                            }
                            seen_coerce = true;
                            f_coerce = map.next_value::<Option<bool>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::DoubleRangePropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DoubleRangeProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    boost: f_boost,
                    coerce: f_coerce,
                    index: f_index,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DoubleRangeProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.coerce {
            map.serialize_entry("coerce", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DoubleRangePropertyType {
    #[doc = "`double_range`"]
    DoubleRange,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DoubleRangePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DoubleRangePropertyType::DoubleRange => "double_range",
            DoubleRangePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DoubleRangePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DoubleRangePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "double_range" => DoubleRangePropertyType::DoubleRange,
            _ => DoubleRangePropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum DynamicMapping {
    String(crate::types::common_mapping::DynamicMappingString),
    Boolean(bool),
}
impl<'de> serde::Deserialize<'de> for DynamicMapping {
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
            Some(b'"') => serde_json::from_str(text).map(DynamicMapping::String),
            Some(b't') | Some(b'f') => serde_json::from_str(text).map(DynamicMapping::Boolean),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a DynamicMapping value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DynamicMappingString {
    #[doc = "`false`"]
    False,
    #[doc = "`strict`"]
    Strict,
    #[doc = "`strict_allow_templates`"]
    StrictAllowTemplates,
    #[doc = "`true`"]
    True,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DynamicMappingString {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DynamicMappingString::False => "false",
            DynamicMappingString::Strict => "strict",
            DynamicMappingString::StrictAllowTemplates => "strict_allow_templates",
            DynamicMappingString::True => "true",
            DynamicMappingString::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DynamicMappingString {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DynamicMappingString {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "false" => DynamicMappingString::False,
            "strict" => DynamicMappingString::Strict,
            "strict_allow_templates" => DynamicMappingString::StrictAllowTemplates,
            "true" => DynamicMappingString::True,
            _ => DynamicMappingString::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct DynamicTemplate {
    pub mapping: Option<crate::types::common_mapping::Property>,
    pub r#match: Option<String>,
    pub match_mapping_type: Option<String>,
    pub match_pattern: Option<crate::types::common_mapping::MatchType>,
    pub path_match: Option<String>,
    pub path_unmatch: Option<String>,
    pub unmatch: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DynamicTemplate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DynamicTemplate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DynamicTemplate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_mapping: Option<crate::types::common_mapping::Property> = None;
                let mut seen_mapping = false;
                let mut f_match: Option<String> = None;
                let mut seen_match = false;
                let mut f_match_mapping_type: Option<String> = None;
                let mut seen_match_mapping_type = false;
                let mut f_match_pattern: Option<crate::types::common_mapping::MatchType> = None;
                let mut seen_match_pattern = false;
                let mut f_path_match: Option<String> = None;
                let mut seen_path_match = false;
                let mut f_path_unmatch: Option<String> = None;
                let mut seen_path_unmatch = false;
                let mut f_unmatch: Option<String> = None;
                let mut seen_unmatch = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "mapping" => {
                            if seen_mapping {
                                return Err(serde::de::Error::duplicate_field("mapping"));
                            }
                            seen_mapping = true;
                            f_mapping =
                                map.next_value::<Option<crate::types::common_mapping::Property>>()?;
                        }
                        "match" => {
                            if seen_match {
                                return Err(serde::de::Error::duplicate_field("match"));
                            }
                            seen_match = true;
                            f_match = map.next_value::<Option<String>>()?;
                        }
                        "match_mapping_type" => {
                            if seen_match_mapping_type {
                                return Err(serde::de::Error::duplicate_field(
                                    "match_mapping_type",
                                ));
                            }
                            seen_match_mapping_type = true;
                            f_match_mapping_type = map.next_value::<Option<String>>()?;
                        }
                        "match_pattern" => {
                            if seen_match_pattern {
                                return Err(serde::de::Error::duplicate_field("match_pattern"));
                            }
                            seen_match_pattern = true;
                            f_match_pattern = map
                                .next_value::<Option<crate::types::common_mapping::MatchType>>()?;
                        }
                        "path_match" => {
                            if seen_path_match {
                                return Err(serde::de::Error::duplicate_field("path_match"));
                            }
                            seen_path_match = true;
                            f_path_match = map.next_value::<Option<String>>()?;
                        }
                        "path_unmatch" => {
                            if seen_path_unmatch {
                                return Err(serde::de::Error::duplicate_field("path_unmatch"));
                            }
                            seen_path_unmatch = true;
                            f_path_unmatch = map.next_value::<Option<String>>()?;
                        }
                        "unmatch" => {
                            if seen_unmatch {
                                return Err(serde::de::Error::duplicate_field("unmatch"));
                            }
                            seen_unmatch = true;
                            f_unmatch = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DynamicTemplate {
                    mapping: f_mapping,
                    r#match: f_match,
                    match_mapping_type: f_match_mapping_type,
                    match_pattern: f_match_pattern,
                    path_match: f_path_match,
                    path_unmatch: f_path_unmatch,
                    unmatch: f_unmatch,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DynamicTemplate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.mapping {
            map.serialize_entry("mapping", v)?;
        }
        if let Some(v) = &self.r#match {
            map.serialize_entry("match", v)?;
        }
        if let Some(v) = &self.match_mapping_type {
            map.serialize_entry("match_mapping_type", v)?;
        }
        if let Some(v) = &self.match_pattern {
            map.serialize_entry("match_pattern", v)?;
        }
        if let Some(v) = &self.path_match {
            map.serialize_entry("path_match", v)?;
        }
        if let Some(v) = &self.path_unmatch {
            map.serialize_entry("path_unmatch", v)?;
        }
        if let Some(v) = &self.unmatch {
            map.serialize_entry("unmatch", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct FieldAliasProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub path: Option<crate::types::common::Field>,
    pub r#type: crate::types::common_mapping::FieldAliasPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FieldAliasProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FieldAliasProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FieldAliasProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_path: Option<crate::types::common::Field> = None;
                let mut seen_path = false;
                let mut f_type: Option<crate::types::common_mapping::FieldAliasPropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "path" => {
                            if seen_path {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            seen_path = true;
                            f_path = map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::FieldAliasPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FieldAliasProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    path: f_path,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FieldAliasProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.path {
            map.serialize_entry("path", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldAliasPropertyType {
    #[doc = "`alias`"]
    Alias,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl FieldAliasPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            FieldAliasPropertyType::Alias => "alias",
            FieldAliasPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for FieldAliasPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FieldAliasPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "alias" => FieldAliasPropertyType::Alias,
            _ => FieldAliasPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct FieldMapping {
    pub full_name: String,
    pub mapping: std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FieldMapping {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FieldMapping;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FieldMapping object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_full_name: Option<String> = None;
                let mut f_mapping: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "full_name" => {
                            if f_full_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("full_name"));
                            }
                            f_full_name = Some(map.next_value::<String>()?);
                        }
                        "mapping" => {
                            if f_mapping.is_some() {
                                return Err(serde::de::Error::duplicate_field("mapping"));
                            }
                            f_mapping = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                crate::types::common_mapping::Property,
                            >>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FieldMapping {
                    full_name: f_full_name
                        .ok_or_else(|| serde::de::Error::missing_field("full_name"))?,
                    mapping: f_mapping.ok_or_else(|| serde::de::Error::missing_field("mapping"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FieldMapping {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("full_name", &self.full_name)?;
        map.serialize_entry("mapping", &self.mapping)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct FieldNamesField {
    pub enabled: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FieldNamesField {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FieldNamesField;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FieldNamesField object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_enabled: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "enabled" => {
                            if f_enabled.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            f_enabled = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FieldNamesField {
                    enabled: f_enabled.ok_or_else(|| serde::de::Error::missing_field("enabled"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FieldNamesField {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("enabled", &self.enabled)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldType {
    #[doc = "`aggregate_metric_double`"]
    AggregateMetricDouble,
    #[doc = "`alias`"]
    Alias,
    #[doc = "`binary`"]
    Binary,
    #[doc = "`boolean`"]
    Boolean,
    #[doc = "`byte`"]
    Byte,
    #[doc = "`completion`"]
    Completion,
    #[doc = "`constant_keyword`"]
    ConstantKeyword,
    #[doc = "`date`"]
    Date,
    #[doc = "`date_nanos`"]
    DateNanos,
    #[doc = "`date_range`"]
    DateRange,
    #[doc = "`double`"]
    Double,
    #[doc = "`double_range`"]
    DoubleRange,
    #[doc = "`flat_object`"]
    FlatObject,
    #[doc = "`float`"]
    Float,
    #[doc = "`float_range`"]
    FloatRange,
    #[doc = "`geo_point`"]
    GeoPoint,
    #[doc = "`geo_shape`"]
    GeoShape,
    #[doc = "`half_float`"]
    HalfFloat,
    #[doc = "`histogram`"]
    Histogram,
    #[doc = "`icu_collation_keyword`"]
    IcuCollationKeyword,
    #[doc = "`integer`"]
    Integer,
    #[doc = "`integer_range`"]
    IntegerRange,
    #[doc = "`ip`"]
    Ip,
    #[doc = "`ip_range`"]
    IpRange,
    #[doc = "`join`"]
    Join,
    #[doc = "`keyword`"]
    Keyword,
    #[doc = "`knn_vector`"]
    KnnVector,
    #[doc = "`long`"]
    Long,
    #[doc = "`long_range`"]
    LongRange,
    #[doc = "`match_only_text`"]
    MatchOnlyText,
    #[doc = "`murmur3`"]
    Murmur3,
    #[doc = "`nested`"]
    Nested,
    #[doc = "`object`"]
    Object,
    #[doc = "`percolator`"]
    Percolator,
    #[doc = "`rank_feature`"]
    RankFeature,
    #[doc = "`rank_features`"]
    RankFeatures,
    #[doc = "`scaled_float`"]
    ScaledFloat,
    #[doc = "`search_as_you_type`"]
    SearchAsYouType,
    #[doc = "`semantic`"]
    Semantic,
    #[doc = "`short`"]
    Short,
    #[doc = "`text`"]
    Text,
    #[doc = "`token_count`"]
    TokenCount,
    #[doc = "`unsigned_long`"]
    UnsignedLong,
    #[doc = "`version`"]
    Version,
    #[doc = "`wildcard`"]
    Wildcard,
    #[doc = "`xy_point`"]
    XyPoint,
    #[doc = "`xy_shape`"]
    XyShape,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl FieldType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            FieldType::AggregateMetricDouble => "aggregate_metric_double",
            FieldType::Alias => "alias",
            FieldType::Binary => "binary",
            FieldType::Boolean => "boolean",
            FieldType::Byte => "byte",
            FieldType::Completion => "completion",
            FieldType::ConstantKeyword => "constant_keyword",
            FieldType::Date => "date",
            FieldType::DateNanos => "date_nanos",
            FieldType::DateRange => "date_range",
            FieldType::Double => "double",
            FieldType::DoubleRange => "double_range",
            FieldType::FlatObject => "flat_object",
            FieldType::Float => "float",
            FieldType::FloatRange => "float_range",
            FieldType::GeoPoint => "geo_point",
            FieldType::GeoShape => "geo_shape",
            FieldType::HalfFloat => "half_float",
            FieldType::Histogram => "histogram",
            FieldType::IcuCollationKeyword => "icu_collation_keyword",
            FieldType::Integer => "integer",
            FieldType::IntegerRange => "integer_range",
            FieldType::Ip => "ip",
            FieldType::IpRange => "ip_range",
            FieldType::Join => "join",
            FieldType::Keyword => "keyword",
            FieldType::KnnVector => "knn_vector",
            FieldType::Long => "long",
            FieldType::LongRange => "long_range",
            FieldType::MatchOnlyText => "match_only_text",
            FieldType::Murmur3 => "murmur3",
            FieldType::Nested => "nested",
            FieldType::Object => "object",
            FieldType::Percolator => "percolator",
            FieldType::RankFeature => "rank_feature",
            FieldType::RankFeatures => "rank_features",
            FieldType::ScaledFloat => "scaled_float",
            FieldType::SearchAsYouType => "search_as_you_type",
            FieldType::Semantic => "semantic",
            FieldType::Short => "short",
            FieldType::Text => "text",
            FieldType::TokenCount => "token_count",
            FieldType::UnsignedLong => "unsigned_long",
            FieldType::Version => "version",
            FieldType::Wildcard => "wildcard",
            FieldType::XyPoint => "xy_point",
            FieldType::XyShape => "xy_shape",
            FieldType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for FieldType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FieldType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "aggregate_metric_double" => FieldType::AggregateMetricDouble,
            "alias" => FieldType::Alias,
            "binary" => FieldType::Binary,
            "boolean" => FieldType::Boolean,
            "byte" => FieldType::Byte,
            "completion" => FieldType::Completion,
            "constant_keyword" => FieldType::ConstantKeyword,
            "date" => FieldType::Date,
            "date_nanos" => FieldType::DateNanos,
            "date_range" => FieldType::DateRange,
            "double" => FieldType::Double,
            "double_range" => FieldType::DoubleRange,
            "flat_object" => FieldType::FlatObject,
            "float" => FieldType::Float,
            "float_range" => FieldType::FloatRange,
            "geo_point" => FieldType::GeoPoint,
            "geo_shape" => FieldType::GeoShape,
            "half_float" => FieldType::HalfFloat,
            "histogram" => FieldType::Histogram,
            "icu_collation_keyword" => FieldType::IcuCollationKeyword,
            "integer" => FieldType::Integer,
            "integer_range" => FieldType::IntegerRange,
            "ip" => FieldType::Ip,
            "ip_range" => FieldType::IpRange,
            "join" => FieldType::Join,
            "keyword" => FieldType::Keyword,
            "knn_vector" => FieldType::KnnVector,
            "long" => FieldType::Long,
            "long_range" => FieldType::LongRange,
            "match_only_text" => FieldType::MatchOnlyText,
            "murmur3" => FieldType::Murmur3,
            "nested" => FieldType::Nested,
            "object" => FieldType::Object,
            "percolator" => FieldType::Percolator,
            "rank_feature" => FieldType::RankFeature,
            "rank_features" => FieldType::RankFeatures,
            "scaled_float" => FieldType::ScaledFloat,
            "search_as_you_type" => FieldType::SearchAsYouType,
            "semantic" => FieldType::Semantic,
            "short" => FieldType::Short,
            "text" => FieldType::Text,
            "token_count" => FieldType::TokenCount,
            "unsigned_long" => FieldType::UnsignedLong,
            "version" => FieldType::Version,
            "wildcard" => FieldType::Wildcard,
            "xy_point" => FieldType::XyPoint,
            "xy_shape" => FieldType::XyShape,
            _ => FieldType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct FlatObjectProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub aggregatable: Option<bool>,
    pub searchable: Option<bool>,
    pub r#type: crate::types::common_mapping::FlatObjectPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FlatObjectProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FlatObjectProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FlatObjectProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_aggregatable: Option<bool> = None;
                let mut seen_aggregatable = false;
                let mut f_searchable: Option<bool> = None;
                let mut seen_searchable = false;
                let mut f_type: Option<crate::types::common_mapping::FlatObjectPropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "aggregatable" => {
                            if seen_aggregatable {
                                return Err(serde::de::Error::duplicate_field("aggregatable"));
                            }
                            seen_aggregatable = true;
                            f_aggregatable = map.next_value::<Option<bool>>()?;
                        }
                        "searchable" => {
                            if seen_searchable {
                                return Err(serde::de::Error::duplicate_field("searchable"));
                            }
                            seen_searchable = true;
                            f_searchable = map.next_value::<Option<bool>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::FlatObjectPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FlatObjectProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    aggregatable: f_aggregatable,
                    searchable: f_searchable,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FlatObjectProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.aggregatable {
            map.serialize_entry("aggregatable", v)?;
        }
        if let Some(v) = &self.searchable {
            map.serialize_entry("searchable", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FlatObjectPropertyType {
    #[doc = "`flat_object`"]
    FlatObject,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl FlatObjectPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            FlatObjectPropertyType::FlatObject => "flat_object",
            FlatObjectPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for FlatObjectPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FlatObjectPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "flat_object" => FlatObjectPropertyType::FlatObject,
            _ => FlatObjectPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct FloatNumberProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub boost: Option<f64>,
    pub coerce: Option<bool>,
    pub ignore_malformed: Option<bool>,
    pub index: Option<bool>,
    pub null_value: Option<f64>,
    pub r#type: crate::types::common_mapping::FloatNumberPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FloatNumberProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FloatNumberProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FloatNumberProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_coerce: Option<bool> = None;
                let mut seen_coerce = false;
                let mut f_ignore_malformed: Option<bool> = None;
                let mut seen_ignore_malformed = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_null_value: Option<f64> = None;
                let mut seen_null_value = false;
                let mut f_type: Option<crate::types::common_mapping::FloatNumberPropertyType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "coerce" => {
                            if seen_coerce {
                                return Err(serde::de::Error::duplicate_field("coerce"));
                            }
                            seen_coerce = true;
                            f_coerce = map.next_value::<Option<bool>>()?;
                        }
                        "ignore_malformed" => {
                            if seen_ignore_malformed {
                                return Err(serde::de::Error::duplicate_field("ignore_malformed"));
                            }
                            seen_ignore_malformed = true;
                            f_ignore_malformed = map.next_value::<Option<bool>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "null_value" => {
                            if seen_null_value {
                                return Err(serde::de::Error::duplicate_field("null_value"));
                            }
                            seen_null_value = true;
                            f_null_value = map.next_value::<Option<f64>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::FloatNumberPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FloatNumberProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    boost: f_boost,
                    coerce: f_coerce,
                    ignore_malformed: f_ignore_malformed,
                    index: f_index,
                    null_value: f_null_value,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FloatNumberProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.coerce {
            map.serialize_entry("coerce", v)?;
        }
        if let Some(v) = &self.ignore_malformed {
            map.serialize_entry("ignore_malformed", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.null_value {
            map.serialize_entry("null_value", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FloatNumberPropertyType {
    #[doc = "`float`"]
    Float,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl FloatNumberPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            FloatNumberPropertyType::Float => "float",
            FloatNumberPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for FloatNumberPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FloatNumberPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "float" => FloatNumberPropertyType::Float,
            _ => FloatNumberPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct FloatRangeProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub boost: Option<f64>,
    pub coerce: Option<bool>,
    pub index: Option<bool>,
    pub r#type: crate::types::common_mapping::FloatRangePropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FloatRangeProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FloatRangeProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FloatRangeProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_coerce: Option<bool> = None;
                let mut seen_coerce = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_type: Option<crate::types::common_mapping::FloatRangePropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "coerce" => {
                            if seen_coerce {
                                return Err(serde::de::Error::duplicate_field("coerce"));
                            }
                            seen_coerce = true;
                            f_coerce = map.next_value::<Option<bool>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::FloatRangePropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FloatRangeProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    boost: f_boost,
                    coerce: f_coerce,
                    index: f_index,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FloatRangeProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.coerce {
            map.serialize_entry("coerce", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FloatRangePropertyType {
    #[doc = "`float_range`"]
    FloatRange,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl FloatRangePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            FloatRangePropertyType::FloatRange => "float_range",
            FloatRangePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for FloatRangePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FloatRangePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "float_range" => FloatRangePropertyType::FloatRange,
            _ => FloatRangePropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GeoOrientation {
    #[doc = "`left` (also accepted: `LEFT`)"]
    Left,
    #[doc = "`clockwise`"]
    Clockwise,
    #[doc = "`cw`"]
    Cw,
    #[doc = "`right` (also accepted: `RIGHT`)"]
    Right,
    #[doc = "`counterclockwise`"]
    Counterclockwise,
    #[doc = "`ccw`"]
    Ccw,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl GeoOrientation {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            GeoOrientation::Left => "left",
            GeoOrientation::Clockwise => "clockwise",
            GeoOrientation::Cw => "cw",
            GeoOrientation::Right => "right",
            GeoOrientation::Counterclockwise => "counterclockwise",
            GeoOrientation::Ccw => "ccw",
            GeoOrientation::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for GeoOrientation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GeoOrientation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "left" | "LEFT" => GeoOrientation::Left,
            "clockwise" => GeoOrientation::Clockwise,
            "cw" => GeoOrientation::Cw,
            "right" | "RIGHT" => GeoOrientation::Right,
            "counterclockwise" => GeoOrientation::Counterclockwise,
            "ccw" => GeoOrientation::Ccw,
            _ => GeoOrientation::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct GeoPointProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub ignore_malformed: Option<bool>,
    pub ignore_z_value: Option<bool>,
    pub null_value: Option<crate::types::common::GeoLocation>,
    pub r#type: crate::types::common_mapping::GeoPointPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GeoPointProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GeoPointProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoPointProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_ignore_malformed: Option<bool> = None;
                let mut seen_ignore_malformed = false;
                let mut f_ignore_z_value: Option<bool> = None;
                let mut seen_ignore_z_value = false;
                let mut f_null_value: Option<crate::types::common::GeoLocation> = None;
                let mut seen_null_value = false;
                let mut f_type: Option<crate::types::common_mapping::GeoPointPropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "ignore_malformed" => {
                            if seen_ignore_malformed {
                                return Err(serde::de::Error::duplicate_field("ignore_malformed"));
                            }
                            seen_ignore_malformed = true;
                            f_ignore_malformed = map.next_value::<Option<bool>>()?;
                        }
                        "ignore_z_value" => {
                            if seen_ignore_z_value {
                                return Err(serde::de::Error::duplicate_field("ignore_z_value"));
                            }
                            seen_ignore_z_value = true;
                            f_ignore_z_value = map.next_value::<Option<bool>>()?;
                        }
                        "null_value" => {
                            if seen_null_value {
                                return Err(serde::de::Error::duplicate_field("null_value"));
                            }
                            seen_null_value = true;
                            f_null_value =
                                map.next_value::<Option<crate::types::common::GeoLocation>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::GeoPointPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GeoPointProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    ignore_malformed: f_ignore_malformed,
                    ignore_z_value: f_ignore_z_value,
                    null_value: f_null_value,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GeoPointProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.ignore_malformed {
            map.serialize_entry("ignore_malformed", v)?;
        }
        if let Some(v) = &self.ignore_z_value {
            map.serialize_entry("ignore_z_value", v)?;
        }
        if let Some(v) = &self.null_value {
            map.serialize_entry("null_value", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GeoPointPropertyType {
    #[doc = "`geo_point`"]
    GeoPoint,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl GeoPointPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            GeoPointPropertyType::GeoPoint => "geo_point",
            GeoPointPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for GeoPointPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GeoPointPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "geo_point" => GeoPointPropertyType::GeoPoint,
            _ => GeoPointPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct GeoShapeProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub coerce: Option<bool>,
    pub distance_error_pct: Option<f64>,
    pub ignore_malformed: Option<bool>,
    pub ignore_z_value: Option<bool>,
    pub orientation: Option<crate::types::common_mapping::GeoOrientation>,
    pub strategy: Option<crate::types::common_mapping::GeoStrategy>,
    pub r#type: crate::types::common_mapping::GeoShapePropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GeoShapeProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GeoShapeProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoShapeProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_coerce: Option<bool> = None;
                let mut seen_coerce = false;
                let mut f_distance_error_pct: Option<f64> = None;
                let mut seen_distance_error_pct = false;
                let mut f_ignore_malformed: Option<bool> = None;
                let mut seen_ignore_malformed = false;
                let mut f_ignore_z_value: Option<bool> = None;
                let mut seen_ignore_z_value = false;
                let mut f_orientation: Option<crate::types::common_mapping::GeoOrientation> = None;
                let mut seen_orientation = false;
                let mut f_strategy: Option<crate::types::common_mapping::GeoStrategy> = None;
                let mut seen_strategy = false;
                let mut f_type: Option<crate::types::common_mapping::GeoShapePropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "coerce" => {
                            if seen_coerce {
                                return Err(serde::de::Error::duplicate_field("coerce"));
                            }
                            seen_coerce = true;
                            f_coerce = map.next_value::<Option<bool>>()?;
                        }
                        "distance_error_pct" => {
                            if seen_distance_error_pct {
                                return Err(serde::de::Error::duplicate_field(
                                    "distance_error_pct",
                                ));
                            }
                            seen_distance_error_pct = true;
                            f_distance_error_pct = map.next_value::<Option<f64>>()?;
                        }
                        "ignore_malformed" => {
                            if seen_ignore_malformed {
                                return Err(serde::de::Error::duplicate_field("ignore_malformed"));
                            }
                            seen_ignore_malformed = true;
                            f_ignore_malformed = map.next_value::<Option<bool>>()?;
                        }
                        "ignore_z_value" => {
                            if seen_ignore_z_value {
                                return Err(serde::de::Error::duplicate_field("ignore_z_value"));
                            }
                            seen_ignore_z_value = true;
                            f_ignore_z_value = map.next_value::<Option<bool>>()?;
                        }
                        "orientation" => {
                            if seen_orientation {
                                return Err(serde::de::Error::duplicate_field("orientation"));
                            }
                            seen_orientation = true;
                            f_orientation = map
                                .next_value::<Option<crate::types::common_mapping::GeoOrientation>>(
                                )?;
                        }
                        "strategy" => {
                            if seen_strategy {
                                return Err(serde::de::Error::duplicate_field("strategy"));
                            }
                            seen_strategy = true;
                            f_strategy = map
                                .next_value::<Option<crate::types::common_mapping::GeoStrategy>>(
                                )?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::GeoShapePropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GeoShapeProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    coerce: f_coerce,
                    distance_error_pct: f_distance_error_pct,
                    ignore_malformed: f_ignore_malformed,
                    ignore_z_value: f_ignore_z_value,
                    orientation: f_orientation,
                    strategy: f_strategy,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GeoShapeProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.coerce {
            map.serialize_entry("coerce", v)?;
        }
        if let Some(v) = &self.distance_error_pct {
            map.serialize_entry("distance_error_pct", v)?;
        }
        if let Some(v) = &self.ignore_malformed {
            map.serialize_entry("ignore_malformed", v)?;
        }
        if let Some(v) = &self.ignore_z_value {
            map.serialize_entry("ignore_z_value", v)?;
        }
        if let Some(v) = &self.orientation {
            map.serialize_entry("orientation", v)?;
        }
        if let Some(v) = &self.strategy {
            map.serialize_entry("strategy", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GeoShapePropertyType {
    #[doc = "`geo_shape`"]
    GeoShape,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl GeoShapePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            GeoShapePropertyType::GeoShape => "geo_shape",
            GeoShapePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for GeoShapePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GeoShapePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "geo_shape" => GeoShapePropertyType::GeoShape,
            _ => GeoShapePropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GeoStrategy {
    #[doc = "`recursive`"]
    Recursive,
    #[doc = "`term`"]
    Term,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl GeoStrategy {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            GeoStrategy::Recursive => "recursive",
            GeoStrategy::Term => "term",
            GeoStrategy::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for GeoStrategy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GeoStrategy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "recursive" => GeoStrategy::Recursive,
            "term" => GeoStrategy::Term,
            _ => GeoStrategy::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct HalfFloatNumberProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub boost: Option<f64>,
    pub coerce: Option<bool>,
    pub ignore_malformed: Option<bool>,
    pub index: Option<bool>,
    pub null_value: Option<f64>,
    pub r#type: crate::types::common_mapping::HalfFloatNumberPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HalfFloatNumberProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HalfFloatNumberProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HalfFloatNumberProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_coerce: Option<bool> = None;
                let mut seen_coerce = false;
                let mut f_ignore_malformed: Option<bool> = None;
                let mut seen_ignore_malformed = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_null_value: Option<f64> = None;
                let mut seen_null_value = false;
                let mut f_type: Option<crate::types::common_mapping::HalfFloatNumberPropertyType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "coerce" => {
                            if seen_coerce {
                                return Err(serde::de::Error::duplicate_field("coerce"));
                            }
                            seen_coerce = true;
                            f_coerce = map.next_value::<Option<bool>>()?;
                        }
                        "ignore_malformed" => {
                            if seen_ignore_malformed {
                                return Err(serde::de::Error::duplicate_field("ignore_malformed"));
                            }
                            seen_ignore_malformed = true;
                            f_ignore_malformed = map.next_value::<Option<bool>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "null_value" => {
                            if seen_null_value {
                                return Err(serde::de::Error::duplicate_field("null_value"));
                            }
                            seen_null_value = true;
                            f_null_value = map.next_value::<Option<f64>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::HalfFloatNumberPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HalfFloatNumberProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    boost: f_boost,
                    coerce: f_coerce,
                    ignore_malformed: f_ignore_malformed,
                    index: f_index,
                    null_value: f_null_value,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HalfFloatNumberProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.coerce {
            map.serialize_entry("coerce", v)?;
        }
        if let Some(v) = &self.ignore_malformed {
            map.serialize_entry("ignore_malformed", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.null_value {
            map.serialize_entry("null_value", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HalfFloatNumberPropertyType {
    #[doc = "`half_float`"]
    HalfFloat,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl HalfFloatNumberPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            HalfFloatNumberPropertyType::HalfFloat => "half_float",
            HalfFloatNumberPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for HalfFloatNumberPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HalfFloatNumberPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "half_float" => HalfFloatNumberPropertyType::HalfFloat,
            _ => HalfFloatNumberPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct HistogramProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_malformed: Option<bool>,
    pub r#type: crate::types::common_mapping::HistogramPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HistogramProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HistogramProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HistogramProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_ignore_malformed: Option<bool> = None;
                let mut seen_ignore_malformed = false;
                let mut f_type: Option<crate::types::common_mapping::HistogramPropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_malformed" => {
                            if seen_ignore_malformed {
                                return Err(serde::de::Error::duplicate_field("ignore_malformed"));
                            }
                            seen_ignore_malformed = true;
                            f_ignore_malformed = map.next_value::<Option<bool>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::HistogramPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HistogramProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    ignore_malformed: f_ignore_malformed,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HistogramProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.ignore_malformed {
            map.serialize_entry("ignore_malformed", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HistogramPropertyType {
    #[doc = "`histogram`"]
    Histogram,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl HistogramPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            HistogramPropertyType::Histogram => "histogram",
            HistogramPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for HistogramPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HistogramPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "histogram" => HistogramPropertyType::Histogram,
            _ => HistogramPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct IcuCollationKeywordProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub alternate: Option<crate::types::common_analysis::IcuCollationAlternate>,
    pub case_first: Option<crate::types::common_analysis::IcuCollationCaseFirst>,
    pub case_level: Option<bool>,
    pub country: Option<String>,
    pub decomposition: Option<crate::types::common_analysis::IcuCollationDecomposition>,
    pub hiragana_quaternary_mode: Option<bool>,
    pub index: Option<bool>,
    pub language: Option<String>,
    pub null_value: Option<String>,
    pub numeric: Option<bool>,
    pub strength: Option<crate::types::common_analysis::IcuCollationStrength>,
    pub r#type: crate::types::common_mapping::IcuCollationKeywordPropertyType,
    pub variable_top: Option<String>,
    pub variant: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IcuCollationKeywordProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IcuCollationKeywordProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IcuCollationKeywordProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_alternate: Option<crate::types::common_analysis::IcuCollationAlternate> =
                    None;
                let mut seen_alternate = false;
                let mut f_case_first: Option<crate::types::common_analysis::IcuCollationCaseFirst> =
                    None;
                let mut seen_case_first = false;
                let mut f_case_level: Option<bool> = None;
                let mut seen_case_level = false;
                let mut f_country: Option<String> = None;
                let mut seen_country = false;
                let mut f_decomposition: Option<
                    crate::types::common_analysis::IcuCollationDecomposition,
                > = None;
                let mut seen_decomposition = false;
                let mut f_hiragana_quaternary_mode: Option<bool> = None;
                let mut seen_hiragana_quaternary_mode = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_language: Option<String> = None;
                let mut seen_language = false;
                let mut f_null_value: Option<String> = None;
                let mut seen_null_value = false;
                let mut f_numeric: Option<bool> = None;
                let mut seen_numeric = false;
                let mut f_strength: Option<crate::types::common_analysis::IcuCollationStrength> =
                    None;
                let mut seen_strength = false;
                let mut f_type: Option<
                    crate::types::common_mapping::IcuCollationKeywordPropertyType,
                > = None;
                let mut f_variable_top: Option<String> = None;
                let mut seen_variable_top = false;
                let mut f_variant: Option<String> = None;
                let mut seen_variant = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "alternate" => {
                            if seen_alternate {
                                return Err(serde::de::Error::duplicate_field("alternate"));
                            }
                            seen_alternate = true;
                            f_alternate = map.next_value::<Option<crate::types::common_analysis::IcuCollationAlternate>>()?;
                        }
                        "case_first" => {
                            if seen_case_first {
                                return Err(serde::de::Error::duplicate_field("case_first"));
                            }
                            seen_case_first = true;
                            f_case_first = map.next_value::<Option<crate::types::common_analysis::IcuCollationCaseFirst>>()?;
                        }
                        "case_level" => {
                            if seen_case_level {
                                return Err(serde::de::Error::duplicate_field("case_level"));
                            }
                            seen_case_level = true;
                            f_case_level = map.next_value::<Option<bool>>()?;
                        }
                        "country" => {
                            if seen_country {
                                return Err(serde::de::Error::duplicate_field("country"));
                            }
                            seen_country = true;
                            f_country = map.next_value::<Option<String>>()?;
                        }
                        "decomposition" => {
                            if seen_decomposition {
                                return Err(serde::de::Error::duplicate_field("decomposition"));
                            }
                            seen_decomposition = true;
                            f_decomposition =
                                map.next_value::<Option<
                                    crate::types::common_analysis::IcuCollationDecomposition,
                                >>()?;
                        }
                        "hiragana_quaternary_mode" => {
                            if seen_hiragana_quaternary_mode {
                                return Err(serde::de::Error::duplicate_field(
                                    "hiragana_quaternary_mode",
                                ));
                            }
                            seen_hiragana_quaternary_mode = true;
                            f_hiragana_quaternary_mode = map.next_value::<Option<bool>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "language" => {
                            if seen_language {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            seen_language = true;
                            f_language = map.next_value::<Option<String>>()?;
                        }
                        "null_value" => {
                            if seen_null_value {
                                return Err(serde::de::Error::duplicate_field("null_value"));
                            }
                            seen_null_value = true;
                            f_null_value = map.next_value::<Option<String>>()?;
                        }
                        "numeric" => {
                            if seen_numeric {
                                return Err(serde::de::Error::duplicate_field("numeric"));
                            }
                            seen_numeric = true;
                            f_numeric = map.next_value::<Option<bool>>()?;
                        }
                        "strength" => {
                            if seen_strength {
                                return Err(serde::de::Error::duplicate_field("strength"));
                            }
                            seen_strength = true;
                            f_strength = map.next_value::<Option<crate::types::common_analysis::IcuCollationStrength>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::IcuCollationKeywordPropertyType>()?);
                        }
                        "variable_top" => {
                            if seen_variable_top {
                                return Err(serde::de::Error::duplicate_field("variable_top"));
                            }
                            seen_variable_top = true;
                            f_variable_top = map.next_value::<Option<String>>()?;
                        }
                        "variant" => {
                            if seen_variant {
                                return Err(serde::de::Error::duplicate_field("variant"));
                            }
                            seen_variant = true;
                            f_variant = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IcuCollationKeywordProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    alternate: f_alternate,
                    case_first: f_case_first,
                    case_level: f_case_level,
                    country: f_country,
                    decomposition: f_decomposition,
                    hiragana_quaternary_mode: f_hiragana_quaternary_mode,
                    index: f_index,
                    language: f_language,
                    null_value: f_null_value,
                    numeric: f_numeric,
                    strength: f_strength,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    variable_top: f_variable_top,
                    variant: f_variant,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IcuCollationKeywordProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.alternate {
            map.serialize_entry("alternate", v)?;
        }
        if let Some(v) = &self.case_first {
            map.serialize_entry("case_first", v)?;
        }
        if let Some(v) = &self.case_level {
            map.serialize_entry("case_level", v)?;
        }
        if let Some(v) = &self.country {
            map.serialize_entry("country", v)?;
        }
        if let Some(v) = &self.decomposition {
            map.serialize_entry("decomposition", v)?;
        }
        if let Some(v) = &self.hiragana_quaternary_mode {
            map.serialize_entry("hiragana_quaternary_mode", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.language {
            map.serialize_entry("language", v)?;
        }
        if let Some(v) = &self.null_value {
            map.serialize_entry("null_value", v)?;
        }
        if let Some(v) = &self.numeric {
            map.serialize_entry("numeric", v)?;
        }
        if let Some(v) = &self.strength {
            map.serialize_entry("strength", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        if let Some(v) = &self.variable_top {
            map.serialize_entry("variable_top", v)?;
        }
        if let Some(v) = &self.variant {
            map.serialize_entry("variant", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuCollationKeywordPropertyType {
    #[doc = "`icu_collation_keyword`"]
    IcuCollationKeyword,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuCollationKeywordPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuCollationKeywordPropertyType::IcuCollationKeyword => "icu_collation_keyword",
            IcuCollationKeywordPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuCollationKeywordPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuCollationKeywordPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "icu_collation_keyword" => IcuCollationKeywordPropertyType::IcuCollationKeyword,
            _ => IcuCollationKeywordPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct IndexField {
    pub enabled: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexField {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexField;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexField object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_enabled: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "enabled" => {
                            if f_enabled.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            f_enabled = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexField {
                    enabled: f_enabled.ok_or_else(|| serde::de::Error::missing_field("enabled"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexField {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("enabled", &self.enabled)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexOptions {
    #[doc = "`docs`"]
    Docs,
    #[doc = "`freqs`"]
    Freqs,
    #[doc = "`offsets`"]
    Offsets,
    #[doc = "`positions`"]
    Positions,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexOptions {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexOptions::Docs => "docs",
            IndexOptions::Freqs => "freqs",
            IndexOptions::Offsets => "offsets",
            IndexOptions::Positions => "positions",
            IndexOptions::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexOptions {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexOptions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "docs" => IndexOptions::Docs,
            "freqs" => IndexOptions::Freqs,
            "offsets" => IndexOptions::Offsets,
            "positions" => IndexOptions::Positions,
            _ => IndexOptions::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct IntegerNumberProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub boost: Option<f64>,
    pub coerce: Option<bool>,
    pub ignore_malformed: Option<bool>,
    pub index: Option<bool>,
    pub null_value: Option<i64>,
    pub r#type: crate::types::common_mapping::IntegerNumberPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IntegerNumberProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IntegerNumberProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IntegerNumberProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_coerce: Option<bool> = None;
                let mut seen_coerce = false;
                let mut f_ignore_malformed: Option<bool> = None;
                let mut seen_ignore_malformed = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_null_value: Option<i64> = None;
                let mut seen_null_value = false;
                let mut f_type: Option<crate::types::common_mapping::IntegerNumberPropertyType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "coerce" => {
                            if seen_coerce {
                                return Err(serde::de::Error::duplicate_field("coerce"));
                            }
                            seen_coerce = true;
                            f_coerce = map.next_value::<Option<bool>>()?;
                        }
                        "ignore_malformed" => {
                            if seen_ignore_malformed {
                                return Err(serde::de::Error::duplicate_field("ignore_malformed"));
                            }
                            seen_ignore_malformed = true;
                            f_ignore_malformed = map.next_value::<Option<bool>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "null_value" => {
                            if seen_null_value {
                                return Err(serde::de::Error::duplicate_field("null_value"));
                            }
                            seen_null_value = true;
                            f_null_value = map.next_value::<Option<i64>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::IntegerNumberPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IntegerNumberProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    boost: f_boost,
                    coerce: f_coerce,
                    ignore_malformed: f_ignore_malformed,
                    index: f_index,
                    null_value: f_null_value,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IntegerNumberProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.coerce {
            map.serialize_entry("coerce", v)?;
        }
        if let Some(v) = &self.ignore_malformed {
            map.serialize_entry("ignore_malformed", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.null_value {
            map.serialize_entry("null_value", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntegerNumberPropertyType {
    #[doc = "`integer`"]
    Integer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IntegerNumberPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IntegerNumberPropertyType::Integer => "integer",
            IntegerNumberPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IntegerNumberPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IntegerNumberPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "integer" => IntegerNumberPropertyType::Integer,
            _ => IntegerNumberPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct IntegerRangeProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub boost: Option<f64>,
    pub coerce: Option<bool>,
    pub index: Option<bool>,
    pub r#type: crate::types::common_mapping::IntegerRangePropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IntegerRangeProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IntegerRangeProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IntegerRangeProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_coerce: Option<bool> = None;
                let mut seen_coerce = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_type: Option<crate::types::common_mapping::IntegerRangePropertyType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "coerce" => {
                            if seen_coerce {
                                return Err(serde::de::Error::duplicate_field("coerce"));
                            }
                            seen_coerce = true;
                            f_coerce = map.next_value::<Option<bool>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::IntegerRangePropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IntegerRangeProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    boost: f_boost,
                    coerce: f_coerce,
                    index: f_index,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IntegerRangeProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.coerce {
            map.serialize_entry("coerce", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntegerRangePropertyType {
    #[doc = "`integer_range`"]
    IntegerRange,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IntegerRangePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IntegerRangePropertyType::IntegerRange => "integer_range",
            IntegerRangePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IntegerRangePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IntegerRangePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "integer_range" => IntegerRangePropertyType::IntegerRange,
            _ => IntegerRangePropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct IpProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub boost: Option<f64>,
    pub ignore_malformed: Option<bool>,
    pub index: Option<bool>,
    pub null_value: Option<String>,
    pub r#type: crate::types::common_mapping::IpPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IpProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IpProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IpProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_ignore_malformed: Option<bool> = None;
                let mut seen_ignore_malformed = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_null_value: Option<String> = None;
                let mut seen_null_value = false;
                let mut f_type: Option<crate::types::common_mapping::IpPropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "ignore_malformed" => {
                            if seen_ignore_malformed {
                                return Err(serde::de::Error::duplicate_field("ignore_malformed"));
                            }
                            seen_ignore_malformed = true;
                            f_ignore_malformed = map.next_value::<Option<bool>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "null_value" => {
                            if seen_null_value {
                                return Err(serde::de::Error::duplicate_field("null_value"));
                            }
                            seen_null_value = true;
                            f_null_value = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(
                                map.next_value::<crate::types::common_mapping::IpPropertyType>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IpProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    boost: f_boost,
                    ignore_malformed: f_ignore_malformed,
                    index: f_index,
                    null_value: f_null_value,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IpProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.ignore_malformed {
            map.serialize_entry("ignore_malformed", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.null_value {
            map.serialize_entry("null_value", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IpPropertyType {
    #[doc = "`ip`"]
    Ip,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IpPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IpPropertyType::Ip => "ip",
            IpPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IpPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IpPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ip" => IpPropertyType::Ip,
            _ => IpPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct IpRangeProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub boost: Option<f64>,
    pub coerce: Option<bool>,
    pub index: Option<bool>,
    pub r#type: crate::types::common_mapping::IpRangePropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IpRangeProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IpRangeProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IpRangeProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_coerce: Option<bool> = None;
                let mut seen_coerce = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_type: Option<crate::types::common_mapping::IpRangePropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "coerce" => {
                            if seen_coerce {
                                return Err(serde::de::Error::duplicate_field("coerce"));
                            }
                            seen_coerce = true;
                            f_coerce = map.next_value::<Option<bool>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::IpRangePropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IpRangeProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    boost: f_boost,
                    coerce: f_coerce,
                    index: f_index,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IpRangeProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.coerce {
            map.serialize_entry("coerce", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IpRangePropertyType {
    #[doc = "`ip_range`"]
    IpRange,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IpRangePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IpRangePropertyType::IpRange => "ip_range",
            IpRangePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IpRangePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IpRangePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ip_range" => IpRangePropertyType::IpRange,
            _ => IpRangePropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct JoinProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub eager_global_ordinals: Option<bool>,
    pub relations: Option<
        std::collections::BTreeMap<
            String,
            crate::types::common_mapping::JoinPropertyRelationsValue,
        >,
    >,
    pub r#type: crate::types::common_mapping::JoinPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for JoinProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = JoinProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a JoinProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_eager_global_ordinals: Option<bool> = None;
                let mut seen_eager_global_ordinals = false;
                let mut f_relations: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::common_mapping::JoinPropertyRelationsValue,
                    >,
                > = None;
                let mut seen_relations = false;
                let mut f_type: Option<crate::types::common_mapping::JoinPropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "eager_global_ordinals" => {
                            if seen_eager_global_ordinals {
                                return Err(serde::de::Error::duplicate_field(
                                    "eager_global_ordinals",
                                ));
                            }
                            seen_eager_global_ordinals = true;
                            f_eager_global_ordinals = map.next_value::<Option<bool>>()?;
                        }
                        "relations" => {
                            if seen_relations {
                                return Err(serde::de::Error::duplicate_field("relations"));
                            }
                            seen_relations = true;
                            f_relations = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::JoinPropertyRelationsValue,
                                >,
                            >>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(
                                map.next_value::<crate::types::common_mapping::JoinPropertyType>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(JoinProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    eager_global_ordinals: f_eager_global_ordinals,
                    relations: f_relations,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for JoinProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.eager_global_ordinals {
            map.serialize_entry("eager_global_ordinals", v)?;
        }
        if let Some(v) = &self.relations {
            map.serialize_entry("relations", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum JoinPropertyRelationsValue {
    RelationName(crate::types::common::RelationName),
    Array(Vec<crate::types::common::RelationName>),
}
impl<'de> serde::Deserialize<'de> for JoinPropertyRelationsValue {
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
            Some(b'"') => serde_json::from_str(text).map(JoinPropertyRelationsValue::RelationName),
            Some(b'[') => serde_json::from_str(text).map(JoinPropertyRelationsValue::Array),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a JoinPropertyRelationsValue value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JoinPropertyType {
    #[doc = "`join`"]
    Join,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl JoinPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            JoinPropertyType::Join => "join",
            JoinPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for JoinPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for JoinPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "join" => JoinPropertyType::Join,
            _ => JoinPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct KeywordProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub boost: Option<f64>,
    pub eager_global_ordinals: Option<bool>,
    pub index: Option<bool>,
    pub index_options: Option<crate::types::common_mapping::IndexOptions>,
    pub normalizer: Option<String>,
    pub norms: Option<bool>,
    pub null_value: Option<String>,
    pub split_queries_on_whitespace: Option<bool>,
    pub r#type: crate::types::common_mapping::KeywordPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for KeywordProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = KeywordProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a KeywordProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_eager_global_ordinals: Option<bool> = None;
                let mut seen_eager_global_ordinals = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_index_options: Option<crate::types::common_mapping::IndexOptions> = None;
                let mut seen_index_options = false;
                let mut f_normalizer: Option<String> = None;
                let mut seen_normalizer = false;
                let mut f_norms: Option<bool> = None;
                let mut seen_norms = false;
                let mut f_null_value: Option<String> = None;
                let mut seen_null_value = false;
                let mut f_split_queries_on_whitespace: Option<bool> = None;
                let mut seen_split_queries_on_whitespace = false;
                let mut f_type: Option<crate::types::common_mapping::KeywordPropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "eager_global_ordinals" => {
                            if seen_eager_global_ordinals {
                                return Err(serde::de::Error::duplicate_field(
                                    "eager_global_ordinals",
                                ));
                            }
                            seen_eager_global_ordinals = true;
                            f_eager_global_ordinals = map.next_value::<Option<bool>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "index_options" => {
                            if seen_index_options {
                                return Err(serde::de::Error::duplicate_field("index_options"));
                            }
                            seen_index_options = true;
                            f_index_options = map
                                .next_value::<Option<crate::types::common_mapping::IndexOptions>>(
                                )?;
                        }
                        "normalizer" => {
                            if seen_normalizer {
                                return Err(serde::de::Error::duplicate_field("normalizer"));
                            }
                            seen_normalizer = true;
                            f_normalizer = map.next_value::<Option<String>>()?;
                        }
                        "norms" => {
                            if seen_norms {
                                return Err(serde::de::Error::duplicate_field("norms"));
                            }
                            seen_norms = true;
                            f_norms = map.next_value::<Option<bool>>()?;
                        }
                        "null_value" => {
                            if seen_null_value {
                                return Err(serde::de::Error::duplicate_field("null_value"));
                            }
                            seen_null_value = true;
                            f_null_value = map.next_value::<Option<String>>()?;
                        }
                        "split_queries_on_whitespace" => {
                            if seen_split_queries_on_whitespace {
                                return Err(serde::de::Error::duplicate_field(
                                    "split_queries_on_whitespace",
                                ));
                            }
                            seen_split_queries_on_whitespace = true;
                            f_split_queries_on_whitespace = map.next_value::<Option<bool>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::KeywordPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(KeywordProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    boost: f_boost,
                    eager_global_ordinals: f_eager_global_ordinals,
                    index: f_index,
                    index_options: f_index_options,
                    normalizer: f_normalizer,
                    norms: f_norms,
                    null_value: f_null_value,
                    split_queries_on_whitespace: f_split_queries_on_whitespace,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for KeywordProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.eager_global_ordinals {
            map.serialize_entry("eager_global_ordinals", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.index_options {
            map.serialize_entry("index_options", v)?;
        }
        if let Some(v) = &self.normalizer {
            map.serialize_entry("normalizer", v)?;
        }
        if let Some(v) = &self.norms {
            map.serialize_entry("norms", v)?;
        }
        if let Some(v) = &self.null_value {
            map.serialize_entry("null_value", v)?;
        }
        if let Some(v) = &self.split_queries_on_whitespace {
            map.serialize_entry("split_queries_on_whitespace", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeywordPropertyType {
    #[doc = "`keyword`"]
    Keyword,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KeywordPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KeywordPropertyType::Keyword => "keyword",
            KeywordPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KeywordPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KeywordPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "keyword" => KeywordPropertyType::Keyword,
            _ => KeywordPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct KnnVectorMethod {
    pub engine: Option<String>,
    pub name: String,
    pub parameters: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub space_type: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for KnnVectorMethod {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = KnnVectorMethod;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a KnnVectorMethod object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_engine: Option<String> = None;
                let mut seen_engine = false;
                let mut f_name: Option<String> = None;
                let mut f_parameters: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_parameters = false;
                let mut f_space_type: Option<String> = None;
                let mut seen_space_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "engine" => {
                            if seen_engine {
                                return Err(serde::de::Error::duplicate_field("engine"));
                            }
                            seen_engine = true;
                            f_engine = map.next_value::<Option<String>>()?;
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<String>()?);
                        }
                        "parameters" => {
                            if seen_parameters {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            seen_parameters = true;
                            f_parameters = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "space_type" => {
                            if seen_space_type {
                                return Err(serde::de::Error::duplicate_field("space_type"));
                            }
                            seen_space_type = true;
                            f_space_type = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(KnnVectorMethod {
                    engine: f_engine,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    parameters: f_parameters,
                    space_type: f_space_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for KnnVectorMethod {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.engine {
            map.serialize_entry("engine", v)?;
        }
        map.serialize_entry("name", &self.name)?;
        if let Some(v) = &self.parameters {
            map.serialize_entry("parameters", v)?;
        }
        if let Some(v) = &self.space_type {
            map.serialize_entry("space_type", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct KnnVectorProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub compression_level: Option<String>,
    pub data_type: Option<String>,
    pub dimension: i64,
    pub method: Option<crate::types::common_mapping::KnnVectorMethod>,
    pub mode: Option<String>,
    pub model_id: Option<String>,
    pub space_type: Option<String>,
    pub r#type: crate::types::common_mapping::KnnVectorPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for KnnVectorProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = KnnVectorProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a KnnVectorProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_compression_level: Option<String> = None;
                let mut seen_compression_level = false;
                let mut f_data_type: Option<String> = None;
                let mut seen_data_type = false;
                let mut f_dimension: Option<i64> = None;
                let mut f_method: Option<crate::types::common_mapping::KnnVectorMethod> = None;
                let mut seen_method = false;
                let mut f_mode: Option<String> = None;
                let mut seen_mode = false;
                let mut f_model_id: Option<String> = None;
                let mut seen_model_id = false;
                let mut f_space_type: Option<String> = None;
                let mut seen_space_type = false;
                let mut f_type: Option<crate::types::common_mapping::KnnVectorPropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "compression_level" => {
                            if seen_compression_level {
                                return Err(serde::de::Error::duplicate_field("compression_level"));
                            }
                            seen_compression_level = true;
                            f_compression_level = map.next_value::<Option<String>>()?;
                        }
                        "data_type" => {
                            if seen_data_type {
                                return Err(serde::de::Error::duplicate_field("data_type"));
                            }
                            seen_data_type = true;
                            f_data_type = map.next_value::<Option<String>>()?;
                        }
                        "dimension" => {
                            if f_dimension.is_some() {
                                return Err(serde::de::Error::duplicate_field("dimension"));
                            }
                            f_dimension = Some(map.next_value::<i64>()?);
                        }
                        "method" => {
                            if seen_method {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            seen_method = true;
                            f_method = map.next_value::<Option<crate::types::common_mapping::KnnVectorMethod>>()?;
                        }
                        "mode" => {
                            if seen_mode {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            seen_mode = true;
                            f_mode = map.next_value::<Option<String>>()?;
                        }
                        "model_id" => {
                            if seen_model_id {
                                return Err(serde::de::Error::duplicate_field("model_id"));
                            }
                            seen_model_id = true;
                            f_model_id = map.next_value::<Option<String>>()?;
                        }
                        "space_type" => {
                            if seen_space_type {
                                return Err(serde::de::Error::duplicate_field("space_type"));
                            }
                            seen_space_type = true;
                            f_space_type = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::KnnVectorPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(KnnVectorProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    compression_level: f_compression_level,
                    data_type: f_data_type,
                    dimension: f_dimension
                        .ok_or_else(|| serde::de::Error::missing_field("dimension"))?,
                    method: f_method,
                    mode: f_mode,
                    model_id: f_model_id,
                    space_type: f_space_type,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for KnnVectorProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.compression_level {
            map.serialize_entry("compression_level", v)?;
        }
        if let Some(v) = &self.data_type {
            map.serialize_entry("data_type", v)?;
        }
        map.serialize_entry("dimension", &self.dimension)?;
        if let Some(v) = &self.method {
            map.serialize_entry("method", v)?;
        }
        if let Some(v) = &self.mode {
            map.serialize_entry("mode", v)?;
        }
        if let Some(v) = &self.model_id {
            map.serialize_entry("model_id", v)?;
        }
        if let Some(v) = &self.space_type {
            map.serialize_entry("space_type", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KnnVectorPropertyType {
    #[doc = "`knn_vector`"]
    KnnVector,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KnnVectorPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KnnVectorPropertyType::KnnVector => "knn_vector",
            KnnVectorPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KnnVectorPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KnnVectorPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "knn_vector" => KnnVectorPropertyType::KnnVector,
            _ => KnnVectorPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct LongNumberProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub boost: Option<f64>,
    pub coerce: Option<bool>,
    pub ignore_malformed: Option<bool>,
    pub index: Option<bool>,
    pub null_value: Option<i64>,
    pub r#type: crate::types::common_mapping::LongNumberPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for LongNumberProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = LongNumberProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a LongNumberProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_coerce: Option<bool> = None;
                let mut seen_coerce = false;
                let mut f_ignore_malformed: Option<bool> = None;
                let mut seen_ignore_malformed = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_null_value: Option<i64> = None;
                let mut seen_null_value = false;
                let mut f_type: Option<crate::types::common_mapping::LongNumberPropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "coerce" => {
                            if seen_coerce {
                                return Err(serde::de::Error::duplicate_field("coerce"));
                            }
                            seen_coerce = true;
                            f_coerce = map.next_value::<Option<bool>>()?;
                        }
                        "ignore_malformed" => {
                            if seen_ignore_malformed {
                                return Err(serde::de::Error::duplicate_field("ignore_malformed"));
                            }
                            seen_ignore_malformed = true;
                            f_ignore_malformed = map.next_value::<Option<bool>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "null_value" => {
                            if seen_null_value {
                                return Err(serde::de::Error::duplicate_field("null_value"));
                            }
                            seen_null_value = true;
                            f_null_value = map.next_value::<Option<i64>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::LongNumberPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(LongNumberProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    boost: f_boost,
                    coerce: f_coerce,
                    ignore_malformed: f_ignore_malformed,
                    index: f_index,
                    null_value: f_null_value,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for LongNumberProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.coerce {
            map.serialize_entry("coerce", v)?;
        }
        if let Some(v) = &self.ignore_malformed {
            map.serialize_entry("ignore_malformed", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.null_value {
            map.serialize_entry("null_value", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LongNumberPropertyType {
    #[doc = "`long`"]
    Long,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl LongNumberPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            LongNumberPropertyType::Long => "long",
            LongNumberPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for LongNumberPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LongNumberPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "long" => LongNumberPropertyType::Long,
            _ => LongNumberPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct LongRangeProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub boost: Option<f64>,
    pub coerce: Option<bool>,
    pub index: Option<bool>,
    pub r#type: crate::types::common_mapping::LongRangePropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for LongRangeProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = LongRangeProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a LongRangeProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_coerce: Option<bool> = None;
                let mut seen_coerce = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_type: Option<crate::types::common_mapping::LongRangePropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "coerce" => {
                            if seen_coerce {
                                return Err(serde::de::Error::duplicate_field("coerce"));
                            }
                            seen_coerce = true;
                            f_coerce = map.next_value::<Option<bool>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::LongRangePropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(LongRangeProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    boost: f_boost,
                    coerce: f_coerce,
                    index: f_index,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for LongRangeProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.coerce {
            map.serialize_entry("coerce", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LongRangePropertyType {
    #[doc = "`long_range`"]
    LongRange,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl LongRangePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            LongRangePropertyType::LongRange => "long_range",
            LongRangePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for LongRangePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LongRangePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "long_range" => LongRangePropertyType::LongRange,
            _ => LongRangePropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct MatchOnlyTextProperty {
    pub copy_to: Option<crate::types::common::Fields>,
    #[doc = "Multi-fields allow the same string value to be indexed in multiple ways for different purposes, such as one field for search and a multi-field for sorting and aggregations, or the same string value analyzed by different analyzers."]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub r#type: crate::types::common_mapping::MatchOnlyTextPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MatchOnlyTextProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MatchOnlyTextProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MatchOnlyTextProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_type: Option<crate::types::common_mapping::MatchOnlyTextPropertyType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::MatchOnlyTextPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MatchOnlyTextProperty {
                    copy_to: f_copy_to,
                    fields: f_fields,
                    meta: f_meta,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MatchOnlyTextProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MatchOnlyTextPropertyType {
    #[doc = "`match_only_text`"]
    MatchOnlyText,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl MatchOnlyTextPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            MatchOnlyTextPropertyType::MatchOnlyText => "match_only_text",
            MatchOnlyTextPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for MatchOnlyTextPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MatchOnlyTextPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "match_only_text" => MatchOnlyTextPropertyType::MatchOnlyText,
            _ => MatchOnlyTextPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MatchType {
    #[doc = "`regex`"]
    Regex,
    #[doc = "`simple`"]
    Simple,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl MatchType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            MatchType::Regex => "regex",
            MatchType::Simple => "simple",
            MatchType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for MatchType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MatchType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "regex" => MatchType::Regex,
            "simple" => MatchType::Simple,
            _ => MatchType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Murmur3HashProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub r#type: crate::types::common_mapping::Murmur3HashPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Murmur3HashProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Murmur3HashProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Murmur3HashProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_type: Option<crate::types::common_mapping::Murmur3HashPropertyType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::Murmur3HashPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Murmur3HashProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Murmur3HashProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Murmur3HashPropertyType {
    #[doc = "`murmur3`"]
    Murmur3,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl Murmur3HashPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            Murmur3HashPropertyType::Murmur3 => "murmur3",
            Murmur3HashPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for Murmur3HashPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for Murmur3HashPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "murmur3" => Murmur3HashPropertyType::Murmur3,
            _ => Murmur3HashPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct NestedProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub enabled: Option<bool>,
    pub include_in_parent: Option<bool>,
    pub include_in_root: Option<bool>,
    pub r#type: crate::types::common_mapping::NestedPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NestedProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NestedProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NestedProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_enabled: Option<bool> = None;
                let mut seen_enabled = false;
                let mut f_include_in_parent: Option<bool> = None;
                let mut seen_include_in_parent = false;
                let mut f_include_in_root: Option<bool> = None;
                let mut seen_include_in_root = false;
                let mut f_type: Option<crate::types::common_mapping::NestedPropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "enabled" => {
                            if seen_enabled {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            seen_enabled = true;
                            f_enabled = map.next_value::<Option<bool>>()?;
                        }
                        "include_in_parent" => {
                            if seen_include_in_parent {
                                return Err(serde::de::Error::duplicate_field("include_in_parent"));
                            }
                            seen_include_in_parent = true;
                            f_include_in_parent = map.next_value::<Option<bool>>()?;
                        }
                        "include_in_root" => {
                            if seen_include_in_root {
                                return Err(serde::de::Error::duplicate_field("include_in_root"));
                            }
                            seen_include_in_root = true;
                            f_include_in_root = map.next_value::<Option<bool>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(
                                map.next_value::<crate::types::common_mapping::NestedPropertyType>(
                                )?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NestedProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    enabled: f_enabled,
                    include_in_parent: f_include_in_parent,
                    include_in_root: f_include_in_root,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NestedProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.enabled {
            map.serialize_entry("enabled", v)?;
        }
        if let Some(v) = &self.include_in_parent {
            map.serialize_entry("include_in_parent", v)?;
        }
        if let Some(v) = &self.include_in_root {
            map.serialize_entry("include_in_root", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NestedPropertyType {
    #[doc = "`nested`"]
    Nested,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl NestedPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            NestedPropertyType::Nested => "nested",
            NestedPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for NestedPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NestedPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "nested" => NestedPropertyType::Nested,
            _ => NestedPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ObjectProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub enabled: Option<bool>,
    pub r#type: Option<crate::types::common_mapping::ObjectPropertyType>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ObjectProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ObjectProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ObjectProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_enabled: Option<bool> = None;
                let mut seen_enabled = false;
                let mut f_type: Option<crate::types::common_mapping::ObjectPropertyType> = None;
                let mut seen_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "enabled" => {
                            if seen_enabled {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            seen_enabled = true;
                            f_enabled = map.next_value::<Option<bool>>()?;
                        }
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<crate::types::common_mapping::ObjectPropertyType>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ObjectProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    enabled: f_enabled,
                    r#type: f_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ObjectProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.enabled {
            map.serialize_entry("enabled", v)?;
        }
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
pub enum ObjectPropertyType {
    #[doc = "`object`"]
    Object,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ObjectPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ObjectPropertyType::Object => "object",
            ObjectPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ObjectPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ObjectPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "object" => ObjectPropertyType::Object,
            _ => ObjectPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct PercolatorProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub r#type: crate::types::common_mapping::PercolatorPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PercolatorProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PercolatorProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PercolatorProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_type: Option<crate::types::common_mapping::PercolatorPropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::PercolatorPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PercolatorProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PercolatorProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PercolatorPropertyType {
    #[doc = "`percolator`"]
    Percolator,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PercolatorPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PercolatorPropertyType::Percolator => "percolator",
            PercolatorPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PercolatorPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PercolatorPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "percolator" => PercolatorPropertyType::Percolator,
            _ => PercolatorPropertyType::Unknown(s),
        })
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Property {
    raw: Box<serde_json::value::RawValue>,
}
impl Property {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `binary_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_binary_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::BinaryProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "binary_property")
    }
    #[doc = "Parses the value as the `boolean_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_boolean_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::BooleanProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "boolean_property")
    }
    #[doc = "Parses the value as the `join_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_join_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::JoinProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "join_property")
    }
    #[doc = "Parses the value as the `keyword_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_keyword_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::KeywordProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "keyword_property")
    }
    #[doc = "Parses the value as the `match_only_text_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_match_only_text_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::MatchOnlyTextProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "match_only_text_property")
    }
    #[doc = "Parses the value as the `percolator_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_percolator_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::PercolatorProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "percolator_property")
    }
    #[doc = "Parses the value as the `rank_feature_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_rank_feature_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::RankFeatureProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "rank_feature_property")
    }
    #[doc = "Parses the value as the `rank_features_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_rank_features_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::RankFeaturesProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "rank_features_property")
    }
    #[doc = "Parses the value as the `search_as_you_type_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_search_as_you_type_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::SearchAsYouTypeProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "search_as_you_type_property")
    }
    #[doc = "Parses the value as the `text_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_text_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::TextProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "text_property")
    }
    #[doc = "Parses the value as the `version_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_version_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::VersionProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "version_property")
    }
    #[doc = "Parses the value as the `wildcard_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_wildcard_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::WildcardProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "wildcard_property")
    }
    #[doc = "Parses the value as the `date_nanos_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_date_nanos_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::DateNanosProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "date_nanos_property")
    }
    #[doc = "Parses the value as the `date_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_date_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::DateProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "date_property")
    }
    #[doc = "Parses the value as the `aggregate_metric_double_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_aggregate_metric_double_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::AggregateMetricDoubleProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "aggregate_metric_double_property")
    }
    #[doc = "Parses the value as the `flat_object_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_flat_object_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::FlatObjectProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "flat_object_property")
    }
    #[doc = "Parses the value as the `nested_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_nested_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::NestedProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "nested_property")
    }
    #[doc = "Parses the value as the `object_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_object_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::ObjectProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "object_property")
    }
    #[doc = "Parses the value as the `completion_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_completion_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::CompletionProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "completion_property")
    }
    #[doc = "Parses the value as the `constant_keyword_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_constant_keyword_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::ConstantKeywordProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "constant_keyword_property")
    }
    #[doc = "Parses the value as the `field_alias_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_field_alias_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::FieldAliasProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "field_alias_property")
    }
    #[doc = "Parses the value as the `histogram_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_histogram_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::HistogramProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "histogram_property")
    }
    #[doc = "Parses the value as the `ip_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_ip_property(
        &self,
    ) -> std::result::Result<crate::types::common_mapping::IpProperty, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "ip_property")
    }
    #[doc = "Parses the value as the `murmur_3_hash_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_murmur_3_hash_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::Murmur3HashProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "murmur_3_hash_property")
    }
    #[doc = "Parses the value as the `token_count_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_token_count_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::TokenCountProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "token_count_property")
    }
    #[doc = "Parses the value as the `geo_point_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_geo_point_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::GeoPointProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "geo_point_property")
    }
    #[doc = "Parses the value as the `geo_shape_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_geo_shape_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::GeoShapeProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "geo_shape_property")
    }
    #[doc = "Parses the value as the `xy_point_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_xy_point_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::XyPointProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "xy_point_property")
    }
    #[doc = "Parses the value as the `xy_shape_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_xy_shape_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::XyShapeProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "xy_shape_property")
    }
    #[doc = "Parses the value as the `byte_number_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_byte_number_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::ByteNumberProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "byte_number_property")
    }
    #[doc = "Parses the value as the `double_number_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_double_number_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::DoubleNumberProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "double_number_property")
    }
    #[doc = "Parses the value as the `float_number_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_float_number_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::FloatNumberProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "float_number_property")
    }
    #[doc = "Parses the value as the `half_float_number_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_half_float_number_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::HalfFloatNumberProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "half_float_number_property")
    }
    #[doc = "Parses the value as the `integer_number_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_integer_number_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::IntegerNumberProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "integer_number_property")
    }
    #[doc = "Parses the value as the `long_number_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_long_number_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::LongNumberProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "long_number_property")
    }
    #[doc = "Parses the value as the `scaled_float_number_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_scaled_float_number_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::ScaledFloatNumberProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "scaled_float_number_property")
    }
    #[doc = "Parses the value as the `semantic_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_semantic_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::SemanticProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "semantic_property")
    }
    #[doc = "Parses the value as the `short_number_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_short_number_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::ShortNumberProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "short_number_property")
    }
    #[doc = "Parses the value as the `unsigned_long_number_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_unsigned_long_number_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::UnsignedLongNumberProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "unsigned_long_number_property")
    }
    #[doc = "Parses the value as the `date_range_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_date_range_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::DateRangeProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "date_range_property")
    }
    #[doc = "Parses the value as the `double_range_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_double_range_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::DoubleRangeProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "double_range_property")
    }
    #[doc = "Parses the value as the `float_range_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_float_range_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::FloatRangeProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "float_range_property")
    }
    #[doc = "Parses the value as the `integer_range_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_integer_range_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::IntegerRangeProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "integer_range_property")
    }
    #[doc = "Parses the value as the `ip_range_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_ip_range_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::IpRangeProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "ip_range_property")
    }
    #[doc = "Parses the value as the `long_range_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_long_range_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::LongRangeProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "long_range_property")
    }
    #[doc = "Parses the value as the `knn_vector_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_knn_vector_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::KnnVectorProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "knn_vector_property")
    }
    #[doc = "Parses the value as the `icu_collation_keyword_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_icu_collation_keyword_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::IcuCollationKeywordProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "icu_collation_keyword_property")
    }
}

#[derive(Debug, Clone)]
pub struct RankFeatureProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub positive_score_impact: Option<bool>,
    pub r#type: crate::types::common_mapping::RankFeaturePropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RankFeatureProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RankFeatureProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RankFeatureProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_positive_score_impact: Option<bool> = None;
                let mut seen_positive_score_impact = false;
                let mut f_type: Option<crate::types::common_mapping::RankFeaturePropertyType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "positive_score_impact" => {
                            if seen_positive_score_impact {
                                return Err(serde::de::Error::duplicate_field(
                                    "positive_score_impact",
                                ));
                            }
                            seen_positive_score_impact = true;
                            f_positive_score_impact = map.next_value::<Option<bool>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::RankFeaturePropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RankFeatureProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    positive_score_impact: f_positive_score_impact,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RankFeatureProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.positive_score_impact {
            map.serialize_entry("positive_score_impact", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RankFeaturePropertyType {
    #[doc = "`rank_feature`"]
    RankFeature,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl RankFeaturePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            RankFeaturePropertyType::RankFeature => "rank_feature",
            RankFeaturePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for RankFeaturePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RankFeaturePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "rank_feature" => RankFeaturePropertyType::RankFeature,
            _ => RankFeaturePropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct RankFeaturesProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub r#type: crate::types::common_mapping::RankFeaturesPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RankFeaturesProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RankFeaturesProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RankFeaturesProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_type: Option<crate::types::common_mapping::RankFeaturesPropertyType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::RankFeaturesPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RankFeaturesProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RankFeaturesProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RankFeaturesPropertyType {
    #[doc = "`rank_features`"]
    RankFeatures,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl RankFeaturesPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            RankFeaturesPropertyType::RankFeatures => "rank_features",
            RankFeaturesPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for RankFeaturesPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RankFeaturesPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "rank_features" => RankFeaturesPropertyType::RankFeatures,
            _ => RankFeaturesPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct RoutingField {
    pub required: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RoutingField {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RoutingField;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RoutingField object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_required: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "required" => {
                            if f_required.is_some() {
                                return Err(serde::de::Error::duplicate_field("required"));
                            }
                            f_required = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RoutingField {
                    required: f_required
                        .ok_or_else(|| serde::de::Error::missing_field("required"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RoutingField {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("required", &self.required)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ScaledFloatNumberProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub boost: Option<f64>,
    pub coerce: Option<bool>,
    pub ignore_malformed: Option<bool>,
    pub index: Option<bool>,
    pub null_value: Option<f64>,
    pub scaling_factor: Option<f64>,
    pub r#type: crate::types::common_mapping::ScaledFloatNumberPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ScaledFloatNumberProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ScaledFloatNumberProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ScaledFloatNumberProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_coerce: Option<bool> = None;
                let mut seen_coerce = false;
                let mut f_ignore_malformed: Option<bool> = None;
                let mut seen_ignore_malformed = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_null_value: Option<f64> = None;
                let mut seen_null_value = false;
                let mut f_scaling_factor: Option<f64> = None;
                let mut seen_scaling_factor = false;
                let mut f_type: Option<
                    crate::types::common_mapping::ScaledFloatNumberPropertyType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "coerce" => {
                            if seen_coerce {
                                return Err(serde::de::Error::duplicate_field("coerce"));
                            }
                            seen_coerce = true;
                            f_coerce = map.next_value::<Option<bool>>()?;
                        }
                        "ignore_malformed" => {
                            if seen_ignore_malformed {
                                return Err(serde::de::Error::duplicate_field("ignore_malformed"));
                            }
                            seen_ignore_malformed = true;
                            f_ignore_malformed = map.next_value::<Option<bool>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "null_value" => {
                            if seen_null_value {
                                return Err(serde::de::Error::duplicate_field("null_value"));
                            }
                            seen_null_value = true;
                            f_null_value = map.next_value::<Option<f64>>()?;
                        }
                        "scaling_factor" => {
                            if seen_scaling_factor {
                                return Err(serde::de::Error::duplicate_field("scaling_factor"));
                            }
                            seen_scaling_factor = true;
                            f_scaling_factor = map.next_value::<Option<f64>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::ScaledFloatNumberPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ScaledFloatNumberProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    boost: f_boost,
                    coerce: f_coerce,
                    ignore_malformed: f_ignore_malformed,
                    index: f_index,
                    null_value: f_null_value,
                    scaling_factor: f_scaling_factor,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ScaledFloatNumberProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.coerce {
            map.serialize_entry("coerce", v)?;
        }
        if let Some(v) = &self.ignore_malformed {
            map.serialize_entry("ignore_malformed", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.null_value {
            map.serialize_entry("null_value", v)?;
        }
        if let Some(v) = &self.scaling_factor {
            map.serialize_entry("scaling_factor", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScaledFloatNumberPropertyType {
    #[doc = "`scaled_float`"]
    ScaledFloat,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ScaledFloatNumberPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ScaledFloatNumberPropertyType::ScaledFloat => "scaled_float",
            ScaledFloatNumberPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ScaledFloatNumberPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScaledFloatNumberPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "scaled_float" => ScaledFloatNumberPropertyType::ScaledFloat,
            _ => ScaledFloatNumberPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SearchAsYouTypeProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub analyzer: Option<String>,
    pub index: Option<bool>,
    pub index_options: Option<crate::types::common_mapping::IndexOptions>,
    pub max_shingle_size: Option<i64>,
    pub norms: Option<bool>,
    pub search_analyzer: Option<String>,
    pub search_quote_analyzer: Option<String>,
    pub term_vector: Option<crate::types::common_mapping::TermVectorOption>,
    pub r#type: crate::types::common_mapping::SearchAsYouTypePropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SearchAsYouTypeProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SearchAsYouTypeProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchAsYouTypeProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_analyzer: Option<String> = None;
                let mut seen_analyzer = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_index_options: Option<crate::types::common_mapping::IndexOptions> = None;
                let mut seen_index_options = false;
                let mut f_max_shingle_size: Option<i64> = None;
                let mut seen_max_shingle_size = false;
                let mut f_norms: Option<bool> = None;
                let mut seen_norms = false;
                let mut f_search_analyzer: Option<String> = None;
                let mut seen_search_analyzer = false;
                let mut f_search_quote_analyzer: Option<String> = None;
                let mut seen_search_quote_analyzer = false;
                let mut f_term_vector: Option<crate::types::common_mapping::TermVectorOption> =
                    None;
                let mut seen_term_vector = false;
                let mut f_type: Option<crate::types::common_mapping::SearchAsYouTypePropertyType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "analyzer" => {
                            if seen_analyzer {
                                return Err(serde::de::Error::duplicate_field("analyzer"));
                            }
                            seen_analyzer = true;
                            f_analyzer = map.next_value::<Option<String>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "index_options" => {
                            if seen_index_options {
                                return Err(serde::de::Error::duplicate_field("index_options"));
                            }
                            seen_index_options = true;
                            f_index_options = map
                                .next_value::<Option<crate::types::common_mapping::IndexOptions>>(
                                )?;
                        }
                        "max_shingle_size" => {
                            if seen_max_shingle_size {
                                return Err(serde::de::Error::duplicate_field("max_shingle_size"));
                            }
                            seen_max_shingle_size = true;
                            f_max_shingle_size = map.next_value::<Option<i64>>()?;
                        }
                        "norms" => {
                            if seen_norms {
                                return Err(serde::de::Error::duplicate_field("norms"));
                            }
                            seen_norms = true;
                            f_norms = map.next_value::<Option<bool>>()?;
                        }
                        "search_analyzer" => {
                            if seen_search_analyzer {
                                return Err(serde::de::Error::duplicate_field("search_analyzer"));
                            }
                            seen_search_analyzer = true;
                            f_search_analyzer = map.next_value::<Option<String>>()?;
                        }
                        "search_quote_analyzer" => {
                            if seen_search_quote_analyzer {
                                return Err(serde::de::Error::duplicate_field(
                                    "search_quote_analyzer",
                                ));
                            }
                            seen_search_quote_analyzer = true;
                            f_search_quote_analyzer = map.next_value::<Option<String>>()?;
                        }
                        "term_vector" => {
                            if seen_term_vector {
                                return Err(serde::de::Error::duplicate_field("term_vector"));
                            }
                            seen_term_vector = true;
                            f_term_vector = map.next_value::<Option<crate::types::common_mapping::TermVectorOption>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::SearchAsYouTypePropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SearchAsYouTypeProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    analyzer: f_analyzer,
                    index: f_index,
                    index_options: f_index_options,
                    max_shingle_size: f_max_shingle_size,
                    norms: f_norms,
                    search_analyzer: f_search_analyzer,
                    search_quote_analyzer: f_search_quote_analyzer,
                    term_vector: f_term_vector,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SearchAsYouTypeProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.analyzer {
            map.serialize_entry("analyzer", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.index_options {
            map.serialize_entry("index_options", v)?;
        }
        if let Some(v) = &self.max_shingle_size {
            map.serialize_entry("max_shingle_size", v)?;
        }
        if let Some(v) = &self.norms {
            map.serialize_entry("norms", v)?;
        }
        if let Some(v) = &self.search_analyzer {
            map.serialize_entry("search_analyzer", v)?;
        }
        if let Some(v) = &self.search_quote_analyzer {
            map.serialize_entry("search_quote_analyzer", v)?;
        }
        if let Some(v) = &self.term_vector {
            map.serialize_entry("term_vector", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchAsYouTypePropertyType {
    #[doc = "`search_as_you_type`"]
    SearchAsYouType,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SearchAsYouTypePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SearchAsYouTypePropertyType::SearchAsYouType => "search_as_you_type",
            SearchAsYouTypePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SearchAsYouTypePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SearchAsYouTypePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "search_as_you_type" => SearchAsYouTypePropertyType::SearchAsYouType,
            _ => SearchAsYouTypePropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SemanticChunkingStrategy {
    pub algorithm: String,
    pub parameters: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SemanticChunkingStrategy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SemanticChunkingStrategy;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SemanticChunkingStrategy object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_algorithm: Option<String> = None;
                let mut f_parameters: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_parameters = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "algorithm" => {
                            if f_algorithm.is_some() {
                                return Err(serde::de::Error::duplicate_field("algorithm"));
                            }
                            f_algorithm = Some(map.next_value::<String>()?);
                        }
                        "parameters" => {
                            if seen_parameters {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            seen_parameters = true;
                            f_parameters = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SemanticChunkingStrategy {
                    algorithm: f_algorithm
                        .ok_or_else(|| serde::de::Error::missing_field("algorithm"))?,
                    parameters: f_parameters,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SemanticChunkingStrategy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("algorithm", &self.algorithm)?;
        if let Some(v) = &self.parameters {
            map.serialize_entry("parameters", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SemanticDenseEmbeddingConfig {
    pub compression_level: Option<String>,
    pub data_type: Option<String>,
    pub method: Option<crate::types::common_mapping::KnnVectorMethod>,
    pub mode: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SemanticDenseEmbeddingConfig {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SemanticDenseEmbeddingConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SemanticDenseEmbeddingConfig object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_compression_level: Option<String> = None;
                let mut seen_compression_level = false;
                let mut f_data_type: Option<String> = None;
                let mut seen_data_type = false;
                let mut f_method: Option<crate::types::common_mapping::KnnVectorMethod> = None;
                let mut seen_method = false;
                let mut f_mode: Option<String> = None;
                let mut seen_mode = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "compression_level" => {
                            if seen_compression_level {
                                return Err(serde::de::Error::duplicate_field("compression_level"));
                            }
                            seen_compression_level = true;
                            f_compression_level = map.next_value::<Option<String>>()?;
                        }
                        "data_type" => {
                            if seen_data_type {
                                return Err(serde::de::Error::duplicate_field("data_type"));
                            }
                            seen_data_type = true;
                            f_data_type = map.next_value::<Option<String>>()?;
                        }
                        "method" => {
                            if seen_method {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            seen_method = true;
                            f_method = map.next_value::<Option<crate::types::common_mapping::KnnVectorMethod>>()?;
                        }
                        "mode" => {
                            if seen_mode {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            seen_mode = true;
                            f_mode = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SemanticDenseEmbeddingConfig {
                    compression_level: f_compression_level,
                    data_type: f_data_type,
                    method: f_method,
                    mode: f_mode,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SemanticDenseEmbeddingConfig {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.compression_level {
            map.serialize_entry("compression_level", v)?;
        }
        if let Some(v) = &self.data_type {
            map.serialize_entry("data_type", v)?;
        }
        if let Some(v) = &self.method {
            map.serialize_entry("method", v)?;
        }
        if let Some(v) = &self.mode {
            map.serialize_entry("mode", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SemanticProperty {
    pub chunking: Option<crate::types::common_mapping::SemanticPropertyChunking>,
    pub dense_embedding_config: Option<crate::types::common_mapping::SemanticDenseEmbeddingConfig>,
    pub model_id: String,
    pub raw_field_type: Option<String>,
    pub search_model_id: Option<String>,
    pub semantic_field_search_analyzer: Option<String>,
    pub semantic_info_field_name: Option<String>,
    pub skip_existing_embedding: Option<bool>,
    pub sparse_encoding_config: Option<crate::types::common_mapping::SemanticSparseEncodingConfig>,
    pub r#type: crate::types::common_mapping::SemanticPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SemanticProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SemanticProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SemanticProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_chunking: Option<crate::types::common_mapping::SemanticPropertyChunking> =
                    None;
                let mut seen_chunking = false;
                let mut f_dense_embedding_config: Option<
                    crate::types::common_mapping::SemanticDenseEmbeddingConfig,
                > = None;
                let mut seen_dense_embedding_config = false;
                let mut f_model_id: Option<String> = None;
                let mut f_raw_field_type: Option<String> = None;
                let mut seen_raw_field_type = false;
                let mut f_search_model_id: Option<String> = None;
                let mut seen_search_model_id = false;
                let mut f_semantic_field_search_analyzer: Option<String> = None;
                let mut seen_semantic_field_search_analyzer = false;
                let mut f_semantic_info_field_name: Option<String> = None;
                let mut seen_semantic_info_field_name = false;
                let mut f_skip_existing_embedding: Option<bool> = None;
                let mut seen_skip_existing_embedding = false;
                let mut f_sparse_encoding_config: Option<
                    crate::types::common_mapping::SemanticSparseEncodingConfig,
                > = None;
                let mut seen_sparse_encoding_config = false;
                let mut f_type: Option<crate::types::common_mapping::SemanticPropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "chunking" => {
                            if seen_chunking {
                                return Err(serde::de::Error::duplicate_field("chunking"));
                            }
                            seen_chunking = true;
                            f_chunking = map.next_value::<Option<crate::types::common_mapping::SemanticPropertyChunking>>()?;
                        }
                        "dense_embedding_config" => {
                            if seen_dense_embedding_config {
                                return Err(serde::de::Error::duplicate_field(
                                    "dense_embedding_config",
                                ));
                            }
                            seen_dense_embedding_config = true;
                            f_dense_embedding_config = map.next_value::<Option<
                                crate::types::common_mapping::SemanticDenseEmbeddingConfig,
                            >>()?;
                        }
                        "model_id" => {
                            if f_model_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("model_id"));
                            }
                            f_model_id = Some(map.next_value::<String>()?);
                        }
                        "raw_field_type" => {
                            if seen_raw_field_type {
                                return Err(serde::de::Error::duplicate_field("raw_field_type"));
                            }
                            seen_raw_field_type = true;
                            f_raw_field_type = map.next_value::<Option<String>>()?;
                        }
                        "search_model_id" => {
                            if seen_search_model_id {
                                return Err(serde::de::Error::duplicate_field("search_model_id"));
                            }
                            seen_search_model_id = true;
                            f_search_model_id = map.next_value::<Option<String>>()?;
                        }
                        "semantic_field_search_analyzer" => {
                            if seen_semantic_field_search_analyzer {
                                return Err(serde::de::Error::duplicate_field(
                                    "semantic_field_search_analyzer",
                                ));
                            }
                            seen_semantic_field_search_analyzer = true;
                            f_semantic_field_search_analyzer =
                                map.next_value::<Option<String>>()?;
                        }
                        "semantic_info_field_name" => {
                            if seen_semantic_info_field_name {
                                return Err(serde::de::Error::duplicate_field(
                                    "semantic_info_field_name",
                                ));
                            }
                            seen_semantic_info_field_name = true;
                            f_semantic_info_field_name = map.next_value::<Option<String>>()?;
                        }
                        "skip_existing_embedding" => {
                            if seen_skip_existing_embedding {
                                return Err(serde::de::Error::duplicate_field(
                                    "skip_existing_embedding",
                                ));
                            }
                            seen_skip_existing_embedding = true;
                            f_skip_existing_embedding = map.next_value::<Option<bool>>()?;
                        }
                        "sparse_encoding_config" => {
                            if seen_sparse_encoding_config {
                                return Err(serde::de::Error::duplicate_field(
                                    "sparse_encoding_config",
                                ));
                            }
                            seen_sparse_encoding_config = true;
                            f_sparse_encoding_config = map.next_value::<Option<
                                crate::types::common_mapping::SemanticSparseEncodingConfig,
                            >>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::SemanticPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SemanticProperty {
                    chunking: f_chunking,
                    dense_embedding_config: f_dense_embedding_config,
                    model_id: f_model_id
                        .ok_or_else(|| serde::de::Error::missing_field("model_id"))?,
                    raw_field_type: f_raw_field_type,
                    search_model_id: f_search_model_id,
                    semantic_field_search_analyzer: f_semantic_field_search_analyzer,
                    semantic_info_field_name: f_semantic_info_field_name,
                    skip_existing_embedding: f_skip_existing_embedding,
                    sparse_encoding_config: f_sparse_encoding_config,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SemanticProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.chunking {
            map.serialize_entry("chunking", v)?;
        }
        if let Some(v) = &self.dense_embedding_config {
            map.serialize_entry("dense_embedding_config", v)?;
        }
        map.serialize_entry("model_id", &self.model_id)?;
        if let Some(v) = &self.raw_field_type {
            map.serialize_entry("raw_field_type", v)?;
        }
        if let Some(v) = &self.search_model_id {
            map.serialize_entry("search_model_id", v)?;
        }
        if let Some(v) = &self.semantic_field_search_analyzer {
            map.serialize_entry("semantic_field_search_analyzer", v)?;
        }
        if let Some(v) = &self.semantic_info_field_name {
            map.serialize_entry("semantic_info_field_name", v)?;
        }
        if let Some(v) = &self.skip_existing_embedding {
            map.serialize_entry("skip_existing_embedding", v)?;
        }
        if let Some(v) = &self.sparse_encoding_config {
            map.serialize_entry("sparse_encoding_config", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SemanticPropertyChunking {
    Boolean(bool),
    Array(Vec<crate::types::common_mapping::SemanticChunkingStrategy>),
}
impl<'de> serde::Deserialize<'de> for SemanticPropertyChunking {
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
            Some(b't') | Some(b'f') => {
                serde_json::from_str(text).map(SemanticPropertyChunking::Boolean)
            }
            Some(b'[') => serde_json::from_str(text).map(SemanticPropertyChunking::Array),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a SemanticPropertyChunking value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SemanticPropertyType {
    #[doc = "`semantic`"]
    Semantic,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SemanticPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SemanticPropertyType::Semantic => "semantic",
            SemanticPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SemanticPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SemanticPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "semantic" => SemanticPropertyType::Semantic,
            _ => SemanticPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SemanticSparseEncodingConfig {
    pub prune_ratio: Option<f64>,
    pub prune_type: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SemanticSparseEncodingConfig {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SemanticSparseEncodingConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SemanticSparseEncodingConfig object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_prune_ratio: Option<f64> = None;
                let mut seen_prune_ratio = false;
                let mut f_prune_type: Option<String> = None;
                let mut seen_prune_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "prune_ratio" => {
                            if seen_prune_ratio {
                                return Err(serde::de::Error::duplicate_field("prune_ratio"));
                            }
                            seen_prune_ratio = true;
                            f_prune_ratio = map.next_value::<Option<f64>>()?;
                        }
                        "prune_type" => {
                            if seen_prune_type {
                                return Err(serde::de::Error::duplicate_field("prune_type"));
                            }
                            seen_prune_type = true;
                            f_prune_type = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SemanticSparseEncodingConfig {
                    prune_ratio: f_prune_ratio,
                    prune_type: f_prune_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SemanticSparseEncodingConfig {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.prune_ratio {
            map.serialize_entry("prune_ratio", v)?;
        }
        if let Some(v) = &self.prune_type {
            map.serialize_entry("prune_type", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShortNumberProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub boost: Option<f64>,
    pub coerce: Option<bool>,
    pub ignore_malformed: Option<bool>,
    pub index: Option<bool>,
    pub null_value: Option<crate::types::common::Short>,
    pub r#type: crate::types::common_mapping::ShortNumberPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShortNumberProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShortNumberProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShortNumberProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_coerce: Option<bool> = None;
                let mut seen_coerce = false;
                let mut f_ignore_malformed: Option<bool> = None;
                let mut seen_ignore_malformed = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_null_value: Option<crate::types::common::Short> = None;
                let mut seen_null_value = false;
                let mut f_type: Option<crate::types::common_mapping::ShortNumberPropertyType> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "coerce" => {
                            if seen_coerce {
                                return Err(serde::de::Error::duplicate_field("coerce"));
                            }
                            seen_coerce = true;
                            f_coerce = map.next_value::<Option<bool>>()?;
                        }
                        "ignore_malformed" => {
                            if seen_ignore_malformed {
                                return Err(serde::de::Error::duplicate_field("ignore_malformed"));
                            }
                            seen_ignore_malformed = true;
                            f_ignore_malformed = map.next_value::<Option<bool>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "null_value" => {
                            if seen_null_value {
                                return Err(serde::de::Error::duplicate_field("null_value"));
                            }
                            seen_null_value = true;
                            f_null_value =
                                map.next_value::<Option<crate::types::common::Short>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::ShortNumberPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShortNumberProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    boost: f_boost,
                    coerce: f_coerce,
                    ignore_malformed: f_ignore_malformed,
                    index: f_index,
                    null_value: f_null_value,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShortNumberProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.coerce {
            map.serialize_entry("coerce", v)?;
        }
        if let Some(v) = &self.ignore_malformed {
            map.serialize_entry("ignore_malformed", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.null_value {
            map.serialize_entry("null_value", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShortNumberPropertyType {
    #[doc = "`short`"]
    Short,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ShortNumberPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ShortNumberPropertyType::Short => "short",
            ShortNumberPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ShortNumberPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShortNumberPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "short" => ShortNumberPropertyType::Short,
            _ => ShortNumberPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SizeField {
    pub enabled: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SizeField {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SizeField;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SizeField object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_enabled: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "enabled" => {
                            if f_enabled.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            f_enabled = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SizeField {
                    enabled: f_enabled.ok_or_else(|| serde::de::Error::missing_field("enabled"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SizeField {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("enabled", &self.enabled)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SourceField {
    pub compress: Option<bool>,
    pub compress_threshold: Option<String>,
    pub enabled: Option<bool>,
    pub excludes: Option<Vec<String>>,
    pub includes: Option<Vec<String>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SourceField {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SourceField;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SourceField object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_compress: Option<bool> = None;
                let mut seen_compress = false;
                let mut f_compress_threshold: Option<String> = None;
                let mut seen_compress_threshold = false;
                let mut f_enabled: Option<bool> = None;
                let mut seen_enabled = false;
                let mut f_excludes: Option<Vec<String>> = None;
                let mut seen_excludes = false;
                let mut f_includes: Option<Vec<String>> = None;
                let mut seen_includes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "compress" => {
                            if seen_compress {
                                return Err(serde::de::Error::duplicate_field("compress"));
                            }
                            seen_compress = true;
                            f_compress = map.next_value::<Option<bool>>()?;
                        }
                        "compress_threshold" => {
                            if seen_compress_threshold {
                                return Err(serde::de::Error::duplicate_field(
                                    "compress_threshold",
                                ));
                            }
                            seen_compress_threshold = true;
                            f_compress_threshold = map.next_value::<Option<String>>()?;
                        }
                        "enabled" => {
                            if seen_enabled {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            seen_enabled = true;
                            f_enabled = map.next_value::<Option<bool>>()?;
                        }
                        "excludes" => {
                            if seen_excludes {
                                return Err(serde::de::Error::duplicate_field("excludes"));
                            }
                            seen_excludes = true;
                            f_excludes = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "includes" => {
                            if seen_includes {
                                return Err(serde::de::Error::duplicate_field("includes"));
                            }
                            seen_includes = true;
                            f_includes = map.next_value::<Option<Vec<String>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SourceField {
                    compress: f_compress,
                    compress_threshold: f_compress_threshold,
                    enabled: f_enabled,
                    excludes: f_excludes,
                    includes: f_includes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SourceField {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.compress {
            map.serialize_entry("compress", v)?;
        }
        if let Some(v) = &self.compress_threshold {
            map.serialize_entry("compress_threshold", v)?;
        }
        if let Some(v) = &self.enabled {
            map.serialize_entry("enabled", v)?;
        }
        if let Some(v) = &self.excludes {
            map.serialize_entry("excludes", v)?;
        }
        if let Some(v) = &self.includes {
            map.serialize_entry("includes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SuggestContext {
    pub name: crate::types::common::Name,
    pub path: Option<crate::types::common::Field>,
    pub precision: Option<crate::types::common_mapping::SuggestContextPrecision>,
    pub r#type: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SuggestContext {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SuggestContext;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SuggestContext object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut f_path: Option<crate::types::common::Field> = None;
                let mut seen_path = false;
                let mut f_precision: Option<crate::types::common_mapping::SuggestContextPrecision> =
                    None;
                let mut seen_precision = false;
                let mut f_type: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        "path" => {
                            if seen_path {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            seen_path = true;
                            f_path = map.next_value::<Option<crate::types::common::Field>>()?;
                        }
                        "precision" => {
                            if seen_precision {
                                return Err(serde::de::Error::duplicate_field("precision"));
                            }
                            seen_precision = true;
                            f_precision = map.next_value::<Option<crate::types::common_mapping::SuggestContextPrecision>>()?;
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
                Ok(SuggestContext {
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    path: f_path,
                    precision: f_precision,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SuggestContext {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("name", &self.name)?;
        if let Some(v) = &self.path {
            map.serialize_entry("path", v)?;
        }
        if let Some(v) = &self.precision {
            map.serialize_entry("precision", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SuggestContextPrecision {
    Number(f64),
    String(String),
}
impl<'de> serde::Deserialize<'de> for SuggestContextPrecision {
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
                serde_json::from_str(text).map(SuggestContextPrecision::Number)
            }
            Some(b'"') => serde_json::from_str(text).map(SuggestContextPrecision::String),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a SuggestContextPrecision value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TermVectorOption {
    #[doc = "`no`"]
    No,
    #[doc = "`with_offsets`"]
    WithOffsets,
    #[doc = "`with_positions`"]
    WithPositions,
    #[doc = "`with_positions_offsets`"]
    WithPositionsOffsets,
    #[doc = "`with_positions_offsets_payloads`"]
    WithPositionsOffsetsPayloads,
    #[doc = "`with_positions_payloads`"]
    WithPositionsPayloads,
    #[doc = "`yes`"]
    Yes,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TermVectorOption {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TermVectorOption::No => "no",
            TermVectorOption::WithOffsets => "with_offsets",
            TermVectorOption::WithPositions => "with_positions",
            TermVectorOption::WithPositionsOffsets => "with_positions_offsets",
            TermVectorOption::WithPositionsOffsetsPayloads => "with_positions_offsets_payloads",
            TermVectorOption::WithPositionsPayloads => "with_positions_payloads",
            TermVectorOption::Yes => "yes",
            TermVectorOption::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TermVectorOption {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TermVectorOption {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "no" => TermVectorOption::No,
            "with_offsets" => TermVectorOption::WithOffsets,
            "with_positions" => TermVectorOption::WithPositions,
            "with_positions_offsets" => TermVectorOption::WithPositionsOffsets,
            "with_positions_offsets_payloads" => TermVectorOption::WithPositionsOffsetsPayloads,
            "with_positions_payloads" => TermVectorOption::WithPositionsPayloads,
            "yes" => TermVectorOption::Yes,
            _ => TermVectorOption::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct TextIndexPrefixes {
    pub max_chars: i64,
    pub min_chars: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TextIndexPrefixes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TextIndexPrefixes;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TextIndexPrefixes object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_max_chars: Option<i64> = None;
                let mut f_min_chars: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "max_chars" => {
                            if f_max_chars.is_some() {
                                return Err(serde::de::Error::duplicate_field("max_chars"));
                            }
                            f_max_chars = Some(map.next_value::<i64>()?);
                        }
                        "min_chars" => {
                            if f_min_chars.is_some() {
                                return Err(serde::de::Error::duplicate_field("min_chars"));
                            }
                            f_min_chars = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TextIndexPrefixes {
                    max_chars: f_max_chars
                        .ok_or_else(|| serde::de::Error::missing_field("max_chars"))?,
                    min_chars: f_min_chars
                        .ok_or_else(|| serde::de::Error::missing_field("min_chars"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TextIndexPrefixes {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("max_chars", &self.max_chars)?;
        map.serialize_entry("min_chars", &self.min_chars)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TextProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub analyzer: Option<String>,
    pub boost: Option<f64>,
    pub eager_global_ordinals: Option<bool>,
    pub fielddata: Option<bool>,
    pub fielddata_frequency_filter: Option<crate::types::indices_common::FielddataFrequencyFilter>,
    pub index: Option<bool>,
    pub index_options: Option<crate::types::common_mapping::IndexOptions>,
    pub index_phrases: Option<bool>,
    pub index_prefixes: Option<crate::types::common_mapping::TextIndexPrefixes>,
    pub norms: Option<bool>,
    pub position_increment_gap: Option<i64>,
    pub search_analyzer: Option<String>,
    pub search_quote_analyzer: Option<String>,
    pub term_vector: Option<crate::types::common_mapping::TermVectorOption>,
    pub r#type: crate::types::common_mapping::TextPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TextProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TextProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TextProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_analyzer: Option<String> = None;
                let mut seen_analyzer = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_eager_global_ordinals: Option<bool> = None;
                let mut seen_eager_global_ordinals = false;
                let mut f_fielddata: Option<bool> = None;
                let mut seen_fielddata = false;
                let mut f_fielddata_frequency_filter: Option<
                    crate::types::indices_common::FielddataFrequencyFilter,
                > = None;
                let mut seen_fielddata_frequency_filter = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_index_options: Option<crate::types::common_mapping::IndexOptions> = None;
                let mut seen_index_options = false;
                let mut f_index_phrases: Option<bool> = None;
                let mut seen_index_phrases = false;
                let mut f_index_prefixes: Option<crate::types::common_mapping::TextIndexPrefixes> =
                    None;
                let mut seen_index_prefixes = false;
                let mut f_norms: Option<bool> = None;
                let mut seen_norms = false;
                let mut f_position_increment_gap: Option<i64> = None;
                let mut seen_position_increment_gap = false;
                let mut f_search_analyzer: Option<String> = None;
                let mut seen_search_analyzer = false;
                let mut f_search_quote_analyzer: Option<String> = None;
                let mut seen_search_quote_analyzer = false;
                let mut f_term_vector: Option<crate::types::common_mapping::TermVectorOption> =
                    None;
                let mut seen_term_vector = false;
                let mut f_type: Option<crate::types::common_mapping::TextPropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "analyzer" => {
                            if seen_analyzer {
                                return Err(serde::de::Error::duplicate_field("analyzer"));
                            }
                            seen_analyzer = true;
                            f_analyzer = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "eager_global_ordinals" => {
                            if seen_eager_global_ordinals {
                                return Err(serde::de::Error::duplicate_field(
                                    "eager_global_ordinals",
                                ));
                            }
                            seen_eager_global_ordinals = true;
                            f_eager_global_ordinals = map.next_value::<Option<bool>>()?;
                        }
                        "fielddata" => {
                            if seen_fielddata {
                                return Err(serde::de::Error::duplicate_field("fielddata"));
                            }
                            seen_fielddata = true;
                            f_fielddata = map.next_value::<Option<bool>>()?;
                        }
                        "fielddata_frequency_filter" => {
                            if seen_fielddata_frequency_filter {
                                return Err(serde::de::Error::duplicate_field(
                                    "fielddata_frequency_filter",
                                ));
                            }
                            seen_fielddata_frequency_filter = true;
                            f_fielddata_frequency_filter = map.next_value::<Option<crate::types::indices_common::FielddataFrequencyFilter>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "index_options" => {
                            if seen_index_options {
                                return Err(serde::de::Error::duplicate_field("index_options"));
                            }
                            seen_index_options = true;
                            f_index_options = map
                                .next_value::<Option<crate::types::common_mapping::IndexOptions>>(
                                )?;
                        }
                        "index_phrases" => {
                            if seen_index_phrases {
                                return Err(serde::de::Error::duplicate_field("index_phrases"));
                            }
                            seen_index_phrases = true;
                            f_index_phrases = map.next_value::<Option<bool>>()?;
                        }
                        "index_prefixes" => {
                            if seen_index_prefixes {
                                return Err(serde::de::Error::duplicate_field("index_prefixes"));
                            }
                            seen_index_prefixes = true;
                            f_index_prefixes = map.next_value::<Option<crate::types::common_mapping::TextIndexPrefixes>>()?;
                        }
                        "norms" => {
                            if seen_norms {
                                return Err(serde::de::Error::duplicate_field("norms"));
                            }
                            seen_norms = true;
                            f_norms = map.next_value::<Option<bool>>()?;
                        }
                        "position_increment_gap" => {
                            if seen_position_increment_gap {
                                return Err(serde::de::Error::duplicate_field(
                                    "position_increment_gap",
                                ));
                            }
                            seen_position_increment_gap = true;
                            f_position_increment_gap = map.next_value::<Option<i64>>()?;
                        }
                        "search_analyzer" => {
                            if seen_search_analyzer {
                                return Err(serde::de::Error::duplicate_field("search_analyzer"));
                            }
                            seen_search_analyzer = true;
                            f_search_analyzer = map.next_value::<Option<String>>()?;
                        }
                        "search_quote_analyzer" => {
                            if seen_search_quote_analyzer {
                                return Err(serde::de::Error::duplicate_field(
                                    "search_quote_analyzer",
                                ));
                            }
                            seen_search_quote_analyzer = true;
                            f_search_quote_analyzer = map.next_value::<Option<String>>()?;
                        }
                        "term_vector" => {
                            if seen_term_vector {
                                return Err(serde::de::Error::duplicate_field("term_vector"));
                            }
                            seen_term_vector = true;
                            f_term_vector = map.next_value::<Option<crate::types::common_mapping::TermVectorOption>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(
                                map.next_value::<crate::types::common_mapping::TextPropertyType>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TextProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    analyzer: f_analyzer,
                    boost: f_boost,
                    eager_global_ordinals: f_eager_global_ordinals,
                    fielddata: f_fielddata,
                    fielddata_frequency_filter: f_fielddata_frequency_filter,
                    index: f_index,
                    index_options: f_index_options,
                    index_phrases: f_index_phrases,
                    index_prefixes: f_index_prefixes,
                    norms: f_norms,
                    position_increment_gap: f_position_increment_gap,
                    search_analyzer: f_search_analyzer,
                    search_quote_analyzer: f_search_quote_analyzer,
                    term_vector: f_term_vector,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TextProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.analyzer {
            map.serialize_entry("analyzer", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.eager_global_ordinals {
            map.serialize_entry("eager_global_ordinals", v)?;
        }
        if let Some(v) = &self.fielddata {
            map.serialize_entry("fielddata", v)?;
        }
        if let Some(v) = &self.fielddata_frequency_filter {
            map.serialize_entry("fielddata_frequency_filter", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.index_options {
            map.serialize_entry("index_options", v)?;
        }
        if let Some(v) = &self.index_phrases {
            map.serialize_entry("index_phrases", v)?;
        }
        if let Some(v) = &self.index_prefixes {
            map.serialize_entry("index_prefixes", v)?;
        }
        if let Some(v) = &self.norms {
            map.serialize_entry("norms", v)?;
        }
        if let Some(v) = &self.position_increment_gap {
            map.serialize_entry("position_increment_gap", v)?;
        }
        if let Some(v) = &self.search_analyzer {
            map.serialize_entry("search_analyzer", v)?;
        }
        if let Some(v) = &self.search_quote_analyzer {
            map.serialize_entry("search_quote_analyzer", v)?;
        }
        if let Some(v) = &self.term_vector {
            map.serialize_entry("term_vector", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextPropertyType {
    #[doc = "`text`"]
    Text,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TextPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TextPropertyType::Text => "text",
            TextPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TextPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TextPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "text" => TextPropertyType::Text,
            _ => TextPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct TokenCountProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub analyzer: Option<String>,
    pub boost: Option<f64>,
    pub enable_position_increments: Option<bool>,
    pub index: Option<bool>,
    pub null_value: Option<f64>,
    pub r#type: crate::types::common_mapping::TokenCountPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TokenCountProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TokenCountProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TokenCountProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_analyzer: Option<String> = None;
                let mut seen_analyzer = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_enable_position_increments: Option<bool> = None;
                let mut seen_enable_position_increments = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_null_value: Option<f64> = None;
                let mut seen_null_value = false;
                let mut f_type: Option<crate::types::common_mapping::TokenCountPropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "analyzer" => {
                            if seen_analyzer {
                                return Err(serde::de::Error::duplicate_field("analyzer"));
                            }
                            seen_analyzer = true;
                            f_analyzer = map.next_value::<Option<String>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "enable_position_increments" => {
                            if seen_enable_position_increments {
                                return Err(serde::de::Error::duplicate_field(
                                    "enable_position_increments",
                                ));
                            }
                            seen_enable_position_increments = true;
                            f_enable_position_increments = map.next_value::<Option<bool>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "null_value" => {
                            if seen_null_value {
                                return Err(serde::de::Error::duplicate_field("null_value"));
                            }
                            seen_null_value = true;
                            f_null_value = map.next_value::<Option<f64>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::TokenCountPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TokenCountProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    analyzer: f_analyzer,
                    boost: f_boost,
                    enable_position_increments: f_enable_position_increments,
                    index: f_index,
                    null_value: f_null_value,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TokenCountProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.analyzer {
            map.serialize_entry("analyzer", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.enable_position_increments {
            map.serialize_entry("enable_position_increments", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.null_value {
            map.serialize_entry("null_value", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenCountPropertyType {
    #[doc = "`token_count`"]
    TokenCount,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TokenCountPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TokenCountPropertyType::TokenCount => "token_count",
            TokenCountPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TokenCountPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TokenCountPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "token_count" => TokenCountPropertyType::TokenCount,
            _ => TokenCountPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct TypeMapping {
    pub data_stream_timestamp: Option<crate::types::common_mapping::DataStreamTimestamp>,
    pub field_names: Option<crate::types::common_mapping::FieldNamesField>,
    pub meta: Option<crate::types::common::Metadata>,
    pub routing: Option<crate::types::common_mapping::RoutingField>,
    pub size: Option<crate::types::common_mapping::SizeField>,
    pub source: Option<crate::types::common_mapping::SourceField>,
    pub all_field: Option<crate::types::common_mapping::AllField>,
    pub date_detection: Option<bool>,
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub dynamic_date_formats: Option<Vec<String>>,
    pub dynamic_templates: Option<
        Vec<std::collections::BTreeMap<String, crate::types::common_mapping::DynamicTemplate>>,
    >,
    pub enabled: Option<bool>,
    pub index_field: Option<crate::types::common_mapping::IndexField>,
    pub numeric_detection: Option<bool>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TypeMapping {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TypeMapping;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TypeMapping object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_data_stream_timestamp: Option<
                    crate::types::common_mapping::DataStreamTimestamp,
                > = None;
                let mut seen_data_stream_timestamp = false;
                let mut f_field_names: Option<crate::types::common_mapping::FieldNamesField> = None;
                let mut seen_field_names = false;
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_routing: Option<crate::types::common_mapping::RoutingField> = None;
                let mut seen_routing = false;
                let mut f_size: Option<crate::types::common_mapping::SizeField> = None;
                let mut seen_size = false;
                let mut f_source: Option<crate::types::common_mapping::SourceField> = None;
                let mut seen_source = false;
                let mut f_all_field: Option<crate::types::common_mapping::AllField> = None;
                let mut seen_all_field = false;
                let mut f_date_detection: Option<bool> = None;
                let mut seen_date_detection = false;
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_dynamic_date_formats: Option<Vec<String>> = None;
                let mut seen_dynamic_date_formats = false;
                let mut f_dynamic_templates: Option<
                    Vec<
                        std::collections::BTreeMap<
                            String,
                            crate::types::common_mapping::DynamicTemplate,
                        >,
                    >,
                > = None;
                let mut seen_dynamic_templates = false;
                let mut f_enabled: Option<bool> = None;
                let mut seen_enabled = false;
                let mut f_index_field: Option<crate::types::common_mapping::IndexField> = None;
                let mut seen_index_field = false;
                let mut f_numeric_detection: Option<bool> = None;
                let mut seen_numeric_detection = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_data_stream_timestamp" => {
                            if seen_data_stream_timestamp {
                                return Err(serde::de::Error::duplicate_field(
                                    "_data_stream_timestamp",
                                ));
                            }
                            seen_data_stream_timestamp = true;
                            f_data_stream_timestamp = map.next_value::<Option<crate::types::common_mapping::DataStreamTimestamp>>()?;
                        }
                        "_field_names" => {
                            if seen_field_names {
                                return Err(serde::de::Error::duplicate_field("_field_names"));
                            }
                            seen_field_names = true;
                            f_field_names = map.next_value::<Option<crate::types::common_mapping::FieldNamesField>>()?;
                        }
                        "_meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("_meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "_routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("_routing"));
                            }
                            seen_routing = true;
                            f_routing = map
                                .next_value::<Option<crate::types::common_mapping::RoutingField>>(
                                )?;
                        }
                        "_size" => {
                            if seen_size {
                                return Err(serde::de::Error::duplicate_field("_size"));
                            }
                            seen_size = true;
                            f_size = map
                                .next_value::<Option<crate::types::common_mapping::SizeField>>()?;
                        }
                        "_source" => {
                            if seen_source {
                                return Err(serde::de::Error::duplicate_field("_source"));
                            }
                            seen_source = true;
                            f_source = map
                                .next_value::<Option<crate::types::common_mapping::SourceField>>(
                                )?;
                        }
                        "all_field" => {
                            if seen_all_field {
                                return Err(serde::de::Error::duplicate_field("all_field"));
                            }
                            seen_all_field = true;
                            f_all_field =
                                map.next_value::<Option<crate::types::common_mapping::AllField>>()?;
                        }
                        "date_detection" => {
                            if seen_date_detection {
                                return Err(serde::de::Error::duplicate_field("date_detection"));
                            }
                            seen_date_detection = true;
                            f_date_detection = map.next_value::<Option<bool>>()?;
                        }
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "dynamic_date_formats" => {
                            if seen_dynamic_date_formats {
                                return Err(serde::de::Error::duplicate_field(
                                    "dynamic_date_formats",
                                ));
                            }
                            seen_dynamic_date_formats = true;
                            f_dynamic_date_formats = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "dynamic_templates" => {
                            if seen_dynamic_templates {
                                return Err(serde::de::Error::duplicate_field("dynamic_templates"));
                            }
                            seen_dynamic_templates = true;
                            f_dynamic_templates = map.next_value::<Option<
                                Vec<
                                    std::collections::BTreeMap<
                                        String,
                                        crate::types::common_mapping::DynamicTemplate,
                                    >,
                                >,
                            >>()?;
                        }
                        "enabled" => {
                            if seen_enabled {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            seen_enabled = true;
                            f_enabled = map.next_value::<Option<bool>>()?;
                        }
                        "index_field" => {
                            if seen_index_field {
                                return Err(serde::de::Error::duplicate_field("index_field"));
                            }
                            seen_index_field = true;
                            f_index_field = map
                                .next_value::<Option<crate::types::common_mapping::IndexField>>()?;
                        }
                        "numeric_detection" => {
                            if seen_numeric_detection {
                                return Err(serde::de::Error::duplicate_field("numeric_detection"));
                            }
                            seen_numeric_detection = true;
                            f_numeric_detection = map.next_value::<Option<bool>>()?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TypeMapping {
                    data_stream_timestamp: f_data_stream_timestamp,
                    field_names: f_field_names,
                    meta: f_meta,
                    routing: f_routing,
                    size: f_size,
                    source: f_source,
                    all_field: f_all_field,
                    date_detection: f_date_detection,
                    dynamic: f_dynamic,
                    dynamic_date_formats: f_dynamic_date_formats,
                    dynamic_templates: f_dynamic_templates,
                    enabled: f_enabled,
                    index_field: f_index_field,
                    numeric_detection: f_numeric_detection,
                    properties: f_properties,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TypeMapping {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.data_stream_timestamp {
            map.serialize_entry("_data_stream_timestamp", v)?;
        }
        if let Some(v) = &self.field_names {
            map.serialize_entry("_field_names", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("_meta", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("_routing", v)?;
        }
        if let Some(v) = &self.size {
            map.serialize_entry("_size", v)?;
        }
        if let Some(v) = &self.source {
            map.serialize_entry("_source", v)?;
        }
        if let Some(v) = &self.all_field {
            map.serialize_entry("all_field", v)?;
        }
        if let Some(v) = &self.date_detection {
            map.serialize_entry("date_detection", v)?;
        }
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.dynamic_date_formats {
            map.serialize_entry("dynamic_date_formats", v)?;
        }
        if let Some(v) = &self.dynamic_templates {
            map.serialize_entry("dynamic_templates", v)?;
        }
        if let Some(v) = &self.enabled {
            map.serialize_entry("enabled", v)?;
        }
        if let Some(v) = &self.index_field {
            map.serialize_entry("index_field", v)?;
        }
        if let Some(v) = &self.numeric_detection {
            map.serialize_entry("numeric_detection", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct UnsignedLongNumberProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub boost: Option<f64>,
    pub coerce: Option<bool>,
    pub ignore_malformed: Option<bool>,
    pub index: Option<bool>,
    pub null_value: Option<crate::types::common::Ulong>,
    pub r#type: crate::types::common_mapping::UnsignedLongNumberPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UnsignedLongNumberProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UnsignedLongNumberProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UnsignedLongNumberProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_boost: Option<f64> = None;
                let mut seen_boost = false;
                let mut f_coerce: Option<bool> = None;
                let mut seen_coerce = false;
                let mut f_ignore_malformed: Option<bool> = None;
                let mut seen_ignore_malformed = false;
                let mut f_index: Option<bool> = None;
                let mut seen_index = false;
                let mut f_null_value: Option<crate::types::common::Ulong> = None;
                let mut seen_null_value = false;
                let mut f_type: Option<
                    crate::types::common_mapping::UnsignedLongNumberPropertyType,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "boost" => {
                            if seen_boost {
                                return Err(serde::de::Error::duplicate_field("boost"));
                            }
                            seen_boost = true;
                            f_boost = map.next_value::<Option<f64>>()?;
                        }
                        "coerce" => {
                            if seen_coerce {
                                return Err(serde::de::Error::duplicate_field("coerce"));
                            }
                            seen_coerce = true;
                            f_coerce = map.next_value::<Option<bool>>()?;
                        }
                        "ignore_malformed" => {
                            if seen_ignore_malformed {
                                return Err(serde::de::Error::duplicate_field("ignore_malformed"));
                            }
                            seen_ignore_malformed = true;
                            f_ignore_malformed = map.next_value::<Option<bool>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<bool>>()?;
                        }
                        "null_value" => {
                            if seen_null_value {
                                return Err(serde::de::Error::duplicate_field("null_value"));
                            }
                            seen_null_value = true;
                            f_null_value =
                                map.next_value::<Option<crate::types::common::Ulong>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::UnsignedLongNumberPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UnsignedLongNumberProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    boost: f_boost,
                    coerce: f_coerce,
                    ignore_malformed: f_ignore_malformed,
                    index: f_index,
                    null_value: f_null_value,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UnsignedLongNumberProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.boost {
            map.serialize_entry("boost", v)?;
        }
        if let Some(v) = &self.coerce {
            map.serialize_entry("coerce", v)?;
        }
        if let Some(v) = &self.ignore_malformed {
            map.serialize_entry("ignore_malformed", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.null_value {
            map.serialize_entry("null_value", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnsignedLongNumberPropertyType {
    #[doc = "`unsigned_long`"]
    UnsignedLong,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl UnsignedLongNumberPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            UnsignedLongNumberPropertyType::UnsignedLong => "unsigned_long",
            UnsignedLongNumberPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for UnsignedLongNumberPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UnsignedLongNumberPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "unsigned_long" => UnsignedLongNumberPropertyType::UnsignedLong,
            _ => UnsignedLongNumberPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct VersionProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub r#type: crate::types::common_mapping::VersionPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for VersionProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = VersionProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a VersionProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_type: Option<crate::types::common_mapping::VersionPropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::VersionPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(VersionProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for VersionProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VersionPropertyType {
    #[doc = "`version`"]
    Version,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl VersionPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            VersionPropertyType::Version => "version",
            VersionPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for VersionPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VersionPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "version" => VersionPropertyType::Version,
            _ => VersionPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct WildcardProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub normalizer: Option<String>,
    pub null_value: Option<String>,
    pub r#type: crate::types::common_mapping::WildcardPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for WildcardProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = WildcardProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a WildcardProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_normalizer: Option<String> = None;
                let mut seen_normalizer = false;
                let mut f_null_value: Option<String> = None;
                let mut seen_null_value = false;
                let mut f_type: Option<crate::types::common_mapping::WildcardPropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "normalizer" => {
                            if seen_normalizer {
                                return Err(serde::de::Error::duplicate_field("normalizer"));
                            }
                            seen_normalizer = true;
                            f_normalizer = map.next_value::<Option<String>>()?;
                        }
                        "null_value" => {
                            if seen_null_value {
                                return Err(serde::de::Error::duplicate_field("null_value"));
                            }
                            seen_null_value = true;
                            f_null_value = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::WildcardPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(WildcardProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    normalizer: f_normalizer,
                    null_value: f_null_value,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for WildcardProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.normalizer {
            map.serialize_entry("normalizer", v)?;
        }
        if let Some(v) = &self.null_value {
            map.serialize_entry("null_value", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WildcardPropertyType {
    #[doc = "`wildcard`"]
    Wildcard,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl WildcardPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            WildcardPropertyType::Wildcard => "wildcard",
            WildcardPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for WildcardPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for WildcardPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "wildcard" => WildcardPropertyType::Wildcard,
            _ => WildcardPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct XyPointProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub ignore_malformed: Option<bool>,
    pub ignore_z_value: Option<bool>,
    pub null_value: Option<crate::types::common::XyLocation>,
    pub r#type: crate::types::common_mapping::XyPointPropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for XyPointProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = XyPointProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a XyPointProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_ignore_malformed: Option<bool> = None;
                let mut seen_ignore_malformed = false;
                let mut f_ignore_z_value: Option<bool> = None;
                let mut seen_ignore_z_value = false;
                let mut f_null_value: Option<crate::types::common::XyLocation> = None;
                let mut seen_null_value = false;
                let mut f_type: Option<crate::types::common_mapping::XyPointPropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "ignore_malformed" => {
                            if seen_ignore_malformed {
                                return Err(serde::de::Error::duplicate_field("ignore_malformed"));
                            }
                            seen_ignore_malformed = true;
                            f_ignore_malformed = map.next_value::<Option<bool>>()?;
                        }
                        "ignore_z_value" => {
                            if seen_ignore_z_value {
                                return Err(serde::de::Error::duplicate_field("ignore_z_value"));
                            }
                            seen_ignore_z_value = true;
                            f_ignore_z_value = map.next_value::<Option<bool>>()?;
                        }
                        "null_value" => {
                            if seen_null_value {
                                return Err(serde::de::Error::duplicate_field("null_value"));
                            }
                            seen_null_value = true;
                            f_null_value =
                                map.next_value::<Option<crate::types::common::XyLocation>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::XyPointPropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(XyPointProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    ignore_malformed: f_ignore_malformed,
                    ignore_z_value: f_ignore_z_value,
                    null_value: f_null_value,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for XyPointProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.ignore_malformed {
            map.serialize_entry("ignore_malformed", v)?;
        }
        if let Some(v) = &self.ignore_z_value {
            map.serialize_entry("ignore_z_value", v)?;
        }
        if let Some(v) = &self.null_value {
            map.serialize_entry("null_value", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum XyPointPropertyType {
    #[doc = "`xy_point`"]
    XyPoint,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl XyPointPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            XyPointPropertyType::XyPoint => "xy_point",
            XyPointPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for XyPointPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for XyPointPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "xy_point" => XyPointPropertyType::XyPoint,
            _ => XyPointPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct XyShapeProperty {
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub copy_to: Option<crate::types::common::Fields>,
    pub similarity: Option<String>,
    pub store: Option<bool>,
    pub doc_values: Option<bool>,
    pub coerce: Option<bool>,
    pub ignore_malformed: Option<bool>,
    pub ignore_z_value: Option<bool>,
    pub orientation: Option<crate::types::common_mapping::GeoOrientation>,
    pub r#type: crate::types::common_mapping::XyShapePropertyType,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for XyShapeProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = XyShapeProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a XyShapeProperty object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::common_mapping::DynamicMapping> = None;
                let mut seen_dynamic = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_fields = false;
                let mut f_ignore_above: Option<i64> = None;
                let mut seen_ignore_above = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_meta = false;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
                > = None;
                let mut seen_properties = false;
                let mut f_copy_to: Option<crate::types::common::Fields> = None;
                let mut seen_copy_to = false;
                let mut f_similarity: Option<String> = None;
                let mut seen_similarity = false;
                let mut f_store: Option<bool> = None;
                let mut seen_store = false;
                let mut f_doc_values: Option<bool> = None;
                let mut seen_doc_values = false;
                let mut f_coerce: Option<bool> = None;
                let mut seen_coerce = false;
                let mut f_ignore_malformed: Option<bool> = None;
                let mut seen_ignore_malformed = false;
                let mut f_ignore_z_value: Option<bool> = None;
                let mut seen_ignore_z_value = false;
                let mut f_orientation: Option<crate::types::common_mapping::GeoOrientation> = None;
                let mut seen_orientation = false;
                let mut f_type: Option<crate::types::common_mapping::XyShapePropertyType> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map
                                .next_value::<Option<crate::types::common_mapping::DynamicMapping>>(
                                )?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "ignore_above" => {
                            if seen_ignore_above {
                                return Err(serde::de::Error::duplicate_field("ignore_above"));
                            }
                            seen_ignore_above = true;
                            f_ignore_above = map.next_value::<Option<i64>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "properties" => {
                            if seen_properties {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            seen_properties = true;
                            f_properties = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common_mapping::Property,
                                >,
                            >>()?;
                        }
                        "copy_to" => {
                            if seen_copy_to {
                                return Err(serde::de::Error::duplicate_field("copy_to"));
                            }
                            seen_copy_to = true;
                            f_copy_to = map.next_value::<Option<crate::types::common::Fields>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<bool>>()?;
                        }
                        "doc_values" => {
                            if seen_doc_values {
                                return Err(serde::de::Error::duplicate_field("doc_values"));
                            }
                            seen_doc_values = true;
                            f_doc_values = map.next_value::<Option<bool>>()?;
                        }
                        "coerce" => {
                            if seen_coerce {
                                return Err(serde::de::Error::duplicate_field("coerce"));
                            }
                            seen_coerce = true;
                            f_coerce = map.next_value::<Option<bool>>()?;
                        }
                        "ignore_malformed" => {
                            if seen_ignore_malformed {
                                return Err(serde::de::Error::duplicate_field("ignore_malformed"));
                            }
                            seen_ignore_malformed = true;
                            f_ignore_malformed = map.next_value::<Option<bool>>()?;
                        }
                        "ignore_z_value" => {
                            if seen_ignore_z_value {
                                return Err(serde::de::Error::duplicate_field("ignore_z_value"));
                            }
                            seen_ignore_z_value = true;
                            f_ignore_z_value = map.next_value::<Option<bool>>()?;
                        }
                        "orientation" => {
                            if seen_orientation {
                                return Err(serde::de::Error::duplicate_field("orientation"));
                            }
                            seen_orientation = true;
                            f_orientation = map
                                .next_value::<Option<crate::types::common_mapping::GeoOrientation>>(
                                )?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::common_mapping::XyShapePropertyType>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(XyShapeProperty {
                    dynamic: f_dynamic,
                    fields: f_fields,
                    ignore_above: f_ignore_above,
                    meta: f_meta,
                    properties: f_properties,
                    copy_to: f_copy_to,
                    similarity: f_similarity,
                    store: f_store,
                    doc_values: f_doc_values,
                    coerce: f_coerce,
                    ignore_malformed: f_ignore_malformed,
                    ignore_z_value: f_ignore_z_value,
                    orientation: f_orientation,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for XyShapeProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.ignore_above {
            map.serialize_entry("ignore_above", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.properties {
            map.serialize_entry("properties", v)?;
        }
        if let Some(v) = &self.copy_to {
            map.serialize_entry("copy_to", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.doc_values {
            map.serialize_entry("doc_values", v)?;
        }
        if let Some(v) = &self.coerce {
            map.serialize_entry("coerce", v)?;
        }
        if let Some(v) = &self.ignore_malformed {
            map.serialize_entry("ignore_malformed", v)?;
        }
        if let Some(v) = &self.ignore_z_value {
            map.serialize_entry("ignore_z_value", v)?;
        }
        if let Some(v) = &self.orientation {
            map.serialize_entry("orientation", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum XyShapePropertyType {
    #[doc = "`xy_shape`"]
    XyShape,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl XyShapePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            XyShapePropertyType::XyShape => "xy_shape",
            XyShapePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for XyShapePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for XyShapePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "xy_shape" => XyShapePropertyType::XyShape,
            _ => XyShapePropertyType::Unknown(s),
        })
    }
}
