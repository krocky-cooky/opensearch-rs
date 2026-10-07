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

//! Response types of the `cluster_pending_tasks` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct PendingTask {
    #[doc = "Indicates whether the pending tasks are currently executing or not."]
    pub executing: bool,
    #[doc = "The number that represents when the task has been inserted into the task queue."]
    pub insert_order: i32,
    #[doc = "The priority of the pending task. The valid priorities in descending priority order are: `IMMEDIATE` > `URGENT` > `HIGH` > `NORMAL` > `LOW` > `LANGUID`."]
    pub priority: String,
    #[doc = "A general description of the cluster task that may include a reason and origin."]
    pub source: String,
    pub time_in_execution: Option<crate::types::common::Duration>,
    pub time_in_execution_millis: Option<crate::types::common::DurationValueUnitMillis>,
    pub time_in_queue: Option<crate::types::common::Duration>,
    pub time_in_queue_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PendingTask {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PendingTask;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PendingTask object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_executing: Option<bool> = None;
                let mut f_insert_order: Option<i32> = None;
                let mut f_priority: Option<String> = None;
                let mut f_source: Option<String> = None;
                let mut f_time_in_execution: Option<crate::types::common::Duration> = None;
                let mut seen_time_in_execution = false;
                let mut f_time_in_execution_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut seen_time_in_execution_millis = false;
                let mut f_time_in_queue: Option<crate::types::common::Duration> = None;
                let mut seen_time_in_queue = false;
                let mut f_time_in_queue_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "executing" => {
                            if f_executing.is_some() {
                                return Err(serde::de::Error::duplicate_field("executing"));
                            }
                            f_executing = Some(map.next_value::<bool>()?);
                        }
                        "insert_order" => {
                            if f_insert_order.is_some() {
                                return Err(serde::de::Error::duplicate_field("insert_order"));
                            }
                            f_insert_order = Some(map.next_value::<i32>()?);
                        }
                        "priority" => {
                            if f_priority.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            f_priority = Some(map.next_value::<String>()?);
                        }
                        "source" => {
                            if f_source.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            f_source = Some(map.next_value::<String>()?);
                        }
                        "time_in_execution" => {
                            if seen_time_in_execution {
                                return Err(serde::de::Error::duplicate_field("time_in_execution"));
                            }
                            seen_time_in_execution = true;
                            f_time_in_execution =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "time_in_execution_millis" => {
                            if seen_time_in_execution_millis {
                                return Err(serde::de::Error::duplicate_field(
                                    "time_in_execution_millis",
                                ));
                            }
                            seen_time_in_execution_millis = true;
                            f_time_in_execution_millis = map.next_value::<Option<crate::types::common::DurationValueUnitMillis>>()?;
                        }
                        "time_in_queue" => {
                            if seen_time_in_queue {
                                return Err(serde::de::Error::duplicate_field("time_in_queue"));
                            }
                            seen_time_in_queue = true;
                            f_time_in_queue =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "time_in_queue_millis" => {
                            if f_time_in_queue_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "time_in_queue_millis",
                                ));
                            }
                            f_time_in_queue_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PendingTask {
                    executing: f_executing
                        .ok_or_else(|| serde::de::Error::missing_field("executing"))?,
                    insert_order: f_insert_order
                        .ok_or_else(|| serde::de::Error::missing_field("insert_order"))?,
                    priority: f_priority
                        .ok_or_else(|| serde::de::Error::missing_field("priority"))?,
                    source: f_source.ok_or_else(|| serde::de::Error::missing_field("source"))?,
                    time_in_execution: f_time_in_execution,
                    time_in_execution_millis: f_time_in_execution_millis,
                    time_in_queue: f_time_in_queue,
                    time_in_queue_millis: f_time_in_queue_millis
                        .ok_or_else(|| serde::de::Error::missing_field("time_in_queue_millis"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PendingTask {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("executing", &self.executing)?;
        map.serialize_entry("insert_order", &self.insert_order)?;
        map.serialize_entry("priority", &self.priority)?;
        map.serialize_entry("source", &self.source)?;
        if let Some(v) = &self.time_in_execution {
            map.serialize_entry("time_in_execution", v)?;
        }
        if let Some(v) = &self.time_in_execution_millis {
            map.serialize_entry("time_in_execution_millis", v)?;
        }
        if let Some(v) = &self.time_in_queue {
            map.serialize_entry("time_in_queue", v)?;
        }
        map.serialize_entry("time_in_queue_millis", &self.time_in_queue_millis)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `cluster.pending_tasks` API"]
pub type PendingTasksResponse = crate::types::cluster::PendingTasksResponse;
