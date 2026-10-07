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

//! Response types of the `security_analytics_alerts` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct ActionExecutionResult {
    pub action_id: Option<String>,
    pub last_execution_time: Option<String>,
    pub throttled_count: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ActionExecutionResult {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ActionExecutionResult;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ActionExecutionResult object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_action_id: Option<String> = None;
                let mut seen_action_id = false;
                let mut f_last_execution_time: Option<String> = None;
                let mut seen_last_execution_time = false;
                let mut f_throttled_count: Option<i64> = None;
                let mut seen_throttled_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "action_id" => {
                            if seen_action_id {
                                return Err(serde::de::Error::duplicate_field("action_id"));
                            }
                            seen_action_id = true;
                            f_action_id = map.next_value::<Option<String>>()?;
                        }
                        "last_execution_time" => {
                            if seen_last_execution_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "last_execution_time",
                                ));
                            }
                            seen_last_execution_time = true;
                            f_last_execution_time = map.next_value::<Option<String>>()?;
                        }
                        "throttled_count" => {
                            if seen_throttled_count {
                                return Err(serde::de::Error::duplicate_field("throttled_count"));
                            }
                            seen_throttled_count = true;
                            f_throttled_count = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ActionExecutionResult {
                    action_id: f_action_id,
                    last_execution_time: f_last_execution_time,
                    throttled_count: f_throttled_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ActionExecutionResult {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.action_id {
            map.serialize_entry("action_id", v)?;
        }
        if let Some(v) = &self.last_execution_time {
            map.serialize_entry("last_execution_time", v)?;
        }
        if let Some(v) = &self.throttled_count {
            map.serialize_entry("throttled_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Alert {
    pub acknowledged_time: Option<String>,
    pub action_execution_results:
        Option<Vec<crate::types::security_analytics_alerts::ActionExecutionResult>>,
    pub alert_history: Option<Vec<crate::types::security_analytics_alerts::AlertError>>,
    pub detector_id: Option<String>,
    pub end_time: Option<String>,
    pub error_message: Option<String>,
    pub finding_ids: Option<Vec<String>>,
    pub id: Option<String>,
    pub last_notification_time: Option<String>,
    pub related_doc_ids: Option<Vec<String>>,
    pub schema_version: Option<i64>,
    pub severity: Option<String>,
    pub start_time: Option<String>,
    pub state: Option<crate::types::security_analytics_alerts::AlertState>,
    pub trigger_id: Option<String>,
    pub trigger_name: Option<String>,
    pub version: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Alert {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Alert;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Alert object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_acknowledged_time: Option<String> = None;
                let mut seen_acknowledged_time = false;
                let mut f_action_execution_results: Option<
                    Vec<crate::types::security_analytics_alerts::ActionExecutionResult>,
                > = None;
                let mut seen_action_execution_results = false;
                let mut f_alert_history: Option<
                    Vec<crate::types::security_analytics_alerts::AlertError>,
                > = None;
                let mut seen_alert_history = false;
                let mut f_detector_id: Option<String> = None;
                let mut seen_detector_id = false;
                let mut f_end_time: Option<String> = None;
                let mut seen_end_time = false;
                let mut f_error_message: Option<String> = None;
                let mut seen_error_message = false;
                let mut f_finding_ids: Option<Vec<String>> = None;
                let mut seen_finding_ids = false;
                let mut f_id: Option<String> = None;
                let mut seen_id = false;
                let mut f_last_notification_time: Option<String> = None;
                let mut seen_last_notification_time = false;
                let mut f_related_doc_ids: Option<Vec<String>> = None;
                let mut seen_related_doc_ids = false;
                let mut f_schema_version: Option<i64> = None;
                let mut seen_schema_version = false;
                let mut f_severity: Option<String> = None;
                let mut seen_severity = false;
                let mut f_start_time: Option<String> = None;
                let mut seen_start_time = false;
                let mut f_state: Option<crate::types::security_analytics_alerts::AlertState> = None;
                let mut seen_state = false;
                let mut f_trigger_id: Option<String> = None;
                let mut seen_trigger_id = false;
                let mut f_trigger_name: Option<String> = None;
                let mut seen_trigger_name = false;
                let mut f_version: Option<i64> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "acknowledged_time" => {
                            if seen_acknowledged_time {
                                return Err(serde::de::Error::duplicate_field("acknowledged_time"));
                            }
                            seen_acknowledged_time = true;
                            f_acknowledged_time = map.next_value::<Option<String>>()?;
                        }
                        "action_execution_results" => {
                            if seen_action_execution_results {
                                return Err(serde::de::Error::duplicate_field(
                                    "action_execution_results",
                                ));
                            }
                            seen_action_execution_results = true;
                            f_action_execution_results = map.next_value::<Option<
                                Vec<crate::types::security_analytics_alerts::ActionExecutionResult>,
                            >>()?;
                        }
                        "alert_history" => {
                            if seen_alert_history {
                                return Err(serde::de::Error::duplicate_field("alert_history"));
                            }
                            seen_alert_history = true;
                            f_alert_history =
                                map.next_value::<Option<
                                    Vec<crate::types::security_analytics_alerts::AlertError>,
                                >>()?;
                        }
                        "detector_id" => {
                            if seen_detector_id {
                                return Err(serde::de::Error::duplicate_field("detector_id"));
                            }
                            seen_detector_id = true;
                            f_detector_id = map.next_value::<Option<String>>()?;
                        }
                        "end_time" => {
                            if seen_end_time {
                                return Err(serde::de::Error::duplicate_field("end_time"));
                            }
                            seen_end_time = true;
                            f_end_time = map.next_value::<Option<String>>()?;
                        }
                        "error_message" => {
                            if seen_error_message {
                                return Err(serde::de::Error::duplicate_field("error_message"));
                            }
                            seen_error_message = true;
                            f_error_message = map.next_value::<Option<String>>()?;
                        }
                        "finding_ids" => {
                            if seen_finding_ids {
                                return Err(serde::de::Error::duplicate_field("finding_ids"));
                            }
                            seen_finding_ids = true;
                            f_finding_ids = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<String>>()?;
                        }
                        "last_notification_time" => {
                            if seen_last_notification_time {
                                return Err(serde::de::Error::duplicate_field(
                                    "last_notification_time",
                                ));
                            }
                            seen_last_notification_time = true;
                            f_last_notification_time = map.next_value::<Option<String>>()?;
                        }
                        "related_doc_ids" => {
                            if seen_related_doc_ids {
                                return Err(serde::de::Error::duplicate_field("related_doc_ids"));
                            }
                            seen_related_doc_ids = true;
                            f_related_doc_ids = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "schema_version" => {
                            if seen_schema_version {
                                return Err(serde::de::Error::duplicate_field("schema_version"));
                            }
                            seen_schema_version = true;
                            f_schema_version = map.next_value::<Option<i64>>()?;
                        }
                        "severity" => {
                            if seen_severity {
                                return Err(serde::de::Error::duplicate_field("severity"));
                            }
                            seen_severity = true;
                            f_severity = map.next_value::<Option<String>>()?;
                        }
                        "start_time" => {
                            if seen_start_time {
                                return Err(serde::de::Error::duplicate_field("start_time"));
                            }
                            seen_start_time = true;
                            f_start_time = map.next_value::<Option<String>>()?;
                        }
                        "state" => {
                            if seen_state {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            seen_state = true;
                            f_state = map.next_value::<Option<crate::types::security_analytics_alerts::AlertState>>()?;
                        }
                        "trigger_id" => {
                            if seen_trigger_id {
                                return Err(serde::de::Error::duplicate_field("trigger_id"));
                            }
                            seen_trigger_id = true;
                            f_trigger_id = map.next_value::<Option<String>>()?;
                        }
                        "trigger_name" => {
                            if seen_trigger_name {
                                return Err(serde::de::Error::duplicate_field("trigger_name"));
                            }
                            seen_trigger_name = true;
                            f_trigger_name = map.next_value::<Option<String>>()?;
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Alert {
                    acknowledged_time: f_acknowledged_time,
                    action_execution_results: f_action_execution_results,
                    alert_history: f_alert_history,
                    detector_id: f_detector_id,
                    end_time: f_end_time,
                    error_message: f_error_message,
                    finding_ids: f_finding_ids,
                    id: f_id,
                    last_notification_time: f_last_notification_time,
                    related_doc_ids: f_related_doc_ids,
                    schema_version: f_schema_version,
                    severity: f_severity,
                    start_time: f_start_time,
                    state: f_state,
                    trigger_id: f_trigger_id,
                    trigger_name: f_trigger_name,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Alert {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.acknowledged_time {
            map.serialize_entry("acknowledged_time", v)?;
        }
        if let Some(v) = &self.action_execution_results {
            map.serialize_entry("action_execution_results", v)?;
        }
        if let Some(v) = &self.alert_history {
            map.serialize_entry("alert_history", v)?;
        }
        if let Some(v) = &self.detector_id {
            map.serialize_entry("detector_id", v)?;
        }
        if let Some(v) = &self.end_time {
            map.serialize_entry("end_time", v)?;
        }
        if let Some(v) = &self.error_message {
            map.serialize_entry("error_message", v)?;
        }
        if let Some(v) = &self.finding_ids {
            map.serialize_entry("finding_ids", v)?;
        }
        if let Some(v) = &self.id {
            map.serialize_entry("id", v)?;
        }
        if let Some(v) = &self.last_notification_time {
            map.serialize_entry("last_notification_time", v)?;
        }
        if let Some(v) = &self.related_doc_ids {
            map.serialize_entry("related_doc_ids", v)?;
        }
        if let Some(v) = &self.schema_version {
            map.serialize_entry("schema_version", v)?;
        }
        if let Some(v) = &self.severity {
            map.serialize_entry("severity", v)?;
        }
        if let Some(v) = &self.start_time {
            map.serialize_entry("start_time", v)?;
        }
        if let Some(v) = &self.state {
            map.serialize_entry("state", v)?;
        }
        if let Some(v) = &self.trigger_id {
            map.serialize_entry("trigger_id", v)?;
        }
        if let Some(v) = &self.trigger_name {
            map.serialize_entry("trigger_name", v)?;
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

#[derive(Debug, Clone)]
pub struct AlertError {
    pub message: Option<String>,
    pub timestamp: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AlertError {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AlertError;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AlertError object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_message: Option<String> = None;
                let mut seen_message = false;
                let mut f_timestamp: Option<String> = None;
                let mut seen_timestamp = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "message" => {
                            if seen_message {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            seen_message = true;
                            f_message = map.next_value::<Option<String>>()?;
                        }
                        "timestamp" => {
                            if seen_timestamp {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            seen_timestamp = true;
                            f_timestamp = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AlertError {
                    message: f_message,
                    timestamp: f_timestamp,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AlertError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.message {
            map.serialize_entry("message", v)?;
        }
        if let Some(v) = &self.timestamp {
            map.serialize_entry("timestamp", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AlertState {
    #[doc = "`ACKNOWLEDGED`"]
    Acknowledged,
    #[doc = "`ACTIVE`"]
    Active,
    #[doc = "`COMPLETED`"]
    Completed,
    #[doc = "`DELETED`"]
    Deleted,
    #[doc = "`ERROR`"]
    Error,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl AlertState {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            AlertState::Acknowledged => "ACKNOWLEDGED",
            AlertState::Active => "ACTIVE",
            AlertState::Completed => "COMPLETED",
            AlertState::Deleted => "DELETED",
            AlertState::Error => "ERROR",
            AlertState::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for AlertState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AlertState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ACKNOWLEDGED" => AlertState::Acknowledged,
            "ACTIVE" => AlertState::Active,
            "COMPLETED" => AlertState::Completed,
            "DELETED" => AlertState::Deleted,
            "ERROR" => AlertState::Error,
            _ => AlertState::Unknown(s),
        })
    }
}

pub type Alerts = Vec<crate::types::security_analytics_alerts::Alert>;

#[derive(Debug, Clone)]
pub struct GetAlertsResponse {
    pub alerts: crate::types::security_analytics_alerts::Alerts,
    pub total_alerts: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GetAlertsResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GetAlertsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GetAlertsResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_alerts: Option<crate::types::security_analytics_alerts::Alerts> = None;
                let mut f_total_alerts: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "alerts" => {
                            if f_alerts.is_some() {
                                return Err(serde::de::Error::duplicate_field("alerts"));
                            }
                            f_alerts = Some(
                                map.next_value::<crate::types::security_analytics_alerts::Alerts>(
                                )?,
                            );
                        }
                        "total_alerts" => {
                            if f_total_alerts.is_some() {
                                return Err(serde::de::Error::duplicate_field("total_alerts"));
                            }
                            f_total_alerts = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GetAlertsResponse {
                    alerts: f_alerts.ok_or_else(|| serde::de::Error::missing_field("alerts"))?,
                    total_alerts: f_total_alerts
                        .ok_or_else(|| serde::de::Error::missing_field("total_alerts"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GetAlertsResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("alerts", &self.alerts)?;
        map.serialize_entry("total_alerts", &self.total_alerts)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
