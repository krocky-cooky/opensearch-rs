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

//! Response types of the `notifications` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct CreateConfigResponse {
    pub config_id: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CreateConfigResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CreateConfigResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CreateConfigResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_config_id: Option<String> = None;
                let mut seen_config_id = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "config_id" => {
                            if seen_config_id {
                                return Err(serde::de::Error::duplicate_field("config_id"));
                            }
                            seen_config_id = true;
                            f_config_id = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CreateConfigResponse {
                    config_id: f_config_id,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CreateConfigResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.config_id {
            map.serialize_entry("config_id", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ListChannelsResponse {
    pub channel_list: Option<Vec<crate::types::notifications_common::NotificationChannel>>,
    pub start_index: Option<i64>,
    pub total_hit_relation: Option<crate::types::notifications_common::TotalHitRelation>,
    pub total_hits: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ListChannelsResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ListChannelsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ListChannelsResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_channel_list: Option<
                    Vec<crate::types::notifications_common::NotificationChannel>,
                > = None;
                let mut seen_channel_list = false;
                let mut f_start_index: Option<i64> = None;
                let mut seen_start_index = false;
                let mut f_total_hit_relation: Option<
                    crate::types::notifications_common::TotalHitRelation,
                > = None;
                let mut seen_total_hit_relation = false;
                let mut f_total_hits: Option<i64> = None;
                let mut seen_total_hits = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "channel_list" => {
                            if seen_channel_list {
                                return Err(serde::de::Error::duplicate_field("channel_list"));
                            }
                            seen_channel_list = true;
                            f_channel_list = map.next_value::<Option<
                                Vec<crate::types::notifications_common::NotificationChannel>,
                            >>()?;
                        }
                        "start_index" => {
                            if seen_start_index {
                                return Err(serde::de::Error::duplicate_field("start_index"));
                            }
                            seen_start_index = true;
                            f_start_index = map.next_value::<Option<i64>>()?;
                        }
                        "total_hit_relation" => {
                            if seen_total_hit_relation {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_hit_relation",
                                ));
                            }
                            seen_total_hit_relation = true;
                            f_total_hit_relation = map.next_value::<Option<crate::types::notifications_common::TotalHitRelation>>()?;
                        }
                        "total_hits" => {
                            if seen_total_hits {
                                return Err(serde::de::Error::duplicate_field("total_hits"));
                            }
                            seen_total_hits = true;
                            f_total_hits = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ListChannelsResponse {
                    channel_list: f_channel_list,
                    start_index: f_start_index,
                    total_hit_relation: f_total_hit_relation,
                    total_hits: f_total_hits,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ListChannelsResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.channel_list {
            map.serialize_entry("channel_list", v)?;
        }
        if let Some(v) = &self.start_index {
            map.serialize_entry("start_index", v)?;
        }
        if let Some(v) = &self.total_hit_relation {
            map.serialize_entry("total_hit_relation", v)?;
        }
        if let Some(v) = &self.total_hits {
            map.serialize_entry("total_hits", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ListFeaturesResponse {
    pub allowed_config_type_list:
        Option<Vec<crate::types::notifications_common::NotificationConfigType>>,
    pub plugin_features: Option<crate::types::notifications_common::NotificationsPluginFeaturesMap>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ListFeaturesResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ListFeaturesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ListFeaturesResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_allowed_config_type_list: Option<
                    Vec<crate::types::notifications_common::NotificationConfigType>,
                > = None;
                let mut seen_allowed_config_type_list = false;
                let mut f_plugin_features: Option<
                    crate::types::notifications_common::NotificationsPluginFeaturesMap,
                > = None;
                let mut seen_plugin_features = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "allowed_config_type_list" => {
                            if seen_allowed_config_type_list {
                                return Err(serde::de::Error::duplicate_field(
                                    "allowed_config_type_list",
                                ));
                            }
                            seen_allowed_config_type_list = true;
                            f_allowed_config_type_list = map.next_value::<Option<
                                Vec<crate::types::notifications_common::NotificationConfigType>,
                            >>()?;
                        }
                        "plugin_features" => {
                            if seen_plugin_features {
                                return Err(serde::de::Error::duplicate_field("plugin_features"));
                            }
                            seen_plugin_features = true;
                            f_plugin_features = map.next_value::<Option<
                                crate::types::notifications_common::NotificationsPluginFeaturesMap,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ListFeaturesResponse {
                    allowed_config_type_list: f_allowed_config_type_list,
                    plugin_features: f_plugin_features,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ListFeaturesResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.allowed_config_type_list {
            map.serialize_entry("allowed_config_type_list", v)?;
        }
        if let Some(v) = &self.plugin_features {
            map.serialize_entry("plugin_features", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SendTestResponse {
    pub event_source: Option<crate::types::notifications_common::EventSource>,
    pub status_list: Option<Vec<crate::types::notifications_common::EventStatus>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SendTestResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SendTestResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SendTestResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_event_source: Option<crate::types::notifications_common::EventSource> =
                    None;
                let mut seen_event_source = false;
                let mut f_status_list: Option<
                    Vec<crate::types::notifications_common::EventStatus>,
                > = None;
                let mut seen_status_list = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "event_source" => {
                            if seen_event_source {
                                return Err(serde::de::Error::duplicate_field("event_source"));
                            }
                            seen_event_source = true;
                            f_event_source = map.next_value::<Option<crate::types::notifications_common::EventSource>>()?;
                        }
                        "status_list" => {
                            if seen_status_list {
                                return Err(serde::de::Error::duplicate_field("status_list"));
                            }
                            seen_status_list = true;
                            f_status_list = map.next_value::<Option<Vec<crate::types::notifications_common::EventStatus>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SendTestResponse {
                    event_source: f_event_source,
                    status_list: f_status_list,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SendTestResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.event_source {
            map.serialize_entry("event_source", v)?;
        }
        if let Some(v) = &self.status_list {
            map.serialize_entry("status_list", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct UpdateConfigResponse {
    pub config_id: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UpdateConfigResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UpdateConfigResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UpdateConfigResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_config_id: Option<String> = None;
                let mut seen_config_id = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "config_id" => {
                            if seen_config_id {
                                return Err(serde::de::Error::duplicate_field("config_id"));
                            }
                            seen_config_id = true;
                            f_config_id = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UpdateConfigResponse {
                    config_id: f_config_id,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UpdateConfigResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.config_id {
            map.serialize_entry("config_id", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
