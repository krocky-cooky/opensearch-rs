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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Action {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<crate::types::ml_common::Headers>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_process_function: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_process_function: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[doc = "The type of operation."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ActionType {
    #[serde(rename = "ADD")]
    Add,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "UPDATE")]
    Update,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AdditionalConfig {
    #[doc = "The distance metric for k-NN search."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space_type: Option<String>,
}

#[doc = "Additional information query."]
pub type AdditionalInfo = std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

#[doc = "The agent type."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AgentType {
    #[serde(rename = "conversational")]
    Conversational,
    #[serde(rename = "conversational_flow")]
    ConversationalFlow,
    #[serde(rename = "flow")]
    Flow,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AlgorithmOperations {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deploy: Option<crate::types::ml_common::ModelStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execute: Option<crate::types::ml_common::ModelStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predict: Option<crate::types::ml_common::ModelStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub register: Option<crate::types::ml_common::ModelStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub train: Option<crate::types::ml_common::ModelStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub train_predict: Option<crate::types::ml_common::ModelStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub undeploy: Option<crate::types::ml_common::ModelStats>,
}

pub type Algorithms =
    std::collections::BTreeMap<String, crate::types::ml_common::AlgorithmOperations>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Buckets {
    #[doc = "The end time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<crate::types::ml_common::Entity>>,
    #[doc = "The overall aggregate value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overall_aggregate_value: Option<f64>,
    #[doc = "The start time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ByteBuffer {
    #[doc = "The byte buffer array."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub array: Option<String>,
    #[doc = "The byte buffer order."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<crate::types::ml_common::ByteOrder>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ByteOrder {
    #[serde(rename = "BIG_ENDIAN")]
    BigEndian,
    #[serde(rename = "LITTLE_ENDIAN")]
    LittleEndian,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ColumnMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_type: Option<crate::types::ml_common::ColumnType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<crate::types::common::Name>,
}

#[doc = "The column type."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ColumnType {
    #[serde(rename = "BOOLEAN")]
    Boolean,
    #[serde(rename = "DOUBLE")]
    Double,
    #[serde(rename = "INTEGER")]
    Integer,
    #[serde(rename = "STRING")]
    String,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[doc = "The connector protocol."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ConnectorProtocol {
    #[serde(rename = "aws_sigv4")]
    AwsSigv4,
    #[serde(rename = "http")]
    Http,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
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
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_text: Option<String> = None;
                let mut f_type: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "text" => f_text = map.next_value::<Option<String>>()?,
                        "type" => f_type = map.next_value::<Option<String>>()?,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataAsMap {
    #[doc = "The streaming content chunk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[doc = "Whether this is the last chunk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_last: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeleteAgenticMemoryResponse {
    #[doc = "Number of batches processed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batches: Option<i64>,
    #[doc = "Number of documents created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[doc = "Number of documents deleted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<i64>,
    #[doc = "Any failures occurred during the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<crate::types::common::BulkByScrollFailure>>,
    #[doc = "Number of no-operation updates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noops: Option<i64>,
    #[doc = "Number of requests processed per second."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests_per_second: Option<f64>,
    #[doc = "The result of delete operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retries: Option<crate::types::common::Retries>,
    #[doc = "Time that the request was throttled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throttled_millis: Option<i64>,
    #[doc = "Time until throttling is lifted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throttled_until_millis: Option<i64>,
    #[doc = "Whether the request timed out."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timed_out: Option<bool>,
    #[doc = "Time taken to execute the request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub took: Option<i64>,
    #[doc = "Total number of documents processed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[doc = "Number of documents updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<i64>,
    #[doc = "Number of version conflicts encountered."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_conflicts: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Entity {
    #[doc = "The base value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_value: Option<f64>,
    #[doc = "The contribution value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contribution_value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<Vec<String>>,
    #[doc = "The new value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_value: Option<f64>,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExecuteAlgorithmResponseObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<crate::types::ml_common::ExecuteAnomalyLocalizationResponse>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExecuteAnomalyLocalizationResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<crate::types::ml_common::Result>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExecuteLocalSampleCalculatorResponse {
    #[doc = "The result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExecuteToolResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inference_results: Option<Vec<crate::types::ml_common::InferenceResults>>,
}

#[doc = "The function name."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FunctionName {
    #[serde(rename = "AD_LIBSVM")]
    AdLibsvm,
    #[serde(rename = "AGENT")]
    Agent,
    #[serde(rename = "ANOMALY_LOCALIZATION")]
    AnomalyLocalization,
    #[serde(rename = "BATCH_RCF")]
    BatchRcf,
    #[serde(rename = "CONNECTOR")]
    Connector,
    #[serde(rename = "FIT_RCF")]
    FitRcf,
    #[serde(rename = "KMEANS")]
    Kmeans,
    #[serde(rename = "LINEAR_REGRESSION")]
    LinearRegression,
    #[serde(rename = "LOCAL_SAMPLE_CALCULATOR")]
    LocalSampleCalculator,
    #[serde(rename = "LOGISTIC_REGRESSION")]
    LogisticRegression,
    #[serde(rename = "METRICS_CORRELATION")]
    MetricsCorrelation,
    #[serde(rename = "QUESTION_ANSWERING")]
    QuestionAnswering,
    #[serde(rename = "RCF_SUMMARIZE")]
    RcfSummarize,
    #[serde(rename = "REMOTE")]
    Remote,
    #[serde(rename = "SAMPLE_ALGO")]
    SampleAlgo,
    #[serde(rename = "SPARSE_ENCODING")]
    SparseEncoding,
    #[serde(rename = "SPARSE_TOKENIZE")]
    SparseTokenize,
    #[serde(rename = "TEXT_EMBEDDING")]
    TextEmbedding,
    #[serde(rename = "TEXT_SIMILARITY")]
    TextSimilarity,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetAgentResponse {
    #[doc = "The created time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    #[doc = "The agent description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Whether the agent is hidden."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    #[doc = "The last updated time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<crate::types::common::Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<crate::types::ml_common::ToolItems>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::types::ml_common::AgentType>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetAgenticMemoryResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<crate::types::ml_common::ActionType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The created time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    #[doc = "Whether inference was enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub infer: Option<bool>,
    #[doc = "The last updated time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
    #[doc = "The extracted long-term memory fact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    #[doc = "The ID of the memory container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_container_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_embedding: Option<Vec<f64>>,
    #[doc = "The ID of the memory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<crate::types::ml_common::Message>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::types::ml_common::Metadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<crate::types::ml_common::Namespace>,
    #[doc = "The number of namespaces."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace_size: Option<i64>,
    #[doc = "The ID of the memory owner."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload_type: Option<crate::types::ml_common::PayloadType>,
    #[doc = "The ID for the strategy instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy_type: Option<crate::types::ml_common::StrategyType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetConnectorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<crate::types::ml_common::Action>>,
    #[doc = "The created time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    #[doc = "The connector description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The last updated time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<crate::types::common::Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<crate::types::ml_common::Parameters>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<crate::types::ml_common::ConnectorProtocol>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetMemoryContainerResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<crate::types::ml_common::MemoryContainerConfiguration>,
    #[doc = "The created time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    #[doc = "The memory container description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The last updated time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<crate::types::common::Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<crate::types::ml_common::Owner>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetProfileResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<crate::types::ml_common::Nodes>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetStatsResponse {
    #[doc = "The config index status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ml_config_index_status: Option<crate::types::ml_common::MlIndexStatus>,
    #[doc = "The connector count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ml_connector_count: Option<i64>,
    #[doc = "The connector index status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ml_connector_index_status: Option<crate::types::ml_common::MlIndexStatus>,
    #[doc = "The controller index status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ml_controller_index_status: Option<crate::types::ml_common::MlIndexStatus>,
    #[doc = "The model count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ml_model_count: Option<i64>,
    #[doc = "The model index status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ml_model_index_status: Option<crate::types::ml_common::MlIndexStatus>,
    #[doc = "The task index status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ml_task_index_status: Option<crate::types::ml_common::MlIndexStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<crate::types::ml_common::NodeStats>,
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
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_content_type: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "content_type" => f_content_type = map.next_value::<Option<String>>()?,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HitsTotal {
    #[doc = "The relation."]
    pub relation: String,
    #[doc = "The total number of hits."]
    pub value: i64,
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
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_number_of_replicas: Option<String> = None;
                let mut f_number_of_shards: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "number_of_replicas" => {
                            f_number_of_replicas = map.next_value::<Option<String>>()?
                        }
                        "number_of_shards" => {
                            f_number_of_shards = map.next_value::<Option<String>>()?
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
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_long_term_memory_history_index: Option<
                    crate::types::ml_common::MemoryIndexSettings,
                > = None;
                let mut f_long_term_memory_index: Option<
                    crate::types::ml_common::MemoryIndexSettings,
                > = None;
                let mut f_session_index: Option<crate::types::ml_common::MemoryIndexSettings> =
                    None;
                let mut f_short_term_memory_index: Option<
                    crate::types::ml_common::MemoryIndexSettings,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "long_term_memory_history_index" => {
                            f_long_term_memory_history_index = map
                                .next_value::<Option<crate::types::ml_common::MemoryIndexSettings>>(
                                )?
                        }
                        "long_term_memory_index" => {
                            f_long_term_memory_index = map
                                .next_value::<Option<crate::types::ml_common::MemoryIndexSettings>>(
                                )?
                        }
                        "session_index" => {
                            f_session_index = map
                                .next_value::<Option<crate::types::ml_common::MemoryIndexSettings>>(
                                )?
                        }
                        "short_term_memory_index" => {
                            f_short_term_memory_index = map
                                .next_value::<Option<crate::types::ml_common::MemoryIndexSettings>>(
                                )?
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InferenceResults {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<Vec<crate::types::ml_common::Output>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Memory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<crate::types::ml_common::AdditionalInfo>,
    #[doc = "The create time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_id: Option<crate::types::common::Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<crate::types::common::Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[doc = "The updated time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    #[doc = "The username of the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryContainerConfiguration {
    #[doc = "Whether history will be persisted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_history: Option<bool>,
    #[doc = "Whether session will be persisted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_session: Option<bool>,
    #[doc = "The embedding dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embedding_dimension: Option<i64>,
    #[doc = "The embedding model ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embedding_model_id: Option<String>,
    #[doc = "The embedding model type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embedding_model_type: Option<String>,
    #[doc = "Custom prefix for memory indices."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_settings: Option<crate::types::ml_common::IndexSettings>,
    #[doc = "The LLM ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub llm_id: Option<String>,
    #[doc = "The maximum number of similar memories retrieved during consolidation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_infer_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<crate::types::ml_common::Parameters>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategies: Option<Vec<crate::types::ml_common::Strategy>>,
    #[doc = "Whether to use system indices."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_system_index: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryIndexSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<crate::types::ml_common::Index>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Message {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<crate::types::ml_common::AdditionalInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<crate::types::ml_common::Content>>,
    #[doc = "The create time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[doc = "The question in the message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_id: Option<crate::types::common::Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<crate::types::common::Name>,
    #[doc = "The system name that generated the response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "The parent message ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_message_id: Option<String>,
    #[doc = "The prompt template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_template: Option<String>,
    #[doc = "The answer to the question."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    #[doc = "The role of the message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[doc = "The trace number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_number: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Messages {
    #[doc = "The content of the message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[doc = "The role of the message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

pub type Metadata = std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MlIndexStatus {
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "non-existent")]
    NonExistent,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "yellow")]
    Yellow,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[doc = "The input type."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MlInputDataType {
    #[serde(rename = "DATA_FRAME")]
    DataFrame,
    #[serde(rename = "QUESTION_ANSWERING")]
    QuestionAnswering,
    #[serde(rename = "REMOTE")]
    Remote,
    #[serde(rename = "SEARCH_QUERY")]
    SearchQuery,
    #[serde(rename = "TEXT_DOCS")]
    TextDocs,
    #[serde(rename = "TEXT_SIMILARITY")]
    TextSimilarity,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[doc = "The result data type."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MlResultDataType {
    #[serde(rename = "BOOLEAN")]
    Boolean,
    #[serde(rename = "FLOAT16")]
    Float16,
    #[serde(rename = "FLOAT32")]
    Float32,
    #[serde(rename = "FLOAT64")]
    Float64,
    #[serde(rename = "INT32")]
    Int32,
    #[serde(rename = "INT64")]
    Int64,
    #[serde(rename = "INT8")]
    Int8,
    #[serde(rename = "STRING")]
    String,
    #[serde(rename = "UINT8")]
    Uint8,
    #[serde(rename = "UNKNOWN")]
    Value10,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[doc = "Task type."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MlTaskType {
    #[serde(rename = "BATCH_INGEST")]
    BatchIngest,
    #[serde(rename = "BATCH_PREDICTION")]
    BatchPrediction,
    #[serde(rename = "DEPLOY_MODEL")]
    DeployModel,
    #[serde(rename = "EXECUTION")]
    Execution,
    #[serde(rename = "PREDICTION")]
    Prediction,
    #[serde(rename = "REGISTER_MODEL")]
    RegisterModel,
    #[serde(rename = "TRAINING")]
    Training,
    #[serde(rename = "TRAINING_AND_PREDICTION")]
    TrainingAndPrediction,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Model {
    #[doc = "The algorithm."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    #[doc = "The created time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    #[doc = "Whether the model is hidden."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    #[doc = "The last registered time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_registered_time: Option<i64>,
    #[doc = "The last updated time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_config: Option<crate::types::ml_common::ModelConfig>,
    #[doc = "The model content hash value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_content_hash_value: Option<String>,
    #[doc = "The model content size in bytes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_content_size_in_bytes: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_format: Option<crate::types::ml_common::ModelFormat>,
    #[doc = "The model group ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_group_id: Option<String>,
    pub model_state: crate::types::ml_common::ModelState,
    #[doc = "The model version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
    #[doc = "The model name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The total chunks."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_chunks: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModelConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_config: Option<crate::types::ml_common::AdditionalConfig>,
    #[doc = "The all config."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_config: Option<String>,
    #[doc = "The embedding dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embedding_dimension: Option<i64>,
    #[doc = "The framework type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub framework_type: Option<String>,
    #[doc = "The model type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_type: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ModelFormat {
    #[serde(rename = "ONNX")]
    Onnx,
    #[serde(rename = "TORCH_SCRIPT")]
    TorchScript,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModelGroup {
    pub access: crate::types::ml_common::ModelGroupAccessMode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    #[doc = "The model group description."]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
    #[doc = "The latest version."]
    pub latest_version: i32,
    #[doc = "The model group name."]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<crate::types::ml_common::Owner>,
}

#[doc = "The model group access mode."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ModelGroupAccessMode {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "restricted")]
    Restricted,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModelGroupRegistration {
    #[doc = "The model group ID."]
    pub model_group_id: String,
    #[doc = "The status."]
    pub status: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModelProfile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deploy: Option<crate::types::ml_common::ModelStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execute: Option<crate::types::ml_common::ModelStats>,
    #[doc = "The estimated memory size in CPU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_size_estimation_cpu: Option<i64>,
    #[doc = "The estimated memory size in GPU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_size_estimation_gpu: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_state: Option<crate::types::ml_common::ModelState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predict: Option<crate::types::ml_common::ModelStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predict_request_stats: Option<crate::types::ml_common::PredictRequestStats>,
    #[doc = "The predictor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predictor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub register: Option<crate::types::ml_common::ModelStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_worker_nodes: Option<Vec<crate::types::common::NodeIds>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub train: Option<crate::types::ml_common::ModelStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub train_predict: Option<crate::types::ml_common::ModelStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub undeploy: Option<crate::types::ml_common::ModelStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker_nodes: Option<Vec<crate::types::common::NodeIds>>,
}

#[doc = "The model state."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ModelState {
    #[serde(rename = "DEPLOYED")]
    Deployed,
    #[serde(rename = "DEPLOYING")]
    Deploying,
    #[serde(rename = "DEPLOY_FAILED")]
    DeployFailed,
    #[serde(rename = "PARTIALLY_DEPLOYED")]
    PartiallyDeployed,
    #[serde(rename = "REGISTERED")]
    Registered,
    #[serde(rename = "REGISTERING")]
    Registering,
    #[serde(rename = "UNDEPLOYED")]
    Undeployed,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModelStats {
    #[doc = "The failure count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ml_action_failure_count: Option<i64>,
    #[doc = "The request count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ml_action_request_count: Option<i64>,
    #[doc = "The executing task count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ml_executing_task_count: Option<i64>,
}

pub type Models = std::collections::BTreeMap<String, crate::types::ml_common::ModelProfile>;

pub type Namespace = std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Node {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub models: Option<crate::types::ml_common::Models>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasks: Option<crate::types::ml_common::Tasks>,
}

pub type NodeStats = std::collections::BTreeMap<String, crate::types::ml_common::NodeStatsDetails>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeStatsDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algorithms: Option<crate::types::ml_common::Algorithms>,
    #[doc = "The circuit breaker trigger count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ml_circuit_breaker_trigger_count: Option<i64>,
    #[doc = "The deployed model count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ml_deployed_model_count: Option<i64>,
    #[doc = "The executing task count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ml_executing_task_count: Option<i64>,
    #[doc = "The failure count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ml_failure_count: Option<i64>,
    #[doc = "The JVM heap usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ml_jvm_heap_usage: Option<i64>,
    #[doc = "The request count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ml_request_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub models: Option<crate::types::ml_common::Models>,
}

pub type Nodes = std::collections::BTreeMap<String, crate::types::ml_common::Node>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Output {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub byte_buffer: Option<crate::types::ml_common::ByteBuffer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<f64>>,
    #[serde(rename = "dataAsMap")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_as_map: Option<crate::types::ml_common::DataAsMap>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<crate::types::ml_common::MlResultDataType>,
    #[doc = "The output name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The output result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<Vec<i64>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Owner {
    #[doc = "The backend roles."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend_roles: Option<Vec<String>>,
    #[doc = "The custom attribute names."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_attribute_names: Option<Vec<String>>,
    pub name: crate::types::common::Name,
    #[doc = "The roles."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[doc = "The user requested tenant."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_requested_tenant: Option<String>,
    #[doc = "The user requested tenant access."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_requested_tenant_access: Option<String>,
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
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_llm_interface: Option<String> = None;
                let mut f_inputs: Option<String> = None;
                let mut f_messages: Option<Vec<crate::types::ml_common::Messages>> = None;
                let mut f_question: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_llm_interface" => f_llm_interface = map.next_value::<Option<String>>()?,
                        "inputs" => f_inputs = map.next_value::<Option<String>>()?,
                        "messages" => {
                            f_messages =
                                map.next_value::<Option<Vec<crate::types::ml_common::Messages>>>()?
                        }
                        "question" => f_question = map.next_value::<Option<String>>()?,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PayloadType {
    #[serde(rename = "conversational")]
    Conversational,
    #[serde(rename = "data")]
    Data,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PredictModelOutput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub byte_buffer: Option<crate::types::ml_common::ByteBuffer>,
    pub data: Vec<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<crate::types::ml_common::MlResultDataType>,
    #[doc = "The output name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<Vec<i64>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PredictModelResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inference_results: Option<Vec<crate::types::ml_common::PredictModelResult>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PredictModelResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<Vec<crate::types::ml_common::PredictModelOutput>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PredictRequestStats {
    #[doc = "The average latency in milliseconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average: Option<f64>,
    #[doc = "The total predict requests on this node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "The maximum latency in milliseconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[doc = "The minimum latency in milliseconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[doc = "The 50th percentile latency in milliseconds."]
    #[serde(rename = "p50")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p_50: Option<f64>,
    #[doc = "The 90th percentile latency in milliseconds."]
    #[serde(rename = "p90")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p_90: Option<f64>,
    #[doc = "The 99th percentile latency in milliseconds."]
    #[serde(rename = "p99")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p_99: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PredictResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inference_results: Option<Vec<crate::types::ml_common::InferenceResults>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prediction_result: Option<crate::types::ml_common::PredictionResult>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::types::ml_common::Status>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PredictionResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_metas: Option<Vec<crate::types::ml_common::ColumnMeta>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<crate::types::ml_common::Rows>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RateLimiter {
    #[doc = "The maximum limit."]
    pub limit: crate::types::common::StringifiedDouble,
    #[doc = "The unit of time."]
    pub unit: crate::types::ml_common::RateLimiterUnit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RateLimiterUnit {
    #[serde(rename = "DAYS")]
    Days,
    #[serde(rename = "HOURS")]
    Hours,
    #[serde(rename = "MICROSECONDS")]
    Microseconds,
    #[serde(rename = "MILLISECONDS")]
    Milliseconds,
    #[serde(rename = "MINUTES")]
    Minutes,
    #[serde(rename = "NANOSECONDS")]
    Nanoseconds,
    #[serde(rename = "SECONDS")]
    Seconds,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Result {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Vec<crate::types::ml_common::Buckets>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Rows {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<crate::types::ml_common::Values>>,
}

pub type SearchAgentsResponse = crate::types::ml_common::SearchResponse;

pub type SearchConnectorsResponse = crate::types::ml_common::SearchResponse;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchHits {
    pub hits: Vec<crate::types::ml_common::SearchHitsHit>,
    #[doc = "The maximum score."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_score: Option<f64>,
    pub total: crate::types::ml_common::HitsTotal,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchHitsHit {
    #[serde(rename = "_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::types::common::Id>,
    #[serde(rename = "_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<crate::types::common::IndexName>,
    #[doc = "The primary term."]
    #[serde(rename = "_primary_term")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_term: Option<i32>,
    #[doc = "The score."]
    #[serde(rename = "_score")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(rename = "_seq_no")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    #[serde(rename = "_source")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::types::ml_common::Source>,
    #[serde(rename = "_version")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionNumber>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_id: Option<crate::types::common::Name>,
    #[doc = "The sort values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<f64>>,
}

pub type SearchMemoryResponse = crate::types::ml_common::SearchResponse;

pub type SearchMessageResponse = crate::types::ml_common::SearchResponse;

pub type SearchModelGroupsResponse = crate::types::ml_common::SearchResponse;

pub type SearchModelsResponse = crate::types::ml_common::SearchResponse;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchResponse {
    #[serde(rename = "_shards")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shards: Option<crate::types::common::ShardStatistics>,
    pub hits: crate::types::ml_common::SearchHits,
    #[doc = "Whether the search timed out."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timed_out: Option<bool>,
    #[doc = "The time taken to execute the search."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub took: Option<i32>,
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
    pub latest_version: Option<i32>,
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
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_access: Option<crate::types::ml_common::ModelGroupAccessMode> = None;
                let mut f_actions: Option<Vec<crate::types::ml_common::Action>> = None;
                let mut f_additional_info: Option<crate::types::ml_common::AdditionalInfo> = None;
                let mut f_algorithm: Option<String> = None;
                let mut f_app_type: Option<String> = None;
                let mut f_application_type: Option<String> = None;
                let mut f_auto_redeploy_retry_times: Option<i64> = None;
                let mut f_backend_roles: Option<Vec<String>> = None;
                let mut f_chunk_number: Option<i64> = None;
                let mut f_configuration: Option<
                    crate::types::ml_common::MemoryContainerConfiguration,
                > = None;
                let mut f_connector_id: Option<String> = None;
                let mut f_create_time: Option<crate::types::ml_common::SourceCreateTime> = None;
                let mut f_created_time: Option<i64> = None;
                let mut f_current_worker_node_count: Option<f64> = None;
                let mut f_deploy_to_all_nodes: Option<bool> = None;
                let mut f_description: Option<String> = None;
                let mut f_error: Option<String> = None;
                let mut f_function_name: Option<crate::types::ml_common::FunctionName> = None;
                let mut f_input: Option<String> = None;
                let mut f_input_type: Option<crate::types::ml_common::MlInputDataType> = None;
                let mut f_is_async: Option<bool> = None;
                let mut f_is_hidden: Option<bool> = None;
                let mut f_last_deployed_time: Option<i64> = None;
                let mut f_last_registered_time: Option<i64> = None;
                let mut f_last_update_time: Option<i64> = None;
                let mut f_last_updated_time: Option<i64> = None;
                let mut f_latest_version: Option<i32> = None;
                let mut f_memory: Option<crate::types::ml_common::Memory> = None;
                let mut f_memory_id: Option<crate::types::common::Name> = None;
                let mut f_metadata: Option<crate::types::ml_common::Metadata> = None;
                let mut f_model_config: Option<crate::types::ml_common::ModelConfig> = None;
                let mut f_model_content_hash_value: Option<String> = None;
                let mut f_model_content_size_in_bytes: Option<i64> = None;
                let mut f_model_format: Option<crate::types::ml_common::ModelFormat> = None;
                let mut f_model_group_id: Option<String> = None;
                let mut f_model_id: Option<crate::types::common::Name> = None;
                let mut f_model_state: Option<crate::types::ml_common::ModelState> = None;
                let mut f_model_task_type: Option<String> = None;
                let mut f_model_version: Option<String> = None;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut f_namespace: Option<crate::types::ml_common::Namespace> = None;
                let mut f_namespace_size: Option<i64> = None;
                let mut f_origin: Option<String> = None;
                let mut f_owner: Option<crate::types::ml_common::Owner> = None;
                let mut f_owner_id: Option<String> = None;
                let mut f_parameters: Option<crate::types::ml_common::Parameters> = None;
                let mut f_parent_message_id: Option<String> = None;
                let mut f_payload_type: Option<crate::types::ml_common::PayloadType> = None;
                let mut f_planning_worker_node_count: Option<f64> = None;
                let mut f_planning_worker_nodes: Option<Vec<crate::types::common::NodeIds>> = None;
                let mut f_prompt_template: Option<String> = None;
                let mut f_protocol: Option<crate::types::ml_common::ConnectorProtocol> = None;
                let mut f_response: Option<String> = None;
                let mut f_state: Option<crate::types::ml_common::Status> = None;
                let mut f_task_type: Option<crate::types::ml_common::MlTaskType> = None;
                let mut f_tools: Option<Vec<crate::types::ml_common::ToolItems>> = None;
                let mut f_total_chunks: Option<i64> = None;
                let mut f_trace_number: Option<String> = None;
                let mut f_type: Option<crate::types::ml_common::AgentType> = None;
                let mut f_updated_time: Option<String> = None;
                let mut f_url: Option<String> = None;
                let mut f_user: Option<String> = None;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut f_worker_node: Option<Vec<crate::types::common::NodeIds>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "access" => f_access = map.next_value::<Option<crate::types::ml_common::ModelGroupAccessMode>>()?,
                        "actions" => f_actions = map.next_value::<Option<Vec<crate::types::ml_common::Action>>>()?,
                        "additional_info" => f_additional_info = map.next_value::<Option<crate::types::ml_common::AdditionalInfo>>()?,
                        "algorithm" => f_algorithm = map.next_value::<Option<String>>()?,
                        "app_type" => f_app_type = map.next_value::<Option<String>>()?,
                        "application_type" => f_application_type = map.next_value::<Option<String>>()?,
                        "auto_redeploy_retry_times" => f_auto_redeploy_retry_times = map.next_value::<Option<i64>>()?,
                        "backend_roles" => f_backend_roles = map.next_value::<Option<Vec<String>>>()?,
                        "chunk_number" => f_chunk_number = map.next_value::<Option<i64>>()?,
                        "configuration" => f_configuration = map.next_value::<Option<crate::types::ml_common::MemoryContainerConfiguration>>()?,
                        "connector_id" => f_connector_id = map.next_value::<Option<String>>()?,
                        "create_time" => f_create_time = map.next_value::<Option<crate::types::ml_common::SourceCreateTime>>()?,
                        "created_time" => f_created_time = map.next_value::<Option<i64>>()?,
                        "current_worker_node_count" => f_current_worker_node_count = map.next_value::<Option<f64>>()?,
                        "deploy_to_all_nodes" => f_deploy_to_all_nodes = map.next_value::<Option<bool>>()?,
                        "description" => f_description = map.next_value::<Option<String>>()?,
                        "error" => f_error = map.next_value::<Option<String>>()?,
                        "function_name" => f_function_name = map.next_value::<Option<crate::types::ml_common::FunctionName>>()?,
                        "input" => f_input = map.next_value::<Option<String>>()?,
                        "input_type" => f_input_type = map.next_value::<Option<crate::types::ml_common::MlInputDataType>>()?,
                        "is_async" => f_is_async = map.next_value::<Option<bool>>()?,
                        "is_hidden" => f_is_hidden = map.next_value::<Option<bool>>()?,
                        "last_deployed_time" => f_last_deployed_time = map.next_value::<Option<i64>>()?,
                        "last_registered_time" => f_last_registered_time = map.next_value::<Option<i64>>()?,
                        "last_update_time" => f_last_update_time = map.next_value::<Option<i64>>()?,
                        "last_updated_time" => f_last_updated_time = map.next_value::<Option<i64>>()?,
                        "latest_version" => f_latest_version = map.next_value::<Option<i32>>()?,
                        "memory" => f_memory = map.next_value::<Option<crate::types::ml_common::Memory>>()?,
                        "memory_id" => f_memory_id = map.next_value::<Option<crate::types::common::Name>>()?,
                        "metadata" => f_metadata = map.next_value::<Option<crate::types::ml_common::Metadata>>()?,
                        "model_config" => f_model_config = map.next_value::<Option<crate::types::ml_common::ModelConfig>>()?,
                        "model_content_hash_value" => f_model_content_hash_value = map.next_value::<Option<String>>()?,
                        "model_content_size_in_bytes" => f_model_content_size_in_bytes = map.next_value::<Option<i64>>()?,
                        "model_format" => f_model_format = map.next_value::<Option<crate::types::ml_common::ModelFormat>>()?,
                        "model_group_id" => f_model_group_id = map.next_value::<Option<String>>()?,
                        "model_id" => f_model_id = map.next_value::<Option<crate::types::common::Name>>()?,
                        "model_state" => f_model_state = map.next_value::<Option<crate::types::ml_common::ModelState>>()?,
                        "model_task_type" => f_model_task_type = map.next_value::<Option<String>>()?,
                        "model_version" => f_model_version = map.next_value::<Option<String>>()?,
                        "name" => f_name = map.next_value::<Option<crate::types::common::Name>>()?,
                        "namespace" => f_namespace = map.next_value::<Option<crate::types::ml_common::Namespace>>()?,
                        "namespace_size" => f_namespace_size = map.next_value::<Option<i64>>()?,
                        "origin" => f_origin = map.next_value::<Option<String>>()?,
                        "owner" => f_owner = map.next_value::<Option<crate::types::ml_common::Owner>>()?,
                        "owner_id" => f_owner_id = map.next_value::<Option<String>>()?,
                        "parameters" => f_parameters = map.next_value::<Option<crate::types::ml_common::Parameters>>()?,
                        "parent_message_id" => f_parent_message_id = map.next_value::<Option<String>>()?,
                        "payload_type" => f_payload_type = map.next_value::<Option<crate::types::ml_common::PayloadType>>()?,
                        "planning_worker_node_count" => f_planning_worker_node_count = map.next_value::<Option<f64>>()?,
                        "planning_worker_nodes" => f_planning_worker_nodes = map.next_value::<Option<Vec<crate::types::common::NodeIds>>>()?,
                        "prompt_template" => f_prompt_template = map.next_value::<Option<String>>()?,
                        "protocol" => f_protocol = map.next_value::<Option<crate::types::ml_common::ConnectorProtocol>>()?,
                        "response" => f_response = map.next_value::<Option<String>>()?,
                        "state" => f_state = map.next_value::<Option<crate::types::ml_common::Status>>()?,
                        "task_type" => f_task_type = map.next_value::<Option<crate::types::ml_common::MlTaskType>>()?,
                        "tools" => f_tools = map.next_value::<Option<Vec<crate::types::ml_common::ToolItems>>>()?,
                        "total_chunks" => f_total_chunks = map.next_value::<Option<i64>>()?,
                        "trace_number" => f_trace_number = map.next_value::<Option<String>>()?,
                        "type" => f_type = map.next_value::<Option<crate::types::ml_common::AgentType>>()?,
                        "updated_time" => f_updated_time = map.next_value::<Option<String>>()?,
                        "url" => f_url = map.next_value::<Option<String>>()?,
                        "user" => f_user = map.next_value::<Option<String>>()?,
                        "version" => f_version = map.next_value::<Option<crate::types::common::VersionString>>()?,
                        "worker_node" => f_worker_node = map.next_value::<Option<Vec<crate::types::common::NodeIds>>>()?,
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
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = SourceCreateTime;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SourceCreateTime value")
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(SourceCreateTime::String)
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::I64Deserializer::new(v))
                    .map(SourceCreateTime::Integer)
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::U64Deserializer::new(v))
                    .map(SourceCreateTime::Integer)
            }
            fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::F64Deserializer::new(v))
                    .map(SourceCreateTime::Integer)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[doc = "The status."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Status {
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "COMPLETED_WITH_ERROR")]
    CompletedWithError,
    #[serde(rename = "CREATED")]
    Created,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "RUNNING")]
    Running,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Strategy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<crate::types::ml_common::StrategyConfiguration>,
    #[doc = "Whether to enable the strategy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The strategy ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::types::ml_common::StrategyType>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StrategyConfiguration {
    #[doc = "The LLM ID for strategy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub llm_id: Option<String>,
    #[doc = "JSONPath expression for extracting LLM results from responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub llm_result_path: Option<String>,
    #[doc = "Custom system prompt to override default strategy prompt."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
}

#[doc = "The strategy type."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StrategyType {
    #[serde(rename = "SEMANTIC")]
    Semantic,
    #[serde(rename = "SUMMARY")]
    Summary,
    #[serde(rename = "USER_PREFERENCE")]
    UserPreference,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Task {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function_name: Option<crate::types::ml_common::FunctionName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_async: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    pub state: crate::types::ml_common::TaskState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_type: Option<crate::types::ml_common::MlTaskType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker_node: Option<Vec<crate::types::common::NodeIds>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TaskState {
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "CANCELLING")]
    Cancelling,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "COMPLETED_WITH_ERROR")]
    CompletedWithError,
    #[serde(rename = "CREATED")]
    Created,
    #[serde(rename = "EXPIRED")]
    Expired,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "RUNNING")]
    Running,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

pub type Tasks = std::collections::BTreeMap<String, crate::types::ml_common::Task>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tool {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<crate::types::ml_common::ToolAttributes>,
    #[doc = "The tool description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<crate::types::common::Name>,
    #[doc = "The tool type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
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
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_input_schema: Option<String> = None;
                let mut f_strict: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "input_schema" => f_input_schema = map.next_value::<Option<String>>()?,
                        "strict" => f_strict = map.next_value::<Option<bool>>()?,
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
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_attributes: Option<crate::types::ml_common::ToolAttributes> = None;
                let mut f_description: Option<String> = None;
                let mut f_include_output_in_agent_response: Option<bool> = None;
                let mut f_name: Option<String> = None;
                let mut f_parameters: Option<crate::types::ml_common::Parameters> = None;
                let mut f_type: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "attributes" => {
                            f_attributes =
                                map.next_value::<Option<crate::types::ml_common::ToolAttributes>>()?
                        }
                        "description" => f_description = map.next_value::<Option<String>>()?,
                        "include_output_in_agent_response" => {
                            f_include_output_in_agent_response = map.next_value::<Option<bool>>()?
                        }
                        "name" => f_name = map.next_value::<Option<String>>()?,
                        "parameters" => {
                            f_parameters =
                                map.next_value::<Option<crate::types::ml_common::Parameters>>()?
                        }
                        "type" => f_type = map.next_value::<Option<String>>()?,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TrainPredictResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prediction_result: Option<crate::types::ml_common::PredictionResult>,
    pub status: crate::types::ml_common::Status,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TrainResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_id: Option<crate::types::common::Name>,
    pub status: crate::types::ml_common::Status,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UndeployModelNode {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<crate::types::ml_common::UndeployModelNodeStats>,
}

pub type UndeployModelNodeStats =
    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

pub type UndeployModelResponse =
    std::collections::BTreeMap<String, crate::types::ml_common::UndeployModelNode>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnloadModelNode {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<crate::types::ml_common::UnloadModelNodeStats>,
}

pub type UnloadModelNodeStats =
    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

pub type UnloadModelResponse =
    std::collections::BTreeMap<String, crate::types::ml_common::UnloadModelNode>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateModelGroupResponse {
    #[doc = "The status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Values {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_type: Option<crate::types::ml_common::ColumnType>,
    #[doc = "The value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
