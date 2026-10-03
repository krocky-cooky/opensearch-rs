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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AllocationDecision {
    pub decider: String,
    pub decision: crate::types::cluster_allocation_explain::AllocationExplainDecision,
    pub explanation: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AllocationExplainDecision {
    #[serde(rename = "ALWAYS")]
    Always,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "THROTTLE")]
    Throttle,
    #[serde(rename = "YES")]
    Yes,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[doc = "The response of the `cluster.allocation_explain` API"]
pub type AllocationExplainResponse = crate::types::cluster::AllocationExplainResponse;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AllocationStore {
    pub allocation_id: String,
    pub found: bool,
    pub in_sync: bool,
    pub matching_size_in_bytes: crate::types::common::ByteCount,
    pub matching_sync_id: bool,
    pub store_exception: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterInfo {
    pub nodes:
        std::collections::BTreeMap<String, crate::types::cluster_allocation_explain::NodeDiskUsage>,
    pub reserved_sizes: Vec<crate::types::cluster_allocation_explain::ReservedSize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shard_data_set_sizes: Option<std::collections::BTreeMap<String, String>>,
    pub shard_paths: std::collections::BTreeMap<String, String>,
    pub shard_sizes: std::collections::BTreeMap<
        String,
        crate::types::cluster_allocation_explain::ClusterInfoShardSizesValue,
    >,
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
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = ClusterInfoShardSizesValue;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterInfoShardSizesValue value")
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::I64Deserializer::new(v))
                    .map(ClusterInfoShardSizesValue::ByteCount)
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::U64Deserializer::new(v))
                    .map(ClusterInfoShardSizesValue::ByteCount)
            }
            fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::F64Deserializer::new(v))
                    .map(ClusterInfoShardSizesValue::ByteCount)
            }
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                serde::Deserialize::deserialize(serde::de::value::StrDeserializer::new(v))
                    .map(ClusterInfoShardSizesValue::HumanReadableByteCount)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CurrentNode {
    pub attributes: std::collections::BTreeMap<String, String>,
    pub id: crate::types::common::Id,
    pub name: crate::types::common::Name,
    pub transport_address: crate::types::common::TransportAddress,
    pub weight_ranking: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Decision {
    #[serde(rename = "allocation_delayed")]
    AllocationDelayed,
    #[serde(rename = "awaiting_info")]
    AwaitingInfo,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "no_attempt")]
    NoAttempt,
    #[serde(rename = "no_valid_shard_copy")]
    NoValidShardCopy,
    #[serde(rename = "throttled")]
    Throttled,
    #[serde(rename = "worse_balance")]
    WorseBalance,
    #[serde(rename = "yes")]
    Yes,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiskUsage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free: Option<crate::types::common::HumanReadableByteCount>,
    pub free_bytes: crate::types::common::ByteCount,
    pub free_disk_percent: crate::types::common::PercentageNumber,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::common::HumanReadableByteCount>,
    pub total_bytes: crate::types::common::ByteCount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used: Option<crate::types::common::HumanReadableByteCount>,
    pub used_bytes: crate::types::common::ByteCount,
    pub used_disk_percent: crate::types::common::PercentageNumber,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeAllocationExplanation {
    pub deciders: Vec<crate::types::cluster_allocation_explain::AllocationDecision>,
    pub node_attributes: std::collections::BTreeMap<String, String>,
    pub node_decision: crate::types::cluster_allocation_explain::Decision,
    pub node_id: crate::types::common::Id,
    pub node_name: crate::types::common::Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<crate::types::cluster_allocation_explain::AllocationStore>,
    pub transport_address: crate::types::common::TransportAddress,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight_ranking: Option<i32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeDiskUsage {
    pub least_available: crate::types::cluster_allocation_explain::DiskUsage,
    pub most_available: crate::types::cluster_allocation_explain::DiskUsage,
    pub node_name: crate::types::common::Name,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReservedSize {
    pub node_id: crate::types::common::Id,
    pub path: String,
    pub shards: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnassignedInformation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocation_status: Option<String>,
    pub at: crate::types::common::DateTime,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delayed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_allocation_attempts: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_allocation_status: Option<String>,
    pub reason: crate::types::cluster_allocation_explain::UnassignedInformationReason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UnassignedInformationReason {
    #[serde(rename = "ALLOCATION_FAILED")]
    AllocationFailed,
    #[serde(rename = "CLUSTER_RECOVERED")]
    ClusterRecovered,
    #[serde(rename = "DANGLING_INDEX_IMPORTED")]
    DanglingIndexImported,
    #[serde(rename = "EXISTING_INDEX_RESTORED")]
    ExistingIndexRestored,
    #[serde(rename = "FORCED_EMPTY_PRIMARY")]
    ForcedEmptyPrimary,
    #[serde(rename = "INDEX_CREATED")]
    IndexCreated,
    #[serde(rename = "INDEX_REOPENED")]
    IndexReopened,
    #[serde(rename = "MANUAL_ALLOCATION")]
    ManualAllocation,
    #[serde(rename = "NEW_INDEX_RESTORED")]
    NewIndexRestored,
    #[serde(rename = "NODE_LEFT")]
    NodeLeft,
    #[serde(rename = "PRIMARY_FAILED")]
    PrimaryFailed,
    #[serde(rename = "REALLOCATED_REPLICA")]
    ReallocatedReplica,
    #[serde(rename = "REINITIALIZED")]
    Reinitialized,
    #[serde(rename = "REPLICA_ADDED")]
    ReplicaAdded,
    #[serde(rename = "REROUTE_CANCELLED")]
    RerouteCancelled,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}
