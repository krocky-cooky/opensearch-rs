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

//! Response types of the `indices_validate_query` specification namespace

#![allow(missing_docs)]

#[doc = "The validation results for an index query."]
#[derive(Debug, Clone)]
pub struct IndicesValidationExplanation {
    #[doc = "The error message if validation failed."]
    pub error: Option<String>,
    #[doc = "The explanation of why the query is valid or invalid."]
    pub explanation: Option<String>,
    pub index: crate::types::common::IndexName,
    #[doc = "Whether the query is valid for the index."]
    pub valid: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndicesValidationExplanation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndicesValidationExplanation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndicesValidationExplanation object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_error: Option<String> = None;
                let mut seen_error = false;
                let mut f_explanation: Option<String> = None;
                let mut seen_explanation = false;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut f_valid: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "error" => {
                            if seen_error {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            seen_error = true;
                            f_error = map.next_value::<Option<String>>()?;
                        }
                        "explanation" => {
                            if seen_explanation {
                                return Err(serde::de::Error::duplicate_field("explanation"));
                            }
                            seen_explanation = true;
                            f_explanation = map.next_value::<Option<String>>()?;
                        }
                        "index" => {
                            if f_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            f_index = Some(map.next_value::<crate::types::common::IndexName>()?);
                        }
                        "valid" => {
                            if f_valid.is_some() {
                                return Err(serde::de::Error::duplicate_field("valid"));
                            }
                            f_valid = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndicesValidationExplanation {
                    error: f_error,
                    explanation: f_explanation,
                    index: f_index.ok_or_else(|| serde::de::Error::missing_field("index"))?,
                    valid: f_valid.ok_or_else(|| serde::de::Error::missing_field("valid"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndicesValidationExplanation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.error {
            map.serialize_entry("error", v)?;
        }
        if let Some(v) = &self.explanation {
            map.serialize_entry("explanation", v)?;
        }
        map.serialize_entry("index", &self.index)?;
        map.serialize_entry("valid", &self.valid)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `indices.validate_query` API"]
pub type ValidateQueryResponse = crate::types::indices::ValidateQueryResponse;
