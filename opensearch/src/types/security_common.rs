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

//! Response types of the `security_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct AccountDetails {
    pub backend_roles: Option<Vec<String>>,
    pub custom_attribute_names: Option<Vec<String>>,
    pub is_hidden: Option<bool>,
    pub is_internal_user: Option<bool>,
    pub is_reserved: Option<bool>,
    pub roles: Option<Vec<String>>,
    pub tenants: Option<crate::types::security_common::UserTenants>,
    pub user_name: Option<String>,
    pub user_requested_tenant: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AccountDetails {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AccountDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AccountDetails object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_backend_roles: Option<Vec<String>> = None;
                let mut seen_backend_roles = false;
                let mut f_custom_attribute_names: Option<Vec<String>> = None;
                let mut seen_custom_attribute_names = false;
                let mut f_is_hidden: Option<bool> = None;
                let mut seen_is_hidden = false;
                let mut f_is_internal_user: Option<bool> = None;
                let mut seen_is_internal_user = false;
                let mut f_is_reserved: Option<bool> = None;
                let mut seen_is_reserved = false;
                let mut f_roles: Option<Vec<String>> = None;
                let mut seen_roles = false;
                let mut f_tenants: Option<crate::types::security_common::UserTenants> = None;
                let mut seen_tenants = false;
                let mut f_user_name: Option<String> = None;
                let mut seen_user_name = false;
                let mut f_user_requested_tenant: Option<String> = None;
                let mut seen_user_requested_tenant = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "backend_roles" => {
                            if seen_backend_roles {
                                return Err(serde::de::Error::duplicate_field("backend_roles"));
                            }
                            seen_backend_roles = true;
                            f_backend_roles = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "custom_attribute_names" => {
                            if seen_custom_attribute_names {
                                return Err(serde::de::Error::duplicate_field(
                                    "custom_attribute_names",
                                ));
                            }
                            seen_custom_attribute_names = true;
                            f_custom_attribute_names = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "is_hidden" => {
                            if seen_is_hidden {
                                return Err(serde::de::Error::duplicate_field("is_hidden"));
                            }
                            seen_is_hidden = true;
                            f_is_hidden = map.next_value::<Option<bool>>()?;
                        }
                        "is_internal_user" => {
                            if seen_is_internal_user {
                                return Err(serde::de::Error::duplicate_field("is_internal_user"));
                            }
                            seen_is_internal_user = true;
                            f_is_internal_user = map.next_value::<Option<bool>>()?;
                        }
                        "is_reserved" => {
                            if seen_is_reserved {
                                return Err(serde::de::Error::duplicate_field("is_reserved"));
                            }
                            seen_is_reserved = true;
                            f_is_reserved = map.next_value::<Option<bool>>()?;
                        }
                        "roles" => {
                            if seen_roles {
                                return Err(serde::de::Error::duplicate_field("roles"));
                            }
                            seen_roles = true;
                            f_roles = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "tenants" => {
                            if seen_tenants {
                                return Err(serde::de::Error::duplicate_field("tenants"));
                            }
                            seen_tenants = true;
                            f_tenants = map
                                .next_value::<Option<crate::types::security_common::UserTenants>>(
                                )?;
                        }
                        "user_name" => {
                            if seen_user_name {
                                return Err(serde::de::Error::duplicate_field("user_name"));
                            }
                            seen_user_name = true;
                            f_user_name = map.next_value::<Option<String>>()?;
                        }
                        "user_requested_tenant" => {
                            if seen_user_requested_tenant {
                                return Err(serde::de::Error::duplicate_field(
                                    "user_requested_tenant",
                                ));
                            }
                            seen_user_requested_tenant = true;
                            f_user_requested_tenant = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AccountDetails {
                    backend_roles: f_backend_roles,
                    custom_attribute_names: f_custom_attribute_names,
                    is_hidden: f_is_hidden,
                    is_internal_user: f_is_internal_user,
                    is_reserved: f_is_reserved,
                    roles: f_roles,
                    tenants: f_tenants,
                    user_name: f_user_name,
                    user_requested_tenant: f_user_requested_tenant,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AccountDetails {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.backend_roles {
            map.serialize_entry("backend_roles", v)?;
        }
        if let Some(v) = &self.custom_attribute_names {
            map.serialize_entry("custom_attribute_names", v)?;
        }
        if let Some(v) = &self.is_hidden {
            map.serialize_entry("is_hidden", v)?;
        }
        if let Some(v) = &self.is_internal_user {
            map.serialize_entry("is_internal_user", v)?;
        }
        if let Some(v) = &self.is_reserved {
            map.serialize_entry("is_reserved", v)?;
        }
        if let Some(v) = &self.roles {
            map.serialize_entry("roles", v)?;
        }
        if let Some(v) = &self.tenants {
            map.serialize_entry("tenants", v)?;
        }
        if let Some(v) = &self.user_name {
            map.serialize_entry("user_name", v)?;
        }
        if let Some(v) = &self.user_requested_tenant {
            map.serialize_entry("user_requested_tenant", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ActionGroup {
    pub allowed_actions: Option<Vec<String>>,
    pub description: Option<String>,
    pub hidden: Option<bool>,
    pub reserved: Option<bool>,
    pub r#static: Option<bool>,
    pub r#type: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ActionGroup {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ActionGroup;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ActionGroup object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_allowed_actions: Option<Vec<String>> = None;
                let mut seen_allowed_actions = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_hidden: Option<bool> = None;
                let mut seen_hidden = false;
                let mut f_reserved: Option<bool> = None;
                let mut seen_reserved = false;
                let mut f_static: Option<bool> = None;
                let mut seen_static = false;
                let mut f_type: Option<String> = None;
                let mut seen_type = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "allowed_actions" => {
                            if seen_allowed_actions {
                                return Err(serde::de::Error::duplicate_field("allowed_actions"));
                            }
                            seen_allowed_actions = true;
                            f_allowed_actions = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "hidden" => {
                            if seen_hidden {
                                return Err(serde::de::Error::duplicate_field("hidden"));
                            }
                            seen_hidden = true;
                            f_hidden = map.next_value::<Option<bool>>()?;
                        }
                        "reserved" => {
                            if seen_reserved {
                                return Err(serde::de::Error::duplicate_field("reserved"));
                            }
                            seen_reserved = true;
                            f_reserved = map.next_value::<Option<bool>>()?;
                        }
                        "static" => {
                            if seen_static {
                                return Err(serde::de::Error::duplicate_field("static"));
                            }
                            seen_static = true;
                            f_static = map.next_value::<Option<bool>>()?;
                        }
                        "type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ActionGroup {
                    allowed_actions: f_allowed_actions,
                    description: f_description,
                    hidden: f_hidden,
                    reserved: f_reserved,
                    r#static: f_static,
                    r#type: f_type,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ActionGroup {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.allowed_actions {
            map.serialize_entry("allowed_actions", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.hidden {
            map.serialize_entry("hidden", v)?;
        }
        if let Some(v) = &self.reserved {
            map.serialize_entry("reserved", v)?;
        }
        if let Some(v) = &self.r#static {
            map.serialize_entry("static", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("type", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type ActionGroupsMap =
    std::collections::BTreeMap<String, crate::types::security_common::ActionGroup>;

#[derive(Debug, Clone)]
pub struct AllowListConfig {
    pub enabled: Option<bool>,
    #[doc = "An object with APIs as key and array of HTTP methods as values."]
    pub requests: Option<std::collections::BTreeMap<String, Vec<String>>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AllowListConfig {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AllowListConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AllowListConfig object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_enabled: Option<bool> = None;
                let mut seen_enabled = false;
                let mut f_requests: Option<std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut seen_requests = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "enabled" => {
                            if seen_enabled {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            seen_enabled = true;
                            f_enabled = map.next_value::<Option<bool>>()?;
                        }
                        "requests" => {
                            if seen_requests {
                                return Err(serde::de::Error::duplicate_field("requests"));
                            }
                            seen_requests = true;
                            f_requests = map.next_value::<Option<std::collections::BTreeMap<String, Vec<String>>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AllowListConfig {
                    enabled: f_enabled,
                    requests: f_requests,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AllowListConfig {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.enabled {
            map.serialize_entry("enabled", v)?;
        }
        if let Some(v) = &self.requests {
            map.serialize_entry("requests", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct AuditConfig {
    pub audit: Option<crate::types::security_common::AuditLogsConfig>,
    pub compliance: Option<crate::types::security_common::ComplianceConfig>,
    pub enabled: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AuditConfig {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AuditConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AuditConfig object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_audit: Option<crate::types::security_common::AuditLogsConfig> = None;
                let mut seen_audit = false;
                let mut f_compliance: Option<crate::types::security_common::ComplianceConfig> =
                    None;
                let mut seen_compliance = false;
                let mut f_enabled: Option<bool> = None;
                let mut seen_enabled = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "audit" => {
                            if seen_audit {
                                return Err(serde::de::Error::duplicate_field("audit"));
                            }
                            seen_audit = true;
                            f_audit = map.next_value::<Option<crate::types::security_common::AuditLogsConfig>>()?;
                        }
                        "compliance" => {
                            if seen_compliance {
                                return Err(serde::de::Error::duplicate_field("compliance"));
                            }
                            seen_compliance = true;
                            f_compliance = map.next_value::<Option<crate::types::security_common::ComplianceConfig>>()?;
                        }
                        "enabled" => {
                            if seen_enabled {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            seen_enabled = true;
                            f_enabled = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AuditConfig {
                    audit: f_audit,
                    compliance: f_compliance,
                    enabled: f_enabled,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AuditConfig {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.audit {
            map.serialize_entry("audit", v)?;
        }
        if let Some(v) = &self.compliance {
            map.serialize_entry("compliance", v)?;
        }
        if let Some(v) = &self.enabled {
            map.serialize_entry("enabled", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct AuditConfigWithReadOnly {
    pub readonly: Option<Vec<String>>,
    pub config: Option<crate::types::security_common::AuditConfig>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AuditConfigWithReadOnly {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AuditConfigWithReadOnly;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AuditConfigWithReadOnly object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_readonly: Option<Vec<String>> = None;
                let mut seen_readonly = false;
                let mut f_config: Option<crate::types::security_common::AuditConfig> = None;
                let mut seen_config = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_readonly" => {
                            if seen_readonly {
                                return Err(serde::de::Error::duplicate_field("_readonly"));
                            }
                            seen_readonly = true;
                            f_readonly = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "config" => {
                            if seen_config {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            seen_config = true;
                            f_config = map
                                .next_value::<Option<crate::types::security_common::AuditConfig>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AuditConfigWithReadOnly {
                    readonly: f_readonly,
                    config: f_config,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AuditConfigWithReadOnly {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.readonly {
            map.serialize_entry("_readonly", v)?;
        }
        if let Some(v) = &self.config {
            map.serialize_entry("config", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct AuditLogsConfig {
    pub disabled_rest_categories: Option<Vec<String>>,
    pub disabled_transport_categories: Option<Vec<String>>,
    pub enable_rest: Option<bool>,
    pub enable_transport: Option<bool>,
    pub exclude_sensitive_headers: Option<bool>,
    pub ignore_headers: Option<Vec<String>>,
    pub ignore_requests: Option<Vec<String>>,
    pub ignore_url_params: Option<Vec<String>>,
    pub ignore_users: Option<Vec<String>>,
    pub log_request_body: Option<bool>,
    pub resolve_bulk_requests: Option<bool>,
    pub resolve_indices: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AuditLogsConfig {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AuditLogsConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AuditLogsConfig object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_disabled_rest_categories: Option<Vec<String>> = None;
                let mut seen_disabled_rest_categories = false;
                let mut f_disabled_transport_categories: Option<Vec<String>> = None;
                let mut seen_disabled_transport_categories = false;
                let mut f_enable_rest: Option<bool> = None;
                let mut seen_enable_rest = false;
                let mut f_enable_transport: Option<bool> = None;
                let mut seen_enable_transport = false;
                let mut f_exclude_sensitive_headers: Option<bool> = None;
                let mut seen_exclude_sensitive_headers = false;
                let mut f_ignore_headers: Option<Vec<String>> = None;
                let mut seen_ignore_headers = false;
                let mut f_ignore_requests: Option<Vec<String>> = None;
                let mut seen_ignore_requests = false;
                let mut f_ignore_url_params: Option<Vec<String>> = None;
                let mut seen_ignore_url_params = false;
                let mut f_ignore_users: Option<Vec<String>> = None;
                let mut seen_ignore_users = false;
                let mut f_log_request_body: Option<bool> = None;
                let mut seen_log_request_body = false;
                let mut f_resolve_bulk_requests: Option<bool> = None;
                let mut seen_resolve_bulk_requests = false;
                let mut f_resolve_indices: Option<bool> = None;
                let mut seen_resolve_indices = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "disabled_rest_categories" => {
                            if seen_disabled_rest_categories {
                                return Err(serde::de::Error::duplicate_field(
                                    "disabled_rest_categories",
                                ));
                            }
                            seen_disabled_rest_categories = true;
                            f_disabled_rest_categories = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "disabled_transport_categories" => {
                            if seen_disabled_transport_categories {
                                return Err(serde::de::Error::duplicate_field(
                                    "disabled_transport_categories",
                                ));
                            }
                            seen_disabled_transport_categories = true;
                            f_disabled_transport_categories =
                                map.next_value::<Option<Vec<String>>>()?;
                        }
                        "enable_rest" => {
                            if seen_enable_rest {
                                return Err(serde::de::Error::duplicate_field("enable_rest"));
                            }
                            seen_enable_rest = true;
                            f_enable_rest = map.next_value::<Option<bool>>()?;
                        }
                        "enable_transport" => {
                            if seen_enable_transport {
                                return Err(serde::de::Error::duplicate_field("enable_transport"));
                            }
                            seen_enable_transport = true;
                            f_enable_transport = map.next_value::<Option<bool>>()?;
                        }
                        "exclude_sensitive_headers" => {
                            if seen_exclude_sensitive_headers {
                                return Err(serde::de::Error::duplicate_field(
                                    "exclude_sensitive_headers",
                                ));
                            }
                            seen_exclude_sensitive_headers = true;
                            f_exclude_sensitive_headers = map.next_value::<Option<bool>>()?;
                        }
                        "ignore_headers" => {
                            if seen_ignore_headers {
                                return Err(serde::de::Error::duplicate_field("ignore_headers"));
                            }
                            seen_ignore_headers = true;
                            f_ignore_headers = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "ignore_requests" => {
                            if seen_ignore_requests {
                                return Err(serde::de::Error::duplicate_field("ignore_requests"));
                            }
                            seen_ignore_requests = true;
                            f_ignore_requests = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "ignore_url_params" => {
                            if seen_ignore_url_params {
                                return Err(serde::de::Error::duplicate_field("ignore_url_params"));
                            }
                            seen_ignore_url_params = true;
                            f_ignore_url_params = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "ignore_users" => {
                            if seen_ignore_users {
                                return Err(serde::de::Error::duplicate_field("ignore_users"));
                            }
                            seen_ignore_users = true;
                            f_ignore_users = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "log_request_body" => {
                            if seen_log_request_body {
                                return Err(serde::de::Error::duplicate_field("log_request_body"));
                            }
                            seen_log_request_body = true;
                            f_log_request_body = map.next_value::<Option<bool>>()?;
                        }
                        "resolve_bulk_requests" => {
                            if seen_resolve_bulk_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "resolve_bulk_requests",
                                ));
                            }
                            seen_resolve_bulk_requests = true;
                            f_resolve_bulk_requests = map.next_value::<Option<bool>>()?;
                        }
                        "resolve_indices" => {
                            if seen_resolve_indices {
                                return Err(serde::de::Error::duplicate_field("resolve_indices"));
                            }
                            seen_resolve_indices = true;
                            f_resolve_indices = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AuditLogsConfig {
                    disabled_rest_categories: f_disabled_rest_categories,
                    disabled_transport_categories: f_disabled_transport_categories,
                    enable_rest: f_enable_rest,
                    enable_transport: f_enable_transport,
                    exclude_sensitive_headers: f_exclude_sensitive_headers,
                    ignore_headers: f_ignore_headers,
                    ignore_requests: f_ignore_requests,
                    ignore_url_params: f_ignore_url_params,
                    ignore_users: f_ignore_users,
                    log_request_body: f_log_request_body,
                    resolve_bulk_requests: f_resolve_bulk_requests,
                    resolve_indices: f_resolve_indices,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AuditLogsConfig {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.disabled_rest_categories {
            map.serialize_entry("disabled_rest_categories", v)?;
        }
        if let Some(v) = &self.disabled_transport_categories {
            map.serialize_entry("disabled_transport_categories", v)?;
        }
        if let Some(v) = &self.enable_rest {
            map.serialize_entry("enable_rest", v)?;
        }
        if let Some(v) = &self.enable_transport {
            map.serialize_entry("enable_transport", v)?;
        }
        if let Some(v) = &self.exclude_sensitive_headers {
            map.serialize_entry("exclude_sensitive_headers", v)?;
        }
        if let Some(v) = &self.ignore_headers {
            map.serialize_entry("ignore_headers", v)?;
        }
        if let Some(v) = &self.ignore_requests {
            map.serialize_entry("ignore_requests", v)?;
        }
        if let Some(v) = &self.ignore_url_params {
            map.serialize_entry("ignore_url_params", v)?;
        }
        if let Some(v) = &self.ignore_users {
            map.serialize_entry("ignore_users", v)?;
        }
        if let Some(v) = &self.log_request_body {
            map.serialize_entry("log_request_body", v)?;
        }
        if let Some(v) = &self.resolve_bulk_requests {
            map.serialize_entry("resolve_bulk_requests", v)?;
        }
        if let Some(v) = &self.resolve_indices {
            map.serialize_entry("resolve_indices", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct AuthInfo {
    #[doc = "The backend roles associated with the user."]
    pub backend_roles: Option<Vec<String>>,
    #[doc = "The name of the attributes associated with the user."]
    pub custom_attribute_names: Option<Vec<String>>,
    #[doc = "The number of peer certificates related to the user."]
    pub peer_certificates: Option<crate::types::security_common::AuthInfoPeerCertificates>,
    #[doc = "The user's principal."]
    pub principal: Option<String>,
    #[doc = "The IP address of remote user."]
    pub remote_address: Option<String>,
    #[doc = "The roles associated with the user."]
    pub roles: Option<Vec<String>>,
    #[doc = "The size of the user's backend roles in bytes."]
    pub size_of_backendroles: Option<String>,
    #[doc = "The size of the user's custom attributes in bytes."]
    pub size_of_custom_attributes: Option<String>,
    #[doc = "The size of user contained in memory."]
    pub size_of_user: Option<String>,
    #[doc = "The logout URL."]
    pub sso_logout_url: Option<String>,
    #[doc = "The tenants the user has access to with `read-write` or `read-only` access indicators."]
    pub tenants: Option<std::collections::BTreeMap<String, bool>>,
    #[doc = "A user object as a string."]
    pub user: Option<String>,
    #[doc = "The username."]
    pub user_name: Option<String>,
    #[doc = "The name of the tenant the user would like to switch to."]
    pub user_requested_tenant: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AuthInfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AuthInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AuthInfo object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_backend_roles: Option<Vec<String>> = None;
                let mut seen_backend_roles = false;
                let mut f_custom_attribute_names: Option<Vec<String>> = None;
                let mut seen_custom_attribute_names = false;
                let mut f_peer_certificates: Option<
                    crate::types::security_common::AuthInfoPeerCertificates,
                > = None;
                let mut seen_peer_certificates = false;
                let mut f_principal: Option<String> = None;
                let mut seen_principal = false;
                let mut f_remote_address: Option<String> = None;
                let mut seen_remote_address = false;
                let mut f_roles: Option<Vec<String>> = None;
                let mut seen_roles = false;
                let mut f_size_of_backendroles: Option<String> = None;
                let mut seen_size_of_backendroles = false;
                let mut f_size_of_custom_attributes: Option<String> = None;
                let mut seen_size_of_custom_attributes = false;
                let mut f_size_of_user: Option<String> = None;
                let mut seen_size_of_user = false;
                let mut f_sso_logout_url: Option<String> = None;
                let mut seen_sso_logout_url = false;
                let mut f_tenants: Option<std::collections::BTreeMap<String, bool>> = None;
                let mut seen_tenants = false;
                let mut f_user: Option<String> = None;
                let mut seen_user = false;
                let mut f_user_name: Option<String> = None;
                let mut seen_user_name = false;
                let mut f_user_requested_tenant: Option<String> = None;
                let mut seen_user_requested_tenant = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "backend_roles" => {
                            if seen_backend_roles {
                                return Err(serde::de::Error::duplicate_field("backend_roles"));
                            }
                            seen_backend_roles = true;
                            f_backend_roles = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "custom_attribute_names" => {
                            if seen_custom_attribute_names {
                                return Err(serde::de::Error::duplicate_field(
                                    "custom_attribute_names",
                                ));
                            }
                            seen_custom_attribute_names = true;
                            f_custom_attribute_names = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "peer_certificates" => {
                            if seen_peer_certificates {
                                return Err(serde::de::Error::duplicate_field("peer_certificates"));
                            }
                            seen_peer_certificates = true;
                            f_peer_certificates = map.next_value::<Option<crate::types::security_common::AuthInfoPeerCertificates>>()?;
                        }
                        "principal" => {
                            if seen_principal {
                                return Err(serde::de::Error::duplicate_field("principal"));
                            }
                            seen_principal = true;
                            f_principal = map.next_value::<Option<String>>()?;
                        }
                        "remote_address" => {
                            if seen_remote_address {
                                return Err(serde::de::Error::duplicate_field("remote_address"));
                            }
                            seen_remote_address = true;
                            f_remote_address = map.next_value::<Option<String>>()?;
                        }
                        "roles" => {
                            if seen_roles {
                                return Err(serde::de::Error::duplicate_field("roles"));
                            }
                            seen_roles = true;
                            f_roles = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "size_of_backendroles" => {
                            if seen_size_of_backendroles {
                                return Err(serde::de::Error::duplicate_field(
                                    "size_of_backendroles",
                                ));
                            }
                            seen_size_of_backendroles = true;
                            f_size_of_backendroles = map.next_value::<Option<String>>()?;
                        }
                        "size_of_custom_attributes" => {
                            if seen_size_of_custom_attributes {
                                return Err(serde::de::Error::duplicate_field(
                                    "size_of_custom_attributes",
                                ));
                            }
                            seen_size_of_custom_attributes = true;
                            f_size_of_custom_attributes = map.next_value::<Option<String>>()?;
                        }
                        "size_of_user" => {
                            if seen_size_of_user {
                                return Err(serde::de::Error::duplicate_field("size_of_user"));
                            }
                            seen_size_of_user = true;
                            f_size_of_user = map.next_value::<Option<String>>()?;
                        }
                        "sso_logout_url" => {
                            if seen_sso_logout_url {
                                return Err(serde::de::Error::duplicate_field("sso_logout_url"));
                            }
                            seen_sso_logout_url = true;
                            f_sso_logout_url = map.next_value::<Option<String>>()?;
                        }
                        "tenants" => {
                            if seen_tenants {
                                return Err(serde::de::Error::duplicate_field("tenants"));
                            }
                            seen_tenants = true;
                            f_tenants = map
                                .next_value::<Option<std::collections::BTreeMap<String, bool>>>()?;
                        }
                        "user" => {
                            if seen_user {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            seen_user = true;
                            f_user = map.next_value::<Option<String>>()?;
                        }
                        "user_name" => {
                            if seen_user_name {
                                return Err(serde::de::Error::duplicate_field("user_name"));
                            }
                            seen_user_name = true;
                            f_user_name = map.next_value::<Option<String>>()?;
                        }
                        "user_requested_tenant" => {
                            if seen_user_requested_tenant {
                                return Err(serde::de::Error::duplicate_field(
                                    "user_requested_tenant",
                                ));
                            }
                            seen_user_requested_tenant = true;
                            f_user_requested_tenant = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AuthInfo {
                    backend_roles: f_backend_roles,
                    custom_attribute_names: f_custom_attribute_names,
                    peer_certificates: f_peer_certificates,
                    principal: f_principal,
                    remote_address: f_remote_address,
                    roles: f_roles,
                    size_of_backendroles: f_size_of_backendroles,
                    size_of_custom_attributes: f_size_of_custom_attributes,
                    size_of_user: f_size_of_user,
                    sso_logout_url: f_sso_logout_url,
                    tenants: f_tenants,
                    user: f_user,
                    user_name: f_user_name,
                    user_requested_tenant: f_user_requested_tenant,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AuthInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.backend_roles {
            map.serialize_entry("backend_roles", v)?;
        }
        if let Some(v) = &self.custom_attribute_names {
            map.serialize_entry("custom_attribute_names", v)?;
        }
        if let Some(v) = &self.peer_certificates {
            map.serialize_entry("peer_certificates", v)?;
        }
        if let Some(v) = &self.principal {
            map.serialize_entry("principal", v)?;
        }
        if let Some(v) = &self.remote_address {
            map.serialize_entry("remote_address", v)?;
        }
        if let Some(v) = &self.roles {
            map.serialize_entry("roles", v)?;
        }
        if let Some(v) = &self.size_of_backendroles {
            map.serialize_entry("size_of_backendroles", v)?;
        }
        if let Some(v) = &self.size_of_custom_attributes {
            map.serialize_entry("size_of_custom_attributes", v)?;
        }
        if let Some(v) = &self.size_of_user {
            map.serialize_entry("size_of_user", v)?;
        }
        if let Some(v) = &self.sso_logout_url {
            map.serialize_entry("sso_logout_url", v)?;
        }
        if let Some(v) = &self.tenants {
            map.serialize_entry("tenants", v)?;
        }
        if let Some(v) = &self.user {
            map.serialize_entry("user", v)?;
        }
        if let Some(v) = &self.user_name {
            map.serialize_entry("user_name", v)?;
        }
        if let Some(v) = &self.user_requested_tenant {
            map.serialize_entry("user_requested_tenant", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The number of peer certificates related to the user."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum AuthInfoPeerCertificates {
    Number(f64),
    String(String),
}
impl<'de> serde::Deserialize<'de> for AuthInfoPeerCertificates {
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
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(AuthInfoPeerCertificates::Number)
            }
            Some(b'"') => serde_json::from_str(text).map(AuthInfoPeerCertificates::String),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a AuthInfoPeerCertificates value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct CertificateTypes {
    pub http: Option<
        Vec<std::collections::BTreeMap<String, crate::types::security_common::CertificatesDetail>>,
    >,
    pub transport: Option<
        Vec<std::collections::BTreeMap<String, crate::types::security_common::CertificatesDetail>>,
    >,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CertificateTypes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CertificateTypes;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CertificateTypes object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_http: Option<
                    Vec<
                        std::collections::BTreeMap<
                            String,
                            crate::types::security_common::CertificatesDetail,
                        >,
                    >,
                > = None;
                let mut seen_http = false;
                let mut f_transport: Option<
                    Vec<
                        std::collections::BTreeMap<
                            String,
                            crate::types::security_common::CertificatesDetail,
                        >,
                    >,
                > = None;
                let mut seen_transport = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "http" => {
                            if seen_http {
                                return Err(serde::de::Error::duplicate_field("http"));
                            }
                            seen_http = true;
                            f_http = map.next_value::<Option<
                                Vec<
                                    std::collections::BTreeMap<
                                        String,
                                        crate::types::security_common::CertificatesDetail,
                                    >,
                                >,
                            >>()?;
                        }
                        "transport" => {
                            if seen_transport {
                                return Err(serde::de::Error::duplicate_field("transport"));
                            }
                            seen_transport = true;
                            f_transport = map.next_value::<Option<
                                Vec<
                                    std::collections::BTreeMap<
                                        String,
                                        crate::types::security_common::CertificatesDetail,
                                    >,
                                >,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CertificateTypes {
                    http: f_http,
                    transport: f_transport,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CertificateTypes {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.http {
            map.serialize_entry("http", v)?;
        }
        if let Some(v) = &self.transport {
            map.serialize_entry("transport", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct CertificatesDetail {
    pub issuer_dn: Option<String>,
    pub not_after: Option<String>,
    pub not_before: Option<String>,
    pub san: Option<String>,
    pub subject_dn: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CertificatesDetail {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CertificatesDetail;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CertificatesDetail object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_issuer_dn: Option<String> = None;
                let mut seen_issuer_dn = false;
                let mut f_not_after: Option<String> = None;
                let mut seen_not_after = false;
                let mut f_not_before: Option<String> = None;
                let mut seen_not_before = false;
                let mut f_san: Option<String> = None;
                let mut seen_san = false;
                let mut f_subject_dn: Option<String> = None;
                let mut seen_subject_dn = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "issuer_dn" => {
                            if seen_issuer_dn {
                                return Err(serde::de::Error::duplicate_field("issuer_dn"));
                            }
                            seen_issuer_dn = true;
                            f_issuer_dn = map.next_value::<Option<String>>()?;
                        }
                        "not_after" => {
                            if seen_not_after {
                                return Err(serde::de::Error::duplicate_field("not_after"));
                            }
                            seen_not_after = true;
                            f_not_after = map.next_value::<Option<String>>()?;
                        }
                        "not_before" => {
                            if seen_not_before {
                                return Err(serde::de::Error::duplicate_field("not_before"));
                            }
                            seen_not_before = true;
                            f_not_before = map.next_value::<Option<String>>()?;
                        }
                        "san" => {
                            if seen_san {
                                return Err(serde::de::Error::duplicate_field("san"));
                            }
                            seen_san = true;
                            f_san = map.next_value::<Option<String>>()?;
                        }
                        "subject_dn" => {
                            if seen_subject_dn {
                                return Err(serde::de::Error::duplicate_field("subject_dn"));
                            }
                            seen_subject_dn = true;
                            f_subject_dn = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CertificatesDetail {
                    issuer_dn: f_issuer_dn,
                    not_after: f_not_after,
                    not_before: f_not_before,
                    san: f_san,
                    subject_dn: f_subject_dn,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CertificatesDetail {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.issuer_dn {
            map.serialize_entry("issuer_dn", v)?;
        }
        if let Some(v) = &self.not_after {
            map.serialize_entry("not_after", v)?;
        }
        if let Some(v) = &self.not_before {
            map.serialize_entry("not_before", v)?;
        }
        if let Some(v) = &self.san {
            map.serialize_entry("san", v)?;
        }
        if let Some(v) = &self.subject_dn {
            map.serialize_entry("subject_dn", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct CertificatesPerNode {
    pub certificates:
        Option<std::collections::BTreeMap<String, crate::types::security_common::CertificateTypes>>,
    #[doc = "The name of the node."]
    pub name: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CertificatesPerNode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CertificatesPerNode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CertificatesPerNode object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_certificates: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::security_common::CertificateTypes,
                    >,
                > = None;
                let mut seen_certificates = false;
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "certificates" => {
                            if seen_certificates {
                                return Err(serde::de::Error::duplicate_field("certificates"));
                            }
                            seen_certificates = true;
                            f_certificates = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::security_common::CertificateTypes,
                                >,
                            >>()?;
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
                Ok(CertificatesPerNode {
                    certificates: f_certificates,
                    name: f_name,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CertificatesPerNode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.certificates {
            map.serialize_entry("certificates", v)?;
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

#[derive(Debug, Clone)]
pub struct ComplianceConfig {
    pub enabled: Option<bool>,
    pub external_config: Option<bool>,
    pub internal_config: Option<bool>,
    pub read_ignore_users: Option<Vec<String>>,
    pub read_metadata_only: Option<bool>,
    pub read_watched_fields: Option<Box<serde_json::value::RawValue>>,
    pub write_ignore_users: Option<Vec<String>>,
    pub write_log_diffs: Option<bool>,
    pub write_metadata_only: Option<bool>,
    pub write_watched_indices: Option<Vec<String>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ComplianceConfig {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ComplianceConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ComplianceConfig object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_enabled: Option<bool> = None;
                let mut seen_enabled = false;
                let mut f_external_config: Option<bool> = None;
                let mut seen_external_config = false;
                let mut f_internal_config: Option<bool> = None;
                let mut seen_internal_config = false;
                let mut f_read_ignore_users: Option<Vec<String>> = None;
                let mut seen_read_ignore_users = false;
                let mut f_read_metadata_only: Option<bool> = None;
                let mut seen_read_metadata_only = false;
                let mut f_read_watched_fields: Option<Box<serde_json::value::RawValue>> = None;
                let mut seen_read_watched_fields = false;
                let mut f_write_ignore_users: Option<Vec<String>> = None;
                let mut seen_write_ignore_users = false;
                let mut f_write_log_diffs: Option<bool> = None;
                let mut seen_write_log_diffs = false;
                let mut f_write_metadata_only: Option<bool> = None;
                let mut seen_write_metadata_only = false;
                let mut f_write_watched_indices: Option<Vec<String>> = None;
                let mut seen_write_watched_indices = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "enabled" => {
                            if seen_enabled {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            seen_enabled = true;
                            f_enabled = map.next_value::<Option<bool>>()?;
                        }
                        "external_config" => {
                            if seen_external_config {
                                return Err(serde::de::Error::duplicate_field("external_config"));
                            }
                            seen_external_config = true;
                            f_external_config = map.next_value::<Option<bool>>()?;
                        }
                        "internal_config" => {
                            if seen_internal_config {
                                return Err(serde::de::Error::duplicate_field("internal_config"));
                            }
                            seen_internal_config = true;
                            f_internal_config = map.next_value::<Option<bool>>()?;
                        }
                        "read_ignore_users" => {
                            if seen_read_ignore_users {
                                return Err(serde::de::Error::duplicate_field("read_ignore_users"));
                            }
                            seen_read_ignore_users = true;
                            f_read_ignore_users = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "read_metadata_only" => {
                            if seen_read_metadata_only {
                                return Err(serde::de::Error::duplicate_field(
                                    "read_metadata_only",
                                ));
                            }
                            seen_read_metadata_only = true;
                            f_read_metadata_only = map.next_value::<Option<bool>>()?;
                        }
                        "read_watched_fields" => {
                            if seen_read_watched_fields {
                                return Err(serde::de::Error::duplicate_field(
                                    "read_watched_fields",
                                ));
                            }
                            seen_read_watched_fields = true;
                            f_read_watched_fields =
                                map.next_value::<Option<Box<serde_json::value::RawValue>>>()?;
                        }
                        "write_ignore_users" => {
                            if seen_write_ignore_users {
                                return Err(serde::de::Error::duplicate_field(
                                    "write_ignore_users",
                                ));
                            }
                            seen_write_ignore_users = true;
                            f_write_ignore_users = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "write_log_diffs" => {
                            if seen_write_log_diffs {
                                return Err(serde::de::Error::duplicate_field("write_log_diffs"));
                            }
                            seen_write_log_diffs = true;
                            f_write_log_diffs = map.next_value::<Option<bool>>()?;
                        }
                        "write_metadata_only" => {
                            if seen_write_metadata_only {
                                return Err(serde::de::Error::duplicate_field(
                                    "write_metadata_only",
                                ));
                            }
                            seen_write_metadata_only = true;
                            f_write_metadata_only = map.next_value::<Option<bool>>()?;
                        }
                        "write_watched_indices" => {
                            if seen_write_watched_indices {
                                return Err(serde::de::Error::duplicate_field(
                                    "write_watched_indices",
                                ));
                            }
                            seen_write_watched_indices = true;
                            f_write_watched_indices = map.next_value::<Option<Vec<String>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ComplianceConfig {
                    enabled: f_enabled,
                    external_config: f_external_config,
                    internal_config: f_internal_config,
                    read_ignore_users: f_read_ignore_users,
                    read_metadata_only: f_read_metadata_only,
                    read_watched_fields: f_read_watched_fields,
                    write_ignore_users: f_write_ignore_users,
                    write_log_diffs: f_write_log_diffs,
                    write_metadata_only: f_write_metadata_only,
                    write_watched_indices: f_write_watched_indices,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ComplianceConfig {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.enabled {
            map.serialize_entry("enabled", v)?;
        }
        if let Some(v) = &self.external_config {
            map.serialize_entry("external_config", v)?;
        }
        if let Some(v) = &self.internal_config {
            map.serialize_entry("internal_config", v)?;
        }
        if let Some(v) = &self.read_ignore_users {
            map.serialize_entry("read_ignore_users", v)?;
        }
        if let Some(v) = &self.read_metadata_only {
            map.serialize_entry("read_metadata_only", v)?;
        }
        if let Some(v) = &self.read_watched_fields {
            map.serialize_entry("read_watched_fields", v)?;
        }
        if let Some(v) = &self.write_ignore_users {
            map.serialize_entry("write_ignore_users", v)?;
        }
        if let Some(v) = &self.write_log_diffs {
            map.serialize_entry("write_log_diffs", v)?;
        }
        if let Some(v) = &self.write_metadata_only {
            map.serialize_entry("write_metadata_only", v)?;
        }
        if let Some(v) = &self.write_watched_indices {
            map.serialize_entry("write_watched_indices", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct DashboardsInfo {
    #[doc = "The default tenant setting for the dashboard."]
    pub default_tenant: Option<String>,
    #[doc = "Indicates whether multi-tenancy is enabled."]
    pub multitenancy_enabled: Option<bool>,
    #[doc = "Indicates whether `DNFOF` is enabled."]
    pub not_fail_on_forbidden_enabled: Option<bool>,
    #[doc = "The name of the dashboard's index."]
    pub opensearch_dashboards_index: Option<String>,
    #[doc = "Indicates whether multi-tenancy is enabled."]
    pub opensearch_dashboards_mt_enabled: Option<bool>,
    #[doc = "The name of the user used to connect dashboard's to the server."]
    pub opensearch_dashboards_server_user: Option<String>,
    #[doc = "The error message when a password validation fails."]
    pub password_validation_error_message: Option<String>,
    #[doc = "The regular expression used perform password validation."]
    pub password_validation_regex: Option<String>,
    #[doc = "Indicates whether a private tenant is enabled for all users."]
    pub private_tenant_enabled: Option<bool>,
    #[doc = "A list of available sign-in options."]
    pub sign_in_options: Option<Vec<String>>,
    #[doc = "User's name"]
    pub user_name: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DashboardsInfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DashboardsInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DashboardsInfo object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_default_tenant: Option<String> = None;
                let mut seen_default_tenant = false;
                let mut f_multitenancy_enabled: Option<bool> = None;
                let mut seen_multitenancy_enabled = false;
                let mut f_not_fail_on_forbidden_enabled: Option<bool> = None;
                let mut seen_not_fail_on_forbidden_enabled = false;
                let mut f_opensearch_dashboards_index: Option<String> = None;
                let mut seen_opensearch_dashboards_index = false;
                let mut f_opensearch_dashboards_mt_enabled: Option<bool> = None;
                let mut seen_opensearch_dashboards_mt_enabled = false;
                let mut f_opensearch_dashboards_server_user: Option<String> = None;
                let mut seen_opensearch_dashboards_server_user = false;
                let mut f_password_validation_error_message: Option<String> = None;
                let mut seen_password_validation_error_message = false;
                let mut f_password_validation_regex: Option<String> = None;
                let mut seen_password_validation_regex = false;
                let mut f_private_tenant_enabled: Option<bool> = None;
                let mut seen_private_tenant_enabled = false;
                let mut f_sign_in_options: Option<Vec<String>> = None;
                let mut seen_sign_in_options = false;
                let mut f_user_name: Option<String> = None;
                let mut seen_user_name = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "default_tenant" => {
                            if seen_default_tenant {
                                return Err(serde::de::Error::duplicate_field("default_tenant"));
                            }
                            seen_default_tenant = true;
                            f_default_tenant = map.next_value::<Option<String>>()?;
                        }
                        "multitenancy_enabled" => {
                            if seen_multitenancy_enabled {
                                return Err(serde::de::Error::duplicate_field(
                                    "multitenancy_enabled",
                                ));
                            }
                            seen_multitenancy_enabled = true;
                            f_multitenancy_enabled = map.next_value::<Option<bool>>()?;
                        }
                        "not_fail_on_forbidden_enabled" => {
                            if seen_not_fail_on_forbidden_enabled {
                                return Err(serde::de::Error::duplicate_field(
                                    "not_fail_on_forbidden_enabled",
                                ));
                            }
                            seen_not_fail_on_forbidden_enabled = true;
                            f_not_fail_on_forbidden_enabled = map.next_value::<Option<bool>>()?;
                        }
                        "opensearch_dashboards_index" => {
                            if seen_opensearch_dashboards_index {
                                return Err(serde::de::Error::duplicate_field(
                                    "opensearch_dashboards_index",
                                ));
                            }
                            seen_opensearch_dashboards_index = true;
                            f_opensearch_dashboards_index = map.next_value::<Option<String>>()?;
                        }
                        "opensearch_dashboards_mt_enabled" => {
                            if seen_opensearch_dashboards_mt_enabled {
                                return Err(serde::de::Error::duplicate_field(
                                    "opensearch_dashboards_mt_enabled",
                                ));
                            }
                            seen_opensearch_dashboards_mt_enabled = true;
                            f_opensearch_dashboards_mt_enabled =
                                map.next_value::<Option<bool>>()?;
                        }
                        "opensearch_dashboards_server_user" => {
                            if seen_opensearch_dashboards_server_user {
                                return Err(serde::de::Error::duplicate_field(
                                    "opensearch_dashboards_server_user",
                                ));
                            }
                            seen_opensearch_dashboards_server_user = true;
                            f_opensearch_dashboards_server_user =
                                map.next_value::<Option<String>>()?;
                        }
                        "password_validation_error_message" => {
                            if seen_password_validation_error_message {
                                return Err(serde::de::Error::duplicate_field(
                                    "password_validation_error_message",
                                ));
                            }
                            seen_password_validation_error_message = true;
                            f_password_validation_error_message =
                                map.next_value::<Option<String>>()?;
                        }
                        "password_validation_regex" => {
                            if seen_password_validation_regex {
                                return Err(serde::de::Error::duplicate_field(
                                    "password_validation_regex",
                                ));
                            }
                            seen_password_validation_regex = true;
                            f_password_validation_regex = map.next_value::<Option<String>>()?;
                        }
                        "private_tenant_enabled" => {
                            if seen_private_tenant_enabled {
                                return Err(serde::de::Error::duplicate_field(
                                    "private_tenant_enabled",
                                ));
                            }
                            seen_private_tenant_enabled = true;
                            f_private_tenant_enabled = map.next_value::<Option<bool>>()?;
                        }
                        "sign_in_options" => {
                            if seen_sign_in_options {
                                return Err(serde::de::Error::duplicate_field("sign_in_options"));
                            }
                            seen_sign_in_options = true;
                            f_sign_in_options = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "user_name" => {
                            if seen_user_name {
                                return Err(serde::de::Error::duplicate_field("user_name"));
                            }
                            seen_user_name = true;
                            f_user_name = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DashboardsInfo {
                    default_tenant: f_default_tenant,
                    multitenancy_enabled: f_multitenancy_enabled,
                    not_fail_on_forbidden_enabled: f_not_fail_on_forbidden_enabled,
                    opensearch_dashboards_index: f_opensearch_dashboards_index,
                    opensearch_dashboards_mt_enabled: f_opensearch_dashboards_mt_enabled,
                    opensearch_dashboards_server_user: f_opensearch_dashboards_server_user,
                    password_validation_error_message: f_password_validation_error_message,
                    password_validation_regex: f_password_validation_regex,
                    private_tenant_enabled: f_private_tenant_enabled,
                    sign_in_options: f_sign_in_options,
                    user_name: f_user_name,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DashboardsInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.default_tenant {
            map.serialize_entry("default_tenant", v)?;
        }
        if let Some(v) = &self.multitenancy_enabled {
            map.serialize_entry("multitenancy_enabled", v)?;
        }
        if let Some(v) = &self.not_fail_on_forbidden_enabled {
            map.serialize_entry("not_fail_on_forbidden_enabled", v)?;
        }
        if let Some(v) = &self.opensearch_dashboards_index {
            map.serialize_entry("opensearch_dashboards_index", v)?;
        }
        if let Some(v) = &self.opensearch_dashboards_mt_enabled {
            map.serialize_entry("opensearch_dashboards_mt_enabled", v)?;
        }
        if let Some(v) = &self.opensearch_dashboards_server_user {
            map.serialize_entry("opensearch_dashboards_server_user", v)?;
        }
        if let Some(v) = &self.password_validation_error_message {
            map.serialize_entry("password_validation_error_message", v)?;
        }
        if let Some(v) = &self.password_validation_regex {
            map.serialize_entry("password_validation_regex", v)?;
        }
        if let Some(v) = &self.private_tenant_enabled {
            map.serialize_entry("private_tenant_enabled", v)?;
        }
        if let Some(v) = &self.sign_in_options {
            map.serialize_entry("sign_in_options", v)?;
        }
        if let Some(v) = &self.user_name {
            map.serialize_entry("user_name", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct DistinguishedNames {
    pub nodes_dn: Option<Vec<String>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DistinguishedNames {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DistinguishedNames;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DistinguishedNames object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_nodes_dn: Option<Vec<String>> = None;
                let mut seen_nodes_dn = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "nodes_dn" => {
                            if seen_nodes_dn {
                                return Err(serde::de::Error::duplicate_field("nodes_dn"));
                            }
                            seen_nodes_dn = true;
                            f_nodes_dn = map.next_value::<Option<Vec<String>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DistinguishedNames {
                    nodes_dn: f_nodes_dn,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DistinguishedNames {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.nodes_dn {
            map.serialize_entry("nodes_dn", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type DistinguishedNamesMap =
    std::collections::BTreeMap<String, crate::types::security_common::DistinguishedNames>;

#[derive(Debug, Clone)]
pub struct DynamicConfig {
    pub dynamic: Option<crate::types::security_common::DynamicOptions>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DynamicConfig {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DynamicConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DynamicConfig object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dynamic: Option<crate::types::security_common::DynamicOptions> = None;
                let mut seen_dynamic = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dynamic" => {
                            if seen_dynamic {
                                return Err(serde::de::Error::duplicate_field("dynamic"));
                            }
                            seen_dynamic = true;
                            f_dynamic = map.next_value::<Option<crate::types::security_common::DynamicOptions>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DynamicConfig {
                    dynamic: f_dynamic,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DynamicConfig {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dynamic {
            map.serialize_entry("dynamic", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct DynamicOptions {
    pub auth_failure_listeners:
        Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub authc: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub authz: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub disable_intertransport_auth: Option<bool>,
    pub disable_rest_auth: Option<bool>,
    pub do_not_fail_on_forbidden: Option<bool>,
    pub do_not_fail_on_forbidden_empty: Option<bool>,
    pub filtered_alias_mode: Option<String>,
    pub hosts_resolver_mode: Option<String>,
    pub http: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub kibana: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub multi_rolespan_enabled: Option<bool>,
    pub on_behalf_of: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub opensearch_dashboards:
        Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub respect_request_indices_options: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DynamicOptions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DynamicOptions;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DynamicOptions object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_auth_failure_listeners: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_auth_failure_listeners = false;
                let mut f_authc: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_authc = false;
                let mut f_authz: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_authz = false;
                let mut f_disable_intertransport_auth: Option<bool> = None;
                let mut seen_disable_intertransport_auth = false;
                let mut f_disable_rest_auth: Option<bool> = None;
                let mut seen_disable_rest_auth = false;
                let mut f_do_not_fail_on_forbidden: Option<bool> = None;
                let mut seen_do_not_fail_on_forbidden = false;
                let mut f_do_not_fail_on_forbidden_empty: Option<bool> = None;
                let mut seen_do_not_fail_on_forbidden_empty = false;
                let mut f_filtered_alias_mode: Option<String> = None;
                let mut seen_filtered_alias_mode = false;
                let mut f_hosts_resolver_mode: Option<String> = None;
                let mut seen_hosts_resolver_mode = false;
                let mut f_http: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_http = false;
                let mut f_kibana: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_kibana = false;
                let mut f_multi_rolespan_enabled: Option<bool> = None;
                let mut seen_multi_rolespan_enabled = false;
                let mut f_on_behalf_of: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_on_behalf_of = false;
                let mut f_opensearch_dashboards: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_opensearch_dashboards = false;
                let mut f_respect_request_indices_options: Option<bool> = None;
                let mut seen_respect_request_indices_options = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "auth_failure_listeners" => {
                            if seen_auth_failure_listeners {
                                return Err(serde::de::Error::duplicate_field(
                                    "auth_failure_listeners",
                                ));
                            }
                            seen_auth_failure_listeners = true;
                            f_auth_failure_listeners = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "authc" => {
                            if seen_authc {
                                return Err(serde::de::Error::duplicate_field("authc"));
                            }
                            seen_authc = true;
                            f_authc = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "authz" => {
                            if seen_authz {
                                return Err(serde::de::Error::duplicate_field("authz"));
                            }
                            seen_authz = true;
                            f_authz = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "disable_intertransport_auth" => {
                            if seen_disable_intertransport_auth {
                                return Err(serde::de::Error::duplicate_field(
                                    "disable_intertransport_auth",
                                ));
                            }
                            seen_disable_intertransport_auth = true;
                            f_disable_intertransport_auth = map.next_value::<Option<bool>>()?;
                        }
                        "disable_rest_auth" => {
                            if seen_disable_rest_auth {
                                return Err(serde::de::Error::duplicate_field("disable_rest_auth"));
                            }
                            seen_disable_rest_auth = true;
                            f_disable_rest_auth = map.next_value::<Option<bool>>()?;
                        }
                        "do_not_fail_on_forbidden" => {
                            if seen_do_not_fail_on_forbidden {
                                return Err(serde::de::Error::duplicate_field(
                                    "do_not_fail_on_forbidden",
                                ));
                            }
                            seen_do_not_fail_on_forbidden = true;
                            f_do_not_fail_on_forbidden = map.next_value::<Option<bool>>()?;
                        }
                        "do_not_fail_on_forbidden_empty" => {
                            if seen_do_not_fail_on_forbidden_empty {
                                return Err(serde::de::Error::duplicate_field(
                                    "do_not_fail_on_forbidden_empty",
                                ));
                            }
                            seen_do_not_fail_on_forbidden_empty = true;
                            f_do_not_fail_on_forbidden_empty = map.next_value::<Option<bool>>()?;
                        }
                        "filtered_alias_mode" => {
                            if seen_filtered_alias_mode {
                                return Err(serde::de::Error::duplicate_field(
                                    "filtered_alias_mode",
                                ));
                            }
                            seen_filtered_alias_mode = true;
                            f_filtered_alias_mode = map.next_value::<Option<String>>()?;
                        }
                        "hosts_resolver_mode" => {
                            if seen_hosts_resolver_mode {
                                return Err(serde::de::Error::duplicate_field(
                                    "hosts_resolver_mode",
                                ));
                            }
                            seen_hosts_resolver_mode = true;
                            f_hosts_resolver_mode = map.next_value::<Option<String>>()?;
                        }
                        "http" => {
                            if seen_http {
                                return Err(serde::de::Error::duplicate_field("http"));
                            }
                            seen_http = true;
                            f_http = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "kibana" => {
                            if seen_kibana {
                                return Err(serde::de::Error::duplicate_field("kibana"));
                            }
                            seen_kibana = true;
                            f_kibana = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "multi_rolespan_enabled" => {
                            if seen_multi_rolespan_enabled {
                                return Err(serde::de::Error::duplicate_field(
                                    "multi_rolespan_enabled",
                                ));
                            }
                            seen_multi_rolespan_enabled = true;
                            f_multi_rolespan_enabled = map.next_value::<Option<bool>>()?;
                        }
                        "on_behalf_of" => {
                            if seen_on_behalf_of {
                                return Err(serde::de::Error::duplicate_field("on_behalf_of"));
                            }
                            seen_on_behalf_of = true;
                            f_on_behalf_of = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "opensearch-dashboards" => {
                            if seen_opensearch_dashboards {
                                return Err(serde::de::Error::duplicate_field(
                                    "opensearch-dashboards",
                                ));
                            }
                            seen_opensearch_dashboards = true;
                            f_opensearch_dashboards = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "respect_request_indices_options" => {
                            if seen_respect_request_indices_options {
                                return Err(serde::de::Error::duplicate_field(
                                    "respect_request_indices_options",
                                ));
                            }
                            seen_respect_request_indices_options = true;
                            f_respect_request_indices_options = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DynamicOptions {
                    auth_failure_listeners: f_auth_failure_listeners,
                    authc: f_authc,
                    authz: f_authz,
                    disable_intertransport_auth: f_disable_intertransport_auth,
                    disable_rest_auth: f_disable_rest_auth,
                    do_not_fail_on_forbidden: f_do_not_fail_on_forbidden,
                    do_not_fail_on_forbidden_empty: f_do_not_fail_on_forbidden_empty,
                    filtered_alias_mode: f_filtered_alias_mode,
                    hosts_resolver_mode: f_hosts_resolver_mode,
                    http: f_http,
                    kibana: f_kibana,
                    multi_rolespan_enabled: f_multi_rolespan_enabled,
                    on_behalf_of: f_on_behalf_of,
                    opensearch_dashboards: f_opensearch_dashboards,
                    respect_request_indices_options: f_respect_request_indices_options,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DynamicOptions {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.auth_failure_listeners {
            map.serialize_entry("auth_failure_listeners", v)?;
        }
        if let Some(v) = &self.authc {
            map.serialize_entry("authc", v)?;
        }
        if let Some(v) = &self.authz {
            map.serialize_entry("authz", v)?;
        }
        if let Some(v) = &self.disable_intertransport_auth {
            map.serialize_entry("disable_intertransport_auth", v)?;
        }
        if let Some(v) = &self.disable_rest_auth {
            map.serialize_entry("disable_rest_auth", v)?;
        }
        if let Some(v) = &self.do_not_fail_on_forbidden {
            map.serialize_entry("do_not_fail_on_forbidden", v)?;
        }
        if let Some(v) = &self.do_not_fail_on_forbidden_empty {
            map.serialize_entry("do_not_fail_on_forbidden_empty", v)?;
        }
        if let Some(v) = &self.filtered_alias_mode {
            map.serialize_entry("filtered_alias_mode", v)?;
        }
        if let Some(v) = &self.hosts_resolver_mode {
            map.serialize_entry("hosts_resolver_mode", v)?;
        }
        if let Some(v) = &self.http {
            map.serialize_entry("http", v)?;
        }
        if let Some(v) = &self.kibana {
            map.serialize_entry("kibana", v)?;
        }
        if let Some(v) = &self.multi_rolespan_enabled {
            map.serialize_entry("multi_rolespan_enabled", v)?;
        }
        if let Some(v) = &self.on_behalf_of {
            map.serialize_entry("on_behalf_of", v)?;
        }
        if let Some(v) = &self.opensearch_dashboards {
            map.serialize_entry("opensearch-dashboards", v)?;
        }
        if let Some(v) = &self.respect_request_indices_options {
            map.serialize_entry("respect_request_indices_options", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct GenerateOBOToken {
    #[doc = "The generated OBO token."]
    pub authentication_token: Option<String>,
    #[doc = "The duration of the token."]
    pub duration_seconds: Option<String>,
    #[doc = "The name of the entity requesting token."]
    pub user: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GenerateOBOToken {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GenerateOBOToken;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GenerateOBOToken object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_authentication_token: Option<String> = None;
                let mut seen_authentication_token = false;
                let mut f_duration_seconds: Option<String> = None;
                let mut seen_duration_seconds = false;
                let mut f_user: Option<String> = None;
                let mut seen_user = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "authenticationToken" => {
                            if seen_authentication_token {
                                return Err(serde::de::Error::duplicate_field(
                                    "authenticationToken",
                                ));
                            }
                            seen_authentication_token = true;
                            f_authentication_token = map.next_value::<Option<String>>()?;
                        }
                        "durationSeconds" => {
                            if seen_duration_seconds {
                                return Err(serde::de::Error::duplicate_field("durationSeconds"));
                            }
                            seen_duration_seconds = true;
                            f_duration_seconds = map.next_value::<Option<String>>()?;
                        }
                        "user" => {
                            if seen_user {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            seen_user = true;
                            f_user = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GenerateOBOToken {
                    authentication_token: f_authentication_token,
                    duration_seconds: f_duration_seconds,
                    user: f_user,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GenerateOBOToken {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.authentication_token {
            map.serialize_entry("authenticationToken", v)?;
        }
        if let Some(v) = &self.duration_seconds {
            map.serialize_entry("durationSeconds", v)?;
        }
        if let Some(v) = &self.user {
            map.serialize_entry("user", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct GetCertificates {
    pub http_certificates_list: Option<Vec<crate::types::security_common::CertificatesDetail>>,
    pub transport_certificates_list: Option<Vec<crate::types::security_common::CertificatesDetail>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GetCertificates {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GetCertificates;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GetCertificates object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_http_certificates_list: Option<
                    Vec<crate::types::security_common::CertificatesDetail>,
                > = None;
                let mut seen_http_certificates_list = false;
                let mut f_transport_certificates_list: Option<
                    Vec<crate::types::security_common::CertificatesDetail>,
                > = None;
                let mut seen_transport_certificates_list = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "http_certificates_list" => {
                            if seen_http_certificates_list {
                                return Err(serde::de::Error::duplicate_field(
                                    "http_certificates_list",
                                ));
                            }
                            seen_http_certificates_list = true;
                            f_http_certificates_list = map.next_value::<Option<Vec<crate::types::security_common::CertificatesDetail>>>()?;
                        }
                        "transport_certificates_list" => {
                            if seen_transport_certificates_list {
                                return Err(serde::de::Error::duplicate_field(
                                    "transport_certificates_list",
                                ));
                            }
                            seen_transport_certificates_list = true;
                            f_transport_certificates_list = map.next_value::<Option<Vec<crate::types::security_common::CertificatesDetail>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GetCertificates {
                    http_certificates_list: f_http_certificates_list,
                    transport_certificates_list: f_transport_certificates_list,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GetCertificates {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.http_certificates_list {
            map.serialize_entry("http_certificates_list", v)?;
        }
        if let Some(v) = &self.transport_certificates_list {
            map.serialize_entry("transport_certificates_list", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct GetCertificatesNew {
    #[doc = "Contains statistics about the number of nodes selected by the request's node filters."]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    #[doc = "Name of this cluster."]
    pub cluster_name: Option<String>,
    pub _nodes: Option<
        std::collections::BTreeMap<String, crate::types::security_common::CertificatesPerNode>,
    >,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GetCertificatesNew {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GetCertificatesNew;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GetCertificatesNew object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_nodes: Option<crate::types::common::NodeStatistics> = None;
                let mut seen_nodes = false;
                let mut f_cluster_name: Option<String> = None;
                let mut seen_cluster_name = false;
                let mut f__nodes: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::security_common::CertificatesPerNode,
                    >,
                > = None;
                let mut seen__nodes = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_nodes" => {
                            if seen_nodes {
                                return Err(serde::de::Error::duplicate_field("_nodes"));
                            }
                            seen_nodes = true;
                            f_nodes =
                                map.next_value::<Option<crate::types::common::NodeStatistics>>()?;
                        }
                        "cluster_name" => {
                            if seen_cluster_name {
                                return Err(serde::de::Error::duplicate_field("cluster_name"));
                            }
                            seen_cluster_name = true;
                            f_cluster_name = map.next_value::<Option<String>>()?;
                        }
                        "nodes" => {
                            if seen__nodes {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            seen__nodes = true;
                            f__nodes = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::security_common::CertificatesPerNode,
                                >,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GetCertificatesNew {
                    nodes: f_nodes,
                    cluster_name: f_cluster_name,
                    _nodes: f__nodes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GetCertificatesNew {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.nodes {
            map.serialize_entry("_nodes", v)?;
        }
        if let Some(v) = &self.cluster_name {
            map.serialize_entry("cluster_name", v)?;
        }
        if let Some(v) = &self._nodes {
            map.serialize_entry("nodes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct HealthInfo {
    pub message: Option<String>,
    pub mode: Option<String>,
    pub settings: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub status: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HealthInfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HealthInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HealthInfo object")
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
                let mut f_mode: Option<String> = None;
                let mut seen_mode = false;
                let mut f_settings: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_settings = false;
                let mut f_status: Option<String> = None;
                let mut seen_status = false;
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
                        "mode" => {
                            if seen_mode {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            seen_mode = true;
                            f_mode = map.next_value::<Option<String>>()?;
                        }
                        "settings" => {
                            if seen_settings {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            seen_settings = true;
                            f_settings = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "status" => {
                            if seen_status {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            seen_status = true;
                            f_status = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HealthInfo {
                    message: f_message,
                    mode: f_mode,
                    settings: f_settings,
                    status: f_status,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HealthInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.message {
            map.serialize_entry("message", v)?;
        }
        if let Some(v) = &self.mode {
            map.serialize_entry("mode", v)?;
        }
        if let Some(v) = &self.settings {
            map.serialize_entry("settings", v)?;
        }
        if let Some(v) = &self.status {
            map.serialize_entry("status", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IndexPermission {
    pub allowed_actions: Option<Vec<String>>,
    pub dls: Option<String>,
    pub fls: Option<Vec<String>>,
    pub index_patterns: Option<Vec<String>>,
    pub masked_fields: Option<Vec<String>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexPermission {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexPermission;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexPermission object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_allowed_actions: Option<Vec<String>> = None;
                let mut seen_allowed_actions = false;
                let mut f_dls: Option<String> = None;
                let mut seen_dls = false;
                let mut f_fls: Option<Vec<String>> = None;
                let mut seen_fls = false;
                let mut f_index_patterns: Option<Vec<String>> = None;
                let mut seen_index_patterns = false;
                let mut f_masked_fields: Option<Vec<String>> = None;
                let mut seen_masked_fields = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "allowed_actions" => {
                            if seen_allowed_actions {
                                return Err(serde::de::Error::duplicate_field("allowed_actions"));
                            }
                            seen_allowed_actions = true;
                            f_allowed_actions = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "dls" => {
                            if seen_dls {
                                return Err(serde::de::Error::duplicate_field("dls"));
                            }
                            seen_dls = true;
                            f_dls = map.next_value::<Option<String>>()?;
                        }
                        "fls" => {
                            if seen_fls {
                                return Err(serde::de::Error::duplicate_field("fls"));
                            }
                            seen_fls = true;
                            f_fls = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "index_patterns" => {
                            if seen_index_patterns {
                                return Err(serde::de::Error::duplicate_field("index_patterns"));
                            }
                            seen_index_patterns = true;
                            f_index_patterns = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "masked_fields" => {
                            if seen_masked_fields {
                                return Err(serde::de::Error::duplicate_field("masked_fields"));
                            }
                            seen_masked_fields = true;
                            f_masked_fields = map.next_value::<Option<Vec<String>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexPermission {
                    allowed_actions: f_allowed_actions,
                    dls: f_dls,
                    fls: f_fls,
                    index_patterns: f_index_patterns,
                    masked_fields: f_masked_fields,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexPermission {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.allowed_actions {
            map.serialize_entry("allowed_actions", v)?;
        }
        if let Some(v) = &self.dls {
            map.serialize_entry("dls", v)?;
        }
        if let Some(v) = &self.fls {
            map.serialize_entry("fls", v)?;
        }
        if let Some(v) = &self.index_patterns {
            map.serialize_entry("index_patterns", v)?;
        }
        if let Some(v) = &self.masked_fields {
            map.serialize_entry("masked_fields", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct MultiTenancyConfig {
    pub default_tenant: Option<String>,
    pub multitenancy_enabled: Option<bool>,
    pub private_tenant_enabled: Option<bool>,
    pub sign_in_options: Option<Vec<String>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MultiTenancyConfig {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MultiTenancyConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MultiTenancyConfig object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_default_tenant: Option<String> = None;
                let mut seen_default_tenant = false;
                let mut f_multitenancy_enabled: Option<bool> = None;
                let mut seen_multitenancy_enabled = false;
                let mut f_private_tenant_enabled: Option<bool> = None;
                let mut seen_private_tenant_enabled = false;
                let mut f_sign_in_options: Option<Vec<String>> = None;
                let mut seen_sign_in_options = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "default_tenant" => {
                            if seen_default_tenant {
                                return Err(serde::de::Error::duplicate_field("default_tenant"));
                            }
                            seen_default_tenant = true;
                            f_default_tenant = map.next_value::<Option<String>>()?;
                        }
                        "multitenancy_enabled" => {
                            if seen_multitenancy_enabled {
                                return Err(serde::de::Error::duplicate_field(
                                    "multitenancy_enabled",
                                ));
                            }
                            seen_multitenancy_enabled = true;
                            f_multitenancy_enabled = map.next_value::<Option<bool>>()?;
                        }
                        "private_tenant_enabled" => {
                            if seen_private_tenant_enabled {
                                return Err(serde::de::Error::duplicate_field(
                                    "private_tenant_enabled",
                                ));
                            }
                            seen_private_tenant_enabled = true;
                            f_private_tenant_enabled = map.next_value::<Option<bool>>()?;
                        }
                        "sign_in_options" => {
                            if seen_sign_in_options {
                                return Err(serde::de::Error::duplicate_field("sign_in_options"));
                            }
                            seen_sign_in_options = true;
                            f_sign_in_options = map.next_value::<Option<Vec<String>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MultiTenancyConfig {
                    default_tenant: f_default_tenant,
                    multitenancy_enabled: f_multitenancy_enabled,
                    private_tenant_enabled: f_private_tenant_enabled,
                    sign_in_options: f_sign_in_options,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MultiTenancyConfig {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.default_tenant {
            map.serialize_entry("default_tenant", v)?;
        }
        if let Some(v) = &self.multitenancy_enabled {
            map.serialize_entry("multitenancy_enabled", v)?;
        }
        if let Some(v) = &self.private_tenant_enabled {
            map.serialize_entry("private_tenant_enabled", v)?;
        }
        if let Some(v) = &self.sign_in_options {
            map.serialize_entry("sign_in_options", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Ok {
    #[doc = "The message returned as part of an `OK` response."]
    pub message: Option<String>,
    pub status: Option<crate::types::security_common::OkStatus>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Ok {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Ok;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Ok object")
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
                let mut f_status: Option<crate::types::security_common::OkStatus> = None;
                let mut seen_status = false;
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
                        "status" => {
                            if seen_status {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            seen_status = true;
                            f_status = map
                                .next_value::<Option<crate::types::security_common::OkStatus>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Ok {
                    message: f_message,
                    status: f_status,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Ok {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.message {
            map.serialize_entry("message", v)?;
        }
        if let Some(v) = &self.status {
            map.serialize_entry("status", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum OkStatus {
    Number(f64),
    String(String),
}
impl<'de> serde::Deserialize<'de> for OkStatus {
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
            Some(b'-') | Some(b'0'..=b'9') => serde_json::from_str(text).map(OkStatus::Number),
            Some(b'"') => serde_json::from_str(text).map(OkStatus::String),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a OkStatus value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct PermissionsInfo {
    #[doc = "An object with disabled APIs as keys and an array of HTTP methods as values."]
    pub disabled_endpoints: Option<std::collections::BTreeMap<String, Vec<String>>>,
    pub has_api_access: Option<bool>,
    pub user: Option<String>,
    pub user_name: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PermissionsInfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PermissionsInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PermissionsInfo object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_disabled_endpoints: Option<
                    std::collections::BTreeMap<String, Vec<String>>,
                > = None;
                let mut seen_disabled_endpoints = false;
                let mut f_has_api_access: Option<bool> = None;
                let mut seen_has_api_access = false;
                let mut f_user: Option<String> = None;
                let mut seen_user = false;
                let mut f_user_name: Option<String> = None;
                let mut seen_user_name = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "disabled_endpoints" => {
                            if seen_disabled_endpoints {
                                return Err(serde::de::Error::duplicate_field(
                                    "disabled_endpoints",
                                ));
                            }
                            seen_disabled_endpoints = true;
                            f_disabled_endpoints = map.next_value::<Option<std::collections::BTreeMap<String, Vec<String>>>>()?;
                        }
                        "has_api_access" => {
                            if seen_has_api_access {
                                return Err(serde::de::Error::duplicate_field("has_api_access"));
                            }
                            seen_has_api_access = true;
                            f_has_api_access = map.next_value::<Option<bool>>()?;
                        }
                        "user" => {
                            if seen_user {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            seen_user = true;
                            f_user = map.next_value::<Option<String>>()?;
                        }
                        "user_name" => {
                            if seen_user_name {
                                return Err(serde::de::Error::duplicate_field("user_name"));
                            }
                            seen_user_name = true;
                            f_user_name = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PermissionsInfo {
                    disabled_endpoints: f_disabled_endpoints,
                    has_api_access: f_has_api_access,
                    user: f_user,
                    user_name: f_user_name,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PermissionsInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.disabled_endpoints {
            map.serialize_entry("disabled_endpoints", v)?;
        }
        if let Some(v) = &self.has_api_access {
            map.serialize_entry("has_api_access", v)?;
        }
        if let Some(v) = &self.user {
            map.serialize_entry("user", v)?;
        }
        if let Some(v) = &self.user_name {
            map.serialize_entry("user_name", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Role {
    pub cluster_permissions: Option<Vec<String>>,
    pub description: Option<String>,
    pub hidden: Option<bool>,
    pub index_permissions: Option<Vec<crate::types::security_common::IndexPermission>>,
    pub reserved: Option<bool>,
    pub r#static: Option<bool>,
    pub tenant_permissions: Option<Vec<crate::types::security_common::TenantPermission>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Role {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Role;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Role object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cluster_permissions: Option<Vec<String>> = None;
                let mut seen_cluster_permissions = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_hidden: Option<bool> = None;
                let mut seen_hidden = false;
                let mut f_index_permissions: Option<
                    Vec<crate::types::security_common::IndexPermission>,
                > = None;
                let mut seen_index_permissions = false;
                let mut f_reserved: Option<bool> = None;
                let mut seen_reserved = false;
                let mut f_static: Option<bool> = None;
                let mut seen_static = false;
                let mut f_tenant_permissions: Option<
                    Vec<crate::types::security_common::TenantPermission>,
                > = None;
                let mut seen_tenant_permissions = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cluster_permissions" => {
                            if seen_cluster_permissions {
                                return Err(serde::de::Error::duplicate_field(
                                    "cluster_permissions",
                                ));
                            }
                            seen_cluster_permissions = true;
                            f_cluster_permissions = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "hidden" => {
                            if seen_hidden {
                                return Err(serde::de::Error::duplicate_field("hidden"));
                            }
                            seen_hidden = true;
                            f_hidden = map.next_value::<Option<bool>>()?;
                        }
                        "index_permissions" => {
                            if seen_index_permissions {
                                return Err(serde::de::Error::duplicate_field("index_permissions"));
                            }
                            seen_index_permissions = true;
                            f_index_permissions = map.next_value::<Option<Vec<crate::types::security_common::IndexPermission>>>()?;
                        }
                        "reserved" => {
                            if seen_reserved {
                                return Err(serde::de::Error::duplicate_field("reserved"));
                            }
                            seen_reserved = true;
                            f_reserved = map.next_value::<Option<bool>>()?;
                        }
                        "static" => {
                            if seen_static {
                                return Err(serde::de::Error::duplicate_field("static"));
                            }
                            seen_static = true;
                            f_static = map.next_value::<Option<bool>>()?;
                        }
                        "tenant_permissions" => {
                            if seen_tenant_permissions {
                                return Err(serde::de::Error::duplicate_field(
                                    "tenant_permissions",
                                ));
                            }
                            seen_tenant_permissions = true;
                            f_tenant_permissions = map.next_value::<Option<Vec<crate::types::security_common::TenantPermission>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Role {
                    cluster_permissions: f_cluster_permissions,
                    description: f_description,
                    hidden: f_hidden,
                    index_permissions: f_index_permissions,
                    reserved: f_reserved,
                    r#static: f_static,
                    tenant_permissions: f_tenant_permissions,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cluster_permissions {
            map.serialize_entry("cluster_permissions", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.hidden {
            map.serialize_entry("hidden", v)?;
        }
        if let Some(v) = &self.index_permissions {
            map.serialize_entry("index_permissions", v)?;
        }
        if let Some(v) = &self.reserved {
            map.serialize_entry("reserved", v)?;
        }
        if let Some(v) = &self.r#static {
            map.serialize_entry("static", v)?;
        }
        if let Some(v) = &self.tenant_permissions {
            map.serialize_entry("tenant_permissions", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RoleMapping {
    pub and_backend_roles: Option<Vec<String>>,
    pub backend_roles: Option<Vec<String>>,
    pub description: Option<String>,
    pub hidden: Option<bool>,
    pub hosts: Option<Vec<String>>,
    pub reserved: Option<bool>,
    pub users: Option<Vec<String>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RoleMapping {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RoleMapping;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RoleMapping object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_and_backend_roles: Option<Vec<String>> = None;
                let mut seen_and_backend_roles = false;
                let mut f_backend_roles: Option<Vec<String>> = None;
                let mut seen_backend_roles = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_hidden: Option<bool> = None;
                let mut seen_hidden = false;
                let mut f_hosts: Option<Vec<String>> = None;
                let mut seen_hosts = false;
                let mut f_reserved: Option<bool> = None;
                let mut seen_reserved = false;
                let mut f_users: Option<Vec<String>> = None;
                let mut seen_users = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "and_backend_roles" => {
                            if seen_and_backend_roles {
                                return Err(serde::de::Error::duplicate_field("and_backend_roles"));
                            }
                            seen_and_backend_roles = true;
                            f_and_backend_roles = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "backend_roles" => {
                            if seen_backend_roles {
                                return Err(serde::de::Error::duplicate_field("backend_roles"));
                            }
                            seen_backend_roles = true;
                            f_backend_roles = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "hidden" => {
                            if seen_hidden {
                                return Err(serde::de::Error::duplicate_field("hidden"));
                            }
                            seen_hidden = true;
                            f_hidden = map.next_value::<Option<bool>>()?;
                        }
                        "hosts" => {
                            if seen_hosts {
                                return Err(serde::de::Error::duplicate_field("hosts"));
                            }
                            seen_hosts = true;
                            f_hosts = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "reserved" => {
                            if seen_reserved {
                                return Err(serde::de::Error::duplicate_field("reserved"));
                            }
                            seen_reserved = true;
                            f_reserved = map.next_value::<Option<bool>>()?;
                        }
                        "users" => {
                            if seen_users {
                                return Err(serde::de::Error::duplicate_field("users"));
                            }
                            seen_users = true;
                            f_users = map.next_value::<Option<Vec<String>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RoleMapping {
                    and_backend_roles: f_and_backend_roles,
                    backend_roles: f_backend_roles,
                    description: f_description,
                    hidden: f_hidden,
                    hosts: f_hosts,
                    reserved: f_reserved,
                    users: f_users,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RoleMapping {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.and_backend_roles {
            map.serialize_entry("and_backend_roles", v)?;
        }
        if let Some(v) = &self.backend_roles {
            map.serialize_entry("backend_roles", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.hidden {
            map.serialize_entry("hidden", v)?;
        }
        if let Some(v) = &self.hosts {
            map.serialize_entry("hosts", v)?;
        }
        if let Some(v) = &self.reserved {
            map.serialize_entry("reserved", v)?;
        }
        if let Some(v) = &self.users {
            map.serialize_entry("users", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type RoleMappings =
    std::collections::BTreeMap<String, crate::types::security_common::RoleMapping>;

pub type RolesMap = std::collections::BTreeMap<String, crate::types::security_common::Role>;

#[derive(Debug, Clone)]
pub struct SSLInfo {
    #[doc = "A list of domain names from local certificates."]
    pub local_certificates_list: Option<Vec<String>>,
    #[doc = "The number of certificates."]
    pub peer_certificates: crate::types::security_common::SSLInfoPeerCertificates,
    #[doc = "A list of domain names from peer certificates."]
    pub peer_certificates_list: Option<Vec<String>>,
    #[doc = "The user's principal."]
    pub principal: Option<String>,
    #[doc = "The cipher for this SSL setup."]
    pub ssl_cipher: Option<String>,
    #[doc = "Whether OpenSSL is available."]
    pub ssl_openssl_available: Option<bool>,
    #[doc = "The reason OpenSSL is unavailable."]
    pub ssl_openssl_non_available_cause: Option<String>,
    #[doc = "Whether the hostname validation is supported."]
    pub ssl_openssl_supports_hostname_validation: Option<bool>,
    #[doc = "Whether `KMF` is supported."]
    pub ssl_openssl_supports_key_manager_factory: Option<bool>,
    #[doc = "Version of OpenSSL."]
    pub ssl_openssl_version: Option<crate::types::security_common::SSLInfoSslOpensslVersion>,
    #[doc = "The full version string for the OpenSSL version."]
    pub ssl_openssl_version_string: Option<String>,
    #[doc = "The protocol for this SSL setup."]
    pub ssl_protocol: Option<String>,
    #[doc = "Returns the HTTP provider's name."]
    pub ssl_provider_http: Option<String>,
    #[doc = "Returns the transport client's name."]
    pub ssl_provider_transport_client: String,
    #[doc = "Returns the transport server's name."]
    pub ssl_provider_transport_server: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SSLInfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SSLInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SSLInfo object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_local_certificates_list: Option<Vec<String>> = None;
                let mut seen_local_certificates_list = false;
                let mut f_peer_certificates: Option<
                    crate::types::security_common::SSLInfoPeerCertificates,
                > = None;
                let mut f_peer_certificates_list: Option<Vec<String>> = None;
                let mut seen_peer_certificates_list = false;
                let mut f_principal: Option<String> = None;
                let mut seen_principal = false;
                let mut f_ssl_cipher: Option<String> = None;
                let mut seen_ssl_cipher = false;
                let mut f_ssl_openssl_available: Option<bool> = None;
                let mut seen_ssl_openssl_available = false;
                let mut f_ssl_openssl_non_available_cause: Option<String> = None;
                let mut seen_ssl_openssl_non_available_cause = false;
                let mut f_ssl_openssl_supports_hostname_validation: Option<bool> = None;
                let mut seen_ssl_openssl_supports_hostname_validation = false;
                let mut f_ssl_openssl_supports_key_manager_factory: Option<bool> = None;
                let mut seen_ssl_openssl_supports_key_manager_factory = false;
                let mut f_ssl_openssl_version: Option<
                    crate::types::security_common::SSLInfoSslOpensslVersion,
                > = None;
                let mut seen_ssl_openssl_version = false;
                let mut f_ssl_openssl_version_string: Option<String> = None;
                let mut seen_ssl_openssl_version_string = false;
                let mut f_ssl_protocol: Option<String> = None;
                let mut seen_ssl_protocol = false;
                let mut f_ssl_provider_http: Option<String> = None;
                let mut seen_ssl_provider_http = false;
                let mut f_ssl_provider_transport_client: Option<String> = None;
                let mut f_ssl_provider_transport_server: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "local_certificates_list" => {
                            if seen_local_certificates_list {
                                return Err(serde::de::Error::duplicate_field(
                                    "local_certificates_list",
                                ));
                            }
                            seen_local_certificates_list = true;
                            f_local_certificates_list = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "peer_certificates" => {
                            if f_peer_certificates.is_some() {
                                return Err(serde::de::Error::duplicate_field("peer_certificates"));
                            }
                            f_peer_certificates = Some(map.next_value::<crate::types::security_common::SSLInfoPeerCertificates>()?);
                        }
                        "peer_certificates_list" => {
                            if seen_peer_certificates_list {
                                return Err(serde::de::Error::duplicate_field(
                                    "peer_certificates_list",
                                ));
                            }
                            seen_peer_certificates_list = true;
                            f_peer_certificates_list = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "principal" => {
                            if seen_principal {
                                return Err(serde::de::Error::duplicate_field("principal"));
                            }
                            seen_principal = true;
                            f_principal = map.next_value::<Option<String>>()?;
                        }
                        "ssl_cipher" => {
                            if seen_ssl_cipher {
                                return Err(serde::de::Error::duplicate_field("ssl_cipher"));
                            }
                            seen_ssl_cipher = true;
                            f_ssl_cipher = map.next_value::<Option<String>>()?;
                        }
                        "ssl_openssl_available" => {
                            if seen_ssl_openssl_available {
                                return Err(serde::de::Error::duplicate_field(
                                    "ssl_openssl_available",
                                ));
                            }
                            seen_ssl_openssl_available = true;
                            f_ssl_openssl_available = map.next_value::<Option<bool>>()?;
                        }
                        "ssl_openssl_non_available_cause" => {
                            if seen_ssl_openssl_non_available_cause {
                                return Err(serde::de::Error::duplicate_field(
                                    "ssl_openssl_non_available_cause",
                                ));
                            }
                            seen_ssl_openssl_non_available_cause = true;
                            f_ssl_openssl_non_available_cause =
                                map.next_value::<Option<String>>()?;
                        }
                        "ssl_openssl_supports_hostname_validation" => {
                            if seen_ssl_openssl_supports_hostname_validation {
                                return Err(serde::de::Error::duplicate_field(
                                    "ssl_openssl_supports_hostname_validation",
                                ));
                            }
                            seen_ssl_openssl_supports_hostname_validation = true;
                            f_ssl_openssl_supports_hostname_validation =
                                map.next_value::<Option<bool>>()?;
                        }
                        "ssl_openssl_supports_key_manager_factory" => {
                            if seen_ssl_openssl_supports_key_manager_factory {
                                return Err(serde::de::Error::duplicate_field(
                                    "ssl_openssl_supports_key_manager_factory",
                                ));
                            }
                            seen_ssl_openssl_supports_key_manager_factory = true;
                            f_ssl_openssl_supports_key_manager_factory =
                                map.next_value::<Option<bool>>()?;
                        }
                        "ssl_openssl_version" => {
                            if seen_ssl_openssl_version {
                                return Err(serde::de::Error::duplicate_field(
                                    "ssl_openssl_version",
                                ));
                            }
                            seen_ssl_openssl_version = true;
                            f_ssl_openssl_version = map.next_value::<Option<crate::types::security_common::SSLInfoSslOpensslVersion>>()?;
                        }
                        "ssl_openssl_version_string" => {
                            if seen_ssl_openssl_version_string {
                                return Err(serde::de::Error::duplicate_field(
                                    "ssl_openssl_version_string",
                                ));
                            }
                            seen_ssl_openssl_version_string = true;
                            f_ssl_openssl_version_string = map.next_value::<Option<String>>()?;
                        }
                        "ssl_protocol" => {
                            if seen_ssl_protocol {
                                return Err(serde::de::Error::duplicate_field("ssl_protocol"));
                            }
                            seen_ssl_protocol = true;
                            f_ssl_protocol = map.next_value::<Option<String>>()?;
                        }
                        "ssl_provider_http" => {
                            if seen_ssl_provider_http {
                                return Err(serde::de::Error::duplicate_field("ssl_provider_http"));
                            }
                            seen_ssl_provider_http = true;
                            f_ssl_provider_http = map.next_value::<Option<String>>()?;
                        }
                        "ssl_provider_transport_client" => {
                            if f_ssl_provider_transport_client.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "ssl_provider_transport_client",
                                ));
                            }
                            f_ssl_provider_transport_client = Some(map.next_value::<String>()?);
                        }
                        "ssl_provider_transport_server" => {
                            if f_ssl_provider_transport_server.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "ssl_provider_transport_server",
                                ));
                            }
                            f_ssl_provider_transport_server = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SSLInfo {
                    local_certificates_list: f_local_certificates_list,
                    peer_certificates: f_peer_certificates
                        .ok_or_else(|| serde::de::Error::missing_field("peer_certificates"))?,
                    peer_certificates_list: f_peer_certificates_list,
                    principal: f_principal,
                    ssl_cipher: f_ssl_cipher,
                    ssl_openssl_available: f_ssl_openssl_available,
                    ssl_openssl_non_available_cause: f_ssl_openssl_non_available_cause,
                    ssl_openssl_supports_hostname_validation:
                        f_ssl_openssl_supports_hostname_validation,
                    ssl_openssl_supports_key_manager_factory:
                        f_ssl_openssl_supports_key_manager_factory,
                    ssl_openssl_version: f_ssl_openssl_version,
                    ssl_openssl_version_string: f_ssl_openssl_version_string,
                    ssl_protocol: f_ssl_protocol,
                    ssl_provider_http: f_ssl_provider_http,
                    ssl_provider_transport_client: f_ssl_provider_transport_client.ok_or_else(
                        || serde::de::Error::missing_field("ssl_provider_transport_client"),
                    )?,
                    ssl_provider_transport_server: f_ssl_provider_transport_server.ok_or_else(
                        || serde::de::Error::missing_field("ssl_provider_transport_server"),
                    )?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SSLInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.local_certificates_list {
            map.serialize_entry("local_certificates_list", v)?;
        }
        map.serialize_entry("peer_certificates", &self.peer_certificates)?;
        if let Some(v) = &self.peer_certificates_list {
            map.serialize_entry("peer_certificates_list", v)?;
        }
        if let Some(v) = &self.principal {
            map.serialize_entry("principal", v)?;
        }
        if let Some(v) = &self.ssl_cipher {
            map.serialize_entry("ssl_cipher", v)?;
        }
        if let Some(v) = &self.ssl_openssl_available {
            map.serialize_entry("ssl_openssl_available", v)?;
        }
        if let Some(v) = &self.ssl_openssl_non_available_cause {
            map.serialize_entry("ssl_openssl_non_available_cause", v)?;
        }
        if let Some(v) = &self.ssl_openssl_supports_hostname_validation {
            map.serialize_entry("ssl_openssl_supports_hostname_validation", v)?;
        }
        if let Some(v) = &self.ssl_openssl_supports_key_manager_factory {
            map.serialize_entry("ssl_openssl_supports_key_manager_factory", v)?;
        }
        if let Some(v) = &self.ssl_openssl_version {
            map.serialize_entry("ssl_openssl_version", v)?;
        }
        if let Some(v) = &self.ssl_openssl_version_string {
            map.serialize_entry("ssl_openssl_version_string", v)?;
        }
        if let Some(v) = &self.ssl_protocol {
            map.serialize_entry("ssl_protocol", v)?;
        }
        if let Some(v) = &self.ssl_provider_http {
            map.serialize_entry("ssl_provider_http", v)?;
        }
        map.serialize_entry(
            "ssl_provider_transport_client",
            &self.ssl_provider_transport_client,
        )?;
        map.serialize_entry(
            "ssl_provider_transport_server",
            &self.ssl_provider_transport_server,
        )?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The number of certificates."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SSLInfoPeerCertificates {
    Number(f64),
    String(String),
}
impl<'de> serde::Deserialize<'de> for SSLInfoPeerCertificates {
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
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(SSLInfoPeerCertificates::Number)
            }
            Some(b'"') => serde_json::from_str(text).map(SSLInfoPeerCertificates::String),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a SSLInfoPeerCertificates value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "Version of OpenSSL."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SSLInfoSslOpensslVersion {
    Number(f64),
    String(String),
}
impl<'de> serde::Deserialize<'de> for SSLInfoSslOpensslVersion {
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
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(SSLInfoSslOpensslVersion::Number)
            }
            Some(b'"') => serde_json::from_str(text).map(SSLInfoSslOpensslVersion::String),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a SSLInfoSslOpensslVersion value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub config: Option<crate::types::security_common::DynamicConfig>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SecurityConfig {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SecurityConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SecurityConfig object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_config: Option<crate::types::security_common::DynamicConfig> = None;
                let mut seen_config = false;
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
                            f_config = map
                                .next_value::<Option<crate::types::security_common::DynamicConfig>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SecurityConfig {
                    config: f_config,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SecurityConfig {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.config {
            map.serialize_entry("config", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Tenant {
    pub description: Option<String>,
    pub hidden: Option<bool>,
    pub reserved: Option<bool>,
    pub r#static: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Tenant {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Tenant;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Tenant object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_hidden: Option<bool> = None;
                let mut seen_hidden = false;
                let mut f_reserved: Option<bool> = None;
                let mut seen_reserved = false;
                let mut f_static: Option<bool> = None;
                let mut seen_static = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "hidden" => {
                            if seen_hidden {
                                return Err(serde::de::Error::duplicate_field("hidden"));
                            }
                            seen_hidden = true;
                            f_hidden = map.next_value::<Option<bool>>()?;
                        }
                        "reserved" => {
                            if seen_reserved {
                                return Err(serde::de::Error::duplicate_field("reserved"));
                            }
                            seen_reserved = true;
                            f_reserved = map.next_value::<Option<bool>>()?;
                        }
                        "static" => {
                            if seen_static {
                                return Err(serde::de::Error::duplicate_field("static"));
                            }
                            seen_static = true;
                            f_static = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Tenant {
                    description: f_description,
                    hidden: f_hidden,
                    reserved: f_reserved,
                    r#static: f_static,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Tenant {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.hidden {
            map.serialize_entry("hidden", v)?;
        }
        if let Some(v) = &self.reserved {
            map.serialize_entry("reserved", v)?;
        }
        if let Some(v) = &self.r#static {
            map.serialize_entry("static", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type TenantInfo = std::collections::BTreeMap<String, String>;

#[derive(Debug, Clone)]
pub struct TenantPermission {
    pub allowed_actions: Option<Vec<String>>,
    pub tenant_patterns: Option<Vec<String>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TenantPermission {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TenantPermission;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TenantPermission object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_allowed_actions: Option<Vec<String>> = None;
                let mut seen_allowed_actions = false;
                let mut f_tenant_patterns: Option<Vec<String>> = None;
                let mut seen_tenant_patterns = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "allowed_actions" => {
                            if seen_allowed_actions {
                                return Err(serde::de::Error::duplicate_field("allowed_actions"));
                            }
                            seen_allowed_actions = true;
                            f_allowed_actions = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "tenant_patterns" => {
                            if seen_tenant_patterns {
                                return Err(serde::de::Error::duplicate_field("tenant_patterns"));
                            }
                            seen_tenant_patterns = true;
                            f_tenant_patterns = map.next_value::<Option<Vec<String>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TenantPermission {
                    allowed_actions: f_allowed_actions,
                    tenant_patterns: f_tenant_patterns,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TenantPermission {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.allowed_actions {
            map.serialize_entry("allowed_actions", v)?;
        }
        if let Some(v) = &self.tenant_patterns {
            map.serialize_entry("tenant_patterns", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type TenantsMap = std::collections::BTreeMap<String, crate::types::security_common::Tenant>;

#[derive(Debug, Clone)]
pub struct UpgradeCheck {
    pub status: Option<String>,
    pub upgrade_actions:
        Option<std::collections::BTreeMap<String, std::collections::BTreeMap<String, Vec<String>>>>,
    pub upgrade_available: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UpgradeCheck {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UpgradeCheck;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UpgradeCheck object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_status: Option<String> = None;
                let mut seen_status = false;
                let mut f_upgrade_actions: Option<
                    std::collections::BTreeMap<
                        String,
                        std::collections::BTreeMap<String, Vec<String>>,
                    >,
                > = None;
                let mut seen_upgrade_actions = false;
                let mut f_upgrade_available: Option<bool> = None;
                let mut seen_upgrade_available = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "status" => {
                            if seen_status {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            seen_status = true;
                            f_status = map.next_value::<Option<String>>()?;
                        }
                        "upgradeActions" => {
                            if seen_upgrade_actions {
                                return Err(serde::de::Error::duplicate_field("upgradeActions"));
                            }
                            seen_upgrade_actions = true;
                            f_upgrade_actions = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    std::collections::BTreeMap<String, Vec<String>>,
                                >,
                            >>()?;
                        }
                        "upgradeAvailable" => {
                            if seen_upgrade_available {
                                return Err(serde::de::Error::duplicate_field("upgradeAvailable"));
                            }
                            seen_upgrade_available = true;
                            f_upgrade_available = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UpgradeCheck {
                    status: f_status,
                    upgrade_actions: f_upgrade_actions,
                    upgrade_available: f_upgrade_available,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UpgradeCheck {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.status {
            map.serialize_entry("status", v)?;
        }
        if let Some(v) = &self.upgrade_actions {
            map.serialize_entry("upgradeActions", v)?;
        }
        if let Some(v) = &self.upgrade_available {
            map.serialize_entry("upgradeAvailable", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct UpgradePerform {
    pub status: Option<String>,
    pub upgrades:
        Option<std::collections::BTreeMap<String, std::collections::BTreeMap<String, Vec<String>>>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UpgradePerform {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UpgradePerform;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UpgradePerform object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_status: Option<String> = None;
                let mut seen_status = false;
                let mut f_upgrades: Option<
                    std::collections::BTreeMap<
                        String,
                        std::collections::BTreeMap<String, Vec<String>>,
                    >,
                > = None;
                let mut seen_upgrades = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "status" => {
                            if seen_status {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            seen_status = true;
                            f_status = map.next_value::<Option<String>>()?;
                        }
                        "upgrades" => {
                            if seen_upgrades {
                                return Err(serde::de::Error::duplicate_field("upgrades"));
                            }
                            seen_upgrades = true;
                            f_upgrades = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    std::collections::BTreeMap<String, Vec<String>>,
                                >,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UpgradePerform {
                    status: f_status,
                    upgrades: f_upgrades,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UpgradePerform {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.status {
            map.serialize_entry("status", v)?;
        }
        if let Some(v) = &self.upgrades {
            map.serialize_entry("upgrades", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub attributes: Option<crate::types::security_common::UserAttributes>,
    pub backend_roles: Option<Vec<String>>,
    pub description: Option<String>,
    pub hash: Option<String>,
    pub hidden: Option<bool>,
    pub opendistro_security_roles: Option<Vec<String>>,
    pub password: Option<String>,
    pub reserved: Option<bool>,
    pub r#static: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for User {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = User;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a User object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_attributes: Option<crate::types::security_common::UserAttributes> = None;
                let mut seen_attributes = false;
                let mut f_backend_roles: Option<Vec<String>> = None;
                let mut seen_backend_roles = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_hash: Option<String> = None;
                let mut seen_hash = false;
                let mut f_hidden: Option<bool> = None;
                let mut seen_hidden = false;
                let mut f_opendistro_security_roles: Option<Vec<String>> = None;
                let mut seen_opendistro_security_roles = false;
                let mut f_password: Option<String> = None;
                let mut seen_password = false;
                let mut f_reserved: Option<bool> = None;
                let mut seen_reserved = false;
                let mut f_static: Option<bool> = None;
                let mut seen_static = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "attributes" => {
                            if seen_attributes {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            seen_attributes = true;
                            f_attributes = map.next_value::<Option<crate::types::security_common::UserAttributes>>()?;
                        }
                        "backend_roles" => {
                            if seen_backend_roles {
                                return Err(serde::de::Error::duplicate_field("backend_roles"));
                            }
                            seen_backend_roles = true;
                            f_backend_roles = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "hash" => {
                            if seen_hash {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            seen_hash = true;
                            f_hash = map.next_value::<Option<String>>()?;
                        }
                        "hidden" => {
                            if seen_hidden {
                                return Err(serde::de::Error::duplicate_field("hidden"));
                            }
                            seen_hidden = true;
                            f_hidden = map.next_value::<Option<bool>>()?;
                        }
                        "opendistro_security_roles" => {
                            if seen_opendistro_security_roles {
                                return Err(serde::de::Error::duplicate_field(
                                    "opendistro_security_roles",
                                ));
                            }
                            seen_opendistro_security_roles = true;
                            f_opendistro_security_roles =
                                map.next_value::<Option<Vec<String>>>()?;
                        }
                        "password" => {
                            if seen_password {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            seen_password = true;
                            f_password = map.next_value::<Option<String>>()?;
                        }
                        "reserved" => {
                            if seen_reserved {
                                return Err(serde::de::Error::duplicate_field("reserved"));
                            }
                            seen_reserved = true;
                            f_reserved = map.next_value::<Option<bool>>()?;
                        }
                        "static" => {
                            if seen_static {
                                return Err(serde::de::Error::duplicate_field("static"));
                            }
                            seen_static = true;
                            f_static = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(User {
                    attributes: f_attributes,
                    backend_roles: f_backend_roles,
                    description: f_description,
                    hash: f_hash,
                    hidden: f_hidden,
                    opendistro_security_roles: f_opendistro_security_roles,
                    password: f_password,
                    reserved: f_reserved,
                    r#static: f_static,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for User {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.attributes {
            map.serialize_entry("attributes", v)?;
        }
        if let Some(v) = &self.backend_roles {
            map.serialize_entry("backend_roles", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.hash {
            map.serialize_entry("hash", v)?;
        }
        if let Some(v) = &self.hidden {
            map.serialize_entry("hidden", v)?;
        }
        if let Some(v) = &self.opendistro_security_roles {
            map.serialize_entry("opendistro_security_roles", v)?;
        }
        if let Some(v) = &self.password {
            map.serialize_entry("password", v)?;
        }
        if let Some(v) = &self.reserved {
            map.serialize_entry("reserved", v)?;
        }
        if let Some(v) = &self.r#static {
            map.serialize_entry("static", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type UserAttributes = std::collections::BTreeMap<String, String>;

#[derive(Debug, Clone)]
pub struct UserTenants {
    pub admin: Option<bool>,
    pub admin_tenant: Option<bool>,
    pub global_tenant: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UserTenants {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UserTenants;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UserTenants object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_admin: Option<bool> = None;
                let mut seen_admin = false;
                let mut f_admin_tenant: Option<bool> = None;
                let mut seen_admin_tenant = false;
                let mut f_global_tenant: Option<bool> = None;
                let mut seen_global_tenant = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "admin" => {
                            if seen_admin {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            seen_admin = true;
                            f_admin = map.next_value::<Option<bool>>()?;
                        }
                        "admin_tenant" => {
                            if seen_admin_tenant {
                                return Err(serde::de::Error::duplicate_field("admin_tenant"));
                            }
                            seen_admin_tenant = true;
                            f_admin_tenant = map.next_value::<Option<bool>>()?;
                        }
                        "global_tenant" => {
                            if seen_global_tenant {
                                return Err(serde::de::Error::duplicate_field("global_tenant"));
                            }
                            seen_global_tenant = true;
                            f_global_tenant = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UserTenants {
                    admin: f_admin,
                    admin_tenant: f_admin_tenant,
                    global_tenant: f_global_tenant,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UserTenants {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.admin {
            map.serialize_entry("admin", v)?;
        }
        if let Some(v) = &self.admin_tenant {
            map.serialize_entry("admin_tenant", v)?;
        }
        if let Some(v) = &self.global_tenant {
            map.serialize_entry("global_tenant", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type UsersMap = std::collections::BTreeMap<String, crate::types::security_common::User>;

#[derive(Debug, Clone)]
pub struct WhoAmI {
    pub dn: Option<String>,
    pub is_admin: Option<bool>,
    pub is_node_certificate_request: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for WhoAmI {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = WhoAmI;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a WhoAmI object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_dn: Option<String> = None;
                let mut seen_dn = false;
                let mut f_is_admin: Option<bool> = None;
                let mut seen_is_admin = false;
                let mut f_is_node_certificate_request: Option<bool> = None;
                let mut seen_is_node_certificate_request = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "dn" => {
                            if seen_dn {
                                return Err(serde::de::Error::duplicate_field("dn"));
                            }
                            seen_dn = true;
                            f_dn = map.next_value::<Option<String>>()?;
                        }
                        "is_admin" => {
                            if seen_is_admin {
                                return Err(serde::de::Error::duplicate_field("is_admin"));
                            }
                            seen_is_admin = true;
                            f_is_admin = map.next_value::<Option<bool>>()?;
                        }
                        "is_node_certificate_request" => {
                            if seen_is_node_certificate_request {
                                return Err(serde::de::Error::duplicate_field(
                                    "is_node_certificate_request",
                                ));
                            }
                            seen_is_node_certificate_request = true;
                            f_is_node_certificate_request = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(WhoAmI {
                    dn: f_dn,
                    is_admin: f_is_admin,
                    is_node_certificate_request: f_is_node_certificate_request,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for WhoAmI {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.dn {
            map.serialize_entry("dn", v)?;
        }
        if let Some(v) = &self.is_admin {
            map.serialize_entry("is_admin", v)?;
        }
        if let Some(v) = &self.is_node_certificate_request {
            map.serialize_entry("is_node_certificate_request", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
