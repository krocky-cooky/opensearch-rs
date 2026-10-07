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

//! Response types of the `cluster_allocation_explain` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct AllocationDecision {
    pub decider: String,
    pub decision: crate::types::cluster_allocation_explain::AllocationExplainDecision,
    pub explanation: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AllocationDecision {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AllocationDecision;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AllocationDecision object")
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
                let mut f_decision: Option<
                    crate::types::cluster_allocation_explain::AllocationExplainDecision,
                > = None;
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
                            f_decision = Some(map.next_value::<crate::types::cluster_allocation_explain::AllocationExplainDecision>()?);
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
                Ok(AllocationDecision {
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
impl serde::Serialize for AllocationDecision {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AllocationExplainDecision {
    #[doc = "`ALWAYS`"]
    Always,
    #[doc = "`NO`"]
    No,
    #[doc = "`THROTTLE`"]
    Throttle,
    #[doc = "`YES`"]
    Yes,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl AllocationExplainDecision {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            AllocationExplainDecision::Always => "ALWAYS",
            AllocationExplainDecision::No => "NO",
            AllocationExplainDecision::Throttle => "THROTTLE",
            AllocationExplainDecision::Yes => "YES",
            AllocationExplainDecision::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for AllocationExplainDecision {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AllocationExplainDecision {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ALWAYS" => AllocationExplainDecision::Always,
            "NO" => AllocationExplainDecision::No,
            "THROTTLE" => AllocationExplainDecision::Throttle,
            "YES" => AllocationExplainDecision::Yes,
            _ => AllocationExplainDecision::Unknown(s),
        })
    }
}

#[doc = "The response of the `cluster.allocation_explain` API"]
pub type AllocationExplainResponse = crate::types::cluster::AllocationExplainResponse;

#[derive(Debug, Clone)]
pub struct AllocationStore {
    pub allocation_id: String,
    pub found: bool,
    pub in_sync: bool,
    pub matching_size_in_bytes: crate::types::common::ByteCount,
    pub matching_sync_id: bool,
    pub store_exception: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AllocationStore {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AllocationStore;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AllocationStore object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_allocation_id: Option<String> = None;
                let mut f_found: Option<bool> = None;
                let mut f_in_sync: Option<bool> = None;
                let mut f_matching_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_matching_sync_id: Option<bool> = None;
                let mut f_store_exception: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "allocation_id" => {
                            if f_allocation_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("allocation_id"));
                            }
                            f_allocation_id = Some(map.next_value::<String>()?);
                        }
                        "found" => {
                            if f_found.is_some() {
                                return Err(serde::de::Error::duplicate_field("found"));
                            }
                            f_found = Some(map.next_value::<bool>()?);
                        }
                        "in_sync" => {
                            if f_in_sync.is_some() {
                                return Err(serde::de::Error::duplicate_field("in_sync"));
                            }
                            f_in_sync = Some(map.next_value::<bool>()?);
                        }
                        "matching_size_in_bytes" => {
                            if f_matching_size_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "matching_size_in_bytes",
                                ));
                            }
                            f_matching_size_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "matching_sync_id" => {
                            if f_matching_sync_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("matching_sync_id"));
                            }
                            f_matching_sync_id = Some(map.next_value::<bool>()?);
                        }
                        "store_exception" => {
                            if f_store_exception.is_some() {
                                return Err(serde::de::Error::duplicate_field("store_exception"));
                            }
                            f_store_exception = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AllocationStore {
                    allocation_id: f_allocation_id
                        .ok_or_else(|| serde::de::Error::missing_field("allocation_id"))?,
                    found: f_found.ok_or_else(|| serde::de::Error::missing_field("found"))?,
                    in_sync: f_in_sync.ok_or_else(|| serde::de::Error::missing_field("in_sync"))?,
                    matching_size_in_bytes: f_matching_size_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("matching_size_in_bytes"))?,
                    matching_sync_id: f_matching_sync_id
                        .ok_or_else(|| serde::de::Error::missing_field("matching_sync_id"))?,
                    store_exception: f_store_exception
                        .ok_or_else(|| serde::de::Error::missing_field("store_exception"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AllocationStore {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("allocation_id", &self.allocation_id)?;
        map.serialize_entry("found", &self.found)?;
        map.serialize_entry("in_sync", &self.in_sync)?;
        map.serialize_entry("matching_size_in_bytes", &self.matching_size_in_bytes)?;
        map.serialize_entry("matching_sync_id", &self.matching_sync_id)?;
        map.serialize_entry("store_exception", &self.store_exception)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ClusterInfo {
    pub nodes:
        std::collections::BTreeMap<String, crate::types::cluster_allocation_explain::NodeDiskUsage>,
    pub reserved_sizes: Vec<crate::types::cluster_allocation_explain::ReservedSize>,
    pub shard_data_set_sizes: Option<std::collections::BTreeMap<String, String>>,
    pub shard_paths: std::collections::BTreeMap<String, String>,
    pub shard_sizes: std::collections::BTreeMap<
        String,
        crate::types::cluster_allocation_explain::ClusterInfoShardSizesValue,
    >,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterInfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterInfo object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_nodes: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::cluster_allocation_explain::NodeDiskUsage,
                    >,
                > = None;
                let mut f_reserved_sizes: Option<
                    Vec<crate::types::cluster_allocation_explain::ReservedSize>,
                > = None;
                let mut f_shard_data_set_sizes: Option<std::collections::BTreeMap<String, String>> =
                    None;
                let mut seen_shard_data_set_sizes = false;
                let mut f_shard_paths: Option<std::collections::BTreeMap<String, String>> = None;
                let mut f_shard_sizes: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::cluster_allocation_explain::ClusterInfoShardSizesValue,
                    >,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "nodes" => {
                            if f_nodes.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            f_nodes = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                crate::types::cluster_allocation_explain::NodeDiskUsage,
                            >>()?);
                        }
                        "reserved_sizes" => {
                            if f_reserved_sizes.is_some() {
                                return Err(serde::de::Error::duplicate_field("reserved_sizes"));
                            }
                            f_reserved_sizes = Some(map.next_value::<Vec<crate::types::cluster_allocation_explain::ReservedSize>>()?);
                        }
                        "shard_data_set_sizes" => {
                            if seen_shard_data_set_sizes {
                                return Err(serde::de::Error::duplicate_field(
                                    "shard_data_set_sizes",
                                ));
                            }
                            seen_shard_data_set_sizes = true;
                            f_shard_data_set_sizes = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "shard_paths" => {
                            if f_shard_paths.is_some() {
                                return Err(serde::de::Error::duplicate_field("shard_paths"));
                            }
                            f_shard_paths = Some(
                                map.next_value::<std::collections::BTreeMap<String, String>>()?,
                            );
                        }
                        "shard_sizes" => {
                            if f_shard_sizes.is_some() {
                                return Err(serde::de::Error::duplicate_field("shard_sizes"));
                            }
                            f_shard_sizes = Some(map.next_value::<std::collections::BTreeMap<String, crate::types::cluster_allocation_explain::ClusterInfoShardSizesValue>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterInfo {
                    nodes: f_nodes.ok_or_else(|| serde::de::Error::missing_field("nodes"))?,
                    reserved_sizes: f_reserved_sizes
                        .ok_or_else(|| serde::de::Error::missing_field("reserved_sizes"))?,
                    shard_data_set_sizes: f_shard_data_set_sizes,
                    shard_paths: f_shard_paths
                        .ok_or_else(|| serde::de::Error::missing_field("shard_paths"))?,
                    shard_sizes: f_shard_sizes
                        .ok_or_else(|| serde::de::Error::missing_field("shard_sizes"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("nodes", &self.nodes)?;
        map.serialize_entry("reserved_sizes", &self.reserved_sizes)?;
        if let Some(v) = &self.shard_data_set_sizes {
            map.serialize_entry("shard_data_set_sizes", v)?;
        }
        map.serialize_entry("shard_paths", &self.shard_paths)?;
        map.serialize_entry("shard_sizes", &self.shard_sizes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum ClusterInfoShardSizesValue {
    ByteCount(crate::types::common::ByteCount),
    HumanReadableByteCount(crate::types::common::HumanReadableByteCount),
}
impl<'de> serde::Deserialize<'de> for ClusterInfoShardSizesValue {
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
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(ClusterInfoShardSizesValue::ByteCount)
            }
            Some(b'"') => {
                serde_json::from_str(text).map(ClusterInfoShardSizesValue::HumanReadableByteCount)
            }
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a ClusterInfoShardSizesValue value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct CurrentNode {
    pub attributes: std::collections::BTreeMap<String, String>,
    pub id: crate::types::common::Id,
    pub name: crate::types::common::Name,
    pub transport_address: crate::types::common::TransportAddress,
    pub weight_ranking: i32,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CurrentNode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CurrentNode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CurrentNode object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_attributes: Option<std::collections::BTreeMap<String, String>> = None;
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut f_transport_address: Option<crate::types::common::TransportAddress> = None;
                let mut f_weight_ranking: Option<i32> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "attributes" => {
                            if f_attributes.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            f_attributes = Some(
                                map.next_value::<std::collections::BTreeMap<String, String>>()?,
                            );
                        }
                        "id" => {
                            if f_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            f_id = Some(map.next_value::<crate::types::common::Id>()?);
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        "transport_address" => {
                            if f_transport_address.is_some() {
                                return Err(serde::de::Error::duplicate_field("transport_address"));
                            }
                            f_transport_address =
                                Some(map.next_value::<crate::types::common::TransportAddress>()?);
                        }
                        "weight_ranking" => {
                            if f_weight_ranking.is_some() {
                                return Err(serde::de::Error::duplicate_field("weight_ranking"));
                            }
                            f_weight_ranking = Some(map.next_value::<i32>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CurrentNode {
                    attributes: f_attributes
                        .ok_or_else(|| serde::de::Error::missing_field("attributes"))?,
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("id"))?,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    transport_address: f_transport_address
                        .ok_or_else(|| serde::de::Error::missing_field("transport_address"))?,
                    weight_ranking: f_weight_ranking
                        .ok_or_else(|| serde::de::Error::missing_field("weight_ranking"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CurrentNode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("attributes", &self.attributes)?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("transport_address", &self.transport_address)?;
        map.serialize_entry("weight_ranking", &self.weight_ranking)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Decision {
    #[doc = "`allocation_delayed`"]
    AllocationDelayed,
    #[doc = "`awaiting_info`"]
    AwaitingInfo,
    #[doc = "`no`"]
    No,
    #[doc = "`no_attempt`"]
    NoAttempt,
    #[doc = "`no_valid_shard_copy`"]
    NoValidShardCopy,
    #[doc = "`throttled`"]
    Throttled,
    #[doc = "`worse_balance`"]
    WorseBalance,
    #[doc = "`yes`"]
    Yes,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl Decision {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            Decision::AllocationDelayed => "allocation_delayed",
            Decision::AwaitingInfo => "awaiting_info",
            Decision::No => "no",
            Decision::NoAttempt => "no_attempt",
            Decision::NoValidShardCopy => "no_valid_shard_copy",
            Decision::Throttled => "throttled",
            Decision::WorseBalance => "worse_balance",
            Decision::Yes => "yes",
            Decision::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for Decision {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for Decision {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "allocation_delayed" => Decision::AllocationDelayed,
            "awaiting_info" => Decision::AwaitingInfo,
            "no" => Decision::No,
            "no_attempt" => Decision::NoAttempt,
            "no_valid_shard_copy" => Decision::NoValidShardCopy,
            "throttled" => Decision::Throttled,
            "worse_balance" => Decision::WorseBalance,
            "yes" => Decision::Yes,
            _ => Decision::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct DiskUsage {
    pub free: Option<crate::types::common::HumanReadableByteCount>,
    pub free_bytes: crate::types::common::ByteCount,
    pub free_disk_percent: crate::types::common::PercentageNumber,
    pub path: String,
    pub total: Option<crate::types::common::HumanReadableByteCount>,
    pub total_bytes: crate::types::common::ByteCount,
    pub used: Option<crate::types::common::HumanReadableByteCount>,
    pub used_bytes: crate::types::common::ByteCount,
    pub used_disk_percent: crate::types::common::PercentageNumber,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DiskUsage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DiskUsage;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DiskUsage object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_free: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_free = false;
                let mut f_free_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_free_disk_percent: Option<crate::types::common::PercentageNumber> = None;
                let mut f_path: Option<String> = None;
                let mut f_total: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_total = false;
                let mut f_total_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_used: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_used = false;
                let mut f_used_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_used_disk_percent: Option<crate::types::common::PercentageNumber> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "free" => {
                            if seen_free {
                                return Err(serde::de::Error::duplicate_field("free"));
                            }
                            seen_free = true;
                            f_free = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "free_bytes" => {
                            if f_free_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("free_bytes"));
                            }
                            f_free_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "free_disk_percent" => {
                            if f_free_disk_percent.is_some() {
                                return Err(serde::de::Error::duplicate_field("free_disk_percent"));
                            }
                            f_free_disk_percent =
                                Some(map.next_value::<crate::types::common::PercentageNumber>()?);
                        }
                        "path" => {
                            if f_path.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            f_path = Some(map.next_value::<String>()?);
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "total_bytes" => {
                            if f_total_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("total_bytes"));
                            }
                            f_total_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "used" => {
                            if seen_used {
                                return Err(serde::de::Error::duplicate_field("used"));
                            }
                            seen_used = true;
                            f_used = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "used_bytes" => {
                            if f_used_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("used_bytes"));
                            }
                            f_used_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "used_disk_percent" => {
                            if f_used_disk_percent.is_some() {
                                return Err(serde::de::Error::duplicate_field("used_disk_percent"));
                            }
                            f_used_disk_percent =
                                Some(map.next_value::<crate::types::common::PercentageNumber>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DiskUsage {
                    free: f_free,
                    free_bytes: f_free_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("free_bytes"))?,
                    free_disk_percent: f_free_disk_percent
                        .ok_or_else(|| serde::de::Error::missing_field("free_disk_percent"))?,
                    path: f_path.ok_or_else(|| serde::de::Error::missing_field("path"))?,
                    total: f_total,
                    total_bytes: f_total_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("total_bytes"))?,
                    used: f_used,
                    used_bytes: f_used_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("used_bytes"))?,
                    used_disk_percent: f_used_disk_percent
                        .ok_or_else(|| serde::de::Error::missing_field("used_disk_percent"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DiskUsage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.free {
            map.serialize_entry("free", v)?;
        }
        map.serialize_entry("free_bytes", &self.free_bytes)?;
        map.serialize_entry("free_disk_percent", &self.free_disk_percent)?;
        map.serialize_entry("path", &self.path)?;
        if let Some(v) = &self.total {
            map.serialize_entry("total", v)?;
        }
        map.serialize_entry("total_bytes", &self.total_bytes)?;
        if let Some(v) = &self.used {
            map.serialize_entry("used", v)?;
        }
        map.serialize_entry("used_bytes", &self.used_bytes)?;
        map.serialize_entry("used_disk_percent", &self.used_disk_percent)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NodeAllocationExplanation {
    pub deciders: Vec<crate::types::cluster_allocation_explain::AllocationDecision>,
    pub node_attributes: std::collections::BTreeMap<String, String>,
    pub node_decision: crate::types::cluster_allocation_explain::Decision,
    pub node_id: crate::types::common::Id,
    pub node_name: crate::types::common::Name,
    pub store: Option<crate::types::cluster_allocation_explain::AllocationStore>,
    pub transport_address: crate::types::common::TransportAddress,
    pub weight_ranking: Option<i32>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeAllocationExplanation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeAllocationExplanation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeAllocationExplanation object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_deciders: Option<
                    Vec<crate::types::cluster_allocation_explain::AllocationDecision>,
                > = None;
                let mut f_node_attributes: Option<std::collections::BTreeMap<String, String>> =
                    None;
                let mut f_node_decision: Option<
                    crate::types::cluster_allocation_explain::Decision,
                > = None;
                let mut f_node_id: Option<crate::types::common::Id> = None;
                let mut f_node_name: Option<crate::types::common::Name> = None;
                let mut f_store: Option<crate::types::cluster_allocation_explain::AllocationStore> =
                    None;
                let mut seen_store = false;
                let mut f_transport_address: Option<crate::types::common::TransportAddress> = None;
                let mut f_weight_ranking: Option<i32> = None;
                let mut seen_weight_ranking = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "deciders" => {
                            if f_deciders.is_some() {
                                return Err(serde::de::Error::duplicate_field("deciders"));
                            }
                            f_deciders = Some(map.next_value::<Vec<
                                crate::types::cluster_allocation_explain::AllocationDecision,
                            >>()?);
                        }
                        "node_attributes" => {
                            if f_node_attributes.is_some() {
                                return Err(serde::de::Error::duplicate_field("node_attributes"));
                            }
                            f_node_attributes = Some(
                                map.next_value::<std::collections::BTreeMap<String, String>>()?,
                            );
                        }
                        "node_decision" => {
                            if f_node_decision.is_some() {
                                return Err(serde::de::Error::duplicate_field("node_decision"));
                            }
                            f_node_decision = Some(map.next_value::<crate::types::cluster_allocation_explain::Decision>()?);
                        }
                        "node_id" => {
                            if f_node_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("node_id"));
                            }
                            f_node_id = Some(map.next_value::<crate::types::common::Id>()?);
                        }
                        "node_name" => {
                            if f_node_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("node_name"));
                            }
                            f_node_name = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store =
                                map.next_value::<Option<
                                    crate::types::cluster_allocation_explain::AllocationStore,
                                >>()?;
                        }
                        "transport_address" => {
                            if f_transport_address.is_some() {
                                return Err(serde::de::Error::duplicate_field("transport_address"));
                            }
                            f_transport_address =
                                Some(map.next_value::<crate::types::common::TransportAddress>()?);
                        }
                        "weight_ranking" => {
                            if seen_weight_ranking {
                                return Err(serde::de::Error::duplicate_field("weight_ranking"));
                            }
                            seen_weight_ranking = true;
                            f_weight_ranking = map.next_value::<Option<i32>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeAllocationExplanation {
                    deciders: f_deciders
                        .ok_or_else(|| serde::de::Error::missing_field("deciders"))?,
                    node_attributes: f_node_attributes
                        .ok_or_else(|| serde::de::Error::missing_field("node_attributes"))?,
                    node_decision: f_node_decision
                        .ok_or_else(|| serde::de::Error::missing_field("node_decision"))?,
                    node_id: f_node_id.ok_or_else(|| serde::de::Error::missing_field("node_id"))?,
                    node_name: f_node_name
                        .ok_or_else(|| serde::de::Error::missing_field("node_name"))?,
                    store: f_store,
                    transport_address: f_transport_address
                        .ok_or_else(|| serde::de::Error::missing_field("transport_address"))?,
                    weight_ranking: f_weight_ranking,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeAllocationExplanation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("deciders", &self.deciders)?;
        map.serialize_entry("node_attributes", &self.node_attributes)?;
        map.serialize_entry("node_decision", &self.node_decision)?;
        map.serialize_entry("node_id", &self.node_id)?;
        map.serialize_entry("node_name", &self.node_name)?;
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        map.serialize_entry("transport_address", &self.transport_address)?;
        if let Some(v) = &self.weight_ranking {
            map.serialize_entry("weight_ranking", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NodeDiskUsage {
    pub least_available: crate::types::cluster_allocation_explain::DiskUsage,
    pub most_available: crate::types::cluster_allocation_explain::DiskUsage,
    pub node_name: crate::types::common::Name,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeDiskUsage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeDiskUsage;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeDiskUsage object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_least_available: Option<
                    crate::types::cluster_allocation_explain::DiskUsage,
                > = None;
                let mut f_most_available: Option<
                    crate::types::cluster_allocation_explain::DiskUsage,
                > = None;
                let mut f_node_name: Option<crate::types::common::Name> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "least_available" => {
                            if f_least_available.is_some() {
                                return Err(serde::de::Error::duplicate_field("least_available"));
                            }
                            f_least_available = Some(map.next_value::<crate::types::cluster_allocation_explain::DiskUsage>()?);
                        }
                        "most_available" => {
                            if f_most_available.is_some() {
                                return Err(serde::de::Error::duplicate_field("most_available"));
                            }
                            f_most_available = Some(map.next_value::<crate::types::cluster_allocation_explain::DiskUsage>()?);
                        }
                        "node_name" => {
                            if f_node_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("node_name"));
                            }
                            f_node_name = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeDiskUsage {
                    least_available: f_least_available
                        .ok_or_else(|| serde::de::Error::missing_field("least_available"))?,
                    most_available: f_most_available
                        .ok_or_else(|| serde::de::Error::missing_field("most_available"))?,
                    node_name: f_node_name
                        .ok_or_else(|| serde::de::Error::missing_field("node_name"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeDiskUsage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("least_available", &self.least_available)?;
        map.serialize_entry("most_available", &self.most_available)?;
        map.serialize_entry("node_name", &self.node_name)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ReservedSize {
    pub node_id: crate::types::common::Id,
    pub path: String,
    pub shards: Vec<String>,
    pub total: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ReservedSize {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ReservedSize;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ReservedSize object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_node_id: Option<crate::types::common::Id> = None;
                let mut f_path: Option<String> = None;
                let mut f_shards: Option<Vec<String>> = None;
                let mut f_total: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "node_id" => {
                            if f_node_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("node_id"));
                            }
                            f_node_id = Some(map.next_value::<crate::types::common::Id>()?);
                        }
                        "path" => {
                            if f_path.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            f_path = Some(map.next_value::<String>()?);
                        }
                        "shards" => {
                            if f_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("shards"));
                            }
                            f_shards = Some(map.next_value::<Vec<String>>()?);
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ReservedSize {
                    node_id: f_node_id.ok_or_else(|| serde::de::Error::missing_field("node_id"))?,
                    path: f_path.ok_or_else(|| serde::de::Error::missing_field("path"))?,
                    shards: f_shards.ok_or_else(|| serde::de::Error::missing_field("shards"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ReservedSize {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("node_id", &self.node_id)?;
        map.serialize_entry("path", &self.path)?;
        map.serialize_entry("shards", &self.shards)?;
        map.serialize_entry("total", &self.total)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct UnassignedInformation {
    pub allocation_status: Option<String>,
    pub at: crate::types::common::DateTime,
    pub delayed: Option<bool>,
    pub details: Option<String>,
    pub failed_allocation_attempts: Option<i32>,
    pub last_allocation_status: Option<String>,
    pub reason: crate::types::cluster_allocation_explain::UnassignedInformationReason,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UnassignedInformation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UnassignedInformation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UnassignedInformation object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_allocation_status: Option<String> = None;
                let mut seen_allocation_status = false;
                let mut f_at: Option<crate::types::common::DateTime> = None;
                let mut f_delayed: Option<bool> = None;
                let mut seen_delayed = false;
                let mut f_details: Option<String> = None;
                let mut seen_details = false;
                let mut f_failed_allocation_attempts: Option<i32> = None;
                let mut seen_failed_allocation_attempts = false;
                let mut f_last_allocation_status: Option<String> = None;
                let mut seen_last_allocation_status = false;
                let mut f_reason: Option<
                    crate::types::cluster_allocation_explain::UnassignedInformationReason,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "allocation_status" => {
                            if seen_allocation_status {
                                return Err(serde::de::Error::duplicate_field("allocation_status"));
                            }
                            seen_allocation_status = true;
                            f_allocation_status = map.next_value::<Option<String>>()?;
                        }
                        "at" => {
                            if f_at.is_some() {
                                return Err(serde::de::Error::duplicate_field("at"));
                            }
                            f_at = Some(map.next_value::<crate::types::common::DateTime>()?);
                        }
                        "delayed" => {
                            if seen_delayed {
                                return Err(serde::de::Error::duplicate_field("delayed"));
                            }
                            seen_delayed = true;
                            f_delayed = map.next_value::<Option<bool>>()?;
                        }
                        "details" => {
                            if seen_details {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            seen_details = true;
                            f_details = map.next_value::<Option<String>>()?;
                        }
                        "failed_allocation_attempts" => {
                            if seen_failed_allocation_attempts {
                                return Err(serde::de::Error::duplicate_field(
                                    "failed_allocation_attempts",
                                ));
                            }
                            seen_failed_allocation_attempts = true;
                            f_failed_allocation_attempts = map.next_value::<Option<i32>>()?;
                        }
                        "last_allocation_status" => {
                            if seen_last_allocation_status {
                                return Err(serde::de::Error::duplicate_field(
                                    "last_allocation_status",
                                ));
                            }
                            seen_last_allocation_status = true;
                            f_last_allocation_status = map.next_value::<Option<String>>()?;
                        }
                        "reason" => {
                            if f_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            f_reason = Some(map.next_value::<crate::types::cluster_allocation_explain::UnassignedInformationReason>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UnassignedInformation {
                    allocation_status: f_allocation_status,
                    at: f_at.ok_or_else(|| serde::de::Error::missing_field("at"))?,
                    delayed: f_delayed,
                    details: f_details,
                    failed_allocation_attempts: f_failed_allocation_attempts,
                    last_allocation_status: f_last_allocation_status,
                    reason: f_reason.ok_or_else(|| serde::de::Error::missing_field("reason"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UnassignedInformation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.allocation_status {
            map.serialize_entry("allocation_status", v)?;
        }
        map.serialize_entry("at", &self.at)?;
        if let Some(v) = &self.delayed {
            map.serialize_entry("delayed", v)?;
        }
        if let Some(v) = &self.details {
            map.serialize_entry("details", v)?;
        }
        if let Some(v) = &self.failed_allocation_attempts {
            map.serialize_entry("failed_allocation_attempts", v)?;
        }
        if let Some(v) = &self.last_allocation_status {
            map.serialize_entry("last_allocation_status", v)?;
        }
        map.serialize_entry("reason", &self.reason)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnassignedInformationReason {
    #[doc = "`ALLOCATION_FAILED`"]
    AllocationFailed,
    #[doc = "`CLUSTER_RECOVERED`"]
    ClusterRecovered,
    #[doc = "`DANGLING_INDEX_IMPORTED`"]
    DanglingIndexImported,
    #[doc = "`EXISTING_INDEX_RESTORED`"]
    ExistingIndexRestored,
    #[doc = "`FORCED_EMPTY_PRIMARY`"]
    ForcedEmptyPrimary,
    #[doc = "`INDEX_CREATED`"]
    IndexCreated,
    #[doc = "`INDEX_REOPENED`"]
    IndexReopened,
    #[doc = "`MANUAL_ALLOCATION`"]
    ManualAllocation,
    #[doc = "`NEW_INDEX_RESTORED`"]
    NewIndexRestored,
    #[doc = "`NODE_LEFT`"]
    NodeLeft,
    #[doc = "`PRIMARY_FAILED`"]
    PrimaryFailed,
    #[doc = "`REALLOCATED_REPLICA`"]
    ReallocatedReplica,
    #[doc = "`REINITIALIZED`"]
    Reinitialized,
    #[doc = "`REPLICA_ADDED`"]
    ReplicaAdded,
    #[doc = "`REROUTE_CANCELLED`"]
    RerouteCancelled,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl UnassignedInformationReason {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            UnassignedInformationReason::AllocationFailed => "ALLOCATION_FAILED",
            UnassignedInformationReason::ClusterRecovered => "CLUSTER_RECOVERED",
            UnassignedInformationReason::DanglingIndexImported => "DANGLING_INDEX_IMPORTED",
            UnassignedInformationReason::ExistingIndexRestored => "EXISTING_INDEX_RESTORED",
            UnassignedInformationReason::ForcedEmptyPrimary => "FORCED_EMPTY_PRIMARY",
            UnassignedInformationReason::IndexCreated => "INDEX_CREATED",
            UnassignedInformationReason::IndexReopened => "INDEX_REOPENED",
            UnassignedInformationReason::ManualAllocation => "MANUAL_ALLOCATION",
            UnassignedInformationReason::NewIndexRestored => "NEW_INDEX_RESTORED",
            UnassignedInformationReason::NodeLeft => "NODE_LEFT",
            UnassignedInformationReason::PrimaryFailed => "PRIMARY_FAILED",
            UnassignedInformationReason::ReallocatedReplica => "REALLOCATED_REPLICA",
            UnassignedInformationReason::Reinitialized => "REINITIALIZED",
            UnassignedInformationReason::ReplicaAdded => "REPLICA_ADDED",
            UnassignedInformationReason::RerouteCancelled => "REROUTE_CANCELLED",
            UnassignedInformationReason::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for UnassignedInformationReason {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UnassignedInformationReason {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ALLOCATION_FAILED" => UnassignedInformationReason::AllocationFailed,
            "CLUSTER_RECOVERED" => UnassignedInformationReason::ClusterRecovered,
            "DANGLING_INDEX_IMPORTED" => UnassignedInformationReason::DanglingIndexImported,
            "EXISTING_INDEX_RESTORED" => UnassignedInformationReason::ExistingIndexRestored,
            "FORCED_EMPTY_PRIMARY" => UnassignedInformationReason::ForcedEmptyPrimary,
            "INDEX_CREATED" => UnassignedInformationReason::IndexCreated,
            "INDEX_REOPENED" => UnassignedInformationReason::IndexReopened,
            "MANUAL_ALLOCATION" => UnassignedInformationReason::ManualAllocation,
            "NEW_INDEX_RESTORED" => UnassignedInformationReason::NewIndexRestored,
            "NODE_LEFT" => UnassignedInformationReason::NodeLeft,
            "PRIMARY_FAILED" => UnassignedInformationReason::PrimaryFailed,
            "REALLOCATED_REPLICA" => UnassignedInformationReason::ReallocatedReplica,
            "REINITIALIZED" => UnassignedInformationReason::Reinitialized,
            "REPLICA_ADDED" => UnassignedInformationReason::ReplicaAdded,
            "REROUTE_CANCELLED" => UnassignedInformationReason::RerouteCancelled,
            _ => UnassignedInformationReason::Unknown(s),
        })
    }
}
