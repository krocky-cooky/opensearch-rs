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

//! Response types of the `indices_resolve_index` specification namespace

#![allow(missing_docs)]

#[doc = "The resolved information for an index alias."]
#[derive(Debug, Clone)]
pub struct ResolveIndexAliasItem {
    pub indices: crate::types::common::Indices,
    pub name: crate::types::common::Name,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ResolveIndexAliasItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ResolveIndexAliasItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ResolveIndexAliasItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_indices: Option<crate::types::common::Indices> = None;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "indices" => {
                            if f_indices.is_some() {
                                return Err(serde::de::Error::duplicate_field("indices"));
                            }
                            f_indices = Some(map.next_value::<crate::types::common::Indices>()?);
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
                Ok(ResolveIndexAliasItem {
                    indices: f_indices.ok_or_else(|| serde::de::Error::missing_field("indices"))?,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ResolveIndexAliasItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("indices", &self.indices)?;
        map.serialize_entry("name", &self.name)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The resolved information for a data stream."]
#[derive(Debug, Clone)]
pub struct ResolveIndexDataStreamsItem {
    pub backing_indices: crate::types::common::Indices,
    pub name: crate::types::common::DataStreamName,
    pub timestamp_field: crate::types::common::Field,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ResolveIndexDataStreamsItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ResolveIndexDataStreamsItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ResolveIndexDataStreamsItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_backing_indices: Option<crate::types::common::Indices> = None;
                let mut f_name: Option<crate::types::common::DataStreamName> = None;
                let mut f_timestamp_field: Option<crate::types::common::Field> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "backing_indices" => {
                            if f_backing_indices.is_some() {
                                return Err(serde::de::Error::duplicate_field("backing_indices"));
                            }
                            f_backing_indices =
                                Some(map.next_value::<crate::types::common::Indices>()?);
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name =
                                Some(map.next_value::<crate::types::common::DataStreamName>()?);
                        }
                        "timestamp_field" => {
                            if f_timestamp_field.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp_field"));
                            }
                            f_timestamp_field =
                                Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ResolveIndexDataStreamsItem {
                    backing_indices: f_backing_indices
                        .ok_or_else(|| serde::de::Error::missing_field("backing_indices"))?,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    timestamp_field: f_timestamp_field
                        .ok_or_else(|| serde::de::Error::missing_field("timestamp_field"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ResolveIndexDataStreamsItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("backing_indices", &self.backing_indices)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("timestamp_field", &self.timestamp_field)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The resolved information for an index."]
#[derive(Debug, Clone)]
pub struct ResolveIndexItem {
    #[doc = "The list of aliases associated with the index."]
    pub aliases: Option<Vec<String>>,
    #[doc = "The list of index attributes."]
    pub attributes: Vec<String>,
    pub data_stream: Option<crate::types::common::DataStreamName>,
    pub name: crate::types::common::Name,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ResolveIndexItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ResolveIndexItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ResolveIndexItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_aliases: Option<Vec<String>> = None;
                let mut seen_aliases = false;
                let mut f_attributes: Option<Vec<String>> = None;
                let mut f_data_stream: Option<crate::types::common::DataStreamName> = None;
                let mut seen_data_stream = false;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "aliases" => {
                            if seen_aliases {
                                return Err(serde::de::Error::duplicate_field("aliases"));
                            }
                            seen_aliases = true;
                            f_aliases = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "attributes" => {
                            if f_attributes.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            f_attributes = Some(map.next_value::<Vec<String>>()?);
                        }
                        "data_stream" => {
                            if seen_data_stream {
                                return Err(serde::de::Error::duplicate_field("data_stream"));
                            }
                            seen_data_stream = true;
                            f_data_stream =
                                map.next_value::<Option<crate::types::common::DataStreamName>>()?;
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
                Ok(ResolveIndexItem {
                    aliases: f_aliases,
                    attributes: f_attributes
                        .ok_or_else(|| serde::de::Error::missing_field("attributes"))?,
                    data_stream: f_data_stream,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ResolveIndexItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.aliases {
            map.serialize_entry("aliases", v)?;
        }
        map.serialize_entry("attributes", &self.attributes)?;
        if let Some(v) = &self.data_stream {
            map.serialize_entry("data_stream", v)?;
        }
        map.serialize_entry("name", &self.name)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `indices.resolve_index` API"]
pub type ResolveIndexResponse = crate::types::indices::ResolveIndexResponse;
