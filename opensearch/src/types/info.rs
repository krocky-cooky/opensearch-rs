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

//! Response types of the `info` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct InfoResponse {
    pub cluster_name: crate::types::common::Name,
    pub cluster_uuid: crate::types::common::Uuid,
    pub name: crate::types::common::Name,
    pub tagline: String,
    pub version: crate::types::common::OpenSearchVersionInfo,
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
                let mut f_cluster_name: Option<crate::types::common::Name> = None;
                let mut f_cluster_uuid: Option<crate::types::common::Uuid> = None;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut f_tagline: Option<String> = None;
                let mut f_version: Option<crate::types::common::OpenSearchVersionInfo> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cluster_name" => {
                            if f_cluster_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster_name"));
                            }
                            f_cluster_name = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        "cluster_uuid" => {
                            if f_cluster_uuid.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster_uuid"));
                            }
                            f_cluster_uuid = Some(map.next_value::<crate::types::common::Uuid>()?);
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        "tagline" => {
                            if f_tagline.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagline"));
                            }
                            f_tagline = Some(map.next_value::<String>()?);
                        }
                        "version" => {
                            if f_version.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            f_version = Some(
                                map.next_value::<crate::types::common::OpenSearchVersionInfo>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(InfoResponse {
                    cluster_name: f_cluster_name
                        .ok_or_else(|| serde::de::Error::missing_field("cluster_name"))?,
                    cluster_uuid: f_cluster_uuid
                        .ok_or_else(|| serde::de::Error::missing_field("cluster_uuid"))?,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    tagline: f_tagline.ok_or_else(|| serde::de::Error::missing_field("tagline"))?,
                    version: f_version.ok_or_else(|| serde::de::Error::missing_field("version"))?,
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
        map.serialize_entry("cluster_name", &self.cluster_name)?;
        map.serialize_entry("cluster_uuid", &self.cluster_uuid)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("tagline", &self.tagline)?;
        map.serialize_entry("version", &self.version)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
