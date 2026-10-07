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

//! Response types of the `nodes` specification namespace

#![allow(missing_docs)]

#[doc = "The response containing node information."]
#[derive(Debug, Clone)]
pub struct InfoResponse {
    #[doc = "Contains statistics about the number of nodes selected by the request's node filters."]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    pub cluster_name: crate::types::common::Name,
    pub _nodes: std::collections::BTreeMap<String, crate::types::nodes_info::NodeInfo>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for InfoResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = InfoResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a InfoResponse object")
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
                let mut f__nodes: Option<
                    std::collections::BTreeMap<String, crate::types::nodes_info::NodeInfo>,
                > = None;
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
                        "nodes" => {
                            if f__nodes.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            f__nodes = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                crate::types::nodes_info::NodeInfo,
                            >>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(InfoResponse {
                    nodes: f_nodes,
                    cluster_name: f_cluster_name
                        .ok_or_else(|| serde::de::Error::missing_field("cluster_name"))?,
                    _nodes: f__nodes.ok_or_else(|| serde::de::Error::missing_field("nodes"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for InfoResponse {
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
        map.serialize_entry("nodes", &self._nodes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response containing secure settings reload results."]
#[derive(Debug, Clone)]
pub struct ReloadSecureSettingsResponse {
    #[doc = "Contains statistics about the number of nodes selected by the request's node filters."]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    pub cluster_name: crate::types::common::Name,
    pub _nodes: std::collections::BTreeMap<
        String,
        crate::types::nodes_reload_secure_settings::NodeReloadResult,
    >,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ReloadSecureSettingsResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ReloadSecureSettingsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ReloadSecureSettingsResponse object")
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
                let mut f__nodes: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::nodes_reload_secure_settings::NodeReloadResult,
                    >,
                > = None;
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
                        "nodes" => {
                            if f__nodes.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            f__nodes = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                crate::types::nodes_reload_secure_settings::NodeReloadResult,
                            >>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ReloadSecureSettingsResponse {
                    nodes: f_nodes,
                    cluster_name: f_cluster_name
                        .ok_or_else(|| serde::de::Error::missing_field("cluster_name"))?,
                    _nodes: f__nodes.ok_or_else(|| serde::de::Error::missing_field("nodes"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ReloadSecureSettingsResponse {
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
        map.serialize_entry("nodes", &self._nodes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response containing node statistics."]
#[derive(Debug, Clone)]
pub struct StatsResponse {
    #[doc = "Contains statistics about the number of nodes selected by the request's node filters."]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    pub cluster_name: Option<crate::types::common::Name>,
    pub _nodes: std::collections::BTreeMap<String, crate::types::nodes_stats::Stats>,
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
                let mut seen_cluster_name = false;
                let mut f__nodes: Option<
                    std::collections::BTreeMap<String, crate::types::nodes_stats::Stats>,
                > = None;
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
                            if seen_cluster_name {
                                return Err(serde::de::Error::duplicate_field("cluster_name"));
                            }
                            seen_cluster_name = true;
                            f_cluster_name =
                                map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "nodes" => {
                            if f__nodes.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            f__nodes = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                crate::types::nodes_stats::Stats,
                            >>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StatsResponse {
                    nodes: f_nodes,
                    cluster_name: f_cluster_name,
                    _nodes: f__nodes.ok_or_else(|| serde::de::Error::missing_field("nodes"))?,
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
        if let Some(v) = &self.cluster_name {
            map.serialize_entry("cluster_name", v)?;
        }
        map.serialize_entry("nodes", &self._nodes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response containing node usage information."]
#[derive(Debug, Clone)]
pub struct UsageResponse {
    #[doc = "Contains statistics about the number of nodes selected by the request's node filters."]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    pub cluster_name: crate::types::common::Name,
    pub _nodes: std::collections::BTreeMap<String, crate::types::nodes_usage::NodeUsage>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UsageResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UsageResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UsageResponse object")
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
                let mut f__nodes: Option<
                    std::collections::BTreeMap<String, crate::types::nodes_usage::NodeUsage>,
                > = None;
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
                        "nodes" => {
                            if f__nodes.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            f__nodes = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                crate::types::nodes_usage::NodeUsage,
                            >>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UsageResponse {
                    nodes: f_nodes,
                    cluster_name: f_cluster_name
                        .ok_or_else(|| serde::de::Error::missing_field("cluster_name"))?,
                    _nodes: f__nodes.ok_or_else(|| serde::de::Error::missing_field("nodes"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UsageResponse {
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
        map.serialize_entry("nodes", &self._nodes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
