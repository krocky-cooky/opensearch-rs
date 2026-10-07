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

//! Response types of the `tasks_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct PersistentTaskStatus {
    pub state: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PersistentTaskStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PersistentTaskStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PersistentTaskStatus object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_state: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "state" => {
                            if f_state.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            f_state = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PersistentTaskStatus {
                    state: f_state.ok_or_else(|| serde::de::Error::missing_field("state"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PersistentTaskStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("state", &self.state)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type RawTaskStatus = std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

#[derive(Debug, Clone)]
pub struct ReplicationTaskStatus {
    pub phase: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ReplicationTaskStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ReplicationTaskStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ReplicationTaskStatus object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_phase: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "phase" => {
                            if f_phase.is_some() {
                                return Err(serde::de::Error::duplicate_field("phase"));
                            }
                            f_phase = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ReplicationTaskStatus {
                    phase: f_phase.ok_or_else(|| serde::de::Error::missing_field("phase"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ReplicationTaskStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("phase", &self.phase)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Task status information can vary wildly from task to task."]
#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Status {
    raw: Box<serde_json::value::RawValue>,
}
impl Status {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `replication_task_status` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_replication_task_status(
        &self,
    ) -> std::result::Result<
        crate::types::tasks_common::ReplicationTaskStatus,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "replication_task_status")
    }
    #[doc = "Parses the value as the `bulk_by_scroll_task_status` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_bulk_by_scroll_task_status(
        &self,
    ) -> std::result::Result<
        crate::types::common::BulkByScrollTaskStatus,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "bulk_by_scroll_task_status")
    }
    #[doc = "Parses the value as the `persistent_task_status` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_persistent_task_status(
        &self,
    ) -> std::result::Result<
        crate::types::tasks_common::PersistentTaskStatus,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "persistent_task_status")
    }
    #[doc = "Parses the value as the `raw_task_status` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_raw_task_status(
        &self,
    ) -> std::result::Result<
        crate::types::tasks_common::RawTaskStatus,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "raw_task_status")
    }
}

