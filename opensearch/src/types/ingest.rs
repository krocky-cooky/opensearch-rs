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

//! Response types of the `ingest` specification namespace

#![allow(missing_docs)]

pub type GetPipelineResponse =
    std::collections::BTreeMap<String, crate::types::ingest_common::Pipeline>;

#[derive(Debug, Clone)]
pub struct ProcessorGrokResponse {
    pub patterns: std::collections::BTreeMap<String, String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ProcessorGrokResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ProcessorGrokResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ProcessorGrokResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_patterns: Option<std::collections::BTreeMap<String, String>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "patterns" => {
                            if f_patterns.is_some() {
                                return Err(serde::de::Error::duplicate_field("patterns"));
                            }
                            f_patterns = Some(
                                map.next_value::<std::collections::BTreeMap<String, String>>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ProcessorGrokResponse {
                    patterns: f_patterns
                        .ok_or_else(|| serde::de::Error::missing_field("patterns"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ProcessorGrokResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("patterns", &self.patterns)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SimulateResponse {
    pub docs: Vec<crate::types::ingest_simulate::PipelineSimulation>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SimulateResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SimulateResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SimulateResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_docs: Option<Vec<crate::types::ingest_simulate::PipelineSimulation>> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "docs" => {
                            if f_docs.is_some() {
                                return Err(serde::de::Error::duplicate_field("docs"));
                            }
                            f_docs = Some(map.next_value::<Vec<crate::types::ingest_simulate::PipelineSimulation>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SimulateResponse {
                    docs: f_docs.ok_or_else(|| serde::de::Error::missing_field("docs"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SimulateResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("docs", &self.docs)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
