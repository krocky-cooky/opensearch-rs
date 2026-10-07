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

//! Response types of the `ml_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct Action {
    pub action_type: Option<String>,
    pub headers: Option<crate::types::ml_common::Headers>,
    pub method: Option<String>,
    pub post_process_function: Option<String>,
    pub pre_process_function: Option<String>,
    pub request_body: Option<String>,
    pub url: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Action;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Action object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_action_type: Option<String> = None;
                let mut seen_action_type = false;
                let mut f_headers: Option<crate::types::ml_common::Headers> = None;
                let mut seen_headers = false;
                let mut f_method: Option<String> = None;
                let mut seen_method = false;
                let mut f_post_process_function: Option<String> = None;
                let mut seen_post_process_function = false;
                let mut f_pre_process_function: Option<String> = None;
                let mut seen_pre_process_function = false;
                let mut f_request_body: Option<String> = None;
                let mut seen_request_body = false;
                let mut f_url: Option<String> = None;
                let mut seen_url = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "action_type" => {
                            if seen_action_type {
                                return Err(serde::de::Error::duplicate_field("action_type"));
                            }
                            seen_action_type = true;
                            f_action_type = map.next_value::<Option<String>>()?;
                        }
                        "headers" => {
                            if seen_headers {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            seen_headers = true;
                            f_headers =
                                map.next_value::<Option<crate::types::ml_common::Headers>>()?;
                        }
                        "method" => {
                            if seen_method {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            seen_method = true;
                            f_method = map.next_value::<Option<String>>()?;
                        }
                        "post_process_function" => {
                            if seen_post_process_function {
                                return Err(serde::de::Error::duplicate_field(
                                    "post_process_function",
                                ));
                            }
                            seen_post_process_function = true;
                            f_post_process_function = map.next_value::<Option<String>>()?;
                        }
                        "pre_process_function" => {
                            if seen_pre_process_function {
                                return Err(serde::de::Error::duplicate_field(
                                    "pre_process_function",
                                ));
                            }
                            seen_pre_process_function = true;
                            f_pre_process_function = map.next_value::<Option<String>>()?;
                        }
                        "request_body" => {
                            if seen_request_body {
                                return Err(serde::de::Error::duplicate_field("request_body"));
                            }
                            seen_request_body = true;
                            f_request_body = map.next_value::<Option<String>>()?;
                        }
                        "url" => {
                            if seen_url {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            seen_url = true;
                            f_url = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Action {
                    action_type: f_action_type,
                    headers: f_headers,
                    method: f_method,
                    post_process_function: f_post_process_function,
                    pre_process_function: f_pre_process_function,
                    request_body: f_request_body,
                    url: f_url,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Action {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.action_type {
            map.serialize_entry("action_type", v)?;
        }
        if let Some(v) = &self.headers {
            map.serialize_entry("headers", v)?;
        }
        if let Some(v) = &self.method {
            map.serialize_entry("method", v)?;
        }
        if let Some(v) = &self.post_process_function {
            map.serialize_entry("post_process_function", v)?;
        }
        if let Some(v) = &self.pre_process_function {
            map.serialize_entry("pre_process_function", v)?;
        }
        if let Some(v) = &self.request_body {
            map.serialize_entry("request_body", v)?;
        }
        if let Some(v) = &self.url {
            map.serialize_entry("url", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The type of operation."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActionType {
    #[doc = "`ADD`"]
    Add,
    #[doc = "`DELETE`"]
    Delete,
    #[doc = "`UPDATE`"]
    Update,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ActionType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ActionType::Add => "ADD",
            ActionType::Delete => "DELETE",
            ActionType::Update => "UPDATE",
            ActionType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ActionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ActionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ADD" => ActionType::Add,
            "DELETE" => ActionType::Delete,
            "UPDATE" => ActionType::Update,
            _ => ActionType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct AdditionalConfig {
    #[doc = "The distance metric for k-NN search."]
    pub space_type: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AdditionalConfig {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AdditionalConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AdditionalConfig object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_space_type: Option<String> = None;
                let mut seen_space_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
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
                Ok(AdditionalConfig {
                    space_type: f_space_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AdditionalConfig {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.space_type {
            map.serialize_entry("space_type", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Additional information query."]
pub type AdditionalInfo = std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

#[doc = "The agent type."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentType {
    #[doc = "`conversational`"]
    Conversational,
    #[doc = "`conversational_flow`"]
    ConversationalFlow,
    #[doc = "`flow`"]
    Flow,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl AgentType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            AgentType::Conversational => "conversational",
            AgentType::ConversationalFlow => "conversational_flow",
            AgentType::Flow => "flow",
            AgentType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for AgentType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AgentType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "conversational" => AgentType::Conversational,
            "conversational_flow" => AgentType::ConversationalFlow,
            "flow" => AgentType::Flow,
            _ => AgentType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct AlgorithmOperations {
    pub deploy: Option<crate::types::ml_common::ModelStats>,
    pub execute: Option<crate::types::ml_common::ModelStats>,
    pub predict: Option<crate::types::ml_common::ModelStats>,
    pub register: Option<crate::types::ml_common::ModelStats>,
    pub train: Option<crate::types::ml_common::ModelStats>,
    pub train_predict: Option<crate::types::ml_common::ModelStats>,
    pub undeploy: Option<crate::types::ml_common::ModelStats>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AlgorithmOperations {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AlgorithmOperations;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AlgorithmOperations object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_deploy: Option<crate::types::ml_common::ModelStats> = None;
                let mut seen_deploy = false;
                let mut f_execute: Option<crate::types::ml_common::ModelStats> = None;
                let mut seen_execute = false;
                let mut f_predict: Option<crate::types::ml_common::ModelStats> = None;
                let mut seen_predict = false;
                let mut f_register: Option<crate::types::ml_common::ModelStats> = None;
                let mut seen_register = false;
                let mut f_train: Option<crate::types::ml_common::ModelStats> = None;
                let mut seen_train = false;
                let mut f_train_predict: Option<crate::types::ml_common::ModelStats> = None;
                let mut seen_train_predict = false;
                let mut f_undeploy: Option<crate::types::ml_common::ModelStats> = None;
                let mut seen_undeploy = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "deploy" => {
                            if seen_deploy {
                                return Err(serde::de::Error::duplicate_field("deploy"));
                            }
                            seen_deploy = true;
                            f_deploy =
                                map.next_value::<Option<crate::types::ml_common::ModelStats>>()?;
                        }
                        "execute" => {
                            if seen_execute {
                                return Err(serde::de::Error::duplicate_field("execute"));
                            }
                            seen_execute = true;
                            f_execute =
                                map.next_value::<Option<crate::types::ml_common::ModelStats>>()?;
                        }
                        "predict" => {
                            if seen_predict {
                                return Err(serde::de::Error::duplicate_field("predict"));
                            }
                            seen_predict = true;
                            f_predict =
                                map.next_value::<Option<crate::types::ml_common::ModelStats>>()?;
                        }
                        "register" => {
                            if seen_register {
                                return Err(serde::de::Error::duplicate_field("register"));
                            }
                            seen_register = true;
                            f_register =
                                map.next_value::<Option<crate::types::ml_common::ModelStats>>()?;
                        }
                        "train" => {
                            if seen_train {
                                return Err(serde::de::Error::duplicate_field("train"));
                            }
                            seen_train = true;
                            f_train =
                                map.next_value::<Option<crate::types::ml_common::ModelStats>>()?;
                        }
                        "train_predict" => {
                            if seen_train_predict {
                                return Err(serde::de::Error::duplicate_field("train_predict"));
                            }
                            seen_train_predict = true;
                            f_train_predict =
                                map.next_value::<Option<crate::types::ml_common::ModelStats>>()?;
                        }
                        "undeploy" => {
                            if seen_undeploy {
                                return Err(serde::de::Error::duplicate_field("undeploy"));
                            }
                            seen_undeploy = true;
                            f_undeploy =
                                map.next_value::<Option<crate::types::ml_common::ModelStats>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AlgorithmOperations {
                    deploy: f_deploy,
                    execute: f_execute,
                    predict: f_predict,
                    register: f_register,
                    train: f_train,
                    train_predict: f_train_predict,
                    undeploy: f_undeploy,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AlgorithmOperations {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.deploy {
            map.serialize_entry("deploy", v)?;
        }
        if let Some(v) = &self.execute {
            map.serialize_entry("execute", v)?;
        }
        if let Some(v) = &self.predict {
            map.serialize_entry("predict", v)?;
        }
        if let Some(v) = &self.register {
            map.serialize_entry("register", v)?;
        }
        if let Some(v) = &self.train {
            map.serialize_entry("train", v)?;
        }
        if let Some(v) = &self.train_predict {
            map.serialize_entry("train_predict", v)?;
        }
        if let Some(v) = &self.undeploy {
            map.serialize_entry("undeploy", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type Algorithms =
    std::collections::BTreeMap<String, crate::types::ml_common::AlgorithmOperations>;

#[derive(Debug, Clone)]
pub struct Buckets {
    #[doc = "The end time."]
    pub end_time: Option<i64>,
    pub entities: Option<Vec<crate::types::ml_common::Entity>>,
    #[doc = "The overall aggregate value."]
    pub overall_aggregate_value: Option<f64>,
    #[doc = "The start time."]
    pub start_time: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Buckets {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Buckets;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Buckets object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_end_time: Option<i64> = None;
                let mut seen_end_time = false;
                let mut f_entities: Option<Vec<crate::types::ml_common::Entity>> = None;
                let mut seen_entities = false;
                let mut f_overall_aggregate_value: Option<f64> = None;
                let mut seen_overall_aggregate_value = false;
                let mut f_start_time: Option<i64> = None;
                let mut seen_start_time = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "end_time" => {
                            if seen_end_time {
                                return Err(serde::de::Error::duplicate_field("end_time"));
                            }
                            seen_end_time = true;
                            f_end_time = map.next_value::<Option<i64>>()?;
                        }
                        "entities" => {
                            if seen_entities {
                                return Err(serde::de::Error::duplicate_field("entities"));
                            }
                            seen_entities = true;
                            f_entities =
                                map.next_value::<Option<Vec<crate::types::ml_common::Entity>>>()?;
                        }
                        "overall_aggregate_value" => {
                            if seen_overall_aggregate_value {
                                return Err(serde::de::Error::duplicate_field(
                                    "overall_aggregate_value",
                                ));
                            }
                            seen_overall_aggregate_value = true;
                            f_overall_aggregate_value = map.next_value::<Option<f64>>()?;
                        }
                        "start_time" => {
                            if seen_start_time {
                                return Err(serde::de::Error::duplicate_field("start_time"));
                            }
                            seen_start_time = true;
                            f_start_time = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Buckets {
                    end_time: f_end_time,
                    entities: f_entities,
                    overall_aggregate_value: f_overall_aggregate_value,
                    start_time: f_start_time,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Buckets {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.end_time {
            map.serialize_entry("end_time", v)?;
        }
        if let Some(v) = &self.entities {
            map.serialize_entry("entities", v)?;
        }
        if let Some(v) = &self.overall_aggregate_value {
            map.serialize_entry("overall_aggregate_value", v)?;
        }
        if let Some(v) = &self.start_time {
            map.serialize_entry("start_time", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ByteBuffer {
    #[doc = "The byte buffer array."]
    pub array: Option<String>,
    #[doc = "The byte buffer order."]
    pub order: Option<crate::types::ml_common::ByteOrder>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ByteBuffer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ByteBuffer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ByteBuffer object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_array: Option<String> = None;
                let mut seen_array = false;
                let mut f_order: Option<crate::types::ml_common::ByteOrder> = None;
                let mut seen_order = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "array" => {
                            if seen_array {
                                return Err(serde::de::Error::duplicate_field("array"));
                            }
                            seen_array = true;
                            f_array = map.next_value::<Option<String>>()?;
                        }
                        "order" => {
                            if seen_order {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            seen_order = true;
                            f_order =
                                map.next_value::<Option<crate::types::ml_common::ByteOrder>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ByteBuffer {
                    array: f_array,
                    order: f_order,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ByteBuffer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.array {
            map.serialize_entry("array", v)?;
        }
        if let Some(v) = &self.order {
            map.serialize_entry("order", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ByteOrder {
    #[doc = "`BIG_ENDIAN`"]
    BigEndian,
    #[doc = "`LITTLE_ENDIAN`"]
    LittleEndian,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ByteOrder {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ByteOrder::BigEndian => "BIG_ENDIAN",
            ByteOrder::LittleEndian => "LITTLE_ENDIAN",
            ByteOrder::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ByteOrder {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ByteOrder {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "BIG_ENDIAN" => ByteOrder::BigEndian,
            "LITTLE_ENDIAN" => ByteOrder::LittleEndian,
            _ => ByteOrder::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ColumnMeta {
    pub column_type: Option<crate::types::ml_common::ColumnType>,
    pub name: Option<crate::types::common::Name>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ColumnMeta {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ColumnMeta;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ColumnMeta object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_column_type: Option<crate::types::ml_common::ColumnType> = None;
                let mut seen_column_type = false;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut seen_name = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "column_type" => {
                            if seen_column_type {
                                return Err(serde::de::Error::duplicate_field("column_type"));
                            }
                            seen_column_type = true;
                            f_column_type =
                                map.next_value::<Option<crate::types::ml_common::ColumnType>>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ColumnMeta {
                    column_type: f_column_type,
                    name: f_name,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ColumnMeta {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.column_type {
            map.serialize_entry("column_type", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The column type."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ColumnType {
    #[doc = "`BOOLEAN`"]
    Boolean,
    #[doc = "`DOUBLE`"]
    Double,
    #[doc = "`INTEGER`"]
    Integer,
    #[doc = "`STRING`"]
    String,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ColumnType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ColumnType::Boolean => "BOOLEAN",
            ColumnType::Double => "DOUBLE",
            ColumnType::Integer => "INTEGER",
            ColumnType::String => "STRING",
            ColumnType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ColumnType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ColumnType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "BOOLEAN" => ColumnType::Boolean,
            "DOUBLE" => ColumnType::Double,
            "INTEGER" => ColumnType::Integer,
            "STRING" => ColumnType::String,
            _ => ColumnType::Unknown(s),
        })
    }
}

#[doc = "The connector protocol."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectorProtocol {
    #[doc = "`aws_sigv4`"]
    AwsSigv4,
    #[doc = "`http`"]
    Http,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ConnectorProtocol {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ConnectorProtocol::AwsSigv4 => "aws_sigv4",
            ConnectorProtocol::Http => "http",
            ConnectorProtocol::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ConnectorProtocol {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ConnectorProtocol {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "aws_sigv4" => ConnectorProtocol::AwsSigv4,
            "http" => ConnectorProtocol::Http,
            _ => ConnectorProtocol::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Content {
    pub text: Option<String>,
    pub r#type: Option<String>,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Content {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Content;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Content object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_text: Option<String> = None;
                let mut seen_text = false;
                let mut f_type: Option<String> = None;
                let mut seen_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "text" => {
                            if seen_text {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            seen_text = true;
                            f_text = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Content {
                    text: f_text,
                    r#type: f_type,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Content {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.text {
            map.serialize_entry("text", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("type", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct DataAsMap {
    #[doc = "The streaming content chunk."]
    pub content: Option<String>,
    #[doc = "Whether this is the last chunk."]
    pub is_last: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DataAsMap {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DataAsMap;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DataAsMap object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_content: Option<String> = None;
                let mut seen_content = false;
                let mut f_is_last: Option<bool> = None;
                let mut seen_is_last = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "content" => {
                            if seen_content {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            seen_content = true;
                            f_content = map.next_value::<Option<String>>()?;
                        }
                        "is_last" => {
                            if seen_is_last {
                                return Err(serde::de::Error::duplicate_field("is_last"));
                            }
                            seen_is_last = true;
                            f_is_last = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DataAsMap {
                    content: f_content,
                    is_last: f_is_last,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DataAsMap {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.content {
            map.serialize_entry("content", v)?;
        }
        if let Some(v) = &self.is_last {
            map.serialize_entry("is_last", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct DeleteAgenticMemoryResponse {
    #[doc = "Number of batches processed."]
    pub batches: Option<i64>,
    #[doc = "Number of documents created."]
    pub created: Option<i64>,
    #[doc = "Number of documents deleted."]
    pub deleted: Option<i64>,
    #[doc = "Any failures occurred during the operation."]
    pub failures: Option<Vec<crate::types::common::BulkByScrollFailure>>,
    #[doc = "Number of no-operation updates."]
    pub noops: Option<i64>,
    #[doc = "Number of requests processed per second."]
    pub requests_per_second: Option<f64>,
    #[doc = "The result of delete operation."]
    pub result: Option<String>,
    pub retries: Option<crate::types::common::Retries>,
    #[doc = "Time that the request was throttled."]
    pub throttled_millis: Option<i64>,
    #[doc = "Time until throttling is lifted."]
    pub throttled_until_millis: Option<i64>,
    #[doc = "Whether the request timed out."]
    pub timed_out: Option<bool>,
    #[doc = "Time taken to execute the request."]
    pub took: Option<i64>,
    #[doc = "Total number of documents processed."]
    pub total: Option<i64>,
    #[doc = "Number of documents updated."]
    pub updated: Option<i64>,
    #[doc = "Number of version conflicts encountered."]
    pub version_conflicts: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DeleteAgenticMemoryResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DeleteAgenticMemoryResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DeleteAgenticMemoryResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_batches: Option<i64> = None;
                let mut seen_batches = false;
                let mut f_created: Option<i64> = None;
                let mut seen_created = false;
                let mut f_deleted: Option<i64> = None;
                let mut seen_deleted = false;
                let mut f_failures: Option<Vec<crate::types::common::BulkByScrollFailure>> = None;
                let mut seen_failures = false;
                let mut f_noops: Option<i64> = None;
                let mut seen_noops = false;
                let mut f_requests_per_second: Option<f64> = None;
                let mut seen_requests_per_second = false;
                let mut f_result: Option<String> = None;
                let mut seen_result = false;
                let mut f_retries: Option<crate::types::common::Retries> = None;
                let mut seen_retries = false;
                let mut f_throttled_millis: Option<i64> = None;
                let mut seen_throttled_millis = false;
                let mut f_throttled_until_millis: Option<i64> = None;
                let mut seen_throttled_until_millis = false;
                let mut f_timed_out: Option<bool> = None;
                let mut seen_timed_out = false;
                let mut f_took: Option<i64> = None;
                let mut seen_took = false;
                let mut f_total: Option<i64> = None;
                let mut seen_total = false;
                let mut f_updated: Option<i64> = None;
                let mut seen_updated = false;
                let mut f_version_conflicts: Option<i64> = None;
                let mut seen_version_conflicts = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "batches" => {
                            if seen_batches {
                                return Err(serde::de::Error::duplicate_field("batches"));
                            }
                            seen_batches = true;
                            f_batches = map.next_value::<Option<i64>>()?;
                        }
                        "created" => {
                            if seen_created {
                                return Err(serde::de::Error::duplicate_field("created"));
                            }
                            seen_created = true;
                            f_created = map.next_value::<Option<i64>>()?;
                        }
                        "deleted" => {
                            if seen_deleted {
                                return Err(serde::de::Error::duplicate_field("deleted"));
                            }
                            seen_deleted = true;
                            f_deleted = map.next_value::<Option<i64>>()?;
                        }
                        "failures" => {
                            if seen_failures {
                                return Err(serde::de::Error::duplicate_field("failures"));
                            }
                            seen_failures = true;
                            f_failures = map.next_value::<Option<Vec<crate::types::common::BulkByScrollFailure>>>()?;
                        }
                        "noops" => {
                            if seen_noops {
                                return Err(serde::de::Error::duplicate_field("noops"));
                            }
                            seen_noops = true;
                            f_noops = map.next_value::<Option<i64>>()?;
                        }
                        "requests_per_second" => {
                            if seen_requests_per_second {
                                return Err(serde::de::Error::duplicate_field(
                                    "requests_per_second",
                                ));
                            }
                            seen_requests_per_second = true;
                            f_requests_per_second = map.next_value::<Option<f64>>()?;
                        }
                        "result" => {
                            if seen_result {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            seen_result = true;
                            f_result = map.next_value::<Option<String>>()?;
                        }
                        "retries" => {
                            if seen_retries {
                                return Err(serde::de::Error::duplicate_field("retries"));
                            }
                            seen_retries = true;
                            f_retries =
                                map.next_value::<Option<crate::types::common::Retries>>()?;
                        }
                        "throttled_millis" => {
                            if seen_throttled_millis {
                                return Err(serde::de::Error::duplicate_field("throttled_millis"));
                            }
                            seen_throttled_millis = true;
                            f_throttled_millis = map.next_value::<Option<i64>>()?;
                        }
                        "throttled_until_millis" => {
                            if seen_throttled_until_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "throttled_until_millis",
                                ));
                            }
                            seen_throttled_until_millis = true;
                            f_throttled_until_millis = map.next_value::<Option<i64>>()?;
                        }
                        "timed_out" => {
                            if seen_timed_out {
                                return Err(serde::de::Error::duplicate_field("timed_out"));
                            }
                            seen_timed_out = true;
                            f_timed_out = map.next_value::<Option<bool>>()?;
                        }
                        "took" => {
                            if seen_took {
                                return Err(serde::de::Error::duplicate_field("took"));
                            }
                            seen_took = true;
                            f_took = map.next_value::<Option<i64>>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map.next_value::<Option<i64>>()?;
                        }
                        "updated" => {
                            if seen_updated {
                                return Err(serde::de::Error::duplicate_field("updated"));
                            }
                            seen_updated = true;
                            f_updated = map.next_value::<Option<i64>>()?;
                        }
                        "version_conflicts" => {
                            if seen_version_conflicts {
                                return Err(serde::de::Error::duplicate_field("version_conflicts"));
                            }
                            seen_version_conflicts = true;
                            f_version_conflicts = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DeleteAgenticMemoryResponse {
                    batches: f_batches,
                    created: f_created,
                    deleted: f_deleted,
                    failures: f_failures,
                    noops: f_noops,
                    requests_per_second: f_requests_per_second,
                    result: f_result,
                    retries: f_retries,
                    throttled_millis: f_throttled_millis,
                    throttled_until_millis: f_throttled_until_millis,
                    timed_out: f_timed_out,
                    took: f_took,
                    total: f_total,
                    updated: f_updated,
                    version_conflicts: f_version_conflicts,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DeleteAgenticMemoryResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.batches {
            map.serialize_entry("batches", v)?;
        }
        if let Some(v) = &self.created {
            map.serialize_entry("created", v)?;
        }
        if let Some(v) = &self.deleted {
            map.serialize_entry("deleted", v)?;
        }
        if let Some(v) = &self.failures {
            map.serialize_entry("failures", v)?;
        }
        if let Some(v) = &self.noops {
            map.serialize_entry("noops", v)?;
        }
        if let Some(v) = &self.requests_per_second {
            map.serialize_entry("requests_per_second", v)?;
        }
        if let Some(v) = &self.result {
            map.serialize_entry("result", v)?;
        }
        if let Some(v) = &self.retries {
            map.serialize_entry("retries", v)?;
        }
        if let Some(v) = &self.throttled_millis {
            map.serialize_entry("throttled_millis", v)?;
        }
        if let Some(v) = &self.throttled_until_millis {
            map.serialize_entry("throttled_until_millis", v)?;
        }
        if let Some(v) = &self.timed_out {
            map.serialize_entry("timed_out", v)?;
        }
        if let Some(v) = &self.took {
            map.serialize_entry("took", v)?;
        }
        if let Some(v) = &self.total {
            map.serialize_entry("total", v)?;
        }
        if let Some(v) = &self.updated {
            map.serialize_entry("updated", v)?;
        }
        if let Some(v) = &self.version_conflicts {
            map.serialize_entry("version_conflicts", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Entity {
    #[doc = "The base value."]
    pub base_value: Option<f64>,
    #[doc = "The contribution value."]
    pub contribution_value: Option<f64>,
    pub key: Option<Vec<String>>,
    #[doc = "The new value."]
    pub new_value: Option<f64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Entity {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Entity;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Entity object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_base_value: Option<f64> = None;
                let mut seen_base_value = false;
                let mut f_contribution_value: Option<f64> = None;
                let mut seen_contribution_value = false;
                let mut f_key: Option<Vec<String>> = None;
                let mut seen_key = false;
                let mut f_new_value: Option<f64> = None;
                let mut seen_new_value = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "base_value" => {
                            if seen_base_value {
                                return Err(serde::de::Error::duplicate_field("base_value"));
                            }
                            seen_base_value = true;
                            f_base_value = map.next_value::<Option<f64>>()?;
                        }
                        "contribution_value" => {
                            if seen_contribution_value {
                                return Err(serde::de::Error::duplicate_field(
                                    "contribution_value",
                                ));
                            }
                            seen_contribution_value = true;
                            f_contribution_value = map.next_value::<Option<f64>>()?;
                        }
                        "key" => {
                            if seen_key {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            seen_key = true;
                            f_key = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "new_value" => {
                            if seen_new_value {
                                return Err(serde::de::Error::duplicate_field("new_value"));
                            }
                            seen_new_value = true;
                            f_new_value = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Entity {
                    base_value: f_base_value,
                    contribution_value: f_contribution_value,
                    key: f_key,
                    new_value: f_new_value,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Entity {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.base_value {
            map.serialize_entry("base_value", v)?;
        }
        if let Some(v) = &self.contribution_value {
            map.serialize_entry("contribution_value", v)?;
        }
        if let Some(v) = &self.key {
            map.serialize_entry("key", v)?;
        }
        if let Some(v) = &self.new_value {
            map.serialize_entry("new_value", v)?;
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
pub struct ExecuteAlgorithmResponse {
    raw: Box<serde_json::value::RawValue>,
}
impl ExecuteAlgorithmResponse {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `execute_local_sample_calculator_response` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_execute_local_sample_calculator_response(
        &self,
    ) -> std::result::Result<
        crate::types::ml_common::ExecuteLocalSampleCalculatorResponse,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "execute_local_sample_calculator_response")
    }
    #[doc = "Parses the value as the `object` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_object(
        &self,
    ) -> std::result::Result<
        crate::types::ml_common::ExecuteAlgorithmResponseObject,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "object")
    }
}

#[derive(Debug, Clone)]
pub struct ExecuteAlgorithmResponseObject {
    pub results: Option<Vec<crate::types::ml_common::ExecuteAnomalyLocalizationResponse>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ExecuteAlgorithmResponseObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ExecuteAlgorithmResponseObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ExecuteAlgorithmResponseObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_results: Option<
                    Vec<crate::types::ml_common::ExecuteAnomalyLocalizationResponse>,
                > = None;
                let mut seen_results = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "results" => {
                            if seen_results {
                                return Err(serde::de::Error::duplicate_field("results"));
                            }
                            seen_results = true;
                            f_results = map.next_value::<Option<
                                Vec<crate::types::ml_common::ExecuteAnomalyLocalizationResponse>,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ExecuteAlgorithmResponseObject {
                    results: f_results,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ExecuteAlgorithmResponseObject {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.results {
            map.serialize_entry("results", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ExecuteAnomalyLocalizationResponse {
    pub name: Option<String>,
    pub result: Option<crate::types::ml_common::Result>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ExecuteAnomalyLocalizationResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ExecuteAnomalyLocalizationResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ExecuteAnomalyLocalizationResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_result: Option<crate::types::ml_common::Result> = None;
                let mut seen_result = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "result" => {
                            if seen_result {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            seen_result = true;
                            f_result =
                                map.next_value::<Option<crate::types::ml_common::Result>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ExecuteAnomalyLocalizationResponse {
                    name: f_name,
                    result: f_result,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ExecuteAnomalyLocalizationResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.result {
            map.serialize_entry("result", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ExecuteLocalSampleCalculatorResponse {
    #[doc = "The result."]
    pub result: Option<f64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ExecuteLocalSampleCalculatorResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ExecuteLocalSampleCalculatorResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ExecuteLocalSampleCalculatorResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_result: Option<f64> = None;
                let mut seen_result = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "result" => {
                            if seen_result {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            seen_result = true;
                            f_result = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ExecuteLocalSampleCalculatorResponse {
                    result: f_result,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ExecuteLocalSampleCalculatorResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.result {
            map.serialize_entry("result", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ExecuteToolResponse {
    pub inference_results: Option<Vec<crate::types::ml_common::InferenceResults>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ExecuteToolResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ExecuteToolResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ExecuteToolResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_inference_results: Option<
                    Vec<crate::types::ml_common::InferenceResults>,
                > = None;
                let mut seen_inference_results = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "inference_results" => {
                            if seen_inference_results {
                                return Err(serde::de::Error::duplicate_field("inference_results"));
                            }
                            seen_inference_results = true;
                            f_inference_results = map.next_value::<Option<Vec<crate::types::ml_common::InferenceResults>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ExecuteToolResponse {
                    inference_results: f_inference_results,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ExecuteToolResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.inference_results {
            map.serialize_entry("inference_results", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The function name."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FunctionName {
    #[doc = "`AD_LIBSVM` (also accepted: `ad_libsvm`)"]
    AdLibsvm,
    #[doc = "`AGENT` (also accepted: `agent`)"]
    Agent,
    #[doc = "`ANOMALY_LOCALIZATION` (also accepted: `anomaly_localization`)"]
    AnomalyLocalization,
    #[doc = "`BATCH_RCF` (also accepted: `batch_rcf`)"]
    BatchRcf,
    #[doc = "`CONNECTOR` (also accepted: `connector`)"]
    Connector,
    #[doc = "`FIT_RCF` (also accepted: `fit_rcf`)"]
    FitRcf,
    #[doc = "`KMEANS` (also accepted: `kmeans`)"]
    Kmeans,
    #[doc = "`LINEAR_REGRESSION` (also accepted: `linear_regression`)"]
    LinearRegression,
    #[doc = "`LOCAL_SAMPLE_CALCULATOR` (also accepted: `local_sample_calculator`)"]
    LocalSampleCalculator,
    #[doc = "`LOGISTIC_REGRESSION` (also accepted: `logistic_regression`)"]
    LogisticRegression,
    #[doc = "`METRICS_CORRELATION` (also accepted: `metrics_correlation`)"]
    MetricsCorrelation,
    #[doc = "`QUESTION_ANSWERING` (also accepted: `question_answering`)"]
    QuestionAnswering,
    #[doc = "`RCF_SUMMARIZE` (also accepted: `rcf_summarize`)"]
    RcfSummarize,
    #[doc = "`REMOTE` (also accepted: `remote`)"]
    Remote,
    #[doc = "`SAMPLE_ALGO` (also accepted: `sample_algo`)"]
    SampleAlgo,
    #[doc = "`SPARSE_ENCODING` (also accepted: `sparse_encoding`)"]
    SparseEncoding,
    #[doc = "`SPARSE_TOKENIZE` (also accepted: `sparse_tokenize`)"]
    SparseTokenize,
    #[doc = "`TEXT_EMBEDDING` (also accepted: `text_embedding`)"]
    TextEmbedding,
    #[doc = "`TEXT_SIMILARITY` (also accepted: `text_similarity`)"]
    TextSimilarity,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl FunctionName {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            FunctionName::AdLibsvm => "AD_LIBSVM",
            FunctionName::Agent => "AGENT",
            FunctionName::AnomalyLocalization => "ANOMALY_LOCALIZATION",
            FunctionName::BatchRcf => "BATCH_RCF",
            FunctionName::Connector => "CONNECTOR",
            FunctionName::FitRcf => "FIT_RCF",
            FunctionName::Kmeans => "KMEANS",
            FunctionName::LinearRegression => "LINEAR_REGRESSION",
            FunctionName::LocalSampleCalculator => "LOCAL_SAMPLE_CALCULATOR",
            FunctionName::LogisticRegression => "LOGISTIC_REGRESSION",
            FunctionName::MetricsCorrelation => "METRICS_CORRELATION",
            FunctionName::QuestionAnswering => "QUESTION_ANSWERING",
            FunctionName::RcfSummarize => "RCF_SUMMARIZE",
            FunctionName::Remote => "REMOTE",
            FunctionName::SampleAlgo => "SAMPLE_ALGO",
            FunctionName::SparseEncoding => "SPARSE_ENCODING",
            FunctionName::SparseTokenize => "SPARSE_TOKENIZE",
            FunctionName::TextEmbedding => "TEXT_EMBEDDING",
            FunctionName::TextSimilarity => "TEXT_SIMILARITY",
            FunctionName::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for FunctionName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FunctionName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "AD_LIBSVM" | "ad_libsvm" => FunctionName::AdLibsvm,
            "AGENT" | "agent" => FunctionName::Agent,
            "ANOMALY_LOCALIZATION" | "anomaly_localization" => FunctionName::AnomalyLocalization,
            "BATCH_RCF" | "batch_rcf" => FunctionName::BatchRcf,
            "CONNECTOR" | "connector" => FunctionName::Connector,
            "FIT_RCF" | "fit_rcf" => FunctionName::FitRcf,
            "KMEANS" | "kmeans" => FunctionName::Kmeans,
            "LINEAR_REGRESSION" | "linear_regression" => FunctionName::LinearRegression,
            "LOCAL_SAMPLE_CALCULATOR" | "local_sample_calculator" => {
                FunctionName::LocalSampleCalculator
            }
            "LOGISTIC_REGRESSION" | "logistic_regression" => FunctionName::LogisticRegression,
            "METRICS_CORRELATION" | "metrics_correlation" => FunctionName::MetricsCorrelation,
            "QUESTION_ANSWERING" | "question_answering" => FunctionName::QuestionAnswering,
            "RCF_SUMMARIZE" | "rcf_summarize" => FunctionName::RcfSummarize,
            "REMOTE" | "remote" => FunctionName::Remote,
            "SAMPLE_ALGO" | "sample_algo" => FunctionName::SampleAlgo,
            "SPARSE_ENCODING" | "sparse_encoding" => FunctionName::SparseEncoding,
            "SPARSE_TOKENIZE" | "sparse_tokenize" => FunctionName::SparseTokenize,
            "TEXT_EMBEDDING" | "text_embedding" => FunctionName::TextEmbedding,
            "TEXT_SIMILARITY" | "text_similarity" => FunctionName::TextSimilarity,
            _ => FunctionName::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetAgentResponse {
    #[doc = "The created time."]
    pub created_time: Option<i64>,
    #[doc = "The agent description."]
    pub description: Option<String>,
    #[doc = "Whether the agent is hidden."]
    pub is_hidden: Option<bool>,
    #[doc = "The last updated time."]
    pub last_updated_time: Option<i64>,
    pub name: Option<crate::types::common::Name>,
    pub tools: Option<Vec<crate::types::ml_common::ToolItems>>,
    pub r#type: Option<crate::types::ml_common::AgentType>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GetAgentResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GetAgentResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GetAgentResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_created_time: Option<i64> = None;
                let mut seen_created_time = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_is_hidden: Option<bool> = None;
                let mut seen_is_hidden = false;
                let mut f_last_updated_time: Option<i64> = None;
                let mut seen_last_updated_time = false;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut seen_name = false;
                let mut f_tools: Option<Vec<crate::types::ml_common::ToolItems>> = None;
                let mut seen_tools = false;
                let mut f_type: Option<crate::types::ml_common::AgentType> = None;
                let mut seen_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "created_time" => {
                            if seen_created_time {
                                return Err(serde::de::Error::duplicate_field("created_time"));
                            }
                            seen_created_time = true;
                            f_created_time = map.next_value::<Option<i64>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "is_hidden" => {
                            if seen_is_hidden {
                                return Err(serde::de::Error::duplicate_field("is_hidden"));
                            }
                            seen_is_hidden = true;
                            f_is_hidden = map.next_value::<Option<bool>>()?;
                        }
                        "last_updated_time" => {
                            if seen_last_updated_time {
                                return Err(serde::de::Error::duplicate_field("last_updated_time"));
                            }
                            seen_last_updated_time = true;
                            f_last_updated_time = map.next_value::<Option<i64>>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "tools" => {
                            if seen_tools {
                                return Err(serde::de::Error::duplicate_field("tools"));
                            }
                            seen_tools = true;
                            f_tools = map
                                .next_value::<Option<Vec<crate::types::ml_common::ToolItems>>>()?;
                        }
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type =
                                map.next_value::<Option<crate::types::ml_common::AgentType>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GetAgentResponse {
                    created_time: f_created_time,
                    description: f_description,
                    is_hidden: f_is_hidden,
                    last_updated_time: f_last_updated_time,
                    name: f_name,
                    tools: f_tools,
                    r#type: f_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GetAgentResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.created_time {
            map.serialize_entry("created_time", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.is_hidden {
            map.serialize_entry("is_hidden", v)?;
        }
        if let Some(v) = &self.last_updated_time {
            map.serialize_entry("last_updated_time", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.tools {
            map.serialize_entry("tools", v)?;
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

#[derive(Debug, Clone)]
pub struct GetAgenticMemoryResponse {
    pub action: Option<crate::types::ml_common::ActionType>,
    pub after: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub before: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The created time."]
    pub created_time: Option<i64>,
    #[doc = "Whether inference was enabled."]
    pub infer: Option<bool>,
    #[doc = "The last updated time."]
    pub last_updated_time: Option<i64>,
    #[doc = "The extracted long-term memory fact."]
    pub memory: Option<String>,
    #[doc = "The ID of the memory container."]
    pub memory_container_id: Option<String>,
    pub memory_embedding: Option<Vec<f64>>,
    #[doc = "The ID of the memory."]
    pub memory_id: Option<String>,
    pub messages: Option<Vec<crate::types::ml_common::Message>>,
    pub metadata: Option<crate::types::ml_common::Metadata>,
    pub namespace: Option<crate::types::ml_common::Namespace>,
    #[doc = "The number of namespaces."]
    pub namespace_size: Option<i64>,
    #[doc = "The ID of the memory owner."]
    pub owner_id: Option<String>,
    pub payload_type: Option<crate::types::ml_common::PayloadType>,
    #[doc = "The ID for the strategy instance."]
    pub strategy_id: Option<String>,
    pub strategy_type: Option<crate::types::ml_common::StrategyType>,
    pub tags: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GetAgenticMemoryResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GetAgenticMemoryResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GetAgenticMemoryResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_action: Option<crate::types::ml_common::ActionType> = None;
                let mut seen_action = false;
                let mut f_after: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_after = false;
                let mut f_before: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_before = false;
                let mut f_created_time: Option<i64> = None;
                let mut seen_created_time = false;
                let mut f_infer: Option<bool> = None;
                let mut seen_infer = false;
                let mut f_last_updated_time: Option<i64> = None;
                let mut seen_last_updated_time = false;
                let mut f_memory: Option<String> = None;
                let mut seen_memory = false;
                let mut f_memory_container_id: Option<String> = None;
                let mut seen_memory_container_id = false;
                let mut f_memory_embedding: Option<Vec<f64>> = None;
                let mut seen_memory_embedding = false;
                let mut f_memory_id: Option<String> = None;
                let mut seen_memory_id = false;
                let mut f_messages: Option<Vec<crate::types::ml_common::Message>> = None;
                let mut seen_messages = false;
                let mut f_metadata: Option<crate::types::ml_common::Metadata> = None;
                let mut seen_metadata = false;
                let mut f_namespace: Option<crate::types::ml_common::Namespace> = None;
                let mut seen_namespace = false;
                let mut f_namespace_size: Option<i64> = None;
                let mut seen_namespace_size = false;
                let mut f_owner_id: Option<String> = None;
                let mut seen_owner_id = false;
                let mut f_payload_type: Option<crate::types::ml_common::PayloadType> = None;
                let mut seen_payload_type = false;
                let mut f_strategy_id: Option<String> = None;
                let mut seen_strategy_id = false;
                let mut f_strategy_type: Option<crate::types::ml_common::StrategyType> = None;
                let mut seen_strategy_type = false;
                let mut f_tags: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_tags = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "action" => {
                            if seen_action {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            seen_action = true;
                            f_action =
                                map.next_value::<Option<crate::types::ml_common::ActionType>>()?;
                        }
                        "after" => {
                            if seen_after {
                                return Err(serde::de::Error::duplicate_field("after"));
                            }
                            seen_after = true;
                            f_after = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "before" => {
                            if seen_before {
                                return Err(serde::de::Error::duplicate_field("before"));
                            }
                            seen_before = true;
                            f_before = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "created_time" => {
                            if seen_created_time {
                                return Err(serde::de::Error::duplicate_field("created_time"));
                            }
                            seen_created_time = true;
                            f_created_time = map.next_value::<Option<i64>>()?;
                        }
                        "infer" => {
                            if seen_infer {
                                return Err(serde::de::Error::duplicate_field("infer"));
                            }
                            seen_infer = true;
                            f_infer = map.next_value::<Option<bool>>()?;
                        }
                        "last_updated_time" => {
                            if seen_last_updated_time {
                                return Err(serde::de::Error::duplicate_field("last_updated_time"));
                            }
                            seen_last_updated_time = true;
                            f_last_updated_time = map.next_value::<Option<i64>>()?;
                        }
                        "memory" => {
                            if seen_memory {
                                return Err(serde::de::Error::duplicate_field("memory"));
                            }
                            seen_memory = true;
                            f_memory = map.next_value::<Option<String>>()?;
                        }
                        "memory_container_id" => {
                            if seen_memory_container_id {
                                return Err(serde::de::Error::duplicate_field(
                                    "memory_container_id",
                                ));
                            }
                            seen_memory_container_id = true;
                            f_memory_container_id = map.next_value::<Option<String>>()?;
                        }
                        "memory_embedding" => {
                            if seen_memory_embedding {
                                return Err(serde::de::Error::duplicate_field("memory_embedding"));
                            }
                            seen_memory_embedding = true;
                            f_memory_embedding = map.next_value::<Option<Vec<f64>>>()?;
                        }
                        "memory_id" => {
                            if seen_memory_id {
                                return Err(serde::de::Error::duplicate_field("memory_id"));
                            }
                            seen_memory_id = true;
                            f_memory_id = map.next_value::<Option<String>>()?;
                        }
                        "messages" => {
                            if seen_messages {
                                return Err(serde::de::Error::duplicate_field("messages"));
                            }
                            seen_messages = true;
                            f_messages =
                                map.next_value::<Option<Vec<crate::types::ml_common::Message>>>()?;
                        }
                        "metadata" => {
                            if seen_metadata {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            seen_metadata = true;
                            f_metadata =
                                map.next_value::<Option<crate::types::ml_common::Metadata>>()?;
                        }
                        "namespace" => {
                            if seen_namespace {
                                return Err(serde::de::Error::duplicate_field("namespace"));
                            }
                            seen_namespace = true;
                            f_namespace =
                                map.next_value::<Option<crate::types::ml_common::Namespace>>()?;
                        }
                        "namespace_size" => {
                            if seen_namespace_size {
                                return Err(serde::de::Error::duplicate_field("namespace_size"));
                            }
                            seen_namespace_size = true;
                            f_namespace_size = map.next_value::<Option<i64>>()?;
                        }
                        "owner_id" => {
                            if seen_owner_id {
                                return Err(serde::de::Error::duplicate_field("owner_id"));
                            }
                            seen_owner_id = true;
                            f_owner_id = map.next_value::<Option<String>>()?;
                        }
                        "payload_type" => {
                            if seen_payload_type {
                                return Err(serde::de::Error::duplicate_field("payload_type"));
                            }
                            seen_payload_type = true;
                            f_payload_type =
                                map.next_value::<Option<crate::types::ml_common::PayloadType>>()?;
                        }
                        "strategy_id" => {
                            if seen_strategy_id {
                                return Err(serde::de::Error::duplicate_field("strategy_id"));
                            }
                            seen_strategy_id = true;
                            f_strategy_id = map.next_value::<Option<String>>()?;
                        }
                        "strategy_type" => {
                            if seen_strategy_type {
                                return Err(serde::de::Error::duplicate_field("strategy_type"));
                            }
                            seen_strategy_type = true;
                            f_strategy_type =
                                map.next_value::<Option<crate::types::ml_common::StrategyType>>()?;
                        }
                        "tags" => {
                            if seen_tags {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            seen_tags = true;
                            f_tags = map.next_value::<Option<
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
                Ok(GetAgenticMemoryResponse {
                    action: f_action,
                    after: f_after,
                    before: f_before,
                    created_time: f_created_time,
                    infer: f_infer,
                    last_updated_time: f_last_updated_time,
                    memory: f_memory,
                    memory_container_id: f_memory_container_id,
                    memory_embedding: f_memory_embedding,
                    memory_id: f_memory_id,
                    messages: f_messages,
                    metadata: f_metadata,
                    namespace: f_namespace,
                    namespace_size: f_namespace_size,
                    owner_id: f_owner_id,
                    payload_type: f_payload_type,
                    strategy_id: f_strategy_id,
                    strategy_type: f_strategy_type,
                    tags: f_tags,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GetAgenticMemoryResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.action {
            map.serialize_entry("action", v)?;
        }
        if let Some(v) = &self.after {
            map.serialize_entry("after", v)?;
        }
        if let Some(v) = &self.before {
            map.serialize_entry("before", v)?;
        }
        if let Some(v) = &self.created_time {
            map.serialize_entry("created_time", v)?;
        }
        if let Some(v) = &self.infer {
            map.serialize_entry("infer", v)?;
        }
        if let Some(v) = &self.last_updated_time {
            map.serialize_entry("last_updated_time", v)?;
        }
        if let Some(v) = &self.memory {
            map.serialize_entry("memory", v)?;
        }
        if let Some(v) = &self.memory_container_id {
            map.serialize_entry("memory_container_id", v)?;
        }
        if let Some(v) = &self.memory_embedding {
            map.serialize_entry("memory_embedding", v)?;
        }
        if let Some(v) = &self.memory_id {
            map.serialize_entry("memory_id", v)?;
        }
        if let Some(v) = &self.messages {
            map.serialize_entry("messages", v)?;
        }
        if let Some(v) = &self.metadata {
            map.serialize_entry("metadata", v)?;
        }
        if let Some(v) = &self.namespace {
            map.serialize_entry("namespace", v)?;
        }
        if let Some(v) = &self.namespace_size {
            map.serialize_entry("namespace_size", v)?;
        }
        if let Some(v) = &self.owner_id {
            map.serialize_entry("owner_id", v)?;
        }
        if let Some(v) = &self.payload_type {
            map.serialize_entry("payload_type", v)?;
        }
        if let Some(v) = &self.strategy_id {
            map.serialize_entry("strategy_id", v)?;
        }
        if let Some(v) = &self.strategy_type {
            map.serialize_entry("strategy_type", v)?;
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
pub struct GetConnectorResponse {
    pub actions: Option<Vec<crate::types::ml_common::Action>>,
    #[doc = "The created time."]
    pub created_time: Option<i64>,
    #[doc = "The connector description."]
    pub description: Option<String>,
    #[doc = "The last updated time."]
    pub last_updated_time: Option<i64>,
    pub name: Option<crate::types::common::Name>,
    pub parameters: Option<crate::types::ml_common::Parameters>,
    pub protocol: Option<crate::types::ml_common::ConnectorProtocol>,
    pub version: Option<crate::types::common::VersionString>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GetConnectorResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GetConnectorResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GetConnectorResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_actions: Option<Vec<crate::types::ml_common::Action>> = None;
                let mut seen_actions = false;
                let mut f_created_time: Option<i64> = None;
                let mut seen_created_time = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_last_updated_time: Option<i64> = None;
                let mut seen_last_updated_time = false;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut seen_name = false;
                let mut f_parameters: Option<crate::types::ml_common::Parameters> = None;
                let mut seen_parameters = false;
                let mut f_protocol: Option<crate::types::ml_common::ConnectorProtocol> = None;
                let mut seen_protocol = false;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "actions" => {
                            if seen_actions {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            seen_actions = true;
                            f_actions =
                                map.next_value::<Option<Vec<crate::types::ml_common::Action>>>()?;
                        }
                        "created_time" => {
                            if seen_created_time {
                                return Err(serde::de::Error::duplicate_field("created_time"));
                            }
                            seen_created_time = true;
                            f_created_time = map.next_value::<Option<i64>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "last_updated_time" => {
                            if seen_last_updated_time {
                                return Err(serde::de::Error::duplicate_field("last_updated_time"));
                            }
                            seen_last_updated_time = true;
                            f_last_updated_time = map.next_value::<Option<i64>>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "parameters" => {
                            if seen_parameters {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            seen_parameters = true;
                            f_parameters =
                                map.next_value::<Option<crate::types::ml_common::Parameters>>()?;
                        }
                        "protocol" => {
                            if seen_protocol {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            seen_protocol = true;
                            f_protocol = map
                                .next_value::<Option<crate::types::ml_common::ConnectorProtocol>>(
                                )?;
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GetConnectorResponse {
                    actions: f_actions,
                    created_time: f_created_time,
                    description: f_description,
                    last_updated_time: f_last_updated_time,
                    name: f_name,
                    parameters: f_parameters,
                    protocol: f_protocol,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GetConnectorResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.actions {
            map.serialize_entry("actions", v)?;
        }
        if let Some(v) = &self.created_time {
            map.serialize_entry("created_time", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.last_updated_time {
            map.serialize_entry("last_updated_time", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.parameters {
            map.serialize_entry("parameters", v)?;
        }
        if let Some(v) = &self.protocol {
            map.serialize_entry("protocol", v)?;
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
pub struct GetMemoryContainerResponse {
    pub configuration: Option<crate::types::ml_common::MemoryContainerConfiguration>,
    #[doc = "The created time."]
    pub created_time: Option<i64>,
    #[doc = "The memory container description."]
    pub description: Option<String>,
    #[doc = "The last updated time."]
    pub last_updated_time: Option<i64>,
    pub name: Option<crate::types::common::Name>,
    pub owner: Option<crate::types::ml_common::Owner>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GetMemoryContainerResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GetMemoryContainerResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GetMemoryContainerResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_configuration: Option<
                    crate::types::ml_common::MemoryContainerConfiguration,
                > = None;
                let mut seen_configuration = false;
                let mut f_created_time: Option<i64> = None;
                let mut seen_created_time = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_last_updated_time: Option<i64> = None;
                let mut seen_last_updated_time = false;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut seen_name = false;
                let mut f_owner: Option<crate::types::ml_common::Owner> = None;
                let mut seen_owner = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "configuration" => {
                            if seen_configuration {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            seen_configuration = true;
                            f_configuration = map.next_value::<Option<crate::types::ml_common::MemoryContainerConfiguration>>()?;
                        }
                        "created_time" => {
                            if seen_created_time {
                                return Err(serde::de::Error::duplicate_field("created_time"));
                            }
                            seen_created_time = true;
                            f_created_time = map.next_value::<Option<i64>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "last_updated_time" => {
                            if seen_last_updated_time {
                                return Err(serde::de::Error::duplicate_field("last_updated_time"));
                            }
                            seen_last_updated_time = true;
                            f_last_updated_time = map.next_value::<Option<i64>>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "owner" => {
                            if seen_owner {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            seen_owner = true;
                            f_owner = map.next_value::<Option<crate::types::ml_common::Owner>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GetMemoryContainerResponse {
                    configuration: f_configuration,
                    created_time: f_created_time,
                    description: f_description,
                    last_updated_time: f_last_updated_time,
                    name: f_name,
                    owner: f_owner,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GetMemoryContainerResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.configuration {
            map.serialize_entry("configuration", v)?;
        }
        if let Some(v) = &self.created_time {
            map.serialize_entry("created_time", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.last_updated_time {
            map.serialize_entry("last_updated_time", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.owner {
            map.serialize_entry("owner", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct GetProfileResponse {
    pub nodes: Option<crate::types::ml_common::Nodes>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GetProfileResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GetProfileResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GetProfileResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_nodes: Option<crate::types::ml_common::Nodes> = None;
                let mut seen_nodes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "nodes" => {
                            if seen_nodes {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            seen_nodes = true;
                            f_nodes = map.next_value::<Option<crate::types::ml_common::Nodes>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GetProfileResponse {
                    nodes: f_nodes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GetProfileResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.nodes {
            map.serialize_entry("nodes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct GetStatsResponse {
    #[doc = "The config index status."]
    pub ml_config_index_status: Option<crate::types::ml_common::MlIndexStatus>,
    #[doc = "The connector count."]
    pub ml_connector_count: Option<i64>,
    #[doc = "The connector index status."]
    pub ml_connector_index_status: Option<crate::types::ml_common::MlIndexStatus>,
    #[doc = "The controller index status."]
    pub ml_controller_index_status: Option<crate::types::ml_common::MlIndexStatus>,
    #[doc = "The model count."]
    pub ml_model_count: Option<i64>,
    #[doc = "The model index status."]
    pub ml_model_index_status: Option<crate::types::ml_common::MlIndexStatus>,
    #[doc = "The task index status."]
    pub ml_task_index_status: Option<crate::types::ml_common::MlIndexStatus>,
    pub nodes: Option<crate::types::ml_common::NodeStats>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GetStatsResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GetStatsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GetStatsResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_ml_config_index_status: Option<crate::types::ml_common::MlIndexStatus> =
                    None;
                let mut seen_ml_config_index_status = false;
                let mut f_ml_connector_count: Option<i64> = None;
                let mut seen_ml_connector_count = false;
                let mut f_ml_connector_index_status: Option<
                    crate::types::ml_common::MlIndexStatus,
                > = None;
                let mut seen_ml_connector_index_status = false;
                let mut f_ml_controller_index_status: Option<
                    crate::types::ml_common::MlIndexStatus,
                > = None;
                let mut seen_ml_controller_index_status = false;
                let mut f_ml_model_count: Option<i64> = None;
                let mut seen_ml_model_count = false;
                let mut f_ml_model_index_status: Option<crate::types::ml_common::MlIndexStatus> =
                    None;
                let mut seen_ml_model_index_status = false;
                let mut f_ml_task_index_status: Option<crate::types::ml_common::MlIndexStatus> =
                    None;
                let mut seen_ml_task_index_status = false;
                let mut f_nodes: Option<crate::types::ml_common::NodeStats> = None;
                let mut seen_nodes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "ml_config_index_status" => {
                            if seen_ml_config_index_status {
                                return Err(serde::de::Error::duplicate_field(
                                    "ml_config_index_status",
                                ));
                            }
                            seen_ml_config_index_status = true;
                            f_ml_config_index_status =
                                map.next_value::<Option<crate::types::ml_common::MlIndexStatus>>()?;
                        }
                        "ml_connector_count" => {
                            if seen_ml_connector_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "ml_connector_count",
                                ));
                            }
                            seen_ml_connector_count = true;
                            f_ml_connector_count = map.next_value::<Option<i64>>()?;
                        }
                        "ml_connector_index_status" => {
                            if seen_ml_connector_index_status {
                                return Err(serde::de::Error::duplicate_field(
                                    "ml_connector_index_status",
                                ));
                            }
                            seen_ml_connector_index_status = true;
                            f_ml_connector_index_status =
                                map.next_value::<Option<crate::types::ml_common::MlIndexStatus>>()?;
                        }
                        "ml_controller_index_status" => {
                            if seen_ml_controller_index_status {
                                return Err(serde::de::Error::duplicate_field(
                                    "ml_controller_index_status",
                                ));
                            }
                            seen_ml_controller_index_status = true;
                            f_ml_controller_index_status =
                                map.next_value::<Option<crate::types::ml_common::MlIndexStatus>>()?;
                        }
                        "ml_model_count" => {
                            if seen_ml_model_count {
                                return Err(serde::de::Error::duplicate_field("ml_model_count"));
                            }
                            seen_ml_model_count = true;
                            f_ml_model_count = map.next_value::<Option<i64>>()?;
                        }
                        "ml_model_index_status" => {
                            if seen_ml_model_index_status {
                                return Err(serde::de::Error::duplicate_field(
                                    "ml_model_index_status",
                                ));
                            }
                            seen_ml_model_index_status = true;
                            f_ml_model_index_status =
                                map.next_value::<Option<crate::types::ml_common::MlIndexStatus>>()?;
                        }
                        "ml_task_index_status" => {
                            if seen_ml_task_index_status {
                                return Err(serde::de::Error::duplicate_field(
                                    "ml_task_index_status",
                                ));
                            }
                            seen_ml_task_index_status = true;
                            f_ml_task_index_status =
                                map.next_value::<Option<crate::types::ml_common::MlIndexStatus>>()?;
                        }
                        "nodes" => {
                            if seen_nodes {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            seen_nodes = true;
                            f_nodes =
                                map.next_value::<Option<crate::types::ml_common::NodeStats>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GetStatsResponse {
                    ml_config_index_status: f_ml_config_index_status,
                    ml_connector_count: f_ml_connector_count,
                    ml_connector_index_status: f_ml_connector_index_status,
                    ml_controller_index_status: f_ml_controller_index_status,
                    ml_model_count: f_ml_model_count,
                    ml_model_index_status: f_ml_model_index_status,
                    ml_task_index_status: f_ml_task_index_status,
                    nodes: f_nodes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GetStatsResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.ml_config_index_status {
            map.serialize_entry("ml_config_index_status", v)?;
        }
        if let Some(v) = &self.ml_connector_count {
            map.serialize_entry("ml_connector_count", v)?;
        }
        if let Some(v) = &self.ml_connector_index_status {
            map.serialize_entry("ml_connector_index_status", v)?;
        }
        if let Some(v) = &self.ml_controller_index_status {
            map.serialize_entry("ml_controller_index_status", v)?;
        }
        if let Some(v) = &self.ml_model_count {
            map.serialize_entry("ml_model_count", v)?;
        }
        if let Some(v) = &self.ml_model_index_status {
            map.serialize_entry("ml_model_index_status", v)?;
        }
        if let Some(v) = &self.ml_task_index_status {
            map.serialize_entry("ml_task_index_status", v)?;
        }
        if let Some(v) = &self.nodes {
            map.serialize_entry("nodes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Headers {
    pub content_type: Option<String>,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Headers {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Headers;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Headers object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_content_type: Option<String> = None;
                let mut seen_content_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "content_type" => {
                            if seen_content_type {
                                return Err(serde::de::Error::duplicate_field("content_type"));
                            }
                            seen_content_type = true;
                            f_content_type = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Headers {
                    content_type: f_content_type,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Headers {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.content_type {
            map.serialize_entry("content_type", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct HitsTotal {
    #[doc = "The relation."]
    pub relation: String,
    #[doc = "The total number of hits."]
    pub value: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HitsTotal {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HitsTotal;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HitsTotal object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_relation: Option<String> = None;
                let mut f_value: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "relation" => {
                            if f_relation.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            f_relation = Some(map.next_value::<String>()?);
                        }
                        "value" => {
                            if f_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            f_value = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HitsTotal {
                    relation: f_relation
                        .ok_or_else(|| serde::de::Error::missing_field("relation"))?,
                    value: f_value.ok_or_else(|| serde::de::Error::missing_field("value"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HitsTotal {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("relation", &self.relation)?;
        map.serialize_entry("value", &self.value)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Index {
    #[doc = "The number of replicas."]
    pub number_of_replicas: Option<String>,
    #[doc = "The number of shards."]
    pub number_of_shards: Option<String>,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Index {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Index;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Index object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_number_of_replicas: Option<String> = None;
                let mut seen_number_of_replicas = false;
                let mut f_number_of_shards: Option<String> = None;
                let mut seen_number_of_shards = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "number_of_replicas" => {
                            if seen_number_of_replicas {
                                return Err(serde::de::Error::duplicate_field(
                                    "number_of_replicas",
                                ));
                            }
                            seen_number_of_replicas = true;
                            f_number_of_replicas = map.next_value::<Option<String>>()?;
                        }
                        "number_of_shards" => {
                            if seen_number_of_shards {
                                return Err(serde::de::Error::duplicate_field("number_of_shards"));
                            }
                            seen_number_of_shards = true;
                            f_number_of_shards = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Index {
                    number_of_replicas: f_number_of_replicas,
                    number_of_shards: f_number_of_shards,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Index {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.number_of_replicas {
            map.serialize_entry("number_of_replicas", v)?;
        }
        if let Some(v) = &self.number_of_shards {
            map.serialize_entry("number_of_shards", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IndexSettings {
    pub long_term_memory_history_index: Option<crate::types::ml_common::MemoryIndexSettings>,
    pub long_term_memory_index: Option<crate::types::ml_common::MemoryIndexSettings>,
    pub session_index: Option<crate::types::ml_common::MemoryIndexSettings>,
    pub short_term_memory_index: Option<crate::types::ml_common::MemoryIndexSettings>,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettings {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettings;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettings object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_long_term_memory_history_index: Option<
                    crate::types::ml_common::MemoryIndexSettings,
                > = None;
                let mut seen_long_term_memory_history_index = false;
                let mut f_long_term_memory_index: Option<
                    crate::types::ml_common::MemoryIndexSettings,
                > = None;
                let mut seen_long_term_memory_index = false;
                let mut f_session_index: Option<crate::types::ml_common::MemoryIndexSettings> =
                    None;
                let mut seen_session_index = false;
                let mut f_short_term_memory_index: Option<
                    crate::types::ml_common::MemoryIndexSettings,
                > = None;
                let mut seen_short_term_memory_index = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "long_term_memory_history_index" => {
                            if seen_long_term_memory_history_index {
                                return Err(serde::de::Error::duplicate_field(
                                    "long_term_memory_history_index",
                                ));
                            }
                            seen_long_term_memory_history_index = true;
                            f_long_term_memory_history_index = map
                                .next_value::<Option<crate::types::ml_common::MemoryIndexSettings>>(
                                )?;
                        }
                        "long_term_memory_index" => {
                            if seen_long_term_memory_index {
                                return Err(serde::de::Error::duplicate_field(
                                    "long_term_memory_index",
                                ));
                            }
                            seen_long_term_memory_index = true;
                            f_long_term_memory_index = map
                                .next_value::<Option<crate::types::ml_common::MemoryIndexSettings>>(
                                )?;
                        }
                        "session_index" => {
                            if seen_session_index {
                                return Err(serde::de::Error::duplicate_field("session_index"));
                            }
                            seen_session_index = true;
                            f_session_index = map
                                .next_value::<Option<crate::types::ml_common::MemoryIndexSettings>>(
                                )?;
                        }
                        "short_term_memory_index" => {
                            if seen_short_term_memory_index {
                                return Err(serde::de::Error::duplicate_field(
                                    "short_term_memory_index",
                                ));
                            }
                            seen_short_term_memory_index = true;
                            f_short_term_memory_index = map
                                .next_value::<Option<crate::types::ml_common::MemoryIndexSettings>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettings {
                    long_term_memory_history_index: f_long_term_memory_history_index,
                    long_term_memory_index: f_long_term_memory_index,
                    session_index: f_session_index,
                    short_term_memory_index: f_short_term_memory_index,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettings {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.long_term_memory_history_index {
            map.serialize_entry("long_term_memory_history_index", v)?;
        }
        if let Some(v) = &self.long_term_memory_index {
            map.serialize_entry("long_term_memory_index", v)?;
        }
        if let Some(v) = &self.session_index {
            map.serialize_entry("session_index", v)?;
        }
        if let Some(v) = &self.short_term_memory_index {
            map.serialize_entry("short_term_memory_index", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct InferenceResults {
    pub output: Option<Vec<crate::types::ml_common::Output>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for InferenceResults {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = InferenceResults;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a InferenceResults object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_output: Option<Vec<crate::types::ml_common::Output>> = None;
                let mut seen_output = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "output" => {
                            if seen_output {
                                return Err(serde::de::Error::duplicate_field("output"));
                            }
                            seen_output = true;
                            f_output =
                                map.next_value::<Option<Vec<crate::types::ml_common::Output>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(InferenceResults {
                    output: f_output,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for InferenceResults {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.output {
            map.serialize_entry("output", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Memory {
    pub additional_info: Option<crate::types::ml_common::AdditionalInfo>,
    #[doc = "The create time."]
    pub create_time: Option<String>,
    pub memory_id: Option<crate::types::common::Name>,
    pub name: Option<crate::types::common::Name>,
    pub r#type: Option<String>,
    #[doc = "The updated time."]
    pub updated_time: Option<String>,
    #[doc = "The username of the user."]
    pub user: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Memory {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Memory;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Memory object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_additional_info: Option<crate::types::ml_common::AdditionalInfo> = None;
                let mut seen_additional_info = false;
                let mut f_create_time: Option<String> = None;
                let mut seen_create_time = false;
                let mut f_memory_id: Option<crate::types::common::Name> = None;
                let mut seen_memory_id = false;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut seen_name = false;
                let mut f_type: Option<String> = None;
                let mut seen_type = false;
                let mut f_updated_time: Option<String> = None;
                let mut seen_updated_time = false;
                let mut f_user: Option<String> = None;
                let mut seen_user = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "additional_info" => {
                            if seen_additional_info {
                                return Err(serde::de::Error::duplicate_field("additional_info"));
                            }
                            seen_additional_info = true;
                            f_additional_info = map
                                .next_value::<Option<crate::types::ml_common::AdditionalInfo>>()?;
                        }
                        "create_time" => {
                            if seen_create_time {
                                return Err(serde::de::Error::duplicate_field("create_time"));
                            }
                            seen_create_time = true;
                            f_create_time = map.next_value::<Option<String>>()?;
                        }
                        "memory_id" => {
                            if seen_memory_id {
                                return Err(serde::de::Error::duplicate_field("memory_id"));
                            }
                            seen_memory_id = true;
                            f_memory_id = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<String>>()?;
                        }
                        "updated_time" => {
                            if seen_updated_time {
                                return Err(serde::de::Error::duplicate_field("updated_time"));
                            }
                            seen_updated_time = true;
                            f_updated_time = map.next_value::<Option<String>>()?;
                        }
                        "user" => {
                            if seen_user {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            seen_user = true;
                            f_user = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Memory {
                    additional_info: f_additional_info,
                    create_time: f_create_time,
                    memory_id: f_memory_id,
                    name: f_name,
                    r#type: f_type,
                    updated_time: f_updated_time,
                    user: f_user,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Memory {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.additional_info {
            map.serialize_entry("additional_info", v)?;
        }
        if let Some(v) = &self.create_time {
            map.serialize_entry("create_time", v)?;
        }
        if let Some(v) = &self.memory_id {
            map.serialize_entry("memory_id", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("type", v)?;
        }
        if let Some(v) = &self.updated_time {
            map.serialize_entry("updated_time", v)?;
        }
        if let Some(v) = &self.user {
            map.serialize_entry("user", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct MemoryContainerConfiguration {
    #[doc = "Whether history will be persisted."]
    pub disable_history: Option<bool>,
    #[doc = "Whether session will be persisted."]
    pub disable_session: Option<bool>,
    #[doc = "The embedding dimension."]
    pub embedding_dimension: Option<i64>,
    #[doc = "The embedding model ID."]
    pub embedding_model_id: Option<String>,
    #[doc = "The embedding model type."]
    pub embedding_model_type: Option<String>,
    #[doc = "Custom prefix for memory indices."]
    pub index_prefix: Option<String>,
    pub index_settings: Option<crate::types::ml_common::IndexSettings>,
    #[doc = "The LLM ID."]
    pub llm_id: Option<String>,
    #[doc = "The maximum number of similar memories retrieved during consolidation."]
    pub max_infer_size: Option<i64>,
    pub parameters: Option<crate::types::ml_common::Parameters>,
    pub strategies: Option<Vec<crate::types::ml_common::Strategy>>,
    #[doc = "Whether to use system indices."]
    pub use_system_index: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MemoryContainerConfiguration {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MemoryContainerConfiguration;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MemoryContainerConfiguration object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_disable_history: Option<bool> = None;
                let mut seen_disable_history = false;
                let mut f_disable_session: Option<bool> = None;
                let mut seen_disable_session = false;
                let mut f_embedding_dimension: Option<i64> = None;
                let mut seen_embedding_dimension = false;
                let mut f_embedding_model_id: Option<String> = None;
                let mut seen_embedding_model_id = false;
                let mut f_embedding_model_type: Option<String> = None;
                let mut seen_embedding_model_type = false;
                let mut f_index_prefix: Option<String> = None;
                let mut seen_index_prefix = false;
                let mut f_index_settings: Option<crate::types::ml_common::IndexSettings> = None;
                let mut seen_index_settings = false;
                let mut f_llm_id: Option<String> = None;
                let mut seen_llm_id = false;
                let mut f_max_infer_size: Option<i64> = None;
                let mut seen_max_infer_size = false;
                let mut f_parameters: Option<crate::types::ml_common::Parameters> = None;
                let mut seen_parameters = false;
                let mut f_strategies: Option<Vec<crate::types::ml_common::Strategy>> = None;
                let mut seen_strategies = false;
                let mut f_use_system_index: Option<bool> = None;
                let mut seen_use_system_index = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "disable_history" => {
                            if seen_disable_history {
                                return Err(serde::de::Error::duplicate_field("disable_history"));
                            }
                            seen_disable_history = true;
                            f_disable_history = map.next_value::<Option<bool>>()?;
                        }
                        "disable_session" => {
                            if seen_disable_session {
                                return Err(serde::de::Error::duplicate_field("disable_session"));
                            }
                            seen_disable_session = true;
                            f_disable_session = map.next_value::<Option<bool>>()?;
                        }
                        "embedding_dimension" => {
                            if seen_embedding_dimension {
                                return Err(serde::de::Error::duplicate_field(
                                    "embedding_dimension",
                                ));
                            }
                            seen_embedding_dimension = true;
                            f_embedding_dimension = map.next_value::<Option<i64>>()?;
                        }
                        "embedding_model_id" => {
                            if seen_embedding_model_id {
                                return Err(serde::de::Error::duplicate_field(
                                    "embedding_model_id",
                                ));
                            }
                            seen_embedding_model_id = true;
                            f_embedding_model_id = map.next_value::<Option<String>>()?;
                        }
                        "embedding_model_type" => {
                            if seen_embedding_model_type {
                                return Err(serde::de::Error::duplicate_field(
                                    "embedding_model_type",
                                ));
                            }
                            seen_embedding_model_type = true;
                            f_embedding_model_type = map.next_value::<Option<String>>()?;
                        }
                        "index_prefix" => {
                            if seen_index_prefix {
                                return Err(serde::de::Error::duplicate_field("index_prefix"));
                            }
                            seen_index_prefix = true;
                            f_index_prefix = map.next_value::<Option<String>>()?;
                        }
                        "index_settings" => {
                            if seen_index_settings {
                                return Err(serde::de::Error::duplicate_field("index_settings"));
                            }
                            seen_index_settings = true;
                            f_index_settings =
                                map.next_value::<Option<crate::types::ml_common::IndexSettings>>()?;
                        }
                        "llm_id" => {
                            if seen_llm_id {
                                return Err(serde::de::Error::duplicate_field("llm_id"));
                            }
                            seen_llm_id = true;
                            f_llm_id = map.next_value::<Option<String>>()?;
                        }
                        "max_infer_size" => {
                            if seen_max_infer_size {
                                return Err(serde::de::Error::duplicate_field("max_infer_size"));
                            }
                            seen_max_infer_size = true;
                            f_max_infer_size = map.next_value::<Option<i64>>()?;
                        }
                        "parameters" => {
                            if seen_parameters {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            seen_parameters = true;
                            f_parameters =
                                map.next_value::<Option<crate::types::ml_common::Parameters>>()?;
                        }
                        "strategies" => {
                            if seen_strategies {
                                return Err(serde::de::Error::duplicate_field("strategies"));
                            }
                            seen_strategies = true;
                            f_strategies =
                                map.next_value::<Option<Vec<crate::types::ml_common::Strategy>>>()?;
                        }
                        "use_system_index" => {
                            if seen_use_system_index {
                                return Err(serde::de::Error::duplicate_field("use_system_index"));
                            }
                            seen_use_system_index = true;
                            f_use_system_index = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MemoryContainerConfiguration {
                    disable_history: f_disable_history,
                    disable_session: f_disable_session,
                    embedding_dimension: f_embedding_dimension,
                    embedding_model_id: f_embedding_model_id,
                    embedding_model_type: f_embedding_model_type,
                    index_prefix: f_index_prefix,
                    index_settings: f_index_settings,
                    llm_id: f_llm_id,
                    max_infer_size: f_max_infer_size,
                    parameters: f_parameters,
                    strategies: f_strategies,
                    use_system_index: f_use_system_index,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MemoryContainerConfiguration {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.disable_history {
            map.serialize_entry("disable_history", v)?;
        }
        if let Some(v) = &self.disable_session {
            map.serialize_entry("disable_session", v)?;
        }
        if let Some(v) = &self.embedding_dimension {
            map.serialize_entry("embedding_dimension", v)?;
        }
        if let Some(v) = &self.embedding_model_id {
            map.serialize_entry("embedding_model_id", v)?;
        }
        if let Some(v) = &self.embedding_model_type {
            map.serialize_entry("embedding_model_type", v)?;
        }
        if let Some(v) = &self.index_prefix {
            map.serialize_entry("index_prefix", v)?;
        }
        if let Some(v) = &self.index_settings {
            map.serialize_entry("index_settings", v)?;
        }
        if let Some(v) = &self.llm_id {
            map.serialize_entry("llm_id", v)?;
        }
        if let Some(v) = &self.max_infer_size {
            map.serialize_entry("max_infer_size", v)?;
        }
        if let Some(v) = &self.parameters {
            map.serialize_entry("parameters", v)?;
        }
        if let Some(v) = &self.strategies {
            map.serialize_entry("strategies", v)?;
        }
        if let Some(v) = &self.use_system_index {
            map.serialize_entry("use_system_index", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct MemoryIndexSettings {
    pub index: Option<crate::types::ml_common::Index>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MemoryIndexSettings {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MemoryIndexSettings;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MemoryIndexSettings object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_index: Option<crate::types::ml_common::Index> = None;
                let mut seen_index = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<crate::types::ml_common::Index>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MemoryIndexSettings {
                    index: f_index,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MemoryIndexSettings {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
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
pub struct Message {
    pub additional_info: Option<crate::types::ml_common::AdditionalInfo>,
    pub content: Option<Vec<crate::types::ml_common::Content>>,
    #[doc = "The create time."]
    pub create_time: Option<String>,
    #[doc = "The question in the message."]
    pub input: Option<String>,
    pub memory_id: Option<crate::types::common::Name>,
    pub message_id: Option<crate::types::common::Name>,
    #[doc = "The system name that generated the response."]
    pub origin: Option<String>,
    #[doc = "The parent message ID."]
    pub parent_message_id: Option<String>,
    #[doc = "The prompt template."]
    pub prompt_template: Option<String>,
    #[doc = "The answer to the question."]
    pub response: Option<String>,
    #[doc = "The role of the message."]
    pub role: Option<String>,
    #[doc = "The trace number."]
    pub trace_number: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Message;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Message object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_additional_info: Option<crate::types::ml_common::AdditionalInfo> = None;
                let mut seen_additional_info = false;
                let mut f_content: Option<Vec<crate::types::ml_common::Content>> = None;
                let mut seen_content = false;
                let mut f_create_time: Option<String> = None;
                let mut seen_create_time = false;
                let mut f_input: Option<String> = None;
                let mut seen_input = false;
                let mut f_memory_id: Option<crate::types::common::Name> = None;
                let mut seen_memory_id = false;
                let mut f_message_id: Option<crate::types::common::Name> = None;
                let mut seen_message_id = false;
                let mut f_origin: Option<String> = None;
                let mut seen_origin = false;
                let mut f_parent_message_id: Option<String> = None;
                let mut seen_parent_message_id = false;
                let mut f_prompt_template: Option<String> = None;
                let mut seen_prompt_template = false;
                let mut f_response: Option<String> = None;
                let mut seen_response = false;
                let mut f_role: Option<String> = None;
                let mut seen_role = false;
                let mut f_trace_number: Option<i64> = None;
                let mut seen_trace_number = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "additional_info" => {
                            if seen_additional_info {
                                return Err(serde::de::Error::duplicate_field("additional_info"));
                            }
                            seen_additional_info = true;
                            f_additional_info = map
                                .next_value::<Option<crate::types::ml_common::AdditionalInfo>>()?;
                        }
                        "content" => {
                            if seen_content {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            seen_content = true;
                            f_content =
                                map.next_value::<Option<Vec<crate::types::ml_common::Content>>>()?;
                        }
                        "create_time" => {
                            if seen_create_time {
                                return Err(serde::de::Error::duplicate_field("create_time"));
                            }
                            seen_create_time = true;
                            f_create_time = map.next_value::<Option<String>>()?;
                        }
                        "input" => {
                            if seen_input {
                                return Err(serde::de::Error::duplicate_field("input"));
                            }
                            seen_input = true;
                            f_input = map.next_value::<Option<String>>()?;
                        }
                        "memory_id" => {
                            if seen_memory_id {
                                return Err(serde::de::Error::duplicate_field("memory_id"));
                            }
                            seen_memory_id = true;
                            f_memory_id = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "message_id" => {
                            if seen_message_id {
                                return Err(serde::de::Error::duplicate_field("message_id"));
                            }
                            seen_message_id = true;
                            f_message_id =
                                map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "origin" => {
                            if seen_origin {
                                return Err(serde::de::Error::duplicate_field("origin"));
                            }
                            seen_origin = true;
                            f_origin = map.next_value::<Option<String>>()?;
                        }
                        "parent_message_id" => {
                            if seen_parent_message_id {
                                return Err(serde::de::Error::duplicate_field("parent_message_id"));
                            }
                            seen_parent_message_id = true;
                            f_parent_message_id = map.next_value::<Option<String>>()?;
                        }
                        "prompt_template" => {
                            if seen_prompt_template {
                                return Err(serde::de::Error::duplicate_field("prompt_template"));
                            }
                            seen_prompt_template = true;
                            f_prompt_template = map.next_value::<Option<String>>()?;
                        }
                        "response" => {
                            if seen_response {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            seen_response = true;
                            f_response = map.next_value::<Option<String>>()?;
                        }
                        "role" => {
                            if seen_role {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            seen_role = true;
                            f_role = map.next_value::<Option<String>>()?;
                        }
                        "trace_number" => {
                            if seen_trace_number {
                                return Err(serde::de::Error::duplicate_field("trace_number"));
                            }
                            seen_trace_number = true;
                            f_trace_number = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Message {
                    additional_info: f_additional_info,
                    content: f_content,
                    create_time: f_create_time,
                    input: f_input,
                    memory_id: f_memory_id,
                    message_id: f_message_id,
                    origin: f_origin,
                    parent_message_id: f_parent_message_id,
                    prompt_template: f_prompt_template,
                    response: f_response,
                    role: f_role,
                    trace_number: f_trace_number,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.additional_info {
            map.serialize_entry("additional_info", v)?;
        }
        if let Some(v) = &self.content {
            map.serialize_entry("content", v)?;
        }
        if let Some(v) = &self.create_time {
            map.serialize_entry("create_time", v)?;
        }
        if let Some(v) = &self.input {
            map.serialize_entry("input", v)?;
        }
        if let Some(v) = &self.memory_id {
            map.serialize_entry("memory_id", v)?;
        }
        if let Some(v) = &self.message_id {
            map.serialize_entry("message_id", v)?;
        }
        if let Some(v) = &self.origin {
            map.serialize_entry("origin", v)?;
        }
        if let Some(v) = &self.parent_message_id {
            map.serialize_entry("parent_message_id", v)?;
        }
        if let Some(v) = &self.prompt_template {
            map.serialize_entry("prompt_template", v)?;
        }
        if let Some(v) = &self.response {
            map.serialize_entry("response", v)?;
        }
        if let Some(v) = &self.role {
            map.serialize_entry("role", v)?;
        }
        if let Some(v) = &self.trace_number {
            map.serialize_entry("trace_number", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Messages {
    #[doc = "The content of the message."]
    pub content: Option<String>,
    #[doc = "The role of the message."]
    pub role: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Messages {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Messages;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Messages object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_content: Option<String> = None;
                let mut seen_content = false;
                let mut f_role: Option<String> = None;
                let mut seen_role = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "content" => {
                            if seen_content {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            seen_content = true;
                            f_content = map.next_value::<Option<String>>()?;
                        }
                        "role" => {
                            if seen_role {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            seen_role = true;
                            f_role = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Messages {
                    content: f_content,
                    role: f_role,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Messages {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.content {
            map.serialize_entry("content", v)?;
        }
        if let Some(v) = &self.role {
            map.serialize_entry("role", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type Metadata = std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MlIndexStatus {
    #[doc = "`green`"]
    Green,
    #[doc = "`non-existent`"]
    NonExistent,
    #[doc = "`red`"]
    Red,
    #[doc = "`yellow`"]
    Yellow,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl MlIndexStatus {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            MlIndexStatus::Green => "green",
            MlIndexStatus::NonExistent => "non-existent",
            MlIndexStatus::Red => "red",
            MlIndexStatus::Yellow => "yellow",
            MlIndexStatus::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for MlIndexStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MlIndexStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "green" => MlIndexStatus::Green,
            "non-existent" => MlIndexStatus::NonExistent,
            "red" => MlIndexStatus::Red,
            "yellow" => MlIndexStatus::Yellow,
            _ => MlIndexStatus::Unknown(s),
        })
    }
}

#[doc = "The input type."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MlInputDataType {
    #[doc = "`DATA_FRAME`"]
    DataFrame,
    #[doc = "`QUESTION_ANSWERING`"]
    QuestionAnswering,
    #[doc = "`REMOTE`"]
    Remote,
    #[doc = "`SEARCH_QUERY`"]
    SearchQuery,
    #[doc = "`TEXT_DOCS`"]
    TextDocs,
    #[doc = "`TEXT_SIMILARITY`"]
    TextSimilarity,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl MlInputDataType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            MlInputDataType::DataFrame => "DATA_FRAME",
            MlInputDataType::QuestionAnswering => "QUESTION_ANSWERING",
            MlInputDataType::Remote => "REMOTE",
            MlInputDataType::SearchQuery => "SEARCH_QUERY",
            MlInputDataType::TextDocs => "TEXT_DOCS",
            MlInputDataType::TextSimilarity => "TEXT_SIMILARITY",
            MlInputDataType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for MlInputDataType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MlInputDataType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "DATA_FRAME" => MlInputDataType::DataFrame,
            "QUESTION_ANSWERING" => MlInputDataType::QuestionAnswering,
            "REMOTE" => MlInputDataType::Remote,
            "SEARCH_QUERY" => MlInputDataType::SearchQuery,
            "TEXT_DOCS" => MlInputDataType::TextDocs,
            "TEXT_SIMILARITY" => MlInputDataType::TextSimilarity,
            _ => MlInputDataType::Unknown(s),
        })
    }
}

#[doc = "The result data type."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MlResultDataType {
    #[doc = "`BOOLEAN`"]
    Boolean,
    #[doc = "`FLOAT16`"]
    Float16,
    #[doc = "`FLOAT32`"]
    Float32,
    #[doc = "`FLOAT64`"]
    Float64,
    #[doc = "`INT32`"]
    Int32,
    #[doc = "`INT64`"]
    Int64,
    #[doc = "`INT8`"]
    Int8,
    #[doc = "`STRING`"]
    String,
    #[doc = "`UINT8`"]
    Uint8,
    #[doc = "`UNKNOWN`"]
    Value10,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl MlResultDataType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            MlResultDataType::Boolean => "BOOLEAN",
            MlResultDataType::Float16 => "FLOAT16",
            MlResultDataType::Float32 => "FLOAT32",
            MlResultDataType::Float64 => "FLOAT64",
            MlResultDataType::Int32 => "INT32",
            MlResultDataType::Int64 => "INT64",
            MlResultDataType::Int8 => "INT8",
            MlResultDataType::String => "STRING",
            MlResultDataType::Uint8 => "UINT8",
            MlResultDataType::Value10 => "UNKNOWN",
            MlResultDataType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for MlResultDataType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MlResultDataType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "BOOLEAN" => MlResultDataType::Boolean,
            "FLOAT16" => MlResultDataType::Float16,
            "FLOAT32" => MlResultDataType::Float32,
            "FLOAT64" => MlResultDataType::Float64,
            "INT32" => MlResultDataType::Int32,
            "INT64" => MlResultDataType::Int64,
            "INT8" => MlResultDataType::Int8,
            "STRING" => MlResultDataType::String,
            "UINT8" => MlResultDataType::Uint8,
            "UNKNOWN" => MlResultDataType::Value10,
            _ => MlResultDataType::Unknown(s),
        })
    }
}

#[doc = "Task type."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MlTaskType {
    #[doc = "`BATCH_INGEST`"]
    BatchIngest,
    #[doc = "`BATCH_PREDICTION`"]
    BatchPrediction,
    #[doc = "`DEPLOY_MODEL`"]
    DeployModel,
    #[doc = "`EXECUTION`"]
    Execution,
    #[doc = "`PREDICTION`"]
    Prediction,
    #[doc = "`REGISTER_MODEL`"]
    RegisterModel,
    #[doc = "`TRAINING`"]
    Training,
    #[doc = "`TRAINING_AND_PREDICTION`"]
    TrainingAndPrediction,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl MlTaskType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            MlTaskType::BatchIngest => "BATCH_INGEST",
            MlTaskType::BatchPrediction => "BATCH_PREDICTION",
            MlTaskType::DeployModel => "DEPLOY_MODEL",
            MlTaskType::Execution => "EXECUTION",
            MlTaskType::Prediction => "PREDICTION",
            MlTaskType::RegisterModel => "REGISTER_MODEL",
            MlTaskType::Training => "TRAINING",
            MlTaskType::TrainingAndPrediction => "TRAINING_AND_PREDICTION",
            MlTaskType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for MlTaskType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MlTaskType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "BATCH_INGEST" => MlTaskType::BatchIngest,
            "BATCH_PREDICTION" => MlTaskType::BatchPrediction,
            "DEPLOY_MODEL" => MlTaskType::DeployModel,
            "EXECUTION" => MlTaskType::Execution,
            "PREDICTION" => MlTaskType::Prediction,
            "REGISTER_MODEL" => MlTaskType::RegisterModel,
            "TRAINING" => MlTaskType::Training,
            "TRAINING_AND_PREDICTION" => MlTaskType::TrainingAndPrediction,
            _ => MlTaskType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Model {
    #[doc = "The algorithm."]
    pub algorithm: Option<String>,
    #[doc = "The created time."]
    pub created_time: Option<i64>,
    #[doc = "Whether the model is hidden."]
    pub is_hidden: Option<bool>,
    #[doc = "The last registered time."]
    pub last_registered_time: Option<i64>,
    #[doc = "The last updated time."]
    pub last_updated_time: Option<i64>,
    pub model_config: Option<crate::types::ml_common::ModelConfig>,
    #[doc = "The model content hash value."]
    pub model_content_hash_value: Option<String>,
    #[doc = "The model content size in bytes."]
    pub model_content_size_in_bytes: Option<i64>,
    pub model_format: Option<crate::types::ml_common::ModelFormat>,
    #[doc = "The model group ID."]
    pub model_group_id: Option<String>,
    pub model_state: crate::types::ml_common::ModelState,
    #[doc = "The model version."]
    pub model_version: Option<String>,
    #[doc = "The model name."]
    pub name: Option<String>,
    #[doc = "The total chunks."]
    pub total_chunks: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Model {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Model;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Model object")
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
                let mut seen_algorithm = false;
                let mut f_created_time: Option<i64> = None;
                let mut seen_created_time = false;
                let mut f_is_hidden: Option<bool> = None;
                let mut seen_is_hidden = false;
                let mut f_last_registered_time: Option<i64> = None;
                let mut seen_last_registered_time = false;
                let mut f_last_updated_time: Option<i64> = None;
                let mut seen_last_updated_time = false;
                let mut f_model_config: Option<crate::types::ml_common::ModelConfig> = None;
                let mut seen_model_config = false;
                let mut f_model_content_hash_value: Option<String> = None;
                let mut seen_model_content_hash_value = false;
                let mut f_model_content_size_in_bytes: Option<i64> = None;
                let mut seen_model_content_size_in_bytes = false;
                let mut f_model_format: Option<crate::types::ml_common::ModelFormat> = None;
                let mut seen_model_format = false;
                let mut f_model_group_id: Option<String> = None;
                let mut seen_model_group_id = false;
                let mut f_model_state: Option<crate::types::ml_common::ModelState> = None;
                let mut f_model_version: Option<String> = None;
                let mut seen_model_version = false;
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_total_chunks: Option<i64> = None;
                let mut seen_total_chunks = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "algorithm" => {
                            if seen_algorithm {
                                return Err(serde::de::Error::duplicate_field("algorithm"));
                            }
                            seen_algorithm = true;
                            f_algorithm = map.next_value::<Option<String>>()?;
                        }
                        "created_time" => {
                            if seen_created_time {
                                return Err(serde::de::Error::duplicate_field("created_time"));
                            }
                            seen_created_time = true;
                            f_created_time = map.next_value::<Option<i64>>()?;
                        }
                        "is_hidden" => {
                            if seen_is_hidden {
                                return Err(serde::de::Error::duplicate_field("is_hidden"));
                            }
                            seen_is_hidden = true;
                            f_is_hidden = map.next_value::<Option<bool>>()?;
                        }
                        "last_registered_time" => {
                            if seen_last_registered_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "last_registered_time",
                                ));
                            }
                            seen_last_registered_time = true;
                            f_last_registered_time = map.next_value::<Option<i64>>()?;
                        }
                        "last_updated_time" => {
                            if seen_last_updated_time {
                                return Err(serde::de::Error::duplicate_field("last_updated_time"));
                            }
                            seen_last_updated_time = true;
                            f_last_updated_time = map.next_value::<Option<i64>>()?;
                        }
                        "model_config" => {
                            if seen_model_config {
                                return Err(serde::de::Error::duplicate_field("model_config"));
                            }
                            seen_model_config = true;
                            f_model_config =
                                map.next_value::<Option<crate::types::ml_common::ModelConfig>>()?;
                        }
                        "model_content_hash_value" => {
                            if seen_model_content_hash_value {
                                return Err(serde::de::Error::duplicate_field(
                                    "model_content_hash_value",
                                ));
                            }
                            seen_model_content_hash_value = true;
                            f_model_content_hash_value = map.next_value::<Option<String>>()?;
                        }
                        "model_content_size_in_bytes" => {
                            if seen_model_content_size_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "model_content_size_in_bytes",
                                ));
                            }
                            seen_model_content_size_in_bytes = true;
                            f_model_content_size_in_bytes = map.next_value::<Option<i64>>()?;
                        }
                        "model_format" => {
                            if seen_model_format {
                                return Err(serde::de::Error::duplicate_field("model_format"));
                            }
                            seen_model_format = true;
                            f_model_format =
                                map.next_value::<Option<crate::types::ml_common::ModelFormat>>()?;
                        }
                        "model_group_id" => {
                            if seen_model_group_id {
                                return Err(serde::de::Error::duplicate_field("model_group_id"));
                            }
                            seen_model_group_id = true;
                            f_model_group_id = map.next_value::<Option<String>>()?;
                        }
                        "model_state" => {
                            if f_model_state.is_some() {
                                return Err(serde::de::Error::duplicate_field("model_state"));
                            }
                            f_model_state =
                                Some(map.next_value::<crate::types::ml_common::ModelState>()?);
                        }
                        "model_version" => {
                            if seen_model_version {
                                return Err(serde::de::Error::duplicate_field("model_version"));
                            }
                            seen_model_version = true;
                            f_model_version = map.next_value::<Option<String>>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "total_chunks" => {
                            if seen_total_chunks {
                                return Err(serde::de::Error::duplicate_field("total_chunks"));
                            }
                            seen_total_chunks = true;
                            f_total_chunks = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Model {
                    algorithm: f_algorithm,
                    created_time: f_created_time,
                    is_hidden: f_is_hidden,
                    last_registered_time: f_last_registered_time,
                    last_updated_time: f_last_updated_time,
                    model_config: f_model_config,
                    model_content_hash_value: f_model_content_hash_value,
                    model_content_size_in_bytes: f_model_content_size_in_bytes,
                    model_format: f_model_format,
                    model_group_id: f_model_group_id,
                    model_state: f_model_state
                        .ok_or_else(|| serde::de::Error::missing_field("model_state"))?,
                    model_version: f_model_version,
                    name: f_name,
                    total_chunks: f_total_chunks,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Model {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.algorithm {
            map.serialize_entry("algorithm", v)?;
        }
        if let Some(v) = &self.created_time {
            map.serialize_entry("created_time", v)?;
        }
        if let Some(v) = &self.is_hidden {
            map.serialize_entry("is_hidden", v)?;
        }
        if let Some(v) = &self.last_registered_time {
            map.serialize_entry("last_registered_time", v)?;
        }
        if let Some(v) = &self.last_updated_time {
            map.serialize_entry("last_updated_time", v)?;
        }
        if let Some(v) = &self.model_config {
            map.serialize_entry("model_config", v)?;
        }
        if let Some(v) = &self.model_content_hash_value {
            map.serialize_entry("model_content_hash_value", v)?;
        }
        if let Some(v) = &self.model_content_size_in_bytes {
            map.serialize_entry("model_content_size_in_bytes", v)?;
        }
        if let Some(v) = &self.model_format {
            map.serialize_entry("model_format", v)?;
        }
        if let Some(v) = &self.model_group_id {
            map.serialize_entry("model_group_id", v)?;
        }
        map.serialize_entry("model_state", &self.model_state)?;
        if let Some(v) = &self.model_version {
            map.serialize_entry("model_version", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.total_chunks {
            map.serialize_entry("total_chunks", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ModelConfig {
    pub additional_config: Option<crate::types::ml_common::AdditionalConfig>,
    #[doc = "The all config."]
    pub all_config: Option<String>,
    #[doc = "The embedding dimension."]
    pub embedding_dimension: Option<i64>,
    #[doc = "The framework type."]
    pub framework_type: Option<String>,
    #[doc = "The model type."]
    pub model_type: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ModelConfig {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ModelConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ModelConfig object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_additional_config: Option<crate::types::ml_common::AdditionalConfig> =
                    None;
                let mut seen_additional_config = false;
                let mut f_all_config: Option<String> = None;
                let mut seen_all_config = false;
                let mut f_embedding_dimension: Option<i64> = None;
                let mut seen_embedding_dimension = false;
                let mut f_framework_type: Option<String> = None;
                let mut seen_framework_type = false;
                let mut f_model_type: Option<String> = None;
                let mut seen_model_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "additional_config" => {
                            if seen_additional_config {
                                return Err(serde::de::Error::duplicate_field("additional_config"));
                            }
                            seen_additional_config = true;
                            f_additional_config = map
                                .next_value::<Option<crate::types::ml_common::AdditionalConfig>>(
                                )?;
                        }
                        "all_config" => {
                            if seen_all_config {
                                return Err(serde::de::Error::duplicate_field("all_config"));
                            }
                            seen_all_config = true;
                            f_all_config = map.next_value::<Option<String>>()?;
                        }
                        "embedding_dimension" => {
                            if seen_embedding_dimension {
                                return Err(serde::de::Error::duplicate_field(
                                    "embedding_dimension",
                                ));
                            }
                            seen_embedding_dimension = true;
                            f_embedding_dimension = map.next_value::<Option<i64>>()?;
                        }
                        "framework_type" => {
                            if seen_framework_type {
                                return Err(serde::de::Error::duplicate_field("framework_type"));
                            }
                            seen_framework_type = true;
                            f_framework_type = map.next_value::<Option<String>>()?;
                        }
                        "model_type" => {
                            if seen_model_type {
                                return Err(serde::de::Error::duplicate_field("model_type"));
                            }
                            seen_model_type = true;
                            f_model_type = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ModelConfig {
                    additional_config: f_additional_config,
                    all_config: f_all_config,
                    embedding_dimension: f_embedding_dimension,
                    framework_type: f_framework_type,
                    model_type: f_model_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ModelConfig {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.additional_config {
            map.serialize_entry("additional_config", v)?;
        }
        if let Some(v) = &self.all_config {
            map.serialize_entry("all_config", v)?;
        }
        if let Some(v) = &self.embedding_dimension {
            map.serialize_entry("embedding_dimension", v)?;
        }
        if let Some(v) = &self.framework_type {
            map.serialize_entry("framework_type", v)?;
        }
        if let Some(v) = &self.model_type {
            map.serialize_entry("model_type", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModelFormat {
    #[doc = "`ONNX`"]
    Onnx,
    #[doc = "`TORCH_SCRIPT`"]
    TorchScript,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ModelFormat {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ModelFormat::Onnx => "ONNX",
            ModelFormat::TorchScript => "TORCH_SCRIPT",
            ModelFormat::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ModelFormat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ModelFormat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ONNX" => ModelFormat::Onnx,
            "TORCH_SCRIPT" => ModelFormat::TorchScript,
            _ => ModelFormat::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ModelGroup {
    pub access: crate::types::ml_common::ModelGroupAccessMode,
    pub created_time: Option<i64>,
    #[doc = "The model group description."]
    pub description: String,
    pub last_updated_time: Option<i64>,
    #[doc = "The latest version."]
    pub latest_version: i64,
    #[doc = "The model group name."]
    pub name: String,
    pub owner: Option<crate::types::ml_common::Owner>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ModelGroup {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ModelGroup;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ModelGroup object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_access: Option<crate::types::ml_common::ModelGroupAccessMode> = None;
                let mut f_created_time: Option<i64> = None;
                let mut seen_created_time = false;
                let mut f_description: Option<String> = None;
                let mut f_last_updated_time: Option<i64> = None;
                let mut seen_last_updated_time = false;
                let mut f_latest_version: Option<i64> = None;
                let mut f_name: Option<String> = None;
                let mut f_owner: Option<crate::types::ml_common::Owner> = None;
                let mut seen_owner = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "access" => {
                            if f_access.is_some() {
                                return Err(serde::de::Error::duplicate_field("access"));
                            }
                            f_access = Some(
                                map.next_value::<crate::types::ml_common::ModelGroupAccessMode>()?,
                            );
                        }
                        "created_time" => {
                            if seen_created_time {
                                return Err(serde::de::Error::duplicate_field("created_time"));
                            }
                            seen_created_time = true;
                            f_created_time = map.next_value::<Option<i64>>()?;
                        }
                        "description" => {
                            if f_description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            f_description = Some(map.next_value::<String>()?);
                        }
                        "last_updated_time" => {
                            if seen_last_updated_time {
                                return Err(serde::de::Error::duplicate_field("last_updated_time"));
                            }
                            seen_last_updated_time = true;
                            f_last_updated_time = map.next_value::<Option<i64>>()?;
                        }
                        "latest_version" => {
                            if f_latest_version.is_some() {
                                return Err(serde::de::Error::duplicate_field("latest_version"));
                            }
                            f_latest_version = Some(map.next_value::<i64>()?);
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<String>()?);
                        }
                        "owner" => {
                            if seen_owner {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            seen_owner = true;
                            f_owner = map.next_value::<Option<crate::types::ml_common::Owner>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ModelGroup {
                    access: f_access.ok_or_else(|| serde::de::Error::missing_field("access"))?,
                    created_time: f_created_time,
                    description: f_description
                        .ok_or_else(|| serde::de::Error::missing_field("description"))?,
                    last_updated_time: f_last_updated_time,
                    latest_version: f_latest_version
                        .ok_or_else(|| serde::de::Error::missing_field("latest_version"))?,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    owner: f_owner,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ModelGroup {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("access", &self.access)?;
        if let Some(v) = &self.created_time {
            map.serialize_entry("created_time", v)?;
        }
        map.serialize_entry("description", &self.description)?;
        if let Some(v) = &self.last_updated_time {
            map.serialize_entry("last_updated_time", v)?;
        }
        map.serialize_entry("latest_version", &self.latest_version)?;
        map.serialize_entry("name", &self.name)?;
        if let Some(v) = &self.owner {
            map.serialize_entry("owner", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The model group access mode."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModelGroupAccessMode {
    #[doc = "`private`"]
    Private,
    #[doc = "`public`"]
    Public,
    #[doc = "`restricted`"]
    Restricted,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ModelGroupAccessMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ModelGroupAccessMode::Private => "private",
            ModelGroupAccessMode::Public => "public",
            ModelGroupAccessMode::Restricted => "restricted",
            ModelGroupAccessMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ModelGroupAccessMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ModelGroupAccessMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "private" => ModelGroupAccessMode::Private,
            "public" => ModelGroupAccessMode::Public,
            "restricted" => ModelGroupAccessMode::Restricted,
            _ => ModelGroupAccessMode::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ModelGroupRegistration {
    #[doc = "The model group ID."]
    pub model_group_id: String,
    #[doc = "The status."]
    pub status: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ModelGroupRegistration {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ModelGroupRegistration;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ModelGroupRegistration object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_model_group_id: Option<String> = None;
                let mut f_status: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "model_group_id" => {
                            if f_model_group_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("model_group_id"));
                            }
                            f_model_group_id = Some(map.next_value::<String>()?);
                        }
                        "status" => {
                            if f_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            f_status = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ModelGroupRegistration {
                    model_group_id: f_model_group_id
                        .ok_or_else(|| serde::de::Error::missing_field("model_group_id"))?,
                    status: f_status.ok_or_else(|| serde::de::Error::missing_field("status"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ModelGroupRegistration {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("model_group_id", &self.model_group_id)?;
        map.serialize_entry("status", &self.status)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ModelProfile {
    pub deploy: Option<crate::types::ml_common::ModelStats>,
    pub execute: Option<crate::types::ml_common::ModelStats>,
    #[doc = "The estimated memory size in CPU."]
    pub memory_size_estimation_cpu: Option<i64>,
    #[doc = "The estimated memory size in GPU."]
    pub memory_size_estimation_gpu: Option<i64>,
    pub model_state: Option<crate::types::ml_common::ModelState>,
    pub predict: Option<crate::types::ml_common::ModelStats>,
    pub predict_request_stats: Option<crate::types::ml_common::PredictRequestStats>,
    #[doc = "The predictor."]
    pub predictor: Option<String>,
    pub register: Option<crate::types::ml_common::ModelStats>,
    pub target_worker_nodes: Option<Vec<crate::types::common::NodeIds>>,
    pub train: Option<crate::types::ml_common::ModelStats>,
    pub train_predict: Option<crate::types::ml_common::ModelStats>,
    pub undeploy: Option<crate::types::ml_common::ModelStats>,
    pub worker_nodes: Option<Vec<crate::types::common::NodeIds>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ModelProfile {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ModelProfile;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ModelProfile object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_deploy: Option<crate::types::ml_common::ModelStats> = None;
                let mut seen_deploy = false;
                let mut f_execute: Option<crate::types::ml_common::ModelStats> = None;
                let mut seen_execute = false;
                let mut f_memory_size_estimation_cpu: Option<i64> = None;
                let mut seen_memory_size_estimation_cpu = false;
                let mut f_memory_size_estimation_gpu: Option<i64> = None;
                let mut seen_memory_size_estimation_gpu = false;
                let mut f_model_state: Option<crate::types::ml_common::ModelState> = None;
                let mut seen_model_state = false;
                let mut f_predict: Option<crate::types::ml_common::ModelStats> = None;
                let mut seen_predict = false;
                let mut f_predict_request_stats: Option<
                    crate::types::ml_common::PredictRequestStats,
                > = None;
                let mut seen_predict_request_stats = false;
                let mut f_predictor: Option<String> = None;
                let mut seen_predictor = false;
                let mut f_register: Option<crate::types::ml_common::ModelStats> = None;
                let mut seen_register = false;
                let mut f_target_worker_nodes: Option<Vec<crate::types::common::NodeIds>> = None;
                let mut seen_target_worker_nodes = false;
                let mut f_train: Option<crate::types::ml_common::ModelStats> = None;
                let mut seen_train = false;
                let mut f_train_predict: Option<crate::types::ml_common::ModelStats> = None;
                let mut seen_train_predict = false;
                let mut f_undeploy: Option<crate::types::ml_common::ModelStats> = None;
                let mut seen_undeploy = false;
                let mut f_worker_nodes: Option<Vec<crate::types::common::NodeIds>> = None;
                let mut seen_worker_nodes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "deploy" => {
                            if seen_deploy {
                                return Err(serde::de::Error::duplicate_field("deploy"));
                            }
                            seen_deploy = true;
                            f_deploy =
                                map.next_value::<Option<crate::types::ml_common::ModelStats>>()?;
                        }
                        "execute" => {
                            if seen_execute {
                                return Err(serde::de::Error::duplicate_field("execute"));
                            }
                            seen_execute = true;
                            f_execute =
                                map.next_value::<Option<crate::types::ml_common::ModelStats>>()?;
                        }
                        "memory_size_estimation_cpu" => {
                            if seen_memory_size_estimation_cpu {
                                return Err(serde::de::Error::duplicate_field(
                                    "memory_size_estimation_cpu",
                                ));
                            }
                            seen_memory_size_estimation_cpu = true;
                            f_memory_size_estimation_cpu = map.next_value::<Option<i64>>()?;
                        }
                        "memory_size_estimation_gpu" => {
                            if seen_memory_size_estimation_gpu {
                                return Err(serde::de::Error::duplicate_field(
                                    "memory_size_estimation_gpu",
                                ));
                            }
                            seen_memory_size_estimation_gpu = true;
                            f_memory_size_estimation_gpu = map.next_value::<Option<i64>>()?;
                        }
                        "model_state" => {
                            if seen_model_state {
                                return Err(serde::de::Error::duplicate_field("model_state"));
                            }
                            seen_model_state = true;
                            f_model_state =
                                map.next_value::<Option<crate::types::ml_common::ModelState>>()?;
                        }
                        "predict" => {
                            if seen_predict {
                                return Err(serde::de::Error::duplicate_field("predict"));
                            }
                            seen_predict = true;
                            f_predict =
                                map.next_value::<Option<crate::types::ml_common::ModelStats>>()?;
                        }
                        "predict_request_stats" => {
                            if seen_predict_request_stats {
                                return Err(serde::de::Error::duplicate_field(
                                    "predict_request_stats",
                                ));
                            }
                            seen_predict_request_stats = true;
                            f_predict_request_stats = map
                                .next_value::<Option<crate::types::ml_common::PredictRequestStats>>(
                                )?;
                        }
                        "predictor" => {
                            if seen_predictor {
                                return Err(serde::de::Error::duplicate_field("predictor"));
                            }
                            seen_predictor = true;
                            f_predictor = map.next_value::<Option<String>>()?;
                        }
                        "register" => {
                            if seen_register {
                                return Err(serde::de::Error::duplicate_field("register"));
                            }
                            seen_register = true;
                            f_register =
                                map.next_value::<Option<crate::types::ml_common::ModelStats>>()?;
                        }
                        "target_worker_nodes" => {
                            if seen_target_worker_nodes {
                                return Err(serde::de::Error::duplicate_field(
                                    "target_worker_nodes",
                                ));
                            }
                            seen_target_worker_nodes = true;
                            f_target_worker_nodes =
                                map.next_value::<Option<Vec<crate::types::common::NodeIds>>>()?;
                        }
                        "train" => {
                            if seen_train {
                                return Err(serde::de::Error::duplicate_field("train"));
                            }
                            seen_train = true;
                            f_train =
                                map.next_value::<Option<crate::types::ml_common::ModelStats>>()?;
                        }
                        "train_predict" => {
                            if seen_train_predict {
                                return Err(serde::de::Error::duplicate_field("train_predict"));
                            }
                            seen_train_predict = true;
                            f_train_predict =
                                map.next_value::<Option<crate::types::ml_common::ModelStats>>()?;
                        }
                        "undeploy" => {
                            if seen_undeploy {
                                return Err(serde::de::Error::duplicate_field("undeploy"));
                            }
                            seen_undeploy = true;
                            f_undeploy =
                                map.next_value::<Option<crate::types::ml_common::ModelStats>>()?;
                        }
                        "worker_nodes" => {
                            if seen_worker_nodes {
                                return Err(serde::de::Error::duplicate_field("worker_nodes"));
                            }
                            seen_worker_nodes = true;
                            f_worker_nodes =
                                map.next_value::<Option<Vec<crate::types::common::NodeIds>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ModelProfile {
                    deploy: f_deploy,
                    execute: f_execute,
                    memory_size_estimation_cpu: f_memory_size_estimation_cpu,
                    memory_size_estimation_gpu: f_memory_size_estimation_gpu,
                    model_state: f_model_state,
                    predict: f_predict,
                    predict_request_stats: f_predict_request_stats,
                    predictor: f_predictor,
                    register: f_register,
                    target_worker_nodes: f_target_worker_nodes,
                    train: f_train,
                    train_predict: f_train_predict,
                    undeploy: f_undeploy,
                    worker_nodes: f_worker_nodes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ModelProfile {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.deploy {
            map.serialize_entry("deploy", v)?;
        }
        if let Some(v) = &self.execute {
            map.serialize_entry("execute", v)?;
        }
        if let Some(v) = &self.memory_size_estimation_cpu {
            map.serialize_entry("memory_size_estimation_cpu", v)?;
        }
        if let Some(v) = &self.memory_size_estimation_gpu {
            map.serialize_entry("memory_size_estimation_gpu", v)?;
        }
        if let Some(v) = &self.model_state {
            map.serialize_entry("model_state", v)?;
        }
        if let Some(v) = &self.predict {
            map.serialize_entry("predict", v)?;
        }
        if let Some(v) = &self.predict_request_stats {
            map.serialize_entry("predict_request_stats", v)?;
        }
        if let Some(v) = &self.predictor {
            map.serialize_entry("predictor", v)?;
        }
        if let Some(v) = &self.register {
            map.serialize_entry("register", v)?;
        }
        if let Some(v) = &self.target_worker_nodes {
            map.serialize_entry("target_worker_nodes", v)?;
        }
        if let Some(v) = &self.train {
            map.serialize_entry("train", v)?;
        }
        if let Some(v) = &self.train_predict {
            map.serialize_entry("train_predict", v)?;
        }
        if let Some(v) = &self.undeploy {
            map.serialize_entry("undeploy", v)?;
        }
        if let Some(v) = &self.worker_nodes {
            map.serialize_entry("worker_nodes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The model state."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModelState {
    #[doc = "`DEPLOYED`"]
    Deployed,
    #[doc = "`DEPLOYING`"]
    Deploying,
    #[doc = "`DEPLOY_FAILED`"]
    DeployFailed,
    #[doc = "`PARTIALLY_DEPLOYED`"]
    PartiallyDeployed,
    #[doc = "`REGISTERED`"]
    Registered,
    #[doc = "`REGISTERING`"]
    Registering,
    #[doc = "`UNDEPLOYED`"]
    Undeployed,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ModelState {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ModelState::Deployed => "DEPLOYED",
            ModelState::Deploying => "DEPLOYING",
            ModelState::DeployFailed => "DEPLOY_FAILED",
            ModelState::PartiallyDeployed => "PARTIALLY_DEPLOYED",
            ModelState::Registered => "REGISTERED",
            ModelState::Registering => "REGISTERING",
            ModelState::Undeployed => "UNDEPLOYED",
            ModelState::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ModelState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ModelState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "DEPLOYED" => ModelState::Deployed,
            "DEPLOYING" => ModelState::Deploying,
            "DEPLOY_FAILED" => ModelState::DeployFailed,
            "PARTIALLY_DEPLOYED" => ModelState::PartiallyDeployed,
            "REGISTERED" => ModelState::Registered,
            "REGISTERING" => ModelState::Registering,
            "UNDEPLOYED" => ModelState::Undeployed,
            _ => ModelState::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ModelStats {
    #[doc = "The failure count."]
    pub ml_action_failure_count: Option<i64>,
    #[doc = "The request count."]
    pub ml_action_request_count: Option<i64>,
    #[doc = "The executing task count."]
    pub ml_executing_task_count: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ModelStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ModelStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ModelStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_ml_action_failure_count: Option<i64> = None;
                let mut seen_ml_action_failure_count = false;
                let mut f_ml_action_request_count: Option<i64> = None;
                let mut seen_ml_action_request_count = false;
                let mut f_ml_executing_task_count: Option<i64> = None;
                let mut seen_ml_executing_task_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "ml_action_failure_count" => {
                            if seen_ml_action_failure_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "ml_action_failure_count",
                                ));
                            }
                            seen_ml_action_failure_count = true;
                            f_ml_action_failure_count = map.next_value::<Option<i64>>()?;
                        }
                        "ml_action_request_count" => {
                            if seen_ml_action_request_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "ml_action_request_count",
                                ));
                            }
                            seen_ml_action_request_count = true;
                            f_ml_action_request_count = map.next_value::<Option<i64>>()?;
                        }
                        "ml_executing_task_count" => {
                            if seen_ml_executing_task_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "ml_executing_task_count",
                                ));
                            }
                            seen_ml_executing_task_count = true;
                            f_ml_executing_task_count = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ModelStats {
                    ml_action_failure_count: f_ml_action_failure_count,
                    ml_action_request_count: f_ml_action_request_count,
                    ml_executing_task_count: f_ml_executing_task_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ModelStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.ml_action_failure_count {
            map.serialize_entry("ml_action_failure_count", v)?;
        }
        if let Some(v) = &self.ml_action_request_count {
            map.serialize_entry("ml_action_request_count", v)?;
        }
        if let Some(v) = &self.ml_executing_task_count {
            map.serialize_entry("ml_executing_task_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type Models = std::collections::BTreeMap<String, crate::types::ml_common::ModelProfile>;

pub type Namespace = std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

#[derive(Debug, Clone)]
pub struct Node {
    pub models: Option<crate::types::ml_common::Models>,
    pub tasks: Option<crate::types::ml_common::Tasks>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Node;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Node object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_models: Option<crate::types::ml_common::Models> = None;
                let mut seen_models = false;
                let mut f_tasks: Option<crate::types::ml_common::Tasks> = None;
                let mut seen_tasks = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "models" => {
                            if seen_models {
                                return Err(serde::de::Error::duplicate_field("models"));
                            }
                            seen_models = true;
                            f_models =
                                map.next_value::<Option<crate::types::ml_common::Models>>()?;
                        }
                        "tasks" => {
                            if seen_tasks {
                                return Err(serde::de::Error::duplicate_field("tasks"));
                            }
                            seen_tasks = true;
                            f_tasks = map.next_value::<Option<crate::types::ml_common::Tasks>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Node {
                    models: f_models,
                    tasks: f_tasks,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.models {
            map.serialize_entry("models", v)?;
        }
        if let Some(v) = &self.tasks {
            map.serialize_entry("tasks", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type NodeStats = std::collections::BTreeMap<String, crate::types::ml_common::NodeStatsDetails>;

#[derive(Debug, Clone)]
pub struct NodeStatsDetails {
    pub algorithms: Option<crate::types::ml_common::Algorithms>,
    #[doc = "The circuit breaker trigger count."]
    pub ml_circuit_breaker_trigger_count: Option<i64>,
    #[doc = "The deployed model count."]
    pub ml_deployed_model_count: Option<i64>,
    #[doc = "The executing task count."]
    pub ml_executing_task_count: Option<i64>,
    #[doc = "The failure count."]
    pub ml_failure_count: Option<i64>,
    #[doc = "The JVM heap usage."]
    pub ml_jvm_heap_usage: Option<i64>,
    #[doc = "The request count."]
    pub ml_request_count: Option<i64>,
    pub models: Option<crate::types::ml_common::Models>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeStatsDetails {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeStatsDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeStatsDetails object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_algorithms: Option<crate::types::ml_common::Algorithms> = None;
                let mut seen_algorithms = false;
                let mut f_ml_circuit_breaker_trigger_count: Option<i64> = None;
                let mut seen_ml_circuit_breaker_trigger_count = false;
                let mut f_ml_deployed_model_count: Option<i64> = None;
                let mut seen_ml_deployed_model_count = false;
                let mut f_ml_executing_task_count: Option<i64> = None;
                let mut seen_ml_executing_task_count = false;
                let mut f_ml_failure_count: Option<i64> = None;
                let mut seen_ml_failure_count = false;
                let mut f_ml_jvm_heap_usage: Option<i64> = None;
                let mut seen_ml_jvm_heap_usage = false;
                let mut f_ml_request_count: Option<i64> = None;
                let mut seen_ml_request_count = false;
                let mut f_models: Option<crate::types::ml_common::Models> = None;
                let mut seen_models = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "algorithms" => {
                            if seen_algorithms {
                                return Err(serde::de::Error::duplicate_field("algorithms"));
                            }
                            seen_algorithms = true;
                            f_algorithms =
                                map.next_value::<Option<crate::types::ml_common::Algorithms>>()?;
                        }
                        "ml_circuit_breaker_trigger_count" => {
                            if seen_ml_circuit_breaker_trigger_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "ml_circuit_breaker_trigger_count",
                                ));
                            }
                            seen_ml_circuit_breaker_trigger_count = true;
                            f_ml_circuit_breaker_trigger_count = map.next_value::<Option<i64>>()?;
                        }
                        "ml_deployed_model_count" => {
                            if seen_ml_deployed_model_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "ml_deployed_model_count",
                                ));
                            }
                            seen_ml_deployed_model_count = true;
                            f_ml_deployed_model_count = map.next_value::<Option<i64>>()?;
                        }
                        "ml_executing_task_count" => {
                            if seen_ml_executing_task_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "ml_executing_task_count",
                                ));
                            }
                            seen_ml_executing_task_count = true;
                            f_ml_executing_task_count = map.next_value::<Option<i64>>()?;
                        }
                        "ml_failure_count" => {
                            if seen_ml_failure_count {
                                return Err(serde::de::Error::duplicate_field("ml_failure_count"));
                            }
                            seen_ml_failure_count = true;
                            f_ml_failure_count = map.next_value::<Option<i64>>()?;
                        }
                        "ml_jvm_heap_usage" => {
                            if seen_ml_jvm_heap_usage {
                                return Err(serde::de::Error::duplicate_field("ml_jvm_heap_usage"));
                            }
                            seen_ml_jvm_heap_usage = true;
                            f_ml_jvm_heap_usage = map.next_value::<Option<i64>>()?;
                        }
                        "ml_request_count" => {
                            if seen_ml_request_count {
                                return Err(serde::de::Error::duplicate_field("ml_request_count"));
                            }
                            seen_ml_request_count = true;
                            f_ml_request_count = map.next_value::<Option<i64>>()?;
                        }
                        "models" => {
                            if seen_models {
                                return Err(serde::de::Error::duplicate_field("models"));
                            }
                            seen_models = true;
                            f_models =
                                map.next_value::<Option<crate::types::ml_common::Models>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeStatsDetails {
                    algorithms: f_algorithms,
                    ml_circuit_breaker_trigger_count: f_ml_circuit_breaker_trigger_count,
                    ml_deployed_model_count: f_ml_deployed_model_count,
                    ml_executing_task_count: f_ml_executing_task_count,
                    ml_failure_count: f_ml_failure_count,
                    ml_jvm_heap_usage: f_ml_jvm_heap_usage,
                    ml_request_count: f_ml_request_count,
                    models: f_models,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeStatsDetails {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.algorithms {
            map.serialize_entry("algorithms", v)?;
        }
        if let Some(v) = &self.ml_circuit_breaker_trigger_count {
            map.serialize_entry("ml_circuit_breaker_trigger_count", v)?;
        }
        if let Some(v) = &self.ml_deployed_model_count {
            map.serialize_entry("ml_deployed_model_count", v)?;
        }
        if let Some(v) = &self.ml_executing_task_count {
            map.serialize_entry("ml_executing_task_count", v)?;
        }
        if let Some(v) = &self.ml_failure_count {
            map.serialize_entry("ml_failure_count", v)?;
        }
        if let Some(v) = &self.ml_jvm_heap_usage {
            map.serialize_entry("ml_jvm_heap_usage", v)?;
        }
        if let Some(v) = &self.ml_request_count {
            map.serialize_entry("ml_request_count", v)?;
        }
        if let Some(v) = &self.models {
            map.serialize_entry("models", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type Nodes = std::collections::BTreeMap<String, crate::types::ml_common::Node>;

#[derive(Debug, Clone)]
pub struct Output {
    pub byte_buffer: Option<crate::types::ml_common::ByteBuffer>,
    pub data: Option<Vec<f64>>,
    pub data_as_map: Option<crate::types::ml_common::DataAsMap>,
    pub data_type: Option<crate::types::ml_common::MlResultDataType>,
    #[doc = "The output name."]
    pub name: Option<String>,
    #[doc = "The output result."]
    pub result: Option<String>,
    pub shape: Option<Vec<i64>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Output {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Output;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Output object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_byte_buffer: Option<crate::types::ml_common::ByteBuffer> = None;
                let mut seen_byte_buffer = false;
                let mut f_data: Option<Vec<f64>> = None;
                let mut seen_data = false;
                let mut f_data_as_map: Option<crate::types::ml_common::DataAsMap> = None;
                let mut seen_data_as_map = false;
                let mut f_data_type: Option<crate::types::ml_common::MlResultDataType> = None;
                let mut seen_data_type = false;
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_result: Option<String> = None;
                let mut seen_result = false;
                let mut f_shape: Option<Vec<i64>> = None;
                let mut seen_shape = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "byte_buffer" => {
                            if seen_byte_buffer {
                                return Err(serde::de::Error::duplicate_field("byte_buffer"));
                            }
                            seen_byte_buffer = true;
                            f_byte_buffer =
                                map.next_value::<Option<crate::types::ml_common::ByteBuffer>>()?;
                        }
                        "data" => {
                            if seen_data {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            seen_data = true;
                            f_data = map.next_value::<Option<Vec<f64>>>()?;
                        }
                        "dataAsMap" => {
                            if seen_data_as_map {
                                return Err(serde::de::Error::duplicate_field("dataAsMap"));
                            }
                            seen_data_as_map = true;
                            f_data_as_map =
                                map.next_value::<Option<crate::types::ml_common::DataAsMap>>()?;
                        }
                        "data_type" => {
                            if seen_data_type {
                                return Err(serde::de::Error::duplicate_field("data_type"));
                            }
                            seen_data_type = true;
                            f_data_type = map
                                .next_value::<Option<crate::types::ml_common::MlResultDataType>>(
                                )?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "result" => {
                            if seen_result {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            seen_result = true;
                            f_result = map.next_value::<Option<String>>()?;
                        }
                        "shape" => {
                            if seen_shape {
                                return Err(serde::de::Error::duplicate_field("shape"));
                            }
                            seen_shape = true;
                            f_shape = map.next_value::<Option<Vec<i64>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Output {
                    byte_buffer: f_byte_buffer,
                    data: f_data,
                    data_as_map: f_data_as_map,
                    data_type: f_data_type,
                    name: f_name,
                    result: f_result,
                    shape: f_shape,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Output {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.byte_buffer {
            map.serialize_entry("byte_buffer", v)?;
        }
        if let Some(v) = &self.data {
            map.serialize_entry("data", v)?;
        }
        if let Some(v) = &self.data_as_map {
            map.serialize_entry("dataAsMap", v)?;
        }
        if let Some(v) = &self.data_type {
            map.serialize_entry("data_type", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.result {
            map.serialize_entry("result", v)?;
        }
        if let Some(v) = &self.shape {
            map.serialize_entry("shape", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Owner {
    #[doc = "The backend roles."]
    pub backend_roles: Option<Vec<String>>,
    #[doc = "The custom attribute names."]
    pub custom_attribute_names: Option<Vec<String>>,
    pub name: crate::types::common::Name,
    #[doc = "The roles."]
    pub roles: Option<Vec<String>>,
    #[doc = "The user requested tenant."]
    pub user_requested_tenant: Option<String>,
    #[doc = "The user requested tenant access."]
    pub user_requested_tenant_access: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Owner {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Owner;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Owner object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_backend_roles: Option<Vec<String>> = None;
                let mut seen_backend_roles = false;
                let mut f_custom_attribute_names: Option<Vec<String>> = None;
                let mut seen_custom_attribute_names = false;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut f_roles: Option<Vec<String>> = None;
                let mut seen_roles = false;
                let mut f_user_requested_tenant: Option<String> = None;
                let mut seen_user_requested_tenant = false;
                let mut f_user_requested_tenant_access: Option<String> = None;
                let mut seen_user_requested_tenant_access = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "backend_roles" => {
                            if seen_backend_roles {
                                return Err(serde::de::Error::duplicate_field("backend_roles"));
                            }
                            seen_backend_roles = true;
                            f_backend_roles = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "custom_attribute_names" => {
                            if seen_custom_attribute_names {
                                return Err(serde::de::Error::duplicate_field(
                                    "custom_attribute_names",
                                ));
                            }
                            seen_custom_attribute_names = true;
                            f_custom_attribute_names = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        "roles" => {
                            if seen_roles {
                                return Err(serde::de::Error::duplicate_field("roles"));
                            }
                            seen_roles = true;
                            f_roles = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "user_requested_tenant" => {
                            if seen_user_requested_tenant {
                                return Err(serde::de::Error::duplicate_field(
                                    "user_requested_tenant",
                                ));
                            }
                            seen_user_requested_tenant = true;
                            f_user_requested_tenant = map.next_value::<Option<String>>()?;
                        }
                        "user_requested_tenant_access" => {
                            if seen_user_requested_tenant_access {
                                return Err(serde::de::Error::duplicate_field(
                                    "user_requested_tenant_access",
                                ));
                            }
                            seen_user_requested_tenant_access = true;
                            f_user_requested_tenant_access = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Owner {
                    backend_roles: f_backend_roles,
                    custom_attribute_names: f_custom_attribute_names,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    roles: f_roles,
                    user_requested_tenant: f_user_requested_tenant,
                    user_requested_tenant_access: f_user_requested_tenant_access,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Owner {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.backend_roles {
            map.serialize_entry("backend_roles", v)?;
        }
        if let Some(v) = &self.custom_attribute_names {
            map.serialize_entry("custom_attribute_names", v)?;
        }
        map.serialize_entry("name", &self.name)?;
        if let Some(v) = &self.roles {
            map.serialize_entry("roles", v)?;
        }
        if let Some(v) = &self.user_requested_tenant {
            map.serialize_entry("user_requested_tenant", v)?;
        }
        if let Some(v) = &self.user_requested_tenant_access {
            map.serialize_entry("user_requested_tenant_access", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Parameters {
    pub llm_interface: Option<String>,
    pub inputs: Option<String>,
    pub messages: Option<Vec<crate::types::ml_common::Messages>>,
    pub question: Option<String>,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Parameters {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Parameters;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Parameters object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_llm_interface: Option<String> = None;
                let mut seen_llm_interface = false;
                let mut f_inputs: Option<String> = None;
                let mut seen_inputs = false;
                let mut f_messages: Option<Vec<crate::types::ml_common::Messages>> = None;
                let mut seen_messages = false;
                let mut f_question: Option<String> = None;
                let mut seen_question = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_llm_interface" => {
                            if seen_llm_interface {
                                return Err(serde::de::Error::duplicate_field("_llm_interface"));
                            }
                            seen_llm_interface = true;
                            f_llm_interface = map.next_value::<Option<String>>()?;
                        }
                        "inputs" => {
                            if seen_inputs {
                                return Err(serde::de::Error::duplicate_field("inputs"));
                            }
                            seen_inputs = true;
                            f_inputs = map.next_value::<Option<String>>()?;
                        }
                        "messages" => {
                            if seen_messages {
                                return Err(serde::de::Error::duplicate_field("messages"));
                            }
                            seen_messages = true;
                            f_messages =
                                map.next_value::<Option<Vec<crate::types::ml_common::Messages>>>()?;
                        }
                        "question" => {
                            if seen_question {
                                return Err(serde::de::Error::duplicate_field("question"));
                            }
                            seen_question = true;
                            f_question = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Parameters {
                    llm_interface: f_llm_interface,
                    inputs: f_inputs,
                    messages: f_messages,
                    question: f_question,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Parameters {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.llm_interface {
            map.serialize_entry("_llm_interface", v)?;
        }
        if let Some(v) = &self.inputs {
            map.serialize_entry("inputs", v)?;
        }
        if let Some(v) = &self.messages {
            map.serialize_entry("messages", v)?;
        }
        if let Some(v) = &self.question {
            map.serialize_entry("question", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The type of payload."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PayloadType {
    #[doc = "`conversational`"]
    Conversational,
    #[doc = "`data`"]
    Data,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PayloadType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PayloadType::Conversational => "conversational",
            PayloadType::Data => "data",
            PayloadType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PayloadType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PayloadType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "conversational" => PayloadType::Conversational,
            "data" => PayloadType::Data,
            _ => PayloadType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct PredictModelOutput {
    pub byte_buffer: Option<crate::types::ml_common::ByteBuffer>,
    pub data: Vec<f64>,
    pub data_type: Option<crate::types::ml_common::MlResultDataType>,
    #[doc = "The output name."]
    pub name: Option<String>,
    pub shape: Option<Vec<i64>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PredictModelOutput {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PredictModelOutput;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PredictModelOutput object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_byte_buffer: Option<crate::types::ml_common::ByteBuffer> = None;
                let mut seen_byte_buffer = false;
                let mut f_data: Option<Vec<f64>> = None;
                let mut f_data_type: Option<crate::types::ml_common::MlResultDataType> = None;
                let mut seen_data_type = false;
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_shape: Option<Vec<i64>> = None;
                let mut seen_shape = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "byte_buffer" => {
                            if seen_byte_buffer {
                                return Err(serde::de::Error::duplicate_field("byte_buffer"));
                            }
                            seen_byte_buffer = true;
                            f_byte_buffer =
                                map.next_value::<Option<crate::types::ml_common::ByteBuffer>>()?;
                        }
                        "data" => {
                            if f_data.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            f_data = Some(map.next_value::<Vec<f64>>()?);
                        }
                        "data_type" => {
                            if seen_data_type {
                                return Err(serde::de::Error::duplicate_field("data_type"));
                            }
                            seen_data_type = true;
                            f_data_type = map
                                .next_value::<Option<crate::types::ml_common::MlResultDataType>>(
                                )?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "shape" => {
                            if seen_shape {
                                return Err(serde::de::Error::duplicate_field("shape"));
                            }
                            seen_shape = true;
                            f_shape = map.next_value::<Option<Vec<i64>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PredictModelOutput {
                    byte_buffer: f_byte_buffer,
                    data: f_data.ok_or_else(|| serde::de::Error::missing_field("data"))?,
                    data_type: f_data_type,
                    name: f_name,
                    shape: f_shape,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PredictModelOutput {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.byte_buffer {
            map.serialize_entry("byte_buffer", v)?;
        }
        map.serialize_entry("data", &self.data)?;
        if let Some(v) = &self.data_type {
            map.serialize_entry("data_type", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.shape {
            map.serialize_entry("shape", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PredictModelResponse {
    pub inference_results: Option<Vec<crate::types::ml_common::PredictModelResult>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PredictModelResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PredictModelResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PredictModelResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_inference_results: Option<
                    Vec<crate::types::ml_common::PredictModelResult>,
                > = None;
                let mut seen_inference_results = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "inference_results" => {
                            if seen_inference_results {
                                return Err(serde::de::Error::duplicate_field("inference_results"));
                            }
                            seen_inference_results = true;
                            f_inference_results = map.next_value::<Option<Vec<crate::types::ml_common::PredictModelResult>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PredictModelResponse {
                    inference_results: f_inference_results,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PredictModelResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.inference_results {
            map.serialize_entry("inference_results", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PredictModelResult {
    pub output: Option<Vec<crate::types::ml_common::PredictModelOutput>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PredictModelResult {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PredictModelResult;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PredictModelResult object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_output: Option<Vec<crate::types::ml_common::PredictModelOutput>> = None;
                let mut seen_output = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "output" => {
                            if seen_output {
                                return Err(serde::de::Error::duplicate_field("output"));
                            }
                            seen_output = true;
                            f_output = map.next_value::<Option<Vec<crate::types::ml_common::PredictModelOutput>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PredictModelResult {
                    output: f_output,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PredictModelResult {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.output {
            map.serialize_entry("output", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PredictRequestStats {
    #[doc = "The average latency in milliseconds."]
    pub average: Option<f64>,
    #[doc = "The total predict requests on this node."]
    pub count: Option<i64>,
    #[doc = "The maximum latency in milliseconds."]
    pub max: Option<f64>,
    #[doc = "The minimum latency in milliseconds."]
    pub min: Option<f64>,
    #[doc = "The 50th percentile latency in milliseconds."]
    pub p_50: Option<f64>,
    #[doc = "The 90th percentile latency in milliseconds."]
    pub p_90: Option<f64>,
    #[doc = "The 99th percentile latency in milliseconds."]
    pub p_99: Option<f64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PredictRequestStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PredictRequestStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PredictRequestStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_average: Option<f64> = None;
                let mut seen_average = false;
                let mut f_count: Option<i64> = None;
                let mut seen_count = false;
                let mut f_max: Option<f64> = None;
                let mut seen_max = false;
                let mut f_min: Option<f64> = None;
                let mut seen_min = false;
                let mut f_p_50: Option<f64> = None;
                let mut seen_p_50 = false;
                let mut f_p_90: Option<f64> = None;
                let mut seen_p_90 = false;
                let mut f_p_99: Option<f64> = None;
                let mut seen_p_99 = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "average" => {
                            if seen_average {
                                return Err(serde::de::Error::duplicate_field("average"));
                            }
                            seen_average = true;
                            f_average = map.next_value::<Option<f64>>()?;
                        }
                        "count" => {
                            if seen_count {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            seen_count = true;
                            f_count = map.next_value::<Option<i64>>()?;
                        }
                        "max" => {
                            if seen_max {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            seen_max = true;
                            f_max = map.next_value::<Option<f64>>()?;
                        }
                        "min" => {
                            if seen_min {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            seen_min = true;
                            f_min = map.next_value::<Option<f64>>()?;
                        }
                        "p50" => {
                            if seen_p_50 {
                                return Err(serde::de::Error::duplicate_field("p50"));
                            }
                            seen_p_50 = true;
                            f_p_50 = map.next_value::<Option<f64>>()?;
                        }
                        "p90" => {
                            if seen_p_90 {
                                return Err(serde::de::Error::duplicate_field("p90"));
                            }
                            seen_p_90 = true;
                            f_p_90 = map.next_value::<Option<f64>>()?;
                        }
                        "p99" => {
                            if seen_p_99 {
                                return Err(serde::de::Error::duplicate_field("p99"));
                            }
                            seen_p_99 = true;
                            f_p_99 = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PredictRequestStats {
                    average: f_average,
                    count: f_count,
                    max: f_max,
                    min: f_min,
                    p_50: f_p_50,
                    p_90: f_p_90,
                    p_99: f_p_99,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PredictRequestStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.average {
            map.serialize_entry("average", v)?;
        }
        if let Some(v) = &self.count {
            map.serialize_entry("count", v)?;
        }
        if let Some(v) = &self.max {
            map.serialize_entry("max", v)?;
        }
        if let Some(v) = &self.min {
            map.serialize_entry("min", v)?;
        }
        if let Some(v) = &self.p_50 {
            map.serialize_entry("p50", v)?;
        }
        if let Some(v) = &self.p_90 {
            map.serialize_entry("p90", v)?;
        }
        if let Some(v) = &self.p_99 {
            map.serialize_entry("p99", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PredictResponse {
    pub inference_results: Option<Vec<crate::types::ml_common::InferenceResults>>,
    pub prediction_result: Option<crate::types::ml_common::PredictionResult>,
    pub status: Option<crate::types::ml_common::Status>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PredictResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PredictResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PredictResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_inference_results: Option<
                    Vec<crate::types::ml_common::InferenceResults>,
                > = None;
                let mut seen_inference_results = false;
                let mut f_prediction_result: Option<crate::types::ml_common::PredictionResult> =
                    None;
                let mut seen_prediction_result = false;
                let mut f_status: Option<crate::types::ml_common::Status> = None;
                let mut seen_status = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "inference_results" => {
                            if seen_inference_results {
                                return Err(serde::de::Error::duplicate_field("inference_results"));
                            }
                            seen_inference_results = true;
                            f_inference_results = map.next_value::<Option<Vec<crate::types::ml_common::InferenceResults>>>()?;
                        }
                        "prediction_result" => {
                            if seen_prediction_result {
                                return Err(serde::de::Error::duplicate_field("prediction_result"));
                            }
                            seen_prediction_result = true;
                            f_prediction_result = map
                                .next_value::<Option<crate::types::ml_common::PredictionResult>>(
                                )?;
                        }
                        "status" => {
                            if seen_status {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            seen_status = true;
                            f_status =
                                map.next_value::<Option<crate::types::ml_common::Status>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PredictResponse {
                    inference_results: f_inference_results,
                    prediction_result: f_prediction_result,
                    status: f_status,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PredictResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.inference_results {
            map.serialize_entry("inference_results", v)?;
        }
        if let Some(v) = &self.prediction_result {
            map.serialize_entry("prediction_result", v)?;
        }
        if let Some(v) = &self.status {
            map.serialize_entry("status", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PredictionResult {
    pub column_metas: Option<Vec<crate::types::ml_common::ColumnMeta>>,
    pub rows: Option<Vec<crate::types::ml_common::Rows>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PredictionResult {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PredictionResult;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PredictionResult object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_column_metas: Option<Vec<crate::types::ml_common::ColumnMeta>> = None;
                let mut seen_column_metas = false;
                let mut f_rows: Option<Vec<crate::types::ml_common::Rows>> = None;
                let mut seen_rows = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "column_metas" => {
                            if seen_column_metas {
                                return Err(serde::de::Error::duplicate_field("column_metas"));
                            }
                            seen_column_metas = true;
                            f_column_metas = map
                                .next_value::<Option<Vec<crate::types::ml_common::ColumnMeta>>>()?;
                        }
                        "rows" => {
                            if seen_rows {
                                return Err(serde::de::Error::duplicate_field("rows"));
                            }
                            seen_rows = true;
                            f_rows =
                                map.next_value::<Option<Vec<crate::types::ml_common::Rows>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PredictionResult {
                    column_metas: f_column_metas,
                    rows: f_rows,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PredictionResult {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.column_metas {
            map.serialize_entry("column_metas", v)?;
        }
        if let Some(v) = &self.rows {
            map.serialize_entry("rows", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RateLimiter {
    #[doc = "The maximum limit."]
    pub limit: crate::types::common::StringifiedDouble,
    #[doc = "The unit of time."]
    pub unit: crate::types::ml_common::RateLimiterUnit,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RateLimiter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RateLimiter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RateLimiter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_limit: Option<crate::types::common::StringifiedDouble> = None;
                let mut f_unit: Option<crate::types::ml_common::RateLimiterUnit> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "limit" => {
                            if f_limit.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            f_limit =
                                Some(map.next_value::<crate::types::common::StringifiedDouble>()?);
                        }
                        "unit" => {
                            if f_unit.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            f_unit =
                                Some(map.next_value::<crate::types::ml_common::RateLimiterUnit>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RateLimiter {
                    limit: f_limit.ok_or_else(|| serde::de::Error::missing_field("limit"))?,
                    unit: f_unit.ok_or_else(|| serde::de::Error::missing_field("unit"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RateLimiter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("limit", &self.limit)?;
        map.serialize_entry("unit", &self.unit)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RateLimiterUnit {
    #[doc = "`DAYS`"]
    Days,
    #[doc = "`HOURS`"]
    Hours,
    #[doc = "`MICROSECONDS`"]
    Microseconds,
    #[doc = "`MILLISECONDS`"]
    Milliseconds,
    #[doc = "`MINUTES`"]
    Minutes,
    #[doc = "`NANOSECONDS`"]
    Nanoseconds,
    #[doc = "`SECONDS`"]
    Seconds,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl RateLimiterUnit {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            RateLimiterUnit::Days => "DAYS",
            RateLimiterUnit::Hours => "HOURS",
            RateLimiterUnit::Microseconds => "MICROSECONDS",
            RateLimiterUnit::Milliseconds => "MILLISECONDS",
            RateLimiterUnit::Minutes => "MINUTES",
            RateLimiterUnit::Nanoseconds => "NANOSECONDS",
            RateLimiterUnit::Seconds => "SECONDS",
            RateLimiterUnit::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for RateLimiterUnit {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RateLimiterUnit {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "DAYS" => RateLimiterUnit::Days,
            "HOURS" => RateLimiterUnit::Hours,
            "MICROSECONDS" => RateLimiterUnit::Microseconds,
            "MILLISECONDS" => RateLimiterUnit::Milliseconds,
            "MINUTES" => RateLimiterUnit::Minutes,
            "NANOSECONDS" => RateLimiterUnit::Nanoseconds,
            "SECONDS" => RateLimiterUnit::Seconds,
            _ => RateLimiterUnit::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Result {
    pub buckets: Option<Vec<crate::types::ml_common::Buckets>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Result {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Result;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Result object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_buckets: Option<Vec<crate::types::ml_common::Buckets>> = None;
                let mut seen_buckets = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "buckets" => {
                            if seen_buckets {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            seen_buckets = true;
                            f_buckets =
                                map.next_value::<Option<Vec<crate::types::ml_common::Buckets>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Result {
                    buckets: f_buckets,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Result {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.buckets {
            map.serialize_entry("buckets", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Rows {
    pub values: Option<Vec<crate::types::ml_common::Values>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Rows {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Rows;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Rows object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_values: Option<Vec<crate::types::ml_common::Values>> = None;
                let mut seen_values = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "values" => {
                            if seen_values {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            seen_values = true;
                            f_values =
                                map.next_value::<Option<Vec<crate::types::ml_common::Values>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Rows {
                    values: f_values,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Rows {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.values {
            map.serialize_entry("values", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type SearchAgentsResponse = crate::types::ml_common::SearchResponse;

pub type SearchConnectorsResponse = crate::types::ml_common::SearchResponse;

#[derive(Debug, Clone)]
pub struct SearchHits {
    pub hits: Vec<crate::types::ml_common::SearchHitsHit>,
    #[doc = "The maximum score."]
    pub max_score: Option<f64>,
    pub total: crate::types::ml_common::HitsTotal,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SearchHits {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SearchHits;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchHits object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_hits: Option<Vec<crate::types::ml_common::SearchHitsHit>> = None;
                let mut f_max_score: Option<f64> = None;
                let mut seen_max_score = false;
                let mut f_total: Option<crate::types::ml_common::HitsTotal> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "hits" => {
                            if f_hits.is_some() {
                                return Err(serde::de::Error::duplicate_field("hits"));
                            }
                            f_hits = Some(
                                map.next_value::<Vec<crate::types::ml_common::SearchHitsHit>>()?,
                            );
                        }
                        "max_score" => {
                            if seen_max_score {
                                return Err(serde::de::Error::duplicate_field("max_score"));
                            }
                            seen_max_score = true;
                            f_max_score = map.next_value::<Option<f64>>()?;
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(map.next_value::<crate::types::ml_common::HitsTotal>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SearchHits {
                    hits: f_hits.ok_or_else(|| serde::de::Error::missing_field("hits"))?,
                    max_score: f_max_score,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SearchHits {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("hits", &self.hits)?;
        if let Some(v) = &self.max_score {
            map.serialize_entry("max_score", v)?;
        }
        map.serialize_entry("total", &self.total)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SearchHitsHit {
    pub id: Option<crate::types::common::Id>,
    pub index: Option<crate::types::common::IndexName>,
    #[doc = "The primary term."]
    pub primary_term: Option<i64>,
    #[doc = "The score."]
    pub score: Option<f64>,
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    pub source: Option<crate::types::ml_common::Source>,
    pub version: Option<crate::types::common::VersionNumber>,
    pub model_id: Option<crate::types::common::Name>,
    #[doc = "The sort values."]
    pub sort: Option<Vec<f64>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SearchHitsHit {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SearchHitsHit;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchHitsHit object")
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
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut seen_index = false;
                let mut f_primary_term: Option<i64> = None;
                let mut seen_primary_term = false;
                let mut f_score: Option<f64> = None;
                let mut seen_score = false;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut seen_seq_no = false;
                let mut f_source: Option<crate::types::ml_common::Source> = None;
                let mut seen_source = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version = false;
                let mut f_model_id: Option<crate::types::common::Name> = None;
                let mut seen_model_id = false;
                let mut f_sort: Option<Vec<f64>> = None;
                let mut seen_sort = false;
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
                        "_index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("_index"));
                            }
                            seen_index = true;
                            f_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "_primary_term" => {
                            if seen_primary_term {
                                return Err(serde::de::Error::duplicate_field("_primary_term"));
                            }
                            seen_primary_term = true;
                            f_primary_term = map.next_value::<Option<i64>>()?;
                        }
                        "_score" => {
                            if seen_score {
                                return Err(serde::de::Error::duplicate_field("_score"));
                            }
                            seen_score = true;
                            f_score = map.next_value::<Option<f64>>()?;
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
                                map.next_value::<Option<crate::types::ml_common::Source>>()?;
                        }
                        "_version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionNumber>>()?;
                        }
                        "model_id" => {
                            if seen_model_id {
                                return Err(serde::de::Error::duplicate_field("model_id"));
                            }
                            seen_model_id = true;
                            f_model_id = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "sort" => {
                            if seen_sort {
                                return Err(serde::de::Error::duplicate_field("sort"));
                            }
                            seen_sort = true;
                            f_sort = map.next_value::<Option<Vec<f64>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SearchHitsHit {
                    id: f_id,
                    index: f_index,
                    primary_term: f_primary_term,
                    score: f_score,
                    seq_no: f_seq_no,
                    source: f_source,
                    version: f_version,
                    model_id: f_model_id,
                    sort: f_sort,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SearchHitsHit {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.id {
            map.serialize_entry("_id", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("_index", v)?;
        }
        if let Some(v) = &self.primary_term {
            map.serialize_entry("_primary_term", v)?;
        }
        if let Some(v) = &self.score {
            map.serialize_entry("_score", v)?;
        }
        if let Some(v) = &self.seq_no {
            map.serialize_entry("_seq_no", v)?;
        }
        if let Some(v) = &self.source {
            map.serialize_entry("_source", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("_version", v)?;
        }
        if let Some(v) = &self.model_id {
            map.serialize_entry("model_id", v)?;
        }
        if let Some(v) = &self.sort {
            map.serialize_entry("sort", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type SearchMemoryResponse = crate::types::ml_common::SearchResponse;

pub type SearchMessageResponse = crate::types::ml_common::SearchResponse;

pub type SearchModelGroupsResponse = crate::types::ml_common::SearchResponse;

pub type SearchModelsResponse = crate::types::ml_common::SearchResponse;

#[derive(Debug, Clone)]
pub struct SearchResponse {
    pub shards: Option<crate::types::common::ShardStatistics>,
    pub hits: crate::types::ml_common::SearchHits,
    #[doc = "Whether the search timed out."]
    pub timed_out: Option<bool>,
    #[doc = "The time taken to execute the search."]
    pub took: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SearchResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SearchResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_shards: Option<crate::types::common::ShardStatistics> = None;
                let mut seen_shards = false;
                let mut f_hits: Option<crate::types::ml_common::SearchHits> = None;
                let mut f_timed_out: Option<bool> = None;
                let mut seen_timed_out = false;
                let mut f_took: Option<i64> = None;
                let mut seen_took = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_shards" => {
                            if seen_shards {
                                return Err(serde::de::Error::duplicate_field("_shards"));
                            }
                            seen_shards = true;
                            f_shards =
                                map.next_value::<Option<crate::types::common::ShardStatistics>>()?;
                        }
                        "hits" => {
                            if f_hits.is_some() {
                                return Err(serde::de::Error::duplicate_field("hits"));
                            }
                            f_hits = Some(map.next_value::<crate::types::ml_common::SearchHits>()?);
                        }
                        "timed_out" => {
                            if seen_timed_out {
                                return Err(serde::de::Error::duplicate_field("timed_out"));
                            }
                            seen_timed_out = true;
                            f_timed_out = map.next_value::<Option<bool>>()?;
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
                Ok(SearchResponse {
                    shards: f_shards,
                    hits: f_hits.ok_or_else(|| serde::de::Error::missing_field("hits"))?,
                    timed_out: f_timed_out,
                    took: f_took,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SearchResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.shards {
            map.serialize_entry("_shards", v)?;
        }
        map.serialize_entry("hits", &self.hits)?;
        if let Some(v) = &self.timed_out {
            map.serialize_entry("timed_out", v)?;
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

pub type SearchTasksResponse = crate::types::ml_common::SearchResponse;

#[derive(Debug, Clone)]
pub struct Source {
    pub access: Option<crate::types::ml_common::ModelGroupAccessMode>,
    pub actions: Option<Vec<crate::types::ml_common::Action>>,
    pub additional_info: Option<crate::types::ml_common::AdditionalInfo>,
    #[doc = "The algorithm."]
    pub algorithm: Option<String>,
    #[doc = "The app type."]
    pub app_type: Option<String>,
    #[doc = "The application type."]
    pub application_type: Option<String>,
    #[doc = "The auto redeploy retry times."]
    pub auto_redeploy_retry_times: Option<i64>,
    #[doc = "The backend roles."]
    pub backend_roles: Option<Vec<String>>,
    #[doc = "The chunk number."]
    pub chunk_number: Option<i64>,
    pub configuration: Option<crate::types::ml_common::MemoryContainerConfiguration>,
    #[doc = "The connector ID."]
    pub connector_id: Option<String>,
    #[doc = "The create time."]
    pub create_time: Option<crate::types::ml_common::SourceCreateTime>,
    #[doc = "The created time."]
    pub created_time: Option<i64>,
    #[doc = "The current worker node count."]
    pub current_worker_node_count: Option<f64>,
    #[doc = "Whether to deploy to all nodes."]
    pub deploy_to_all_nodes: Option<bool>,
    #[doc = "The model description."]
    pub description: Option<String>,
    #[doc = "The error message."]
    pub error: Option<String>,
    pub function_name: Option<crate::types::ml_common::FunctionName>,
    #[doc = "The question in the message."]
    pub input: Option<String>,
    pub input_type: Option<crate::types::ml_common::MlInputDataType>,
    #[doc = "Whether the task is asynchronous."]
    pub is_async: Option<bool>,
    #[doc = "Whether the model is hidden."]
    pub is_hidden: Option<bool>,
    #[doc = "The last deployed time."]
    pub last_deployed_time: Option<i64>,
    #[doc = "The last registered time."]
    pub last_registered_time: Option<i64>,
    #[doc = "The last update time."]
    pub last_update_time: Option<i64>,
    #[doc = "The last updated time."]
    pub last_updated_time: Option<i64>,
    #[doc = "The latest version."]
    pub latest_version: Option<i64>,
    pub memory: Option<crate::types::ml_common::Memory>,
    pub memory_id: Option<crate::types::common::Name>,
    pub metadata: Option<crate::types::ml_common::Metadata>,
    pub model_config: Option<crate::types::ml_common::ModelConfig>,
    #[doc = "The model content hash value."]
    pub model_content_hash_value: Option<String>,
    #[doc = "The model content size in bytes."]
    pub model_content_size_in_bytes: Option<i64>,
    pub model_format: Option<crate::types::ml_common::ModelFormat>,
    #[doc = "The model group ID."]
    pub model_group_id: Option<String>,
    pub model_id: Option<crate::types::common::Name>,
    pub model_state: Option<crate::types::ml_common::ModelState>,
    #[doc = "The model task type."]
    pub model_task_type: Option<String>,
    #[doc = "The model version."]
    pub model_version: Option<String>,
    pub name: Option<crate::types::common::Name>,
    pub namespace: Option<crate::types::ml_common::Namespace>,
    #[doc = "The number of namespaces."]
    pub namespace_size: Option<i64>,
    #[doc = "The system name that generated the response."]
    pub origin: Option<String>,
    pub owner: Option<crate::types::ml_common::Owner>,
    #[doc = "The ID of the memory owner."]
    pub owner_id: Option<String>,
    pub parameters: Option<crate::types::ml_common::Parameters>,
    #[doc = "The parent message ID."]
    pub parent_message_id: Option<String>,
    pub payload_type: Option<crate::types::ml_common::PayloadType>,
    #[doc = "The planning worker node count."]
    pub planning_worker_node_count: Option<f64>,
    pub planning_worker_nodes: Option<Vec<crate::types::common::NodeIds>>,
    #[doc = "The prompt template."]
    pub prompt_template: Option<String>,
    pub protocol: Option<crate::types::ml_common::ConnectorProtocol>,
    #[doc = "The answer to the question."]
    pub response: Option<String>,
    pub state: Option<crate::types::ml_common::Status>,
    pub task_type: Option<crate::types::ml_common::MlTaskType>,
    pub tools: Option<Vec<crate::types::ml_common::ToolItems>>,
    #[doc = "The total chunks."]
    pub total_chunks: Option<i64>,
    #[doc = "The trace number."]
    pub trace_number: Option<String>,
    pub r#type: Option<crate::types::ml_common::AgentType>,
    #[doc = "The updated time."]
    pub updated_time: Option<String>,
    #[doc = "The model URL."]
    pub url: Option<String>,
    #[doc = "The username of the user."]
    pub user: Option<String>,
    pub version: Option<crate::types::common::VersionString>,
    pub worker_node: Option<Vec<crate::types::common::NodeIds>>,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Source {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Source;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Source object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_access: Option<crate::types::ml_common::ModelGroupAccessMode> = None;
                let mut seen_access = false;
                let mut f_actions: Option<Vec<crate::types::ml_common::Action>> = None;
                let mut seen_actions = false;
                let mut f_additional_info: Option<crate::types::ml_common::AdditionalInfo> = None;
                let mut seen_additional_info = false;
                let mut f_algorithm: Option<String> = None;
                let mut seen_algorithm = false;
                let mut f_app_type: Option<String> = None;
                let mut seen_app_type = false;
                let mut f_application_type: Option<String> = None;
                let mut seen_application_type = false;
                let mut f_auto_redeploy_retry_times: Option<i64> = None;
                let mut seen_auto_redeploy_retry_times = false;
                let mut f_backend_roles: Option<Vec<String>> = None;
                let mut seen_backend_roles = false;
                let mut f_chunk_number: Option<i64> = None;
                let mut seen_chunk_number = false;
                let mut f_configuration: Option<
                    crate::types::ml_common::MemoryContainerConfiguration,
                > = None;
                let mut seen_configuration = false;
                let mut f_connector_id: Option<String> = None;
                let mut seen_connector_id = false;
                let mut f_create_time: Option<crate::types::ml_common::SourceCreateTime> = None;
                let mut seen_create_time = false;
                let mut f_created_time: Option<i64> = None;
                let mut seen_created_time = false;
                let mut f_current_worker_node_count: Option<f64> = None;
                let mut seen_current_worker_node_count = false;
                let mut f_deploy_to_all_nodes: Option<bool> = None;
                let mut seen_deploy_to_all_nodes = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_error: Option<String> = None;
                let mut seen_error = false;
                let mut f_function_name: Option<crate::types::ml_common::FunctionName> = None;
                let mut seen_function_name = false;
                let mut f_input: Option<String> = None;
                let mut seen_input = false;
                let mut f_input_type: Option<crate::types::ml_common::MlInputDataType> = None;
                let mut seen_input_type = false;
                let mut f_is_async: Option<bool> = None;
                let mut seen_is_async = false;
                let mut f_is_hidden: Option<bool> = None;
                let mut seen_is_hidden = false;
                let mut f_last_deployed_time: Option<i64> = None;
                let mut seen_last_deployed_time = false;
                let mut f_last_registered_time: Option<i64> = None;
                let mut seen_last_registered_time = false;
                let mut f_last_update_time: Option<i64> = None;
                let mut seen_last_update_time = false;
                let mut f_last_updated_time: Option<i64> = None;
                let mut seen_last_updated_time = false;
                let mut f_latest_version: Option<i64> = None;
                let mut seen_latest_version = false;
                let mut f_memory: Option<crate::types::ml_common::Memory> = None;
                let mut seen_memory = false;
                let mut f_memory_id: Option<crate::types::common::Name> = None;
                let mut seen_memory_id = false;
                let mut f_metadata: Option<crate::types::ml_common::Metadata> = None;
                let mut seen_metadata = false;
                let mut f_model_config: Option<crate::types::ml_common::ModelConfig> = None;
                let mut seen_model_config = false;
                let mut f_model_content_hash_value: Option<String> = None;
                let mut seen_model_content_hash_value = false;
                let mut f_model_content_size_in_bytes: Option<i64> = None;
                let mut seen_model_content_size_in_bytes = false;
                let mut f_model_format: Option<crate::types::ml_common::ModelFormat> = None;
                let mut seen_model_format = false;
                let mut f_model_group_id: Option<String> = None;
                let mut seen_model_group_id = false;
                let mut f_model_id: Option<crate::types::common::Name> = None;
                let mut seen_model_id = false;
                let mut f_model_state: Option<crate::types::ml_common::ModelState> = None;
                let mut seen_model_state = false;
                let mut f_model_task_type: Option<String> = None;
                let mut seen_model_task_type = false;
                let mut f_model_version: Option<String> = None;
                let mut seen_model_version = false;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut seen_name = false;
                let mut f_namespace: Option<crate::types::ml_common::Namespace> = None;
                let mut seen_namespace = false;
                let mut f_namespace_size: Option<i64> = None;
                let mut seen_namespace_size = false;
                let mut f_origin: Option<String> = None;
                let mut seen_origin = false;
                let mut f_owner: Option<crate::types::ml_common::Owner> = None;
                let mut seen_owner = false;
                let mut f_owner_id: Option<String> = None;
                let mut seen_owner_id = false;
                let mut f_parameters: Option<crate::types::ml_common::Parameters> = None;
                let mut seen_parameters = false;
                let mut f_parent_message_id: Option<String> = None;
                let mut seen_parent_message_id = false;
                let mut f_payload_type: Option<crate::types::ml_common::PayloadType> = None;
                let mut seen_payload_type = false;
                let mut f_planning_worker_node_count: Option<f64> = None;
                let mut seen_planning_worker_node_count = false;
                let mut f_planning_worker_nodes: Option<Vec<crate::types::common::NodeIds>> = None;
                let mut seen_planning_worker_nodes = false;
                let mut f_prompt_template: Option<String> = None;
                let mut seen_prompt_template = false;
                let mut f_protocol: Option<crate::types::ml_common::ConnectorProtocol> = None;
                let mut seen_protocol = false;
                let mut f_response: Option<String> = None;
                let mut seen_response = false;
                let mut f_state: Option<crate::types::ml_common::Status> = None;
                let mut seen_state = false;
                let mut f_task_type: Option<crate::types::ml_common::MlTaskType> = None;
                let mut seen_task_type = false;
                let mut f_tools: Option<Vec<crate::types::ml_common::ToolItems>> = None;
                let mut seen_tools = false;
                let mut f_total_chunks: Option<i64> = None;
                let mut seen_total_chunks = false;
                let mut f_trace_number: Option<String> = None;
                let mut seen_trace_number = false;
                let mut f_type: Option<crate::types::ml_common::AgentType> = None;
                let mut seen_type = false;
                let mut f_updated_time: Option<String> = None;
                let mut seen_updated_time = false;
                let mut f_url: Option<String> = None;
                let mut seen_url = false;
                let mut f_user: Option<String> = None;
                let mut seen_user = false;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_worker_node: Option<Vec<crate::types::common::NodeIds>> = None;
                let mut seen_worker_node = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "access" => {
                            if seen_access {
                                return Err(serde::de::Error::duplicate_field("access"));
                            }
                            seen_access = true;
                            f_access = map.next_value::<Option<crate::types::ml_common::ModelGroupAccessMode>>()?;
                        }
                        "actions" => {
                            if seen_actions {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            seen_actions = true;
                            f_actions =
                                map.next_value::<Option<Vec<crate::types::ml_common::Action>>>()?;
                        }
                        "additional_info" => {
                            if seen_additional_info {
                                return Err(serde::de::Error::duplicate_field("additional_info"));
                            }
                            seen_additional_info = true;
                            f_additional_info = map
                                .next_value::<Option<crate::types::ml_common::AdditionalInfo>>()?;
                        }
                        "algorithm" => {
                            if seen_algorithm {
                                return Err(serde::de::Error::duplicate_field("algorithm"));
                            }
                            seen_algorithm = true;
                            f_algorithm = map.next_value::<Option<String>>()?;
                        }
                        "app_type" => {
                            if seen_app_type {
                                return Err(serde::de::Error::duplicate_field("app_type"));
                            }
                            seen_app_type = true;
                            f_app_type = map.next_value::<Option<String>>()?;
                        }
                        "application_type" => {
                            if seen_application_type {
                                return Err(serde::de::Error::duplicate_field("application_type"));
                            }
                            seen_application_type = true;
                            f_application_type = map.next_value::<Option<String>>()?;
                        }
                        "auto_redeploy_retry_times" => {
                            if seen_auto_redeploy_retry_times {
                                return Err(serde::de::Error::duplicate_field(
                                    "auto_redeploy_retry_times",
                                ));
                            }
                            seen_auto_redeploy_retry_times = true;
                            f_auto_redeploy_retry_times = map.next_value::<Option<i64>>()?;
                        }
                        "backend_roles" => {
                            if seen_backend_roles {
                                return Err(serde::de::Error::duplicate_field("backend_roles"));
                            }
                            seen_backend_roles = true;
                            f_backend_roles = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "chunk_number" => {
                            if seen_chunk_number {
                                return Err(serde::de::Error::duplicate_field("chunk_number"));
                            }
                            seen_chunk_number = true;
                            f_chunk_number = map.next_value::<Option<i64>>()?;
                        }
                        "configuration" => {
                            if seen_configuration {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            seen_configuration = true;
                            f_configuration = map.next_value::<Option<crate::types::ml_common::MemoryContainerConfiguration>>()?;
                        }
                        "connector_id" => {
                            if seen_connector_id {
                                return Err(serde::de::Error::duplicate_field("connector_id"));
                            }
                            seen_connector_id = true;
                            f_connector_id = map.next_value::<Option<String>>()?;
                        }
                        "create_time" => {
                            if seen_create_time {
                                return Err(serde::de::Error::duplicate_field("create_time"));
                            }
                            seen_create_time = true;
                            f_create_time = map
                                .next_value::<Option<crate::types::ml_common::SourceCreateTime>>(
                                )?;
                        }
                        "created_time" => {
                            if seen_created_time {
                                return Err(serde::de::Error::duplicate_field("created_time"));
                            }
                            seen_created_time = true;
                            f_created_time = map.next_value::<Option<i64>>()?;
                        }
                        "current_worker_node_count" => {
                            if seen_current_worker_node_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "current_worker_node_count",
                                ));
                            }
                            seen_current_worker_node_count = true;
                            f_current_worker_node_count = map.next_value::<Option<f64>>()?;
                        }
                        "deploy_to_all_nodes" => {
                            if seen_deploy_to_all_nodes {
                                return Err(serde::de::Error::duplicate_field(
                                    "deploy_to_all_nodes",
                                ));
                            }
                            seen_deploy_to_all_nodes = true;
                            f_deploy_to_all_nodes = map.next_value::<Option<bool>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "error" => {
                            if seen_error {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            seen_error = true;
                            f_error = map.next_value::<Option<String>>()?;
                        }
                        "function_name" => {
                            if seen_function_name {
                                return Err(serde::de::Error::duplicate_field("function_name"));
                            }
                            seen_function_name = true;
                            f_function_name =
                                map.next_value::<Option<crate::types::ml_common::FunctionName>>()?;
                        }
                        "input" => {
                            if seen_input {
                                return Err(serde::de::Error::duplicate_field("input"));
                            }
                            seen_input = true;
                            f_input = map.next_value::<Option<String>>()?;
                        }
                        "input_type" => {
                            if seen_input_type {
                                return Err(serde::de::Error::duplicate_field("input_type"));
                            }
                            seen_input_type = true;
                            f_input_type = map
                                .next_value::<Option<crate::types::ml_common::MlInputDataType>>()?;
                        }
                        "is_async" => {
                            if seen_is_async {
                                return Err(serde::de::Error::duplicate_field("is_async"));
                            }
                            seen_is_async = true;
                            f_is_async = map.next_value::<Option<bool>>()?;
                        }
                        "is_hidden" => {
                            if seen_is_hidden {
                                return Err(serde::de::Error::duplicate_field("is_hidden"));
                            }
                            seen_is_hidden = true;
                            f_is_hidden = map.next_value::<Option<bool>>()?;
                        }
                        "last_deployed_time" => {
                            if seen_last_deployed_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "last_deployed_time",
                                ));
                            }
                            seen_last_deployed_time = true;
                            f_last_deployed_time = map.next_value::<Option<i64>>()?;
                        }
                        "last_registered_time" => {
                            if seen_last_registered_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "last_registered_time",
                                ));
                            }
                            seen_last_registered_time = true;
                            f_last_registered_time = map.next_value::<Option<i64>>()?;
                        }
                        "last_update_time" => {
                            if seen_last_update_time {
                                return Err(serde::de::Error::duplicate_field("last_update_time"));
                            }
                            seen_last_update_time = true;
                            f_last_update_time = map.next_value::<Option<i64>>()?;
                        }
                        "last_updated_time" => {
                            if seen_last_updated_time {
                                return Err(serde::de::Error::duplicate_field("last_updated_time"));
                            }
                            seen_last_updated_time = true;
                            f_last_updated_time = map.next_value::<Option<i64>>()?;
                        }
                        "latest_version" => {
                            if seen_latest_version {
                                return Err(serde::de::Error::duplicate_field("latest_version"));
                            }
                            seen_latest_version = true;
                            f_latest_version = map.next_value::<Option<i64>>()?;
                        }
                        "memory" => {
                            if seen_memory {
                                return Err(serde::de::Error::duplicate_field("memory"));
                            }
                            seen_memory = true;
                            f_memory =
                                map.next_value::<Option<crate::types::ml_common::Memory>>()?;
                        }
                        "memory_id" => {
                            if seen_memory_id {
                                return Err(serde::de::Error::duplicate_field("memory_id"));
                            }
                            seen_memory_id = true;
                            f_memory_id = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "metadata" => {
                            if seen_metadata {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            seen_metadata = true;
                            f_metadata =
                                map.next_value::<Option<crate::types::ml_common::Metadata>>()?;
                        }
                        "model_config" => {
                            if seen_model_config {
                                return Err(serde::de::Error::duplicate_field("model_config"));
                            }
                            seen_model_config = true;
                            f_model_config =
                                map.next_value::<Option<crate::types::ml_common::ModelConfig>>()?;
                        }
                        "model_content_hash_value" => {
                            if seen_model_content_hash_value {
                                return Err(serde::de::Error::duplicate_field(
                                    "model_content_hash_value",
                                ));
                            }
                            seen_model_content_hash_value = true;
                            f_model_content_hash_value = map.next_value::<Option<String>>()?;
                        }
                        "model_content_size_in_bytes" => {
                            if seen_model_content_size_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "model_content_size_in_bytes",
                                ));
                            }
                            seen_model_content_size_in_bytes = true;
                            f_model_content_size_in_bytes = map.next_value::<Option<i64>>()?;
                        }
                        "model_format" => {
                            if seen_model_format {
                                return Err(serde::de::Error::duplicate_field("model_format"));
                            }
                            seen_model_format = true;
                            f_model_format =
                                map.next_value::<Option<crate::types::ml_common::ModelFormat>>()?;
                        }
                        "model_group_id" => {
                            if seen_model_group_id {
                                return Err(serde::de::Error::duplicate_field("model_group_id"));
                            }
                            seen_model_group_id = true;
                            f_model_group_id = map.next_value::<Option<String>>()?;
                        }
                        "model_id" => {
                            if seen_model_id {
                                return Err(serde::de::Error::duplicate_field("model_id"));
                            }
                            seen_model_id = true;
                            f_model_id = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "model_state" => {
                            if seen_model_state {
                                return Err(serde::de::Error::duplicate_field("model_state"));
                            }
                            seen_model_state = true;
                            f_model_state =
                                map.next_value::<Option<crate::types::ml_common::ModelState>>()?;
                        }
                        "model_task_type" => {
                            if seen_model_task_type {
                                return Err(serde::de::Error::duplicate_field("model_task_type"));
                            }
                            seen_model_task_type = true;
                            f_model_task_type = map.next_value::<Option<String>>()?;
                        }
                        "model_version" => {
                            if seen_model_version {
                                return Err(serde::de::Error::duplicate_field("model_version"));
                            }
                            seen_model_version = true;
                            f_model_version = map.next_value::<Option<String>>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "namespace" => {
                            if seen_namespace {
                                return Err(serde::de::Error::duplicate_field("namespace"));
                            }
                            seen_namespace = true;
                            f_namespace =
                                map.next_value::<Option<crate::types::ml_common::Namespace>>()?;
                        }
                        "namespace_size" => {
                            if seen_namespace_size {
                                return Err(serde::de::Error::duplicate_field("namespace_size"));
                            }
                            seen_namespace_size = true;
                            f_namespace_size = map.next_value::<Option<i64>>()?;
                        }
                        "origin" => {
                            if seen_origin {
                                return Err(serde::de::Error::duplicate_field("origin"));
                            }
                            seen_origin = true;
                            f_origin = map.next_value::<Option<String>>()?;
                        }
                        "owner" => {
                            if seen_owner {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            seen_owner = true;
                            f_owner = map.next_value::<Option<crate::types::ml_common::Owner>>()?;
                        }
                        "owner_id" => {
                            if seen_owner_id {
                                return Err(serde::de::Error::duplicate_field("owner_id"));
                            }
                            seen_owner_id = true;
                            f_owner_id = map.next_value::<Option<String>>()?;
                        }
                        "parameters" => {
                            if seen_parameters {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            seen_parameters = true;
                            f_parameters =
                                map.next_value::<Option<crate::types::ml_common::Parameters>>()?;
                        }
                        "parent_message_id" => {
                            if seen_parent_message_id {
                                return Err(serde::de::Error::duplicate_field("parent_message_id"));
                            }
                            seen_parent_message_id = true;
                            f_parent_message_id = map.next_value::<Option<String>>()?;
                        }
                        "payload_type" => {
                            if seen_payload_type {
                                return Err(serde::de::Error::duplicate_field("payload_type"));
                            }
                            seen_payload_type = true;
                            f_payload_type =
                                map.next_value::<Option<crate::types::ml_common::PayloadType>>()?;
                        }
                        "planning_worker_node_count" => {
                            if seen_planning_worker_node_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "planning_worker_node_count",
                                ));
                            }
                            seen_planning_worker_node_count = true;
                            f_planning_worker_node_count = map.next_value::<Option<f64>>()?;
                        }
                        "planning_worker_nodes" => {
                            if seen_planning_worker_nodes {
                                return Err(serde::de::Error::duplicate_field(
                                    "planning_worker_nodes",
                                ));
                            }
                            seen_planning_worker_nodes = true;
                            f_planning_worker_nodes =
                                map.next_value::<Option<Vec<crate::types::common::NodeIds>>>()?;
                        }
                        "prompt_template" => {
                            if seen_prompt_template {
                                return Err(serde::de::Error::duplicate_field("prompt_template"));
                            }
                            seen_prompt_template = true;
                            f_prompt_template = map.next_value::<Option<String>>()?;
                        }
                        "protocol" => {
                            if seen_protocol {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            seen_protocol = true;
                            f_protocol = map
                                .next_value::<Option<crate::types::ml_common::ConnectorProtocol>>(
                                )?;
                        }
                        "response" => {
                            if seen_response {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            seen_response = true;
                            f_response = map.next_value::<Option<String>>()?;
                        }
                        "state" => {
                            if seen_state {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            seen_state = true;
                            f_state =
                                map.next_value::<Option<crate::types::ml_common::Status>>()?;
                        }
                        "task_type" => {
                            if seen_task_type {
                                return Err(serde::de::Error::duplicate_field("task_type"));
                            }
                            seen_task_type = true;
                            f_task_type =
                                map.next_value::<Option<crate::types::ml_common::MlTaskType>>()?;
                        }
                        "tools" => {
                            if seen_tools {
                                return Err(serde::de::Error::duplicate_field("tools"));
                            }
                            seen_tools = true;
                            f_tools = map
                                .next_value::<Option<Vec<crate::types::ml_common::ToolItems>>>()?;
                        }
                        "total_chunks" => {
                            if seen_total_chunks {
                                return Err(serde::de::Error::duplicate_field("total_chunks"));
                            }
                            seen_total_chunks = true;
                            f_total_chunks = map.next_value::<Option<i64>>()?;
                        }
                        "trace_number" => {
                            if seen_trace_number {
                                return Err(serde::de::Error::duplicate_field("trace_number"));
                            }
                            seen_trace_number = true;
                            f_trace_number = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type =
                                map.next_value::<Option<crate::types::ml_common::AgentType>>()?;
                        }
                        "updated_time" => {
                            if seen_updated_time {
                                return Err(serde::de::Error::duplicate_field("updated_time"));
                            }
                            seen_updated_time = true;
                            f_updated_time = map.next_value::<Option<String>>()?;
                        }
                        "url" => {
                            if seen_url {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            seen_url = true;
                            f_url = map.next_value::<Option<String>>()?;
                        }
                        "user" => {
                            if seen_user {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            seen_user = true;
                            f_user = map.next_value::<Option<String>>()?;
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "worker_node" => {
                            if seen_worker_node {
                                return Err(serde::de::Error::duplicate_field("worker_node"));
                            }
                            seen_worker_node = true;
                            f_worker_node =
                                map.next_value::<Option<Vec<crate::types::common::NodeIds>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Source {
                    access: f_access,
                    actions: f_actions,
                    additional_info: f_additional_info,
                    algorithm: f_algorithm,
                    app_type: f_app_type,
                    application_type: f_application_type,
                    auto_redeploy_retry_times: f_auto_redeploy_retry_times,
                    backend_roles: f_backend_roles,
                    chunk_number: f_chunk_number,
                    configuration: f_configuration,
                    connector_id: f_connector_id,
                    create_time: f_create_time,
                    created_time: f_created_time,
                    current_worker_node_count: f_current_worker_node_count,
                    deploy_to_all_nodes: f_deploy_to_all_nodes,
                    description: f_description,
                    error: f_error,
                    function_name: f_function_name,
                    input: f_input,
                    input_type: f_input_type,
                    is_async: f_is_async,
                    is_hidden: f_is_hidden,
                    last_deployed_time: f_last_deployed_time,
                    last_registered_time: f_last_registered_time,
                    last_update_time: f_last_update_time,
                    last_updated_time: f_last_updated_time,
                    latest_version: f_latest_version,
                    memory: f_memory,
                    memory_id: f_memory_id,
                    metadata: f_metadata,
                    model_config: f_model_config,
                    model_content_hash_value: f_model_content_hash_value,
                    model_content_size_in_bytes: f_model_content_size_in_bytes,
                    model_format: f_model_format,
                    model_group_id: f_model_group_id,
                    model_id: f_model_id,
                    model_state: f_model_state,
                    model_task_type: f_model_task_type,
                    model_version: f_model_version,
                    name: f_name,
                    namespace: f_namespace,
                    namespace_size: f_namespace_size,
                    origin: f_origin,
                    owner: f_owner,
                    owner_id: f_owner_id,
                    parameters: f_parameters,
                    parent_message_id: f_parent_message_id,
                    payload_type: f_payload_type,
                    planning_worker_node_count: f_planning_worker_node_count,
                    planning_worker_nodes: f_planning_worker_nodes,
                    prompt_template: f_prompt_template,
                    protocol: f_protocol,
                    response: f_response,
                    state: f_state,
                    task_type: f_task_type,
                    tools: f_tools,
                    total_chunks: f_total_chunks,
                    trace_number: f_trace_number,
                    r#type: f_type,
                    updated_time: f_updated_time,
                    url: f_url,
                    user: f_user,
                    version: f_version,
                    worker_node: f_worker_node,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Source {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.access {
            map.serialize_entry("access", v)?;
        }
        if let Some(v) = &self.actions {
            map.serialize_entry("actions", v)?;
        }
        if let Some(v) = &self.additional_info {
            map.serialize_entry("additional_info", v)?;
        }
        if let Some(v) = &self.algorithm {
            map.serialize_entry("algorithm", v)?;
        }
        if let Some(v) = &self.app_type {
            map.serialize_entry("app_type", v)?;
        }
        if let Some(v) = &self.application_type {
            map.serialize_entry("application_type", v)?;
        }
        if let Some(v) = &self.auto_redeploy_retry_times {
            map.serialize_entry("auto_redeploy_retry_times", v)?;
        }
        if let Some(v) = &self.backend_roles {
            map.serialize_entry("backend_roles", v)?;
        }
        if let Some(v) = &self.chunk_number {
            map.serialize_entry("chunk_number", v)?;
        }
        if let Some(v) = &self.configuration {
            map.serialize_entry("configuration", v)?;
        }
        if let Some(v) = &self.connector_id {
            map.serialize_entry("connector_id", v)?;
        }
        if let Some(v) = &self.create_time {
            map.serialize_entry("create_time", v)?;
        }
        if let Some(v) = &self.created_time {
            map.serialize_entry("created_time", v)?;
        }
        if let Some(v) = &self.current_worker_node_count {
            map.serialize_entry("current_worker_node_count", v)?;
        }
        if let Some(v) = &self.deploy_to_all_nodes {
            map.serialize_entry("deploy_to_all_nodes", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.error {
            map.serialize_entry("error", v)?;
        }
        if let Some(v) = &self.function_name {
            map.serialize_entry("function_name", v)?;
        }
        if let Some(v) = &self.input {
            map.serialize_entry("input", v)?;
        }
        if let Some(v) = &self.input_type {
            map.serialize_entry("input_type", v)?;
        }
        if let Some(v) = &self.is_async {
            map.serialize_entry("is_async", v)?;
        }
        if let Some(v) = &self.is_hidden {
            map.serialize_entry("is_hidden", v)?;
        }
        if let Some(v) = &self.last_deployed_time {
            map.serialize_entry("last_deployed_time", v)?;
        }
        if let Some(v) = &self.last_registered_time {
            map.serialize_entry("last_registered_time", v)?;
        }
        if let Some(v) = &self.last_update_time {
            map.serialize_entry("last_update_time", v)?;
        }
        if let Some(v) = &self.last_updated_time {
            map.serialize_entry("last_updated_time", v)?;
        }
        if let Some(v) = &self.latest_version {
            map.serialize_entry("latest_version", v)?;
        }
        if let Some(v) = &self.memory {
            map.serialize_entry("memory", v)?;
        }
        if let Some(v) = &self.memory_id {
            map.serialize_entry("memory_id", v)?;
        }
        if let Some(v) = &self.metadata {
            map.serialize_entry("metadata", v)?;
        }
        if let Some(v) = &self.model_config {
            map.serialize_entry("model_config", v)?;
        }
        if let Some(v) = &self.model_content_hash_value {
            map.serialize_entry("model_content_hash_value", v)?;
        }
        if let Some(v) = &self.model_content_size_in_bytes {
            map.serialize_entry("model_content_size_in_bytes", v)?;
        }
        if let Some(v) = &self.model_format {
            map.serialize_entry("model_format", v)?;
        }
        if let Some(v) = &self.model_group_id {
            map.serialize_entry("model_group_id", v)?;
        }
        if let Some(v) = &self.model_id {
            map.serialize_entry("model_id", v)?;
        }
        if let Some(v) = &self.model_state {
            map.serialize_entry("model_state", v)?;
        }
        if let Some(v) = &self.model_task_type {
            map.serialize_entry("model_task_type", v)?;
        }
        if let Some(v) = &self.model_version {
            map.serialize_entry("model_version", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.namespace {
            map.serialize_entry("namespace", v)?;
        }
        if let Some(v) = &self.namespace_size {
            map.serialize_entry("namespace_size", v)?;
        }
        if let Some(v) = &self.origin {
            map.serialize_entry("origin", v)?;
        }
        if let Some(v) = &self.owner {
            map.serialize_entry("owner", v)?;
        }
        if let Some(v) = &self.owner_id {
            map.serialize_entry("owner_id", v)?;
        }
        if let Some(v) = &self.parameters {
            map.serialize_entry("parameters", v)?;
        }
        if let Some(v) = &self.parent_message_id {
            map.serialize_entry("parent_message_id", v)?;
        }
        if let Some(v) = &self.payload_type {
            map.serialize_entry("payload_type", v)?;
        }
        if let Some(v) = &self.planning_worker_node_count {
            map.serialize_entry("planning_worker_node_count", v)?;
        }
        if let Some(v) = &self.planning_worker_nodes {
            map.serialize_entry("planning_worker_nodes", v)?;
        }
        if let Some(v) = &self.prompt_template {
            map.serialize_entry("prompt_template", v)?;
        }
        if let Some(v) = &self.protocol {
            map.serialize_entry("protocol", v)?;
        }
        if let Some(v) = &self.response {
            map.serialize_entry("response", v)?;
        }
        if let Some(v) = &self.state {
            map.serialize_entry("state", v)?;
        }
        if let Some(v) = &self.task_type {
            map.serialize_entry("task_type", v)?;
        }
        if let Some(v) = &self.tools {
            map.serialize_entry("tools", v)?;
        }
        if let Some(v) = &self.total_chunks {
            map.serialize_entry("total_chunks", v)?;
        }
        if let Some(v) = &self.trace_number {
            map.serialize_entry("trace_number", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("type", v)?;
        }
        if let Some(v) = &self.updated_time {
            map.serialize_entry("updated_time", v)?;
        }
        if let Some(v) = &self.url {
            map.serialize_entry("url", v)?;
        }
        if let Some(v) = &self.user {
            map.serialize_entry("user", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.worker_node {
            map.serialize_entry("worker_node", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The create time."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SourceCreateTime {
    String(String),
    Integer(i64),
}
impl<'de> serde::Deserialize<'de> for SourceCreateTime {
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
            Some(b'"') => serde_json::from_str(text).map(SourceCreateTime::String),
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(SourceCreateTime::Integer)
            }
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a SourceCreateTime value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The status."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Status {
    #[doc = "`CANCELLED`"]
    Cancelled,
    #[doc = "`COMPLETED`"]
    Completed,
    #[doc = "`COMPLETED_WITH_ERROR`"]
    CompletedWithError,
    #[doc = "`CREATED`"]
    Created,
    #[doc = "`FAILED`"]
    Failed,
    #[doc = "`RUNNING`"]
    Running,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl Status {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            Status::Cancelled => "CANCELLED",
            Status::Completed => "COMPLETED",
            Status::CompletedWithError => "COMPLETED_WITH_ERROR",
            Status::Created => "CREATED",
            Status::Failed => "FAILED",
            Status::Running => "RUNNING",
            Status::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "CANCELLED" => Status::Cancelled,
            "COMPLETED" => Status::Completed,
            "COMPLETED_WITH_ERROR" => Status::CompletedWithError,
            "CREATED" => Status::Created,
            "FAILED" => Status::Failed,
            "RUNNING" => Status::Running,
            _ => Status::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Strategy {
    pub configuration: Option<crate::types::ml_common::StrategyConfiguration>,
    #[doc = "Whether to enable the strategy."]
    pub enabled: Option<bool>,
    #[doc = "The strategy ID."]
    pub id: Option<String>,
    #[doc = "The namespace."]
    pub namespace: Option<Vec<String>>,
    pub r#type: Option<crate::types::ml_common::StrategyType>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Strategy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Strategy;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Strategy object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_configuration: Option<crate::types::ml_common::StrategyConfiguration> =
                    None;
                let mut seen_configuration = false;
                let mut f_enabled: Option<bool> = None;
                let mut seen_enabled = false;
                let mut f_id: Option<String> = None;
                let mut seen_id = false;
                let mut f_namespace: Option<Vec<String>> = None;
                let mut seen_namespace = false;
                let mut f_type: Option<crate::types::ml_common::StrategyType> = None;
                let mut seen_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "configuration" => {
                            if seen_configuration {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            seen_configuration = true;
                            f_configuration = map.next_value::<Option<crate::types::ml_common::StrategyConfiguration>>()?;
                        }
                        "enabled" => {
                            if seen_enabled {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            seen_enabled = true;
                            f_enabled = map.next_value::<Option<bool>>()?;
                        }
                        "id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<String>>()?;
                        }
                        "namespace" => {
                            if seen_namespace {
                                return Err(serde::de::Error::duplicate_field("namespace"));
                            }
                            seen_namespace = true;
                            f_namespace = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type =
                                map.next_value::<Option<crate::types::ml_common::StrategyType>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Strategy {
                    configuration: f_configuration,
                    enabled: f_enabled,
                    id: f_id,
                    namespace: f_namespace,
                    r#type: f_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Strategy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.configuration {
            map.serialize_entry("configuration", v)?;
        }
        if let Some(v) = &self.enabled {
            map.serialize_entry("enabled", v)?;
        }
        if let Some(v) = &self.id {
            map.serialize_entry("id", v)?;
        }
        if let Some(v) = &self.namespace {
            map.serialize_entry("namespace", v)?;
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

#[derive(Debug, Clone)]
pub struct StrategyConfiguration {
    #[doc = "The LLM ID for strategy."]
    pub llm_id: Option<String>,
    #[doc = "JSONPath expression for extracting LLM results from responses."]
    pub llm_result_path: Option<String>,
    #[doc = "Custom system prompt to override default strategy prompt."]
    pub system_prompt: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StrategyConfiguration {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StrategyConfiguration;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StrategyConfiguration object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_llm_id: Option<String> = None;
                let mut seen_llm_id = false;
                let mut f_llm_result_path: Option<String> = None;
                let mut seen_llm_result_path = false;
                let mut f_system_prompt: Option<String> = None;
                let mut seen_system_prompt = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "llm_id" => {
                            if seen_llm_id {
                                return Err(serde::de::Error::duplicate_field("llm_id"));
                            }
                            seen_llm_id = true;
                            f_llm_id = map.next_value::<Option<String>>()?;
                        }
                        "llm_result_path" => {
                            if seen_llm_result_path {
                                return Err(serde::de::Error::duplicate_field("llm_result_path"));
                            }
                            seen_llm_result_path = true;
                            f_llm_result_path = map.next_value::<Option<String>>()?;
                        }
                        "system_prompt" => {
                            if seen_system_prompt {
                                return Err(serde::de::Error::duplicate_field("system_prompt"));
                            }
                            seen_system_prompt = true;
                            f_system_prompt = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StrategyConfiguration {
                    llm_id: f_llm_id,
                    llm_result_path: f_llm_result_path,
                    system_prompt: f_system_prompt,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StrategyConfiguration {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.llm_id {
            map.serialize_entry("llm_id", v)?;
        }
        if let Some(v) = &self.llm_result_path {
            map.serialize_entry("llm_result_path", v)?;
        }
        if let Some(v) = &self.system_prompt {
            map.serialize_entry("system_prompt", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The strategy type."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StrategyType {
    #[doc = "`SEMANTIC`"]
    Semantic,
    #[doc = "`SUMMARY`"]
    Summary,
    #[doc = "`USER_PREFERENCE`"]
    UserPreference,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl StrategyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            StrategyType::Semantic => "SEMANTIC",
            StrategyType::Summary => "SUMMARY",
            StrategyType::UserPreference => "USER_PREFERENCE",
            StrategyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for StrategyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StrategyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "SEMANTIC" => StrategyType::Semantic,
            "SUMMARY" => StrategyType::Summary,
            "USER_PREFERENCE" => StrategyType::UserPreference,
            _ => StrategyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Task {
    pub create_time: Option<i64>,
    pub error: Option<String>,
    pub function_name: Option<crate::types::ml_common::FunctionName>,
    pub is_async: Option<bool>,
    pub last_update_time: Option<i64>,
    pub model_id: Option<String>,
    pub state: crate::types::ml_common::TaskState,
    pub task_id: Option<String>,
    pub task_type: Option<crate::types::ml_common::MlTaskType>,
    pub worker_node: Option<Vec<crate::types::common::NodeIds>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Task {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Task;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Task object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_create_time: Option<i64> = None;
                let mut seen_create_time = false;
                let mut f_error: Option<String> = None;
                let mut seen_error = false;
                let mut f_function_name: Option<crate::types::ml_common::FunctionName> = None;
                let mut seen_function_name = false;
                let mut f_is_async: Option<bool> = None;
                let mut seen_is_async = false;
                let mut f_last_update_time: Option<i64> = None;
                let mut seen_last_update_time = false;
                let mut f_model_id: Option<String> = None;
                let mut seen_model_id = false;
                let mut f_state: Option<crate::types::ml_common::TaskState> = None;
                let mut f_task_id: Option<String> = None;
                let mut seen_task_id = false;
                let mut f_task_type: Option<crate::types::ml_common::MlTaskType> = None;
                let mut seen_task_type = false;
                let mut f_worker_node: Option<Vec<crate::types::common::NodeIds>> = None;
                let mut seen_worker_node = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "create_time" => {
                            if seen_create_time {
                                return Err(serde::de::Error::duplicate_field("create_time"));
                            }
                            seen_create_time = true;
                            f_create_time = map.next_value::<Option<i64>>()?;
                        }
                        "error" => {
                            if seen_error {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            seen_error = true;
                            f_error = map.next_value::<Option<String>>()?;
                        }
                        "function_name" => {
                            if seen_function_name {
                                return Err(serde::de::Error::duplicate_field("function_name"));
                            }
                            seen_function_name = true;
                            f_function_name =
                                map.next_value::<Option<crate::types::ml_common::FunctionName>>()?;
                        }
                        "is_async" => {
                            if seen_is_async {
                                return Err(serde::de::Error::duplicate_field("is_async"));
                            }
                            seen_is_async = true;
                            f_is_async = map.next_value::<Option<bool>>()?;
                        }
                        "last_update_time" => {
                            if seen_last_update_time {
                                return Err(serde::de::Error::duplicate_field("last_update_time"));
                            }
                            seen_last_update_time = true;
                            f_last_update_time = map.next_value::<Option<i64>>()?;
                        }
                        "model_id" => {
                            if seen_model_id {
                                return Err(serde::de::Error::duplicate_field("model_id"));
                            }
                            seen_model_id = true;
                            f_model_id = map.next_value::<Option<String>>()?;
                        }
                        "state" => {
                            if f_state.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            f_state = Some(map.next_value::<crate::types::ml_common::TaskState>()?);
                        }
                        "task_id" => {
                            if seen_task_id {
                                return Err(serde::de::Error::duplicate_field("task_id"));
                            }
                            seen_task_id = true;
                            f_task_id = map.next_value::<Option<String>>()?;
                        }
                        "task_type" => {
                            if seen_task_type {
                                return Err(serde::de::Error::duplicate_field("task_type"));
                            }
                            seen_task_type = true;
                            f_task_type =
                                map.next_value::<Option<crate::types::ml_common::MlTaskType>>()?;
                        }
                        "worker_node" => {
                            if seen_worker_node {
                                return Err(serde::de::Error::duplicate_field("worker_node"));
                            }
                            seen_worker_node = true;
                            f_worker_node =
                                map.next_value::<Option<Vec<crate::types::common::NodeIds>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Task {
                    create_time: f_create_time,
                    error: f_error,
                    function_name: f_function_name,
                    is_async: f_is_async,
                    last_update_time: f_last_update_time,
                    model_id: f_model_id,
                    state: f_state.ok_or_else(|| serde::de::Error::missing_field("state"))?,
                    task_id: f_task_id,
                    task_type: f_task_type,
                    worker_node: f_worker_node,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Task {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.create_time {
            map.serialize_entry("create_time", v)?;
        }
        if let Some(v) = &self.error {
            map.serialize_entry("error", v)?;
        }
        if let Some(v) = &self.function_name {
            map.serialize_entry("function_name", v)?;
        }
        if let Some(v) = &self.is_async {
            map.serialize_entry("is_async", v)?;
        }
        if let Some(v) = &self.last_update_time {
            map.serialize_entry("last_update_time", v)?;
        }
        if let Some(v) = &self.model_id {
            map.serialize_entry("model_id", v)?;
        }
        map.serialize_entry("state", &self.state)?;
        if let Some(v) = &self.task_id {
            map.serialize_entry("task_id", v)?;
        }
        if let Some(v) = &self.task_type {
            map.serialize_entry("task_type", v)?;
        }
        if let Some(v) = &self.worker_node {
            map.serialize_entry("worker_node", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaskState {
    #[doc = "`CANCELLED`"]
    Cancelled,
    #[doc = "`CANCELLING`"]
    Cancelling,
    #[doc = "`COMPLETED`"]
    Completed,
    #[doc = "`COMPLETED_WITH_ERROR`"]
    CompletedWithError,
    #[doc = "`CREATED`"]
    Created,
    #[doc = "`EXPIRED`"]
    Expired,
    #[doc = "`FAILED`"]
    Failed,
    #[doc = "`RUNNING`"]
    Running,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TaskState {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TaskState::Cancelled => "CANCELLED",
            TaskState::Cancelling => "CANCELLING",
            TaskState::Completed => "COMPLETED",
            TaskState::CompletedWithError => "COMPLETED_WITH_ERROR",
            TaskState::Created => "CREATED",
            TaskState::Expired => "EXPIRED",
            TaskState::Failed => "FAILED",
            TaskState::Running => "RUNNING",
            TaskState::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TaskState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaskState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "CANCELLED" => TaskState::Cancelled,
            "CANCELLING" => TaskState::Cancelling,
            "COMPLETED" => TaskState::Completed,
            "COMPLETED_WITH_ERROR" => TaskState::CompletedWithError,
            "CREATED" => TaskState::Created,
            "EXPIRED" => TaskState::Expired,
            "FAILED" => TaskState::Failed,
            "RUNNING" => TaskState::Running,
            _ => TaskState::Unknown(s),
        })
    }
}

pub type Tasks = std::collections::BTreeMap<String, crate::types::ml_common::Task>;

#[derive(Debug, Clone)]
pub struct Tool {
    pub attributes: Option<crate::types::ml_common::ToolAttributes>,
    #[doc = "The tool description."]
    pub description: Option<String>,
    pub name: Option<crate::types::common::Name>,
    #[doc = "The tool type."]
    pub r#type: Option<String>,
    pub version: Option<crate::types::common::VersionString>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Tool {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Tool;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Tool object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_attributes: Option<crate::types::ml_common::ToolAttributes> = None;
                let mut seen_attributes = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut seen_name = false;
                let mut f_type: Option<String> = None;
                let mut seen_type = false;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "attributes" => {
                            if seen_attributes {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            seen_attributes = true;
                            f_attributes = map
                                .next_value::<Option<crate::types::ml_common::ToolAttributes>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<String>>()?;
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Tool {
                    attributes: f_attributes,
                    description: f_description,
                    name: f_name,
                    r#type: f_type,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Tool {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.attributes {
            map.serialize_entry("attributes", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("type", v)?;
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
pub struct ToolAttributes {
    pub input_schema: Option<String>,
    pub strict: Option<bool>,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ToolAttributes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ToolAttributes;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ToolAttributes object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_input_schema: Option<String> = None;
                let mut seen_input_schema = false;
                let mut f_strict: Option<bool> = None;
                let mut seen_strict = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "input_schema" => {
                            if seen_input_schema {
                                return Err(serde::de::Error::duplicate_field("input_schema"));
                            }
                            seen_input_schema = true;
                            f_input_schema = map.next_value::<Option<String>>()?;
                        }
                        "strict" => {
                            if seen_strict {
                                return Err(serde::de::Error::duplicate_field("strict"));
                            }
                            seen_strict = true;
                            f_strict = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ToolAttributes {
                    input_schema: f_input_schema,
                    strict: f_strict,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ToolAttributes {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.input_schema {
            map.serialize_entry("input_schema", v)?;
        }
        if let Some(v) = &self.strict {
            map.serialize_entry("strict", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ToolItems {
    pub attributes: Option<crate::types::ml_common::ToolAttributes>,
    pub description: Option<String>,
    pub include_output_in_agent_response: Option<bool>,
    pub name: Option<String>,
    pub parameters: Option<crate::types::ml_common::Parameters>,
    pub r#type: Option<String>,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ToolItems {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ToolItems;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ToolItems object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_attributes: Option<crate::types::ml_common::ToolAttributes> = None;
                let mut seen_attributes = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_include_output_in_agent_response: Option<bool> = None;
                let mut seen_include_output_in_agent_response = false;
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_parameters: Option<crate::types::ml_common::Parameters> = None;
                let mut seen_parameters = false;
                let mut f_type: Option<String> = None;
                let mut seen_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "attributes" => {
                            if seen_attributes {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            seen_attributes = true;
                            f_attributes = map
                                .next_value::<Option<crate::types::ml_common::ToolAttributes>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "include_output_in_agent_response" => {
                            if seen_include_output_in_agent_response {
                                return Err(serde::de::Error::duplicate_field(
                                    "include_output_in_agent_response",
                                ));
                            }
                            seen_include_output_in_agent_response = true;
                            f_include_output_in_agent_response =
                                map.next_value::<Option<bool>>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "parameters" => {
                            if seen_parameters {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            seen_parameters = true;
                            f_parameters =
                                map.next_value::<Option<crate::types::ml_common::Parameters>>()?;
                        }
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ToolItems {
                    attributes: f_attributes,
                    description: f_description,
                    include_output_in_agent_response: f_include_output_in_agent_response,
                    name: f_name,
                    parameters: f_parameters,
                    r#type: f_type,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ToolItems {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.attributes {
            map.serialize_entry("attributes", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.include_output_in_agent_response {
            map.serialize_entry("include_output_in_agent_response", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.parameters {
            map.serialize_entry("parameters", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("type", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TrainPredictResponse {
    pub prediction_result: Option<crate::types::ml_common::PredictionResult>,
    pub status: crate::types::ml_common::Status,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TrainPredictResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TrainPredictResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TrainPredictResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_prediction_result: Option<crate::types::ml_common::PredictionResult> =
                    None;
                let mut seen_prediction_result = false;
                let mut f_status: Option<crate::types::ml_common::Status> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "prediction_result" => {
                            if seen_prediction_result {
                                return Err(serde::de::Error::duplicate_field("prediction_result"));
                            }
                            seen_prediction_result = true;
                            f_prediction_result = map
                                .next_value::<Option<crate::types::ml_common::PredictionResult>>(
                                )?;
                        }
                        "status" => {
                            if f_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            f_status = Some(map.next_value::<crate::types::ml_common::Status>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TrainPredictResponse {
                    prediction_result: f_prediction_result,
                    status: f_status.ok_or_else(|| serde::de::Error::missing_field("status"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TrainPredictResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.prediction_result {
            map.serialize_entry("prediction_result", v)?;
        }
        map.serialize_entry("status", &self.status)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TrainResponse {
    pub model_id: Option<crate::types::common::Name>,
    pub status: crate::types::ml_common::Status,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TrainResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TrainResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TrainResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_model_id: Option<crate::types::common::Name> = None;
                let mut seen_model_id = false;
                let mut f_status: Option<crate::types::ml_common::Status> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "model_id" => {
                            if seen_model_id {
                                return Err(serde::de::Error::duplicate_field("model_id"));
                            }
                            seen_model_id = true;
                            f_model_id = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "status" => {
                            if f_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            f_status = Some(map.next_value::<crate::types::ml_common::Status>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TrainResponse {
                    model_id: f_model_id,
                    status: f_status.ok_or_else(|| serde::de::Error::missing_field("status"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TrainResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.model_id {
            map.serialize_entry("model_id", v)?;
        }
        map.serialize_entry("status", &self.status)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct UndeployModelNode {
    pub stats: Option<crate::types::ml_common::UndeployModelNodeStats>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UndeployModelNode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UndeployModelNode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UndeployModelNode object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_stats: Option<crate::types::ml_common::UndeployModelNodeStats> = None;
                let mut seen_stats = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "stats" => {
                            if seen_stats {
                                return Err(serde::de::Error::duplicate_field("stats"));
                            }
                            seen_stats = true;
                            f_stats = map.next_value::<Option<crate::types::ml_common::UndeployModelNodeStats>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UndeployModelNode {
                    stats: f_stats,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UndeployModelNode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.stats {
            map.serialize_entry("stats", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type UndeployModelNodeStats =
    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

pub type UndeployModelResponse =
    std::collections::BTreeMap<String, crate::types::ml_common::UndeployModelNode>;

#[derive(Debug, Clone)]
pub struct UnloadModelNode {
    pub stats: Option<crate::types::ml_common::UnloadModelNodeStats>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UnloadModelNode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UnloadModelNode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UnloadModelNode object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_stats: Option<crate::types::ml_common::UnloadModelNodeStats> = None;
                let mut seen_stats = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "stats" => {
                            if seen_stats {
                                return Err(serde::de::Error::duplicate_field("stats"));
                            }
                            seen_stats = true;
                            f_stats = map.next_value::<Option<crate::types::ml_common::UnloadModelNodeStats>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UnloadModelNode {
                    stats: f_stats,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UnloadModelNode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.stats {
            map.serialize_entry("stats", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type UnloadModelNodeStats =
    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

pub type UnloadModelResponse =
    std::collections::BTreeMap<String, crate::types::ml_common::UnloadModelNode>;

#[derive(Debug, Clone)]
pub struct UpdateModelGroupResponse {
    #[doc = "The status."]
    pub status: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UpdateModelGroupResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UpdateModelGroupResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UpdateModelGroupResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_status: Option<String> = None;
                let mut seen_status = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "status" => {
                            if seen_status {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            seen_status = true;
                            f_status = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UpdateModelGroupResponse {
                    status: f_status,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UpdateModelGroupResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.status {
            map.serialize_entry("status", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Values {
    pub column_type: Option<crate::types::ml_common::ColumnType>,
    #[doc = "The value."]
    pub value: Option<f64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Values {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Values;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Values object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_column_type: Option<crate::types::ml_common::ColumnType> = None;
                let mut seen_column_type = false;
                let mut f_value: Option<f64> = None;
                let mut seen_value = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "column_type" => {
                            if seen_column_type {
                                return Err(serde::de::Error::duplicate_field("column_type"));
                            }
                            seen_column_type = true;
                            f_column_type =
                                map.next_value::<Option<crate::types::ml_common::ColumnType>>()?;
                        }
                        "value" => {
                            if seen_value {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            seen_value = true;
                            f_value = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Values {
                    column_type: f_column_type,
                    value: f_value,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Values {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.column_type {
            map.serialize_entry("column_type", v)?;
        }
        if let Some(v) = &self.value {
            map.serialize_entry("value", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
