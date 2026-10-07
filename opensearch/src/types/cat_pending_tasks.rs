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

//! Response types of the `cat_pending_tasks` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct PendingTasksRecord {
    #[doc = "The task insertion order."]
    pub insert_order: Option<String>,
    #[doc = "The task priority."]
    pub priority: Option<String>,
    #[doc = "The task source."]
    pub source: Option<String>,
    #[doc = "Indicates how long the task has been in queue."]
    pub time_in_queue: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PendingTasksRecord {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PendingTasksRecord;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PendingTasksRecord object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_insert_order: Option<String> = None;
                let mut seen_insert_order = false;
                let mut f_priority: Option<String> = None;
                let mut seen_priority = false;
                let mut f_source: Option<String> = None;
                let mut seen_source = false;
                let mut f_time_in_queue: Option<String> = None;
                let mut seen_time_in_queue = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "insertOrder" => {
                            if seen_insert_order {
                                return Err(serde::de::Error::duplicate_field("insertOrder"));
                            }
                            seen_insert_order = true;
                            f_insert_order = map.next_value::<Option<String>>()?;
                        }
                        "priority" => {
                            if seen_priority {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            seen_priority = true;
                            f_priority = map.next_value::<Option<String>>()?;
                        }
                        "source" => {
                            if seen_source {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            seen_source = true;
                            f_source = map.next_value::<Option<String>>()?;
                        }
                        "timeInQueue" => {
                            if seen_time_in_queue {
                                return Err(serde::de::Error::duplicate_field("timeInQueue"));
                            }
                            seen_time_in_queue = true;
                            f_time_in_queue = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PendingTasksRecord {
                    insert_order: f_insert_order,
                    priority: f_priority,
                    source: f_source,
                    time_in_queue: f_time_in_queue,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PendingTasksRecord {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.insert_order {
            map.serialize_entry("insertOrder", v)?;
        }
        if let Some(v) = &self.priority {
            map.serialize_entry("priority", v)?;
        }
        if let Some(v) = &self.source {
            map.serialize_entry("source", v)?;
        }
        if let Some(v) = &self.time_in_queue {
            map.serialize_entry("timeInQueue", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `cat.pending_tasks` API"]
pub type PendingTasksResponse = crate::types::cat::PendingTasksResponse;
