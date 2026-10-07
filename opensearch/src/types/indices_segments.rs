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

//! Response types of the `indices_segments` specification namespace

#![allow(missing_docs)]

#[doc = "The segment information for an index."]
#[derive(Debug, Clone)]
pub struct IndexSegment {
    #[doc = "The segment information for each shard."]
    pub shards:
        std::collections::BTreeMap<String, crate::types::indices_segments::IndexSegmentShardsValue>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSegment {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSegment;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSegment object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_shards: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::indices_segments::IndexSegmentShardsValue,
                    >,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "shards" => {
                            if f_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("shards"));
                            }
                            f_shards = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                crate::types::indices_segments::IndexSegmentShardsValue,
                            >>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSegment {
                    shards: f_shards.ok_or_else(|| serde::de::Error::missing_field("shards"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSegment {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("shards", &self.shards)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum IndexSegmentShardsValue {
    Array(Vec<crate::types::indices_segments::ShardsSegment>),
    ShardsSegment(crate::types::indices_segments::ShardsSegment),
}
impl<'de> serde::Deserialize<'de> for IndexSegmentShardsValue {
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
            Some(b'[') => serde_json::from_str(text).map(IndexSegmentShardsValue::Array),
            Some(b'{') => serde_json::from_str(text).map(IndexSegmentShardsValue::ShardsSegment),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a IndexSegmentShardsValue value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The detailed information about a segment."]
#[derive(Debug, Clone)]
pub struct Segment {
    #[doc = "The attributes of the segment."]
    pub attributes: std::collections::BTreeMap<String, String>,
    #[doc = "Whether the segment is committed."]
    pub committed: bool,
    #[doc = "Whether the segment is compound."]
    pub compound: bool,
    #[doc = "The number of deleted documents in the segment."]
    pub deleted_docs: i64,
    #[doc = "The generation number of the segment."]
    pub generation: i64,
    pub memory: Option<crate::types::common::HumanReadableByteCount>,
    pub memory_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "The number of documents in the segment."]
    pub num_docs: i64,
    #[doc = "Whether the segment is searchable."]
    pub search: bool,
    pub size: Option<crate::types::common::HumanReadableByteCount>,
    pub size_in_bytes: crate::types::common::ByteCount,
    pub version: crate::types::common::VersionString,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Segment {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Segment;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Segment object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_attributes: Option<std::collections::BTreeMap<String, String>> = None;
                let mut f_committed: Option<bool> = None;
                let mut f_compound: Option<bool> = None;
                let mut f_deleted_docs: Option<i64> = None;
                let mut f_generation: Option<i64> = None;
                let mut f_memory: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_memory = false;
                let mut f_memory_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut seen_memory_in_bytes = false;
                let mut f_num_docs: Option<i64> = None;
                let mut f_search: Option<bool> = None;
                let mut f_size: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_size = false;
                let mut f_size_in_bytes: Option<crate::types::common::ByteCount> = None;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "attributes" => {
                            if f_attributes.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            f_attributes = Some(
                                map.next_value::<std::collections::BTreeMap<String, String>>()?,
                            );
                        }
                        "committed" => {
                            if f_committed.is_some() {
                                return Err(serde::de::Error::duplicate_field("committed"));
                            }
                            f_committed = Some(map.next_value::<bool>()?);
                        }
                        "compound" => {
                            if f_compound.is_some() {
                                return Err(serde::de::Error::duplicate_field("compound"));
                            }
                            f_compound = Some(map.next_value::<bool>()?);
                        }
                        "deleted_docs" => {
                            if f_deleted_docs.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleted_docs"));
                            }
                            f_deleted_docs = Some(map.next_value::<i64>()?);
                        }
                        "generation" => {
                            if f_generation.is_some() {
                                return Err(serde::de::Error::duplicate_field("generation"));
                            }
                            f_generation = Some(map.next_value::<i64>()?);
                        }
                        "memory" => {
                            if seen_memory {
                                return Err(serde::de::Error::duplicate_field("memory"));
                            }
                            seen_memory = true;
                            f_memory = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "memory_in_bytes" => {
                            if seen_memory_in_bytes {
                                return Err(serde::de::Error::duplicate_field("memory_in_bytes"));
                            }
                            seen_memory_in_bytes = true;
                            f_memory_in_bytes =
                                map.next_value::<Option<crate::types::common::ByteCount>>()?;
                        }
                        "num_docs" => {
                            if f_num_docs.is_some() {
                                return Err(serde::de::Error::duplicate_field("num_docs"));
                            }
                            f_num_docs = Some(map.next_value::<i64>()?);
                        }
                        "search" => {
                            if f_search.is_some() {
                                return Err(serde::de::Error::duplicate_field("search"));
                            }
                            f_search = Some(map.next_value::<bool>()?);
                        }
                        "size" => {
                            if seen_size {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            seen_size = true;
                            f_size = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "size_in_bytes" => {
                            if f_size_in_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("size_in_bytes"));
                            }
                            f_size_in_bytes =
                                Some(map.next_value::<crate::types::common::ByteCount>()?);
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
                Ok(Segment {
                    attributes: f_attributes
                        .ok_or_else(|| serde::de::Error::missing_field("attributes"))?,
                    committed: f_committed
                        .ok_or_else(|| serde::de::Error::missing_field("committed"))?,
                    compound: f_compound
                        .ok_or_else(|| serde::de::Error::missing_field("compound"))?,
                    deleted_docs: f_deleted_docs
                        .ok_or_else(|| serde::de::Error::missing_field("deleted_docs"))?,
                    generation: f_generation
                        .ok_or_else(|| serde::de::Error::missing_field("generation"))?,
                    memory: f_memory,
                    memory_in_bytes: f_memory_in_bytes,
                    num_docs: f_num_docs
                        .ok_or_else(|| serde::de::Error::missing_field("num_docs"))?,
                    search: f_search.ok_or_else(|| serde::de::Error::missing_field("search"))?,
                    size: f_size,
                    size_in_bytes: f_size_in_bytes
                        .ok_or_else(|| serde::de::Error::missing_field("size_in_bytes"))?,
                    version: f_version.ok_or_else(|| serde::de::Error::missing_field("version"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Segment {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("attributes", &self.attributes)?;
        map.serialize_entry("committed", &self.committed)?;
        map.serialize_entry("compound", &self.compound)?;
        map.serialize_entry("deleted_docs", &self.deleted_docs)?;
        map.serialize_entry("generation", &self.generation)?;
        if let Some(v) = &self.memory {
            map.serialize_entry("memory", v)?;
        }
        if let Some(v) = &self.memory_in_bytes {
            map.serialize_entry("memory_in_bytes", v)?;
        }
        map.serialize_entry("num_docs", &self.num_docs)?;
        map.serialize_entry("search", &self.search)?;
        if let Some(v) = &self.size {
            map.serialize_entry("size", v)?;
        }
        map.serialize_entry("size_in_bytes", &self.size_in_bytes)?;
        map.serialize_entry("version", &self.version)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `indices.segments` API"]
pub type SegmentsResponse = crate::types::indices::SegmentsResponse;

#[doc = "The routing information for a shard segment."]
#[derive(Debug, Clone)]
pub struct ShardSegmentRouting {
    #[doc = "The node containing the segment."]
    pub node: String,
    #[doc = "Whether this is a primary shard."]
    pub primary: bool,
    #[doc = "The current state of the shard."]
    pub state: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardSegmentRouting {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardSegmentRouting;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardSegmentRouting object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_node: Option<String> = None;
                let mut f_primary: Option<bool> = None;
                let mut f_state: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "node" => {
                            if f_node.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            f_node = Some(map.next_value::<String>()?);
                        }
                        "primary" => {
                            if f_primary.is_some() {
                                return Err(serde::de::Error::duplicate_field("primary"));
                            }
                            f_primary = Some(map.next_value::<bool>()?);
                        }
                        "state" => {
                            if f_state.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            f_state = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardSegmentRouting {
                    node: f_node.ok_or_else(|| serde::de::Error::missing_field("node"))?,
                    primary: f_primary.ok_or_else(|| serde::de::Error::missing_field("primary"))?,
                    state: f_state.ok_or_else(|| serde::de::Error::missing_field("state"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardSegmentRouting {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("node", &self.node)?;
        map.serialize_entry("primary", &self.primary)?;
        map.serialize_entry("state", &self.state)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The segment information for a shard."]
#[derive(Debug, Clone)]
pub struct ShardsSegment {
    #[doc = "The number of committed segments."]
    pub num_committed_segments: i32,
    #[doc = "The number of segments available for search."]
    pub num_search_segments: i32,
    pub routing: crate::types::indices_segments::ShardSegmentRouting,
    #[doc = "The detailed information for each segment."]
    pub segments: std::collections::BTreeMap<String, crate::types::indices_segments::Segment>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardsSegment {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardsSegment;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardsSegment object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_num_committed_segments: Option<i32> = None;
                let mut f_num_search_segments: Option<i32> = None;
                let mut f_routing: Option<crate::types::indices_segments::ShardSegmentRouting> =
                    None;
                let mut f_segments: Option<
                    std::collections::BTreeMap<String, crate::types::indices_segments::Segment>,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "num_committed_segments" => {
                            if f_num_committed_segments.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "num_committed_segments",
                                ));
                            }
                            f_num_committed_segments = Some(map.next_value::<i32>()?);
                        }
                        "num_search_segments" => {
                            if f_num_search_segments.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "num_search_segments",
                                ));
                            }
                            f_num_search_segments = Some(map.next_value::<i32>()?);
                        }
                        "routing" => {
                            if f_routing.is_some() {
                                return Err(serde::de::Error::duplicate_field("routing"));
                            }
                            f_routing = Some(map.next_value::<crate::types::indices_segments::ShardSegmentRouting>()?);
                        }
                        "segments" => {
                            if f_segments.is_some() {
                                return Err(serde::de::Error::duplicate_field("segments"));
                            }
                            f_segments = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                crate::types::indices_segments::Segment,
                            >>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardsSegment {
                    num_committed_segments: f_num_committed_segments
                        .ok_or_else(|| serde::de::Error::missing_field("num_committed_segments"))?,
                    num_search_segments: f_num_search_segments
                        .ok_or_else(|| serde::de::Error::missing_field("num_search_segments"))?,
                    routing: f_routing.ok_or_else(|| serde::de::Error::missing_field("routing"))?,
                    segments: f_segments
                        .ok_or_else(|| serde::de::Error::missing_field("segments"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardsSegment {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("num_committed_segments", &self.num_committed_segments)?;
        map.serialize_entry("num_search_segments", &self.num_search_segments)?;
        map.serialize_entry("routing", &self.routing)?;
        map.serialize_entry("segments", &self.segments)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
