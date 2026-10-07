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

//! Response types of the `cluster_reroute` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct RerouteDecision {
    pub decider: String,
    pub decision: String,
    pub explanation: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RerouteDecision {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RerouteDecision;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RerouteDecision object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_decider: Option<String> = None;
                let mut f_decision: Option<String> = None;
                let mut f_explanation: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "decider" => {
                            if f_decider.is_some() {
                                return Err(serde::de::Error::duplicate_field("decider"));
                            }
                            f_decider = Some(map.next_value::<String>()?);
                        }
                        "decision" => {
                            if f_decision.is_some() {
                                return Err(serde::de::Error::duplicate_field("decision"));
                            }
                            f_decision = Some(map.next_value::<String>()?);
                        }
                        "explanation" => {
                            if f_explanation.is_some() {
                                return Err(serde::de::Error::duplicate_field("explanation"));
                            }
                            f_explanation = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RerouteDecision {
                    decider: f_decider.ok_or_else(|| serde::de::Error::missing_field("decider"))?,
                    decision: f_decision
                        .ok_or_else(|| serde::de::Error::missing_field("decision"))?,
                    explanation: f_explanation
                        .ok_or_else(|| serde::de::Error::missing_field("explanation"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RerouteDecision {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("decider", &self.decider)?;
        map.serialize_entry("decision", &self.decision)?;
        map.serialize_entry("explanation", &self.explanation)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RerouteExplanation {
    pub command: String,
    pub decisions: Vec<crate::types::cluster_reroute::RerouteDecision>,
    pub parameters: crate::types::cluster_reroute::RerouteParameters,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RerouteExplanation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RerouteExplanation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RerouteExplanation object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_command: Option<String> = None;
                let mut f_decisions: Option<Vec<crate::types::cluster_reroute::RerouteDecision>> =
                    None;
                let mut f_parameters: Option<crate::types::cluster_reroute::RerouteParameters> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "command" => {
                            if f_command.is_some() {
                                return Err(serde::de::Error::duplicate_field("command"));
                            }
                            f_command = Some(map.next_value::<String>()?);
                        }
                        "decisions" => {
                            if f_decisions.is_some() {
                                return Err(serde::de::Error::duplicate_field("decisions"));
                            }
                            f_decisions = Some(map.next_value::<Vec<crate::types::cluster_reroute::RerouteDecision>>()?);
                        }
                        "parameters" => {
                            if f_parameters.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            f_parameters = Some(
                                map.next_value::<crate::types::cluster_reroute::RerouteParameters>(
                                )?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RerouteExplanation {
                    command: f_command.ok_or_else(|| serde::de::Error::missing_field("command"))?,
                    decisions: f_decisions
                        .ok_or_else(|| serde::de::Error::missing_field("decisions"))?,
                    parameters: f_parameters
                        .ok_or_else(|| serde::de::Error::missing_field("parameters"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RerouteExplanation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("command", &self.command)?;
        map.serialize_entry("decisions", &self.decisions)?;
        map.serialize_entry("parameters", &self.parameters)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RerouteParameters {
    pub allow_primary: bool,
    pub from_node: Option<crate::types::common::NodeName>,
    pub index: crate::types::common::IndexName,
    pub node: crate::types::common::NodeName,
    pub shard: i32,
    pub to_node: Option<crate::types::common::NodeName>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RerouteParameters {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RerouteParameters;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RerouteParameters object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_allow_primary: Option<bool> = None;
                let mut f_from_node: Option<crate::types::common::NodeName> = None;
                let mut seen_from_node = false;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut f_node: Option<crate::types::common::NodeName> = None;
                let mut f_shard: Option<i32> = None;
                let mut f_to_node: Option<crate::types::common::NodeName> = None;
                let mut seen_to_node = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "allow_primary" => {
                            if f_allow_primary.is_some() {
                                return Err(serde::de::Error::duplicate_field("allow_primary"));
                            }
                            f_allow_primary = Some(map.next_value::<bool>()?);
                        }
                        "from_node" => {
                            if seen_from_node {
                                return Err(serde::de::Error::duplicate_field("from_node"));
                            }
                            seen_from_node = true;
                            f_from_node =
                                map.next_value::<Option<crate::types::common::NodeName>>()?;
                        }
                        "index" => {
                            if f_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            f_index = Some(map.next_value::<crate::types::common::IndexName>()?);
                        }
                        "node" => {
                            if f_node.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            f_node = Some(map.next_value::<crate::types::common::NodeName>()?);
                        }
                        "shard" => {
                            if f_shard.is_some() {
                                return Err(serde::de::Error::duplicate_field("shard"));
                            }
                            f_shard = Some(map.next_value::<i32>()?);
                        }
                        "to_node" => {
                            if seen_to_node {
                                return Err(serde::de::Error::duplicate_field("to_node"));
                            }
                            seen_to_node = true;
                            f_to_node =
                                map.next_value::<Option<crate::types::common::NodeName>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RerouteParameters {
                    allow_primary: f_allow_primary
                        .ok_or_else(|| serde::de::Error::missing_field("allow_primary"))?,
                    from_node: f_from_node,
                    index: f_index.ok_or_else(|| serde::de::Error::missing_field("index"))?,
                    node: f_node.ok_or_else(|| serde::de::Error::missing_field("node"))?,
                    shard: f_shard.ok_or_else(|| serde::de::Error::missing_field("shard"))?,
                    to_node: f_to_node,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RerouteParameters {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("allow_primary", &self.allow_primary)?;
        if let Some(v) = &self.from_node {
            map.serialize_entry("from_node", v)?;
        }
        map.serialize_entry("index", &self.index)?;
        map.serialize_entry("node", &self.node)?;
        map.serialize_entry("shard", &self.shard)?;
        if let Some(v) = &self.to_node {
            map.serialize_entry("to_node", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `cluster.reroute` API"]
pub type RerouteResponse = crate::types::cluster::RerouteResponse;