#[derive(Debug, Clone)]
pub struct TaskExecutingNode {
    #[doc = "The attributes of the node."]
    pub attributes: Option<std::collections::BTreeMap<String, String>>,
    pub host: Option<crate::types::common::Host>,
    pub ip: Option<crate::types::common::Ip>,
    pub name: crate::types::common::Name,
    pub roles: Option<crate::types::common::NodeRoles>,
    pub transport_address: Option<crate::types::common::TransportAddress>,
    pub tasks: std::collections::BTreeMap<String, crate::types::tasks_common::TaskInfo>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TaskExecutingNode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TaskExecutingNode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TaskExecutingNode object")
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
                    std::collections::BTreeMap<String, crate::types::tasks_common::TaskInfo>,
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
                                crate::types::tasks_common::TaskInfo,
                            >>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TaskExecutingNode {
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
impl serde::Serialize for TaskExecutingNode {
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
pub struct TaskGroup {
    pub action: String,
    pub cancellable: bool,
    pub cancellation_time_millis: Option<crate::types::common::EpochTimeUnitMillis>,
    pub cancelled: Option<bool>,
    pub description: Option<String>,
    pub headers: std::collections::BTreeMap<String, String>,
    pub id: i64,
    pub node: crate::types::common::NodeId,
    pub parent_task_id: Option<crate::types::common::TaskId>,
    pub resource_stats: Option<crate::types::common::ResourceStats>,
    pub running_time: Option<crate::types::common::Duration>,
    pub running_time_in_nanos: crate::types::common::DurationValueUnitNanos,
    pub start_time_in_millis: crate::types::common::EpochTimeUnitMillis,
    pub status: Option<crate::types::tasks_common::Status>,
    pub r#type: String,
    pub children: Option<Vec<crate::types::tasks_common::TaskGroup>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TaskGroup {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TaskGroup;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TaskGroup object")
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
                let mut f_cancellation_time_millis: Option<
                    crate::types::common::EpochTimeUnitMillis,
                > = None;
                let mut seen_cancellation_time_millis = false;
                let mut f_cancelled: Option<bool> = None;
                let mut seen_cancelled = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_headers: Option<std::collections::BTreeMap<String, String>> = None;
                let mut f_id: Option<i64> = None;
                let mut f_node: Option<crate::types::common::NodeId> = None;
                let mut f_parent_task_id: Option<crate::types::common::TaskId> = None;
                let mut seen_parent_task_id = false;
                let mut f_resource_stats: Option<crate::types::common::ResourceStats> = None;
                let mut seen_resource_stats = false;
                let mut f_running_time: Option<crate::types::common::Duration> = None;
                let mut seen_running_time = false;
                let mut f_running_time_in_nanos: Option<
                    crate::types::common::DurationValueUnitNanos,
                > = None;
                let mut f_start_time_in_millis: Option<crate::types::common::EpochTimeUnitMillis> =
                    None;
                let mut f_status: Option<crate::types::tasks_common::Status> = None;
                let mut seen_status = false;
                let mut f_type: Option<String> = None;
                let mut f_children: Option<Vec<crate::types::tasks_common::TaskGroup>> = None;
                let mut seen_children = false;
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
                        "cancellation_time_millis" => {
                            if seen_cancellation_time_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "cancellation_time_millis",
                                ));
                            }
                            seen_cancellation_time_millis = true;
                            f_cancellation_time_millis = map
                                .next_value::<Option<crate::types::common::EpochTimeUnitMillis>>(
                                )?;
                        }
                        "cancelled" => {
                            if seen_cancelled {
                                return Err(serde::de::Error::duplicate_field("cancelled"));
                            }
                            seen_cancelled = true;
                            f_cancelled = map.next_value::<Option<bool>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "headers" => {
                            if f_headers.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            f_headers = Some(
                                map.next_value::<std::collections::BTreeMap<String, String>>()?,
                            );
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
                            f_node = Some(map.next_value::<crate::types::common::NodeId>()?);
                        }
                        "parent_task_id" => {
                            if seen_parent_task_id {
                                return Err(serde::de::Error::duplicate_field("parent_task_id"));
                            }
                            seen_parent_task_id = true;
                            f_parent_task_id =
                                map.next_value::<Option<crate::types::common::TaskId>>()?;
                        }
                        "resource_stats" => {
                            if seen_resource_stats {
                                return Err(serde::de::Error::duplicate_field("resource_stats"));
                            }
                            seen_resource_stats = true;
                            f_resource_stats =
                                map.next_value::<Option<crate::types::common::ResourceStats>>()?;
                        }
                        "running_time" => {
                            if seen_running_time {
                                return Err(serde::de::Error::duplicate_field("running_time"));
                            }
                            seen_running_time = true;
                            f_running_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
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
                            if seen_status {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            seen_status = true;
                            f_status =
                                map.next_value::<Option<crate::types::tasks_common::Status>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<String>()?);
                        }
                        "children" => {
                            if seen_children {
                                return Err(serde::de::Error::duplicate_field("children"));
                            }
                            seen_children = true;
                            f_children = map
                                .next_value::<Option<Vec<crate::types::tasks_common::TaskGroup>>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TaskGroup {
                    action: f_action.ok_or_else(|| serde::de::Error::missing_field("action"))?,
                    cancellable: f_cancellable
                        .ok_or_else(|| serde::de::Error::missing_field("cancellable"))?,
                    cancellation_time_millis: f_cancellation_time_millis,
                    cancelled: f_cancelled,
                    description: f_description,
                    headers: f_headers.ok_or_else(|| serde::de::Error::missing_field("headers"))?,
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("id"))?,
                    node: f_node.ok_or_else(|| serde::de::Error::missing_field("node"))?,
                    parent_task_id: f_parent_task_id,
                    resource_stats: f_resource_stats,
                    running_time: f_running_time,
                    running_time_in_nanos: f_running_time_in_nanos
                        .ok_or_else(|| serde::de::Error::missing_field("running_time_in_nanos"))?,
                    start_time_in_millis: f_start_time_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("start_time_in_millis"))?,
                    status: f_status,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    children: f_children,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TaskGroup {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("action", &self.action)?;
        map.serialize_entry("cancellable", &self.cancellable)?;
        if let Some(v) = &self.cancellation_time_millis {
            map.serialize_entry("cancellation_time_millis", v)?;
        }
        if let Some(v) = &self.cancelled {
            map.serialize_entry("cancelled", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        map.serialize_entry("headers", &self.headers)?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("node", &self.node)?;
        if let Some(v) = &self.parent_task_id {
            map.serialize_entry("parent_task_id", v)?;
        }
        if let Some(v) = &self.resource_stats {
            map.serialize_entry("resource_stats", v)?;
        }
        if let Some(v) = &self.running_time {
            map.serialize_entry("running_time", v)?;
        }
        map.serialize_entry("running_time_in_nanos", &self.running_time_in_nanos)?;
        map.serialize_entry("start_time_in_millis", &self.start_time_in_millis)?;
        if let Some(v) = &self.status {
            map.serialize_entry("status", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        if let Some(v) = &self.children {
            map.serialize_entry("children", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TaskInfo {
    pub action: String,
    pub cancellable: bool,
    pub cancellation_time_millis: Option<crate::types::common::EpochTimeUnitMillis>,
    pub cancelled: Option<bool>,
    pub description: Option<String>,
    pub headers: std::collections::BTreeMap<String, String>,
    pub id: i64,
    pub node: crate::types::common::NodeId,
    pub parent_task_id: Option<crate::types::common::TaskId>,
    pub resource_stats: Option<crate::types::common::ResourceStats>,
    pub running_time: Option<crate::types::common::Duration>,
    pub running_time_in_nanos: crate::types::common::DurationValueUnitNanos,
    pub start_time_in_millis: crate::types::common::EpochTimeUnitMillis,
    pub status: Option<crate::types::tasks_common::Status>,
    pub r#type: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TaskInfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TaskInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TaskInfo object")
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
                let mut f_cancellation_time_millis: Option<
                    crate::types::common::EpochTimeUnitMillis,
                > = None;
                let mut seen_cancellation_time_millis = false;
                let mut f_cancelled: Option<bool> = None;
                let mut seen_cancelled = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_headers: Option<std::collections::BTreeMap<String, String>> = None;
                let mut f_id: Option<i64> = None;
                let mut f_node: Option<crate::types::common::NodeId> = None;
                let mut f_parent_task_id: Option<crate::types::common::TaskId> = None;
                let mut seen_parent_task_id = false;
                let mut f_resource_stats: Option<crate::types::common::ResourceStats> = None;
                let mut seen_resource_stats = false;
                let mut f_running_time: Option<crate::types::common::Duration> = None;
                let mut seen_running_time = false;
                let mut f_running_time_in_nanos: Option<
                    crate::types::common::DurationValueUnitNanos,
                > = None;
                let mut f_start_time_in_millis: Option<crate::types::common::EpochTimeUnitMillis> =
                    None;
                let mut f_status: Option<crate::types::tasks_common::Status> = None;
                let mut seen_status = false;
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
                        "cancellation_time_millis" => {
                            if seen_cancellation_time_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "cancellation_time_millis",
                                ));
                            }
                            seen_cancellation_time_millis = true;
                            f_cancellation_time_millis = map
                                .next_value::<Option<crate::types::common::EpochTimeUnitMillis>>(
                                )?;
                        }
                        "cancelled" => {
                            if seen_cancelled {
                                return Err(serde::de::Error::duplicate_field("cancelled"));
                            }
                            seen_cancelled = true;
                            f_cancelled = map.next_value::<Option<bool>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "headers" => {
                            if f_headers.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            f_headers = Some(
                                map.next_value::<std::collections::BTreeMap<String, String>>()?,
                            );
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
                            f_node = Some(map.next_value::<crate::types::common::NodeId>()?);
                        }
                        "parent_task_id" => {
                            if seen_parent_task_id {
                                return Err(serde::de::Error::duplicate_field("parent_task_id"));
                            }
                            seen_parent_task_id = true;
                            f_parent_task_id =
                                map.next_value::<Option<crate::types::common::TaskId>>()?;
                        }
                        "resource_stats" => {
                            if seen_resource_stats {
                                return Err(serde::de::Error::duplicate_field("resource_stats"));
                            }
                            seen_resource_stats = true;
                            f_resource_stats =
                                map.next_value::<Option<crate::types::common::ResourceStats>>()?;
                        }
                        "running_time" => {
                            if seen_running_time {
                                return Err(serde::de::Error::duplicate_field("running_time"));
                            }
                            seen_running_time = true;
                            f_running_time =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
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
                            if seen_status {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            seen_status = true;
                            f_status =
                                map.next_value::<Option<crate::types::tasks_common::Status>>()?;
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
                Ok(TaskInfo {
                    action: f_action.ok_or_else(|| serde::de::Error::missing_field("action"))?,
                    cancellable: f_cancellable
                        .ok_or_else(|| serde::de::Error::missing_field("cancellable"))?,
                    cancellation_time_millis: f_cancellation_time_millis,
                    cancelled: f_cancelled,
                    description: f_description,
                    headers: f_headers.ok_or_else(|| serde::de::Error::missing_field("headers"))?,
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("id"))?,
                    node: f_node.ok_or_else(|| serde::de::Error::missing_field("node"))?,
                    parent_task_id: f_parent_task_id,
                    resource_stats: f_resource_stats,
                    running_time: f_running_time,
                    running_time_in_nanos: f_running_time_in_nanos
                        .ok_or_else(|| serde::de::Error::missing_field("running_time_in_nanos"))?,
                    start_time_in_millis: f_start_time_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("start_time_in_millis"))?,
                    status: f_status,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TaskInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("action", &self.action)?;
        map.serialize_entry("cancellable", &self.cancellable)?;
        if let Some(v) = &self.cancellation_time_millis {
            map.serialize_entry("cancellation_time_millis", v)?;
        }
        if let Some(v) = &self.cancelled {
            map.serialize_entry("cancelled", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        map.serialize_entry("headers", &self.headers)?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("node", &self.node)?;
        if let Some(v) = &self.parent_task_id {
            map.serialize_entry("parent_task_id", v)?;
        }
        if let Some(v) = &self.resource_stats {
            map.serialize_entry("resource_stats", v)?;
        }
        if let Some(v) = &self.running_time {
            map.serialize_entry("running_time", v)?;
        }
        map.serialize_entry("running_time_in_nanos", &self.running_time_in_nanos)?;
        map.serialize_entry("start_time_in_millis", &self.start_time_in_millis)?;
        if let Some(v) = &self.status {
            map.serialize_entry("status", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum TaskInfos {
    GroupedByNone(Vec<crate::types::tasks_common::TaskInfo>),
    GroupedByParents(std::collections::BTreeMap<String, crate::types::tasks_common::TaskGroup>),
}
impl<'de> serde::Deserialize<'de> for TaskInfos {
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
            Some(b'[') => serde_json::from_str(text).map(TaskInfos::GroupedByNone),
            Some(b'{') => serde_json::from_str(text).map(TaskInfos::GroupedByParents),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a TaskInfos value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct TaskListResponseBase {
    pub node_failures: Option<Vec<crate::types::common::ErrorCause>>,
    #[doc = "Task information grouped by node, if `group_by` was set to `node` (the default)."]
    pub nodes:
        Option<std::collections::BTreeMap<String, crate::types::tasks_common::TaskExecutingNode>>,
    pub task_failures: Option<Vec<crate::types::common::TaskFailure>>,
    pub tasks: Option<crate::types::tasks_common::TaskInfos>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TaskListResponseBase {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TaskListResponseBase;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TaskListResponseBase object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_node_failures: Option<Vec<crate::types::common::ErrorCause>> = None;
                let mut seen_node_failures = false;
                let mut f_nodes: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::tasks_common::TaskExecutingNode,
                    >,
                > = None;
                let mut seen_nodes = false;
                let mut f_task_failures: Option<Vec<crate::types::common::TaskFailure>> = None;
                let mut seen_task_failures = false;
                let mut f_tasks: Option<crate::types::tasks_common::TaskInfos> = None;
                let mut seen_tasks = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "node_failures" => {
                            if seen_node_failures {
                                return Err(serde::de::Error::duplicate_field("node_failures"));
                            }
                            seen_node_failures = true;
                            f_node_failures =
                                map.next_value::<Option<Vec<crate::types::common::ErrorCause>>>()?;
                        }
                        "nodes" => {
                            if seen_nodes {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            seen_nodes = true;
                            f_nodes = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::tasks_common::TaskExecutingNode,
                                >,
                            >>()?;
                        }
                        "task_failures" => {
                            if seen_task_failures {
                                return Err(serde::de::Error::duplicate_field("task_failures"));
                            }
                            seen_task_failures = true;
                            f_task_failures =
                                map.next_value::<Option<Vec<crate::types::common::TaskFailure>>>()?;
                        }
                        "tasks" => {
                            if seen_tasks {
                                return Err(serde::de::Error::duplicate_field("tasks"));
                            }
                            seen_tasks = true;
                            f_tasks =
                                map.next_value::<Option<crate::types::tasks_common::TaskInfos>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TaskListResponseBase {
                    node_failures: f_node_failures,
                    nodes: f_nodes,
                    task_failures: f_task_failures,
                    tasks: f_tasks,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TaskListResponseBase {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.node_failures {
            map.serialize_entry("node_failures", v)?;
        }
        if let Some(v) = &self.nodes {
            map.serialize_entry("nodes", v)?;
        }
        if let Some(v) = &self.task_failures {
            map.serialize_entry("task_failures", v)?;
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

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum TaskResponse {
    BulkByScrollResponseBase(crate::types::common::BulkByScrollResponseBase),
}
impl<'de> serde::Deserialize<'de> for TaskResponse {
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
            Some(b'{') => serde_json::from_str(text).map(TaskResponse::BulkByScrollResponseBase),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a TaskResponse value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}
