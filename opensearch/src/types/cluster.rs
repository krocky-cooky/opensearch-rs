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

//! Response types of the `cluster` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct AllocationExplainResponse {
    pub allocate_explanation: Option<String>,
    pub allocation_delay: Option<crate::types::common::Duration>,
    pub allocation_delay_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    pub can_allocate: Option<crate::types::cluster_allocation_explain::Decision>,
    pub can_move_to_other_node: Option<crate::types::cluster_allocation_explain::Decision>,
    pub can_rebalance_cluster: Option<crate::types::cluster_allocation_explain::Decision>,
    pub can_rebalance_cluster_decisions:
        Option<Vec<crate::types::cluster_allocation_explain::AllocationDecision>>,
    pub can_rebalance_to_other_node: Option<crate::types::cluster_allocation_explain::Decision>,
    pub can_remain_decisions:
        Option<Vec<crate::types::cluster_allocation_explain::AllocationDecision>>,
    pub can_remain_on_current_node: Option<crate::types::cluster_allocation_explain::Decision>,
    pub cluster_info: Option<crate::types::cluster_allocation_explain::ClusterInfo>,
    pub configured_delay: Option<crate::types::common::Duration>,
    pub configured_delay_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    pub current_node: Option<crate::types::cluster_allocation_explain::CurrentNode>,
    pub current_state: String,
    pub index: crate::types::common::IndexName,
    pub move_explanation: Option<String>,
    pub node_allocation_decisions:
        Option<Vec<crate::types::cluster_allocation_explain::NodeAllocationExplanation>>,
    pub note: Option<String>,
    pub primary: bool,
    pub rebalance_explanation: Option<String>,
    pub remaining_delay: Option<crate::types::common::Duration>,
    pub remaining_delay_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    pub shard: i32,
    pub unassigned_info: Option<crate::types::cluster_allocation_explain::UnassignedInformation>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AllocationExplainResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AllocationExplainResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AllocationExplainResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_allocate_explanation: Option<String> = None;
                let mut seen_allocate_explanation = false;
                let mut f_allocation_delay: Option<crate::types::common::Duration> = None;
                let mut seen_allocation_delay = false;
                let mut f_allocation_delay_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut seen_allocation_delay_in_millis = false;
                let mut f_can_allocate: Option<crate::types::cluster_allocation_explain::Decision> =
                    None;
                let mut seen_can_allocate = false;
                let mut f_can_move_to_other_node: Option<
                    crate::types::cluster_allocation_explain::Decision,
                > = None;
                let mut seen_can_move_to_other_node = false;
                let mut f_can_rebalance_cluster: Option<
                    crate::types::cluster_allocation_explain::Decision,
                > = None;
                let mut seen_can_rebalance_cluster = false;
                let mut f_can_rebalance_cluster_decisions: Option<
                    Vec<crate::types::cluster_allocation_explain::AllocationDecision>,
                > = None;
                let mut seen_can_rebalance_cluster_decisions = false;
                let mut f_can_rebalance_to_other_node: Option<
                    crate::types::cluster_allocation_explain::Decision,
                > = None;
                let mut seen_can_rebalance_to_other_node = false;
                let mut f_can_remain_decisions: Option<
                    Vec<crate::types::cluster_allocation_explain::AllocationDecision>,
                > = None;
                let mut seen_can_remain_decisions = false;
                let mut f_can_remain_on_current_node: Option<
                    crate::types::cluster_allocation_explain::Decision,
                > = None;
                let mut seen_can_remain_on_current_node = false;
                let mut f_cluster_info: Option<
                    crate::types::cluster_allocation_explain::ClusterInfo,
                > = None;
                let mut seen_cluster_info = false;
                let mut f_configured_delay: Option<crate::types::common::Duration> = None;
                let mut seen_configured_delay = false;
                let mut f_configured_delay_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut seen_configured_delay_in_millis = false;
                let mut f_current_node: Option<
                    crate::types::cluster_allocation_explain::CurrentNode,
                > = None;
                let mut seen_current_node = false;
                let mut f_current_state: Option<String> = None;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut f_move_explanation: Option<String> = None;
                let mut seen_move_explanation = false;
                let mut f_node_allocation_decisions: Option<
                    Vec<crate::types::cluster_allocation_explain::NodeAllocationExplanation>,
                > = None;
                let mut seen_node_allocation_decisions = false;
                let mut f_note: Option<String> = None;
                let mut seen_note = false;
                let mut f_primary: Option<bool> = None;
                let mut f_rebalance_explanation: Option<String> = None;
                let mut seen_rebalance_explanation = false;
                let mut f_remaining_delay: Option<crate::types::common::Duration> = None;
                let mut seen_remaining_delay = false;
                let mut f_remaining_delay_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut seen_remaining_delay_in_millis = false;
                let mut f_shard: Option<i32> = None;
                let mut f_unassigned_info: Option<
                    crate::types::cluster_allocation_explain::UnassignedInformation,
                > = None;
                let mut seen_unassigned_info = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "allocate_explanation" => {
                            if seen_allocate_explanation {
                                return Err(serde::de::Error::duplicate_field(
                                    "allocate_explanation",
                                ));
                            }
                            seen_allocate_explanation = true;
                            f_allocate_explanation = map.next_value::<Option<String>>()?;
                        }
                        "allocation_delay" => {
                            if seen_allocation_delay {
                                return Err(serde::de::Error::duplicate_field("allocation_delay"));
                            }
                            seen_allocation_delay = true;
                            f_allocation_delay =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "allocation_delay_in_millis" => {
                            if seen_allocation_delay_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "allocation_delay_in_millis",
                                ));
                            }
                            seen_allocation_delay_in_millis = true;
                            f_allocation_delay_in_millis = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        "can_allocate" => {
                            if seen_can_allocate {
                                return Err(serde::de::Error::duplicate_field("can_allocate"));
                            }
                            seen_can_allocate = true;
                            f_can_allocate = map.next_value::<Option<crate::types::cluster_allocation_explain::Decision>>()?;
                        }
                        "can_move_to_other_node" => {
                            if seen_can_move_to_other_node {
                                return Err(serde::de::Error::duplicate_field(
                                    "can_move_to_other_node",
                                ));
                            }
                            seen_can_move_to_other_node = true;
                            f_can_move_to_other_node = map.next_value::<Option<crate::types::cluster_allocation_explain::Decision>>()?;
                        }
                        "can_rebalance_cluster" => {
                            if seen_can_rebalance_cluster {
                                return Err(serde::de::Error::duplicate_field(
                                    "can_rebalance_cluster",
                                ));
                            }
                            seen_can_rebalance_cluster = true;
                            f_can_rebalance_cluster = map.next_value::<Option<crate::types::cluster_allocation_explain::Decision>>()?;
                        }
                        "can_rebalance_cluster_decisions" => {
                            if seen_can_rebalance_cluster_decisions {
                                return Err(serde::de::Error::duplicate_field(
                                    "can_rebalance_cluster_decisions",
                                ));
                            }
                            seen_can_rebalance_cluster_decisions = true;
                            f_can_rebalance_cluster_decisions = map.next_value::<Option<
                                Vec<crate::types::cluster_allocation_explain::AllocationDecision>,
                            >>(
                            )?;
                        }
                        "can_rebalance_to_other_node" => {
                            if seen_can_rebalance_to_other_node {
                                return Err(serde::de::Error::duplicate_field(
                                    "can_rebalance_to_other_node",
                                ));
                            }
                            seen_can_rebalance_to_other_node = true;
                            f_can_rebalance_to_other_node = map.next_value::<Option<crate::types::cluster_allocation_explain::Decision>>()?;
                        }
                        "can_remain_decisions" => {
                            if seen_can_remain_decisions {
                                return Err(serde::de::Error::duplicate_field(
                                    "can_remain_decisions",
                                ));
                            }
                            seen_can_remain_decisions = true;
                            f_can_remain_decisions = map.next_value::<Option<
                                Vec<crate::types::cluster_allocation_explain::AllocationDecision>,
                            >>()?;
                        }
                        "can_remain_on_current_node" => {
                            if seen_can_remain_on_current_node {
                                return Err(serde::de::Error::duplicate_field(
                                    "can_remain_on_current_node",
                                ));
                            }
                            seen_can_remain_on_current_node = true;
                            f_can_remain_on_current_node = map.next_value::<Option<crate::types::cluster_allocation_explain::Decision>>()?;
                        }
                        "cluster_info" => {
                            if seen_cluster_info {
                                return Err(serde::de::Error::duplicate_field("cluster_info"));
                            }
                            seen_cluster_info = true;
                            f_cluster_info = map.next_value::<Option<crate::types::cluster_allocation_explain::ClusterInfo>>()?;
                        }
                        "configured_delay" => {
                            if seen_configured_delay {
                                return Err(serde::de::Error::duplicate_field("configured_delay"));
                            }
                            seen_configured_delay = true;
                            f_configured_delay =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "configured_delay_in_millis" => {
                            if seen_configured_delay_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "configured_delay_in_millis",
                                ));
                            }
                            seen_configured_delay_in_millis = true;
                            f_configured_delay_in_millis = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        "current_node" => {
                            if seen_current_node {
                                return Err(serde::de::Error::duplicate_field("current_node"));
                            }
                            seen_current_node = true;
                            f_current_node = map.next_value::<Option<crate::types::cluster_allocation_explain::CurrentNode>>()?;
                        }
                        "current_state" => {
                            if f_current_state.is_some() {
                                return Err(serde::de::Error::duplicate_field("current_state"));
                            }
                            f_current_state = Some(map.next_value::<String>()?);
                        }
                        "index" => {
                            if f_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            f_index = Some(map.next_value::<crate::types::common::IndexName>()?);
                        }
                        "move_explanation" => {
                            if seen_move_explanation {
                                return Err(serde::de::Error::duplicate_field("move_explanation"));
                            }
                            seen_move_explanation = true;
                            f_move_explanation = map.next_value::<Option<String>>()?;
                        }
                        "node_allocation_decisions" => {
                            if seen_node_allocation_decisions {
                                return Err(serde::de::Error::duplicate_field(
                                    "node_allocation_decisions",
                                ));
                            }
                            seen_node_allocation_decisions = true;
                            f_node_allocation_decisions = map.next_value::<Option<Vec<crate::types::cluster_allocation_explain::NodeAllocationExplanation>>>()?;
                        }
                        "note" => {
                            if seen_note {
                                return Err(serde::de::Error::duplicate_field("note"));
                            }
                            seen_note = true;
                            f_note = map.next_value::<Option<String>>()?;
                        }
                        "primary" => {
                            if f_primary.is_some() {
                                return Err(serde::de::Error::duplicate_field("primary"));
                            }
                            f_primary = Some(map.next_value::<bool>()?);
                        }
                        "rebalance_explanation" => {
                            if seen_rebalance_explanation {
                                return Err(serde::de::Error::duplicate_field(
                                    "rebalance_explanation",
                                ));
                            }
                            seen_rebalance_explanation = true;
                            f_rebalance_explanation = map.next_value::<Option<String>>()?;
                        }
                        "remaining_delay" => {
                            if seen_remaining_delay {
                                return Err(serde::de::Error::duplicate_field("remaining_delay"));
                            }
                            seen_remaining_delay = true;
                            f_remaining_delay =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "remaining_delay_in_millis" => {
                            if seen_remaining_delay_in_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "remaining_delay_in_millis",
                                ));
                            }
                            seen_remaining_delay_in_millis = true;
                            f_remaining_delay_in_millis = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        "shard" => {
                            if f_shard.is_some() {
                                return Err(serde::de::Error::duplicate_field("shard"));
                            }
                            f_shard = Some(map.next_value::<i32>()?);
                        }
                        "unassigned_info" => {
                            if seen_unassigned_info {
                                return Err(serde::de::Error::duplicate_field("unassigned_info"));
                            }
                            seen_unassigned_info = true;
                            f_unassigned_info = map.next_value::<Option<
                                crate::types::cluster_allocation_explain::UnassignedInformation,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AllocationExplainResponse {
                    allocate_explanation: f_allocate_explanation,
                    allocation_delay: f_allocation_delay,
                    allocation_delay_in_millis: f_allocation_delay_in_millis,
                    can_allocate: f_can_allocate,
                    can_move_to_other_node: f_can_move_to_other_node,
                    can_rebalance_cluster: f_can_rebalance_cluster,
                    can_rebalance_cluster_decisions: f_can_rebalance_cluster_decisions,
                    can_rebalance_to_other_node: f_can_rebalance_to_other_node,
                    can_remain_decisions: f_can_remain_decisions,
                    can_remain_on_current_node: f_can_remain_on_current_node,
                    cluster_info: f_cluster_info,
                    configured_delay: f_configured_delay,
                    configured_delay_in_millis: f_configured_delay_in_millis,
                    current_node: f_current_node,
                    current_state: f_current_state
                        .ok_or_else(|| serde::de::Error::missing_field("current_state"))?,
                    index: f_index.ok_or_else(|| serde::de::Error::missing_field("index"))?,
                    move_explanation: f_move_explanation,
                    node_allocation_decisions: f_node_allocation_decisions,
                    note: f_note,
                    primary: f_primary.ok_or_else(|| serde::de::Error::missing_field("primary"))?,
                    rebalance_explanation: f_rebalance_explanation,
                    remaining_delay: f_remaining_delay,
                    remaining_delay_in_millis: f_remaining_delay_in_millis,
                    shard: f_shard.ok_or_else(|| serde::de::Error::missing_field("shard"))?,
                    unassigned_info: f_unassigned_info,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AllocationExplainResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.allocate_explanation {
            map.serialize_entry("allocate_explanation", v)?;
        }
        if let Some(v) = &self.allocation_delay {
            map.serialize_entry("allocation_delay", v)?;
        }
        if let Some(v) = &self.allocation_delay_in_millis {
            map.serialize_entry("allocation_delay_in_millis", v)?;
        }
        if let Some(v) = &self.can_allocate {
            map.serialize_entry("can_allocate", v)?;
        }
        if let Some(v) = &self.can_move_to_other_node {
            map.serialize_entry("can_move_to_other_node", v)?;
        }
        if let Some(v) = &self.can_rebalance_cluster {
            map.serialize_entry("can_rebalance_cluster", v)?;
        }
        if let Some(v) = &self.can_rebalance_cluster_decisions {
            map.serialize_entry("can_rebalance_cluster_decisions", v)?;
        }
        if let Some(v) = &self.can_rebalance_to_other_node {
            map.serialize_entry("can_rebalance_to_other_node", v)?;
        }
        if let Some(v) = &self.can_remain_decisions {
            map.serialize_entry("can_remain_decisions", v)?;
        }
        if let Some(v) = &self.can_remain_on_current_node {
            map.serialize_entry("can_remain_on_current_node", v)?;
        }
        if let Some(v) = &self.cluster_info {
            map.serialize_entry("cluster_info", v)?;
        }
        if let Some(v) = &self.configured_delay {
            map.serialize_entry("configured_delay", v)?;
        }
        if let Some(v) = &self.configured_delay_in_millis {
            map.serialize_entry("configured_delay_in_millis", v)?;
        }
        if let Some(v) = &self.current_node {
            map.serialize_entry("current_node", v)?;
        }
        map.serialize_entry("current_state", &self.current_state)?;
        map.serialize_entry("index", &self.index)?;
        if let Some(v) = &self.move_explanation {
            map.serialize_entry("move_explanation", v)?;
        }
        if let Some(v) = &self.node_allocation_decisions {
            map.serialize_entry("node_allocation_decisions", v)?;
        }
        if let Some(v) = &self.note {
            map.serialize_entry("note", v)?;
        }
        map.serialize_entry("primary", &self.primary)?;
        if let Some(v) = &self.rebalance_explanation {
            map.serialize_entry("rebalance_explanation", v)?;
        }
        if let Some(v) = &self.remaining_delay {
            map.serialize_entry("remaining_delay", v)?;
        }
        if let Some(v) = &self.remaining_delay_in_millis {
            map.serialize_entry("remaining_delay_in_millis", v)?;
        }
        map.serialize_entry("shard", &self.shard)?;
        if let Some(v) = &self.unassigned_info {
            map.serialize_entry("unassigned_info", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct GetComponentTemplateResponse {
    pub component_templates: Vec<crate::types::cluster_common::ComponentTemplate>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GetComponentTemplateResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GetComponentTemplateResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GetComponentTemplateResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_component_templates: Option<
                    Vec<crate::types::cluster_common::ComponentTemplate>,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "component_templates" => {
                            if f_component_templates.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "component_templates",
                                ));
                            }
                            f_component_templates = Some(map.next_value::<Vec<crate::types::cluster_common::ComponentTemplate>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GetComponentTemplateResponse {
                    component_templates: f_component_templates
                        .ok_or_else(|| serde::de::Error::missing_field("component_templates"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GetComponentTemplateResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("component_templates", &self.component_templates)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type GetDecommissionAwarenessResponse = std::collections::BTreeMap<
    String,
    crate::types::cluster_decommission_awareness::DecommissionStatus,
>;

#[derive(Debug, Clone)]
pub struct GetSettingsResponse {
    pub defaults: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub persistent: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
    pub transient: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GetSettingsResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GetSettingsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GetSettingsResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_defaults: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_defaults = false;
                let mut f_persistent: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut f_transient: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "defaults" => {
                            if seen_defaults {
                                return Err(serde::de::Error::duplicate_field("defaults"));
                            }
                            seen_defaults = true;
                            f_defaults = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "persistent" => {
                            if f_persistent.is_some() {
                                return Err(serde::de::Error::duplicate_field("persistent"));
                            }
                            f_persistent = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                Box<serde_json::value::RawValue>,
                            >>()?);
                        }
                        "transient" => {
                            if f_transient.is_some() {
                                return Err(serde::de::Error::duplicate_field("transient"));
                            }
                            f_transient = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                Box<serde_json::value::RawValue>,
                            >>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GetSettingsResponse {
                    defaults: f_defaults,
                    persistent: f_persistent
                        .ok_or_else(|| serde::de::Error::missing_field("persistent"))?,
                    transient: f_transient
                        .ok_or_else(|| serde::de::Error::missing_field("transient"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GetSettingsResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.defaults {
            map.serialize_entry("defaults", v)?;
        }
        map.serialize_entry("persistent", &self.persistent)?;
        map.serialize_entry("transient", &self.transient)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct GetWeightedRoutingResponse {
    pub version: Option<crate::types::common::VersionNumber>,
    pub weights: Option<std::collections::BTreeMap<String, String>>,
    pub discovered_cluster_manager: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GetWeightedRoutingResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GetWeightedRoutingResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GetWeightedRoutingResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version = false;
                let mut f_weights: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_weights = false;
                let mut f_discovered_cluster_manager: Option<bool> = None;
                let mut seen_discovered_cluster_manager = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionNumber>>()?;
                        }
                        "weights" => {
                            if seen_weights {
                                return Err(serde::de::Error::duplicate_field("weights"));
                            }
                            seen_weights = true;
                            f_weights = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "discovered_cluster_manager" => {
                            if seen_discovered_cluster_manager {
                                return Err(serde::de::Error::duplicate_field(
                                    "discovered_cluster_manager",
                                ));
                            }
                            seen_discovered_cluster_manager = true;
                            f_discovered_cluster_manager = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GetWeightedRoutingResponse {
                    version: f_version,
                    weights: f_weights,
                    discovered_cluster_manager: f_discovered_cluster_manager,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GetWeightedRoutingResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.version {
            map.serialize_entry("_version", v)?;
        }
        if let Some(v) = &self.weights {
            map.serialize_entry("weights", v)?;
        }
        if let Some(v) = &self.discovered_cluster_manager {
            map.serialize_entry("discovered_cluster_manager", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PendingTasksResponse {
    pub tasks: Vec<crate::types::cluster_pending_tasks::PendingTask>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PendingTasksResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PendingTasksResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PendingTasksResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_tasks: Option<Vec<crate::types::cluster_pending_tasks::PendingTask>> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "tasks" => {
                            if f_tasks.is_some() {
                                return Err(serde::de::Error::duplicate_field("tasks"));
                            }
                            f_tasks = Some(map.next_value::<Vec<crate::types::cluster_pending_tasks::PendingTask>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PendingTasksResponse {
                    tasks: f_tasks.ok_or_else(|| serde::de::Error::missing_field("tasks"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PendingTasksResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("tasks", &self.tasks)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PutSettingsResponse {
    pub acknowledged: bool,
    pub persistent: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
    pub transient: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PutSettingsResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PutSettingsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PutSettingsResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_acknowledged: Option<bool> = None;
                let mut f_persistent: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut f_transient: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "acknowledged" => {
                            if f_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field("acknowledged"));
                            }
                            f_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        "persistent" => {
                            if f_persistent.is_some() {
                                return Err(serde::de::Error::duplicate_field("persistent"));
                            }
                            f_persistent = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                Box<serde_json::value::RawValue>,
                            >>()?);
                        }
                        "transient" => {
                            if f_transient.is_some() {
                                return Err(serde::de::Error::duplicate_field("transient"));
                            }
                            f_transient = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                Box<serde_json::value::RawValue>,
                            >>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PutSettingsResponse {
                    acknowledged: f_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("acknowledged"))?,
                    persistent: f_persistent
                        .ok_or_else(|| serde::de::Error::missing_field("persistent"))?,
                    transient: f_transient
                        .ok_or_else(|| serde::de::Error::missing_field("transient"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PutSettingsResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("acknowledged", &self.acknowledged)?;
        map.serialize_entry("persistent", &self.persistent)?;
        map.serialize_entry("transient", &self.transient)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type RemoteInfoResponse =
    std::collections::BTreeMap<String, crate::types::cluster_remote_info::ClusterRemoteInfo>;

#[derive(Debug, Clone)]
pub struct RerouteResponse {
    #[doc = "For a successful response, this value is always true. On failure, an exception is returned instead."]
    pub acknowledged: bool,
    pub explanations: Option<Vec<crate::types::cluster_reroute::RerouteExplanation>>,
    #[doc = "Shows the internal representation of the structure, which can differ from the external representation.                  "]
    pub state: Option<Box<serde_json::value::RawValue>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RerouteResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RerouteResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RerouteResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_acknowledged: Option<bool> = None;
                let mut f_explanations: Option<
                    Vec<crate::types::cluster_reroute::RerouteExplanation>,
                > = None;
                let mut seen_explanations = false;
                let mut f_state: Option<Box<serde_json::value::RawValue>> = None;
                let mut seen_state = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "acknowledged" => {
                            if f_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field("acknowledged"));
                            }
                            f_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        "explanations" => {
                            if seen_explanations {
                                return Err(serde::de::Error::duplicate_field("explanations"));
                            }
                            seen_explanations = true;
                            f_explanations = map.next_value::<Option<Vec<crate::types::cluster_reroute::RerouteExplanation>>>()?;
                        }
                        "state" => {
                            if seen_state {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            seen_state = true;
                            f_state =
                                map.next_value::<Option<Box<serde_json::value::RawValue>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RerouteResponse {
                    acknowledged: f_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("acknowledged"))?,
                    explanations: f_explanations,
                    state: f_state,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RerouteResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("acknowledged", &self.acknowledged)?;
        if let Some(v) = &self.explanations {
            map.serialize_entry("explanations", v)?;
        }
        if let Some(v) = &self.state {
            map.serialize_entry("state", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type StateResponse = Box<serde_json::value::RawValue>;

#[derive(Debug, Clone)]
pub struct StatsResponse {
    #[doc = "Contains statistics about the number of nodes selected by the request's node filters."]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    #[doc = "The name of the cluster."]
    pub cluster_name: crate::types::common::Name,
    #[doc = "The unique identifier of the cluster."]
    pub cluster_uuid: crate::types::common::Uuid,
    #[doc = "Displays statistics about indexes in the cluster."]
    pub indices: Option<crate::types::cluster_stats::ClusterIndices>,
    #[doc = "Displays statistics about nodes selected by the request's node filters."]
    pub _nodes: Option<crate::types::cluster_stats::ClusterNodes>,
    #[doc = "The health status of the cluster, based on the state of its primary and replica shards."]
    pub status: crate::types::common::HealthStatus,
    #[doc = "The Unix timestamp, in milliseconds, indicating when any cluster statistics were last refreshed."]
    pub timestamp: crate::types::common::EpochTimeUnitMillis,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StatsResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StatsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StatsResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_nodes: Option<crate::types::common::NodeStatistics> = None;
                let mut seen_nodes = false;
                let mut f_cluster_name: Option<crate::types::common::Name> = None;
                let mut f_cluster_uuid: Option<crate::types::common::Uuid> = None;
                let mut f_indices: Option<crate::types::cluster_stats::ClusterIndices> = None;
                let mut seen_indices = false;
                let mut f__nodes: Option<crate::types::cluster_stats::ClusterNodes> = None;
                let mut seen__nodes = false;
                let mut f_status: Option<crate::types::common::HealthStatus> = None;
                let mut f_timestamp: Option<crate::types::common::EpochTimeUnitMillis> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_nodes" => {
                            if seen_nodes {
                                return Err(serde::de::Error::duplicate_field("_nodes"));
                            }
                            seen_nodes = true;
                            f_nodes =
                                map.next_value::<Option<crate::types::common::NodeStatistics>>()?;
                        }
                        "cluster_name" => {
                            if f_cluster_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster_name"));
                            }
                            f_cluster_name = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        "cluster_uuid" => {
                            if f_cluster_uuid.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster_uuid"));
                            }
                            f_cluster_uuid = Some(map.next_value::<crate::types::common::Uuid>()?);
                        }
                        "indices" => {
                            if seen_indices {
                                return Err(serde::de::Error::duplicate_field("indices"));
                            }
                            seen_indices = true;
                            f_indices = map
                                .next_value::<Option<crate::types::cluster_stats::ClusterIndices>>(
                                )?;
                        }
                        "nodes" => {
                            if seen__nodes {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            seen__nodes = true;
                            f__nodes = map
                                .next_value::<Option<crate::types::cluster_stats::ClusterNodes>>(
                                )?;
                        }
                        "status" => {
                            if f_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            f_status =
                                Some(map.next_value::<crate::types::common::HealthStatus>()?);
                        }
                        "timestamp" => {
                            if f_timestamp.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            f_timestamp = Some(
                                map.next_value::<crate::types::common::EpochTimeUnitMillis>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StatsResponse {
                    nodes: f_nodes,
                    cluster_name: f_cluster_name
                        .ok_or_else(|| serde::de::Error::missing_field("cluster_name"))?,
                    cluster_uuid: f_cluster_uuid
                        .ok_or_else(|| serde::de::Error::missing_field("cluster_uuid"))?,
                    indices: f_indices,
                    _nodes: f__nodes,
                    status: f_status.ok_or_else(|| serde::de::Error::missing_field("status"))?,
                    timestamp: f_timestamp
                        .ok_or_else(|| serde::de::Error::missing_field("timestamp"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StatsResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.nodes {
            map.serialize_entry("_nodes", v)?;
        }
        map.serialize_entry("cluster_name", &self.cluster_name)?;
        map.serialize_entry("cluster_uuid", &self.cluster_uuid)?;
        if let Some(v) = &self.indices {
            map.serialize_entry("indices", v)?;
        }
        if let Some(v) = &self._nodes {
            map.serialize_entry("nodes", v)?;
        }
        map.serialize_entry("status", &self.status)?;
        map.serialize_entry("timestamp", &self.timestamp)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
