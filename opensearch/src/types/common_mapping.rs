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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AggregateMetricDoubleProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub default_metric: String,
    pub metrics: Vec<String>,
    pub r#type: crate::types::common_mapping::AggregateMetricDoublePropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AggregateMetricDoublePropertyType {
    #[serde(rename = "aggregate_metric_double")]
    AggregateMetricDouble,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BinaryProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    pub r#type: crate::types::common_mapping::BinaryPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BinaryPropertyType {
    #[serde(rename = "binary")]
    Binary,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BooleanProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fielddata: Option<crate::types::indices_common::NumericFielddata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<bool>,
    pub r#type: crate::types::common_mapping::BooleanPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BooleanPropertyType {
    #[serde(rename = "boolean")]
    Boolean,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ByteNumberProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<crate::types::common::Byte>,
    pub r#type: crate::types::common_mapping::ByteNumberPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ByteNumberPropertyType {
    #[serde(rename = "byte")]
    Byte,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompletionProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<crate::types::common_mapping::SuggestContext>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_input_length: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_position_increments: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_separators: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_analyzer: Option<String>,
    pub r#type: crate::types::common_mapping::CompletionPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CompletionPropertyType {
    #[serde(rename = "completion")]
    Completion,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConstantKeywordProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub r#type: crate::types::common_mapping::ConstantKeywordPropertyType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<serde_json::value::RawValue>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ConstantKeywordPropertyType {
    #[serde(rename = "constant_keyword")]
    ConstantKeyword,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataStreamTimestamp {
    pub enabled: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DateNanosProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<crate::types::common::DateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precision_step: Option<i32>,
    pub r#type: crate::types::common_mapping::DateNanosPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DateNanosPropertyType {
    #[serde(rename = "date_nanos")]
    DateNanos,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DateProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fielddata: Option<crate::types::indices_common::NumericFielddata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<crate::types::common::DateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precision_step: Option<i32>,
    pub r#type: crate::types::common_mapping::DatePropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DatePropertyType {
    #[serde(rename = "date")]
    Date,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DateRangeProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    pub r#type: crate::types::common_mapping::DateRangePropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DateRangePropertyType {
    #[serde(rename = "date_range")]
    DateRange,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DoubleNumberProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<f64>,
    pub r#type: crate::types::common_mapping::DoubleNumberPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DoubleNumberPropertyType {
    #[serde(rename = "double")]
    Double,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DoubleRangeProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    pub r#type: crate::types::common_mapping::DoubleRangePropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DoubleRangePropertyType {
    #[serde(rename = "double_range")]
    DoubleRange,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
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
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = DynamicMapping;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DynamicMapping value")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(DynamicMapping::String)
            }
            fn visit_bool<E>(self, v: bool) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::BoolDeserializer::new(v))
                    .map(DynamicMapping::Boolean)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DynamicMappingString {
    #[serde(rename = "false")]
    False,
    #[serde(rename = "strict")]
    Strict,
    #[serde(rename = "strict_allow_templates")]
    StrictAllowTemplates,
    #[serde(rename = "true")]
    True,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DynamicTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mapping: Option<crate::types::common_mapping::Property>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#match: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_mapping_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_pattern: Option<crate::types::common_mapping::MatchType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_match: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_unmatch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unmatch: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FieldAliasProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<crate::types::common::Field>,
    pub r#type: crate::types::common_mapping::FieldAliasPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FieldAliasPropertyType {
    #[serde(rename = "alias")]
    Alias,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FieldMapping {
    pub full_name: String,
    pub mapping: std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FieldNamesField {
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FieldType {
    #[serde(rename = "aggregate_metric_double")]
    AggregateMetricDouble,
    #[serde(rename = "alias")]
    Alias,
    #[serde(rename = "binary")]
    Binary,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "byte")]
    Byte,
    #[serde(rename = "completion")]
    Completion,
    #[serde(rename = "constant_keyword")]
    ConstantKeyword,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "date_nanos")]
    DateNanos,
    #[serde(rename = "date_range")]
    DateRange,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "double_range")]
    DoubleRange,
    #[serde(rename = "flat_object")]
    FlatObject,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "float_range")]
    FloatRange,
    #[serde(rename = "geo_point")]
    GeoPoint,
    #[serde(rename = "geo_shape")]
    GeoShape,
    #[serde(rename = "half_float")]
    HalfFloat,
    #[serde(rename = "histogram")]
    Histogram,
    #[serde(rename = "icu_collation_keyword")]
    IcuCollationKeyword,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "integer_range")]
    IntegerRange,
    #[serde(rename = "ip")]
    Ip,
    #[serde(rename = "ip_range")]
    IpRange,
    #[serde(rename = "join")]
    Join,
    #[serde(rename = "keyword")]
    Keyword,
    #[serde(rename = "knn_vector")]
    KnnVector,
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "long_range")]
    LongRange,
    #[serde(rename = "match_only_text")]
    MatchOnlyText,
    #[serde(rename = "murmur3")]
    Murmur3,
    #[serde(rename = "nested")]
    Nested,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "percolator")]
    Percolator,
    #[serde(rename = "rank_feature")]
    RankFeature,
    #[serde(rename = "rank_features")]
    RankFeatures,
    #[serde(rename = "scaled_float")]
    ScaledFloat,
    #[serde(rename = "search_as_you_type")]
    SearchAsYouType,
    #[serde(rename = "semantic")]
    Semantic,
    #[serde(rename = "short")]
    Short,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "token_count")]
    TokenCount,
    #[serde(rename = "unsigned_long")]
    UnsignedLong,
    #[serde(rename = "version")]
    Version,
    #[serde(rename = "wildcard")]
    Wildcard,
    #[serde(rename = "xy_point")]
    XyPoint,
    #[serde(rename = "xy_shape")]
    XyShape,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FlatObjectProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregatable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub searchable: Option<bool>,
    pub r#type: crate::types::common_mapping::FlatObjectPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FlatObjectPropertyType {
    #[serde(rename = "flat_object")]
    FlatObject,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FloatNumberProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<f64>,
    pub r#type: crate::types::common_mapping::FloatNumberPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FloatNumberPropertyType {
    #[serde(rename = "float")]
    Float,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FloatRangeProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    pub r#type: crate::types::common_mapping::FloatRangePropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FloatRangePropertyType {
    #[serde(rename = "float_range")]
    FloatRange,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GeoOrientation {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "clockwise")]
    Clockwise,
    #[serde(rename = "cw")]
    Cw,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "counterclockwise")]
    Counterclockwise,
    #[serde(rename = "ccw")]
    Ccw,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeoPointProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_z_value: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<crate::types::common::GeoLocation>,
    pub r#type: crate::types::common_mapping::GeoPointPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GeoPointPropertyType {
    #[serde(rename = "geo_point")]
    GeoPoint,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeoShapeProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distance_error_pct: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_z_value: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<crate::types::common_mapping::GeoOrientation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<crate::types::common_mapping::GeoStrategy>,
    pub r#type: crate::types::common_mapping::GeoShapePropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GeoShapePropertyType {
    #[serde(rename = "geo_shape")]
    GeoShape,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GeoStrategy {
    #[serde(rename = "recursive")]
    Recursive,
    #[serde(rename = "term")]
    Term,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HalfFloatNumberProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<f64>,
    pub r#type: crate::types::common_mapping::HalfFloatNumberPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum HalfFloatNumberPropertyType {
    #[serde(rename = "half_float")]
    HalfFloat,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HistogramProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    pub r#type: crate::types::common_mapping::HistogramPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum HistogramPropertyType {
    #[serde(rename = "histogram")]
    Histogram,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IcuCollationKeywordProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alternate: Option<crate::types::common_analysis::IcuCollationAlternate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub case_first: Option<crate::types::common_analysis::IcuCollationCaseFirst>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub case_level: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decomposition: Option<crate::types::common_analysis::IcuCollationDecomposition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hiragana_quaternary_mode: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numeric: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strength: Option<crate::types::common_analysis::IcuCollationStrength>,
    pub r#type: crate::types::common_mapping::IcuCollationKeywordPropertyType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable_top: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IcuCollationKeywordPropertyType {
    #[serde(rename = "icu_collation_keyword")]
    IcuCollationKeyword,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexField {
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IndexOptions {
    #[serde(rename = "docs")]
    Docs,
    #[serde(rename = "freqs")]
    Freqs,
    #[serde(rename = "offsets")]
    Offsets,
    #[serde(rename = "positions")]
    Positions,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntegerNumberProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<i32>,
    pub r#type: crate::types::common_mapping::IntegerNumberPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IntegerNumberPropertyType {
    #[serde(rename = "integer")]
    Integer,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntegerRangeProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    pub r#type: crate::types::common_mapping::IntegerRangePropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IntegerRangePropertyType {
    #[serde(rename = "integer_range")]
    IntegerRange,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IpProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<String>,
    pub r#type: crate::types::common_mapping::IpPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IpPropertyType {
    #[serde(rename = "ip")]
    Ip,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IpRangeProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    pub r#type: crate::types::common_mapping::IpRangePropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IpRangePropertyType {
    #[serde(rename = "ip_range")]
    IpRange,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JoinProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eager_global_ordinals: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relations: Option<
        std::collections::BTreeMap<
            String,
            crate::types::common_mapping::JoinPropertyRelationsValue,
        >,
    >,
    pub r#type: crate::types::common_mapping::JoinPropertyType,
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
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = JoinPropertyRelationsValue;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a JoinPropertyRelationsValue value")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(JoinPropertyRelationsValue::RelationName)
            }
            fn visit_seq<A>(self, seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
                    .map(JoinPropertyRelationsValue::Array)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum JoinPropertyType {
    #[serde(rename = "join")]
    Join,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeywordProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eager_global_ordinals: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_options: Option<crate::types::common_mapping::IndexOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalizer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub norms: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split_queries_on_whitespace: Option<bool>,
    pub r#type: crate::types::common_mapping::KeywordPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KeywordPropertyType {
    #[serde(rename = "keyword")]
    Keyword,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KnnVectorMethod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space_type: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KnnVectorProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    pub dimension: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<crate::types::common_mapping::KnnVectorMethod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space_type: Option<String>,
    pub r#type: crate::types::common_mapping::KnnVectorPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KnnVectorPropertyType {
    #[serde(rename = "knn_vector")]
    KnnVector,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LongNumberProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<i64>,
    pub r#type: crate::types::common_mapping::LongNumberPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LongNumberPropertyType {
    #[serde(rename = "long")]
    Long,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LongRangeProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    pub r#type: crate::types::common_mapping::LongRangePropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LongRangePropertyType {
    #[serde(rename = "long_range")]
    LongRange,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MatchOnlyTextProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[doc = "Multi-fields allow the same string value to be indexed in multiple ways for different purposes, such as one field for search and a multi-field for sorting and aggregations, or the same string value analyzed by different analyzers."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub r#type: crate::types::common_mapping::MatchOnlyTextPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MatchOnlyTextPropertyType {
    #[serde(rename = "match_only_text")]
    MatchOnlyText,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MatchType {
    #[serde(rename = "regex")]
    Regex,
    #[serde(rename = "simple")]
    Simple,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Murmur3HashProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    pub r#type: crate::types::common_mapping::Murmur3HashPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Murmur3HashPropertyType {
    #[serde(rename = "murmur3")]
    Murmur3,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_in_parent: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_in_root: Option<bool>,
    pub r#type: crate::types::common_mapping::NestedPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NestedPropertyType {
    #[serde(rename = "nested")]
    Nested,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ObjectProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::types::common_mapping::ObjectPropertyType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ObjectPropertyType {
    #[serde(rename = "object")]
    Object,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PercolatorProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub r#type: crate::types::common_mapping::PercolatorPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PercolatorPropertyType {
    #[serde(rename = "percolator")]
    Percolator,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RankFeatureProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub positive_score_impact: Option<bool>,
    pub r#type: crate::types::common_mapping::RankFeaturePropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RankFeaturePropertyType {
    #[serde(rename = "rank_feature")]
    RankFeature,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RankFeaturesProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub r#type: crate::types::common_mapping::RankFeaturesPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RankFeaturesPropertyType {
    #[serde(rename = "rank_features")]
    RankFeatures,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RoutingField {
    pub required: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScaledFloatNumberProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scaling_factor: Option<f64>,
    pub r#type: crate::types::common_mapping::ScaledFloatNumberPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ScaledFloatNumberPropertyType {
    #[serde(rename = "scaled_float")]
    ScaledFloat,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchAsYouTypeProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_options: Option<crate::types::common_mapping::IndexOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_shingle_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub norms: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_analyzer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_quote_analyzer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term_vector: Option<crate::types::common_mapping::TermVectorOption>,
    pub r#type: crate::types::common_mapping::SearchAsYouTypePropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SearchAsYouTypePropertyType {
    #[serde(rename = "search_as_you_type")]
    SearchAsYouType,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemanticChunkingStrategy {
    pub algorithm: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemanticDenseEmbeddingConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<crate::types::common_mapping::KnnVectorMethod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemanticProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chunking: Option<crate::types::common_mapping::SemanticPropertyChunking>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dense_embedding_config: Option<crate::types::common_mapping::SemanticDenseEmbeddingConfig>,
    pub model_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_field_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_model_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic_field_search_analyzer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic_info_field_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip_existing_embedding: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sparse_encoding_config: Option<crate::types::common_mapping::SemanticSparseEncodingConfig>,
    pub r#type: crate::types::common_mapping::SemanticPropertyType,
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
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = SemanticPropertyChunking;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SemanticPropertyChunking value")
            }
            fn visit_bool<E>(self, v: bool) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::BoolDeserializer::new(v))
                    .map(SemanticPropertyChunking::Boolean)
            }
            fn visit_seq<A>(self, seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
                    .map(SemanticPropertyChunking::Array)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SemanticPropertyType {
    #[serde(rename = "semantic")]
    Semantic,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemanticSparseEncodingConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prune_ratio: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prune_type: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShortNumberProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<crate::types::common::Short>,
    pub r#type: crate::types::common_mapping::ShortNumberPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ShortNumberPropertyType {
    #[serde(rename = "short")]
    Short,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SizeField {
    pub enabled: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SourceField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compress_threshold: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SuggestContext {
    pub name: crate::types::common::Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<crate::types::common::Field>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<crate::types::common_mapping::SuggestContextPrecision>,
    pub r#type: String,
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
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = SuggestContextPrecision;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SuggestContextPrecision value")
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::I64Deserializer::new(v))
                    .map(SuggestContextPrecision::Number)
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::U64Deserializer::new(v))
                    .map(SuggestContextPrecision::Number)
            }
            fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::F64Deserializer::new(v))
                    .map(SuggestContextPrecision::Number)
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(SuggestContextPrecision::String)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TermVectorOption {
    #[serde(rename = "no")]
    No,
    #[serde(rename = "with_offsets")]
    WithOffsets,
    #[serde(rename = "with_positions")]
    WithPositions,
    #[serde(rename = "with_positions_offsets")]
    WithPositionsOffsets,
    #[serde(rename = "with_positions_offsets_payloads")]
    WithPositionsOffsetsPayloads,
    #[serde(rename = "with_positions_payloads")]
    WithPositionsPayloads,
    #[serde(rename = "yes")]
    Yes,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TextIndexPrefixes {
    pub max_chars: i32,
    pub min_chars: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TextProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eager_global_ordinals: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fielddata: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fielddata_frequency_filter: Option<crate::types::indices_common::FielddataFrequencyFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_options: Option<crate::types::common_mapping::IndexOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_phrases: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_prefixes: Option<crate::types::common_mapping::TextIndexPrefixes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub norms: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_increment_gap: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_analyzer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_quote_analyzer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term_vector: Option<crate::types::common_mapping::TermVectorOption>,
    pub r#type: crate::types::common_mapping::TextPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TextPropertyType {
    #[serde(rename = "text")]
    Text,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TokenCountProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_position_increments: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<f64>,
    pub r#type: crate::types::common_mapping::TokenCountPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TokenCountPropertyType {
    #[serde(rename = "token_count")]
    TokenCount,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TypeMapping {
    #[serde(rename = "_data_stream_timestamp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_stream_timestamp: Option<crate::types::common_mapping::DataStreamTimestamp>,
    #[serde(rename = "_field_names")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_names: Option<crate::types::common_mapping::FieldNamesField>,
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    #[serde(rename = "_routing")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing: Option<crate::types::common_mapping::RoutingField>,
    #[serde(rename = "_size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<crate::types::common_mapping::SizeField>,
    #[serde(rename = "_source")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::types::common_mapping::SourceField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_field: Option<crate::types::common_mapping::AllField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_detection: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic_date_formats: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic_templates: Option<
        Vec<std::collections::BTreeMap<String, crate::types::common_mapping::DynamicTemplate>>,
    >,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_field: Option<crate::types::common_mapping::IndexField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numeric_detection: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnsignedLongNumberProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<crate::types::common::Ulong>,
    pub r#type: crate::types::common_mapping::UnsignedLongNumberPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UnsignedLongNumberPropertyType {
    #[serde(rename = "unsigned_long")]
    UnsignedLong,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VersionProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    pub r#type: crate::types::common_mapping::VersionPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum VersionPropertyType {
    #[serde(rename = "version")]
    Version,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WildcardProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalizer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<String>,
    pub r#type: crate::types::common_mapping::WildcardPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum WildcardPropertyType {
    #[serde(rename = "wildcard")]
    Wildcard,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct XyPointProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_z_value: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<crate::types::common::XyLocation>,
    pub r#type: crate::types::common_mapping::XyPointPropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum XyPointPropertyType {
    #[serde(rename = "xy_point")]
    XyPoint,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct XyShapeProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i32>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_z_value: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<crate::types::common_mapping::GeoOrientation>,
    pub r#type: crate::types::common_mapping::XyShapePropertyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum XyShapePropertyType {
    #[serde(rename = "xy_shape")]
    XyShape,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}
