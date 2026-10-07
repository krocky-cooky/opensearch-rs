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

//! Response types of the `cluster_remote_info` specification namespace

#![allow(missing_docs)]

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ClusterRemoteInfo {
    raw: Box<serde_json::value::RawValue>,
}
impl ClusterRemoteInfo {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `cluster_remote_sniff_info` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_cluster_remote_sniff_info(
        &self,
    ) -> std::result::Result<
        crate::types::cluster_remote_info::ClusterRemoteSniffInfo,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "cluster_remote_sniff_info")
    }
    #[doc = "Parses the value as the `cluster_remote_proxy_info` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_cluster_remote_proxy_info(
        &self,
    ) -> std::result::Result<
        crate::types::cluster_remote_info::ClusterRemoteProxyInfo,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "cluster_remote_proxy_info")
    }
}

#[derive(Debug, Clone)]
pub struct ClusterRemoteProxyInfo {
    pub connected: bool,
    pub initial_connect_timeout: crate::types::common::Duration,
    pub max_proxy_socket_connections: i32,
    pub mode: crate::types::cluster_remote_info::ClusterRemoteProxyInfoMode,
    pub num_proxy_sockets_connected: i32,
    pub proxy_address: String,
    pub server_name: String,
    pub skip_unavailable: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterRemoteProxyInfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterRemoteProxyInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterRemoteProxyInfo object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_connected: Option<bool> = None;
                let mut f_initial_connect_timeout: Option<crate::types::common::Duration> = None;
                let mut f_max_proxy_socket_connections: Option<i32> = None;
                let mut f_mode: Option<
                    crate::types::cluster_remote_info::ClusterRemoteProxyInfoMode,
                > = None;
                let mut f_num_proxy_sockets_connected: Option<i32> = None;
                let mut f_proxy_address: Option<String> = None;
                let mut f_server_name: Option<String> = None;
                let mut f_skip_unavailable: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "connected" => {
                            if f_connected.is_some() {
                                return Err(serde::de::Error::duplicate_field("connected"));
                            }
                            f_connected = Some(map.next_value::<bool>()?);
                        }
                        "initial_connect_timeout" => {
                            if f_initial_connect_timeout.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "initial_connect_timeout",
                                ));
                            }
                            f_initial_connect_timeout =
                                Some(map.next_value::<crate::types::common::Duration>()?);
                        }
                        "max_proxy_socket_connections" => {
                            if f_max_proxy_socket_connections.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_proxy_socket_connections",
                                ));
                            }
                            f_max_proxy_socket_connections = Some(map.next_value::<i32>()?);
                        }
                        "mode" => {
                            if f_mode.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            f_mode = Some(map.next_value::<crate::types::cluster_remote_info::ClusterRemoteProxyInfoMode>()?);
                        }
                        "num_proxy_sockets_connected" => {
                            if f_num_proxy_sockets_connected.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "num_proxy_sockets_connected",
                                ));
                            }
                            f_num_proxy_sockets_connected = Some(map.next_value::<i32>()?);
                        }
                        "proxy_address" => {
                            if f_proxy_address.is_some() {
                                return Err(serde::de::Error::duplicate_field("proxy_address"));
                            }
                            f_proxy_address = Some(map.next_value::<String>()?);
                        }
                        "server_name" => {
                            if f_server_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("server_name"));
                            }
                            f_server_name = Some(map.next_value::<String>()?);
                        }
                        "skip_unavailable" => {
                            if f_skip_unavailable.is_some() {
                                return Err(serde::de::Error::duplicate_field("skip_unavailable"));
                            }
                            f_skip_unavailable = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterRemoteProxyInfo {
                    connected: f_connected
                        .ok_or_else(|| serde::de::Error::missing_field("connected"))?,
                    initial_connect_timeout: f_initial_connect_timeout.ok_or_else(|| {
                        serde::de::Error::missing_field("initial_connect_timeout")
                    })?,
                    max_proxy_socket_connections: f_max_proxy_socket_connections.ok_or_else(
                        || serde::de::Error::missing_field("max_proxy_socket_connections"),
                    )?,
                    mode: f_mode.ok_or_else(|| serde::de::Error::missing_field("mode"))?,
                    num_proxy_sockets_connected: f_num_proxy_sockets_connected.ok_or_else(
                        || serde::de::Error::missing_field("num_proxy_sockets_connected"),
                    )?,
                    proxy_address: f_proxy_address
                        .ok_or_else(|| serde::de::Error::missing_field("proxy_address"))?,
                    server_name: f_server_name
                        .ok_or_else(|| serde::de::Error::missing_field("server_name"))?,
                    skip_unavailable: f_skip_unavailable
                        .ok_or_else(|| serde::de::Error::missing_field("skip_unavailable"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterRemoteProxyInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("connected", &self.connected)?;
        map.serialize_entry("initial_connect_timeout", &self.initial_connect_timeout)?;
        map.serialize_entry(
            "max_proxy_socket_connections",
            &self.max_proxy_socket_connections,
        )?;
        map.serialize_entry("mode", &self.mode)?;
        map.serialize_entry(
            "num_proxy_sockets_connected",
            &self.num_proxy_sockets_connected,
        )?;
        map.serialize_entry("proxy_address", &self.proxy_address)?;
        map.serialize_entry("server_name", &self.server_name)?;
        map.serialize_entry("skip_unavailable", &self.skip_unavailable)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClusterRemoteProxyInfoMode {
    #[doc = "`proxy`"]
    Proxy,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ClusterRemoteProxyInfoMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ClusterRemoteProxyInfoMode::Proxy => "proxy",
            ClusterRemoteProxyInfoMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ClusterRemoteProxyInfoMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ClusterRemoteProxyInfoMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "proxy" => ClusterRemoteProxyInfoMode::Proxy,
            _ => ClusterRemoteProxyInfoMode::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ClusterRemoteSniffInfo {
    pub connected: bool,
    pub initial_connect_timeout: crate::types::common::Duration,
    pub max_connections_per_cluster: i32,
    pub mode: crate::types::cluster_remote_info::ClusterRemoteSniffInfoMode,
    pub num_nodes_connected: i64,
    pub seeds: Vec<String>,
    pub skip_unavailable: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterRemoteSniffInfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterRemoteSniffInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterRemoteSniffInfo object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_connected: Option<bool> = None;
                let mut f_initial_connect_timeout: Option<crate::types::common::Duration> = None;
                let mut f_max_connections_per_cluster: Option<i32> = None;
                let mut f_mode: Option<
                    crate::types::cluster_remote_info::ClusterRemoteSniffInfoMode,
                > = None;
                let mut f_num_nodes_connected: Option<i64> = None;
                let mut f_seeds: Option<Vec<String>> = None;
                let mut f_skip_unavailable: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "connected" => {
                            if f_connected.is_some() {
                                return Err(serde::de::Error::duplicate_field("connected"));
                            }
                            f_connected = Some(map.next_value::<bool>()?);
                        }
                        "initial_connect_timeout" => {
                            if f_initial_connect_timeout.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "initial_connect_timeout",
                                ));
                            }
                            f_initial_connect_timeout =
                                Some(map.next_value::<crate::types::common::Duration>()?);
                        }
                        "max_connections_per_cluster" => {
                            if f_max_connections_per_cluster.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_connections_per_cluster",
                                ));
                            }
                            f_max_connections_per_cluster = Some(map.next_value::<i32>()?);
                        }
                        "mode" => {
                            if f_mode.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            f_mode = Some(map.next_value::<crate::types::cluster_remote_info::ClusterRemoteSniffInfoMode>()?);
                        }
                        "num_nodes_connected" => {
                            if f_num_nodes_connected.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "num_nodes_connected",
                                ));
                            }
                            f_num_nodes_connected = Some(map.next_value::<i64>()?);
                        }
                        "seeds" => {
                            if f_seeds.is_some() {
                                return Err(serde::de::Error::duplicate_field("seeds"));
                            }
                            f_seeds = Some(map.next_value::<Vec<String>>()?);
                        }
                        "skip_unavailable" => {
                            if f_skip_unavailable.is_some() {
                                return Err(serde::de::Error::duplicate_field("skip_unavailable"));
                            }
                            f_skip_unavailable = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterRemoteSniffInfo {
                    connected: f_connected
                        .ok_or_else(|| serde::de::Error::missing_field("connected"))?,
                    initial_connect_timeout: f_initial_connect_timeout.ok_or_else(|| {
                        serde::de::Error::missing_field("initial_connect_timeout")
                    })?,
                    max_connections_per_cluster: f_max_connections_per_cluster.ok_or_else(
                        || serde::de::Error::missing_field("max_connections_per_cluster"),
                    )?,
                    mode: f_mode.ok_or_else(|| serde::de::Error::missing_field("mode"))?,
                    num_nodes_connected: f_num_nodes_connected
                        .ok_or_else(|| serde::de::Error::missing_field("num_nodes_connected"))?,
                    seeds: f_seeds.ok_or_else(|| serde::de::Error::missing_field("seeds"))?,
                    skip_unavailable: f_skip_unavailable
                        .ok_or_else(|| serde::de::Error::missing_field("skip_unavailable"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterRemoteSniffInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("connected", &self.connected)?;
        map.serialize_entry("initial_connect_timeout", &self.initial_connect_timeout)?;
        map.serialize_entry(
            "max_connections_per_cluster",
            &self.max_connections_per_cluster,
        )?;
        map.serialize_entry("mode", &self.mode)?;
        map.serialize_entry("num_nodes_connected", &self.num_nodes_connected)?;
        map.serialize_entry("seeds", &self.seeds)?;
        map.serialize_entry("skip_unavailable", &self.skip_unavailable)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClusterRemoteSniffInfoMode {
    #[doc = "`sniff`"]
    Sniff,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ClusterRemoteSniffInfoMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ClusterRemoteSniffInfoMode::Sniff => "sniff",
            ClusterRemoteSniffInfoMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ClusterRemoteSniffInfoMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ClusterRemoteSniffInfoMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "sniff" => ClusterRemoteSniffInfoMode::Sniff,
            _ => ClusterRemoteSniffInfoMode::Unknown(s),
        })
    }
}

#[doc = "The response of the `cluster.remote_info` API"]
pub type RemoteInfoResponse = crate::types::cluster::RemoteInfoResponse;
