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

//! Response types of the `wlm_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct QueryGroupResponse {
    #[doc = "The ID of the query group, which can be used to associate query requests with the group and enforce the group's resource limits."]
    pub id: String,
    #[doc = "The name of the query group."]
    pub name: String,
    #[doc = "The resiliency mode of the query group."]
    pub resiliency_mode: crate::types::wlm_common::ResiliencyMode,
    #[doc = "The resource limits for query requests in the query group."]
    pub resource_limits: crate::types::wlm_common::QueryGroupResponseResourceLimits,
    #[doc = "The time at which the query group was last updated."]
    pub updated_at: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for QueryGroupResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = QueryGroupResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a QueryGroupResponse object")
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
                let mut f_name: Option<String> = None;
                let mut f_resiliency_mode: Option<crate::types::wlm_common::ResiliencyMode> = None;
                let mut f_resource_limits: Option<
                    crate::types::wlm_common::QueryGroupResponseResourceLimits,
                > = None;
                let mut f_updated_at: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_id" => {
                            if f_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("_id"));
                            }
                            f_id = Some(map.next_value::<String>()?);
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<String>()?);
                        }
                        "resiliency_mode" => {
                            if f_resiliency_mode.is_some() {
                                return Err(serde::de::Error::duplicate_field("resiliency_mode"));
                            }
                            f_resiliency_mode =
                                Some(map.next_value::<crate::types::wlm_common::ResiliencyMode>()?);
                        }
                        "resource_limits" => {
                            if f_resource_limits.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_limits"));
                            }
                            f_resource_limits = Some(map.next_value::<crate::types::wlm_common::QueryGroupResponseResourceLimits>()?);
                        }
                        "updated_at" => {
                            if f_updated_at.is_some() {
                                return Err(serde::de::Error::duplicate_field("updated_at"));
                            }
                            f_updated_at = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(QueryGroupResponse {
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("_id"))?,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    resiliency_mode: f_resiliency_mode
                        .ok_or_else(|| serde::de::Error::missing_field("resiliency_mode"))?,
                    resource_limits: f_resource_limits
                        .ok_or_else(|| serde::de::Error::missing_field("resource_limits"))?,
                    updated_at: f_updated_at
                        .ok_or_else(|| serde::de::Error::missing_field("updated_at"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for QueryGroupResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("_id", &self.id)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("resiliency_mode", &self.resiliency_mode)?;
        map.serialize_entry("resource_limits", &self.resource_limits)?;
        map.serialize_entry("updated_at", &self.updated_at)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The resource limits for query requests in the query group."]
#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct QueryGroupResponseResourceLimits {
    raw: Box<serde_json::value::RawValue>,
}
impl QueryGroupResponseResourceLimits {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `object` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_object(
        &self,
    ) -> std::result::Result<
        crate::types::wlm_common::QueryGroupResponseResourceLimitsObject,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "object")
    }
    #[doc = "Parses the value as the `object_object` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_object_object(
        &self,
    ) -> std::result::Result<
        crate::types::wlm_common::QueryGroupResponseResourceLimitsObjectObject,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "object_object")
    }
}

#[derive(Debug, Clone)]
pub struct QueryGroupResponseResourceLimitsObject {
    pub cpu: Option<f64>,
    pub memory: f64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for QueryGroupResponseResourceLimitsObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = QueryGroupResponseResourceLimitsObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a QueryGroupResponseResourceLimitsObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cpu: Option<f64> = None;
                let mut seen_cpu = false;
                let mut f_memory: Option<f64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cpu" => {
                            if seen_cpu {
                                return Err(serde::de::Error::duplicate_field("cpu"));
                            }
                            seen_cpu = true;
                            f_cpu = map.next_value::<Option<f64>>()?;
                        }
                        "memory" => {
                            if f_memory.is_some() {
                                return Err(serde::de::Error::duplicate_field("memory"));
                            }
                            f_memory = Some(map.next_value::<f64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(QueryGroupResponseResourceLimitsObject {
                    cpu: f_cpu,
                    memory: f_memory.ok_or_else(|| serde::de::Error::missing_field("memory"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for QueryGroupResponseResourceLimitsObject {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cpu {
            map.serialize_entry("cpu", v)?;
        }
        map.serialize_entry("memory", &self.memory)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct QueryGroupResponseResourceLimitsObjectObject {
    pub cpu: f64,
    pub memory: Option<f64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for QueryGroupResponseResourceLimitsObjectObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = QueryGroupResponseResourceLimitsObjectObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a QueryGroupResponseResourceLimitsObjectObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cpu: Option<f64> = None;
                let mut f_memory: Option<f64> = None;
                let mut seen_memory = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cpu" => {
                            if f_cpu.is_some() {
                                return Err(serde::de::Error::duplicate_field("cpu"));
                            }
                            f_cpu = Some(map.next_value::<f64>()?);
                        }
                        "memory" => {
                            if seen_memory {
                                return Err(serde::de::Error::duplicate_field("memory"));
                            }
                            seen_memory = true;
                            f_memory = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(QueryGroupResponseResourceLimitsObjectObject {
                    cpu: f_cpu.ok_or_else(|| serde::de::Error::missing_field("cpu"))?,
                    memory: f_memory,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for QueryGroupResponseResourceLimitsObjectObject {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("cpu", &self.cpu)?;
        if let Some(v) = &self.memory {
            map.serialize_entry("memory", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResiliencyMode {
    #[doc = "`enforced`"]
    Enforced,
    #[doc = "`monitor`"]
    Monitor,
    #[doc = "`soft`"]
    Soft,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ResiliencyMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ResiliencyMode::Enforced => "enforced",
            ResiliencyMode::Monitor => "monitor",
            ResiliencyMode::Soft => "soft",
            ResiliencyMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ResiliencyMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ResiliencyMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "enforced" => ResiliencyMode::Enforced,
            "monitor" => ResiliencyMode::Monitor,
            "soft" => ResiliencyMode::Soft,
            _ => ResiliencyMode::Unknown(s),
        })
    }
}
