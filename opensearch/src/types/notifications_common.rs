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

#[derive(Debug, Clone)]
pub struct Chime {
    pub url: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Chime {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Chime;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Chime object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_url: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "url" => {
                            if f_url.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            f_url = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Chime {
                    url: f_url.ok_or_else(|| serde::de::Error::missing_field("url"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Chime {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("url", &self.url)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct DeleteConfigsResponse {
    pub delete_response_list: Option<crate::types::notifications_common::DeleteResponseList>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DeleteConfigsResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DeleteConfigsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DeleteConfigsResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_delete_response_list: Option<
                    crate::types::notifications_common::DeleteResponseList,
                > = None;
                let mut seen_delete_response_list = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "delete_response_list" => {
                            if seen_delete_response_list {
                                return Err(serde::de::Error::duplicate_field(
                                    "delete_response_list",
                                ));
                            }
                            seen_delete_response_list = true;
                            f_delete_response_list = map.next_value::<Option<crate::types::notifications_common::DeleteResponseList>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DeleteConfigsResponse {
                    delete_response_list: f_delete_response_list,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DeleteConfigsResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.delete_response_list {
            map.serialize_entry("delete_response_list", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type DeleteResponseList =
    std::collections::BTreeMap<String, crate::types::notifications_common::RestStatus>;

#[derive(Debug, Clone)]
pub struct DeliveryStatus {
    pub status_code: Option<String>,
    pub status_text: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DeliveryStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DeliveryStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DeliveryStatus object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_status_code: Option<String> = None;
                let mut seen_status_code = false;
                let mut f_status_text: Option<String> = None;
                let mut seen_status_text = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "status_code" => {
                            if seen_status_code {
                                return Err(serde::de::Error::duplicate_field("status_code"));
                            }
                            seen_status_code = true;
                            f_status_code = map.next_value::<Option<String>>()?;
                        }
                        "status_text" => {
                            if seen_status_text {
                                return Err(serde::de::Error::duplicate_field("status_text"));
                            }
                            seen_status_text = true;
                            f_status_text = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DeliveryStatus {
                    status_code: f_status_code,
                    status_text: f_status_text,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DeliveryStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.status_code {
            map.serialize_entry("status_code", v)?;
        }
        if let Some(v) = &self.status_text {
            map.serialize_entry("status_text", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Email {
    pub email_account_id: String,
    pub recipient_list: Option<Vec<crate::types::notifications_common::RecipientListItem>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Email;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Email object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_email_account_id: Option<String> = None;
                let mut f_recipient_list: Option<
                    Vec<crate::types::notifications_common::RecipientListItem>,
                > = None;
                let mut seen_recipient_list = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "email_account_id" => {
                            if f_email_account_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("email_account_id"));
                            }
                            f_email_account_id = Some(map.next_value::<String>()?);
                        }
                        "recipient_list" => {
                            if seen_recipient_list {
                                return Err(serde::de::Error::duplicate_field("recipient_list"));
                            }
                            seen_recipient_list = true;
                            f_recipient_list = map.next_value::<Option<
                                Vec<crate::types::notifications_common::RecipientListItem>,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Email {
                    email_account_id: f_email_account_id
                        .ok_or_else(|| serde::de::Error::missing_field("email_account_id"))?,
                    recipient_list: f_recipient_list,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Email {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("email_account_id", &self.email_account_id)?;
        if let Some(v) = &self.recipient_list {
            map.serialize_entry("recipient_list", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EmailEncryptionMethod {
    #[doc = "`none`"]
    None,
    #[doc = "`ssl`"]
    Ssl,
    #[doc = "`start_tls`"]
    StartTls,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl EmailEncryptionMethod {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            EmailEncryptionMethod::None => "none",
            EmailEncryptionMethod::Ssl => "ssl",
            EmailEncryptionMethod::StartTls => "start_tls",
            EmailEncryptionMethod::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for EmailEncryptionMethod {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for EmailEncryptionMethod {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "none" => EmailEncryptionMethod::None,
            "ssl" => EmailEncryptionMethod::Ssl,
            "start_tls" => EmailEncryptionMethod::StartTls,
            _ => EmailEncryptionMethod::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct EmailGroup {
    pub email_group_id_list: Option<Vec<String>>,
    pub recipient_list: Vec<crate::types::notifications_common::RecipientListItem>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for EmailGroup {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = EmailGroup;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a EmailGroup object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_email_group_id_list: Option<Vec<String>> = None;
                let mut seen_email_group_id_list = false;
                let mut f_recipient_list: Option<
                    Vec<crate::types::notifications_common::RecipientListItem>,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "email_group_id_list" => {
                            if seen_email_group_id_list {
                                return Err(serde::de::Error::duplicate_field(
                                    "email_group_id_list",
                                ));
                            }
                            seen_email_group_id_list = true;
                            f_email_group_id_list = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "recipient_list" => {
                            if f_recipient_list.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient_list"));
                            }
                            f_recipient_list = Some(map.next_value::<Vec<crate::types::notifications_common::RecipientListItem>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(EmailGroup {
                    email_group_id_list: f_email_group_id_list,
                    recipient_list: f_recipient_list
                        .ok_or_else(|| serde::de::Error::missing_field("recipient_list"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for EmailGroup {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.email_group_id_list {
            map.serialize_entry("email_group_id_list", v)?;
        }
        map.serialize_entry("recipient_list", &self.recipient_list)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct EmailRecipientStatus {
    pub delivery_status: Option<crate::types::notifications_common::DeliveryStatus>,
    pub recipient: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for EmailRecipientStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = EmailRecipientStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a EmailRecipientStatus object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_delivery_status: Option<
                    crate::types::notifications_common::DeliveryStatus,
                > = None;
                let mut seen_delivery_status = false;
                let mut f_recipient: Option<String> = None;
                let mut seen_recipient = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "delivery_status" => {
                            if seen_delivery_status {
                                return Err(serde::de::Error::duplicate_field("delivery_status"));
                            }
                            seen_delivery_status = true;
                            f_delivery_status = map.next_value::<Option<crate::types::notifications_common::DeliveryStatus>>()?;
                        }
                        "recipient" => {
                            if seen_recipient {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            seen_recipient = true;
                            f_recipient = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(EmailRecipientStatus {
                    delivery_status: f_delivery_status,
                    recipient: f_recipient,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for EmailRecipientStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.delivery_status {
            map.serialize_entry("delivery_status", v)?;
        }
        if let Some(v) = &self.recipient {
            map.serialize_entry("recipient", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct EventSource {
    pub reference_id: Option<String>,
    pub severity: Option<crate::types::notifications_common::SeverityType>,
    pub tags: Option<Vec<String>>,
    pub title: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for EventSource {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = EventSource;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a EventSource object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_reference_id: Option<String> = None;
                let mut seen_reference_id = false;
                let mut f_severity: Option<crate::types::notifications_common::SeverityType> = None;
                let mut seen_severity = false;
                let mut f_tags: Option<Vec<String>> = None;
                let mut seen_tags = false;
                let mut f_title: Option<String> = None;
                let mut seen_title = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "reference_id" => {
                            if seen_reference_id {
                                return Err(serde::de::Error::duplicate_field("reference_id"));
                            }
                            seen_reference_id = true;
                            f_reference_id = map.next_value::<Option<String>>()?;
                        }
                        "severity" => {
                            if seen_severity {
                                return Err(serde::de::Error::duplicate_field("severity"));
                            }
                            seen_severity = true;
                            f_severity = map.next_value::<Option<crate::types::notifications_common::SeverityType>>()?;
                        }
                        "tags" => {
                            if seen_tags {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            seen_tags = true;
                            f_tags = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "title" => {
                            if seen_title {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            seen_title = true;
                            f_title = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(EventSource {
                    reference_id: f_reference_id,
                    severity: f_severity,
                    tags: f_tags,
                    title: f_title,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for EventSource {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.reference_id {
            map.serialize_entry("reference_id", v)?;
        }
        if let Some(v) = &self.severity {
            map.serialize_entry("severity", v)?;
        }
        if let Some(v) = &self.tags {
            map.serialize_entry("tags", v)?;
        }
        if let Some(v) = &self.title {
            map.serialize_entry("title", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct EventStatus {
    pub config_id: Option<String>,
    pub config_name: Option<String>,
    pub config_type: Option<crate::types::notifications_common::NotificationConfigType>,
    pub delivery_status: Option<crate::types::notifications_common::DeliveryStatus>,
    pub email_recipient_status:
        Option<Vec<crate::types::notifications_common::EmailRecipientStatus>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for EventStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = EventStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a EventStatus object")
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
                let mut f_config_name: Option<String> = None;
                let mut seen_config_name = false;
                let mut f_config_type: Option<
                    crate::types::notifications_common::NotificationConfigType,
                > = None;
                let mut seen_config_type = false;
                let mut f_delivery_status: Option<
                    crate::types::notifications_common::DeliveryStatus,
                > = None;
                let mut seen_delivery_status = false;
                let mut f_email_recipient_status: Option<
                    Vec<crate::types::notifications_common::EmailRecipientStatus>,
                > = None;
                let mut seen_email_recipient_status = false;
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
                        "config_name" => {
                            if seen_config_name {
                                return Err(serde::de::Error::duplicate_field("config_name"));
                            }
                            seen_config_name = true;
                            f_config_name = map.next_value::<Option<String>>()?;
                        }
                        "config_type" => {
                            if seen_config_type {
                                return Err(serde::de::Error::duplicate_field("config_type"));
                            }
                            seen_config_type = true;
                            f_config_type = map.next_value::<Option<
                                crate::types::notifications_common::NotificationConfigType,
                            >>()?;
                        }
                        "delivery_status" => {
                            if seen_delivery_status {
                                return Err(serde::de::Error::duplicate_field("delivery_status"));
                            }
                            seen_delivery_status = true;
                            f_delivery_status = map.next_value::<Option<crate::types::notifications_common::DeliveryStatus>>()?;
                        }
                        "email_recipient_status" => {
                            if seen_email_recipient_status {
                                return Err(serde::de::Error::duplicate_field(
                                    "email_recipient_status",
                                ));
                            }
                            seen_email_recipient_status = true;
                            f_email_recipient_status = map.next_value::<Option<
                                Vec<crate::types::notifications_common::EmailRecipientStatus>,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(EventStatus {
                    config_id: f_config_id,
                    config_name: f_config_name,
                    config_type: f_config_type,
                    delivery_status: f_delivery_status,
                    email_recipient_status: f_email_recipient_status,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for EventStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.config_id {
            map.serialize_entry("config_id", v)?;
        }
        if let Some(v) = &self.config_name {
            map.serialize_entry("config_name", v)?;
        }
        if let Some(v) = &self.config_type {
            map.serialize_entry("config_type", v)?;
        }
        if let Some(v) = &self.delivery_status {
            map.serialize_entry("delivery_status", v)?;
        }
        if let Some(v) = &self.email_recipient_status {
            map.serialize_entry("email_recipient_status", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct GetConfigsResponse {
    pub config_list:
        Option<Vec<crate::types::notifications_common::NotificationsConfigsOutputItem>>,
    pub start_index: Option<i64>,
    pub total_hit_relation: Option<crate::types::notifications_common::TotalHitRelation>,
    pub total_hits: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GetConfigsResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GetConfigsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GetConfigsResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_config_list: Option<
                    Vec<crate::types::notifications_common::NotificationsConfigsOutputItem>,
                > = None;
                let mut seen_config_list = false;
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
                        "config_list" => {
                            if seen_config_list {
                                return Err(serde::de::Error::duplicate_field("config_list"));
                            }
                            seen_config_list = true;
                            f_config_list = map.next_value::<Option<Vec<crate::types::notifications_common::NotificationsConfigsOutputItem>>>()?;
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
                Ok(GetConfigsResponse {
                    config_list: f_config_list,
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
impl serde::Serialize for GetConfigsResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.config_list {
            map.serialize_entry("config_list", v)?;
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

pub type HeaderParamsMap = std::collections::BTreeMap<String, i32>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HttpMethodType {
    #[doc = "`PATCH`"]
    Patch,
    #[doc = "`POST`"]
    Post,
    #[doc = "`PUT`"]
    Put,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl HttpMethodType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            HttpMethodType::Patch => "PATCH",
            HttpMethodType::Post => "POST",
            HttpMethodType::Put => "PUT",
            HttpMethodType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for HttpMethodType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HttpMethodType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "PATCH" => HttpMethodType::Patch,
            "POST" => HttpMethodType::Post,
            "PUT" => HttpMethodType::Put,
            _ => HttpMethodType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct MicrosoftTeamsItem {
    pub url: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MicrosoftTeamsItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MicrosoftTeamsItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MicrosoftTeamsItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_url: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "url" => {
                            if f_url.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            f_url = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MicrosoftTeamsItem {
                    url: f_url.ok_or_else(|| serde::de::Error::missing_field("url"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MicrosoftTeamsItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("url", &self.url)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NotificationChannel {
    pub config_id: Option<String>,
    pub config_type: Option<crate::types::notifications_common::NotificationConfigType>,
    pub description: Option<String>,
    pub is_enabled: Option<bool>,
    pub name: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NotificationChannel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NotificationChannel;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NotificationChannel object")
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
                let mut f_config_type: Option<
                    crate::types::notifications_common::NotificationConfigType,
                > = None;
                let mut seen_config_type = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_is_enabled: Option<bool> = None;
                let mut seen_is_enabled = false;
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
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
                        "config_type" => {
                            if seen_config_type {
                                return Err(serde::de::Error::duplicate_field("config_type"));
                            }
                            seen_config_type = true;
                            f_config_type = map.next_value::<Option<
                                crate::types::notifications_common::NotificationConfigType,
                            >>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "is_enabled" => {
                            if seen_is_enabled {
                                return Err(serde::de::Error::duplicate_field("is_enabled"));
                            }
                            seen_is_enabled = true;
                            f_is_enabled = map.next_value::<Option<bool>>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NotificationChannel {
                    config_id: f_config_id,
                    config_type: f_config_type,
                    description: f_description,
                    is_enabled: f_is_enabled,
                    name: f_name,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NotificationChannel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.config_id {
            map.serialize_entry("config_id", v)?;
        }
        if let Some(v) = &self.config_type {
            map.serialize_entry("config_type", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.is_enabled {
            map.serialize_entry("is_enabled", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Type of notification configuration."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NotificationConfigType {
    #[doc = "`chime`"]
    Chime,
    #[doc = "`email`"]
    Email,
    #[doc = "`email_group`"]
    EmailGroup,
    #[doc = "`microsoft_teams`"]
    MicrosoftTeams,
    #[doc = "`ses_account`"]
    SesAccount,
    #[doc = "`slack`"]
    Slack,
    #[doc = "`smtp_account`"]
    SmtpAccount,
    #[doc = "`sns`"]
    Sns,
    #[doc = "`webhook`"]
    Webhook,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl NotificationConfigType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            NotificationConfigType::Chime => "chime",
            NotificationConfigType::Email => "email",
            NotificationConfigType::EmailGroup => "email_group",
            NotificationConfigType::MicrosoftTeams => "microsoft_teams",
            NotificationConfigType::SesAccount => "ses_account",
            NotificationConfigType::Slack => "slack",
            NotificationConfigType::SmtpAccount => "smtp_account",
            NotificationConfigType::Sns => "sns",
            NotificationConfigType::Webhook => "webhook",
            NotificationConfigType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for NotificationConfigType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NotificationConfigType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "chime" => NotificationConfigType::Chime,
            "email" => NotificationConfigType::Email,
            "email_group" => NotificationConfigType::EmailGroup,
            "microsoft_teams" => NotificationConfigType::MicrosoftTeams,
            "ses_account" => NotificationConfigType::SesAccount,
            "slack" => NotificationConfigType::Slack,
            "smtp_account" => NotificationConfigType::SmtpAccount,
            "sns" => NotificationConfigType::Sns,
            "webhook" => NotificationConfigType::Webhook,
            _ => NotificationConfigType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct NotificationsConfigItem {
    pub chime: Option<crate::types::notifications_common::Chime>,
    pub config_type: crate::types::notifications_common::NotificationConfigType,
    pub description: Option<String>,
    pub email: Option<crate::types::notifications_common::Email>,
    pub email_group: Option<crate::types::notifications_common::EmailGroup>,
    pub is_enabled: Option<bool>,
    pub microsoft_teams: Option<crate::types::notifications_common::MicrosoftTeamsItem>,
    pub name: String,
    pub ses_account: Option<crate::types::notifications_common::SesAccount>,
    pub slack: Option<crate::types::notifications_common::SlackItem>,
    pub smtp_account: Option<crate::types::notifications_common::SmtpAccount>,
    pub sns: Option<crate::types::notifications_common::SnsItem>,
    pub webhook: Option<crate::types::notifications_common::Webhook>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NotificationsConfigItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NotificationsConfigItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NotificationsConfigItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_chime: Option<crate::types::notifications_common::Chime> = None;
                let mut seen_chime = false;
                let mut f_config_type: Option<
                    crate::types::notifications_common::NotificationConfigType,
                > = None;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_email: Option<crate::types::notifications_common::Email> = None;
                let mut seen_email = false;
                let mut f_email_group: Option<crate::types::notifications_common::EmailGroup> =
                    None;
                let mut seen_email_group = false;
                let mut f_is_enabled: Option<bool> = None;
                let mut seen_is_enabled = false;
                let mut f_microsoft_teams: Option<
                    crate::types::notifications_common::MicrosoftTeamsItem,
                > = None;
                let mut seen_microsoft_teams = false;
                let mut f_name: Option<String> = None;
                let mut f_ses_account: Option<crate::types::notifications_common::SesAccount> =
                    None;
                let mut seen_ses_account = false;
                let mut f_slack: Option<crate::types::notifications_common::SlackItem> = None;
                let mut seen_slack = false;
                let mut f_smtp_account: Option<crate::types::notifications_common::SmtpAccount> =
                    None;
                let mut seen_smtp_account = false;
                let mut f_sns: Option<crate::types::notifications_common::SnsItem> = None;
                let mut seen_sns = false;
                let mut f_webhook: Option<crate::types::notifications_common::Webhook> = None;
                let mut seen_webhook = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "chime" => {
                            if seen_chime {
                                return Err(serde::de::Error::duplicate_field("chime"));
                            }
                            seen_chime = true;
                            f_chime = map
                                .next_value::<Option<crate::types::notifications_common::Chime>>(
                                )?;
                        }
                        "config_type" => {
                            if f_config_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("config_type"));
                            }
                            f_config_type = Some(map.next_value::<crate::types::notifications_common::NotificationConfigType>()?);
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "email" => {
                            if seen_email {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            seen_email = true;
                            f_email = map
                                .next_value::<Option<crate::types::notifications_common::Email>>(
                                )?;
                        }
                        "email_group" => {
                            if seen_email_group {
                                return Err(serde::de::Error::duplicate_field("email_group"));
                            }
                            seen_email_group = true;
                            f_email_group = map.next_value::<Option<crate::types::notifications_common::EmailGroup>>()?;
                        }
                        "is_enabled" => {
                            if seen_is_enabled {
                                return Err(serde::de::Error::duplicate_field("is_enabled"));
                            }
                            seen_is_enabled = true;
                            f_is_enabled = map.next_value::<Option<bool>>()?;
                        }
                        "microsoft_teams" => {
                            if seen_microsoft_teams {
                                return Err(serde::de::Error::duplicate_field("microsoft_teams"));
                            }
                            seen_microsoft_teams = true;
                            f_microsoft_teams = map.next_value::<Option<crate::types::notifications_common::MicrosoftTeamsItem>>()?;
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<String>()?);
                        }
                        "ses_account" => {
                            if seen_ses_account {
                                return Err(serde::de::Error::duplicate_field("ses_account"));
                            }
                            seen_ses_account = true;
                            f_ses_account = map.next_value::<Option<crate::types::notifications_common::SesAccount>>()?;
                        }
                        "slack" => {
                            if seen_slack {
                                return Err(serde::de::Error::duplicate_field("slack"));
                            }
                            seen_slack = true;
                            f_slack = map.next_value::<Option<crate::types::notifications_common::SlackItem>>()?;
                        }
                        "smtp_account" => {
                            if seen_smtp_account {
                                return Err(serde::de::Error::duplicate_field("smtp_account"));
                            }
                            seen_smtp_account = true;
                            f_smtp_account = map.next_value::<Option<crate::types::notifications_common::SmtpAccount>>()?;
                        }
                        "sns" => {
                            if seen_sns {
                                return Err(serde::de::Error::duplicate_field("sns"));
                            }
                            seen_sns = true;
                            f_sns = map
                                .next_value::<Option<crate::types::notifications_common::SnsItem>>(
                                )?;
                        }
                        "webhook" => {
                            if seen_webhook {
                                return Err(serde::de::Error::duplicate_field("webhook"));
                            }
                            seen_webhook = true;
                            f_webhook = map
                                .next_value::<Option<crate::types::notifications_common::Webhook>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NotificationsConfigItem {
                    chime: f_chime,
                    config_type: f_config_type
                        .ok_or_else(|| serde::de::Error::missing_field("config_type"))?,
                    description: f_description,
                    email: f_email,
                    email_group: f_email_group,
                    is_enabled: f_is_enabled,
                    microsoft_teams: f_microsoft_teams,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    ses_account: f_ses_account,
                    slack: f_slack,
                    smtp_account: f_smtp_account,
                    sns: f_sns,
                    webhook: f_webhook,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NotificationsConfigItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.chime {
            map.serialize_entry("chime", v)?;
        }
        map.serialize_entry("config_type", &self.config_type)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.email {
            map.serialize_entry("email", v)?;
        }
        if let Some(v) = &self.email_group {
            map.serialize_entry("email_group", v)?;
        }
        if let Some(v) = &self.is_enabled {
            map.serialize_entry("is_enabled", v)?;
        }
        if let Some(v) = &self.microsoft_teams {
            map.serialize_entry("microsoft_teams", v)?;
        }
        map.serialize_entry("name", &self.name)?;
        if let Some(v) = &self.ses_account {
            map.serialize_entry("ses_account", v)?;
        }
        if let Some(v) = &self.slack {
            map.serialize_entry("slack", v)?;
        }
        if let Some(v) = &self.smtp_account {
            map.serialize_entry("smtp_account", v)?;
        }
        if let Some(v) = &self.sns {
            map.serialize_entry("sns", v)?;
        }
        if let Some(v) = &self.webhook {
            map.serialize_entry("webhook", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NotificationsConfigsOutputItem {
    pub config: Option<crate::types::notifications_common::NotificationsConfigItem>,
    pub config_id: Option<String>,
    pub created_time_ms: Option<i64>,
    pub last_updated_time_ms: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NotificationsConfigsOutputItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NotificationsConfigsOutputItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NotificationsConfigsOutputItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_config: Option<
                    crate::types::notifications_common::NotificationsConfigItem,
                > = None;
                let mut seen_config = false;
                let mut f_config_id: Option<String> = None;
                let mut seen_config_id = false;
                let mut f_created_time_ms: Option<i64> = None;
                let mut seen_created_time_ms = false;
                let mut f_last_updated_time_ms: Option<i64> = None;
                let mut seen_last_updated_time_ms = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "config" => {
                            if seen_config {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            seen_config = true;
                            f_config = map.next_value::<Option<
                                crate::types::notifications_common::NotificationsConfigItem,
                            >>()?;
                        }
                        "config_id" => {
                            if seen_config_id {
                                return Err(serde::de::Error::duplicate_field("config_id"));
                            }
                            seen_config_id = true;
                            f_config_id = map.next_value::<Option<String>>()?;
                        }
                        "created_time_ms" => {
                            if seen_created_time_ms {
                                return Err(serde::de::Error::duplicate_field("created_time_ms"));
                            }
                            seen_created_time_ms = true;
                            f_created_time_ms = map.next_value::<Option<i64>>()?;
                        }
                        "last_updated_time_ms" => {
                            if seen_last_updated_time_ms {
                                return Err(serde::de::Error::duplicate_field(
                                    "last_updated_time_ms",
                                ));
                            }
                            seen_last_updated_time_ms = true;
                            f_last_updated_time_ms = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NotificationsConfigsOutputItem {
                    config: f_config,
                    config_id: f_config_id,
                    created_time_ms: f_created_time_ms,
                    last_updated_time_ms: f_last_updated_time_ms,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NotificationsConfigsOutputItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.config {
            map.serialize_entry("config", v)?;
        }
        if let Some(v) = &self.config_id {
            map.serialize_entry("config_id", v)?;
        }
        if let Some(v) = &self.created_time_ms {
            map.serialize_entry("created_time_ms", v)?;
        }
        if let Some(v) = &self.last_updated_time_ms {
            map.serialize_entry("last_updated_time_ms", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type NotificationsPluginFeaturesMap = std::collections::BTreeMap<String, String>;

#[derive(Debug, Clone)]
pub struct RecipientListItem {
    pub recipient: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RecipientListItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RecipientListItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RecipientListItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_recipient: Option<String> = None;
                let mut seen_recipient = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "recipient" => {
                            if seen_recipient {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            seen_recipient = true;
                            f_recipient = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RecipientListItem {
                    recipient: f_recipient,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RecipientListItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.recipient {
            map.serialize_entry("recipient", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RestStatus {
    #[doc = "`ACCEPTED`"]
    Accepted,
    #[doc = "`CREATED`"]
    Created,
    #[doc = "`MULTI_STATUS`"]
    MultiStatus,
    #[doc = "`NON_AUTHORITATIVE_INFORMATION`"]
    NonAuthoritativeInformation,
    #[doc = "`NO_CONTENT`"]
    NoContent,
    #[doc = "`OK`"]
    Ok,
    #[doc = "`PARTIAL_CONTENT`"]
    PartialContent,
    #[doc = "`RESET_CONTENT`"]
    ResetContent,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl RestStatus {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            RestStatus::Accepted => "ACCEPTED",
            RestStatus::Created => "CREATED",
            RestStatus::MultiStatus => "MULTI_STATUS",
            RestStatus::NonAuthoritativeInformation => "NON_AUTHORITATIVE_INFORMATION",
            RestStatus::NoContent => "NO_CONTENT",
            RestStatus::Ok => "OK",
            RestStatus::PartialContent => "PARTIAL_CONTENT",
            RestStatus::ResetContent => "RESET_CONTENT",
            RestStatus::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for RestStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RestStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ACCEPTED" => RestStatus::Accepted,
            "CREATED" => RestStatus::Created,
            "MULTI_STATUS" => RestStatus::MultiStatus,
            "NON_AUTHORITATIVE_INFORMATION" => RestStatus::NonAuthoritativeInformation,
            "NO_CONTENT" => RestStatus::NoContent,
            "OK" => RestStatus::Ok,
            "PARTIAL_CONTENT" => RestStatus::PartialContent,
            "RESET_CONTENT" => RestStatus::ResetContent,
            _ => RestStatus::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SesAccount {
    pub from_address: String,
    pub region: String,
    pub role_arn: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SesAccount {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SesAccount;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SesAccount object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_from_address: Option<String> = None;
                let mut f_region: Option<String> = None;
                let mut f_role_arn: Option<String> = None;
                let mut seen_role_arn = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "from_address" => {
                            if f_from_address.is_some() {
                                return Err(serde::de::Error::duplicate_field("from_address"));
                            }
                            f_from_address = Some(map.next_value::<String>()?);
                        }
                        "region" => {
                            if f_region.is_some() {
                                return Err(serde::de::Error::duplicate_field("region"));
                            }
                            f_region = Some(map.next_value::<String>()?);
                        }
                        "role_arn" => {
                            if seen_role_arn {
                                return Err(serde::de::Error::duplicate_field("role_arn"));
                            }
                            seen_role_arn = true;
                            f_role_arn = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SesAccount {
                    from_address: f_from_address
                        .ok_or_else(|| serde::de::Error::missing_field("from_address"))?,
                    region: f_region.ok_or_else(|| serde::de::Error::missing_field("region"))?,
                    role_arn: f_role_arn,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SesAccount {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("from_address", &self.from_address)?;
        map.serialize_entry("region", &self.region)?;
        if let Some(v) = &self.role_arn {
            map.serialize_entry("role_arn", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SeverityType {
    #[doc = "`critical`"]
    Critical,
    #[doc = "`high`"]
    High,
    #[doc = "`info`"]
    Info,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SeverityType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SeverityType::Critical => "critical",
            SeverityType::High => "high",
            SeverityType::Info => "info",
            SeverityType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SeverityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SeverityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "critical" => SeverityType::Critical,
            "high" => SeverityType::High,
            "info" => SeverityType::Info,
            _ => SeverityType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SlackItem {
    pub url: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SlackItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SlackItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SlackItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_url: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "url" => {
                            if f_url.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            f_url = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SlackItem {
                    url: f_url.ok_or_else(|| serde::de::Error::missing_field("url"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SlackItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("url", &self.url)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SmtpAccount {
    pub from_address: String,
    pub host: String,
    pub method: crate::types::notifications_common::EmailEncryptionMethod,
    pub port: i32,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SmtpAccount {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SmtpAccount;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SmtpAccount object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_from_address: Option<String> = None;
                let mut f_host: Option<String> = None;
                let mut f_method: Option<
                    crate::types::notifications_common::EmailEncryptionMethod,
                > = None;
                let mut f_port: Option<i32> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "from_address" => {
                            if f_from_address.is_some() {
                                return Err(serde::de::Error::duplicate_field("from_address"));
                            }
                            f_from_address = Some(map.next_value::<String>()?);
                        }
                        "host" => {
                            if f_host.is_some() {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            f_host = Some(map.next_value::<String>()?);
                        }
                        "method" => {
                            if f_method.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            f_method = Some(map.next_value::<crate::types::notifications_common::EmailEncryptionMethod>()?);
                        }
                        "port" => {
                            if f_port.is_some() {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            f_port = Some(map.next_value::<i32>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SmtpAccount {
                    from_address: f_from_address
                        .ok_or_else(|| serde::de::Error::missing_field("from_address"))?,
                    host: f_host.ok_or_else(|| serde::de::Error::missing_field("host"))?,
                    method: f_method.ok_or_else(|| serde::de::Error::missing_field("method"))?,
                    port: f_port.ok_or_else(|| serde::de::Error::missing_field("port"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SmtpAccount {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("from_address", &self.from_address)?;
        map.serialize_entry("host", &self.host)?;
        map.serialize_entry("method", &self.method)?;
        map.serialize_entry("port", &self.port)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SnsItem {
    pub role_arn: Option<String>,
    pub topic_arn: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SnsItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SnsItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SnsItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_role_arn: Option<String> = None;
                let mut seen_role_arn = false;
                let mut f_topic_arn: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "role_arn" => {
                            if seen_role_arn {
                                return Err(serde::de::Error::duplicate_field("role_arn"));
                            }
                            seen_role_arn = true;
                            f_role_arn = map.next_value::<Option<String>>()?;
                        }
                        "topic_arn" => {
                            if f_topic_arn.is_some() {
                                return Err(serde::de::Error::duplicate_field("topic_arn"));
                            }
                            f_topic_arn = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SnsItem {
                    role_arn: f_role_arn,
                    topic_arn: f_topic_arn
                        .ok_or_else(|| serde::de::Error::missing_field("topic_arn"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SnsItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.role_arn {
            map.serialize_entry("role_arn", v)?;
        }
        map.serialize_entry("topic_arn", &self.topic_arn)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TotalHitRelation {
    #[doc = "`eq`"]
    Eq,
    #[doc = "`gte`"]
    Gte,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TotalHitRelation {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TotalHitRelation::Eq => "eq",
            TotalHitRelation::Gte => "gte",
            TotalHitRelation::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TotalHitRelation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TotalHitRelation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "eq" => TotalHitRelation::Eq,
            "gte" => TotalHitRelation::Gte,
            _ => TotalHitRelation::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Webhook {
    pub header_params: Option<crate::types::notifications_common::HeaderParamsMap>,
    pub method: Option<crate::types::notifications_common::HttpMethodType>,
    pub url: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Webhook {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Webhook;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Webhook object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_header_params: Option<
                    crate::types::notifications_common::HeaderParamsMap,
                > = None;
                let mut seen_header_params = false;
                let mut f_method: Option<crate::types::notifications_common::HttpMethodType> = None;
                let mut seen_method = false;
                let mut f_url: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "header_params" => {
                            if seen_header_params {
                                return Err(serde::de::Error::duplicate_field("header_params"));
                            }
                            seen_header_params = true;
                            f_header_params = map.next_value::<Option<crate::types::notifications_common::HeaderParamsMap>>()?;
                        }
                        "method" => {
                            if seen_method {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            seen_method = true;
                            f_method = map.next_value::<Option<crate::types::notifications_common::HttpMethodType>>()?;
                        }
                        "url" => {
                            if f_url.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            f_url = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Webhook {
                    header_params: f_header_params,
                    method: f_method,
                    url: f_url.ok_or_else(|| serde::de::Error::missing_field("url"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Webhook {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.header_params {
            map.serialize_entry("header_params", v)?;
        }
        if let Some(v) = &self.method {
            map.serialize_entry("method", v)?;
        }
        map.serialize_entry("url", &self.url)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
