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

//! Response types of the `cat_templates` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct TemplatesRecord {
    #[doc = "The component templates that comprise the index template."]
    pub composed_of: Option<String>,
    #[doc = "The template index patterns."]
    pub index_patterns: Option<String>,
    pub name: Option<crate::types::common::Name>,
    #[doc = "The template application order or priority number."]
    pub order: Option<String>,
    #[doc = "The template version."]
    pub version: Option<crate::types::common::VersionString>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TemplatesRecord {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TemplatesRecord;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TemplatesRecord object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_composed_of: Option<String> = None;
                let mut seen_composed_of = false;
                let mut f_index_patterns: Option<String> = None;
                let mut seen_index_patterns = false;
                let mut f_name: Option<crate::types::common::Name> = None;
                let mut seen_name = false;
                let mut f_order: Option<String> = None;
                let mut seen_order = false;
                let mut f_version: Option<crate::types::common::VersionString> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "composed_of" => {
                            if seen_composed_of {
                                return Err(serde::de::Error::duplicate_field("composed_of"));
                            }
                            seen_composed_of = true;
                            f_composed_of = map.next_value::<Option<String>>()?;
                        }
                        "index_patterns" => {
                            if seen_index_patterns {
                                return Err(serde::de::Error::duplicate_field("index_patterns"));
                            }
                            seen_index_patterns = true;
                            f_index_patterns = map.next_value::<Option<String>>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "order" => {
                            if seen_order {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            seen_order = true;
                            f_order = map.next_value::<Option<String>>()?;
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionString>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TemplatesRecord {
                    composed_of: f_composed_of,
                    index_patterns: f_index_patterns,
                    name: f_name,
                    order: f_order,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TemplatesRecord {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.composed_of {
            map.serialize_entry("composed_of", v)?;
        }
        if let Some(v) = &self.index_patterns {
            map.serialize_entry("index_patterns", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.order {
            map.serialize_entry("order", v)?;
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

#[doc = "The response of the `cat.templates` API"]
pub type TemplatesResponse = crate::types::cat::TemplatesResponse;
