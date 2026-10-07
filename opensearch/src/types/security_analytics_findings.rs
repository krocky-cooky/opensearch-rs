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

//! Response types of the `security_analytics_findings` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct Document {
    pub document: Option<String>,
    pub found: Option<bool>,
    pub id: Option<String>,
    pub index: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Document {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Document;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Document object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_document: Option<String> = None;
                let mut seen_document = false;
                let mut f_found: Option<bool> = None;
                let mut seen_found = false;
                let mut f_id: Option<String> = None;
                let mut seen_id = false;
                let mut f_index: Option<String> = None;
                let mut seen_index = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "document" => {
                            if seen_document {
                                return Err(serde::de::Error::duplicate_field("document"));
                            }
                            seen_document = true;
                            f_document = map.next_value::<Option<String>>()?;
                        }
                        "found" => {
                            if seen_found {
                                return Err(serde::de::Error::duplicate_field("found"));
                            }
                            seen_found = true;
                            f_found = map.next_value::<Option<bool>>()?;
                        }
                        "id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<String>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Document {
                    document: f_document,
                    found: f_found,
                    id: f_id,
                    index: f_index,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Document {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.document {
            map.serialize_entry("document", v)?;
        }
        if let Some(v) = &self.found {
            map.serialize_entry("found", v)?;
        }
        if let Some(v) = &self.id {
            map.serialize_entry("id", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Finding {
    pub detector_id: Option<String>,
    pub document_list: Option<Vec<crate::types::security_analytics_findings::Document>>,
    pub id: Option<String>,
    pub index: Option<String>,
    pub queries: Option<Vec<crate::types::security_analytics_findings::Query>>,
    pub related_doc_ids: Option<Vec<String>>,
    pub timestamp: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Finding {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Finding;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Finding object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_detector_id: Option<String> = None;
                let mut seen_detector_id = false;
                let mut f_document_list: Option<
                    Vec<crate::types::security_analytics_findings::Document>,
                > = None;
                let mut seen_document_list = false;
                let mut f_id: Option<String> = None;
                let mut seen_id = false;
                let mut f_index: Option<String> = None;
                let mut seen_index = false;
                let mut f_queries: Option<Vec<crate::types::security_analytics_findings::Query>> =
                    None;
                let mut seen_queries = false;
                let mut f_related_doc_ids: Option<Vec<String>> = None;
                let mut seen_related_doc_ids = false;
                let mut f_timestamp: Option<i64> = None;
                let mut seen_timestamp = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "detectorId" => {
                            if seen_detector_id {
                                return Err(serde::de::Error::duplicate_field("detectorId"));
                            }
                            seen_detector_id = true;
                            f_detector_id = map.next_value::<Option<String>>()?;
                        }
                        "document_list" => {
                            if seen_document_list {
                                return Err(serde::de::Error::duplicate_field("document_list"));
                            }
                            seen_document_list = true;
                            f_document_list =
                                map.next_value::<Option<
                                    Vec<crate::types::security_analytics_findings::Document>,
                                >>()?;
                        }
                        "id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<String>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<String>>()?;
                        }
                        "queries" => {
                            if seen_queries {
                                return Err(serde::de::Error::duplicate_field("queries"));
                            }
                            seen_queries = true;
                            f_queries = map.next_value::<Option<Vec<crate::types::security_analytics_findings::Query>>>()?;
                        }
                        "related_doc_ids" => {
                            if seen_related_doc_ids {
                                return Err(serde::de::Error::duplicate_field("related_doc_ids"));
                            }
                            seen_related_doc_ids = true;
                            f_related_doc_ids = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "timestamp" => {
                            if seen_timestamp {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            seen_timestamp = true;
                            f_timestamp = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Finding {
                    detector_id: f_detector_id,
                    document_list: f_document_list,
                    id: f_id,
                    index: f_index,
                    queries: f_queries,
                    related_doc_ids: f_related_doc_ids,
                    timestamp: f_timestamp,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Finding {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.detector_id {
            map.serialize_entry("detectorId", v)?;
        }
        if let Some(v) = &self.document_list {
            map.serialize_entry("document_list", v)?;
        }
        if let Some(v) = &self.id {
            map.serialize_entry("id", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.queries {
            map.serialize_entry("queries", v)?;
        }
        if let Some(v) = &self.related_doc_ids {
            map.serialize_entry("related_doc_ids", v)?;
        }
        if let Some(v) = &self.timestamp {
            map.serialize_entry("timestamp", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct FindingWithScore {
    pub detector_type: Option<String>,
    pub finding: Option<String>,
    pub rules: Option<Vec<String>>,
    pub score: Option<f64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FindingWithScore {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FindingWithScore;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FindingWithScore object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_detector_type: Option<String> = None;
                let mut seen_detector_type = false;
                let mut f_finding: Option<String> = None;
                let mut seen_finding = false;
                let mut f_rules: Option<Vec<String>> = None;
                let mut seen_rules = false;
                let mut f_score: Option<f64> = None;
                let mut seen_score = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "detector_type" => {
                            if seen_detector_type {
                                return Err(serde::de::Error::duplicate_field("detector_type"));
                            }
                            seen_detector_type = true;
                            f_detector_type = map.next_value::<Option<String>>()?;
                        }
                        "finding" => {
                            if seen_finding {
                                return Err(serde::de::Error::duplicate_field("finding"));
                            }
                            seen_finding = true;
                            f_finding = map.next_value::<Option<String>>()?;
                        }
                        "rules" => {
                            if seen_rules {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            seen_rules = true;
                            f_rules = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "score" => {
                            if seen_score {
                                return Err(serde::de::Error::duplicate_field("score"));
                            }
                            seen_score = true;
                            f_score = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FindingWithScore {
                    detector_type: f_detector_type,
                    finding: f_finding,
                    rules: f_rules,
                    score: f_score,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FindingWithScore {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.detector_type {
            map.serialize_entry("detector_type", v)?;
        }
        if let Some(v) = &self.finding {
            map.serialize_entry("finding", v)?;
        }
        if let Some(v) = &self.rules {
            map.serialize_entry("rules", v)?;
        }
        if let Some(v) = &self.score {
            map.serialize_entry("score", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type Findings = Vec<crate::types::security_analytics_findings::Finding>;

#[derive(Debug, Clone)]
pub struct GetFindingsResponse {
    pub findings: crate::types::security_analytics_findings::Findings,
    pub total_findings: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GetFindingsResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GetFindingsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GetFindingsResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_findings: Option<crate::types::security_analytics_findings::Findings> =
                    None;
                let mut f_total_findings: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "findings" => {
                            if f_findings.is_some() {
                                return Err(serde::de::Error::duplicate_field("findings"));
                            }
                            f_findings = Some(map.next_value::<crate::types::security_analytics_findings::Findings>()?);
                        }
                        "total_findings" => {
                            if f_total_findings.is_some() {
                                return Err(serde::de::Error::duplicate_field("total_findings"));
                            }
                            f_total_findings = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GetFindingsResponse {
                    findings: f_findings
                        .ok_or_else(|| serde::de::Error::missing_field("findings"))?,
                    total_findings: f_total_findings
                        .ok_or_else(|| serde::de::Error::missing_field("total_findings"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GetFindingsResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("findings", &self.findings)?;
        map.serialize_entry("total_findings", &self.total_findings)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Query {
    pub fields: Option<Vec<String>>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub query: Option<String>,
    pub query_field_names: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Query {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Query;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Query object")
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
                let mut f_id: Option<String> = None;
                let mut seen_id = false;
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_query: Option<String> = None;
                let mut seen_query = false;
                let mut f_query_field_names: Option<Vec<String>> = None;
                let mut seen_query_field_names = false;
                let mut f_tags: Option<Vec<String>> = None;
                let mut seen_tags = false;
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
                        "id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<String>>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "query" => {
                            if seen_query {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            seen_query = true;
                            f_query = map.next_value::<Option<String>>()?;
                        }
                        "query_field_names" => {
                            if seen_query_field_names {
                                return Err(serde::de::Error::duplicate_field("query_field_names"));
                            }
                            seen_query_field_names = true;
                            f_query_field_names = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "tags" => {
                            if seen_tags {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            seen_tags = true;
                            f_tags = map.next_value::<Option<Vec<String>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Query {
                    fields: f_fields,
                    id: f_id,
                    name: f_name,
                    query: f_query,
                    query_field_names: f_query_field_names,
                    tags: f_tags,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Query {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.id {
            map.serialize_entry("id", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.query {
            map.serialize_entry("query", v)?;
        }
        if let Some(v) = &self.query_field_names {
            map.serialize_entry("query_field_names", v)?;
        }
        if let Some(v) = &self.tags {
            map.serialize_entry("tags", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SearchFindingCorrelationsResponse {
    pub findings: Vec<crate::types::security_analytics_findings::FindingWithScore>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SearchFindingCorrelationsResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SearchFindingCorrelationsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchFindingCorrelationsResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_findings: Option<
                    Vec<crate::types::security_analytics_findings::FindingWithScore>,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "findings" => {
                            if f_findings.is_some() {
                                return Err(serde::de::Error::duplicate_field("findings"));
                            }
                            f_findings =
                                Some(map.next_value::<Vec<
                                    crate::types::security_analytics_findings::FindingWithScore,
                                >>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SearchFindingCorrelationsResponse {
                    findings: f_findings
                        .ok_or_else(|| serde::de::Error::missing_field("findings"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SearchFindingCorrelationsResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("findings", &self.findings)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
