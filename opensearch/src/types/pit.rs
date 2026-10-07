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

//! Response types of the `pit` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct DeletedPit {
    pub pit_id: Option<String>,
    pub successful: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DeletedPit {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DeletedPit;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DeletedPit object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_pit_id: Option<String> = None;
                let mut seen_pit_id = false;
                let mut f_successful: Option<bool> = None;
                let mut seen_successful = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "pit_id" => {
                            if seen_pit_id {
                                return Err(serde::de::Error::duplicate_field("pit_id"));
                            }
                            seen_pit_id = true;
                            f_pit_id = map.next_value::<Option<String>>()?;
                        }
                        "successful" => {
                            if seen_successful {
                                return Err(serde::de::Error::duplicate_field("successful"));
                            }
                            seen_successful = true;
                            f_successful = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DeletedPit {
                    pit_id: f_pit_id,
                    successful: f_successful,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DeletedPit {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.pit_id {
            map.serialize_entry("pit_id", v)?;
        }
        if let Some(v) = &self.successful {
            map.serialize_entry("successful", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PitDetail {
    pub creation_time: Option<i64>,
    pub keep_alive: Option<i64>,
    pub pit_id: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PitDetail {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PitDetail;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PitDetail object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_creation_time: Option<i64> = None;
                let mut seen_creation_time = false;
                let mut f_keep_alive: Option<i64> = None;
                let mut seen_keep_alive = false;
                let mut f_pit_id: Option<String> = None;
                let mut seen_pit_id = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "creation_time" => {
                            if seen_creation_time {
                                return Err(serde::de::Error::duplicate_field("creation_time"));
                            }
                            seen_creation_time = true;
                            f_creation_time = map.next_value::<Option<i64>>()?;
                        }
                        "keep_alive" => {
                            if seen_keep_alive {
                                return Err(serde::de::Error::duplicate_field("keep_alive"));
                            }
                            seen_keep_alive = true;
                            f_keep_alive = map.next_value::<Option<i64>>()?;
                        }
                        "pit_id" => {
                            if seen_pit_id {
                                return Err(serde::de::Error::duplicate_field("pit_id"));
                            }
                            seen_pit_id = true;
                            f_pit_id = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PitDetail {
                    creation_time: f_creation_time,
                    keep_alive: f_keep_alive,
                    pit_id: f_pit_id,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PitDetail {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.creation_time {
            map.serialize_entry("creation_time", v)?;
        }
        if let Some(v) = &self.keep_alive {
            map.serialize_entry("keep_alive", v)?;
        }
        if let Some(v) = &self.pit_id {
            map.serialize_entry("pit_id", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
