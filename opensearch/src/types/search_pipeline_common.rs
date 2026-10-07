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

//! Response types of the `search_pipeline_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct AgenticContextResponseProcessor {
    #[doc = "Whether to include the agent's execution step summary in the response. Available for conversational agents only."]
    pub agent_steps_summary: Option<bool>,
    pub description: Option<String>,
    #[doc = "Whether to include the generated DSL query in the response. Available for both conversational and flow agents."]
    pub dsl_query: Option<bool>,
    pub ignore_failure: Option<bool>,
    pub tag: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AgenticContextResponseProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AgenticContextResponseProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AgenticContextResponseProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_agent_steps_summary: Option<bool> = None;
                let mut seen_agent_steps_summary = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_dsl_query: Option<bool> = None;
                let mut seen_dsl_query = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "agent_steps_summary" => {
                            if seen_agent_steps_summary {
                                return Err(serde::de::Error::duplicate_field(
                                    "agent_steps_summary",
                                ));
                            }
                            seen_agent_steps_summary = true;
                            f_agent_steps_summary = map.next_value::<Option<bool>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "dsl_query" => {
                            if seen_dsl_query {
                                return Err(serde::de::Error::duplicate_field("dsl_query"));
                            }
                            seen_dsl_query = true;
                            f_dsl_query = map.next_value::<Option<bool>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
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
                Ok(AgenticContextResponseProcessor {
                    agent_steps_summary: f_agent_steps_summary,
                    description: f_description,
                    dsl_query: f_dsl_query,
                    ignore_failure: f_ignore_failure,
                    tag: f_tag,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AgenticContextResponseProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.agent_steps_summary {
            map.serialize_entry("agent_steps_summary", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.dsl_query {
            map.serialize_entry("dsl_query", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
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

#[derive(Debug, Clone)]
pub struct AgenticQueryTranslatorRequestProcessor {
    #[doc = "The ID of the agent to use for query translation."]
    pub agent_id: String,
    pub description: Option<String>,
    pub ignore_failure: Option<bool>,
    pub tag: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AgenticQueryTranslatorRequestProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AgenticQueryTranslatorRequestProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AgenticQueryTranslatorRequestProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_agent_id: Option<String> = None;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "agent_id" => {
                            if f_agent_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("agent_id"));
                            }
                            f_agent_id = Some(map.next_value::<String>()?);
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
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
                Ok(AgenticQueryTranslatorRequestProcessor {
                    agent_id: f_agent_id
                        .ok_or_else(|| serde::de::Error::missing_field("agent_id"))?,
                    description: f_description,
                    ignore_failure: f_ignore_failure,
                    tag: f_tag,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AgenticQueryTranslatorRequestProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("agent_id", &self.agent_id)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
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

#[derive(Debug, Clone)]
pub struct CollapseResponseProcessor {
    pub context_prefix: Option<String>,
    pub description: Option<String>,
    pub field: String,
    pub ignore_failure: Option<bool>,
    pub tag: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CollapseResponseProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CollapseResponseProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CollapseResponseProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_context_prefix: Option<String> = None;
                let mut seen_context_prefix = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_field: Option<String> = None;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "context_prefix" => {
                            if seen_context_prefix {
                                return Err(serde::de::Error::duplicate_field("context_prefix"));
                            }
                            seen_context_prefix = true;
                            f_context_prefix = map.next_value::<Option<String>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<String>()?);
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
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
                Ok(CollapseResponseProcessor {
                    context_prefix: f_context_prefix,
                    description: f_description,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    ignore_failure: f_ignore_failure,
                    tag: f_tag,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CollapseResponseProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.context_prefix {
            map.serialize_entry("context_prefix", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
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

#[derive(Debug, Clone)]
pub struct FilterQueryRequestProcessor {
    pub description: Option<String>,
    pub ignore_failure: Option<bool>,
    pub query: Option<crate::types::common_query_dsl::QueryContainer>,
    pub tag: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FilterQueryRequestProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FilterQueryRequestProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FilterQueryRequestProcessor object")
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
                let mut seen_description = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_query: Option<crate::types::common_query_dsl::QueryContainer> = None;
                let mut seen_query = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "query" => {
                            if seen_query {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            seen_query = true;
                            f_query = map.next_value::<Option<crate::types::common_query_dsl::QueryContainer>>()?;
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
                Ok(FilterQueryRequestProcessor {
                    description: f_description,
                    ignore_failure: f_ignore_failure,
                    query: f_query,
                    tag: f_tag,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FilterQueryRequestProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.query {
            map.serialize_entry("query", v)?;
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

#[derive(Debug, Clone)]
pub struct MLOpenSearchReranker {
    pub model_id: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MLOpenSearchReranker {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MLOpenSearchReranker;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MLOpenSearchReranker object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_model_id: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "model_id" => {
                            if f_model_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("model_id"));
                            }
                            f_model_id = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MLOpenSearchReranker {
                    model_id: f_model_id
                        .ok_or_else(|| serde::de::Error::missing_field("model_id"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MLOpenSearchReranker {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("model_id", &self.model_id)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type NeuralFieldMap = std::collections::BTreeMap<String, String>;

#[derive(Debug, Clone)]
pub struct NeuralQueryEnricherRequestProcessor {
    pub default_model_id: Option<String>,
    pub description: Option<String>,
    pub neural_field_default_id: Option<crate::types::search_pipeline_common::NeuralFieldMap>,
    pub tag: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NeuralQueryEnricherRequestProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NeuralQueryEnricherRequestProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NeuralQueryEnricherRequestProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_default_model_id: Option<String> = None;
                let mut seen_default_model_id = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_neural_field_default_id: Option<
                    crate::types::search_pipeline_common::NeuralFieldMap,
                > = None;
                let mut seen_neural_field_default_id = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "default_model_id" => {
                            if seen_default_model_id {
                                return Err(serde::de::Error::duplicate_field("default_model_id"));
                            }
                            seen_default_model_id = true;
                            f_default_model_id = map.next_value::<Option<String>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "neural_field_default_id" => {
                            if seen_neural_field_default_id {
                                return Err(serde::de::Error::duplicate_field(
                                    "neural_field_default_id",
                                ));
                            }
                            seen_neural_field_default_id = true;
                            f_neural_field_default_id = map.next_value::<Option<crate::types::search_pipeline_common::NeuralFieldMap>>()?;
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
                Ok(NeuralQueryEnricherRequestProcessor {
                    default_model_id: f_default_model_id,
                    description: f_description,
                    neural_field_default_id: f_neural_field_default_id,
                    tag: f_tag,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NeuralQueryEnricherRequestProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.default_model_id {
            map.serialize_entry("default_model_id", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.neural_field_default_id {
            map.serialize_entry("neural_field_default_id", v)?;
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

#[derive(Debug, Clone)]
pub struct NormalizationPhaseResultsProcessor {
    pub combination: Option<crate::types::search_pipeline_common::ScoreCombination>,
    pub description: Option<String>,
    pub ignore_failure: Option<bool>,
    pub normalization: Option<crate::types::search_pipeline_common::ScoreNormalization>,
    pub tag: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NormalizationPhaseResultsProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NormalizationPhaseResultsProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NormalizationPhaseResultsProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_combination: Option<
                    crate::types::search_pipeline_common::ScoreCombination,
                > = None;
                let mut seen_combination = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_normalization: Option<
                    crate::types::search_pipeline_common::ScoreNormalization,
                > = None;
                let mut seen_normalization = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "combination" => {
                            if seen_combination {
                                return Err(serde::de::Error::duplicate_field("combination"));
                            }
                            seen_combination = true;
                            f_combination = map.next_value::<Option<crate::types::search_pipeline_common::ScoreCombination>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "normalization" => {
                            if seen_normalization {
                                return Err(serde::de::Error::duplicate_field("normalization"));
                            }
                            seen_normalization = true;
                            f_normalization =
                                map.next_value::<Option<
                                    crate::types::search_pipeline_common::ScoreNormalization,
                                >>()?;
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
                Ok(NormalizationPhaseResultsProcessor {
                    combination: f_combination,
                    description: f_description,
                    ignore_failure: f_ignore_failure,
                    normalization: f_normalization,
                    tag: f_tag,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NormalizationPhaseResultsProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.combination {
            map.serialize_entry("combination", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.normalization {
            map.serialize_entry("normalization", v)?;
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

#[derive(Debug, Clone)]
pub struct OversampleRequestProcessor {
    pub content_prefix: Option<String>,
    pub description: Option<String>,
    pub ignore_failure: Option<bool>,
    pub sample_factor: f64,
    pub tag: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for OversampleRequestProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = OversampleRequestProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a OversampleRequestProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_content_prefix: Option<String> = None;
                let mut seen_content_prefix = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_sample_factor: Option<f64> = None;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "content_prefix" => {
                            if seen_content_prefix {
                                return Err(serde::de::Error::duplicate_field("content_prefix"));
                            }
                            seen_content_prefix = true;
                            f_content_prefix = map.next_value::<Option<String>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "sample_factor" => {
                            if f_sample_factor.is_some() {
                                return Err(serde::de::Error::duplicate_field("sample_factor"));
                            }
                            f_sample_factor = Some(map.next_value::<f64>()?);
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
                Ok(OversampleRequestProcessor {
                    content_prefix: f_content_prefix,
                    description: f_description,
                    ignore_failure: f_ignore_failure,
                    sample_factor: f_sample_factor
                        .ok_or_else(|| serde::de::Error::missing_field("sample_factor"))?,
                    tag: f_tag,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for OversampleRequestProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.content_prefix {
            map.serialize_entry("content_prefix", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        map.serialize_entry("sample_factor", &self.sample_factor)?;
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PersonalizeSearchRankingResponseProcessor {
    pub campaign_arn: String,
    pub description: Option<String>,
    pub iam_role_arn: Option<String>,
    pub ignore_failure: Option<bool>,
    pub item_id_field: Option<String>,
    pub recipe: String,
    pub tag: Option<String>,
    pub weight: f64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PersonalizeSearchRankingResponseProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PersonalizeSearchRankingResponseProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PersonalizeSearchRankingResponseProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_campaign_arn: Option<String> = None;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_iam_role_arn: Option<String> = None;
                let mut seen_iam_role_arn = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_item_id_field: Option<String> = None;
                let mut seen_item_id_field = false;
                let mut f_recipe: Option<String> = None;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_weight: Option<f64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "campaign_arn" => {
                            if f_campaign_arn.is_some() {
                                return Err(serde::de::Error::duplicate_field("campaign_arn"));
                            }
                            f_campaign_arn = Some(map.next_value::<String>()?);
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "iam_role_arn" => {
                            if seen_iam_role_arn {
                                return Err(serde::de::Error::duplicate_field("iam_role_arn"));
                            }
                            seen_iam_role_arn = true;
                            f_iam_role_arn = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "item_id_field" => {
                            if seen_item_id_field {
                                return Err(serde::de::Error::duplicate_field("item_id_field"));
                            }
                            seen_item_id_field = true;
                            f_item_id_field = map.next_value::<Option<String>>()?;
                        }
                        "recipe" => {
                            if f_recipe.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipe"));
                            }
                            f_recipe = Some(map.next_value::<String>()?);
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "weight" => {
                            if f_weight.is_some() {
                                return Err(serde::de::Error::duplicate_field("weight"));
                            }
                            f_weight = Some(map.next_value::<f64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PersonalizeSearchRankingResponseProcessor {
                    campaign_arn: f_campaign_arn
                        .ok_or_else(|| serde::de::Error::missing_field("campaign_arn"))?,
                    description: f_description,
                    iam_role_arn: f_iam_role_arn,
                    ignore_failure: f_ignore_failure,
                    item_id_field: f_item_id_field,
                    recipe: f_recipe.ok_or_else(|| serde::de::Error::missing_field("recipe"))?,
                    tag: f_tag,
                    weight: f_weight.ok_or_else(|| serde::de::Error::missing_field("weight"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PersonalizeSearchRankingResponseProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("campaign_arn", &self.campaign_arn)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.iam_role_arn {
            map.serialize_entry("iam_role_arn", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.item_id_field {
            map.serialize_entry("item_id_field", v)?;
        }
        map.serialize_entry("recipe", &self.recipe)?;
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("weight", &self.weight)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct PhaseResultsProcessor {
    raw: Box<serde_json::value::RawValue>,
}
impl PhaseResultsProcessor {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `normalization_processor` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_normalization_processor(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::PhaseResultsProcessorNormalizationProcessor,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "normalization_processor")
    }
    #[doc = "Parses the value as the `score_ranker_processor` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_score_ranker_processor(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::PhaseResultsProcessorScoreRankerProcessor,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "score_ranker_processor")
    }
}

#[derive(Debug, Clone)]
pub struct PhaseResultsProcessorNormalizationProcessor {
    pub normalization_processor:
        crate::types::search_pipeline_common::NormalizationPhaseResultsProcessor,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PhaseResultsProcessorNormalizationProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PhaseResultsProcessorNormalizationProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PhaseResultsProcessorNormalizationProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_normalization_processor: Option<
                    crate::types::search_pipeline_common::NormalizationPhaseResultsProcessor,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "normalization-processor" => {
                            if f_normalization_processor.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "normalization-processor",
                                ));
                            }
                            f_normalization_processor = Some(map.next_value::<crate::types::search_pipeline_common::NormalizationPhaseResultsProcessor>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PhaseResultsProcessorNormalizationProcessor {
                    normalization_processor: f_normalization_processor.ok_or_else(|| {
                        serde::de::Error::missing_field("normalization-processor")
                    })?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PhaseResultsProcessorNormalizationProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("normalization-processor", &self.normalization_processor)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PhaseResultsProcessorScoreRankerProcessor {
    pub score_ranker_processor:
        crate::types::search_pipeline_common::ScoreRankerPhaseResultsProcessor,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PhaseResultsProcessorScoreRankerProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PhaseResultsProcessorScoreRankerProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PhaseResultsProcessorScoreRankerProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_score_ranker_processor: Option<
                    crate::types::search_pipeline_common::ScoreRankerPhaseResultsProcessor,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "score-ranker-processor" => {
                            if f_score_ranker_processor.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "score-ranker-processor",
                                ));
                            }
                            f_score_ranker_processor = Some(map.next_value::<crate::types::search_pipeline_common::ScoreRankerPhaseResultsProcessor>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PhaseResultsProcessorScoreRankerProcessor {
                    score_ranker_processor: f_score_ranker_processor
                        .ok_or_else(|| serde::de::Error::missing_field("score-ranker-processor"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PhaseResultsProcessorScoreRankerProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("score-ranker-processor", &self.score_ranker_processor)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RenameFieldResponseProcessor {
    pub description: Option<String>,
    pub field: String,
    pub ignore_failure: Option<bool>,
    pub tag: Option<String>,
    pub target_field: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RenameFieldResponseProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RenameFieldResponseProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RenameFieldResponseProcessor object")
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
                let mut seen_description = false;
                let mut f_field: Option<String> = None;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_target_field: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<String>()?);
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "target_field" => {
                            if f_target_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            f_target_field = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RenameFieldResponseProcessor {
                    description: f_description,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    ignore_failure: f_ignore_failure,
                    tag: f_tag,
                    target_field: f_target_field
                        .ok_or_else(|| serde::de::Error::missing_field("target_field"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RenameFieldResponseProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        map.serialize_entry("target_field", &self.target_field)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct RequestProcessor {
    raw: Box<serde_json::value::RawValue>,
}
impl RequestProcessor {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `agentic_query_translator` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_agentic_query_translator(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::RequestProcessorAgenticQueryTranslator,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "agentic_query_translator")
    }
    #[doc = "Parses the value as the `filter_query` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_filter_query(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::RequestProcessorFilterQuery,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "filter_query")
    }
    #[doc = "Parses the value as the `neural_query_enricher` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_neural_query_enricher(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::RequestProcessorNeuralQueryEnricher,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "neural_query_enricher")
    }
    #[doc = "Parses the value as the `script` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_script(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::RequestProcessorScript,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "script")
    }
    #[doc = "Parses the value as the `oversample` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_oversample(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::RequestProcessorOversample,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "oversample")
    }
}

#[derive(Debug, Clone)]
pub struct RequestProcessorAgenticQueryTranslator {
    pub agentic_query_translator:
        crate::types::search_pipeline_common::AgenticQueryTranslatorRequestProcessor,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RequestProcessorAgenticQueryTranslator {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RequestProcessorAgenticQueryTranslator;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RequestProcessorAgenticQueryTranslator object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_agentic_query_translator: Option<
                    crate::types::search_pipeline_common::AgenticQueryTranslatorRequestProcessor,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "agentic_query_translator" => {
                            if f_agentic_query_translator.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "agentic_query_translator",
                                ));
                            }
                            f_agentic_query_translator = Some(map.next_value::<crate::types::search_pipeline_common::AgenticQueryTranslatorRequestProcessor>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RequestProcessorAgenticQueryTranslator {
                    agentic_query_translator: f_agentic_query_translator.ok_or_else(|| {
                        serde::de::Error::missing_field("agentic_query_translator")
                    })?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RequestProcessorAgenticQueryTranslator {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("agentic_query_translator", &self.agentic_query_translator)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RequestProcessorFilterQuery {
    pub filter_query: crate::types::search_pipeline_common::FilterQueryRequestProcessor,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RequestProcessorFilterQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RequestProcessorFilterQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RequestProcessorFilterQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_filter_query: Option<
                    crate::types::search_pipeline_common::FilterQueryRequestProcessor,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "filter_query" => {
                            if f_filter_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter_query"));
                            }
                            f_filter_query = Some(map.next_value::<crate::types::search_pipeline_common::FilterQueryRequestProcessor>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RequestProcessorFilterQuery {
                    filter_query: f_filter_query
                        .ok_or_else(|| serde::de::Error::missing_field("filter_query"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RequestProcessorFilterQuery {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("filter_query", &self.filter_query)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RequestProcessorNeuralQueryEnricher {
    pub neural_query_enricher:
        crate::types::search_pipeline_common::NeuralQueryEnricherRequestProcessor,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RequestProcessorNeuralQueryEnricher {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RequestProcessorNeuralQueryEnricher;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RequestProcessorNeuralQueryEnricher object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_neural_query_enricher: Option<
                    crate::types::search_pipeline_common::NeuralQueryEnricherRequestProcessor,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "neural_query_enricher" => {
                            if f_neural_query_enricher.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "neural_query_enricher",
                                ));
                            }
                            f_neural_query_enricher = Some(map.next_value::<crate::types::search_pipeline_common::NeuralQueryEnricherRequestProcessor>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RequestProcessorNeuralQueryEnricher {
                    neural_query_enricher: f_neural_query_enricher
                        .ok_or_else(|| serde::de::Error::missing_field("neural_query_enricher"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RequestProcessorNeuralQueryEnricher {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("neural_query_enricher", &self.neural_query_enricher)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RequestProcessorOversample {
    pub oversample: crate::types::search_pipeline_common::OversampleRequestProcessor,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RequestProcessorOversample {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RequestProcessorOversample;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RequestProcessorOversample object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_oversample: Option<
                    crate::types::search_pipeline_common::OversampleRequestProcessor,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "oversample" => {
                            if f_oversample.is_some() {
                                return Err(serde::de::Error::duplicate_field("oversample"));
                            }
                            f_oversample = Some(map.next_value::<crate::types::search_pipeline_common::OversampleRequestProcessor>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RequestProcessorOversample {
                    oversample: f_oversample
                        .ok_or_else(|| serde::de::Error::missing_field("oversample"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RequestProcessorOversample {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("oversample", &self.oversample)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RequestProcessorScript {
    pub script: crate::types::search_pipeline_common::SearchScriptRequestProcessor,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RequestProcessorScript {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RequestProcessorScript;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RequestProcessorScript object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_script: Option<
                    crate::types::search_pipeline_common::SearchScriptRequestProcessor,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "script" => {
                            if f_script.is_some() {
                                return Err(serde::de::Error::duplicate_field("script"));
                            }
                            f_script = Some(map.next_value::<crate::types::search_pipeline_common::SearchScriptRequestProcessor>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RequestProcessorScript {
                    script: f_script.ok_or_else(|| serde::de::Error::missing_field("script"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RequestProcessorScript {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("script", &self.script)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RerankContext {
    pub document_fields: Vec<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RerankContext {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RerankContext;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RerankContext object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_document_fields: Option<Vec<String>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "document_fields" => {
                            if f_document_fields.is_some() {
                                return Err(serde::de::Error::duplicate_field("document_fields"));
                            }
                            f_document_fields = Some(map.next_value::<Vec<String>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RerankContext {
                    document_fields: f_document_fields
                        .ok_or_else(|| serde::de::Error::missing_field("document_fields"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RerankContext {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("document_fields", &self.document_fields)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RerankResponseProcessor {
    pub context: Option<crate::types::search_pipeline_common::RerankContext>,
    pub description: Option<String>,
    pub ignore_failure: Option<bool>,
    pub ml_opensearch: Option<crate::types::search_pipeline_common::MLOpenSearchReranker>,
    pub tag: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RerankResponseProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RerankResponseProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RerankResponseProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_context: Option<crate::types::search_pipeline_common::RerankContext> =
                    None;
                let mut seen_context = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_ml_opensearch: Option<
                    crate::types::search_pipeline_common::MLOpenSearchReranker,
                > = None;
                let mut seen_ml_opensearch = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "context" => {
                            if seen_context {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            seen_context = true;
                            f_context = map.next_value::<Option<crate::types::search_pipeline_common::RerankContext>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "ml_opensearch" => {
                            if seen_ml_opensearch {
                                return Err(serde::de::Error::duplicate_field("ml_opensearch"));
                            }
                            seen_ml_opensearch = true;
                            f_ml_opensearch = map.next_value::<Option<
                                crate::types::search_pipeline_common::MLOpenSearchReranker,
                            >>()?;
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
                Ok(RerankResponseProcessor {
                    context: f_context,
                    description: f_description,
                    ignore_failure: f_ignore_failure,
                    ml_opensearch: f_ml_opensearch,
                    tag: f_tag,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RerankResponseProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.context {
            map.serialize_entry("context", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.ml_opensearch {
            map.serialize_entry("ml_opensearch", v)?;
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

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ResponseProcessor {
    raw: Box<serde_json::value::RawValue>,
}
impl ResponseProcessor {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `agentic_context` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_agentic_context(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::ResponseProcessorAgenticContext,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "agentic_context")
    }
    #[doc = "Parses the value as the `personalize_search_ranking` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_personalize_search_ranking(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::ResponseProcessorPersonalizeSearchRanking,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "personalize_search_ranking")
    }
    #[doc = "Parses the value as the `retrieval_augmented_generation` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_retrieval_augmented_generation(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::ResponseProcessorRetrievalAugmentedGeneration,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "retrieval_augmented_generation")
    }
    #[doc = "Parses the value as the `rename_field` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_rename_field(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::ResponseProcessorRenameField,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "rename_field")
    }
    #[doc = "Parses the value as the `rerank` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_rerank(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::ResponseProcessorRerank,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "rerank")
    }
    #[doc = "Parses the value as the `collapse` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_collapse(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::ResponseProcessorCollapse,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "collapse")
    }
    #[doc = "Parses the value as the `truncate_hits` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_truncate_hits(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::ResponseProcessorTruncateHits,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "truncate_hits")
    }
    #[doc = "Parses the value as the `sort` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_sort(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::ResponseProcessorSort,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "sort")
    }
    #[doc = "Parses the value as the `split` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_split(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::ResponseProcessorSplit,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "split")
    }
}

#[derive(Debug, Clone)]
pub struct ResponseProcessorAgenticContext {
    pub agentic_context: crate::types::search_pipeline_common::AgenticContextResponseProcessor,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ResponseProcessorAgenticContext {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ResponseProcessorAgenticContext;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ResponseProcessorAgenticContext object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_agentic_context: Option<
                    crate::types::search_pipeline_common::AgenticContextResponseProcessor,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "agentic_context" => {
                            if f_agentic_context.is_some() {
                                return Err(serde::de::Error::duplicate_field("agentic_context"));
                            }
                            f_agentic_context = Some(map.next_value::<crate::types::search_pipeline_common::AgenticContextResponseProcessor>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ResponseProcessorAgenticContext {
                    agentic_context: f_agentic_context
                        .ok_or_else(|| serde::de::Error::missing_field("agentic_context"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ResponseProcessorAgenticContext {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("agentic_context", &self.agentic_context)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ResponseProcessorCollapse {
    pub collapse: crate::types::search_pipeline_common::CollapseResponseProcessor,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ResponseProcessorCollapse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ResponseProcessorCollapse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ResponseProcessorCollapse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_collapse: Option<
                    crate::types::search_pipeline_common::CollapseResponseProcessor,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "collapse" => {
                            if f_collapse.is_some() {
                                return Err(serde::de::Error::duplicate_field("collapse"));
                            }
                            f_collapse = Some(map.next_value::<crate::types::search_pipeline_common::CollapseResponseProcessor>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ResponseProcessorCollapse {
                    collapse: f_collapse
                        .ok_or_else(|| serde::de::Error::missing_field("collapse"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ResponseProcessorCollapse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("collapse", &self.collapse)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ResponseProcessorPersonalizeSearchRanking {
    pub personalize_search_ranking:
        crate::types::search_pipeline_common::PersonalizeSearchRankingResponseProcessor,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ResponseProcessorPersonalizeSearchRanking {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ResponseProcessorPersonalizeSearchRanking;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ResponseProcessorPersonalizeSearchRanking object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_personalize_search_ranking: Option<
                    crate::types::search_pipeline_common::PersonalizeSearchRankingResponseProcessor,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "personalize_search_ranking" => {
                            if f_personalize_search_ranking.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "personalize_search_ranking",
                                ));
                            }
                            f_personalize_search_ranking = Some(map.next_value::<crate::types::search_pipeline_common::PersonalizeSearchRankingResponseProcessor>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ResponseProcessorPersonalizeSearchRanking {
                    personalize_search_ranking: f_personalize_search_ranking.ok_or_else(|| {
                        serde::de::Error::missing_field("personalize_search_ranking")
                    })?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ResponseProcessorPersonalizeSearchRanking {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry(
            "personalize_search_ranking",
            &self.personalize_search_ranking,
        )?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ResponseProcessorRenameField {
    pub rename_field: crate::types::search_pipeline_common::RenameFieldResponseProcessor,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ResponseProcessorRenameField {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ResponseProcessorRenameField;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ResponseProcessorRenameField object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_rename_field: Option<
                    crate::types::search_pipeline_common::RenameFieldResponseProcessor,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "rename_field" => {
                            if f_rename_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("rename_field"));
                            }
                            f_rename_field = Some(map.next_value::<crate::types::search_pipeline_common::RenameFieldResponseProcessor>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ResponseProcessorRenameField {
                    rename_field: f_rename_field
                        .ok_or_else(|| serde::de::Error::missing_field("rename_field"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ResponseProcessorRenameField {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("rename_field", &self.rename_field)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ResponseProcessorRerank {
    pub rerank: crate::types::search_pipeline_common::RerankResponseProcessor,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ResponseProcessorRerank {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ResponseProcessorRerank;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ResponseProcessorRerank object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_rerank: Option<
                    crate::types::search_pipeline_common::RerankResponseProcessor,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "rerank" => {
                            if f_rerank.is_some() {
                                return Err(serde::de::Error::duplicate_field("rerank"));
                            }
                            f_rerank = Some(map.next_value::<crate::types::search_pipeline_common::RerankResponseProcessor>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ResponseProcessorRerank {
                    rerank: f_rerank.ok_or_else(|| serde::de::Error::missing_field("rerank"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ResponseProcessorRerank {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("rerank", &self.rerank)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ResponseProcessorRetrievalAugmentedGeneration {
    pub retrieval_augmented_generation:
        crate::types::search_pipeline_common::RetrievalAugmentedGenerationResponseProcessor,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ResponseProcessorRetrievalAugmentedGeneration {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ResponseProcessorRetrievalAugmentedGeneration;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ResponseProcessorRetrievalAugmentedGeneration object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_retrieval_augmented_generation: Option<crate::types::search_pipeline_common::RetrievalAugmentedGenerationResponseProcessor> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "retrieval_augmented_generation" => {
                            if f_retrieval_augmented_generation.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "retrieval_augmented_generation",
                                ));
                            }
                            f_retrieval_augmented_generation = Some(map.next_value::<crate::types::search_pipeline_common::RetrievalAugmentedGenerationResponseProcessor>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ResponseProcessorRetrievalAugmentedGeneration {
                    retrieval_augmented_generation: f_retrieval_augmented_generation.ok_or_else(
                        || serde::de::Error::missing_field("retrieval_augmented_generation"),
                    )?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ResponseProcessorRetrievalAugmentedGeneration {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry(
            "retrieval_augmented_generation",
            &self.retrieval_augmented_generation,
        )?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ResponseProcessorSort {
    pub sort: crate::types::search_pipeline_common::SortResponseProcessor,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ResponseProcessorSort {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ResponseProcessorSort;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ResponseProcessorSort object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_sort: Option<
                    crate::types::search_pipeline_common::SortResponseProcessor,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "sort" => {
                            if f_sort.is_some() {
                                return Err(serde::de::Error::duplicate_field("sort"));
                            }
                            f_sort = Some(map.next_value::<crate::types::search_pipeline_common::SortResponseProcessor>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ResponseProcessorSort {
                    sort: f_sort.ok_or_else(|| serde::de::Error::missing_field("sort"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ResponseProcessorSort {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("sort", &self.sort)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ResponseProcessorSplit {
    pub split: crate::types::search_pipeline_common::SplitResponseProcessor,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ResponseProcessorSplit {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ResponseProcessorSplit;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ResponseProcessorSplit object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_split: Option<
                    crate::types::search_pipeline_common::SplitResponseProcessor,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "split" => {
                            if f_split.is_some() {
                                return Err(serde::de::Error::duplicate_field("split"));
                            }
                            f_split = Some(map.next_value::<crate::types::search_pipeline_common::SplitResponseProcessor>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ResponseProcessorSplit {
                    split: f_split.ok_or_else(|| serde::de::Error::missing_field("split"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ResponseProcessorSplit {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("split", &self.split)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ResponseProcessorTruncateHits {
    pub truncate_hits: crate::types::search_pipeline_common::TruncateHitsResponseProcessor,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ResponseProcessorTruncateHits {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ResponseProcessorTruncateHits;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ResponseProcessorTruncateHits object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_truncate_hits: Option<
                    crate::types::search_pipeline_common::TruncateHitsResponseProcessor,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "truncate_hits" => {
                            if f_truncate_hits.is_some() {
                                return Err(serde::de::Error::duplicate_field("truncate_hits"));
                            }
                            f_truncate_hits = Some(map.next_value::<crate::types::search_pipeline_common::TruncateHitsResponseProcessor>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ResponseProcessorTruncateHits {
                    truncate_hits: f_truncate_hits
                        .ok_or_else(|| serde::de::Error::missing_field("truncate_hits"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ResponseProcessorTruncateHits {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("truncate_hits", &self.truncate_hits)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RetrievalAugmentedGenerationResponseProcessor {
    pub context_field_list: Vec<String>,
    pub description: Option<String>,
    pub model_id: String,
    pub system_prompt: Option<String>,
    pub tag: Option<String>,
    pub user_instructions: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RetrievalAugmentedGenerationResponseProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RetrievalAugmentedGenerationResponseProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RetrievalAugmentedGenerationResponseProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_context_field_list: Option<Vec<String>> = None;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_model_id: Option<String> = None;
                let mut f_system_prompt: Option<String> = None;
                let mut seen_system_prompt = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_user_instructions: Option<String> = None;
                let mut seen_user_instructions = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "context_field_list" => {
                            if f_context_field_list.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "context_field_list",
                                ));
                            }
                            f_context_field_list = Some(map.next_value::<Vec<String>>()?);
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "model_id" => {
                            if f_model_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("model_id"));
                            }
                            f_model_id = Some(map.next_value::<String>()?);
                        }
                        "system_prompt" => {
                            if seen_system_prompt {
                                return Err(serde::de::Error::duplicate_field("system_prompt"));
                            }
                            seen_system_prompt = true;
                            f_system_prompt = map.next_value::<Option<String>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "user_instructions" => {
                            if seen_user_instructions {
                                return Err(serde::de::Error::duplicate_field("user_instructions"));
                            }
                            seen_user_instructions = true;
                            f_user_instructions = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RetrievalAugmentedGenerationResponseProcessor {
                    context_field_list: f_context_field_list
                        .ok_or_else(|| serde::de::Error::missing_field("context_field_list"))?,
                    description: f_description,
                    model_id: f_model_id
                        .ok_or_else(|| serde::de::Error::missing_field("model_id"))?,
                    system_prompt: f_system_prompt,
                    tag: f_tag,
                    user_instructions: f_user_instructions,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RetrievalAugmentedGenerationResponseProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("context_field_list", &self.context_field_list)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        map.serialize_entry("model_id", &self.model_id)?;
        if let Some(v) = &self.system_prompt {
            map.serialize_entry("system_prompt", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        if let Some(v) = &self.user_instructions {
            map.serialize_entry("user_instructions", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ScoreCombination {
    pub parameters: Option<crate::types::search_pipeline_common::ScoreCombinationParameters>,
    pub technique: Option<crate::types::search_pipeline_common::ScoreCombinationTechnique>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ScoreCombination {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ScoreCombination;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ScoreCombination object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_parameters: Option<
                    crate::types::search_pipeline_common::ScoreCombinationParameters,
                > = None;
                let mut seen_parameters = false;
                let mut f_technique: Option<
                    crate::types::search_pipeline_common::ScoreCombinationTechnique,
                > = None;
                let mut seen_technique = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "parameters" => {
                            if seen_parameters {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            seen_parameters = true;
                            f_parameters = map.next_value::<Option<
                                crate::types::search_pipeline_common::ScoreCombinationParameters,
                            >>()?;
                        }
                        "technique" => {
                            if seen_technique {
                                return Err(serde::de::Error::duplicate_field("technique"));
                            }
                            seen_technique = true;
                            f_technique = map.next_value::<Option<
                                crate::types::search_pipeline_common::ScoreCombinationTechnique,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ScoreCombination {
                    parameters: f_parameters,
                    technique: f_technique,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ScoreCombination {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.parameters {
            map.serialize_entry("parameters", v)?;
        }
        if let Some(v) = &self.technique {
            map.serialize_entry("technique", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ScoreCombinationParameters {
    pub weights: Option<Vec<f64>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ScoreCombinationParameters {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ScoreCombinationParameters;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ScoreCombinationParameters object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_weights: Option<Vec<f64>> = None;
                let mut seen_weights = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "weights" => {
                            if seen_weights {
                                return Err(serde::de::Error::duplicate_field("weights"));
                            }
                            seen_weights = true;
                            f_weights = map.next_value::<Option<Vec<f64>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ScoreCombinationParameters {
                    weights: f_weights,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ScoreCombinationParameters {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.weights {
            map.serialize_entry("weights", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScoreCombinationTechnique {
    #[doc = "`arithmetic_mean`"]
    ArithmeticMean,
    #[doc = "`geometric_mean`"]
    GeometricMean,
    #[doc = "`harmonic_mean`"]
    HarmonicMean,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ScoreCombinationTechnique {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ScoreCombinationTechnique::ArithmeticMean => "arithmetic_mean",
            ScoreCombinationTechnique::GeometricMean => "geometric_mean",
            ScoreCombinationTechnique::HarmonicMean => "harmonic_mean",
            ScoreCombinationTechnique::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ScoreCombinationTechnique {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScoreCombinationTechnique {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "arithmetic_mean" => ScoreCombinationTechnique::ArithmeticMean,
            "geometric_mean" => ScoreCombinationTechnique::GeometricMean,
            "harmonic_mean" => ScoreCombinationTechnique::HarmonicMean,
            _ => ScoreCombinationTechnique::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ScoreNormalization {
    pub technique: Option<crate::types::search_pipeline_common::ScoreNormalizationTechnique>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ScoreNormalization {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ScoreNormalization;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ScoreNormalization object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_technique: Option<
                    crate::types::search_pipeline_common::ScoreNormalizationTechnique,
                > = None;
                let mut seen_technique = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "technique" => {
                            if seen_technique {
                                return Err(serde::de::Error::duplicate_field("technique"));
                            }
                            seen_technique = true;
                            f_technique = map.next_value::<Option<
                                crate::types::search_pipeline_common::ScoreNormalizationTechnique,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ScoreNormalization {
                    technique: f_technique,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ScoreNormalization {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.technique {
            map.serialize_entry("technique", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScoreNormalizationTechnique {
    #[doc = "`l2`"]
    L2,
    #[doc = "`min_max`"]
    MinMax,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ScoreNormalizationTechnique {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ScoreNormalizationTechnique::L2 => "l2",
            ScoreNormalizationTechnique::MinMax => "min_max",
            ScoreNormalizationTechnique::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ScoreNormalizationTechnique {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScoreNormalizationTechnique {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "l2" => ScoreNormalizationTechnique::L2,
            "min_max" => ScoreNormalizationTechnique::MinMax,
            _ => ScoreNormalizationTechnique::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ScoreRankerCombination {
    pub rank_constant: Option<i32>,
    pub technique: crate::types::search_pipeline_common::ScoreRankerCombinationTechnique,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ScoreRankerCombination {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ScoreRankerCombination;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ScoreRankerCombination object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_rank_constant: Option<i32> = None;
                let mut seen_rank_constant = false;
                let mut f_technique: Option<
                    crate::types::search_pipeline_common::ScoreRankerCombinationTechnique,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "rank_constant" => {
                            if seen_rank_constant {
                                return Err(serde::de::Error::duplicate_field("rank_constant"));
                            }
                            seen_rank_constant = true;
                            f_rank_constant = map.next_value::<Option<i32>>()?;
                        }
                        "technique" => {
                            if f_technique.is_some() {
                                return Err(serde::de::Error::duplicate_field("technique"));
                            }
                            f_technique = Some(map.next_value::<crate::types::search_pipeline_common::ScoreRankerCombinationTechnique>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ScoreRankerCombination {
                    rank_constant: f_rank_constant,
                    technique: f_technique
                        .ok_or_else(|| serde::de::Error::missing_field("technique"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ScoreRankerCombination {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.rank_constant {
            map.serialize_entry("rank_constant", v)?;
        }
        map.serialize_entry("technique", &self.technique)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScoreRankerCombinationTechnique {
    #[doc = "`rrf`"]
    Rrf,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ScoreRankerCombinationTechnique {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ScoreRankerCombinationTechnique::Rrf => "rrf",
            ScoreRankerCombinationTechnique::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ScoreRankerCombinationTechnique {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScoreRankerCombinationTechnique {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "rrf" => ScoreRankerCombinationTechnique::Rrf,
            _ => ScoreRankerCombinationTechnique::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ScoreRankerPhaseResultsProcessor {
    pub combination: crate::types::search_pipeline_common::ScoreRankerCombination,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ScoreRankerPhaseResultsProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ScoreRankerPhaseResultsProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ScoreRankerPhaseResultsProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_combination: Option<
                    crate::types::search_pipeline_common::ScoreRankerCombination,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "combination" => {
                            if f_combination.is_some() {
                                return Err(serde::de::Error::duplicate_field("combination"));
                            }
                            f_combination = Some(map.next_value::<crate::types::search_pipeline_common::ScoreRankerCombination>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ScoreRankerPhaseResultsProcessor {
                    combination: f_combination
                        .ok_or_else(|| serde::de::Error::missing_field("combination"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ScoreRankerPhaseResultsProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("combination", &self.combination)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type SearchPipelineMap = std::collections::BTreeMap<
    String,
    crate::types::search_pipeline_common::SearchPipelineStructure,
>;

#[derive(Debug, Clone)]
pub struct SearchPipelineStructure {
    pub description: Option<String>,
    pub phase_results_processors:
        Option<Vec<crate::types::search_pipeline_common::PhaseResultsProcessor>>,
    pub request_processors: Option<Vec<crate::types::search_pipeline_common::RequestProcessor>>,
    pub response_processors: Option<Vec<crate::types::search_pipeline_common::ResponseProcessor>>,
    pub version: Option<i32>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SearchPipelineStructure {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SearchPipelineStructure;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchPipelineStructure object")
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
                let mut seen_description = false;
                let mut f_phase_results_processors: Option<
                    Vec<crate::types::search_pipeline_common::PhaseResultsProcessor>,
                > = None;
                let mut seen_phase_results_processors = false;
                let mut f_request_processors: Option<
                    Vec<crate::types::search_pipeline_common::RequestProcessor>,
                > = None;
                let mut seen_request_processors = false;
                let mut f_response_processors: Option<
                    Vec<crate::types::search_pipeline_common::ResponseProcessor>,
                > = None;
                let mut seen_response_processors = false;
                let mut f_version: Option<i32> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "phase_results_processors" => {
                            if seen_phase_results_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "phase_results_processors",
                                ));
                            }
                            seen_phase_results_processors = true;
                            f_phase_results_processors = map.next_value::<Option<
                                Vec<crate::types::search_pipeline_common::PhaseResultsProcessor>,
                            >>()?;
                        }
                        "request_processors" => {
                            if seen_request_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "request_processors",
                                ));
                            }
                            seen_request_processors = true;
                            f_request_processors = map.next_value::<Option<
                                Vec<crate::types::search_pipeline_common::RequestProcessor>,
                            >>()?;
                        }
                        "response_processors" => {
                            if seen_response_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "response_processors",
                                ));
                            }
                            seen_response_processors = true;
                            f_response_processors = map.next_value::<Option<
                                Vec<crate::types::search_pipeline_common::ResponseProcessor>,
                            >>()?;
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version = map.next_value::<Option<i32>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SearchPipelineStructure {
                    description: f_description,
                    phase_results_processors: f_phase_results_processors,
                    request_processors: f_request_processors,
                    response_processors: f_response_processors,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SearchPipelineStructure {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.phase_results_processors {
            map.serialize_entry("phase_results_processors", v)?;
        }
        if let Some(v) = &self.request_processors {
            map.serialize_entry("request_processors", v)?;
        }
        if let Some(v) = &self.response_processors {
            map.serialize_entry("response_processors", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SearchScriptRequestProcessor {
    pub description: Option<String>,
    pub ignore_failure: Option<bool>,
    pub lang: Option<String>,
    pub source: String,
    pub tag: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SearchScriptRequestProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SearchScriptRequestProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchScriptRequestProcessor object")
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
                let mut seen_description = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_lang: Option<String> = None;
                let mut seen_lang = false;
                let mut f_source: Option<String> = None;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "lang" => {
                            if seen_lang {
                                return Err(serde::de::Error::duplicate_field("lang"));
                            }
                            seen_lang = true;
                            f_lang = map.next_value::<Option<String>>()?;
                        }
                        "source" => {
                            if f_source.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            f_source = Some(map.next_value::<String>()?);
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
                Ok(SearchScriptRequestProcessor {
                    description: f_description,
                    ignore_failure: f_ignore_failure,
                    lang: f_lang,
                    source: f_source.ok_or_else(|| serde::de::Error::missing_field("source"))?,
                    tag: f_tag,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SearchScriptRequestProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.lang {
            map.serialize_entry("lang", v)?;
        }
        map.serialize_entry("source", &self.source)?;
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SortResponseProcessor {
    pub description: Option<String>,
    pub field: String,
    pub ignore_failure: Option<bool>,
    pub order: Option<String>,
    pub tag: Option<String>,
    pub target_field: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SortResponseProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SortResponseProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SortResponseProcessor object")
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
                let mut seen_description = false;
                let mut f_field: Option<String> = None;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_order: Option<String> = None;
                let mut seen_order = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_target_field: Option<String> = None;
                let mut seen_target_field = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<String>()?);
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "order" => {
                            if seen_order {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            seen_order = true;
                            f_order = map.next_value::<Option<String>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
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
                Ok(SortResponseProcessor {
                    description: f_description,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    ignore_failure: f_ignore_failure,
                    order: f_order,
                    tag: f_tag,
                    target_field: f_target_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SortResponseProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.order {
            map.serialize_entry("order", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
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
pub struct SplitResponseProcessor {
    pub description: Option<String>,
    pub field: String,
    pub ignore_failure: Option<bool>,
    pub preserve_trailing: Option<bool>,
    pub separator: String,
    pub tag: Option<String>,
    pub target_field: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SplitResponseProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SplitResponseProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SplitResponseProcessor object")
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
                let mut seen_description = false;
                let mut f_field: Option<String> = None;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_preserve_trailing: Option<bool> = None;
                let mut seen_preserve_trailing = false;
                let mut f_separator: Option<String> = None;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_target_field: Option<String> = None;
                let mut seen_target_field = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "field" => {
                            if f_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            f_field = Some(map.next_value::<String>()?);
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "preserve_trailing" => {
                            if seen_preserve_trailing {
                                return Err(serde::de::Error::duplicate_field("preserve_trailing"));
                            }
                            seen_preserve_trailing = true;
                            f_preserve_trailing = map.next_value::<Option<bool>>()?;
                        }
                        "separator" => {
                            if f_separator.is_some() {
                                return Err(serde::de::Error::duplicate_field("separator"));
                            }
                            f_separator = Some(map.next_value::<String>()?);
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
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
                Ok(SplitResponseProcessor {
                    description: f_description,
                    field: f_field.ok_or_else(|| serde::de::Error::missing_field("field"))?,
                    ignore_failure: f_ignore_failure,
                    preserve_trailing: f_preserve_trailing,
                    separator: f_separator
                        .ok_or_else(|| serde::de::Error::missing_field("separator"))?,
                    tag: f_tag,
                    target_field: f_target_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SplitResponseProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        map.serialize_entry("field", &self.field)?;
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.preserve_trailing {
            map.serialize_entry("preserve_trailing", v)?;
        }
        map.serialize_entry("separator", &self.separator)?;
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
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
pub struct TruncateHitsResponseProcessor {
    pub context_prefix: Option<String>,
    pub description: Option<String>,
    pub ignore_failure: Option<bool>,
    pub tag: Option<String>,
    pub target_size: Option<i32>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TruncateHitsResponseProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TruncateHitsResponseProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TruncateHitsResponseProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_context_prefix: Option<String> = None;
                let mut seen_context_prefix = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_ignore_failure: Option<bool> = None;
                let mut seen_ignore_failure = false;
                let mut f_tag: Option<String> = None;
                let mut seen_tag = false;
                let mut f_target_size: Option<i32> = None;
                let mut seen_target_size = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "context_prefix" => {
                            if seen_context_prefix {
                                return Err(serde::de::Error::duplicate_field("context_prefix"));
                            }
                            seen_context_prefix = true;
                            f_context_prefix = map.next_value::<Option<String>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "ignore_failure" => {
                            if seen_ignore_failure {
                                return Err(serde::de::Error::duplicate_field("ignore_failure"));
                            }
                            seen_ignore_failure = true;
                            f_ignore_failure = map.next_value::<Option<bool>>()?;
                        }
                        "tag" => {
                            if seen_tag {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            seen_tag = true;
                            f_tag = map.next_value::<Option<String>>()?;
                        }
                        "target_size" => {
                            if seen_target_size {
                                return Err(serde::de::Error::duplicate_field("target_size"));
                            }
                            seen_target_size = true;
                            f_target_size = map.next_value::<Option<i32>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TruncateHitsResponseProcessor {
                    context_prefix: f_context_prefix,
                    description: f_description,
                    ignore_failure: f_ignore_failure,
                    tag: f_tag,
                    target_size: f_target_size,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TruncateHitsResponseProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.context_prefix {
            map.serialize_entry("context_prefix", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.ignore_failure {
            map.serialize_entry("ignore_failure", v)?;
        }
        if let Some(v) = &self.tag {
            map.serialize_entry("tag", v)?;
        }
        if let Some(v) = &self.target_size {
            map.serialize_entry("target_size", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
