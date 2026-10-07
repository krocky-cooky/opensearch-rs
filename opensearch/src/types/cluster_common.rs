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

//! Response types of the `cluster_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct ComponentTemplate {
    pub component_template: crate::types::cluster_common::ComponentTemplateNode,
    pub name: crate::types::common::Name,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ComponentTemplate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ComponentTemplate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ComponentTemplate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_component_template: Option<
                    crate::types::cluster_common::ComponentTemplateNode,
                > = None;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "component_template" => {
                            if f_component_template.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "component_template",
                                ));
                            }
                            f_component_template = Some(map.next_value::<crate::types::cluster_common::ComponentTemplateNode>()?);
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<crate::types::common::Name>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ComponentTemplate {
                    component_template: f_component_template
                        .ok_or_else(|| serde::de::Error::missing_field("component_template"))?,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ComponentTemplate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("component_template", &self.component_template)?;
        map.serialize_entry("name", &self.name)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ComponentTemplateNode {
    pub meta: Option<crate::types::common::Metadata>,
    pub template: crate::types::cluster_common::ComponentTemplateSummary,
    pub version: Option<crate::types::common::VersionNumber>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ComponentTemplateNode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ComponentTemplateNode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ComponentTemplateNode object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_template: Option<crate::types::cluster_common::ComponentTemplateSummary> =
                    None;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("_meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "template" => {
                            if f_template.is_some() {
                                return Err(serde::de::Error::duplicate_field("template"));
                            }
                            f_template = Some(map.next_value::<crate::types::cluster_common::ComponentTemplateSummary>()?);
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionNumber>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ComponentTemplateNode {
                    meta: f_meta,
                    template: f_template
                        .ok_or_else(|| serde::de::Error::missing_field("template"))?,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ComponentTemplateNode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("_meta", v)?;
        }
        map.serialize_entry("template", &self.template)?;
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
pub struct ComponentTemplateSummary {
    pub meta: Option<crate::types::common::Metadata>,
    pub aliases:
        Option<std::collections::BTreeMap<String, crate::types::indices_common::AliasDefinition>>,
    pub mappings: Option<crate::types::common_mapping::TypeMapping>,
    pub settings:
        Option<std::collections::BTreeMap<String, crate::types::indices_common::IndexSettings>>,
    pub version: Option<crate::types::common::VersionNumber>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ComponentTemplateSummary {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ComponentTemplateSummary;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ComponentTemplateSummary object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_aliases: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::indices_common::AliasDefinition,
                    >,
                > = None;
                let mut seen_aliases = false;
                let mut f_mappings: Option<crate::types::common_mapping::TypeMapping> = None;
                let mut seen_mappings = false;
                let mut f_settings: Option<
                    std::collections::BTreeMap<String, crate::types::indices_common::IndexSettings>,
                > = None;
                let mut seen_settings = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("_meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "aliases" => {
                            if seen_aliases {
                                return Err(serde::de::Error::duplicate_field("aliases"));
                            }
                            seen_aliases = true;
                            f_aliases = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::indices_common::AliasDefinition,
                                >,
                            >>()?;
                        }
                        "mappings" => {
                            if seen_mappings {
                                return Err(serde::de::Error::duplicate_field("mappings"));
                            }
                            seen_mappings = true;
                            f_mappings = map
                                .next_value::<Option<crate::types::common_mapping::TypeMapping>>(
                                )?;
                        }
                        "settings" => {
                            if seen_settings {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            seen_settings = true;
                            f_settings = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::indices_common::IndexSettings,
                                >,
                            >>()?;
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionNumber>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ComponentTemplateSummary {
                    meta: f_meta,
                    aliases: f_aliases,
                    mappings: f_mappings,
                    settings: f_settings,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ComponentTemplateSummary {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("_meta", v)?;
        }
        if let Some(v) = &self.aliases {
            map.serialize_entry("aliases", v)?;
        }
        if let Some(v) = &self.mappings {
            map.serialize_entry("mappings", v)?;
        }
        if let Some(v) = &self.settings {
            map.serialize_entry("settings", v)?;
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
