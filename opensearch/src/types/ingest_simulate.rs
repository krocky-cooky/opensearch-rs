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

//! Response types of the `ingest_simulate` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct DocumentSimulation {
    pub id: crate::types::common::Id,
    pub index: crate::types::common::IndexName,
    pub ingest: crate::types::ingest_simulate::Ingest,
    #[doc = "Value used to send the document to a specific primary shard."]
    pub routing: Option<String>,
    #[doc = "JSON body for the document."]
    pub source: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
    pub version: Option<crate::types::common::StringifiedVersionNumber>,
    pub version_type: Option<crate::types::common::VersionType>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DocumentSimulation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DocumentSimulation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DocumentSimulation object")
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
                let mut f_ingest: Option<crate::types::ingest_simulate::Ingest> = None;
                let mut f_routing: Option<String> = None;
                let mut seen_routing = false;
                let mut f_source: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut f_version: Option<crate::types::common::StringifiedVersionNumber> = None;
                let mut seen_version = false;
                let mut f_version_type: Option<crate::types::common::VersionType> = None;
                let mut seen_version_type = false;
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
                        "_ingest" => {
                            if f_ingest.is_some() {
                                return Err(serde::de::Error::duplicate_field("_ingest"));
                            }
                            f_ingest =
                                Some(map.next_value::<crate::types::ingest_simulate::Ingest>()?);
                        }
                        "_routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("_routing"));
                            }
                            seen_routing = true;
                            f_routing = map.next_value::<Option<String>>()?;
                        }
                        "_source" => {
                            if f_source.is_some() {
                                return Err(serde::de::Error::duplicate_field("_source"));
                            }
                            f_source = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                Box<serde_json::value::RawValue>,
                            >>()?);
                        }
                        "_version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            seen_version = true;
                            f_version = map.next_value::<Option<crate::types::common::StringifiedVersionNumber>>()?;
                        }
                        "_version_type" => {
                            if seen_version_type {
                                return Err(serde::de::Error::duplicate_field("_version_type"));
                            }
                            seen_version_type = true;
                            f_version_type =
                                map.next_value::<Option<crate::types::common::VersionType>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DocumentSimulation {
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("_id"))?,
                    index: f_index.ok_or_else(|| serde::de::Error::missing_field("_index"))?,
                    ingest: f_ingest.ok_or_else(|| serde::de::Error::missing_field("_ingest"))?,
                    routing: f_routing,
                    source: f_source.ok_or_else(|| serde::de::Error::missing_field("_source"))?,
                    version: f_version,
                    version_type: f_version_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DocumentSimulation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("_id", &self.id)?;
        map.serialize_entry("_index", &self.index)?;
        map.serialize_entry("_ingest", &self.ingest)?;
        if let Some(v) = &self.routing {
            map.serialize_entry("_routing", v)?;
        }
        map.serialize_entry("_source", &self.source)?;
        if let Some(v) = &self.version {
            map.serialize_entry("_version", v)?;
        }
        if let Some(v) = &self.version_type {
            map.serialize_entry("_version_type", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Ingest {
    pub pipeline: Option<crate::types::common::Name>,
    pub timestamp: crate::types::common::DateTime,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Ingest {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Ingest;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Ingest object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_pipeline: Option<crate::types::common::Name> = None;
                let mut seen_pipeline = false;
                let mut f_timestamp: Option<crate::types::common::DateTime> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "pipeline" => {
                            if seen_pipeline {
                                return Err(serde::de::Error::duplicate_field("pipeline"));
                            }
                            seen_pipeline = true;
                            f_pipeline = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "timestamp" => {
                            if f_timestamp.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            f_timestamp = Some(map.next_value::<crate::types::common::DateTime>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Ingest {
                    pipeline: f_pipeline,
                    timestamp: f_timestamp
                        .ok_or_else(|| serde::de::Error::missing_field("timestamp"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Ingest {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.pipeline {
            map.serialize_entry("pipeline", v)?;
        }
        map.serialize_entry("timestamp", &self.timestamp)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PipelineSimulation {
    pub doc: Option<crate::types::ingest_simulate::DocumentSimulation>,
    pub processor_results: Option<Vec<crate::types::ingest_simulate::PipelineSimulation>>,
    pub processor_type: Option<String>,
    pub status: Option<crate::types::common::ActionStatusOptions>,
    pub tag: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PipelineSimulation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PipelineSimulation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PipelineSimulation object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_doc: Option<crate::types::ingest_simulate::DocumentSimulation> = None;
                let mut seen_doc = false;
                let mut f_processor_results: Option<
                    Vec<crate::types::ingest_simulate::PipelineSimulation>,
                > = None;
                let mut seen_processor_results = false;
                let mut f_processor_type: Option<String> = None;
                let mut seen_processor_type = false;
                let mut f_status: Option<crate::types::common::ActionStatusOptions> = None;
                let mut seen_status = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "doc" => {
                            if seen_doc {
                                return Err(serde::de::Error::duplicate_field("doc"));
                            }
                            seen_doc = true;
                            f_doc = map.next_value::<Option<crate::types::ingest_simulate::DocumentSimulation>>()?;
                        }
                        "processor_results" => {
                            if seen_processor_results {
                                return Err(serde::de::Error::duplicate_field("processor_results"));
                            }
                            seen_processor_results = true;
                            f_processor_results = map.next_value::<Option<Vec<crate::types::ingest_simulate::PipelineSimulation>>>()?;
                        }
                        "processor_type" => {
                            if seen_processor_type {
                                return Err(serde::de::Error::duplicate_field("processor_type"));
                            }
                            seen_processor_type = true;
                            f_processor_type = map.next_value::<Option<String>>()?;
                        }
                        "status" => {
                            if seen_status {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            seen_status = true;
                            f_status = map
                                .next_value::<Option<crate::types::common::ActionStatusOptions>>(
                                )?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PipelineSimulation {
                    doc: f_doc,
                    processor_results: f_processor_results,
                    processor_type: f_processor_type,
                    status: f_status,
                    tag: f_tag,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PipelineSimulation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.doc {
            map.serialize_entry("doc", v)?;
        }
        if let Some(v) = &self.processor_results {
            map.serialize_entry("processor_results", v)?;
        }
        if let Some(v) = &self.processor_type {
            map.serialize_entry("processor_type", v)?;
        }
        if let Some(v) = &self.status {
            map.serialize_entry("status", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `ingest.simulate` API"]
pub type SimulateResponse = crate::types::ingest::SimulateResponse;
