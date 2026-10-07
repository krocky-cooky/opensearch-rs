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

//! Response types of the `query_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct Credentials {
    #[doc = "The username for authentication."]
    pub password: String,
    #[doc = "The username for authentication."]
    pub username: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Credentials {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Credentials;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Credentials object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_password: Option<String> = None;
                let mut f_username: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "password" => {
                            if f_password.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            f_password = Some(map.next_value::<String>()?);
                        }
                        "username" => {
                            if f_username.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            f_username = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Credentials {
                    password: f_password
                        .ok_or_else(|| serde::de::Error::missing_field("password"))?,
                    username: f_username
                        .ok_or_else(|| serde::de::Error::missing_field("username"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Credentials {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("password", &self.password)?;
        map.serialize_entry("username", &self.username)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct DataSource {
    pub allowed_roles: Option<Vec<String>>,
    pub configuration: Option<crate::types::query_common::DataSourceConfiguration>,
    #[doc = "The connector type for the data source."]
    pub connector: String,
    #[doc = "The description of the data source."]
    pub description: Option<String>,
    #[doc = "The name of the data source."]
    pub name: String,
    #[doc = "The configuration properties for the data source."]
    pub properties: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
    #[doc = "The index where query results are stored."]
    pub result_index: String,
    #[doc = "The current status of the data source."]
    pub status: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DataSource {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DataSource;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DataSource object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_allowed_roles: Option<Vec<String>> = None;
                let mut seen_allowed_roles = false;
                let mut f_configuration: Option<
                    crate::types::query_common::DataSourceConfiguration,
                > = None;
                let mut seen_configuration = false;
                let mut f_connector: Option<String> = None;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_name: Option<String> = None;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut f_result_index: Option<String> = None;
                let mut f_status: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "allowedRoles" => {
                            if seen_allowed_roles {
                                return Err(serde::de::Error::duplicate_field("allowedRoles"));
                            }
                            seen_allowed_roles = true;
                            f_allowed_roles = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "configuration" => {
                            if seen_configuration {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            seen_configuration = true;
                            f_configuration = map.next_value::<Option<crate::types::query_common::DataSourceConfiguration>>()?;
                        }
                        "connector" => {
                            if f_connector.is_some() {
                                return Err(serde::de::Error::duplicate_field("connector"));
                            }
                            f_connector = Some(map.next_value::<String>()?);
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<String>()?);
                        }
                        "properties" => {
                            if f_properties.is_some() {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            f_properties = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                Box<serde_json::value::RawValue>,
                            >>()?);
                        }
                        "resultIndex" => {
                            if f_result_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("resultIndex"));
                            }
                            f_result_index = Some(map.next_value::<String>()?);
                        }
                        "status" => {
                            if f_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            f_status = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DataSource {
                    allowed_roles: f_allowed_roles,
                    configuration: f_configuration,
                    connector: f_connector
                        .ok_or_else(|| serde::de::Error::missing_field("connector"))?,
                    description: f_description,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    properties: f_properties
                        .ok_or_else(|| serde::de::Error::missing_field("properties"))?,
                    result_index: f_result_index
                        .ok_or_else(|| serde::de::Error::missing_field("resultIndex"))?,
                    status: f_status.ok_or_else(|| serde::de::Error::missing_field("status"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DataSource {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.allowed_roles {
            map.serialize_entry("allowedRoles", v)?;
        }
        if let Some(v) = &self.configuration {
            map.serialize_entry("configuration", v)?;
        }
        map.serialize_entry("connector", &self.connector)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("properties", &self.properties)?;
        map.serialize_entry("resultIndex", &self.result_index)?;
        map.serialize_entry("status", &self.status)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct DataSourceConfiguration {
    pub credentials: crate::types::query_common::Credentials,
    #[doc = "The connection endpoint for the data source."]
    pub endpoint: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DataSourceConfiguration {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DataSourceConfiguration;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DataSourceConfiguration object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_credentials: Option<crate::types::query_common::Credentials> = None;
                let mut f_endpoint: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "credentials" => {
                            if f_credentials.is_some() {
                                return Err(serde::de::Error::duplicate_field("credentials"));
                            }
                            f_credentials =
                                Some(map.next_value::<crate::types::query_common::Credentials>()?);
                        }
                        "endpoint" => {
                            if f_endpoint.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            f_endpoint = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DataSourceConfiguration {
                    credentials: f_credentials
                        .ok_or_else(|| serde::de::Error::missing_field("credentials"))?,
                    endpoint: f_endpoint
                        .ok_or_else(|| serde::de::Error::missing_field("endpoint"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DataSourceConfiguration {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("credentials", &self.credentials)?;
        map.serialize_entry("endpoint", &self.endpoint)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type DataSourceList = Vec<crate::types::query_common::DataSource>;

#[doc = "Represents the fundamental reason for an error."]
#[derive(Debug, Clone)]
pub struct DataSourceRetrieve {
    #[doc = "The roles allowed to access this data source."]
    pub allowed_roles: Option<Vec<String>>,
    pub configuration: Option<crate::types::query_common::DataSourceConfiguration>,
    #[doc = "The connector type for the data source."]
    pub connector: String,
    #[doc = "The description of the data source."]
    pub description: Option<String>,
    #[doc = "The name of the data source."]
    pub name: String,
    #[doc = "The configuration properties for the data source."]
    pub properties: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
    #[doc = "The index where query results are stored."]
    pub result_index: String,
    #[doc = "The current status of the data source."]
    pub status: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DataSourceRetrieve {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DataSourceRetrieve;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DataSourceRetrieve object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_allowed_roles: Option<Vec<String>> = None;
                let mut seen_allowed_roles = false;
                let mut f_configuration: Option<
                    crate::types::query_common::DataSourceConfiguration,
                > = None;
                let mut seen_configuration = false;
                let mut f_connector: Option<String> = None;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_name: Option<String> = None;
                let mut f_properties: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut f_result_index: Option<String> = None;
                let mut f_status: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "allowedRoles" => {
                            if seen_allowed_roles {
                                return Err(serde::de::Error::duplicate_field("allowedRoles"));
                            }
                            seen_allowed_roles = true;
                            f_allowed_roles = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "configuration" => {
                            if seen_configuration {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            seen_configuration = true;
                            f_configuration = map.next_value::<Option<crate::types::query_common::DataSourceConfiguration>>()?;
                        }
                        "connector" => {
                            if f_connector.is_some() {
                                return Err(serde::de::Error::duplicate_field("connector"));
                            }
                            f_connector = Some(map.next_value::<String>()?);
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<String>()?);
                        }
                        "properties" => {
                            if f_properties.is_some() {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            f_properties = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                Box<serde_json::value::RawValue>,
                            >>()?);
                        }
                        "resultIndex" => {
                            if f_result_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("resultIndex"));
                            }
                            f_result_index = Some(map.next_value::<String>()?);
                        }
                        "status" => {
                            if f_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            f_status = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DataSourceRetrieve {
                    allowed_roles: f_allowed_roles,
                    configuration: f_configuration,
                    connector: f_connector
                        .ok_or_else(|| serde::de::Error::missing_field("connector"))?,
                    description: f_description,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    properties: f_properties
                        .ok_or_else(|| serde::de::Error::missing_field("properties"))?,
                    result_index: f_result_index
                        .ok_or_else(|| serde::de::Error::missing_field("resultIndex"))?,
                    status: f_status.ok_or_else(|| serde::de::Error::missing_field("status"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DataSourceRetrieve {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.allowed_roles {
            map.serialize_entry("allowedRoles", v)?;
        }
        if let Some(v) = &self.configuration {
            map.serialize_entry("configuration", v)?;
        }
        map.serialize_entry("connector", &self.connector)?;
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("properties", &self.properties)?;
        map.serialize_entry("resultIndex", &self.result_index)?;
        map.serialize_entry("status", &self.status)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
