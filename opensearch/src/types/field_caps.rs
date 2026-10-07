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

//! Response types of the `field_caps` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct FieldCapability {
    #[doc = "Whether this field can be aggregated on all indexes."]
    pub aggregatable: bool,
    pub indices: Option<crate::types::common::Indices>,
    pub meta: Option<std::collections::BTreeMap<String, Vec<String>>>,
    #[doc = "Whether this field is registered as a metadata field."]
    pub metadata_field: Option<bool>,
    pub non_aggregatable_indices: Option<crate::types::common::Indices>,
    pub non_searchable_indices: Option<crate::types::common::Indices>,
    #[doc = "Whether this field is indexed for search on all indexes."]
    pub searchable: bool,
    pub r#type: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FieldCapability {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FieldCapability;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FieldCapability object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_aggregatable: Option<bool> = None;
                let mut f_indices: Option<crate::types::common::Indices> = None;
                let mut seen_indices = false;
                let mut f_meta: Option<std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut seen_meta = false;
                let mut f_metadata_field: Option<bool> = None;
                let mut seen_metadata_field = false;
                let mut f_non_aggregatable_indices: Option<crate::types::common::Indices> = None;
                let mut seen_non_aggregatable_indices = false;
                let mut f_non_searchable_indices: Option<crate::types::common::Indices> = None;
                let mut seen_non_searchable_indices = false;
                let mut f_searchable: Option<bool> = None;
                let mut f_type: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "aggregatable" => {
                            if f_aggregatable.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggregatable"));
                            }
                            f_aggregatable = Some(map.next_value::<bool>()?);
                        }
                        "indices" => {
                            if seen_indices {
                                return Err(serde::de::Error::duplicate_field("indices"));
                            }
                            seen_indices = true;
                            f_indices =
                                map.next_value::<Option<crate::types::common::Indices>>()?;
                        }
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<std::collections::BTreeMap<String, Vec<String>>>>()?;
                        }
                        "metadata_field" => {
                            if seen_metadata_field {
                                return Err(serde::de::Error::duplicate_field("metadata_field"));
                            }
                            seen_metadata_field = true;
                            f_metadata_field = map.next_value::<Option<bool>>()?;
                        }
                        "non_aggregatable_indices" => {
                            if seen_non_aggregatable_indices {
                                return Err(serde::de::Error::duplicate_field(
                                    "non_aggregatable_indices",
                                ));
                            }
                            seen_non_aggregatable_indices = true;
                            f_non_aggregatable_indices =
                                map.next_value::<Option<crate::types::common::Indices>>()?;
                        }
                        "non_searchable_indices" => {
                            if seen_non_searchable_indices {
                                return Err(serde::de::Error::duplicate_field(
                                    "non_searchable_indices",
                                ));
                            }
                            seen_non_searchable_indices = true;
                            f_non_searchable_indices =
                                map.next_value::<Option<crate::types::common::Indices>>()?;
                        }
                        "searchable" => {
                            if f_searchable.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchable"));
                            }
                            f_searchable = Some(map.next_value::<bool>()?);
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FieldCapability {
                    aggregatable: f_aggregatable
                        .ok_or_else(|| serde::de::Error::missing_field("aggregatable"))?,
                    indices: f_indices,
                    meta: f_meta,
                    metadata_field: f_metadata_field,
                    non_aggregatable_indices: f_non_aggregatable_indices,
                    non_searchable_indices: f_non_searchable_indices,
                    searchable: f_searchable
                        .ok_or_else(|| serde::de::Error::missing_field("searchable"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FieldCapability {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("aggregatable", &self.aggregatable)?;
        if let Some(v) = &self.indices {
            map.serialize_entry("indices", v)?;
        }
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.metadata_field {
            map.serialize_entry("metadata_field", v)?;
        }
        if let Some(v) = &self.non_aggregatable_indices {
            map.serialize_entry("non_aggregatable_indices", v)?;
        }
        if let Some(v) = &self.non_searchable_indices {
            map.serialize_entry("non_searchable_indices", v)?;
        }
        map.serialize_entry("searchable", &self.searchable)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct FieldCapsResponse {
    pub fields: std::collections::BTreeMap<
        String,
        std::collections::BTreeMap<String, crate::types::field_caps::FieldCapability>,
    >,
    pub indices: crate::types::common::Indices,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FieldCapsResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FieldCapsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FieldCapsResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_fields: Option<
                    std::collections::BTreeMap<
                        String,
                        std::collections::BTreeMap<
                            String,
                            crate::types::field_caps::FieldCapability,
                        >,
                    >,
                > = None;
                let mut f_indices: Option<crate::types::common::Indices> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "fields" => {
                            if f_fields.is_some() {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            f_fields = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::field_caps::FieldCapability,
                                >,
                            >>()?);
                        }
                        "indices" => {
                            if f_indices.is_some() {
                                return Err(serde::de::Error::duplicate_field("indices"));
                            }
                            f_indices = Some(map.next_value::<crate::types::common::Indices>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FieldCapsResponse {
                    fields: f_fields.ok_or_else(|| serde::de::Error::missing_field("fields"))?,
                    indices: f_indices.ok_or_else(|| serde::de::Error::missing_field("indices"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FieldCapsResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("fields", &self.fields)?;
        map.serialize_entry("indices", &self.indices)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
