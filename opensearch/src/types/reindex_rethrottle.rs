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

//! Response types of the `reindex_rethrottle` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct ReindexNode {
    #[doc = "The attributes of the node."]
    pub attributes: Option<std::collections::BTreeMap<String, String>>,
    pub host: Option<crate::types::common::Host>,
    pub ip: Option<crate::types::common::Ip>,
    pub name: crate::types::common::Name,
    pub roles: Option<crate::types::common::NodeRoles>,
    pub transport_address: Option<crate::types::common::TransportAddress>,
    pub tasks: std::collections::BTreeMap<String, crate::types::reindex_rethrottle::ReindexTask>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ReindexNode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ReindexNode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ReindexNode object")
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
                let mut seen_attributes = false;
                let mut f_host: Option<crate::types::common::Host> = None;
                let mut seen_host = false;
                let mut f_ip: Option<crate::types::common::Ip> = None;
                let mut seen_ip = false;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut f_roles: Option<crate::types::common::NodeRoles> = None;
                let mut seen_roles = false;
                let mut f_transport_address: Option<crate::types::common::TransportAddress> = None;
                let mut seen_transport_address = false;
                let mut f_tasks: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::reindex_rethrottle::ReindexTask,
                    >,
                > = None;
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
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "host" => {
                            if seen_host {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            seen_host = true;
                            f_host = map.next_value::<Option<crate::types::common::Host>>()?;
                        }
                        "ip" => {
                            if seen_ip {
                                return Err(serde::de::Error::duplicate_field("ip"));
                            }
                            seen_ip = true;
                            f_ip = map.next_value::<Option<crate::types::common::Ip>>()?;
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
                            f_roles =
                                map.next_value::<Option<crate::types::common::NodeRoles>>()?;
                        }
                        "transport_address" => {
                            if seen_transport_address {
                                return Err(serde::de::Error::duplicate_field("transport_address"));
                            }
                            seen_transport_address = true;
                            f_transport_address =
                                map.next_value::<Option<crate::types::common::TransportAddress>>()?;
                        }
                        "tasks" => {
                            if f_tasks.is_some() {
                                return Err(serde::de::Error::duplicate_field("tasks"));
                            }
                            f_tasks = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                crate::types::reindex_rethrottle::ReindexTask,
                            >>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ReindexNode {
                    attributes: f_attributes,
                    host: f_host,
                    ip: f_ip,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    roles: f_roles,
                    transport_address: f_transport_address,
                    tasks: f_tasks.ok_or_else(|| serde::de::Error::missing_field("tasks"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ReindexNode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.attributes {
            map.serialize_entry("attributes", v)?;
        }
        if let Some(v) = &self.host {
            map.serialize_entry("host", v)?;
        }
        if let Some(v) = &self.ip {
            map.serialize_entry("ip", v)?;
        }
        map.serialize_entry("name", &self.name)?;
        if let Some(v) = &self.roles {
            map.serialize_entry("roles", v)?;
        }
        if let Some(v) = &self.transport_address {
            map.serialize_entry("transport_address", v)?;
        }
        map.serialize_entry("tasks", &self.tasks)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ReindexRethrottleResponse {
    pub nodes: std::collections::BTreeMap<String, crate::types::reindex_rethrottle::ReindexNode>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ReindexRethrottleResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ReindexRethrottleResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ReindexRethrottleResponse object")
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
                        crate::types::reindex_rethrottle::ReindexNode,
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
                                crate::types::reindex_rethrottle::ReindexNode,
                            >>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ReindexRethrottleResponse {
                    nodes: f_nodes.ok_or_else(|| serde::de::Error::missing_field("nodes"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ReindexRethrottleResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("nodes", &self.nodes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ReindexStatus {
    #[doc = "The number of scroll responses shown by the reindex."]
    pub batches: i64,
    #[doc = "The number of documents that were successfully created."]
    pub created: i64,
    #[doc = "The number of documents that were successfully deleted."]
    pub deleted: i64,
    #[doc = "The number of documents that were ignored because the script used for the reindex operation returned a `noop` value for `ctx.op`."]
    pub noops: i64,
    #[doc = "The number of successful requests per second during the reindex operation."]
    pub requests_per_second: f64,
    pub retries: crate::types::common::Retries,
    pub throttled: Option<crate::types::common::Duration>,
    pub throttled_millis: crate::types::common::DurationValueUnitMillis,
    pub throttled_until: Option<crate::types::common::Duration>,
    pub throttled_until_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The number of documents that were successfully processed."]
    pub total: i64,
    #[doc = "The number of documents that were successfully updated."]
    pub updated: i64,
    #[doc = "The number of version conflicts that reindex hits."]
    pub version_conflicts: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ReindexStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ReindexStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ReindexStatus object")
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
                let mut f_created: Option<i64> = None;
                let mut f_deleted: Option<i64> = None;
                let mut f_noops: Option<i64> = None;
                let mut f_requests_per_second: Option<f64> = None;
                let mut f_retries: Option<crate::types::common::Retries> = None;
                let mut f_throttled: Option<crate::types::common::Duration> = None;
                let mut seen_throttled = false;
                let mut f_throttled_millis: Option<crate::types::common::DurationValueUnitMillis> =
                    None;
                let mut f_throttled_until: Option<crate::types::common::Duration> = None;
                let mut seen_throttled_until = false;
                let mut f_throttled_until_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_total: Option<i64> = None;
                let mut f_updated: Option<i64> = None;
                let mut f_version_conflicts: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "batches" => {
                            if f_batches.is_some() {
                                return Err(serde::de::Error::duplicate_field("batches"));
                            }
                            f_batches = Some(map.next_value::<i64>()?);
                        }
                        "created" => {
                            if f_created.is_some() {
                                return Err(serde::de::Error::duplicate_field("created"));
                            }
                            f_created = Some(map.next_value::<i64>()?);
                        }
                        "deleted" => {
                            if f_deleted.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleted"));
                            }
                            f_deleted = Some(map.next_value::<i64>()?);
                        }
                        "noops" => {
                            if f_noops.is_some() {
                                return Err(serde::de::Error::duplicate_field("noops"));
                            }
                            f_noops = Some(map.next_value::<i64>()?);
                        }
                        "requests_per_second" => {
                            if f_requests_per_second.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "requests_per_second",
                                ));
                            }
                            f_requests_per_second = Some(map.next_value::<f64>()?);
                        }
                        "retries" => {
                            if f_retries.is_some() {
                                return Err(serde::de::Error::duplicate_field("retries"));
                            }
                            f_retries = Some(map.next_value::<crate::types::common::Retries>()?);
                        }
                        "throttled" => {
                            if seen_throttled {
                                return Err(serde::de::Error::duplicate_field("throttled"));
                            }
                            seen_throttled = true;
                            f_throttled =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "throttled_millis" => {
                            if f_throttled_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field("throttled_millis"));
                            }
                            f_throttled_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "throttled_until" => {
                            if seen_throttled_until {
                                return Err(serde::de::Error::duplicate_field("throttled_until"));
                            }
                            seen_throttled_until = true;
                            f_throttled_until =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "throttled_until_millis" => {
                            if f_throttled_until_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "throttled_until_millis",
                                ));
                            }
                            f_throttled_until_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(map.next_value::<i64>()?);
                        }
                        "updated" => {
                            if f_updated.is_some() {
                                return Err(serde::de::Error::duplicate_field("updated"));
                            }
                            f_updated = Some(map.next_value::<i64>()?);
                        }
                        "version_conflicts" => {
                            if f_version_conflicts.is_some() {
                                return Err(serde::de::Error::duplicate_field("version_conflicts"));
                            }
                            f_version_conflicts = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ReindexStatus {
                    batches: f_batches.ok_or_else(|| serde::de::Error::missing_field("batches"))?,
                    created: f_created.ok_or_else(|| serde::de::Error::missing_field("created"))?,
                    deleted: f_deleted.ok_or_else(|| serde::de::Error::missing_field("deleted"))?,
                    noops: f_noops.ok_or_else(|| serde::de::Error::missing_field("noops"))?,
                    requests_per_second: f_requests_per_second
                        .ok_or_else(|| serde::de::Error::missing_field("requests_per_second"))?,
                    retries: f_retries.ok_or_else(|| serde::de::Error::missing_field("retries"))?,
                    throttled: f_throttled,
                    throttled_millis: f_throttled_millis
                        .ok_or_else(|| serde::de::Error::missing_field("throttled_millis"))?,
                    throttled_until: f_throttled_until,
                    throttled_until_millis: f_throttled_until_millis
                        .ok_or_else(|| serde::de::Error::missing_field("throttled_until_millis"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    updated: f_updated.ok_or_else(|| serde::de::Error::missing_field("updated"))?,
                    version_conflicts: f_version_conflicts
                        .ok_or_else(|| serde::de::Error::missing_field("version_conflicts"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ReindexStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("batches", &self.batches)?;
        map.serialize_entry("created", &self.created)?;
        map.serialize_entry("deleted", &self.deleted)?;
        map.serialize_entry("noops", &self.noops)?;
        map.serialize_entry("requests_per_second", &self.requests_per_second)?;
        map.serialize_entry("retries", &self.retries)?;
        if let Some(v) = &self.throttled {
            map.serialize_entry("throttled", v)?;
        }
        map.serialize_entry("throttled_millis", &self.throttled_millis)?;
        if let Some(v) = &self.throttled_until {
            map.serialize_entry("throttled_until", v)?;
        }
        map.serialize_entry("throttled_until_millis", &self.throttled_until_millis)?;
        map.serialize_entry("total", &self.total)?;
        map.serialize_entry("updated", &self.updated)?;
        map.serialize_entry("version_conflicts", &self.version_conflicts)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ReindexTask {
    pub action: String,
    pub cancellable: bool,
    pub cancelled: Option<bool>,
    pub description: String,
    pub headers: crate::types::common::HttpHeaders,
    pub id: i64,
    pub node: crate::types::common::Name,
    pub resource_stats: Option<crate::types::common::ResourceStats>,
    pub running_time_in_nanos: crate::types::common::DurationValueUnitNanos,
    pub start_time_in_millis: crate::types::common::EpochTimeUnitMillis,
    pub status: crate::types::reindex_rethrottle::ReindexStatus,
    pub r#type: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ReindexTask {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ReindexTask;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ReindexTask object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_action: Option<String> = None;
                let mut f_cancellable: Option<bool> = None;
                let mut f_cancelled: Option<bool> = None;
                let mut seen_cancelled = false;
                let mut f_description: Option<String> = None;
                let mut f_headers: Option<crate::types::common::HttpHeaders> = None;
                let mut f_id: Option<i64> = None;
                let mut f_node: Option<crate::types::common::Name> = None;
                let mut f_resource_stats: Option<crate::types::common::ResourceStats> = None;
                let mut seen_resource_stats = false;
                let mut f_running_time_in_nanos: Option<
                    crate::types::common::DurationValueUnitNanos,
                > = None;
                let mut f_start_time_in_millis: Option<crate::types::common::EpochTimeUnitMillis> =
                    None;
                let mut f_status: Option<crate::types::reindex_rethrottle::ReindexStatus> = None;
                let mut f_type: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "action" => {
                            if f_action.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            f_action = Some(map.next_value::<String>()?);
                        }
                        "cancellable" => {
                            if f_cancellable.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancellable"));
                            }
                            f_cancellable = Some(map.next_value::<bool>()?);
                        }
                        "cancelled" => {
                            if seen_cancelled {
                                return Err(serde::de::Error::duplicate_field("cancelled"));
                            }
                            seen_cancelled = true;
                            f_cancelled = map.next_value::<Option<bool>>()?;
                        }
                        "description" => {
                            if f_description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            f_description = Some(map.next_value::<String>()?);
                        }
                        "headers" => {
                            if f_headers.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            f_headers =
                                Some(map.next_value::<crate::types::common::HttpHeaders>()?);
                        }
                        "id" => {
                            if f_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            f_id = Some(map.next_value::<i64>()?);
                        }
                        "node" => {
                            if f_node.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            f_node = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        "resource_stats" => {
                            if seen_resource_stats {
                                return Err(serde::de::Error::duplicate_field("resource_stats"));
                            }
                            seen_resource_stats = true;
                            f_resource_stats =
                                map.next_value::<Option<crate::types::common::ResourceStats>>()?;
                        }
                        "running_time_in_nanos" => {
                            if f_running_time_in_nanos.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "running_time_in_nanos",
                                ));
                            }
                            f_running_time_in_nanos = Some(
                                map.next_value::<crate::types::common::DurationValueUnitNanos>()?,
                            );
                        }
                        "start_time_in_millis" => {
                            if f_start_time_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "start_time_in_millis",
                                ));
                            }
                            f_start_time_in_millis = Some(
                                map.next_value::<crate::types::common::EpochTimeUnitMillis>()?,
                            );
                        }
                        "status" => {
                            if f_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            f_status = Some(
                                map.next_value::<crate::types::reindex_rethrottle::ReindexStatus>(
                                )?,
                            );
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ReindexTask {
                    action: f_action.ok_or_else(|| serde::de::Error::missing_field("action"))?,
                    cancellable: f_cancellable
                        .ok_or_else(|| serde::de::Error::missing_field("cancellable"))?,
                    cancelled: f_cancelled,
                    description: f_description
                        .ok_or_else(|| serde::de::Error::missing_field("description"))?,
                    headers: f_headers.ok_or_else(|| serde::de::Error::missing_field("headers"))?,
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("id"))?,
                    node: f_node.ok_or_else(|| serde::de::Error::missing_field("node"))?,
                    resource_stats: f_resource_stats,
                    running_time_in_nanos: f_running_time_in_nanos
                        .ok_or_else(|| serde::de::Error::missing_field("running_time_in_nanos"))?,
                    start_time_in_millis: f_start_time_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("start_time_in_millis"))?,
                    status: f_status.ok_or_else(|| serde::de::Error::missing_field("status"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ReindexTask {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("action", &self.action)?;
        map.serialize_entry("cancellable", &self.cancellable)?;
        if let Some(v) = &self.cancelled {
            map.serialize_entry("cancelled", v)?;
        }
        map.serialize_entry("description", &self.description)?;
        map.serialize_entry("headers", &self.headers)?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("node", &self.node)?;
        if let Some(v) = &self.resource_stats {
            map.serialize_entry("resource_stats", v)?;
        }
        map.serialize_entry("running_time_in_nanos", &self.running_time_in_nanos)?;
        map.serialize_entry("start_time_in_millis", &self.start_time_in_millis)?;
        map.serialize_entry("status", &self.status)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
