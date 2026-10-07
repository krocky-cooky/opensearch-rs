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

//! Response types of the `cat_segments` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct SegmentsRecord {
    #[doc = "If `true`, the segment is synced to disk. Segments that are synced can survive a hard reboot. If `false`, the data from uncommitted segments is also stored in the transaction log so that OpenSearch is able to replay changes on the next start."]
    pub committed: Option<String>,
    #[doc = "If `true`, the segment is stored in a compound file. This means Lucene merged all files from the segment in a single file to save file descriptors."]
    pub compound: Option<String>,
    #[doc = "The number of documents in the segment. This excludes deleted documents and counts any nested documents separately from their parents. It also excludes documents which were indexed recently and do not yet belong to a segment."]
    pub docs_count: Option<String>,
    #[doc = "The number of deleted documents in the segment, which might be higher or lower than the number of delete operations you have performed. This number excludes deletes that were performed recently and do not yet belong to a segment. Deleted documents are cleaned up by the automatic merge process if it makes sense to do so. Also, OpenSearch creates extra deleted documents to internally track the recent history of operations on a shard."]
    pub docs_deleted: Option<String>,
    #[doc = "The segment generation number. OpenSearch increments this generation number for each segment written then uses this number to derive the segment name."]
    pub generation: Option<String>,
    pub id: Option<crate::types::common::NodeId>,
    pub index: Option<crate::types::common::IndexName>,
    #[doc = "The IP address of the node where it lives."]
    pub ip: Option<String>,
    #[doc = "The shard type: `primary` or `replica`."]
    pub prirep: Option<String>,
    #[doc = "If `true`, the segment is searchable. If `false`, the segment has most likely been written to disk but needs a refresh to be searchable."]
    pub searchable: Option<String>,
    #[doc = "The segment name, which is derived from the segment generation and used internally to create file names in the directory of the shard."]
    pub segment: Option<String>,
    #[doc = "The shard name."]
    pub shard: Option<String>,
    pub size: Option<crate::types::common::HumanReadableByteCount>,
    pub size_memory: Option<String>,
    pub version: Option<crate::types::common::VersionString>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SegmentsRecord {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SegmentsRecord;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SegmentsRecord object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_committed: Option<String> = None;
                let mut seen_committed = false;
                let mut f_compound: Option<String> = None;
                let mut seen_compound = false;
                let mut f_docs_count: Option<String> = None;
                let mut seen_docs_count = false;
                let mut f_docs_deleted: Option<String> = None;
                let mut seen_docs_deleted = false;
                let mut f_generation: Option<String> = None;
                let mut seen_generation = false;
                let mut f_id: Option<crate::types::common::NodeId> = None;
                let mut seen_id = false;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut seen_index = false;
                let mut f_ip: Option<String> = None;
                let mut seen_ip = false;
                let mut f_prirep: Option<String> = None;
                let mut seen_prirep = false;
                let mut f_searchable: Option<String> = None;
                let mut seen_searchable = false;
                let mut f_segment: Option<String> = None;
                let mut seen_segment = false;
                let mut f_shard: Option<String> = None;
                let mut seen_shard = false;
                let mut f_size: Option<crate::types::common::HumanReadableByteCount> = None;
                let mut seen_size = false;
                let mut f_size_memory: Option<String> = None;
                let mut seen_size_memory = false;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "committed" => {
                            if seen_committed {
                                return Err(serde::de::Error::duplicate_field("committed"));
                            }
                            seen_committed = true;
                            f_committed = map.next_value::<Option<String>>()?;
                        }
                        "compound" => {
                            if seen_compound {
                                return Err(serde::de::Error::duplicate_field("compound"));
                            }
                            seen_compound = true;
                            f_compound = map.next_value::<Option<String>>()?;
                        }
                        "docs.count" => {
                            if seen_docs_count {
                                return Err(serde::de::Error::duplicate_field("docs.count"));
                            }
                            seen_docs_count = true;
                            f_docs_count = map.next_value::<Option<String>>()?;
                        }
                        "docs.deleted" => {
                            if seen_docs_deleted {
                                return Err(serde::de::Error::duplicate_field("docs.deleted"));
                            }
                            seen_docs_deleted = true;
                            f_docs_deleted = map.next_value::<Option<String>>()?;
                        }
                        "generation" => {
                            if seen_generation {
                                return Err(serde::de::Error::duplicate_field("generation"));
                            }
                            seen_generation = true;
                            f_generation = map.next_value::<Option<String>>()?;
                        }
                        "id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<crate::types::common::NodeId>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "ip" => {
                            if seen_ip {
                                return Err(serde::de::Error::duplicate_field("ip"));
                            }
                            seen_ip = true;
                            f_ip = map.next_value::<Option<String>>()?;
                        }
                        "prirep" => {
                            if seen_prirep {
                                return Err(serde::de::Error::duplicate_field("prirep"));
                            }
                            seen_prirep = true;
                            f_prirep = map.next_value::<Option<String>>()?;
                        }
                        "searchable" => {
                            if seen_searchable {
                                return Err(serde::de::Error::duplicate_field("searchable"));
                            }
                            seen_searchable = true;
                            f_searchable = map.next_value::<Option<String>>()?;
                        }
                        "segment" => {
                            if seen_segment {
                                return Err(serde::de::Error::duplicate_field("segment"));
                            }
                            seen_segment = true;
                            f_segment = map.next_value::<Option<String>>()?;
                        }
                        "shard" => {
                            if seen_shard {
                                return Err(serde::de::Error::duplicate_field("shard"));
                            }
                            seen_shard = true;
                            f_shard = map.next_value::<Option<String>>()?;
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
                        "size.memory" => {
                            if seen_size_memory {
                                return Err(serde::de::Error::duplicate_field("size.memory"));
                            }
                            seen_size_memory = true;
                            f_size_memory = map.next_value::<Option<String>>()?;
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
                Ok(SegmentsRecord {
                    committed: f_committed,
                    compound: f_compound,
                    docs_count: f_docs_count,
                    docs_deleted: f_docs_deleted,
                    generation: f_generation,
                    id: f_id,
                    index: f_index,
                    ip: f_ip,
                    prirep: f_prirep,
                    searchable: f_searchable,
                    segment: f_segment,
                    shard: f_shard,
                    size: f_size,
                    size_memory: f_size_memory,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SegmentsRecord {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.committed {
            map.serialize_entry("committed", v)?;
        }
        if let Some(v) = &self.compound {
            map.serialize_entry("compound", v)?;
        }
        if let Some(v) = &self.docs_count {
            map.serialize_entry("docs.count", v)?;
        }
        if let Some(v) = &self.docs_deleted {
            map.serialize_entry("docs.deleted", v)?;
        }
        if let Some(v) = &self.generation {
            map.serialize_entry("generation", v)?;
        }
        if let Some(v) = &self.id {
            map.serialize_entry("id", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.ip {
            map.serialize_entry("ip", v)?;
        }
        if let Some(v) = &self.prirep {
            map.serialize_entry("prirep", v)?;
        }
        if let Some(v) = &self.searchable {
            map.serialize_entry("searchable", v)?;
        }
        if let Some(v) = &self.segment {
            map.serialize_entry("segment", v)?;
        }
        if let Some(v) = &self.shard {
            map.serialize_entry("shard", v)?;
        }
        if let Some(v) = &self.size {
            map.serialize_entry("size", v)?;
        }
        if let Some(v) = &self.size_memory {
            map.serialize_entry("size.memory", v)?;
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

#[doc = "The response of the `cat.segments` API"]
pub type SegmentsResponse = crate::types::cat::SegmentsResponse;
