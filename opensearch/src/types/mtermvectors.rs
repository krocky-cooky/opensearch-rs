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

//! Response types of the `mtermvectors` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct MtermvectorsResponse {
    pub docs: Vec<crate::types::mtermvectors::TermVectorsResult>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MtermvectorsResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MtermvectorsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MtermvectorsResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_docs: Option<Vec<crate::types::mtermvectors::TermVectorsResult>> = None;
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
                            f_docs = Some(map.next_value::<Vec<crate::types::mtermvectors::TermVectorsResult>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MtermvectorsResponse {
                    docs: f_docs.ok_or_else(|| serde::de::Error::missing_field("docs"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MtermvectorsResponse {
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

#[derive(Debug, Clone)]
pub struct TermVectorsResult {
    pub id: crate::types::common::Id,
    pub index: crate::types::common::IndexName,
    pub r#type: Option<crate::types::common::Type>,
    pub version: Option<crate::types::common::VersionNumber>,
    pub error: Option<crate::types::common::ErrorCause>,
    pub found: Option<bool>,
    pub term_vectors:
        Option<std::collections::BTreeMap<String, crate::types::termvectors::TermVector>>,
    pub took: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TermVectorsResult {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TermVectorsResult;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TermVectorsResult object")
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
                let mut f_type: Option<crate::types::common::Type> = None;
                let mut seen_type = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version = false;
                let mut f_error: Option<crate::types::common::ErrorCause> = None;
                let mut seen_error = false;
                let mut f_found: Option<bool> = None;
                let mut seen_found = false;
                let mut f_term_vectors: Option<
                    std::collections::BTreeMap<String, crate::types::termvectors::TermVector>,
                > = None;
                let mut seen_term_vectors = false;
                let mut f_took: Option<i64> = None;
                let mut seen_took = false;
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
                        "_type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<crate::types::common::Type>>()?;
                        }
                        "_version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionNumber>>()?;
                        }
                        "error" => {
                            if seen_error {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            seen_error = true;
                            f_error =
                                map.next_value::<Option<crate::types::common::ErrorCause>>()?;
                        }
                        "found" => {
                            if seen_found {
                                return Err(serde::de::Error::duplicate_field("found"));
                            }
                            seen_found = true;
                            f_found = map.next_value::<Option<bool>>()?;
                        }
                        "term_vectors" => {
                            if seen_term_vectors {
                                return Err(serde::de::Error::duplicate_field("term_vectors"));
                            }
                            seen_term_vectors = true;
                            f_term_vectors = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::termvectors::TermVector,
                                >,
                            >>()?;
                        }
                        "took" => {
                            if seen_took {
                                return Err(serde::de::Error::duplicate_field("took"));
                            }
                            seen_took = true;
                            f_took = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TermVectorsResult {
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("_id"))?,
                    index: f_index.ok_or_else(|| serde::de::Error::missing_field("_index"))?,
                    r#type: f_type,
                    version: f_version,
                    error: f_error,
                    found: f_found,
                    term_vectors: f_term_vectors,
                    took: f_took,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TermVectorsResult {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("_id", &self.id)?;
        map.serialize_entry("_index", &self.index)?;
        if let Some(v) = &self.r#type {
            map.serialize_entry("_type", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("_version", v)?;
        }
        if let Some(v) = &self.error {
            map.serialize_entry("error", v)?;
        }
        if let Some(v) = &self.found {
            map.serialize_entry("found", v)?;
        }
        if let Some(v) = &self.term_vectors {
            map.serialize_entry("term_vectors", v)?;
        }
        if let Some(v) = &self.took {
            map.serialize_entry("took", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
