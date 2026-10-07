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

//! Response types of the `flow_framework_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct FlowFrameworkGetResponse {
    #[doc = "When the workflow was created."]
    pub created_time: Option<i64>,
    #[doc = "The description of the workflow."]
    pub description: Option<String>,
    #[doc = "When the workflow was last provisioned."]
    pub last_provisioned_time: Option<f64>,
    #[doc = "When the workflow was last updated."]
    pub last_updated_time: Option<i64>,
    #[doc = "The name of the workflow."]
    pub name: Option<String>,
    #[doc = "A use case ID, which can be used with the Search Workflow API to find related workflows."]
    pub use_case: Option<String>,
    pub user: Option<crate::types::flow_framework_common::User>,
    pub version: Option<crate::types::flow_framework_common::Version>,
    pub workflows: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FlowFrameworkGetResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FlowFrameworkGetResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FlowFrameworkGetResponse object")
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
                let mut f_last_provisioned_time: Option<f64> = None;
                let mut seen_last_provisioned_time = false;
                let mut f_last_updated_time: Option<i64> = None;
                let mut seen_last_updated_time = false;
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_use_case: Option<String> = None;
                let mut seen_use_case = false;
                let mut f_user: Option<crate::types::flow_framework_common::User> = None;
                let mut seen_user = false;
                let mut f_version: Option<crate::types::flow_framework_common::Version> = None;
                let mut seen_version = false;
                let mut f_workflows: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_workflows = false;
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
                        "last_provisioned_time" => {
                            if seen_last_provisioned_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "last_provisioned_time",
                                ));
                            }
                            seen_last_provisioned_time = true;
                            f_last_provisioned_time = map.next_value::<Option<f64>>()?;
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
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "use_case" => {
                            if seen_use_case {
                                return Err(serde::de::Error::duplicate_field("use_case"));
                            }
                            seen_use_case = true;
                            f_use_case = map.next_value::<Option<String>>()?;
                        }
                        "user" => {
                            if seen_user {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            seen_user = true;
                            f_user = map
                                .next_value::<Option<crate::types::flow_framework_common::User>>(
                                )?;
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version = map
                                .next_value::<Option<crate::types::flow_framework_common::Version>>(
                                )?;
                        }
                        "workflows" => {
                            if seen_workflows {
                                return Err(serde::de::Error::duplicate_field("workflows"));
                            }
                            seen_workflows = true;
                            f_workflows = map.next_value::<Option<
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
                Ok(FlowFrameworkGetResponse {
                    created_time: f_created_time,
                    description: f_description,
                    last_provisioned_time: f_last_provisioned_time,
                    last_updated_time: f_last_updated_time,
                    name: f_name,
                    use_case: f_use_case,
                    user: f_user,
                    version: f_version,
                    workflows: f_workflows,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FlowFrameworkGetResponse {
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
        if let Some(v) = &self.last_provisioned_time {
            map.serialize_entry("last_provisioned_time", v)?;
        }
        if let Some(v) = &self.last_updated_time {
            map.serialize_entry("last_updated_time", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.use_case {
            map.serialize_entry("use_case", v)?;
        }
        if let Some(v) = &self.user {
            map.serialize_entry("user", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.workflows {
            map.serialize_entry("workflows", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Hits {
    pub hits: Option<Vec<crate::types::flow_framework_common::ItemsObject>>,
    pub max_score: Option<f64>,
    pub total: Option<crate::types::flow_framework_common::Total>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Hits {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Hits;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Hits object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_hits: Option<Vec<crate::types::flow_framework_common::ItemsObject>> =
                    None;
                let mut seen_hits = false;
                let mut f_max_score: Option<f64> = None;
                let mut seen_max_score = false;
                let mut f_total: Option<crate::types::flow_framework_common::Total> = None;
                let mut seen_total = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "hits" => {
                            if seen_hits {
                                return Err(serde::de::Error::duplicate_field("hits"));
                            }
                            seen_hits = true;
                            f_hits = map.next_value::<Option<Vec<crate::types::flow_framework_common::ItemsObject>>>()?;
                        }
                        "max_score" => {
                            if seen_max_score {
                                return Err(serde::de::Error::duplicate_field("max_score"));
                            }
                            seen_max_score = true;
                            f_max_score = map.next_value::<Option<f64>>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map
                                .next_value::<Option<crate::types::flow_framework_common::Total>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Hits {
                    hits: f_hits,
                    max_score: f_max_score,
                    total: f_total,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Hits {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.hits {
            map.serialize_entry("hits", v)?;
        }
        if let Some(v) = &self.max_score {
            map.serialize_entry("max_score", v)?;
        }
        if let Some(v) = &self.total {
            map.serialize_entry("total", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ItemsObject {
    #[doc = "The document ID."]
    pub id: Option<String>,
    #[doc = "The index name where the document resides."]
    pub index: Option<String>,
    #[doc = "The primary term of the document."]
    pub primary_term: Option<i64>,
    #[doc = "The score of the search hit."]
    pub score: Option<f64>,
    #[doc = "The sequence number of the document."]
    pub seq_no: Option<i64>,
    pub source: Option<crate::types::flow_framework_common::FlowFrameworkGetResponse>,
    #[doc = "The version of the document."]
    pub version: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ItemsObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ItemsObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ItemsObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_id: Option<String> = None;
                let mut seen_id = false;
                let mut f_index: Option<String> = None;
                let mut seen_index = false;
                let mut f_primary_term: Option<i64> = None;
                let mut seen_primary_term = false;
                let mut f_score: Option<f64> = None;
                let mut seen_score = false;
                let mut f_seq_no: Option<i64> = None;
                let mut seen_seq_no = false;
                let mut f_source: Option<
                    crate::types::flow_framework_common::FlowFrameworkGetResponse,
                > = None;
                let mut seen_source = false;
                let mut f_version: Option<i64> = None;
                let mut seen_version = false;
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
                            f_id = map.next_value::<Option<String>>()?;
                        }
                        "_index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("_index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<String>>()?;
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
                            f_seq_no = map.next_value::<Option<i64>>()?;
                        }
                        "_source" => {
                            if seen_source {
                                return Err(serde::de::Error::duplicate_field("_source"));
                            }
                            seen_source = true;
                            f_source = map.next_value::<Option<
                                crate::types::flow_framework_common::FlowFrameworkGetResponse,
                            >>()?;
                        }
                        "_version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            seen_version = true;
                            f_version = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ItemsObject {
                    id: f_id,
                    index: f_index,
                    primary_term: f_primary_term,
                    score: f_score,
                    seq_no: f_seq_no,
                    source: f_source,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ItemsObject {
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
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ResourcesCreated {
    #[doc = "The unique identifier for the created resource."]
    pub resource_id: Option<String>,
    #[doc = "The type of resource created."]
    pub resource_type: Option<String>,
    #[doc = "The unique identifier for the workflow step."]
    pub workflow_step_id: Option<String>,
    #[doc = "The name of the workflow step."]
    pub workflow_step_name: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ResourcesCreated {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ResourcesCreated;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ResourcesCreated object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_resource_id: Option<String> = None;
                let mut seen_resource_id = false;
                let mut f_resource_type: Option<String> = None;
                let mut seen_resource_type = false;
                let mut f_workflow_step_id: Option<String> = None;
                let mut seen_workflow_step_id = false;
                let mut f_workflow_step_name: Option<String> = None;
                let mut seen_workflow_step_name = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "resource_id" => {
                            if seen_resource_id {
                                return Err(serde::de::Error::duplicate_field("resource_id"));
                            }
                            seen_resource_id = true;
                            f_resource_id = map.next_value::<Option<String>>()?;
                        }
                        "resource_type" => {
                            if seen_resource_type {
                                return Err(serde::de::Error::duplicate_field("resource_type"));
                            }
                            seen_resource_type = true;
                            f_resource_type = map.next_value::<Option<String>>()?;
                        }
                        "workflow_step_id" => {
                            if seen_workflow_step_id {
                                return Err(serde::de::Error::duplicate_field("workflow_step_id"));
                            }
                            seen_workflow_step_id = true;
                            f_workflow_step_id = map.next_value::<Option<String>>()?;
                        }
                        "workflow_step_name" => {
                            if seen_workflow_step_name {
                                return Err(serde::de::Error::duplicate_field(
                                    "workflow_step_name",
                                ));
                            }
                            seen_workflow_step_name = true;
                            f_workflow_step_name = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ResourcesCreated {
                    resource_id: f_resource_id,
                    resource_type: f_resource_type,
                    workflow_step_id: f_workflow_step_id,
                    workflow_step_name: f_workflow_step_name,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ResourcesCreated {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.resource_id {
            map.serialize_entry("resource_id", v)?;
        }
        if let Some(v) = &self.resource_type {
            map.serialize_entry("resource_type", v)?;
        }
        if let Some(v) = &self.workflow_step_id {
            map.serialize_entry("workflow_step_id", v)?;
        }
        if let Some(v) = &self.workflow_step_name {
            map.serialize_entry("workflow_step_name", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SearchStateResponse {
    #[doc = "When the provisioning operation completed."]
    pub provision_end_time: Option<crate::types::common::DateTime>,
    #[doc = "When the provisioning operation started."]
    pub provision_start_time: Option<crate::types::common::DateTime>,
    #[doc = "The current progress status of the provisioning operation."]
    pub provisioning_progress: Option<String>,
    pub resources_created:
        Option<crate::types::flow_framework_common::SearchStateResponseResourcesCreated>,
    #[doc = "The current state of the workflow."]
    pub state: Option<String>,
    pub user: Option<crate::types::flow_framework_common::User>,
    #[doc = "The ID of the workflow."]
    pub workflow_id: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SearchStateResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SearchStateResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchStateResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_provision_end_time: Option<crate::types::common::DateTime> = None;
                let mut seen_provision_end_time = false;
                let mut f_provision_start_time: Option<crate::types::common::DateTime> = None;
                let mut seen_provision_start_time = false;
                let mut f_provisioning_progress: Option<String> = None;
                let mut seen_provisioning_progress = false;
                let mut f_resources_created: Option<
                    crate::types::flow_framework_common::SearchStateResponseResourcesCreated,
                > = None;
                let mut seen_resources_created = false;
                let mut f_state: Option<String> = None;
                let mut seen_state = false;
                let mut f_user: Option<crate::types::flow_framework_common::User> = None;
                let mut seen_user = false;
                let mut f_workflow_id: Option<String> = None;
                let mut seen_workflow_id = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "provision_end_time" => {
                            if seen_provision_end_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "provision_end_time",
                                ));
                            }
                            seen_provision_end_time = true;
                            f_provision_end_time =
                                map.next_value::<Option<crate::types::common::DateTime>>()?;
                        }
                        "provision_start_time" => {
                            if seen_provision_start_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "provision_start_time",
                                ));
                            }
                            seen_provision_start_time = true;
                            f_provision_start_time =
                                map.next_value::<Option<crate::types::common::DateTime>>()?;
                        }
                        "provisioning_progress" => {
                            if seen_provisioning_progress {
                                return Err(serde::de::Error::duplicate_field(
                                    "provisioning_progress",
                                ));
                            }
                            seen_provisioning_progress = true;
                            f_provisioning_progress = map.next_value::<Option<String>>()?;
                        }
                        "resources_created" => {
                            if seen_resources_created {
                                return Err(serde::de::Error::duplicate_field("resources_created"));
                            }
                            seen_resources_created = true;
                            f_resources_created = map.next_value::<Option<crate::types::flow_framework_common::SearchStateResponseResourcesCreated>>()?;
                        }
                        "state" => {
                            if seen_state {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            seen_state = true;
                            f_state = map.next_value::<Option<String>>()?;
                        }
                        "user" => {
                            if seen_user {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            seen_user = true;
                            f_user = map
                                .next_value::<Option<crate::types::flow_framework_common::User>>(
                                )?;
                        }
                        "workflow_id" => {
                            if seen_workflow_id {
                                return Err(serde::de::Error::duplicate_field("workflow_id"));
                            }
                            seen_workflow_id = true;
                            f_workflow_id = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SearchStateResponse {
                    provision_end_time: f_provision_end_time,
                    provision_start_time: f_provision_start_time,
                    provisioning_progress: f_provisioning_progress,
                    resources_created: f_resources_created,
                    state: f_state,
                    user: f_user,
                    workflow_id: f_workflow_id,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SearchStateResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.provision_end_time {
            map.serialize_entry("provision_end_time", v)?;
        }
        if let Some(v) = &self.provision_start_time {
            map.serialize_entry("provision_start_time", v)?;
        }
        if let Some(v) = &self.provisioning_progress {
            map.serialize_entry("provisioning_progress", v)?;
        }
        if let Some(v) = &self.resources_created {
            map.serialize_entry("resources_created", v)?;
        }
        if let Some(v) = &self.state {
            map.serialize_entry("state", v)?;
        }
        if let Some(v) = &self.user {
            map.serialize_entry("user", v)?;
        }
        if let Some(v) = &self.workflow_id {
            map.serialize_entry("workflow_id", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SearchStateResponseResourcesCreated {
    ResourcesCreated(crate::types::flow_framework_common::ResourcesCreated),
    Array(Vec<crate::types::flow_framework_common::ResourcesCreated>),
}
impl<'de> serde::Deserialize<'de> for SearchStateResponseResourcesCreated {
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
            Some(b'{') => serde_json::from_str(text)
                .map(SearchStateResponseResourcesCreated::ResourcesCreated),
            Some(b'[') => {
                serde_json::from_str(text).map(SearchStateResponseResourcesCreated::Array)
            }
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a SearchStateResponseResourcesCreated value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct Shards {
    #[doc = "The number of shards where the deletion failed."]
    pub failed: Option<i64>,
    #[doc = "The number of shards where the deletion was skipped."]
    pub skipped: Option<i64>,
    #[doc = "The number of shards where the deletion was successful."]
    pub successful: Option<i64>,
    #[doc = "The total number of shards involved in the deletion."]
    pub total: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Shards {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Shards;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Shards object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_failed: Option<i64> = None;
                let mut seen_failed = false;
                let mut f_skipped: Option<i64> = None;
                let mut seen_skipped = false;
                let mut f_successful: Option<i64> = None;
                let mut seen_successful = false;
                let mut f_total: Option<i64> = None;
                let mut seen_total = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "failed" => {
                            if seen_failed {
                                return Err(serde::de::Error::duplicate_field("failed"));
                            }
                            seen_failed = true;
                            f_failed = map.next_value::<Option<i64>>()?;
                        }
                        "skipped" => {
                            if seen_skipped {
                                return Err(serde::de::Error::duplicate_field("skipped"));
                            }
                            seen_skipped = true;
                            f_skipped = map.next_value::<Option<i64>>()?;
                        }
                        "successful" => {
                            if seen_successful {
                                return Err(serde::de::Error::duplicate_field("successful"));
                            }
                            seen_successful = true;
                            f_successful = map.next_value::<Option<i64>>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Shards {
                    failed: f_failed,
                    skipped: f_skipped,
                    successful: f_successful,
                    total: f_total,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Shards {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.failed {
            map.serialize_entry("failed", v)?;
        }
        if let Some(v) = &self.skipped {
            map.serialize_entry("skipped", v)?;
        }
        if let Some(v) = &self.successful {
            map.serialize_entry("successful", v)?;
        }
        if let Some(v) = &self.total {
            map.serialize_entry("total", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct StateHits {
    #[doc = "The list of search hits."]
    pub hits: Option<Vec<crate::types::flow_framework_common::StateItems>>,
    pub max_score: Option<f64>,
    pub total: Option<crate::types::flow_framework_common::Total>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StateHits {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StateHits;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StateHits object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_hits: Option<Vec<crate::types::flow_framework_common::StateItems>> = None;
                let mut seen_hits = false;
                let mut f_max_score: Option<f64> = None;
                let mut seen_max_score = false;
                let mut f_total: Option<crate::types::flow_framework_common::Total> = None;
                let mut seen_total = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "hits" => {
                            if seen_hits {
                                return Err(serde::de::Error::duplicate_field("hits"));
                            }
                            seen_hits = true;
                            f_hits = map.next_value::<Option<Vec<crate::types::flow_framework_common::StateItems>>>()?;
                        }
                        "max_score" => {
                            if seen_max_score {
                                return Err(serde::de::Error::duplicate_field("max_score"));
                            }
                            seen_max_score = true;
                            f_max_score = map.next_value::<Option<f64>>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map
                                .next_value::<Option<crate::types::flow_framework_common::Total>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StateHits {
                    hits: f_hits,
                    max_score: f_max_score,
                    total: f_total,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StateHits {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.hits {
            map.serialize_entry("hits", v)?;
        }
        if let Some(v) = &self.max_score {
            map.serialize_entry("max_score", v)?;
        }
        if let Some(v) = &self.total {
            map.serialize_entry("total", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct StateItems {
    #[doc = "The document ID."]
    pub id: Option<String>,
    #[doc = "The index name where the document resides."]
    pub index: Option<String>,
    #[doc = "The primary term of the document."]
    pub primary_term: Option<i64>,
    #[doc = "The score of the search hit."]
    pub score: Option<f64>,
    #[doc = "The sequence number of the document."]
    pub seq_no: Option<i64>,
    pub source: Option<crate::types::flow_framework_common::SearchStateResponse>,
    #[doc = "The version of the document."]
    pub version: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StateItems {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StateItems;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StateItems object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_id: Option<String> = None;
                let mut seen_id = false;
                let mut f_index: Option<String> = None;
                let mut seen_index = false;
                let mut f_primary_term: Option<i64> = None;
                let mut seen_primary_term = false;
                let mut f_score: Option<f64> = None;
                let mut seen_score = false;
                let mut f_seq_no: Option<i64> = None;
                let mut seen_seq_no = false;
                let mut f_source: Option<crate::types::flow_framework_common::SearchStateResponse> =
                    None;
                let mut seen_source = false;
                let mut f_version: Option<i64> = None;
                let mut seen_version = false;
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
                            f_id = map.next_value::<Option<String>>()?;
                        }
                        "_index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("_index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<String>>()?;
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
                            f_seq_no = map.next_value::<Option<i64>>()?;
                        }
                        "_source" => {
                            if seen_source {
                                return Err(serde::de::Error::duplicate_field("_source"));
                            }
                            seen_source = true;
                            f_source =
                                map.next_value::<Option<
                                    crate::types::flow_framework_common::SearchStateResponse,
                                >>()?;
                        }
                        "_version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            seen_version = true;
                            f_version = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StateItems {
                    id: f_id,
                    index: f_index,
                    primary_term: f_primary_term,
                    score: f_score,
                    seq_no: f_seq_no,
                    source: f_source,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StateItems {
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
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Total {
    #[doc = "The relation of the total hits."]
    pub relation: Option<String>,
    #[doc = "The total number of matching documents."]
    pub value: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Total {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Total;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Total object")
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
                let mut seen_relation = false;
                let mut f_value: Option<i64> = None;
                let mut seen_value = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "relation" => {
                            if seen_relation {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            seen_relation = true;
                            f_relation = map.next_value::<Option<String>>()?;
                        }
                        "value" => {
                            if seen_value {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            seen_value = true;
                            f_value = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Total {
                    relation: f_relation,
                    value: f_value,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Total {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.relation {
            map.serialize_entry("relation", v)?;
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

#[derive(Debug, Clone)]
pub struct User {
    #[doc = "The list of backend roles assigned to the user."]
    pub backend_roles: Option<Vec<String>>,
    #[doc = "The list of custom attributes associated with the user."]
    pub custom_attribute_names: Option<Vec<String>>,
    #[doc = "The name of the user."]
    pub name: Option<String>,
    #[doc = "The list of roles assigned to the user."]
    pub roles: Option<Vec<String>>,
    #[doc = "The tenant requested by the user."]
    pub user_requested_tenant: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for User {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = User;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a User object")
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
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_roles: Option<Vec<String>> = None;
                let mut seen_roles = false;
                let mut f_user_requested_tenant: Option<String> = None;
                let mut seen_user_requested_tenant = false;
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
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
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
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(User {
                    backend_roles: f_backend_roles,
                    custom_attribute_names: f_custom_attribute_names,
                    name: f_name,
                    roles: f_roles,
                    user_requested_tenant: f_user_requested_tenant,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for User {
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
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.roles {
            map.serialize_entry("roles", v)?;
        }
        if let Some(v) = &self.user_requested_tenant {
            map.serialize_entry("user_requested_tenant", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "A key-value map with two fields, template, which identifies the template version, and compatibility, which identifies a list of minimum required OpenSearch versions."]
#[derive(Debug, Clone)]
pub struct Version {
    #[doc = "A list of minimum required OpenSearch versions."]
    pub compatibility: Option<Vec<String>>,
    #[doc = "The template version of the workflow."]
    pub template: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Version;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Version object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_compatibility: Option<Vec<String>> = None;
                let mut seen_compatibility = false;
                let mut f_template: Option<String> = None;
                let mut seen_template = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "compatibility" => {
                            if seen_compatibility {
                                return Err(serde::de::Error::duplicate_field("compatibility"));
                            }
                            seen_compatibility = true;
                            f_compatibility = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "template" => {
                            if seen_template {
                                return Err(serde::de::Error::duplicate_field("template"));
                            }
                            seen_template = true;
                            f_template = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Version {
                    compatibility: f_compatibility,
                    template: f_template,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.compatibility {
            map.serialize_entry("compatibility", v)?;
        }
        if let Some(v) = &self.template {
            map.serialize_entry("template", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkFlowState {
    #[doc = "`COMPLETED`"]
    Completed,
    #[doc = "`FAILED`"]
    Failed,
    #[doc = "`NOT_STARTED`"]
    NotStarted,
    #[doc = "`PROVISIONING`"]
    Provisioning,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl WorkFlowState {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            WorkFlowState::Completed => "COMPLETED",
            WorkFlowState::Failed => "FAILED",
            WorkFlowState::NotStarted => "NOT_STARTED",
            WorkFlowState::Provisioning => "PROVISIONING",
            WorkFlowState::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for WorkFlowState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for WorkFlowState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "COMPLETED" => WorkFlowState::Completed,
            "FAILED" => WorkFlowState::Failed,
            "NOT_STARTED" => WorkFlowState::NotStarted,
            "PROVISIONING" => WorkFlowState::Provisioning,
            _ => WorkFlowState::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct WorkFlowStatusDefaultResponse {
    #[doc = "The error message if the workflow failed."]
    pub error: Option<String>,
    #[doc = "A list of resources created by the workflow"]
    pub resources_created: Option<Vec<String>>,
    #[doc = "The current state of the workflow."]
    pub state: Option<String>,
    #[doc = "The ID of the workflow."]
    pub workflow_id: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for WorkFlowStatusDefaultResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = WorkFlowStatusDefaultResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a WorkFlowStatusDefaultResponse object")
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
                let mut f_resources_created: Option<Vec<String>> = None;
                let mut seen_resources_created = false;
                let mut f_state: Option<String> = None;
                let mut seen_state = false;
                let mut f_workflow_id: Option<String> = None;
                let mut seen_workflow_id = false;
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
                        "resources_created" => {
                            if seen_resources_created {
                                return Err(serde::de::Error::duplicate_field("resources_created"));
                            }
                            seen_resources_created = true;
                            f_resources_created = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "state" => {
                            if seen_state {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            seen_state = true;
                            f_state = map.next_value::<Option<String>>()?;
                        }
                        "workflow_id" => {
                            if seen_workflow_id {
                                return Err(serde::de::Error::duplicate_field("workflow_id"));
                            }
                            seen_workflow_id = true;
                            f_workflow_id = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(WorkFlowStatusDefaultResponse {
                    error: f_error,
                    resources_created: f_resources_created,
                    state: f_state,
                    workflow_id: f_workflow_id,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for WorkFlowStatusDefaultResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.error {
            map.serialize_entry("error", v)?;
        }
        if let Some(v) = &self.resources_created {
            map.serialize_entry("resources_created", v)?;
        }
        if let Some(v) = &self.state {
            map.serialize_entry("state", v)?;
        }
        if let Some(v) = &self.workflow_id {
            map.serialize_entry("workflow_id", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct WorkFlowStatusFullResponse {
    #[doc = "The error message if the workflow failed."]
    pub error: Option<String>,
    #[doc = "When the provisioning process completed."]
    pub provision_end_time: Option<String>,
    #[doc = "When the provisioning process started."]
    pub provision_start_time: Option<String>,
    #[doc = "The current progress of the provisioning process."]
    pub provisioning_progress: Option<String>,
    #[doc = "A list of resources created by the workflow."]
    pub resources_created: Option<Vec<String>>,
    #[doc = "The current state of the workflow."]
    pub state: Option<crate::types::flow_framework_common::WorkFlowState>,
    pub user: Option<crate::types::flow_framework_common::User>,
    #[doc = "The list of outputs generated by the user."]
    pub user_outputs: Option<Vec<String>>,
    #[doc = "The ID of the workflow."]
    pub workflow_id: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for WorkFlowStatusFullResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = WorkFlowStatusFullResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a WorkFlowStatusFullResponse object")
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
                let mut f_provision_end_time: Option<String> = None;
                let mut seen_provision_end_time = false;
                let mut f_provision_start_time: Option<String> = None;
                let mut seen_provision_start_time = false;
                let mut f_provisioning_progress: Option<String> = None;
                let mut seen_provisioning_progress = false;
                let mut f_resources_created: Option<Vec<String>> = None;
                let mut seen_resources_created = false;
                let mut f_state: Option<crate::types::flow_framework_common::WorkFlowState> = None;
                let mut seen_state = false;
                let mut f_user: Option<crate::types::flow_framework_common::User> = None;
                let mut seen_user = false;
                let mut f_user_outputs: Option<Vec<String>> = None;
                let mut seen_user_outputs = false;
                let mut f_workflow_id: Option<String> = None;
                let mut seen_workflow_id = false;
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
                        "provision_end_time" => {
                            if seen_provision_end_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "provision_end_time",
                                ));
                            }
                            seen_provision_end_time = true;
                            f_provision_end_time = map.next_value::<Option<String>>()?;
                        }
                        "provision_start_time" => {
                            if seen_provision_start_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "provision_start_time",
                                ));
                            }
                            seen_provision_start_time = true;
                            f_provision_start_time = map.next_value::<Option<String>>()?;
                        }
                        "provisioning_progress" => {
                            if seen_provisioning_progress {
                                return Err(serde::de::Error::duplicate_field(
                                    "provisioning_progress",
                                ));
                            }
                            seen_provisioning_progress = true;
                            f_provisioning_progress = map.next_value::<Option<String>>()?;
                        }
                        "resources_created" => {
                            if seen_resources_created {
                                return Err(serde::de::Error::duplicate_field("resources_created"));
                            }
                            seen_resources_created = true;
                            f_resources_created = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "state" => {
                            if seen_state {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            seen_state = true;
                            f_state = map.next_value::<Option<crate::types::flow_framework_common::WorkFlowState>>()?;
                        }
                        "user" => {
                            if seen_user {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            seen_user = true;
                            f_user = map
                                .next_value::<Option<crate::types::flow_framework_common::User>>(
                                )?;
                        }
                        "user_outputs" => {
                            if seen_user_outputs {
                                return Err(serde::de::Error::duplicate_field("user_outputs"));
                            }
                            seen_user_outputs = true;
                            f_user_outputs = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "workflow_id" => {
                            if seen_workflow_id {
                                return Err(serde::de::Error::duplicate_field("workflow_id"));
                            }
                            seen_workflow_id = true;
                            f_workflow_id = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(WorkFlowStatusFullResponse {
                    error: f_error,
                    provision_end_time: f_provision_end_time,
                    provision_start_time: f_provision_start_time,
                    provisioning_progress: f_provisioning_progress,
                    resources_created: f_resources_created,
                    state: f_state,
                    user: f_user,
                    user_outputs: f_user_outputs,
                    workflow_id: f_workflow_id,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for WorkFlowStatusFullResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.error {
            map.serialize_entry("error", v)?;
        }
        if let Some(v) = &self.provision_end_time {
            map.serialize_entry("provision_end_time", v)?;
        }
        if let Some(v) = &self.provision_start_time {
            map.serialize_entry("provision_start_time", v)?;
        }
        if let Some(v) = &self.provisioning_progress {
            map.serialize_entry("provisioning_progress", v)?;
        }
        if let Some(v) = &self.resources_created {
            map.serialize_entry("resources_created", v)?;
        }
        if let Some(v) = &self.state {
            map.serialize_entry("state", v)?;
        }
        if let Some(v) = &self.user {
            map.serialize_entry("user", v)?;
        }
        if let Some(v) = &self.user_outputs {
            map.serialize_entry("user_outputs", v)?;
        }
        if let Some(v) = &self.workflow_id {
            map.serialize_entry("workflow_id", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct WorkflowIDResponse {
    #[doc = "The ID of the workflow."]
    pub workflow_id: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for WorkflowIDResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = WorkflowIDResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a WorkflowIDResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_workflow_id: Option<String> = None;
                let mut seen_workflow_id = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "workflow_id" => {
                            if seen_workflow_id {
                                return Err(serde::de::Error::duplicate_field("workflow_id"));
                            }
                            seen_workflow_id = true;
                            f_workflow_id = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(WorkflowIDResponse {
                    workflow_id: f_workflow_id,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for WorkflowIDResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.workflow_id {
            map.serialize_entry("workflow_id", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct WorkflowSearchResponse {
    pub shards: Option<crate::types::flow_framework_common::Shards>,
    pub hits: Option<crate::types::flow_framework_common::Hits>,
    #[doc = "Whether the request timed out."]
    pub timed_out: Option<bool>,
    #[doc = "The amount of time in milliseconds that the request took to complete."]
    pub took: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for WorkflowSearchResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = WorkflowSearchResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a WorkflowSearchResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_shards: Option<crate::types::flow_framework_common::Shards> = None;
                let mut seen_shards = false;
                let mut f_hits: Option<crate::types::flow_framework_common::Hits> = None;
                let mut seen_hits = false;
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
                            f_shards = map
                                .next_value::<Option<crate::types::flow_framework_common::Shards>>(
                                )?;
                        }
                        "hits" => {
                            if seen_hits {
                                return Err(serde::de::Error::duplicate_field("hits"));
                            }
                            seen_hits = true;
                            f_hits = map
                                .next_value::<Option<crate::types::flow_framework_common::Hits>>(
                                )?;
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
                Ok(WorkflowSearchResponse {
                    shards: f_shards,
                    hits: f_hits,
                    timed_out: f_timed_out,
                    took: f_took,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for WorkflowSearchResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.shards {
            map.serialize_entry("_shards", v)?;
        }
        if let Some(v) = &self.hits {
            map.serialize_entry("hits", v)?;
        }
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

#[derive(Debug, Clone)]
pub struct WorkflowSearchStateResponse {
    pub shards: Option<crate::types::flow_framework_common::Shards>,
    pub hits: Option<crate::types::flow_framework_common::StateHits>,
    #[doc = "Whether the request timed out."]
    pub timed_out: Option<bool>,
    #[doc = "The amount of time in milliseconds that the request took to complete."]
    pub took: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for WorkflowSearchStateResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = WorkflowSearchStateResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a WorkflowSearchStateResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_shards: Option<crate::types::flow_framework_common::Shards> = None;
                let mut seen_shards = false;
                let mut f_hits: Option<crate::types::flow_framework_common::StateHits> = None;
                let mut seen_hits = false;
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
                            f_shards = map
                                .next_value::<Option<crate::types::flow_framework_common::Shards>>(
                                )?;
                        }
                        "hits" => {
                            if seen_hits {
                                return Err(serde::de::Error::duplicate_field("hits"));
                            }
                            seen_hits = true;
                            f_hits = map.next_value::<Option<crate::types::flow_framework_common::StateHits>>()?;
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
                Ok(WorkflowSearchStateResponse {
                    shards: f_shards,
                    hits: f_hits,
                    timed_out: f_timed_out,
                    took: f_took,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for WorkflowSearchStateResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.shards {
            map.serialize_entry("_shards", v)?;
        }
        if let Some(v) = &self.hits {
            map.serialize_entry("hits", v)?;
        }
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

#[doc = "A map of workflow steps and their configurations."]
#[derive(Debug, Clone)]
pub struct WorkflowStep {
    #[doc = "The list of required inputs for the workflow step."]
    pub inputs: Option<Vec<String>>,
    #[doc = "The list of outputs generated by the workflow step."]
    pub outputs: Option<Vec<String>>,
    #[doc = "The list of plugins required for the workflow step."]
    pub required_plugins: Option<Vec<String>>,
    pub timeout: Option<crate::types::common::Duration>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for WorkflowStep {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = WorkflowStep;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a WorkflowStep object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_inputs: Option<Vec<String>> = None;
                let mut seen_inputs = false;
                let mut f_outputs: Option<Vec<String>> = None;
                let mut seen_outputs = false;
                let mut f_required_plugins: Option<Vec<String>> = None;
                let mut seen_required_plugins = false;
                let mut f_timeout: Option<crate::types::common::Duration> = None;
                let mut seen_timeout = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "inputs" => {
                            if seen_inputs {
                                return Err(serde::de::Error::duplicate_field("inputs"));
                            }
                            seen_inputs = true;
                            f_inputs = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "outputs" => {
                            if seen_outputs {
                                return Err(serde::de::Error::duplicate_field("outputs"));
                            }
                            seen_outputs = true;
                            f_outputs = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "required_plugins" => {
                            if seen_required_plugins {
                                return Err(serde::de::Error::duplicate_field("required_plugins"));
                            }
                            seen_required_plugins = true;
                            f_required_plugins = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "timeout" => {
                            if seen_timeout {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            seen_timeout = true;
                            f_timeout =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(WorkflowStep {
                    inputs: f_inputs,
                    outputs: f_outputs,
                    required_plugins: f_required_plugins,
                    timeout: f_timeout,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for WorkflowStep {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.inputs {
            map.serialize_entry("inputs", v)?;
        }
        if let Some(v) = &self.outputs {
            map.serialize_entry("outputs", v)?;
        }
        if let Some(v) = &self.required_plugins {
            map.serialize_entry("required_plugins", v)?;
        }
        if let Some(v) = &self.timeout {
            map.serialize_entry("timeout", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type WorkflowSteps =
    std::collections::BTreeMap<String, crate::types::flow_framework_common::WorkflowStep>;
