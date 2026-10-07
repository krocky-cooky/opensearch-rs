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

//! Response types of the `explain` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct ExplainResponse {
    pub id: crate::types::common::Id,
    pub index: crate::types::common::IndexName,
    pub r#type: Option<crate::types::common::Type>,
    pub explanation: Option<crate::types::explain::Explanation>,
    pub get: Option<crate::types::explain::ExplainResponseGet>,
    pub matched: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ExplainResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ExplainResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ExplainResponse object")
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
                let mut f_explanation: Option<crate::types::explain::Explanation> = None;
                let mut seen_explanation = false;
                let mut f_get: Option<crate::types::explain::ExplainResponseGet> = None;
                let mut seen_get = false;
                let mut f_matched: Option<bool> = None;
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
                        "explanation" => {
                            if seen_explanation {
                                return Err(serde::de::Error::duplicate_field("explanation"));
                            }
                            seen_explanation = true;
                            f_explanation =
                                map.next_value::<Option<crate::types::explain::Explanation>>()?;
                        }
                        "get" => {
                            if seen_get {
                                return Err(serde::de::Error::duplicate_field("get"));
                            }
                            seen_get = true;
                            f_get = map
                                .next_value::<Option<crate::types::explain::ExplainResponseGet>>(
                                )?;
                        }
                        "matched" => {
                            if f_matched.is_some() {
                                return Err(serde::de::Error::duplicate_field("matched"));
                            }
                            f_matched = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ExplainResponse {
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("_id"))?,
                    index: f_index.ok_or_else(|| serde::de::Error::missing_field("_index"))?,
                    r#type: f_type,
                    explanation: f_explanation,
                    get: f_get,
                    matched: f_matched.ok_or_else(|| serde::de::Error::missing_field("matched"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ExplainResponse {
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
        if let Some(v) = &self.explanation {
            map.serialize_entry("explanation", v)?;
        }
        if let Some(v) = &self.get {
            map.serialize_entry("get", v)?;
        }
        map.serialize_entry("matched", &self.matched)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ExplainResponseGet {
    #[doc = "The primary term of the document."]
    pub primary_term: Option<i64>,
    pub routing: Option<crate::types::common::Routing>,
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    pub source: Option<Box<serde_json::value::RawValue>>,
    #[doc = "The fields retrieved from the document."]
    pub fields: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "Whether the document was found."]
    pub found: bool,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ExplainResponseGet {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ExplainResponseGet;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ExplainResponseGet object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_primary_term: Option<i64> = None;
                let mut seen_primary_term = false;
                let mut f_routing: Option<crate::types::common::Routing> = None;
                let mut seen_routing = false;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut seen_seq_no = false;
                let mut f_source: Option<Box<serde_json::value::RawValue>> = None;
                let mut seen_source = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_fields = false;
                let mut f_found: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_primary_term" => {
                            if seen_primary_term {
                                return Err(serde::de::Error::duplicate_field("_primary_term"));
                            }
                            seen_primary_term = true;
                            f_primary_term = map.next_value::<Option<i64>>()?;
                        }
                        "_routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("_routing"));
                            }
                            seen_routing = true;
                            f_routing =
                                map.next_value::<Option<crate::types::common::Routing>>()?;
                        }
                        "_seq_no" => {
                            if seen_seq_no {
                                return Err(serde::de::Error::duplicate_field("_seq_no"));
                            }
                            seen_seq_no = true;
                            f_seq_no =
                                map.next_value::<Option<crate::types::common::SequenceNumber>>()?;
                        }
                        "_source" => {
                            if seen_source {
                                return Err(serde::de::Error::duplicate_field("_source"));
                            }
                            seen_source = true;
                            f_source =
                                map.next_value::<Option<Box<serde_json::value::RawValue>>>()?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "found" => {
                            if f_found.is_some() {
                                return Err(serde::de::Error::duplicate_field("found"));
                            }
                            f_found = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ExplainResponseGet {
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
impl serde::Serialize for ExplainResponseGet {
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
impl ExplainResponseGet {
    #[doc = "Decodes the document body (`_source`) into `T`.\n\nReturns `Ok(None)` when the response carries no `_source` (`_source: false` in the request, or a mapping that disables it), and an error when the document does not match `T`.\n\n`T` may borrow from the response to avoid copying strings: use `#[serde(borrow)] Cow<'a, str>` fields rather than `&'a str`, which cannot hold a string containing JSON escapes."]
    pub fn source_as<'a, T>(&'a self) -> std::result::Result<Option<T>, serde_json::Error>
    where
        T: serde::Deserialize<'a>,
    {
        self.source
            .as_deref()
            .map(|raw| serde_json::from_str(raw.get()))
            .transpose()
    }
}

#[derive(Debug, Clone)]
pub struct Explanation {
    pub description: String,
    pub details: Option<Vec<crate::types::explain::Explanation>>,
    pub value: f64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Explanation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Explanation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Explanation object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut f_details: Option<Vec<crate::types::explain::Explanation>> = None;
                let mut seen_details = false;
                let mut f_value: Option<f64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if f_description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            f_description = Some(map.next_value::<String>()?);
                        }
                        "details" => {
                            if seen_details {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            seen_details = true;
                            f_details = map
                                .next_value::<Option<Vec<crate::types::explain::Explanation>>>()?;
                        }
                        "value" => {
                            if f_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            f_value = Some(map.next_value::<f64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Explanation {
                    description: f_description
                        .ok_or_else(|| serde::de::Error::missing_field("description"))?,
                    details: f_details,
                    value: f_value.ok_or_else(|| serde::de::Error::missing_field("value"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Explanation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("description", &self.description)?;
        if let Some(v) = &self.details {
            map.serialize_entry("details", v)?;
        }
        map.serialize_entry("value", &self.value)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
