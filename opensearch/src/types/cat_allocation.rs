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

//! Response types of the `cat_allocation` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct AllocationRecord {
    #[doc = "Free disk space available to OpenSearch. OpenSearch retrieves this metric from the node's operating system. Disk-based shard allocation uses this metric to assign shards to nodes based on available disk space."]
    pub disk_avail: Option<String>,
    #[doc = "Disk space used by the node's shards. Does not include disk space for the translog or unassigned shards. IMPORTANT: This metric double-counts disk space for hard-linked files, such as those created when shrinking, splitting, or cloning an index."]
    pub disk_indices: Option<String>,
    #[doc = "Total percentage of disk space in use. Calculated as `disk.used / disk.total`."]
    pub disk_percent: Option<crate::types::common::PercentageString>,
    #[doc = "Total disk space for the node, including in-use and available space."]
    pub disk_total: Option<String>,
    #[doc = "Total disk space in use. OpenSearch retrieves this metric from the node's operating system (OS). The metric includes disk space for: OpenSearch, including the translog and unassigned shards; the node's operating system; any other applications or files on the node. Unlike `disk.indices`, this metric does not double-count disk space for hard-linked files."]
    pub disk_used: Option<String>,
    #[doc = "Network host for the node. Set using the `network.host` setting."]
    pub host: Option<crate::types::common::Host>,
    #[doc = "IP address and port for the node."]
    pub ip: Option<crate::types::common::Ip>,
    #[doc = "Name for the node. Set using the `node.name` setting."]
    pub node: Option<String>,
    #[doc = "Number of primary and replica shards assigned to the node."]
    pub shards: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AllocationRecord {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AllocationRecord;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AllocationRecord object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_disk_avail: Option<String> = None;
                let mut seen_disk_avail = false;
                let mut f_disk_indices: Option<String> = None;
                let mut seen_disk_indices = false;
                let mut f_disk_percent: Option<crate::types::common::PercentageString> = None;
                let mut seen_disk_percent = false;
                let mut f_disk_total: Option<String> = None;
                let mut seen_disk_total = false;
                let mut f_disk_used: Option<String> = None;
                let mut seen_disk_used = false;
                let mut f_host: Option<crate::types::common::Host> = None;
                let mut seen_host = false;
                let mut f_ip: Option<crate::types::common::Ip> = None;
                let mut seen_ip = false;
                let mut f_node: Option<String> = None;
                let mut seen_node = false;
                let mut f_shards: Option<String> = None;
                let mut seen_shards = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "disk.avail" => {
                            if seen_disk_avail {
                                return Err(serde::de::Error::duplicate_field("disk.avail"));
                            }
                            seen_disk_avail = true;
                            f_disk_avail = map.next_value::<Option<String>>()?;
                        }
                        "disk.indices" => {
                            if seen_disk_indices {
                                return Err(serde::de::Error::duplicate_field("disk.indices"));
                            }
                            seen_disk_indices = true;
                            f_disk_indices = map.next_value::<Option<String>>()?;
                        }
                        "disk.percent" => {
                            if seen_disk_percent {
                                return Err(serde::de::Error::duplicate_field("disk.percent"));
                            }
                            seen_disk_percent = true;
                            f_disk_percent =
                                map.next_value::<Option<crate::types::common::PercentageString>>()?;
                        }
                        "disk.total" => {
                            if seen_disk_total {
                                return Err(serde::de::Error::duplicate_field("disk.total"));
                            }
                            seen_disk_total = true;
                            f_disk_total = map.next_value::<Option<String>>()?;
                        }
                        "disk.used" => {
                            if seen_disk_used {
                                return Err(serde::de::Error::duplicate_field("disk.used"));
                            }
                            seen_disk_used = true;
                            f_disk_used = map.next_value::<Option<String>>()?;
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
                        "node" => {
                            if seen_node {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            seen_node = true;
                            f_node = map.next_value::<Option<String>>()?;
                        }
                        "shards" => {
                            if seen_shards {
                                return Err(serde::de::Error::duplicate_field("shards"));
                            }
                            seen_shards = true;
                            f_shards = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AllocationRecord {
                    disk_avail: f_disk_avail,
                    disk_indices: f_disk_indices,
                    disk_percent: f_disk_percent,
                    disk_total: f_disk_total,
                    disk_used: f_disk_used,
                    host: f_host,
                    ip: f_ip,
                    node: f_node,
                    shards: f_shards,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AllocationRecord {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.disk_avail {
            map.serialize_entry("disk.avail", v)?;
        }
        if let Some(v) = &self.disk_indices {
            map.serialize_entry("disk.indices", v)?;
        }
        if let Some(v) = &self.disk_percent {
            map.serialize_entry("disk.percent", v)?;
        }
        if let Some(v) = &self.disk_total {
            map.serialize_entry("disk.total", v)?;
        }
        if let Some(v) = &self.disk_used {
            map.serialize_entry("disk.used", v)?;
        }
        if let Some(v) = &self.host {
            map.serialize_entry("host", v)?;
        }
        if let Some(v) = &self.ip {
            map.serialize_entry("ip", v)?;
        }
        if let Some(v) = &self.node {
            map.serialize_entry("node", v)?;
        }
        if let Some(v) = &self.shards {
            map.serialize_entry("shards", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `cat.allocation` API"]
pub type AllocationResponse = crate::types::cat::AllocationResponse;
