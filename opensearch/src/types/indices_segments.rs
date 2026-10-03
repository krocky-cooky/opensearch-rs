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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSegment {
    #[doc = "The segment information for each shard."]
    pub shards:
        std::collections::BTreeMap<String, crate::types::indices_segments::IndexSegmentShardsValue>,
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
        struct KindVisitor;
        impl<'de> serde::de::Visitor<'de> for KindVisitor {
            type Value = IndexSegmentShardsValue;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSegmentShardsValue value")
            }
            fn visit_seq<A>(self, seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
                    .map(IndexSegmentShardsValue::Array)
            }
            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(IndexSegmentShardsValue::ShardsSegment)
            }
        }
        deserializer.deserialize_any(KindVisitor)
    }
}

#[doc = "The detailed information about a segment."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Segment {
    #[doc = "The attributes of the segment."]
    pub attributes: std::collections::BTreeMap<String, String>,
    #[doc = "Whether the segment is committed."]
    pub committed: bool,
    #[doc = "Whether the segment is compound."]
    pub compound: bool,
    #[doc = "The number of deleted documents in the segment."]
    pub deleted_docs: i32,
    #[doc = "The generation number of the segment."]
    pub generation: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<crate::types::common::HumanReadableByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "The number of documents in the segment."]
    pub num_docs: i32,
    #[doc = "Whether the segment is searchable."]
    pub search: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<crate::types::common::HumanReadableByteCount>,
    pub size_in_bytes: crate::types::common::ByteCount,
    pub version: crate::types::common::VersionString,
}

#[doc = "The response of the `indices.segments` API"]
pub type SegmentsResponse = crate::types::indices::SegmentsResponse;

#[doc = "The routing information for a shard segment."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardSegmentRouting {
    #[doc = "The node containing the segment."]
    pub node: String,
    #[doc = "Whether this is a primary shard."]
    pub primary: bool,
    #[doc = "The current state of the shard."]
    pub state: String,
}

#[doc = "The segment information for a shard."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardsSegment {
    #[doc = "The number of committed segments."]
    pub num_committed_segments: i32,
    #[doc = "The number of segments available for search."]
    pub num_search_segments: i32,
    pub routing: crate::types::indices_segments::ShardSegmentRouting,
    #[doc = "The detailed information for each segment."]
    pub segments: std::collections::BTreeMap<String, crate::types::indices_segments::Segment>,
}
