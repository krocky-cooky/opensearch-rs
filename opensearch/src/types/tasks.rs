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

//! Response types of the `tasks` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct GetResponse {
    pub completed: bool,
    pub error: Option<crate::types::common::ErrorCause>,
    pub response: Option<crate::types::tasks_common::TaskResponse>,
    pub task: crate::types::tasks_common::TaskInfo,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GetResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GetResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GetResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_completed: Option<bool> = None;
                let mut f_error: Option<crate::types::common::ErrorCause> = None;
                let mut seen_error = false;
                let mut f_response: Option<crate::types::tasks_common::TaskResponse> = None;
                let mut seen_response = false;
                let mut f_task: Option<crate::types::tasks_common::TaskInfo> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "completed" => {
                            if f_completed.is_some() {
                                return Err(serde::de::Error::duplicate_field("completed"));
                            }
                            f_completed = Some(map.next_value::<bool>()?);
                        }
                        "error" => {
                            if seen_error {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            seen_error = true;
                            f_error =
                                map.next_value::<Option<crate::types::common::ErrorCause>>()?;
                        }
                        "response" => {
                            if seen_response {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            seen_response = true;
                            f_response = map
                                .next_value::<Option<crate::types::tasks_common::TaskResponse>>()?;
                        }
                        "task" => {
                            if f_task.is_some() {
                                return Err(serde::de::Error::duplicate_field("task"));
                            }
                            f_task =
                                Some(map.next_value::<crate::types::tasks_common::TaskInfo>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GetResponse {
                    completed: f_completed
                        .ok_or_else(|| serde::de::Error::missing_field("completed"))?,
                    error: f_error,
                    response: f_response,
                    task: f_task.ok_or_else(|| serde::de::Error::missing_field("task"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GetResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("completed", &self.completed)?;
        if let Some(v) = &self.error {
            map.serialize_entry("error", v)?;
        }
        if let Some(v) = &self.response {
            map.serialize_entry("response", v)?;
        }
        map.serialize_entry("task", &self.task)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
