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

//! Response types of the `indices_simulate_template` specification namespace

#![allow(missing_docs)]

#[doc = "The configuration for overlapping index templates."]
#[derive(Debug, Clone)]
pub struct Overlapping {
    #[doc = "The list of index patterns that may overlap with other templates."]
    pub index_patterns: Vec<String>,
    pub name: crate::types::common::Name,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Overlapping {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Overlapping;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Overlapping object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_index_patterns: Option<Vec<String>> = None;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "index_patterns" => {
                            if f_index_patterns.is_some() {
                                return Err(serde::de::Error::duplicate_field("index_patterns"));
                            }
                            f_index_patterns = Some(map.next_value::<Vec<String>>()?);
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
                Ok(Overlapping {
                    index_patterns: f_index_patterns
                        .ok_or_else(|| serde::de::Error::missing_field("index_patterns"))?,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Overlapping {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("index_patterns", &self.index_patterns)?;
        map.serialize_entry("name", &self.name)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `indices.simulate_template` API"]
pub type SimulateTemplateResponse = crate::types::indices::SimulateTemplateResponse;

#[doc = "The configuration for an index template simulation."]
#[derive(Debug, Clone)]
pub struct Template {
    #[doc = "The map of alias names to their configurations."]
    pub aliases: std::collections::BTreeMap<String, crate::types::indices_common::Alias>,
    pub mappings: Option<crate::types::common_mapping::TypeMapping>,
    pub settings: crate::types::indices_common::IndexSettings,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Template {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Template;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Template object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_aliases: Option<
                    std::collections::BTreeMap<String, crate::types::indices_common::Alias>,
                > = None;
                let mut f_mappings: Option<crate::types::common_mapping::TypeMapping> = None;
                let mut seen_mappings = false;
                let mut f_settings: Option<crate::types::indices_common::IndexSettings> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "aliases" => {
                            if f_aliases.is_some() {
                                return Err(serde::de::Error::duplicate_field("aliases"));
                            }
                            f_aliases = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                crate::types::indices_common::Alias,
                            >>()?);
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
                            if f_settings.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            f_settings = Some(
                                map.next_value::<crate::types::indices_common::IndexSettings>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Template {
                    aliases: f_aliases.ok_or_else(|| serde::de::Error::missing_field("aliases"))?,
                    mappings: f_mappings,
                    settings: f_settings
                        .ok_or_else(|| serde::de::Error::missing_field("settings"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Template {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("aliases", &self.aliases)?;
        if let Some(v) = &self.mappings {
            map.serialize_entry("mappings", v)?;
        }
        map.serialize_entry("settings", &self.settings)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
