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

//! Response types of the `ingestion_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ErrorPolicy {
    #[doc = "`BLOCK`"]
    Block,
    #[doc = "`DROP`"]
    Drop,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ErrorPolicy {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ErrorPolicy::Block => "BLOCK",
            ErrorPolicy::Drop => "DROP",
            ErrorPolicy::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ErrorPolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ErrorPolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "BLOCK" => ErrorPolicy::Block,
            "DROP" => ErrorPolicy::Drop,
            _ => ErrorPolicy::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetIngestionStateResponse {
    pub shards: Option<crate::types::common::ShardStatistics>,
    #[doc = "Ingestion state for a given index and its shards."]
    pub ingestion_state: Option<
        std::collections::BTreeMap<
            String,
            Vec<crate::types::ingestion_common::ShardIngestionState>,
        >,
    >,
    #[doc = "Specifies the next page token. Pass this token in the request to retrieve contents of the next page."]
    pub next_page_token: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GetIngestionStateResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GetIngestionStateResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GetIngestionStateResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_shards: Option<crate::types::common::ShardStatistics> = None;
                let mut seen_shards = false;
                let mut f_ingestion_state: Option<
                    std::collections::BTreeMap<
                        String,
                        Vec<crate::types::ingestion_common::ShardIngestionState>,
                    >,
                > = None;
                let mut seen_ingestion_state = false;
                let mut f_next_page_token: Option<String> = None;
                let mut seen_next_page_token = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_shards" => {
                            if seen_shards {
                                return Err(serde::de::Error::duplicate_field("_shards"));
                            }
                            seen_shards = true;
                            f_shards =
                                map.next_value::<Option<crate::types::common::ShardStatistics>>()?;
                        }
                        "ingestion_state" => {
                            if seen_ingestion_state {
                                return Err(serde::de::Error::duplicate_field("ingestion_state"));
                            }
                            seen_ingestion_state = true;
                            f_ingestion_state = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Vec<crate::types::ingestion_common::ShardIngestionState>,
                                >,
                            >>()?;
                        }
                        "next_page_token" => {
                            if seen_next_page_token {
                                return Err(serde::de::Error::duplicate_field("next_page_token"));
                            }
                            seen_next_page_token = true;
                            f_next_page_token = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GetIngestionStateResponse {
                    shards: f_shards,
                    ingestion_state: f_ingestion_state,
                    next_page_token: f_next_page_token,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GetIngestionStateResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.shards {
            map.serialize_entry("_shards", v)?;
        }
        if let Some(v) = &self.ingestion_state {
            map.serialize_entry("ingestion_state", v)?;
        }
        if let Some(v) = &self.next_page_token {
            map.serialize_entry("next_page_token", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IngestionStateShardFailure {
    #[doc = "Indicates the shard level error message."]
    pub error: String,
    #[doc = "The shard number."]
    pub shard: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IngestionStateShardFailure {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IngestionStateShardFailure;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IngestionStateShardFailure object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_error: Option<String> = None;
                let mut f_shard: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "error" => {
                            if f_error.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            f_error = Some(map.next_value::<String>()?);
                        }
                        "shard" => {
                            if f_shard.is_some() {
                                return Err(serde::de::Error::duplicate_field("shard"));
                            }
                            f_shard = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IngestionStateShardFailure {
                    error: f_error.ok_or_else(|| serde::de::Error::missing_field("error"))?,
                    shard: f_shard.ok_or_else(|| serde::de::Error::missing_field("shard"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IngestionStateShardFailure {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("error", &self.error)?;
        map.serialize_entry("shard", &self.shard)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PauseIngestionResponse {
    #[doc = "Indicates if the pause request has been acknowledged."]
    pub acknowledged: bool,
    pub error: Option<String>,
    pub failures: Option<Vec<crate::types::ingestion_common::IngestionStateShardFailure>>,
    #[doc = "Indicates if the pause request has been acknowledged by individual shards."]
    pub shards_acknowledged: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PauseIngestionResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PauseIngestionResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PauseIngestionResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_acknowledged: Option<bool> = None;
                let mut f_error: Option<String> = None;
                let mut seen_error = false;
                let mut f_failures: Option<
                    Vec<crate::types::ingestion_common::IngestionStateShardFailure>,
                > = None;
                let mut seen_failures = false;
                let mut f_shards_acknowledged: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "acknowledged" => {
                            if f_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field("acknowledged"));
                            }
                            f_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        "error" => {
                            if seen_error {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            seen_error = true;
                            f_error = map.next_value::<Option<String>>()?;
                        }
                        "failures" => {
                            if seen_failures {
                                return Err(serde::de::Error::duplicate_field("failures"));
                            }
                            seen_failures = true;
                            f_failures = map.next_value::<Option<
                                Vec<crate::types::ingestion_common::IngestionStateShardFailure>,
                            >>()?;
                        }
                        "shards_acknowledged" => {
                            if f_shards_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "shards_acknowledged",
                                ));
                            }
                            f_shards_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PauseIngestionResponse {
                    acknowledged: f_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("acknowledged"))?,
                    error: f_error,
                    failures: f_failures,
                    shards_acknowledged: f_shards_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("shards_acknowledged"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PauseIngestionResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("acknowledged", &self.acknowledged)?;
        if let Some(v) = &self.error {
            map.serialize_entry("error", v)?;
        }
        if let Some(v) = &self.failures {
            map.serialize_entry("failures", v)?;
        }
        map.serialize_entry("shards_acknowledged", &self.shards_acknowledged)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PollerState {
    #[doc = "`CLOSED`"]
    Closed,
    #[doc = "`NONE`"]
    None,
    #[doc = "`PAUSED`"]
    Paused,
    #[doc = "`POLLING`"]
    Polling,
    #[doc = "`PROCESSING`"]
    Processing,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PollerState {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PollerState::Closed => "CLOSED",
            PollerState::None => "NONE",
            PollerState::Paused => "PAUSED",
            PollerState::Polling => "POLLING",
            PollerState::Processing => "PROCESSING",
            PollerState::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PollerState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PollerState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "CLOSED" => PollerState::Closed,
            "NONE" => PollerState::None,
            "PAUSED" => PollerState::Paused,
            "POLLING" => PollerState::Polling,
            "PROCESSING" => PollerState::Processing,
            _ => PollerState::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ResumeIngestionResponse {
    #[doc = "Indicates if the resume request has been acknowledged."]
    pub acknowledged: bool,
    pub error: Option<String>,
    pub failures: Option<Vec<crate::types::ingestion_common::IngestionStateShardFailure>>,
    #[doc = "Indicates if the resume request has been acknowledged by individual shards."]
    pub shards_acknowledged: bool,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ResumeIngestionResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ResumeIngestionResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ResumeIngestionResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_acknowledged: Option<bool> = None;
                let mut f_error: Option<String> = None;
                let mut seen_error = false;
                let mut f_failures: Option<
                    Vec<crate::types::ingestion_common::IngestionStateShardFailure>,
                > = None;
                let mut seen_failures = false;
                let mut f_shards_acknowledged: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "acknowledged" => {
                            if f_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field("acknowledged"));
                            }
                            f_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        "error" => {
                            if seen_error {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            seen_error = true;
                            f_error = map.next_value::<Option<String>>()?;
                        }
                        "failures" => {
                            if seen_failures {
                                return Err(serde::de::Error::duplicate_field("failures"));
                            }
                            seen_failures = true;
                            f_failures = map.next_value::<Option<
                                Vec<crate::types::ingestion_common::IngestionStateShardFailure>,
                            >>()?;
                        }
                        "shards_acknowledged" => {
                            if f_shards_acknowledged.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "shards_acknowledged",
                                ));
                            }
                            f_shards_acknowledged = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ResumeIngestionResponse {
                    acknowledged: f_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("acknowledged"))?,
                    error: f_error,
                    failures: f_failures,
                    shards_acknowledged: f_shards_acknowledged
                        .ok_or_else(|| serde::de::Error::missing_field("shards_acknowledged"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ResumeIngestionResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("acknowledged", &self.acknowledged)?;
        if let Some(v) = &self.error {
            map.serialize_entry("error", v)?;
        }
        if let Some(v) = &self.failures {
            map.serialize_entry("failures", v)?;
        }
        map.serialize_entry("shards_acknowledged", &self.shards_acknowledged)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardIngestionState {
    #[doc = "Indicates the shard pointer from where the poller will resume during shard recovery."]
    pub batch_start_pointer: Option<String>,
    #[doc = "Indicates the policy used to handle failed messages."]
    pub error_policy: Option<crate::types::ingestion_common::ErrorPolicy>,
    #[doc = "Indicates if the poller has been paused."]
    pub poller_paused: Option<bool>,
    #[doc = "Indicates the current state of the poller."]
    pub poller_state: Option<crate::types::ingestion_common::PollerState>,
    #[doc = "The shard number."]
    pub shard: Option<i64>,
    #[doc = "Indicates if a write block has been created."]
    pub write_block_enabled: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardIngestionState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardIngestionState;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardIngestionState object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_batch_start_pointer: Option<String> = None;
                let mut seen_batch_start_pointer = false;
                let mut f_error_policy: Option<crate::types::ingestion_common::ErrorPolicy> = None;
                let mut seen_error_policy = false;
                let mut f_poller_paused: Option<bool> = None;
                let mut seen_poller_paused = false;
                let mut f_poller_state: Option<crate::types::ingestion_common::PollerState> = None;
                let mut seen_poller_state = false;
                let mut f_shard: Option<i64> = None;
                let mut seen_shard = false;
                let mut f_write_block_enabled: Option<bool> = None;
                let mut seen_write_block_enabled = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "batch_start_pointer" => {
                            if seen_batch_start_pointer {
                                return Err(serde::de::Error::duplicate_field(
                                    "batch_start_pointer",
                                ));
                            }
                            seen_batch_start_pointer = true;
                            f_batch_start_pointer = map.next_value::<Option<String>>()?;
                        }
                        "error_policy" => {
                            if seen_error_policy {
                                return Err(serde::de::Error::duplicate_field("error_policy"));
                            }
                            seen_error_policy = true;
                            f_error_policy = map
                                .next_value::<Option<crate::types::ingestion_common::ErrorPolicy>>(
                                )?;
                        }
                        "poller_paused" => {
                            if seen_poller_paused {
                                return Err(serde::de::Error::duplicate_field("poller_paused"));
                            }
                            seen_poller_paused = true;
                            f_poller_paused = map.next_value::<Option<bool>>()?;
                        }
                        "poller_state" => {
                            if seen_poller_state {
                                return Err(serde::de::Error::duplicate_field("poller_state"));
                            }
                            seen_poller_state = true;
                            f_poller_state = map
                                .next_value::<Option<crate::types::ingestion_common::PollerState>>(
                                )?;
                        }
                        "shard" => {
                            if seen_shard {
                                return Err(serde::de::Error::duplicate_field("shard"));
                            }
                            seen_shard = true;
                            f_shard = map.next_value::<Option<i64>>()?;
                        }
                        "write_block_enabled" => {
                            if seen_write_block_enabled {
                                return Err(serde::de::Error::duplicate_field(
                                    "write_block_enabled",
                                ));
                            }
                            seen_write_block_enabled = true;
                            f_write_block_enabled = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardIngestionState {
                    batch_start_pointer: f_batch_start_pointer,
                    error_policy: f_error_policy,
                    poller_paused: f_poller_paused,
                    poller_state: f_poller_state,
                    shard: f_shard,
                    write_block_enabled: f_write_block_enabled,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardIngestionState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.batch_start_pointer {
            map.serialize_entry("batch_start_pointer", v)?;
        }
        if let Some(v) = &self.error_policy {
            map.serialize_entry("error_policy", v)?;
        }
        if let Some(v) = &self.poller_paused {
            map.serialize_entry("poller_paused", v)?;
        }
        if let Some(v) = &self.poller_state {
            map.serialize_entry("poller_state", v)?;
        }
        if let Some(v) = &self.shard {
            map.serialize_entry("shard", v)?;
        }
        if let Some(v) = &self.write_block_enabled {
            map.serialize_entry("write_block_enabled", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
