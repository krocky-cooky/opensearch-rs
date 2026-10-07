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

//! Response types of the `update` specification namespace

#![allow(missing_docs)]

#[doc = "The response of the `update` API"]
pub type UpdateResponse = crate::types::update::UpdateWriteResponseBase;

#[derive(Debug, Clone)]
pub struct UpdateWriteResponseBase {
    pub id: crate::types::common::Id,
    pub index: crate::types::common::IndexName,
    #[doc = "The primary term of the document."]
    pub primary_term: i64,
    pub seq_no: crate::types::common::SequenceNumber,
    pub shards: crate::types::common::ShardStatistics,
    pub r#type: Option<crate::types::common::Type>,
    pub version: crate::types::common::VersionNumber,
    pub forced_refresh: Option<bool>,
    pub result: crate::types::common::Result,
    pub get: Option<crate::types::common::InlineGet>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UpdateWriteResponseBase {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UpdateWriteResponseBase;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UpdateWriteResponseBase object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut f_primary_term: Option<i64> = None;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut f_shards: Option<crate::types::common::ShardStatistics> = None;
                let mut f_type: Option<crate::types::common::Type> = None;
                let mut seen_type = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut f_forced_refresh: Option<bool> = None;
                let mut seen_forced_refresh = false;
                let mut f_result: Option<crate::types::common::Result> = None;
                let mut f_get: Option<crate::types::common::InlineGet> = None;
                let mut seen_get = false;
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
                            f_id = Some(map.next_value::<crate::types::common::Id>()?);
                        }
                        "_index" => {
                            if f_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("_index"));
                            }
                            f_index = Some(map.next_value::<crate::types::common::IndexName>()?);
                        }
                        "_primary_term" => {
                            if f_primary_term.is_some() {
                                return Err(serde::de::Error::duplicate_field("_primary_term"));
                            }
                            f_primary_term = Some(map.next_value::<i64>()?);
                        }
                        "_seq_no" => {
                            if f_seq_no.is_some() {
                                return Err(serde::de::Error::duplicate_field("_seq_no"));
                            }
                            f_seq_no =
                                Some(map.next_value::<crate::types::common::SequenceNumber>()?);
                        }
                        "_shards" => {
                            if f_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("_shards"));
                            }
                            f_shards =
                                Some(map.next_value::<crate::types::common::ShardStatistics>()?);
                        }
                        "_type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<crate::types::common::Type>>()?;
                        }
                        "_version" => {
                            if f_version.is_some() {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            f_version =
                                Some(map.next_value::<crate::types::common::VersionNumber>()?);
                        }
                        "forced_refresh" => {
                            if seen_forced_refresh {
                                return Err(serde::de::Error::duplicate_field("forced_refresh"));
                            }
                            seen_forced_refresh = true;
                            f_forced_refresh = map.next_value::<Option<bool>>()?;
                        }
                        "result" => {
                            if f_result.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            f_result = Some(map.next_value::<crate::types::common::Result>()?);
                        }
                        "get" => {
                            if seen_get {
                                return Err(serde::de::Error::duplicate_field("get"));
                            }
                            seen_get = true;
                            f_get = map.next_value::<Option<crate::types::common::InlineGet>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UpdateWriteResponseBase {
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("_id"))?,
                    index: f_index.ok_or_else(|| serde::de::Error::missing_field("_index"))?,
                    primary_term: f_primary_term
                        .ok_or_else(|| serde::de::Error::missing_field("_primary_term"))?,
                    seq_no: f_seq_no.ok_or_else(|| serde::de::Error::missing_field("_seq_no"))?,
                    shards: f_shards.ok_or_else(|| serde::de::Error::missing_field("_shards"))?,
                    r#type: f_type,
                    version: f_version
                        .ok_or_else(|| serde::de::Error::missing_field("_version"))?,
                    forced_refresh: f_forced_refresh,
                    result: f_result.ok_or_else(|| serde::de::Error::missing_field("result"))?,
                    get: f_get,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UpdateWriteResponseBase {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("_id", &self.id)?;
        map.serialize_entry("_index", &self.index)?;
        map.serialize_entry("_primary_term", &self.primary_term)?;
        map.serialize_entry("_seq_no", &self.seq_no)?;
        map.serialize_entry("_shards", &self.shards)?;
        if let Some(v) = &self.r#type {
            map.serialize_entry("_type", v)?;
        }
        map.serialize_entry("_version", &self.version)?;
        if let Some(v) = &self.forced_refresh {
            map.serialize_entry("forced_refresh", v)?;
        }
        map.serialize_entry("result", &self.result)?;
        if let Some(v) = &self.get {
            map.serialize_entry("get", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
