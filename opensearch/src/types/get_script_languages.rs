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

//! Response types of the `get_script_languages` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct GetScriptLanguagesResponse {
    pub language_contexts: Vec<crate::types::get_script_languages::LanguageContext>,
    pub types_allowed: Vec<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GetScriptLanguagesResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GetScriptLanguagesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GetScriptLanguagesResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_language_contexts: Option<
                    Vec<crate::types::get_script_languages::LanguageContext>,
                > = None;
                let mut f_types_allowed: Option<Vec<String>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "language_contexts" => {
                            if f_language_contexts.is_some() {
                                return Err(serde::de::Error::duplicate_field("language_contexts"));
                            }
                            f_language_contexts = Some(map.next_value::<Vec<crate::types::get_script_languages::LanguageContext>>()?);
                        }
                        "types_allowed" => {
                            if f_types_allowed.is_some() {
                                return Err(serde::de::Error::duplicate_field("types_allowed"));
                            }
                            f_types_allowed = Some(map.next_value::<Vec<String>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GetScriptLanguagesResponse {
                    language_contexts: f_language_contexts
                        .ok_or_else(|| serde::de::Error::missing_field("language_contexts"))?,
                    types_allowed: f_types_allowed
                        .ok_or_else(|| serde::de::Error::missing_field("types_allowed"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GetScriptLanguagesResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("language_contexts", &self.language_contexts)?;
        map.serialize_entry("types_allowed", &self.types_allowed)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct LanguageContext {
    pub contexts: Vec<String>,
    pub language: crate::types::common::ScriptLanguage,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for LanguageContext {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = LanguageContext;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a LanguageContext object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_contexts: Option<Vec<String>> = None;
                let mut f_language: Option<crate::types::common::ScriptLanguage> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "contexts" => {
                            if f_contexts.is_some() {
                                return Err(serde::de::Error::duplicate_field("contexts"));
                            }
                            f_contexts = Some(map.next_value::<Vec<String>>()?);
                        }
                        "language" => {
                            if f_language.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            f_language =
                                Some(map.next_value::<crate::types::common::ScriptLanguage>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(LanguageContext {
                    contexts: f_contexts
                        .ok_or_else(|| serde::de::Error::missing_field("contexts"))?,
                    language: f_language
                        .ok_or_else(|| serde::de::Error::missing_field("language"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for LanguageContext {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("contexts", &self.contexts)?;
        map.serialize_entry("language", &self.language)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
