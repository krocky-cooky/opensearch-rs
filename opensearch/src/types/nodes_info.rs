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

//! Response types of the `nodes_info` specification namespace

#![allow(missing_docs)]

#[doc = "The response of the `nodes.info` API"]
pub type InfoResponse = crate::types::nodes::InfoResponse;

#[derive(Debug, Clone)]
pub struct NodeInfo {
    #[doc = "Information about available aggregation types."]
    pub aggregations:
        Option<std::collections::BTreeMap<String, crate::types::nodes_info::NodeInfoAggregation>>,
    #[doc = "The custom attributes of the node."]
    pub attributes: Option<std::collections::BTreeMap<String, String>>,
    pub build_flavor: Option<String>,
    #[doc = "Short hash of the last git commit in this release."]
    pub build_hash: String,
    #[doc = "The type of build, such as `tar`, `docker`, `rpm`."]
    pub build_type: String,
    pub host: Option<crate::types::common::Host>,
    pub http: Option<crate::types::nodes_info::NodeInfoHttp>,
    pub ingest: Option<crate::types::nodes_info::NodeInfoIngest>,
    pub ip: Option<crate::types::common::Ip>,
    pub jvm: Option<crate::types::nodes_info::NodeJvmInfo>,
    #[doc = "The list of modules installed on the node."]
    pub modules: Option<Vec<crate::types::common::PluginStats>>,
    pub name: crate::types::common::Name,
    pub network: Option<crate::types::nodes_info::NodeInfoNetwork>,
    pub os: Option<crate::types::nodes_info::NodeOperatingSystemInfo>,
    #[doc = "The list of plugins installed on the node."]
    pub plugins: Option<Vec<crate::types::common::PluginStats>>,
    pub process: Option<crate::types::nodes_info::NodeProcessInfo>,
    pub roles: crate::types::common::NodeRoles,
    pub search_pipelines: Option<crate::types::nodes_info::NodeInfoSearchPipelines>,
    pub settings: Option<crate::types::nodes_info::NodeInfoSettings>,
    #[doc = "Information about thread pool configurations."]
    pub thread_pool:
        Option<std::collections::BTreeMap<String, crate::types::nodes_info::NodeThreadPoolInfo>>,
    #[doc = "Total heap allowed to be used to hold recently indexed documents before they must be written to disk. This size is a shared pool across all shards on this node, and is controlled by Indexing Buffer settings."]
    pub total_indexing_buffer: Option<crate::types::nodes_info::NodeInfoTotalIndexingBuffer>,
    pub total_indexing_buffer_in_bytes:
        Option<crate::types::nodes_info::NodeInfoTotalIndexingBufferInBytes>,
    pub transport: Option<crate::types::nodes_info::NodeInfoTransport>,
    pub transport_address: Option<crate::types::common::TransportAddress>,
    pub version: crate::types::common::VersionString,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfo object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_aggregations: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::nodes_info::NodeInfoAggregation,
                    >,
                > = None;
                let mut seen_aggregations = false;
                let mut f_attributes: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_attributes = false;
                let mut f_build_flavor: Option<String> = None;
                let mut seen_build_flavor = false;
                let mut f_build_hash: Option<String> = None;
                let mut f_build_type: Option<String> = None;
                let mut f_host: Option<crate::types::common::Host> = None;
                let mut seen_host = false;
                let mut f_http: Option<crate::types::nodes_info::NodeInfoHttp> = None;
                let mut seen_http = false;
                let mut f_ingest: Option<crate::types::nodes_info::NodeInfoIngest> = None;
                let mut seen_ingest = false;
                let mut f_ip: Option<crate::types::common::Ip> = None;
                let mut seen_ip = false;
                let mut f_jvm: Option<crate::types::nodes_info::NodeJvmInfo> = None;
                let mut seen_jvm = false;
                let mut f_modules: Option<Vec<crate::types::common::PluginStats>> = None;
                let mut seen_modules = false;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut f_network: Option<crate::types::nodes_info::NodeInfoNetwork> = None;
                let mut seen_network = false;
                let mut f_os: Option<crate::types::nodes_info::NodeOperatingSystemInfo> = None;
                let mut seen_os = false;
                let mut f_plugins: Option<Vec<crate::types::common::PluginStats>> = None;
                let mut seen_plugins = false;
                let mut f_process: Option<crate::types::nodes_info::NodeProcessInfo> = None;
                let mut seen_process = false;
                let mut f_roles: Option<crate::types::common::NodeRoles> = None;
                let mut f_search_pipelines: Option<
                    crate::types::nodes_info::NodeInfoSearchPipelines,
                > = None;
                let mut seen_search_pipelines = false;
                let mut f_settings: Option<crate::types::nodes_info::NodeInfoSettings> = None;
                let mut seen_settings = false;
                let mut f_thread_pool: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::nodes_info::NodeThreadPoolInfo,
                    >,
                > = None;
                let mut seen_thread_pool = false;
                let mut f_total_indexing_buffer: Option<
                    crate::types::nodes_info::NodeInfoTotalIndexingBuffer,
                > = None;
                let mut seen_total_indexing_buffer = false;
                let mut f_total_indexing_buffer_in_bytes: Option<
                    crate::types::nodes_info::NodeInfoTotalIndexingBufferInBytes,
                > = None;
                let mut seen_total_indexing_buffer_in_bytes = false;
                let mut f_transport: Option<crate::types::nodes_info::NodeInfoTransport> = None;
                let mut seen_transport = false;
                let mut f_transport_address: Option<crate::types::common::TransportAddress> = None;
                let mut seen_transport_address = false;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "aggregations" => {
                            if seen_aggregations {
                                return Err(serde::de::Error::duplicate_field("aggregations"));
                            }
                            seen_aggregations = true;
                            f_aggregations = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::nodes_info::NodeInfoAggregation,
                                >,
                            >>()?;
                        }
                        "attributes" => {
                            if seen_attributes {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            seen_attributes = true;
                            f_attributes = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "build_flavor" => {
                            if seen_build_flavor {
                                return Err(serde::de::Error::duplicate_field("build_flavor"));
                            }
                            seen_build_flavor = true;
                            f_build_flavor = map.next_value::<Option<String>>()?;
                        }
                        "build_hash" => {
                            if f_build_hash.is_some() {
                                return Err(serde::de::Error::duplicate_field("build_hash"));
                            }
                            f_build_hash = Some(map.next_value::<String>()?);
                        }
                        "build_type" => {
                            if f_build_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("build_type"));
                            }
                            f_build_type = Some(map.next_value::<String>()?);
                        }
                        "host" => {
                            if seen_host {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            seen_host = true;
                            f_host = map.next_value::<Option<crate::types::common::Host>>()?;
                        }
                        "http" => {
                            if seen_http {
                                return Err(serde::de::Error::duplicate_field("http"));
                            }
                            seen_http = true;
                            f_http =
                                map.next_value::<Option<crate::types::nodes_info::NodeInfoHttp>>()?;
                        }
                        "ingest" => {
                            if seen_ingest {
                                return Err(serde::de::Error::duplicate_field("ingest"));
                            }
                            seen_ingest = true;
                            f_ingest = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngest>>()?;
                        }
                        "ip" => {
                            if seen_ip {
                                return Err(serde::de::Error::duplicate_field("ip"));
                            }
                            seen_ip = true;
                            f_ip = map.next_value::<Option<crate::types::common::Ip>>()?;
                        }
                        "jvm" => {
                            if seen_jvm {
                                return Err(serde::de::Error::duplicate_field("jvm"));
                            }
                            seen_jvm = true;
                            f_jvm =
                                map.next_value::<Option<crate::types::nodes_info::NodeJvmInfo>>()?;
                        }
                        "modules" => {
                            if seen_modules {
                                return Err(serde::de::Error::duplicate_field("modules"));
                            }
                            seen_modules = true;
                            f_modules =
                                map.next_value::<Option<Vec<crate::types::common::PluginStats>>>()?;
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        "network" => {
                            if seen_network {
                                return Err(serde::de::Error::duplicate_field("network"));
                            }
                            seen_network = true;
                            f_network = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoNetwork>>(
                                )?;
                        }
                        "os" => {
                            if seen_os {
                                return Err(serde::de::Error::duplicate_field("os"));
                            }
                            seen_os = true;
                            f_os = map.next_value::<Option<crate::types::nodes_info::NodeOperatingSystemInfo>>()?;
                        }
                        "plugins" => {
                            if seen_plugins {
                                return Err(serde::de::Error::duplicate_field("plugins"));
                            }
                            seen_plugins = true;
                            f_plugins =
                                map.next_value::<Option<Vec<crate::types::common::PluginStats>>>()?;
                        }
                        "process" => {
                            if seen_process {
                                return Err(serde::de::Error::duplicate_field("process"));
                            }
                            seen_process = true;
                            f_process = map
                                .next_value::<Option<crate::types::nodes_info::NodeProcessInfo>>(
                                )?;
                        }
                        "roles" => {
                            if f_roles.is_some() {
                                return Err(serde::de::Error::duplicate_field("roles"));
                            }
                            f_roles = Some(map.next_value::<crate::types::common::NodeRoles>()?);
                        }
                        "search_pipelines" => {
                            if seen_search_pipelines {
                                return Err(serde::de::Error::duplicate_field("search_pipelines"));
                            }
                            seen_search_pipelines = true;
                            f_search_pipelines = map.next_value::<Option<crate::types::nodes_info::NodeInfoSearchPipelines>>()?;
                        }
                        "settings" => {
                            if seen_settings {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            seen_settings = true;
                            f_settings = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoSettings>>(
                                )?;
                        }
                        "thread_pool" => {
                            if seen_thread_pool {
                                return Err(serde::de::Error::duplicate_field("thread_pool"));
                            }
                            seen_thread_pool = true;
                            f_thread_pool = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::nodes_info::NodeThreadPoolInfo,
                                >,
                            >>()?;
                        }
                        "total_indexing_buffer" => {
                            if seen_total_indexing_buffer {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_indexing_buffer",
                                ));
                            }
                            seen_total_indexing_buffer = true;
                            f_total_indexing_buffer = map.next_value::<Option<crate::types::nodes_info::NodeInfoTotalIndexingBuffer>>()?;
                        }
                        "total_indexing_buffer_in_bytes" => {
                            if seen_total_indexing_buffer_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_indexing_buffer_in_bytes",
                                ));
                            }
                            seen_total_indexing_buffer_in_bytes = true;
                            f_total_indexing_buffer_in_bytes = map
                                .next_value::<Option<
                                    crate::types::nodes_info::NodeInfoTotalIndexingBufferInBytes,
                                >>()?;
                        }
                        "transport" => {
                            if seen_transport {
                                return Err(serde::de::Error::duplicate_field("transport"));
                            }
                            seen_transport = true;
                            f_transport = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoTransport>>(
                                )?;
                        }
                        "transport_address" => {
                            if seen_transport_address {
                                return Err(serde::de::Error::duplicate_field("transport_address"));
                            }
                            seen_transport_address = true;
                            f_transport_address =
                                map.next_value::<Option<crate::types::common::TransportAddress>>()?;
                        }
                        "version" => {
                            if f_version.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            f_version =
                                Some(map.next_value::<crate::types::common::VersionString>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfo {
                    aggregations: f_aggregations,
                    attributes: f_attributes,
                    build_flavor: f_build_flavor,
                    build_hash: f_build_hash
                        .ok_or_else(|| serde::de::Error::missing_field("build_hash"))?,
                    build_type: f_build_type
                        .ok_or_else(|| serde::de::Error::missing_field("build_type"))?,
                    host: f_host,
                    http: f_http,
                    ingest: f_ingest,
                    ip: f_ip,
                    jvm: f_jvm,
                    modules: f_modules,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    network: f_network,
                    os: f_os,
                    plugins: f_plugins,
                    process: f_process,
                    roles: f_roles.ok_or_else(|| serde::de::Error::missing_field("roles"))?,
                    search_pipelines: f_search_pipelines,
                    settings: f_settings,
                    thread_pool: f_thread_pool,
                    total_indexing_buffer: f_total_indexing_buffer,
                    total_indexing_buffer_in_bytes: f_total_indexing_buffer_in_bytes,
                    transport: f_transport,
                    transport_address: f_transport_address,
                    version: f_version.ok_or_else(|| serde::de::Error::missing_field("version"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.aggregations {
            map.serialize_entry("aggregations", v)?;
        }
        if let Some(v) = &self.attributes {
            map.serialize_entry("attributes", v)?;
        }
        if let Some(v) = &self.build_flavor {
            map.serialize_entry("build_flavor", v)?;
        }
        map.serialize_entry("build_hash", &self.build_hash)?;
        map.serialize_entry("build_type", &self.build_type)?;
        if let Some(v) = &self.host {
            map.serialize_entry("host", v)?;
        }
        if let Some(v) = &self.http {
            map.serialize_entry("http", v)?;
        }
        if let Some(v) = &self.ingest {
            map.serialize_entry("ingest", v)?;
        }
        if let Some(v) = &self.ip {
            map.serialize_entry("ip", v)?;
        }
        if let Some(v) = &self.jvm {
            map.serialize_entry("jvm", v)?;
        }
        if let Some(v) = &self.modules {
            map.serialize_entry("modules", v)?;
        }
        map.serialize_entry("name", &self.name)?;
        if let Some(v) = &self.network {
            map.serialize_entry("network", v)?;
        }
        if let Some(v) = &self.os {
            map.serialize_entry("os", v)?;
        }
        if let Some(v) = &self.plugins {
            map.serialize_entry("plugins", v)?;
        }
        if let Some(v) = &self.process {
            map.serialize_entry("process", v)?;
        }
        map.serialize_entry("roles", &self.roles)?;
        if let Some(v) = &self.search_pipelines {
            map.serialize_entry("search_pipelines", v)?;
        }
        if let Some(v) = &self.settings {
            map.serialize_entry("settings", v)?;
        }
        if let Some(v) = &self.thread_pool {
            map.serialize_entry("thread_pool", v)?;
        }
        if let Some(v) = &self.total_indexing_buffer {
            map.serialize_entry("total_indexing_buffer", v)?;
        }
        if let Some(v) = &self.total_indexing_buffer_in_bytes {
            map.serialize_entry("total_indexing_buffer_in_bytes", v)?;
        }
        if let Some(v) = &self.transport {
            map.serialize_entry("transport", v)?;
        }
        if let Some(v) = &self.transport_address {
            map.serialize_entry("transport_address", v)?;
        }
        map.serialize_entry("version", &self.version)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The action-related configuration settings."]
#[derive(Debug, Clone)]
pub struct NodeInfoAction {
    #[doc = "Whether destructive actions require explicit names."]
    pub destructive_requires_name: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoAction {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoAction;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoAction object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_destructive_requires_name: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "destructive_requires_name" => {
                            if f_destructive_requires_name.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "destructive_requires_name",
                                ));
                            }
                            f_destructive_requires_name = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoAction {
                    destructive_requires_name: f_destructive_requires_name.ok_or_else(|| {
                        serde::de::Error::missing_field("destructive_requires_name")
                    })?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoAction {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("destructive_requires_name", &self.destructive_requires_name)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The aggregation configuration."]
#[derive(Debug, Clone)]
pub struct NodeInfoAggregation {
    #[doc = "The list of supported aggregation types."]
    pub types: Vec<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoAggregation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoAggregation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoAggregation object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_types: Option<Vec<String>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "types" => {
                            if f_types.is_some() {
                                return Err(serde::de::Error::duplicate_field("types"));
                            }
                            f_types = Some(map.next_value::<Vec<String>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoAggregation {
                    types: f_types.ok_or_else(|| serde::de::Error::missing_field("types"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoAggregation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("types", &self.types)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The bootstrap configuration settings for the node."]
#[derive(Debug, Clone)]
pub struct NodeInfoBootstrap {
    #[doc = "The memory lock setting for the bootstrap process."]
    pub memory_lock: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoBootstrap {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoBootstrap;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoBootstrap object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_memory_lock: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "memory_lock" => {
                            if f_memory_lock.is_some() {
                                return Err(serde::de::Error::duplicate_field("memory_lock"));
                            }
                            f_memory_lock = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoBootstrap {
                    memory_lock: f_memory_lock
                        .ok_or_else(|| serde::de::Error::missing_field("memory_lock"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoBootstrap {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("memory_lock", &self.memory_lock)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The client node configuration."]
#[derive(Debug, Clone)]
pub struct NodeInfoClient {
    #[doc = "The type of client node configuration."]
    pub r#type: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoClient {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoClient;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoClient object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_type: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
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
                Ok(NodeInfoClient {
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoClient {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The node discovery configuration."]
#[derive(Debug, Clone)]
pub struct NodeInfoDiscovery {
    #[doc = "The configured seed hosts for node discovery."]
    pub seed_hosts: Option<String>,
    #[doc = "The discovery mechanism type."]
    pub r#type: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoDiscovery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoDiscovery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoDiscovery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_seed_hosts: Option<String> = None;
                let mut seen_seed_hosts = false;
                let mut f_type: Option<String> = None;
                let mut seen_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "seed_hosts" => {
                            if seen_seed_hosts {
                                return Err(serde::de::Error::duplicate_field("seed_hosts"));
                            }
                            seen_seed_hosts = true;
                            f_seed_hosts = map.next_value::<Option<String>>()?;
                        }
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoDiscovery {
                    seed_hosts: f_seed_hosts,
                    r#type: f_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoDiscovery {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.seed_hosts {
            map.serialize_entry("seed_hosts", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("type", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Provides HTTP connection information for the node."]
#[derive(Debug, Clone)]
pub struct NodeInfoHttp {
    #[doc = "The addresses the node is bound to for HTTP traffic."]
    pub bound_address: Vec<String>,
    pub max_content_length: Option<crate::types::common::HumanReadableByteCount>,
    pub max_content_length_in_bytes: crate::types::common::ByteCount,
    #[doc = "The HTTP address published to other nodes."]
    pub publish_address: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoHttp {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoHttp;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoHttp object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_bound_address: Option<Vec<String>> = None;
                let mut f_max_content_length: Option<crate::types::common::HumanReadableByteCount> =
                    None;
                let mut seen_max_content_length = false;
                let mut f_max_content_length_in_bytes: Option<crate::types::common::ByteCount> =
                    None;
                let mut f_publish_address: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "bound_address" => {
                            if f_bound_address.is_some() {
                                return Err(serde::de::Error::duplicate_field("bound_address"));
                            }
                            f_bound_address = Some(map.next_value::<Vec<String>>()?);
                        }
                        "max_content_length" => {
                            if seen_max_content_length {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_content_length",
                                ));
                            }
                            seen_max_content_length = true;
                            f_max_content_length = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "max_content_length_in_bytes" => {
                            if f_max_content_length_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_content_length_in_bytes",
                                ));
                            }
                            f_max_content_length_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "publish_address" => {
                            if f_publish_address.is_some() {
                                return Err(serde::de::Error::duplicate_field("publish_address"));
                            }
                            f_publish_address = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoHttp {
                    bound_address: f_bound_address
                        .ok_or_else(|| serde::de::Error::missing_field("bound_address"))?,
                    max_content_length: f_max_content_length,
                    max_content_length_in_bytes: f_max_content_length_in_bytes.ok_or_else(
                        || serde::de::Error::missing_field("max_content_length_in_bytes"),
                    )?,
                    publish_address: f_publish_address
                        .ok_or_else(|| serde::de::Error::missing_field("publish_address"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoHttp {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("bound_address", &self.bound_address)?;
        if let Some(v) = &self.max_content_length {
            map.serialize_entry("max_content_length", v)?;
        }
        map.serialize_entry(
            "max_content_length_in_bytes",
            &self.max_content_length_in_bytes,
        )?;
        map.serialize_entry("publish_address", &self.publish_address)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The ingest pipeline configuration."]
#[derive(Debug, Clone)]
pub struct NodeInfoIngest {
    #[doc = "The list of configured ingest processors."]
    pub processors: Vec<crate::types::nodes_info::NodeInfoIngestProcessor>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoIngest {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoIngest;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoIngest object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_processors: Option<
                    Vec<crate::types::nodes_info::NodeInfoIngestProcessor>,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "processors" => {
                            if f_processors.is_some() {
                                return Err(serde::de::Error::duplicate_field("processors"));
                            }
                            f_processors = Some(map.next_value::<Vec<crate::types::nodes_info::NodeInfoIngestProcessor>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoIngest {
                    processors: f_processors
                        .ok_or_else(|| serde::de::Error::missing_field("processors"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoIngest {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("processors", &self.processors)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The ingest processor download configuration."]
#[derive(Debug, Clone)]
pub struct NodeInfoIngestDownloader {
    #[doc = "Whether the downloader is enabled."]
    pub enabled: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoIngestDownloader {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoIngestDownloader;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoIngestDownloader object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_enabled: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "enabled" => {
                            if f_enabled.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            f_enabled = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoIngestDownloader {
                    enabled: f_enabled.ok_or_else(|| serde::de::Error::missing_field("enabled"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoIngestDownloader {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("enabled", &self.enabled)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The ingest processor configuration details."]
#[derive(Debug, Clone)]
pub struct NodeInfoIngestInfo {
    pub downloader: crate::types::nodes_info::NodeInfoIngestDownloader,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoIngestInfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoIngestInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoIngestInfo object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_downloader: Option<crate::types::nodes_info::NodeInfoIngestDownloader> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "downloader" => {
                            if f_downloader.is_some() {
                                return Err(serde::de::Error::duplicate_field("downloader"));
                            }
                            f_downloader = Some(map.next_value::<crate::types::nodes_info::NodeInfoIngestDownloader>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoIngestInfo {
                    downloader: f_downloader
                        .ok_or_else(|| serde::de::Error::missing_field("downloader"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoIngestInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("downloader", &self.downloader)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The ingest processor configuration."]
#[derive(Debug, Clone)]
pub struct NodeInfoIngestProcessor {
    #[doc = "The type of ingest processor."]
    pub r#type: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoIngestProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoIngestProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoIngestProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_type: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
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
                Ok(NodeInfoIngestProcessor {
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoIngestProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Provides JVM memory settings and statistics."]
#[derive(Debug, Clone)]
pub struct NodeInfoJvmMemory {
    pub direct_max: Option<crate::types::common::HumanReadableByteCount>,
    pub direct_max_in_bytes: crate::types::common::ByteCount,
    pub heap_init: Option<crate::types::common::HumanReadableByteCount>,
    pub heap_init_in_bytes: crate::types::common::ByteCount,
    pub heap_max: Option<crate::types::common::HumanReadableByteCount>,
    pub heap_max_in_bytes: crate::types::common::ByteCount,
    pub non_heap_init: Option<crate::types::common::HumanReadableByteCount>,
    pub non_heap_init_in_bytes: crate::types::common::ByteCount,
    pub non_heap_max: Option<crate::types::common::HumanReadableByteCount>,
    pub non_heap_max_in_bytes: crate::types::common::ByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoJvmMemory {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoJvmMemory;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoJvmMemory object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_direct_max: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_direct_max = false;
                let mut f_direct_max_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_heap_init: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_heap_init = false;
                let mut f_heap_init_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_heap_max: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_heap_max = false;
                let mut f_heap_max_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_non_heap_init: Option<crate::types::common::HumanReadableByteCount> =
                    None;
                let mut seen_non_heap_init = false;
                let mut f_non_heap_init_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_non_heap_max: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_non_heap_max = false;
                let mut f_non_heap_max_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "direct_max" => {
                            if seen_direct_max {
                                return Err(serde::de::Error::duplicate_field("direct_max"));
                            }
                            seen_direct_max = true;
                            f_direct_max = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "direct_max_in_bytes" => {
                            if f_direct_max_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "direct_max_in_bytes",
                                ));
                            }
                            f_direct_max_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "heap_init" => {
                            if seen_heap_init {
                                return Err(serde::de::Error::duplicate_field("heap_init"));
                            }
                            seen_heap_init = true;
                            f_heap_init = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "heap_init_in_bytes" => {
                            if f_heap_init_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "heap_init_in_bytes",
                                ));
                            }
                            f_heap_init_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "heap_max" => {
                            if seen_heap_max {
                                return Err(serde::de::Error::duplicate_field("heap_max"));
                            }
                            seen_heap_max = true;
                            f_heap_max = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "heap_max_in_bytes" => {
                            if f_heap_max_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("heap_max_in_bytes"));
                            }
                            f_heap_max_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "non_heap_init" => {
                            if seen_non_heap_init {
                                return Err(serde::de::Error::duplicate_field("non_heap_init"));
                            }
                            seen_non_heap_init = true;
                            f_non_heap_init = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "non_heap_init_in_bytes" => {
                            if f_non_heap_init_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "non_heap_init_in_bytes",
                                ));
                            }
                            f_non_heap_init_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "non_heap_max" => {
                            if seen_non_heap_max {
                                return Err(serde::de::Error::duplicate_field("non_heap_max"));
                            }
                            seen_non_heap_max = true;
                            f_non_heap_max = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "non_heap_max_in_bytes" => {
                            if f_non_heap_max_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "non_heap_max_in_bytes",
                                ));
                            }
                            f_non_heap_max_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoJvmMemory {
                    direct_max: f_direct_max,
                    direct_max_in_bytes: f_direct_max_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("direct_max_in_bytes"))?,
                    heap_init: f_heap_init,
                    heap_init_in_bytes: f_heap_init_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("heap_init_in_bytes"))?,
                    heap_max: f_heap_max,
                    heap_max_in_bytes: f_heap_max_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("heap_max_in_bytes"))?,
                    non_heap_init: f_non_heap_init,
                    non_heap_init_in_bytes: f_non_heap_init_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("non_heap_init_in_bytes"))?,
                    non_heap_max: f_non_heap_max,
                    non_heap_max_in_bytes: f_non_heap_max_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("non_heap_max_in_bytes"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoJvmMemory {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.direct_max {
            map.serialize_entry("direct_max", v)?;
        }
        map.serialize_entry("direct_max_in_bytes", &self.direct_max_in_bytes)?;
        if let Some(v) = &self.heap_init {
            map.serialize_entry("heap_init", v)?;
        }
        map.serialize_entry("heap_init_in_bytes", &self.heap_init_in_bytes)?;
        if let Some(v) = &self.heap_max {
            map.serialize_entry("heap_max", v)?;
        }
        map.serialize_entry("heap_max_in_bytes", &self.heap_max_in_bytes)?;
        if let Some(v) = &self.non_heap_init {
            map.serialize_entry("non_heap_init", v)?;
        }
        map.serialize_entry("non_heap_init_in_bytes", &self.non_heap_init_in_bytes)?;
        if let Some(v) = &self.non_heap_max {
            map.serialize_entry("non_heap_max", v)?;
        }
        map.serialize_entry("non_heap_max_in_bytes", &self.non_heap_max_in_bytes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Provides memory information for the node."]
#[derive(Debug, Clone)]
pub struct NodeInfoMemory {
    pub total: crate::types::common::HumanReadableByteCount,
    pub total_in_bytes: crate::types::common::ByteCount,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoMemory {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoMemory;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoMemory object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_total: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut f_total_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(
                                map.next_value::<crate::types::common::HumanReadableByteCount>()?,
                            );
                        }
                        "total_in_bytes" => {
                            if f_total_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("total_in_bytes"));
                            }
                            f_total_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoMemory {
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    total_in_bytes: f_total_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("total_in_bytes"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoMemory {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("total", &self.total)?;
        map.serialize_entry("total_in_bytes", &self.total_in_bytes)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Provides network-related information for the node."]
#[derive(Debug, Clone)]
pub struct NodeInfoNetwork {
    pub primary_interface: crate::types::nodes_info::NodeInfoNetworkInterface,
    #[doc = "The interval for refreshing network information."]
    pub refresh_interval: i32,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoNetwork {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoNetwork;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoNetwork object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_primary_interface: Option<
                    crate::types::nodes_info::NodeInfoNetworkInterface,
                > = None;
                let mut f_refresh_interval: Option<i32> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "primary_interface" => {
                            if f_primary_interface.is_some() {
                                return Err(serde::de::Error::duplicate_field("primary_interface"));
                            }
                            f_primary_interface = Some(map.next_value::<crate::types::nodes_info::NodeInfoNetworkInterface>()?);
                        }
                        "refresh_interval" => {
                            if f_refresh_interval.is_some() {
                                return Err(serde::de::Error::duplicate_field("refresh_interval"));
                            }
                            f_refresh_interval = Some(map.next_value::<i32>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoNetwork {
                    primary_interface: f_primary_interface
                        .ok_or_else(|| serde::de::Error::missing_field("primary_interface"))?,
                    refresh_interval: f_refresh_interval
                        .ok_or_else(|| serde::de::Error::missing_field("refresh_interval"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoNetwork {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("primary_interface", &self.primary_interface)?;
        map.serialize_entry("refresh_interval", &self.refresh_interval)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Describes a network interface on the node."]
#[derive(Debug, Clone)]
pub struct NodeInfoNetworkInterface {
    #[doc = "The IP address of the interface."]
    pub address: String,
    #[doc = "The MAC address of the interface."]
    pub mac_address: String,
    pub name: crate::types::common::Name,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoNetworkInterface {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoNetworkInterface;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoNetworkInterface object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_address: Option<String> = None;
                let mut f_mac_address: Option<String> = None;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "address" => {
                            if f_address.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            f_address = Some(map.next_value::<String>()?);
                        }
                        "mac_address" => {
                            if f_mac_address.is_some() {
                                return Err(serde::de::Error::duplicate_field("mac_address"));
                            }
                            f_mac_address = Some(map.next_value::<String>()?);
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoNetworkInterface {
                    address: f_address.ok_or_else(|| serde::de::Error::missing_field("address"))?,
                    mac_address: f_mac_address
                        .ok_or_else(|| serde::de::Error::missing_field("mac_address"))?,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoNetworkInterface {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("address", &self.address)?;
        map.serialize_entry("mac_address", &self.mac_address)?;
        map.serialize_entry("name", &self.name)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Provides CPU information for the node."]
#[derive(Debug, Clone)]
pub struct NodeInfoOSCPU {
    pub cache_size: crate::types::common::HumanReadableByteCount,
    pub cache_size_in_bytes: crate::types::common::ByteCount,
    #[doc = "The number of cores per CPU socket."]
    pub cores_per_socket: i32,
    #[doc = "The CPU clock speed in MHz."]
    pub mhz: i32,
    #[doc = "The CPU model name."]
    pub model: String,
    #[doc = "The total number of CPU cores."]
    pub total_cores: i32,
    #[doc = "The total number of CPU sockets."]
    pub total_sockets: i32,
    #[doc = "The CPU vendor name."]
    pub vendor: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoOSCPU {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoOSCPU;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoOSCPU object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cache_size: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut f_cache_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_cores_per_socket: Option<i32> = None;
                let mut f_mhz: Option<i32> = None;
                let mut f_model: Option<String> = None;
                let mut f_total_cores: Option<i32> = None;
                let mut f_total_sockets: Option<i32> = None;
                let mut f_vendor: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cache_size" => {
                            if f_cache_size.is_some() {
                                return Err(serde::de::Error::duplicate_field("cache_size"));
                            }
                            f_cache_size = Some(
                                map.next_value::<crate::types::common::HumanReadableByteCount>()?,
                            );
                        }
                        "cache_size_in_bytes" => {
                            if f_cache_size_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "cache_size_in_bytes",
                                ));
                            }
                            f_cache_size_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
                        }
                        "cores_per_socket" => {
                            if f_cores_per_socket.is_some() {
                                return Err(serde::de::Error::duplicate_field("cores_per_socket"));
                            }
                            f_cores_per_socket = Some(map.next_value::<i32>()?);
                        }
                        "mhz" => {
                            if f_mhz.is_some() {
                                return Err(serde::de::Error::duplicate_field("mhz"));
                            }
                            f_mhz = Some(map.next_value::<i32>()?);
                        }
                        "model" => {
                            if f_model.is_some() {
                                return Err(serde::de::Error::duplicate_field("model"));
                            }
                            f_model = Some(map.next_value::<String>()?);
                        }
                        "total_cores" => {
                            if f_total_cores.is_some() {
                                return Err(serde::de::Error::duplicate_field("total_cores"));
                            }
                            f_total_cores = Some(map.next_value::<i32>()?);
                        }
                        "total_sockets" => {
                            if f_total_sockets.is_some() {
                                return Err(serde::de::Error::duplicate_field("total_sockets"));
                            }
                            f_total_sockets = Some(map.next_value::<i32>()?);
                        }
                        "vendor" => {
                            if f_vendor.is_some() {
                                return Err(serde::de::Error::duplicate_field("vendor"));
                            }
                            f_vendor = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoOSCPU {
                    cache_size: f_cache_size
                        .ok_or_else(|| serde::de::Error::missing_field("cache_size"))?,
                    cache_size_in_bytes: f_cache_size_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("cache_size_in_bytes"))?,
                    cores_per_socket: f_cores_per_socket
                        .ok_or_else(|| serde::de::Error::missing_field("cores_per_socket"))?,
                    mhz: f_mhz.ok_or_else(|| serde::de::Error::missing_field("mhz"))?,
                    model: f_model.ok_or_else(|| serde::de::Error::missing_field("model"))?,
                    total_cores: f_total_cores
                        .ok_or_else(|| serde::de::Error::missing_field("total_cores"))?,
                    total_sockets: f_total_sockets
                        .ok_or_else(|| serde::de::Error::missing_field("total_sockets"))?,
                    vendor: f_vendor.ok_or_else(|| serde::de::Error::missing_field("vendor"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoOSCPU {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("cache_size", &self.cache_size)?;
        map.serialize_entry("cache_size_in_bytes", &self.cache_size_in_bytes)?;
        map.serialize_entry("cores_per_socket", &self.cores_per_socket)?;
        map.serialize_entry("mhz", &self.mhz)?;
        map.serialize_entry("model", &self.model)?;
        map.serialize_entry("total_cores", &self.total_cores)?;
        map.serialize_entry("total_sockets", &self.total_sockets)?;
        map.serialize_entry("vendor", &self.vendor)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The `filesystem` path configurations for the node."]
#[derive(Debug, Clone)]
pub struct NodeInfoPath {
    #[doc = "The paths configured for data storage."]
    pub data: Option<Vec<String>>,
    #[doc = "The OpenSearch home directory path."]
    pub home: String,
    #[doc = "The path to node log files."]
    pub logs: String,
    #[doc = "The paths configured for snapshot repositories."]
    pub repo: Option<Vec<String>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoPath {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoPath;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoPath object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_data: Option<Vec<String>> = None;
                let mut seen_data = false;
                let mut f_home: Option<String> = None;
                let mut f_logs: Option<String> = None;
                let mut f_repo: Option<Vec<String>> = None;
                let mut seen_repo = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "data" => {
                            if seen_data {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            seen_data = true;
                            f_data = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "home" => {
                            if f_home.is_some() {
                                return Err(serde::de::Error::duplicate_field("home"));
                            }
                            f_home = Some(map.next_value::<String>()?);
                        }
                        "logs" => {
                            if f_logs.is_some() {
                                return Err(serde::de::Error::duplicate_field("logs"));
                            }
                            f_logs = Some(map.next_value::<String>()?);
                        }
                        "repo" => {
                            if seen_repo {
                                return Err(serde::de::Error::duplicate_field("repo"));
                            }
                            seen_repo = true;
                            f_repo = map.next_value::<Option<Vec<String>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoPath {
                    data: f_data,
                    home: f_home.ok_or_else(|| serde::de::Error::missing_field("home"))?,
                    logs: f_logs.ok_or_else(|| serde::de::Error::missing_field("logs"))?,
                    repo: f_repo,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoPath {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.data {
            map.serialize_entry("data", v)?;
        }
        map.serialize_entry("home", &self.home)?;
        map.serialize_entry("logs", &self.logs)?;
        if let Some(v) = &self.repo {
            map.serialize_entry("repo", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The repository configuration information."]
#[derive(Debug, Clone)]
pub struct NodeInfoRepositories {
    pub url: crate::types::nodes_info::NodeInfoRepositoriesUrl,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoRepositories {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoRepositories;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoRepositories object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_url: Option<crate::types::nodes_info::NodeInfoRepositoriesUrl> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "url" => {
                            if f_url.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            f_url = Some(map.next_value::<crate::types::nodes_info::NodeInfoRepositoriesUrl>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoRepositories {
                    url: f_url.ok_or_else(|| serde::de::Error::missing_field("url"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoRepositories {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("url", &self.url)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Specifies allowed repository URLs."]
#[derive(Debug, Clone)]
pub struct NodeInfoRepositoriesUrl {
    #[doc = "The pattern for allowed repository URLs."]
    pub allowed_urls: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoRepositoriesUrl {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoRepositoriesUrl;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoRepositoriesUrl object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_allowed_urls: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "allowed_urls" => {
                            if f_allowed_urls.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowed_urls"));
                            }
                            f_allowed_urls = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoRepositoriesUrl {
                    allowed_urls: f_allowed_urls
                        .ok_or_else(|| serde::de::Error::missing_field("allowed_urls"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoRepositoriesUrl {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("allowed_urls", &self.allowed_urls)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The script execution configuration."]
#[derive(Debug, Clone)]
pub struct NodeInfoScript {
    #[doc = "The types of scripts allowed."]
    pub allowed_types: String,
    #[doc = "The setting to disable maximum script compilation rate."]
    pub disable_max_compilations_rate: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoScript {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoScript;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoScript object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_allowed_types: Option<String> = None;
                let mut f_disable_max_compilations_rate: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "allowed_types" => {
                            if f_allowed_types.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowed_types"));
                            }
                            f_allowed_types = Some(map.next_value::<String>()?);
                        }
                        "disable_max_compilations_rate" => {
                            if f_disable_max_compilations_rate.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "disable_max_compilations_rate",
                                ));
                            }
                            f_disable_max_compilations_rate = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoScript {
                    allowed_types: f_allowed_types
                        .ok_or_else(|| serde::de::Error::missing_field("allowed_types"))?,
                    disable_max_compilations_rate: f_disable_max_compilations_rate.ok_or_else(
                        || serde::de::Error::missing_field("disable_max_compilations_rate"),
                    )?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoScript {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("allowed_types", &self.allowed_types)?;
        map.serialize_entry(
            "disable_max_compilations_rate",
            &self.disable_max_compilations_rate,
        )?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The search operation configuration."]
#[derive(Debug, Clone)]
pub struct NodeInfoSearch {
    pub remote: crate::types::nodes_info::NodeInfoSearchRemote,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoSearch {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoSearch;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoSearch object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_remote: Option<crate::types::nodes_info::NodeInfoSearchRemote> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "remote" => {
                            if f_remote.is_some() {
                                return Err(serde::de::Error::duplicate_field("remote"));
                            }
                            f_remote = Some(
                                map.next_value::<crate::types::nodes_info::NodeInfoSearchRemote>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoSearch {
                    remote: f_remote.ok_or_else(|| serde::de::Error::missing_field("remote"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoSearch {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("remote", &self.remote)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The search pipeline processor configuration."]
#[derive(Debug, Clone)]
pub struct NodeInfoSearchPipelineProcessor {
    #[doc = "The type of search pipeline processor."]
    pub r#type: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoSearchPipelineProcessor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoSearchPipelineProcessor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoSearchPipelineProcessor object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_type: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
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
                Ok(NodeInfoSearchPipelineProcessor {
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoSearchPipelineProcessor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The search pipeline configuration."]
#[derive(Debug, Clone)]
pub struct NodeInfoSearchPipelines {
    #[doc = "The list of phase results processors in the pipeline."]
    pub phase_results_processors:
        Option<Vec<crate::types::nodes_info::NodeInfoSearchPipelineProcessor>>,
    #[doc = "The list of request processors in the pipeline."]
    pub request_processors: Option<Vec<crate::types::nodes_info::NodeInfoSearchPipelineProcessor>>,
    #[doc = "The list of response processors in the pipeline."]
    pub response_processors: Option<Vec<crate::types::nodes_info::NodeInfoSearchPipelineProcessor>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoSearchPipelines {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoSearchPipelines;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoSearchPipelines object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_phase_results_processors: Option<
                    Vec<crate::types::nodes_info::NodeInfoSearchPipelineProcessor>,
                > = None;
                let mut seen_phase_results_processors = false;
                let mut f_request_processors: Option<
                    Vec<crate::types::nodes_info::NodeInfoSearchPipelineProcessor>,
                > = None;
                let mut seen_request_processors = false;
                let mut f_response_processors: Option<
                    Vec<crate::types::nodes_info::NodeInfoSearchPipelineProcessor>,
                > = None;
                let mut seen_response_processors = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "phase_results_processors" => {
                            if seen_phase_results_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "phase_results_processors",
                                ));
                            }
                            seen_phase_results_processors = true;
                            f_phase_results_processors = map.next_value::<Option<
                                Vec<crate::types::nodes_info::NodeInfoSearchPipelineProcessor>,
                            >>()?;
                        }
                        "request_processors" => {
                            if seen_request_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "request_processors",
                                ));
                            }
                            seen_request_processors = true;
                            f_request_processors = map.next_value::<Option<
                                Vec<crate::types::nodes_info::NodeInfoSearchPipelineProcessor>,
                            >>()?;
                        }
                        "response_processors" => {
                            if seen_response_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "response_processors",
                                ));
                            }
                            seen_response_processors = true;
                            f_response_processors = map.next_value::<Option<
                                Vec<crate::types::nodes_info::NodeInfoSearchPipelineProcessor>,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoSearchPipelines {
                    phase_results_processors: f_phase_results_processors,
                    request_processors: f_request_processors,
                    response_processors: f_response_processors,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoSearchPipelines {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.phase_results_processors {
            map.serialize_entry("phase_results_processors", v)?;
        }
        if let Some(v) = &self.request_processors {
            map.serialize_entry("request_processors", v)?;
        }
        if let Some(v) = &self.response_processors {
            map.serialize_entry("response_processors", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The remote search connection configuration."]
#[derive(Debug, Clone)]
pub struct NodeInfoSearchRemote {
    #[doc = "The remote connection settings."]
    pub connect: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoSearchRemote {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoSearchRemote;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoSearchRemote object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_connect: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "connect" => {
                            if f_connect.is_some() {
                                return Err(serde::de::Error::duplicate_field("connect"));
                            }
                            f_connect = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoSearchRemote {
                    connect: f_connect.ok_or_else(|| serde::de::Error::missing_field("connect"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoSearchRemote {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("connect", &self.connect)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration settings for the node."]
#[derive(Debug, Clone)]
pub struct NodeInfoSettings {
    pub action: Option<crate::types::nodes_info::NodeInfoAction>,
    pub bootstrap: Option<crate::types::nodes_info::NodeInfoBootstrap>,
    pub client: crate::types::nodes_info::NodeInfoClient,
    pub cluster: crate::types::nodes_info::NodeInfoSettingsCluster,
    pub discovery: Option<crate::types::nodes_info::NodeInfoDiscovery>,
    pub http: crate::types::nodes_info::NodeInfoSettingsHttp,
    pub index: Option<crate::types::nodes_info::NodeInfoSettingsIndex>,
    pub ingest: Option<crate::types::nodes_info::NodeInfoSettingsIngest>,
    pub network: Option<crate::types::nodes_info::NodeInfoSettingsNetwork>,
    pub node: crate::types::nodes_info::NodeInfoSettingsNode,
    pub path: crate::types::nodes_info::NodeInfoPath,
    pub plugins: Option<crate::types::nodes_info::NodeInfoSettingsPlugins>,
    pub repositories: Option<crate::types::nodes_info::NodeInfoRepositories>,
    pub script: Option<crate::types::nodes_info::NodeInfoScript>,
    pub search: Option<crate::types::nodes_info::NodeInfoSearch>,
    pub transport: crate::types::nodes_info::NodeInfoSettingsTransport,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoSettings {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoSettings;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoSettings object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_action: Option<crate::types::nodes_info::NodeInfoAction> = None;
                let mut seen_action = false;
                let mut f_bootstrap: Option<crate::types::nodes_info::NodeInfoBootstrap> = None;
                let mut seen_bootstrap = false;
                let mut f_client: Option<crate::types::nodes_info::NodeInfoClient> = None;
                let mut f_cluster: Option<crate::types::nodes_info::NodeInfoSettingsCluster> = None;
                let mut f_discovery: Option<crate::types::nodes_info::NodeInfoDiscovery> = None;
                let mut seen_discovery = false;
                let mut f_http: Option<crate::types::nodes_info::NodeInfoSettingsHttp> = None;
                let mut f_index: Option<crate::types::nodes_info::NodeInfoSettingsIndex> = None;
                let mut seen_index = false;
                let mut f_ingest: Option<crate::types::nodes_info::NodeInfoSettingsIngest> = None;
                let mut seen_ingest = false;
                let mut f_network: Option<crate::types::nodes_info::NodeInfoSettingsNetwork> = None;
                let mut seen_network = false;
                let mut f_node: Option<crate::types::nodes_info::NodeInfoSettingsNode> = None;
                let mut f_path: Option<crate::types::nodes_info::NodeInfoPath> = None;
                let mut f_plugins: Option<crate::types::nodes_info::NodeInfoSettingsPlugins> = None;
                let mut seen_plugins = false;
                let mut f_repositories: Option<crate::types::nodes_info::NodeInfoRepositories> =
                    None;
                let mut seen_repositories = false;
                let mut f_script: Option<crate::types::nodes_info::NodeInfoScript> = None;
                let mut seen_script = false;
                let mut f_search: Option<crate::types::nodes_info::NodeInfoSearch> = None;
                let mut seen_search = false;
                let mut f_transport: Option<crate::types::nodes_info::NodeInfoSettingsTransport> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "action" => {
                            if seen_action {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            seen_action = true;
                            f_action = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoAction>>()?;
                        }
                        "bootstrap" => {
                            if seen_bootstrap {
                                return Err(serde::de::Error::duplicate_field("bootstrap"));
                            }
                            seen_bootstrap = true;
                            f_bootstrap = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoBootstrap>>(
                                )?;
                        }
                        "client" => {
                            if f_client.is_some() {
                                return Err(serde::de::Error::duplicate_field("client"));
                            }
                            f_client =
                                Some(map.next_value::<crate::types::nodes_info::NodeInfoClient>()?);
                        }
                        "cluster" => {
                            if f_cluster.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            f_cluster = Some(map.next_value::<crate::types::nodes_info::NodeInfoSettingsCluster>()?);
                        }
                        "discovery" => {
                            if seen_discovery {
                                return Err(serde::de::Error::duplicate_field("discovery"));
                            }
                            seen_discovery = true;
                            f_discovery = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoDiscovery>>(
                                )?;
                        }
                        "http" => {
                            if f_http.is_some() {
                                return Err(serde::de::Error::duplicate_field("http"));
                            }
                            f_http = Some(
                                map.next_value::<crate::types::nodes_info::NodeInfoSettingsHttp>()?,
                            );
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<crate::types::nodes_info::NodeInfoSettingsIndex>>()?;
                        }
                        "ingest" => {
                            if seen_ingest {
                                return Err(serde::de::Error::duplicate_field("ingest"));
                            }
                            seen_ingest = true;
                            f_ingest = map.next_value::<Option<crate::types::nodes_info::NodeInfoSettingsIngest>>()?;
                        }
                        "network" => {
                            if seen_network {
                                return Err(serde::de::Error::duplicate_field("network"));
                            }
                            seen_network = true;
                            f_network = map.next_value::<Option<crate::types::nodes_info::NodeInfoSettingsNetwork>>()?;
                        }
                        "node" => {
                            if f_node.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            f_node = Some(
                                map.next_value::<crate::types::nodes_info::NodeInfoSettingsNode>()?,
                            );
                        }
                        "path" => {
                            if f_path.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            f_path =
                                Some(map.next_value::<crate::types::nodes_info::NodeInfoPath>()?);
                        }
                        "plugins" => {
                            if seen_plugins {
                                return Err(serde::de::Error::duplicate_field("plugins"));
                            }
                            seen_plugins = true;
                            f_plugins = map.next_value::<Option<crate::types::nodes_info::NodeInfoSettingsPlugins>>()?;
                        }
                        "repositories" => {
                            if seen_repositories {
                                return Err(serde::de::Error::duplicate_field("repositories"));
                            }
                            seen_repositories = true;
                            f_repositories = map.next_value::<Option<crate::types::nodes_info::NodeInfoRepositories>>()?;
                        }
                        "script" => {
                            if seen_script {
                                return Err(serde::de::Error::duplicate_field("script"));
                            }
                            seen_script = true;
                            f_script = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoScript>>()?;
                        }
                        "search" => {
                            if seen_search {
                                return Err(serde::de::Error::duplicate_field("search"));
                            }
                            seen_search = true;
                            f_search = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoSearch>>()?;
                        }
                        "transport" => {
                            if f_transport.is_some() {
                                return Err(serde::de::Error::duplicate_field("transport"));
                            }
                            f_transport = Some(map.next_value::<crate::types::nodes_info::NodeInfoSettingsTransport>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoSettings {
                    action: f_action,
                    bootstrap: f_bootstrap,
                    client: f_client.ok_or_else(|| serde::de::Error::missing_field("client"))?,
                    cluster: f_cluster.ok_or_else(|| serde::de::Error::missing_field("cluster"))?,
                    discovery: f_discovery,
                    http: f_http.ok_or_else(|| serde::de::Error::missing_field("http"))?,
                    index: f_index,
                    ingest: f_ingest,
                    network: f_network,
                    node: f_node.ok_or_else(|| serde::de::Error::missing_field("node"))?,
                    path: f_path.ok_or_else(|| serde::de::Error::missing_field("path"))?,
                    plugins: f_plugins,
                    repositories: f_repositories,
                    script: f_script,
                    search: f_search,
                    transport: f_transport
                        .ok_or_else(|| serde::de::Error::missing_field("transport"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoSettings {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.action {
            map.serialize_entry("action", v)?;
        }
        if let Some(v) = &self.bootstrap {
            map.serialize_entry("bootstrap", v)?;
        }
        map.serialize_entry("client", &self.client)?;
        map.serialize_entry("cluster", &self.cluster)?;
        if let Some(v) = &self.discovery {
            map.serialize_entry("discovery", v)?;
        }
        map.serialize_entry("http", &self.http)?;
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.ingest {
            map.serialize_entry("ingest", v)?;
        }
        if let Some(v) = &self.network {
            map.serialize_entry("network", v)?;
        }
        map.serialize_entry("node", &self.node)?;
        map.serialize_entry("path", &self.path)?;
        if let Some(v) = &self.plugins {
            map.serialize_entry("plugins", v)?;
        }
        if let Some(v) = &self.repositories {
            map.serialize_entry("repositories", v)?;
        }
        if let Some(v) = &self.script {
            map.serialize_entry("script", v)?;
        }
        if let Some(v) = &self.search {
            map.serialize_entry("search", v)?;
        }
        map.serialize_entry("transport", &self.transport)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Provides cluster-level settings information."]
#[derive(Debug, Clone)]
pub struct NodeInfoSettingsCluster {
    pub deprecation_indexing: Option<crate::types::nodes_info::NodeInfoSettingsDeprecationIndexing>,
    pub election: Option<crate::types::nodes_info::NodeInfoSettingsClusterElection>,
    pub initial_cluster_manager_nodes: Option<crate::types::common::StringOrStringArray>,
    pub initial_master_nodes: Option<crate::types::common::StringOrStringArray>,
    pub name: crate::types::common::Name,
    pub routing: Option<crate::types::indices_common::IndexRouting>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoSettingsCluster {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoSettingsCluster;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoSettingsCluster object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_deprecation_indexing: Option<
                    crate::types::nodes_info::NodeInfoSettingsDeprecationIndexing,
                > = None;
                let mut seen_deprecation_indexing = false;
                let mut f_election: Option<
                    crate::types::nodes_info::NodeInfoSettingsClusterElection,
                > = None;
                let mut seen_election = false;
                let mut f_initial_cluster_manager_nodes: Option<
                    crate::types::common::StringOrStringArray,
                > = None;
                let mut seen_initial_cluster_manager_nodes = false;
                let mut f_initial_master_nodes: Option<crate::types::common::StringOrStringArray> =
                    None;
                let mut seen_initial_master_nodes = false;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut f_routing: Option<crate::types::indices_common::IndexRouting> = None;
                let mut seen_routing = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "deprecation_indexing" => {
                            if seen_deprecation_indexing {
                                return Err(serde::de::Error::duplicate_field(
                                    "deprecation_indexing",
                                ));
                            }
                            seen_deprecation_indexing = true;
                            f_deprecation_indexing = map.next_value::<Option<
                                crate::types::nodes_info::NodeInfoSettingsDeprecationIndexing,
                            >>()?;
                        }
                        "election" => {
                            if seen_election {
                                return Err(serde::de::Error::duplicate_field("election"));
                            }
                            seen_election = true;
                            f_election =
                                map.next_value::<Option<
                                    crate::types::nodes_info::NodeInfoSettingsClusterElection,
                                >>()?;
                        }
                        "initial_cluster_manager_nodes" => {
                            if seen_initial_cluster_manager_nodes {
                                return Err(serde::de::Error::duplicate_field(
                                    "initial_cluster_manager_nodes",
                                ));
                            }
                            seen_initial_cluster_manager_nodes = true;
                            f_initial_cluster_manager_nodes = map
                                .next_value::<Option<crate::types::common::StringOrStringArray>>(
                                )?;
                        }
                        "initial_master_nodes" => {
                            if seen_initial_master_nodes {
                                return Err(serde::de::Error::duplicate_field(
                                    "initial_master_nodes",
                                ));
                            }
                            seen_initial_master_nodes = true;
                            f_initial_master_nodes = map
                                .next_value::<Option<crate::types::common::StringOrStringArray>>(
                                )?;
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        "routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("routing"));
                            }
                            seen_routing = true;
                            f_routing = map
                                .next_value::<Option<crate::types::indices_common::IndexRouting>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoSettingsCluster {
                    deprecation_indexing: f_deprecation_indexing,
                    election: f_election,
                    initial_cluster_manager_nodes: f_initial_cluster_manager_nodes,
                    initial_master_nodes: f_initial_master_nodes,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    routing: f_routing,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoSettingsCluster {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.deprecation_indexing {
            map.serialize_entry("deprecation_indexing", v)?;
        }
        if let Some(v) = &self.election {
            map.serialize_entry("election", v)?;
        }
        if let Some(v) = &self.initial_cluster_manager_nodes {
            map.serialize_entry("initial_cluster_manager_nodes", v)?;
        }
        if let Some(v) = &self.initial_master_nodes {
            map.serialize_entry("initial_master_nodes", v)?;
        }
        map.serialize_entry("name", &self.name)?;
        if let Some(v) = &self.routing {
            map.serialize_entry("routing", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The cluster election strategy configuration."]
#[derive(Debug, Clone)]
pub struct NodeInfoSettingsClusterElection {
    pub strategy: crate::types::common::Name,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoSettingsClusterElection {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoSettingsClusterElection;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoSettingsClusterElection object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_strategy: Option<crate::types::common::Name> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "strategy" => {
                            if f_strategy.is_some() {
                                return Err(serde::de::Error::duplicate_field("strategy"));
                            }
                            f_strategy = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoSettingsClusterElection {
                    strategy: f_strategy
                        .ok_or_else(|| serde::de::Error::missing_field("strategy"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoSettingsClusterElection {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("strategy", &self.strategy)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Controls deprecation notice indexing behavior."]
#[derive(Debug, Clone)]
pub struct NodeInfoSettingsDeprecationIndexing {
    pub enabled: crate::types::common::StringifiedBoolean,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoSettingsDeprecationIndexing {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoSettingsDeprecationIndexing;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoSettingsDeprecationIndexing object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_enabled: Option<crate::types::common::StringifiedBoolean> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "enabled" => {
                            if f_enabled.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            f_enabled =
                                Some(map.next_value::<crate::types::common::StringifiedBoolean>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoSettingsDeprecationIndexing {
                    enabled: f_enabled.ok_or_else(|| serde::de::Error::missing_field("enabled"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoSettingsDeprecationIndexing {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("enabled", &self.enabled)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The HTTP server configuration."]
#[derive(Debug, Clone)]
pub struct NodeInfoSettingsHttp {
    pub compression: Option<crate::types::common::StringifiedBoolean>,
    pub port: Option<crate::types::common::StringifiedInteger>,
    pub r#type: crate::types::nodes_info::NodeInfoSettingsHttpType,
    #[doc = "The default HTTP server type."]
    pub type_default: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoSettingsHttp {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoSettingsHttp;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoSettingsHttp object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_compression: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_compression = false;
                let mut f_port: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_port = false;
                let mut f_type: Option<crate::types::nodes_info::NodeInfoSettingsHttpType> = None;
                let mut f_type_default: Option<String> = None;
                let mut seen_type_default = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "compression" => {
                            if seen_compression {
                                return Err(serde::de::Error::duplicate_field("compression"));
                            }
                            seen_compression = true;
                            f_compression = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "port" => {
                            if seen_port {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            seen_port = true;
                            f_port = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::nodes_info::NodeInfoSettingsHttpType>()?);
                        }
                        "type.default" => {
                            if seen_type_default {
                                return Err(serde::de::Error::duplicate_field("type.default"));
                            }
                            seen_type_default = true;
                            f_type_default = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoSettingsHttp {
                    compression: f_compression,
                    port: f_port,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    type_default: f_type_default,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoSettingsHttp {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.compression {
            map.serialize_entry("compression", v)?;
        }
        if let Some(v) = &self.port {
            map.serialize_entry("port", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        if let Some(v) = &self.type_default {
            map.serialize_entry("type.default", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The HTTP server type configuration."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum NodeInfoSettingsHttpType {
    Type(String),
    Config(crate::types::nodes_info::NodeInfoSettingsHttpTypeConfig),
}
impl<'de> serde::Deserialize<'de> for NodeInfoSettingsHttpType {
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
            Some(b'"') => serde_json::from_str(text).map(NodeInfoSettingsHttpType::Type),
            Some(b'{') => serde_json::from_str(text).map(NodeInfoSettingsHttpType::Config),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a NodeInfoSettingsHttpType value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The HTTP server type configuration details."]
#[derive(Debug, Clone)]
pub struct NodeInfoSettingsHttpTypeConfig {
    #[doc = "The default HTTP server implementation."]
    pub default: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoSettingsHttpTypeConfig {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoSettingsHttpTypeConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoSettingsHttpTypeConfig object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_default: Option<String> = None;
                let mut seen_default = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "default" => {
                            if seen_default {
                                return Err(serde::de::Error::duplicate_field("default"));
                            }
                            seen_default = true;
                            f_default = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoSettingsHttpTypeConfig {
                    default: f_default,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoSettingsHttpTypeConfig {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.default {
            map.serialize_entry("default", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The index-level settings configuration."]
#[derive(Debug, Clone)]
pub struct NodeInfoSettingsIndex {
    pub store: Option<crate::types::nodes_info::NodeInfoSettingsIndexStore>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoSettingsIndex {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoSettingsIndex;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoSettingsIndex object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_store: Option<crate::types::nodes_info::NodeInfoSettingsIndexStore> =
                    None;
                let mut seen_store = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<crate::types::nodes_info::NodeInfoSettingsIndexStore>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoSettingsIndex {
                    store: f_store,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoSettingsIndex {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The hybrid store configuration."]
#[derive(Debug, Clone)]
pub struct NodeInfoSettingsIndexHybrid {
    pub mmap: Option<crate::types::nodes_info::NodeInfoSettingsIndexStoreMmap>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoSettingsIndexHybrid {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoSettingsIndexHybrid;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoSettingsIndexHybrid object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_mmap: Option<crate::types::nodes_info::NodeInfoSettingsIndexStoreMmap> =
                    None;
                let mut seen_mmap = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "mmap" => {
                            if seen_mmap {
                                return Err(serde::de::Error::duplicate_field("mmap"));
                            }
                            seen_mmap = true;
                            f_mmap =
                                map.next_value::<Option<
                                    crate::types::nodes_info::NodeInfoSettingsIndexStoreMmap,
                                >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoSettingsIndexHybrid {
                    mmap: f_mmap,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoSettingsIndexHybrid {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.mmap {
            map.serialize_entry("mmap", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The index store configuration."]
#[derive(Debug, Clone)]
pub struct NodeInfoSettingsIndexStore {
    pub hybrid: Option<crate::types::nodes_info::NodeInfoSettingsIndexHybrid>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoSettingsIndexStore {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoSettingsIndexStore;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoSettingsIndexStore object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_hybrid: Option<crate::types::nodes_info::NodeInfoSettingsIndexHybrid> =
                    None;
                let mut seen_hybrid = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "hybrid" => {
                            if seen_hybrid {
                                return Err(serde::de::Error::duplicate_field("hybrid"));
                            }
                            seen_hybrid = true;
                            f_hybrid = map.next_value::<Option<crate::types::nodes_info::NodeInfoSettingsIndexHybrid>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoSettingsIndexStore {
                    hybrid: f_hybrid,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoSettingsIndexStore {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.hybrid {
            map.serialize_entry("hybrid", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The memory-mapped store configuration."]
#[derive(Debug, Clone)]
pub struct NodeInfoSettingsIndexStoreMmap {
    #[doc = "The file extensions configured for memory mapping."]
    pub extensions: Option<Vec<String>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoSettingsIndexStoreMmap {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoSettingsIndexStoreMmap;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoSettingsIndexStoreMmap object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_extensions: Option<Vec<String>> = None;
                let mut seen_extensions = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "extensions" => {
                            if seen_extensions {
                                return Err(serde::de::Error::duplicate_field("extensions"));
                            }
                            seen_extensions = true;
                            f_extensions = map.next_value::<Option<Vec<String>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoSettingsIndexStoreMmap {
                    extensions: f_extensions,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoSettingsIndexStoreMmap {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.extensions {
            map.serialize_entry("extensions", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The ingest processor configurations."]
#[derive(Debug, Clone)]
pub struct NodeInfoSettingsIngest {
    pub append: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub attachment: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub bytes: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub circle: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub convert: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub csv: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub date: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub date_index_name: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub dissect: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub dot_expander: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub drop: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub enrich: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub fail: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub foreach: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub geoip: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub grok: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub gsub: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub inference: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub join: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub json: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub kv: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub lowercase: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub pipeline: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub remove: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub rename: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub script: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub set: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub set_security_user: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub sort: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub split: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub trim: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub uppercase: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub urldecode: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    pub user_agent: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoSettingsIngest {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoSettingsIngest;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoSettingsIngest object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_append: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_append = false;
                let mut f_attachment: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_attachment = false;
                let mut f_bytes: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_bytes = false;
                let mut f_circle: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_circle = false;
                let mut f_convert: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_convert = false;
                let mut f_csv: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_csv = false;
                let mut f_date: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_date = false;
                let mut f_date_index_name: Option<crate::types::nodes_info::NodeInfoIngestInfo> =
                    None;
                let mut seen_date_index_name = false;
                let mut f_dissect: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_dissect = false;
                let mut f_dot_expander: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_dot_expander = false;
                let mut f_drop: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_drop = false;
                let mut f_enrich: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_enrich = false;
                let mut f_fail: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_fail = false;
                let mut f_foreach: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_foreach = false;
                let mut f_geoip: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_geoip = false;
                let mut f_grok: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_grok = false;
                let mut f_gsub: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_gsub = false;
                let mut f_inference: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_inference = false;
                let mut f_join: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_join = false;
                let mut f_json: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_json = false;
                let mut f_kv: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_kv = false;
                let mut f_lowercase: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_lowercase = false;
                let mut f_pipeline: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_pipeline = false;
                let mut f_remove: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_remove = false;
                let mut f_rename: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_rename = false;
                let mut f_script: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_script = false;
                let mut f_set: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_set = false;
                let mut f_set_security_user: Option<crate::types::nodes_info::NodeInfoIngestInfo> =
                    None;
                let mut seen_set_security_user = false;
                let mut f_sort: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_sort = false;
                let mut f_split: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_split = false;
                let mut f_trim: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_trim = false;
                let mut f_uppercase: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_uppercase = false;
                let mut f_urldecode: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_urldecode = false;
                let mut f_user_agent: Option<crate::types::nodes_info::NodeInfoIngestInfo> = None;
                let mut seen_user_agent = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "append" => {
                            if seen_append {
                                return Err(serde::de::Error::duplicate_field("append"));
                            }
                            seen_append = true;
                            f_append = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "attachment" => {
                            if seen_attachment {
                                return Err(serde::de::Error::duplicate_field("attachment"));
                            }
                            seen_attachment = true;
                            f_attachment = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "bytes" => {
                            if seen_bytes {
                                return Err(serde::de::Error::duplicate_field("bytes"));
                            }
                            seen_bytes = true;
                            f_bytes = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "circle" => {
                            if seen_circle {
                                return Err(serde::de::Error::duplicate_field("circle"));
                            }
                            seen_circle = true;
                            f_circle = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "convert" => {
                            if seen_convert {
                                return Err(serde::de::Error::duplicate_field("convert"));
                            }
                            seen_convert = true;
                            f_convert = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "csv" => {
                            if seen_csv {
                                return Err(serde::de::Error::duplicate_field("csv"));
                            }
                            seen_csv = true;
                            f_csv = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "date" => {
                            if seen_date {
                                return Err(serde::de::Error::duplicate_field("date"));
                            }
                            seen_date = true;
                            f_date = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "date_index_name" => {
                            if seen_date_index_name {
                                return Err(serde::de::Error::duplicate_field("date_index_name"));
                            }
                            seen_date_index_name = true;
                            f_date_index_name = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "dissect" => {
                            if seen_dissect {
                                return Err(serde::de::Error::duplicate_field("dissect"));
                            }
                            seen_dissect = true;
                            f_dissect = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "dot_expander" => {
                            if seen_dot_expander {
                                return Err(serde::de::Error::duplicate_field("dot_expander"));
                            }
                            seen_dot_expander = true;
                            f_dot_expander = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "drop" => {
                            if seen_drop {
                                return Err(serde::de::Error::duplicate_field("drop"));
                            }
                            seen_drop = true;
                            f_drop = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "enrich" => {
                            if seen_enrich {
                                return Err(serde::de::Error::duplicate_field("enrich"));
                            }
                            seen_enrich = true;
                            f_enrich = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "fail" => {
                            if seen_fail {
                                return Err(serde::de::Error::duplicate_field("fail"));
                            }
                            seen_fail = true;
                            f_fail = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "foreach" => {
                            if seen_foreach {
                                return Err(serde::de::Error::duplicate_field("foreach"));
                            }
                            seen_foreach = true;
                            f_foreach = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "geoip" => {
                            if seen_geoip {
                                return Err(serde::de::Error::duplicate_field("geoip"));
                            }
                            seen_geoip = true;
                            f_geoip = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "grok" => {
                            if seen_grok {
                                return Err(serde::de::Error::duplicate_field("grok"));
                            }
                            seen_grok = true;
                            f_grok = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "gsub" => {
                            if seen_gsub {
                                return Err(serde::de::Error::duplicate_field("gsub"));
                            }
                            seen_gsub = true;
                            f_gsub = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "inference" => {
                            if seen_inference {
                                return Err(serde::de::Error::duplicate_field("inference"));
                            }
                            seen_inference = true;
                            f_inference = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "join" => {
                            if seen_join {
                                return Err(serde::de::Error::duplicate_field("join"));
                            }
                            seen_join = true;
                            f_join = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "json" => {
                            if seen_json {
                                return Err(serde::de::Error::duplicate_field("json"));
                            }
                            seen_json = true;
                            f_json = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "kv" => {
                            if seen_kv {
                                return Err(serde::de::Error::duplicate_field("kv"));
                            }
                            seen_kv = true;
                            f_kv = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "lowercase" => {
                            if seen_lowercase {
                                return Err(serde::de::Error::duplicate_field("lowercase"));
                            }
                            seen_lowercase = true;
                            f_lowercase = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "pipeline" => {
                            if seen_pipeline {
                                return Err(serde::de::Error::duplicate_field("pipeline"));
                            }
                            seen_pipeline = true;
                            f_pipeline = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "remove" => {
                            if seen_remove {
                                return Err(serde::de::Error::duplicate_field("remove"));
                            }
                            seen_remove = true;
                            f_remove = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "rename" => {
                            if seen_rename {
                                return Err(serde::de::Error::duplicate_field("rename"));
                            }
                            seen_rename = true;
                            f_rename = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "script" => {
                            if seen_script {
                                return Err(serde::de::Error::duplicate_field("script"));
                            }
                            seen_script = true;
                            f_script = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "set" => {
                            if seen_set {
                                return Err(serde::de::Error::duplicate_field("set"));
                            }
                            seen_set = true;
                            f_set = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "set_security_user" => {
                            if seen_set_security_user {
                                return Err(serde::de::Error::duplicate_field("set_security_user"));
                            }
                            seen_set_security_user = true;
                            f_set_security_user = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "sort" => {
                            if seen_sort {
                                return Err(serde::de::Error::duplicate_field("sort"));
                            }
                            seen_sort = true;
                            f_sort = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "split" => {
                            if seen_split {
                                return Err(serde::de::Error::duplicate_field("split"));
                            }
                            seen_split = true;
                            f_split = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "trim" => {
                            if seen_trim {
                                return Err(serde::de::Error::duplicate_field("trim"));
                            }
                            seen_trim = true;
                            f_trim = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "uppercase" => {
                            if seen_uppercase {
                                return Err(serde::de::Error::duplicate_field("uppercase"));
                            }
                            seen_uppercase = true;
                            f_uppercase = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "urldecode" => {
                            if seen_urldecode {
                                return Err(serde::de::Error::duplicate_field("urldecode"));
                            }
                            seen_urldecode = true;
                            f_urldecode = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        "user_agent" => {
                            if seen_user_agent {
                                return Err(serde::de::Error::duplicate_field("user_agent"));
                            }
                            seen_user_agent = true;
                            f_user_agent = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoIngestInfo>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoSettingsIngest {
                    append: f_append,
                    attachment: f_attachment,
                    bytes: f_bytes,
                    circle: f_circle,
                    convert: f_convert,
                    csv: f_csv,
                    date: f_date,
                    date_index_name: f_date_index_name,
                    dissect: f_dissect,
                    dot_expander: f_dot_expander,
                    drop: f_drop,
                    enrich: f_enrich,
                    fail: f_fail,
                    foreach: f_foreach,
                    geoip: f_geoip,
                    grok: f_grok,
                    gsub: f_gsub,
                    inference: f_inference,
                    join: f_join,
                    json: f_json,
                    kv: f_kv,
                    lowercase: f_lowercase,
                    pipeline: f_pipeline,
                    remove: f_remove,
                    rename: f_rename,
                    script: f_script,
                    set: f_set,
                    set_security_user: f_set_security_user,
                    sort: f_sort,
                    split: f_split,
                    trim: f_trim,
                    uppercase: f_uppercase,
                    urldecode: f_urldecode,
                    user_agent: f_user_agent,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoSettingsIngest {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.append {
            map.serialize_entry("append", v)?;
        }
        if let Some(v) = &self.attachment {
            map.serialize_entry("attachment", v)?;
        }
        if let Some(v) = &self.bytes {
            map.serialize_entry("bytes", v)?;
        }
        if let Some(v) = &self.circle {
            map.serialize_entry("circle", v)?;
        }
        if let Some(v) = &self.convert {
            map.serialize_entry("convert", v)?;
        }
        if let Some(v) = &self.csv {
            map.serialize_entry("csv", v)?;
        }
        if let Some(v) = &self.date {
            map.serialize_entry("date", v)?;
        }
        if let Some(v) = &self.date_index_name {
            map.serialize_entry("date_index_name", v)?;
        }
        if let Some(v) = &self.dissect {
            map.serialize_entry("dissect", v)?;
        }
        if let Some(v) = &self.dot_expander {
            map.serialize_entry("dot_expander", v)?;
        }
        if let Some(v) = &self.drop {
            map.serialize_entry("drop", v)?;
        }
        if let Some(v) = &self.enrich {
            map.serialize_entry("enrich", v)?;
        }
        if let Some(v) = &self.fail {
            map.serialize_entry("fail", v)?;
        }
        if let Some(v) = &self.foreach {
            map.serialize_entry("foreach", v)?;
        }
        if let Some(v) = &self.geoip {
            map.serialize_entry("geoip", v)?;
        }
        if let Some(v) = &self.grok {
            map.serialize_entry("grok", v)?;
        }
        if let Some(v) = &self.gsub {
            map.serialize_entry("gsub", v)?;
        }
        if let Some(v) = &self.inference {
            map.serialize_entry("inference", v)?;
        }
        if let Some(v) = &self.join {
            map.serialize_entry("join", v)?;
        }
        if let Some(v) = &self.json {
            map.serialize_entry("json", v)?;
        }
        if let Some(v) = &self.kv {
            map.serialize_entry("kv", v)?;
        }
        if let Some(v) = &self.lowercase {
            map.serialize_entry("lowercase", v)?;
        }
        if let Some(v) = &self.pipeline {
            map.serialize_entry("pipeline", v)?;
        }
        if let Some(v) = &self.remove {
            map.serialize_entry("remove", v)?;
        }
        if let Some(v) = &self.rename {
            map.serialize_entry("rename", v)?;
        }
        if let Some(v) = &self.script {
            map.serialize_entry("script", v)?;
        }
        if let Some(v) = &self.set {
            map.serialize_entry("set", v)?;
        }
        if let Some(v) = &self.set_security_user {
            map.serialize_entry("set_security_user", v)?;
        }
        if let Some(v) = &self.sort {
            map.serialize_entry("sort", v)?;
        }
        if let Some(v) = &self.split {
            map.serialize_entry("split", v)?;
        }
        if let Some(v) = &self.trim {
            map.serialize_entry("trim", v)?;
        }
        if let Some(v) = &self.uppercase {
            map.serialize_entry("uppercase", v)?;
        }
        if let Some(v) = &self.urldecode {
            map.serialize_entry("urldecode", v)?;
        }
        if let Some(v) = &self.user_agent {
            map.serialize_entry("user_agent", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The network configuration settings."]
#[derive(Debug, Clone)]
pub struct NodeInfoSettingsNetwork {
    pub host: crate::types::common::Host,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoSettingsNetwork {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoSettingsNetwork;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoSettingsNetwork object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_host: Option<crate::types::common::Host> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "host" => {
                            if f_host.is_some() {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            f_host = Some(map.next_value::<crate::types::common::Host>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoSettingsNetwork {
                    host: f_host.ok_or_else(|| serde::de::Error::missing_field("host"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoSettingsNetwork {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("host", &self.host)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The node-specific configuration settings."]
#[derive(Debug, Clone)]
pub struct NodeInfoSettingsNode {
    #[doc = "The custom attributes assigned to the node."]
    pub attr: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The maximum number of nodes allowed to store data locally"]
    pub max_local_storage_nodes: Option<String>,
    pub name: crate::types::common::Name,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoSettingsNode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoSettingsNode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoSettingsNode object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_attr: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_attr = false;
                let mut f_max_local_storage_nodes: Option<String> = None;
                let mut seen_max_local_storage_nodes = false;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "attr" => {
                            if seen_attr {
                                return Err(serde::de::Error::duplicate_field("attr"));
                            }
                            seen_attr = true;
                            f_attr = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "max_local_storage_nodes" => {
                            if seen_max_local_storage_nodes {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_local_storage_nodes",
                                ));
                            }
                            seen_max_local_storage_nodes = true;
                            f_max_local_storage_nodes = map.next_value::<Option<String>>()?;
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoSettingsNode {
                    attr: f_attr,
                    max_local_storage_nodes: f_max_local_storage_nodes,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoSettingsNode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.attr {
            map.serialize_entry("attr", v)?;
        }
        if let Some(v) = &self.max_local_storage_nodes {
            map.serialize_entry("max_local_storage_nodes", v)?;
        }
        map.serialize_entry("name", &self.name)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The plugin-specific settings."]
pub type NodeInfoSettingsPlugins =
    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

#[doc = "The transport layer configuration settings."]
#[derive(Debug, Clone)]
pub struct NodeInfoSettingsTransport {
    pub r#type: crate::types::nodes_info::NodeInfoSettingsTransportType,
    #[doc = "The default transport type."]
    pub type_default: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoSettingsTransport {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoSettingsTransport;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoSettingsTransport object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_type: Option<crate::types::nodes_info::NodeInfoSettingsTransportType> =
                    None;
                let mut f_type_default: Option<String> = None;
                let mut seen_type_default = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<crate::types::nodes_info::NodeInfoSettingsTransportType>()?);
                        }
                        "type.default" => {
                            if seen_type_default {
                                return Err(serde::de::Error::duplicate_field("type.default"));
                            }
                            seen_type_default = true;
                            f_type_default = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoSettingsTransport {
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    type_default: f_type_default,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoSettingsTransport {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("type", &self.r#type)?;
        if let Some(v) = &self.type_default {
            map.serialize_entry("type.default", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The transport layer type configuration."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum NodeInfoSettingsTransportType {
    Type(String),
    Config(crate::types::nodes_info::NodeInfoSettingsTransportTypeConfig),
}
impl<'de> serde::Deserialize<'de> for NodeInfoSettingsTransportType {
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
            Some(b'"') => serde_json::from_str(text).map(NodeInfoSettingsTransportType::Type),
            Some(b'{') => serde_json::from_str(text).map(NodeInfoSettingsTransportType::Config),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a NodeInfoSettingsTransportType value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The detailed transport type configuration."]
#[derive(Debug, Clone)]
pub struct NodeInfoSettingsTransportTypeConfig {
    #[doc = "The default transport implementation."]
    pub default: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoSettingsTransportTypeConfig {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoSettingsTransportTypeConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoSettingsTransportTypeConfig object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_default: Option<String> = None;
                let mut seen_default = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "default" => {
                            if seen_default {
                                return Err(serde::de::Error::duplicate_field("default"));
                            }
                            seen_default = true;
                            f_default = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoSettingsTransportTypeConfig {
                    default: f_default,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoSettingsTransportTypeConfig {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.default {
            map.serialize_entry("default", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Total heap allowed to be used to hold recently indexed documents before they must be written to disk. This size is a shared pool across all shards on this node, and is controlled by Indexing Buffer settings."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum NodeInfoTotalIndexingBuffer {
    ByteCount(crate::types::common::ByteCount),
    HumanReadableByteCount(crate::types::common::HumanReadableByteCount),
}
impl<'de> serde::Deserialize<'de> for NodeInfoTotalIndexingBuffer {
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
                serde_json::from_str(text).map(NodeInfoTotalIndexingBuffer::ByteCount)
            }
            Some(b'"') => {
                serde_json::from_str(text).map(NodeInfoTotalIndexingBuffer::HumanReadableByteCount)
            }
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a NodeInfoTotalIndexingBuffer value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum NodeInfoTotalIndexingBufferInBytes {
    HumanReadableByteCount(crate::types::common::HumanReadableByteCount),
    ByteCount(crate::types::common::ByteCount),
}
impl<'de> serde::Deserialize<'de> for NodeInfoTotalIndexingBufferInBytes {
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
            Some(b'"') => serde_json::from_str(text)
                .map(NodeInfoTotalIndexingBufferInBytes::HumanReadableByteCount),
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(NodeInfoTotalIndexingBufferInBytes::ByteCount)
            }
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a NodeInfoTotalIndexingBufferInBytes value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The transport layer information."]
#[derive(Debug, Clone)]
pub struct NodeInfoTransport {
    #[doc = "The addresses the node is bound to for transport traffic."]
    pub bound_address: Vec<String>,
    #[doc = "The configured transport profiles."]
    pub profiles: std::collections::BTreeMap<String, String>,
    #[doc = "The published transport address."]
    pub publish_address: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeInfoTransport {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeInfoTransport;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeInfoTransport object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_bound_address: Option<Vec<String>> = None;
                let mut f_profiles: Option<std::collections::BTreeMap<String, String>> = None;
                let mut f_publish_address: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "bound_address" => {
                            if f_bound_address.is_some() {
                                return Err(serde::de::Error::duplicate_field("bound_address"));
                            }
                            f_bound_address = Some(map.next_value::<Vec<String>>()?);
                        }
                        "profiles" => {
                            if f_profiles.is_some() {
                                return Err(serde::de::Error::duplicate_field("profiles"));
                            }
                            f_profiles = Some(
                                map.next_value::<std::collections::BTreeMap<String, String>>()?,
                            );
                        }
                        "publish_address" => {
                            if f_publish_address.is_some() {
                                return Err(serde::de::Error::duplicate_field("publish_address"));
                            }
                            f_publish_address = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeInfoTransport {
                    bound_address: f_bound_address
                        .ok_or_else(|| serde::de::Error::missing_field("bound_address"))?,
                    profiles: f_profiles
                        .ok_or_else(|| serde::de::Error::missing_field("profiles"))?,
                    publish_address: f_publish_address
                        .ok_or_else(|| serde::de::Error::missing_field("publish_address"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeInfoTransport {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("bound_address", &self.bound_address)?;
        map.serialize_entry("profiles", &self.profiles)?;
        map.serialize_entry("publish_address", &self.publish_address)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Provides JVM-specific information for the node."]
#[derive(Debug, Clone)]
pub struct NodeJvmInfo {
    #[doc = "Whether the JDK is bundled with OpenSearch."]
    pub bundled_jdk: bool,
    #[doc = "The garbage collectors enabled in the JVM."]
    pub gc_collectors: Option<Vec<String>>,
    #[doc = "The command line arguments passed to the JVM."]
    pub input_arguments: Option<Vec<String>>,
    pub mem: crate::types::nodes_info::NodeInfoJvmMemory,
    #[doc = "The memory pools available to the JVM."]
    pub memory_pools: Option<Vec<String>>,
    #[doc = "The process identifier of the JVM."]
    pub pid: i32,
    pub start_time: Option<crate::types::common::DateTime>,
    pub start_time_in_millis: crate::types::common::EpochTimeUnitMillis,
    #[doc = "Whether the node is using the bundled JDK."]
    pub using_bundled_jdk: Option<bool>,
    pub using_compressed_ordinary_object_pointers: Option<crate::types::common::StringifiedBoolean>,
    pub version: Option<crate::types::common::VersionString>,
    pub vm_name: Option<crate::types::common::Name>,
    #[doc = "The vendor of the JVM."]
    pub vm_vendor: Option<String>,
    pub vm_version: Option<crate::types::common::VersionString>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeJvmInfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeJvmInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeJvmInfo object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_bundled_jdk: Option<bool> = None;
                let mut f_gc_collectors: Option<Vec<String>> = None;
                let mut seen_gc_collectors = false;
                let mut f_input_arguments: Option<Vec<String>> = None;
                let mut seen_input_arguments = false;
                let mut f_mem: Option<crate::types::nodes_info::NodeInfoJvmMemory> = None;
                let mut f_memory_pools: Option<Vec<String>> = None;
                let mut seen_memory_pools = false;
                let mut f_pid: Option<i32> = None;
                let mut f_start_time: Option<crate::types::common::DateTime> = None;
                let mut seen_start_time = false;
                let mut f_start_time_in_millis: Option<crate::types::common::EpochTimeUnitMillis> =
                    None;
                let mut f_using_bundled_jdk: Option<bool> = None;
                let mut seen_using_bundled_jdk = false;
                let mut f_using_compressed_ordinary_object_pointers: Option<
                    crate::types::common::StringifiedBoolean,
                > = None;
                let mut seen_using_compressed_ordinary_object_pointers = false;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut f_vm_name: Option<crate::types::common::Name> = None;
                let mut seen_vm_name = false;
                let mut f_vm_vendor: Option<String> = None;
                let mut seen_vm_vendor = false;
                let mut f_vm_version: Option<crate::types::common::VersionString> = None;
                let mut seen_vm_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "bundled_jdk" => {
                            if f_bundled_jdk.is_some() {
                                return Err(serde::de::Error::duplicate_field("bundled_jdk"));
                            }
                            f_bundled_jdk = Some(map.next_value::<bool>()?);
                        }
                        "gc_collectors" => {
                            if seen_gc_collectors {
                                return Err(serde::de::Error::duplicate_field("gc_collectors"));
                            }
                            seen_gc_collectors = true;
                            f_gc_collectors = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "input_arguments" => {
                            if seen_input_arguments {
                                return Err(serde::de::Error::duplicate_field("input_arguments"));
                            }
                            seen_input_arguments = true;
                            f_input_arguments = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "mem" => {
                            if f_mem.is_some() {
                                return Err(serde::de::Error::duplicate_field("mem"));
                            }
                            f_mem = Some(
                                map.next_value::<crate::types::nodes_info::NodeInfoJvmMemory>()?,
                            );
                        }
                        "memory_pools" => {
                            if seen_memory_pools {
                                return Err(serde::de::Error::duplicate_field("memory_pools"));
                            }
                            seen_memory_pools = true;
                            f_memory_pools = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "pid" => {
                            if f_pid.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            f_pid = Some(map.next_value::<i32>()?);
                        }
                        "start_time" => {
                            if seen_start_time {
                                return Err(serde::de::Error::duplicate_field("start_time"));
                            }
                            seen_start_time = true;
                            f_start_time =
                                map.next_value::<Option<crate::types::common::DateTime>>()?;
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
                        "using_bundled_jdk" => {
                            if seen_using_bundled_jdk {
                                return Err(serde::de::Error::duplicate_field("using_bundled_jdk"));
                            }
                            seen_using_bundled_jdk = true;
                            f_using_bundled_jdk = map.next_value::<Option<bool>>()?;
                        }
                        "using_compressed_ordinary_object_pointers" => {
                            if seen_using_compressed_ordinary_object_pointers {
                                return Err(serde::de::Error::duplicate_field(
                                    "using_compressed_ordinary_object_pointers",
                                ));
                            }
                            seen_using_compressed_ordinary_object_pointers = true;
                            f_using_compressed_ordinary_object_pointers = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        "vm_name" => {
                            if seen_vm_name {
                                return Err(serde::de::Error::duplicate_field("vm_name"));
                            }
                            seen_vm_name = true;
                            f_vm_name = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "vm_vendor" => {
                            if seen_vm_vendor {
                                return Err(serde::de::Error::duplicate_field("vm_vendor"));
                            }
                            seen_vm_vendor = true;
                            f_vm_vendor = map.next_value::<Option<String>>()?;
                        }
                        "vm_version" => {
                            if seen_vm_version {
                                return Err(serde::de::Error::duplicate_field("vm_version"));
                            }
                            seen_vm_version = true;
                            f_vm_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeJvmInfo {
                    bundled_jdk: f_bundled_jdk
                        .ok_or_else(|| serde::de::Error::missing_field("bundled_jdk"))?,
                    gc_collectors: f_gc_collectors,
                    input_arguments: f_input_arguments,
                    mem: f_mem.ok_or_else(|| serde::de::Error::missing_field("mem"))?,
                    memory_pools: f_memory_pools,
                    pid: f_pid.ok_or_else(|| serde::de::Error::missing_field("pid"))?,
                    start_time: f_start_time,
                    start_time_in_millis: f_start_time_in_millis
                        .ok_or_else(|| serde::de::Error::missing_field("start_time_in_millis"))?,
                    using_bundled_jdk: f_using_bundled_jdk,
                    using_compressed_ordinary_object_pointers:
                        f_using_compressed_ordinary_object_pointers,
                    version: f_version,
                    vm_name: f_vm_name,
                    vm_vendor: f_vm_vendor,
                    vm_version: f_vm_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeJvmInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("bundled_jdk", &self.bundled_jdk)?;
        if let Some(v) = &self.gc_collectors {
            map.serialize_entry("gc_collectors", v)?;
        }
        if let Some(v) = &self.input_arguments {
            map.serialize_entry("input_arguments", v)?;
        }
        map.serialize_entry("mem", &self.mem)?;
        if let Some(v) = &self.memory_pools {
            map.serialize_entry("memory_pools", v)?;
        }
        map.serialize_entry("pid", &self.pid)?;
        if let Some(v) = &self.start_time {
            map.serialize_entry("start_time", v)?;
        }
        map.serialize_entry("start_time_in_millis", &self.start_time_in_millis)?;
        if let Some(v) = &self.using_bundled_jdk {
            map.serialize_entry("using_bundled_jdk", v)?;
        }
        if let Some(v) = &self.using_compressed_ordinary_object_pointers {
            map.serialize_entry("using_compressed_ordinary_object_pointers", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        if let Some(v) = &self.vm_name {
            map.serialize_entry("vm_name", v)?;
        }
        if let Some(v) = &self.vm_vendor {
            map.serialize_entry("vm_vendor", v)?;
        }
        if let Some(v) = &self.vm_version {
            map.serialize_entry("vm_version", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Provides operating system information for the node."]
#[derive(Debug, Clone)]
pub struct NodeOperatingSystemInfo {
    #[doc = "The number of processors actually used to calculate thread pool size. This number can be set with the `node.processors` setting of a node and defaults to the number of processors reported by the OS."]
    pub allocated_processors: Option<i32>,
    #[doc = "Name of the JVM architecture, for example `amd64` or `x86`."]
    pub arch: Option<String>,
    #[doc = "The number of processors available to the Java virtual machine."]
    pub available_processors: i32,
    pub cpu: Option<crate::types::nodes_info::NodeInfoOSCPU>,
    pub mem: Option<crate::types::nodes_info::NodeInfoMemory>,
    pub name: Option<crate::types::common::Name>,
    pub pretty_name: Option<crate::types::common::Name>,
    pub refresh_interval: Option<crate::types::common::Duration>,
    pub refresh_interval_in_millis: crate::types::common::DurationValueUnitMillis,
    pub swap: Option<crate::types::nodes_info::NodeInfoMemory>,
    pub version: Option<crate::types::common::VersionString>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeOperatingSystemInfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeOperatingSystemInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeOperatingSystemInfo object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_allocated_processors: Option<i32> = None;
                let mut seen_allocated_processors = false;
                let mut f_arch: Option<String> = None;
                let mut seen_arch = false;
                let mut f_available_processors: Option<i32> = None;
                let mut f_cpu: Option<crate::types::nodes_info::NodeInfoOSCPU> = None;
                let mut seen_cpu = false;
                let mut f_mem: Option<crate::types::nodes_info::NodeInfoMemory> = None;
                let mut seen_mem = false;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut seen_name = false;
                let mut f_pretty_name: Option<crate::types::common::Name> = None;
                let mut seen_pretty_name = false;
                let mut f_refresh_interval: Option<crate::types::common::Duration> = None;
                let mut seen_refresh_interval = false;
                let mut f_refresh_interval_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut f_swap: Option<crate::types::nodes_info::NodeInfoMemory> = None;
                let mut seen_swap = false;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "allocated_processors" => {
                            if seen_allocated_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "allocated_processors",
                                ));
                            }
                            seen_allocated_processors = true;
                            f_allocated_processors = map.next_value::<Option<i32>>()?;
                        }
                        "arch" => {
                            if seen_arch {
                                return Err(serde::de::Error::duplicate_field("arch"));
                            }
                            seen_arch = true;
                            f_arch = map.next_value::<Option<String>>()?;
                        }
                        "available_processors" => {
                            if f_available_processors.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "available_processors",
                                ));
                            }
                            f_available_processors = Some(map.next_value::<i32>()?);
                        }
                        "cpu" => {
                            if seen_cpu {
                                return Err(serde::de::Error::duplicate_field("cpu"));
                            }
                            seen_cpu = true;
                            f_cpu = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoOSCPU>>()?;
                        }
                        "mem" => {
                            if seen_mem {
                                return Err(serde::de::Error::duplicate_field("mem"));
                            }
                            seen_mem = true;
                            f_mem = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoMemory>>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "pretty_name" => {
                            if seen_pretty_name {
                                return Err(serde::de::Error::duplicate_field("pretty_name"));
                            }
                            seen_pretty_name = true;
                            f_pretty_name =
                                map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "refresh_interval" => {
                            if seen_refresh_interval {
                                return Err(serde::de::Error::duplicate_field("refresh_interval"));
                            }
                            seen_refresh_interval = true;
                            f_refresh_interval =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "refresh_interval_in_millis" => {
                            if f_refresh_interval_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "refresh_interval_in_millis",
                                ));
                            }
                            f_refresh_interval_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        "swap" => {
                            if seen_swap {
                                return Err(serde::de::Error::duplicate_field("swap"));
                            }
                            seen_swap = true;
                            f_swap = map
                                .next_value::<Option<crate::types::nodes_info::NodeInfoMemory>>()?;
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeOperatingSystemInfo {
                    allocated_processors: f_allocated_processors,
                    arch: f_arch,
                    available_processors: f_available_processors
                        .ok_or_else(|| serde::de::Error::missing_field("available_processors"))?,
                    cpu: f_cpu,
                    mem: f_mem,
                    name: f_name,
                    pretty_name: f_pretty_name,
                    refresh_interval: f_refresh_interval,
                    refresh_interval_in_millis: f_refresh_interval_in_millis.ok_or_else(|| {
                        serde::de::Error::missing_field("refresh_interval_in_millis")
                    })?,
                    swap: f_swap,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeOperatingSystemInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.allocated_processors {
            map.serialize_entry("allocated_processors", v)?;
        }
        if let Some(v) = &self.arch {
            map.serialize_entry("arch", v)?;
        }
        map.serialize_entry("available_processors", &self.available_processors)?;
        if let Some(v) = &self.cpu {
            map.serialize_entry("cpu", v)?;
        }
        if let Some(v) = &self.mem {
            map.serialize_entry("mem", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.pretty_name {
            map.serialize_entry("pretty_name", v)?;
        }
        if let Some(v) = &self.refresh_interval {
            map.serialize_entry("refresh_interval", v)?;
        }
        map.serialize_entry(
            "refresh_interval_in_millis",
            &self.refresh_interval_in_millis,
        )?;
        if let Some(v) = &self.swap {
            map.serialize_entry("swap", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Provides process-related information for the node."]
#[derive(Debug, Clone)]
pub struct NodeProcessInfo {
    #[doc = "The process identifier (PID)"]
    pub id: i64,
    #[doc = "Indicates if the process address space has been successfully locked in memory."]
    pub mlockall: bool,
    pub refresh_interval: Option<crate::types::common::Duration>,
    pub refresh_interval_in_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeProcessInfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeProcessInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeProcessInfo object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_id: Option<i64> = None;
                let mut f_mlockall: Option<bool> = None;
                let mut f_refresh_interval: Option<crate::types::common::Duration> = None;
                let mut seen_refresh_interval = false;
                let mut f_refresh_interval_in_millis: Option<
                    crate::types::common::DurationValueUnitMillis,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "id" => {
                            if f_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            f_id = Some(map.next_value::<i64>()?);
                        }
                        "mlockall" => {
                            if f_mlockall.is_some() {
                                return Err(serde::de::Error::duplicate_field("mlockall"));
                            }
                            f_mlockall = Some(map.next_value::<bool>()?);
                        }
                        "refresh_interval" => {
                            if seen_refresh_interval {
                                return Err(serde::de::Error::duplicate_field("refresh_interval"));
                            }
                            seen_refresh_interval = true;
                            f_refresh_interval =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "refresh_interval_in_millis" => {
                            if f_refresh_interval_in_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "refresh_interval_in_millis",
                                ));
                            }
                            f_refresh_interval_in_millis = Some(
                                map.next_value::<crate::types::common::DurationValueUnitMillis>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeProcessInfo {
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("id"))?,
                    mlockall: f_mlockall
                        .ok_or_else(|| serde::de::Error::missing_field("mlockall"))?,
                    refresh_interval: f_refresh_interval,
                    refresh_interval_in_millis: f_refresh_interval_in_millis.ok_or_else(|| {
                        serde::de::Error::missing_field("refresh_interval_in_millis")
                    })?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeProcessInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("mlockall", &self.mlockall)?;
        if let Some(v) = &self.refresh_interval {
            map.serialize_entry("refresh_interval", v)?;
        }
        map.serialize_entry(
            "refresh_interval_in_millis",
            &self.refresh_interval_in_millis,
        )?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The thread pool configuration details."]
#[derive(Debug, Clone)]
pub struct NodeThreadPoolInfo {
    #[doc = "The core number of threads."]
    pub core: Option<i32>,
    pub keep_alive: Option<crate::types::common::Duration>,
    #[doc = "The maximum number of threads."]
    pub max: Option<i32>,
    #[doc = "The size of the task queue."]
    pub queue_size: i32,
    #[doc = "The current size of the thread pool."]
    pub size: Option<i32>,
    #[doc = "The type of thread pool executor."]
    pub r#type: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeThreadPoolInfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeThreadPoolInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeThreadPoolInfo object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_core: Option<i32> = None;
                let mut seen_core = false;
                let mut f_keep_alive: Option<crate::types::common::Duration> = None;
                let mut seen_keep_alive = false;
                let mut f_max: Option<i32> = None;
                let mut seen_max = false;
                let mut f_queue_size: Option<i32> = None;
                let mut f_size: Option<i32> = None;
                let mut seen_size = false;
                let mut f_type: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "core" => {
                            if seen_core {
                                return Err(serde::de::Error::duplicate_field("core"));
                            }
                            seen_core = true;
                            f_core = map.next_value::<Option<i32>>()?;
                        }
                        "keep_alive" => {
                            if seen_keep_alive {
                                return Err(serde::de::Error::duplicate_field("keep_alive"));
                            }
                            seen_keep_alive = true;
                            f_keep_alive =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "max" => {
                            if seen_max {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            seen_max = true;
                            f_max = map.next_value::<Option<i32>>()?;
                        }
                        "queue_size" => {
                            if f_queue_size.is_some() {
                                return Err(serde::de::Error::duplicate_field("queue_size"));
                            }
                            f_queue_size = Some(map.next_value::<i32>()?);
                        }
                        "size" => {
                            if seen_size {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            seen_size = true;
                            f_size = map.next_value::<Option<i32>>()?;
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
                Ok(NodeThreadPoolInfo {
                    core: f_core,
                    keep_alive: f_keep_alive,
                    max: f_max,
                    queue_size: f_queue_size
                        .ok_or_else(|| serde::de::Error::missing_field("queue_size"))?,
                    size: f_size,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeThreadPoolInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.core {
            map.serialize_entry("core", v)?;
        }
        if let Some(v) = &self.keep_alive {
            map.serialize_entry("keep_alive", v)?;
        }
        if let Some(v) = &self.max {
            map.serialize_entry("max", v)?;
        }
        map.serialize_entry("queue_size", &self.queue_size)?;
        if let Some(v) = &self.size {
            map.serialize_entry("size", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
