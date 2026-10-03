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

//! Response types of the `notifications_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Chime {
    pub url: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeleteConfigsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_response_list: Option<crate::types::notifications_common::DeleteResponseList>,
}

pub type DeleteResponseList =
    std::collections::BTreeMap<String, crate::types::notifications_common::RestStatus>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeliveryStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_text: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Email {
    pub email_account_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient_list: Option<Vec<crate::types::notifications_common::RecipientListItem>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EmailEncryptionMethod {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "ssl")]
    Ssl,
    #[serde(rename = "start_tls")]
    StartTls,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmailGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_group_id_list: Option<Vec<String>>,
    pub recipient_list: Vec<crate::types::notifications_common::RecipientListItem>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmailRecipientStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<crate::types::notifications_common::DeliveryStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<crate::types::notifications_common::SeverityType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_type: Option<crate::types::notifications_common::NotificationConfigType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<crate::types::notifications_common::DeliveryStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_recipient_status:
        Option<Vec<crate::types::notifications_common::EmailRecipientStatus>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetConfigsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_list:
        Option<Vec<crate::types::notifications_common::NotificationsConfigsOutputItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_hit_relation: Option<crate::types::notifications_common::TotalHitRelation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_hits: Option<i64>,
}

pub type HeaderParamsMap = std::collections::BTreeMap<String, i32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum HttpMethodType {
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MicrosoftTeamsItem {
    pub url: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotificationChannel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_type: Option<crate::types::notifications_common::NotificationConfigType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[doc = "Type of notification configuration."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NotificationConfigType {
    #[serde(rename = "chime")]
    Chime,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "email_group")]
    EmailGroup,
    #[serde(rename = "microsoft_teams")]
    MicrosoftTeams,
    #[serde(rename = "ses_account")]
    SesAccount,
    #[serde(rename = "slack")]
    Slack,
    #[serde(rename = "smtp_account")]
    SmtpAccount,
    #[serde(rename = "sns")]
    Sns,
    #[serde(rename = "webhook")]
    Webhook,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotificationsConfigItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chime: Option<crate::types::notifications_common::Chime>,
    pub config_type: crate::types::notifications_common::NotificationConfigType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<crate::types::notifications_common::Email>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_group: Option<crate::types::notifications_common::EmailGroup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub microsoft_teams: Option<crate::types::notifications_common::MicrosoftTeamsItem>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ses_account: Option<crate::types::notifications_common::SesAccount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slack: Option<crate::types::notifications_common::SlackItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_account: Option<crate::types::notifications_common::SmtpAccount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sns: Option<crate::types::notifications_common::SnsItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<crate::types::notifications_common::Webhook>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotificationsConfigsOutputItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<crate::types::notifications_common::NotificationsConfigItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time_ms: Option<i64>,
}

pub type NotificationsPluginFeaturesMap = std::collections::BTreeMap<String, String>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecipientListItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RestStatus {
    #[serde(rename = "ACCEPTED")]
    Accepted,
    #[serde(rename = "CREATED")]
    Created,
    #[serde(rename = "MULTI_STATUS")]
    MultiStatus,
    #[serde(rename = "NON_AUTHORITATIVE_INFORMATION")]
    NonAuthoritativeInformation,
    #[serde(rename = "NO_CONTENT")]
    NoContent,
    #[serde(rename = "OK")]
    Ok,
    #[serde(rename = "PARTIAL_CONTENT")]
    PartialContent,
    #[serde(rename = "RESET_CONTENT")]
    ResetContent,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SesAccount {
    pub from_address: String,
    pub region: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SeverityType {
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "info")]
    Info,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SlackItem {
    pub url: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SmtpAccount {
    pub from_address: String,
    pub host: String,
    pub method: crate::types::notifications_common::EmailEncryptionMethod,
    pub port: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SnsItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    pub topic_arn: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TotalHitRelation {
    #[serde(rename = "eq")]
    Eq,
    #[serde(rename = "gte")]
    Gte,
    #[doc = "A value not defined by the specification; the original string is not preserved."]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Webhook {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header_params: Option<crate::types::notifications_common::HeaderParamsMap>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<crate::types::notifications_common::HttpMethodType>,
    pub url: String,
}
