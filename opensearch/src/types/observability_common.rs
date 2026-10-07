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

//! Response types of the `observability_common` specification namespace

#![allow(missing_docs)]

#[doc = "The core observability object containing visualization and query data."]
#[derive(Debug, Clone)]
pub struct ObservabilityObject {
    #[doc = "When the object was created in milliseconds."]
    pub created_time_ms: Option<i64>,
    #[doc = "When the object was last updated in milliseconds."]
    pub last_updated_time_ms: Option<i64>,
    #[doc = "The unique identifier of the observability object."]
    pub object_id: String,
    pub operational_panel: Option<crate::types::observability_common::OperationalPanel>,
    pub saved_query: Option<crate::types::observability_common::SavedQuery>,
    pub saved_visualization: Option<crate::types::observability_common::SavedVisualization>,
    #[doc = "The tenant associated with the observability object."]
    pub tenant: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ObservabilityObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ObservabilityObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ObservabilityObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_created_time_ms: Option<i64> = None;
                let mut seen_created_time_ms = false;
                let mut f_last_updated_time_ms: Option<i64> = None;
                let mut seen_last_updated_time_ms = false;
                let mut f_object_id: Option<String> = None;
                let mut f_operational_panel: Option<
                    crate::types::observability_common::OperationalPanel,
                > = None;
                let mut seen_operational_panel = false;
                let mut f_saved_query: Option<crate::types::observability_common::SavedQuery> =
                    None;
                let mut seen_saved_query = false;
                let mut f_saved_visualization: Option<
                    crate::types::observability_common::SavedVisualization,
                > = None;
                let mut seen_saved_visualization = false;
                let mut f_tenant: Option<String> = None;
                let mut seen_tenant = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "createdTimeMs" => {
                            if seen_created_time_ms {
                                return Err(serde::de::Error::duplicate_field("createdTimeMs"));
                            }
                            seen_created_time_ms = true;
                            f_created_time_ms = map.next_value::<Option<i64>>()?;
                        }
                        "lastUpdatedTimeMs" => {
                            if seen_last_updated_time_ms {
                                return Err(serde::de::Error::duplicate_field("lastUpdatedTimeMs"));
                            }
                            seen_last_updated_time_ms = true;
                            f_last_updated_time_ms = map.next_value::<Option<i64>>()?;
                        }
                        "objectId" => {
                            if f_object_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectId"));
                            }
                            f_object_id = Some(map.next_value::<String>()?);
                        }
                        "operationalPanel" => {
                            if seen_operational_panel {
                                return Err(serde::de::Error::duplicate_field("operationalPanel"));
                            }
                            seen_operational_panel = true;
                            f_operational_panel = map.next_value::<Option<crate::types::observability_common::OperationalPanel>>()?;
                        }
                        "savedQuery" => {
                            if seen_saved_query {
                                return Err(serde::de::Error::duplicate_field("savedQuery"));
                            }
                            seen_saved_query = true;
                            f_saved_query = map.next_value::<Option<crate::types::observability_common::SavedQuery>>()?;
                        }
                        "savedVisualization" => {
                            if seen_saved_visualization {
                                return Err(serde::de::Error::duplicate_field(
                                    "savedVisualization",
                                ));
                            }
                            seen_saved_visualization = true;
                            f_saved_visualization = map.next_value::<Option<crate::types::observability_common::SavedVisualization>>()?;
                        }
                        "tenant" => {
                            if seen_tenant {
                                return Err(serde::de::Error::duplicate_field("tenant"));
                            }
                            seen_tenant = true;
                            f_tenant = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ObservabilityObject {
                    created_time_ms: f_created_time_ms,
                    last_updated_time_ms: f_last_updated_time_ms,
                    object_id: f_object_id
                        .ok_or_else(|| serde::de::Error::missing_field("objectId"))?,
                    operational_panel: f_operational_panel,
                    saved_query: f_saved_query,
                    saved_visualization: f_saved_visualization,
                    tenant: f_tenant,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ObservabilityObject {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.created_time_ms {
            map.serialize_entry("createdTimeMs", v)?;
        }
        if let Some(v) = &self.last_updated_time_ms {
            map.serialize_entry("lastUpdatedTimeMs", v)?;
        }
        map.serialize_entry("objectId", &self.object_id)?;
        if let Some(v) = &self.operational_panel {
            map.serialize_entry("operationalPanel", v)?;
        }
        if let Some(v) = &self.saved_query {
            map.serialize_entry("savedQuery", v)?;
        }
        if let Some(v) = &self.saved_visualization {
            map.serialize_entry("savedVisualization", v)?;
        }
        if let Some(v) = &self.tenant {
            map.serialize_entry("tenant", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "A list of observability objects with pagination details."]
#[derive(Debug, Clone)]
pub struct ObservabilityObjectList {
    #[doc = "The list of observability objects."]
    pub observability_object_list: Vec<crate::types::observability_common::ObservabilityObject>,
    #[doc = "The starting index of the returned objects."]
    pub start_index: i64,
    #[doc = "The relationship of total hits to the query."]
    pub total_hit_relation: String,
    #[doc = "The total number of matching objects."]
    pub total_hits: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ObservabilityObjectList {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ObservabilityObjectList;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ObservabilityObjectList object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_observability_object_list: Option<
                    Vec<crate::types::observability_common::ObservabilityObject>,
                > = None;
                let mut f_start_index: Option<i64> = None;
                let mut f_total_hit_relation: Option<String> = None;
                let mut f_total_hits: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "observabilityObjectList" => {
                            if f_observability_object_list.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "observabilityObjectList",
                                ));
                            }
                            f_observability_object_list =
                                Some(
                                    map.next_value::<Vec<
                                        crate::types::observability_common::ObservabilityObject,
                                    >>()?,
                                );
                        }
                        "startIndex" => {
                            if f_start_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("startIndex"));
                            }
                            f_start_index = Some(map.next_value::<i64>()?);
                        }
                        "totalHitRelation" => {
                            if f_total_hit_relation.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalHitRelation"));
                            }
                            f_total_hit_relation = Some(map.next_value::<String>()?);
                        }
                        "totalHits" => {
                            if f_total_hits.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalHits"));
                            }
                            f_total_hits = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ObservabilityObjectList {
                    observability_object_list: f_observability_object_list.ok_or_else(|| {
                        serde::de::Error::missing_field("observabilityObjectList")
                    })?,
                    start_index: f_start_index
                        .ok_or_else(|| serde::de::Error::missing_field("startIndex"))?,
                    total_hit_relation: f_total_hit_relation
                        .ok_or_else(|| serde::de::Error::missing_field("totalHitRelation"))?,
                    total_hits: f_total_hits
                        .ok_or_else(|| serde::de::Error::missing_field("totalHits"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ObservabilityObjectList {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("observabilityObjectList", &self.observability_object_list)?;
        map.serialize_entry("startIndex", &self.start_index)?;
        map.serialize_entry("totalHitRelation", &self.total_hit_relation)?;
        map.serialize_entry("totalHits", &self.total_hits)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The panel configuration for operational visualizations."]
#[derive(Debug, Clone)]
pub struct OperationalPanel {
    #[doc = "The identifier of the associated application."]
    pub application_id: String,
    #[doc = "The name of the operational panel."]
    pub name: String,
    pub query_filter: crate::types::observability_common::QueryFilter,
    pub time_range: crate::types::observability_common::TimeRange,
    #[doc = "The list of visualizations in the panel."]
    pub visualizations: Vec<crate::types::observability_common::Visualization>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for OperationalPanel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = OperationalPanel;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a OperationalPanel object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_application_id: Option<String> = None;
                let mut f_name: Option<String> = None;
                let mut f_query_filter: Option<crate::types::observability_common::QueryFilter> =
                    None;
                let mut f_time_range: Option<crate::types::observability_common::TimeRange> = None;
                let mut f_visualizations: Option<
                    Vec<crate::types::observability_common::Visualization>,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "applicationId" => {
                            if f_application_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("applicationId"));
                            }
                            f_application_id = Some(map.next_value::<String>()?);
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<String>()?);
                        }
                        "queryFilter" => {
                            if f_query_filter.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryFilter"));
                            }
                            f_query_filter = Some(
                                map.next_value::<crate::types::observability_common::QueryFilter>(
                                )?,
                            );
                        }
                        "timeRange" => {
                            if f_time_range.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeRange"));
                            }
                            f_time_range = Some(
                                map.next_value::<crate::types::observability_common::TimeRange>()?,
                            );
                        }
                        "visualizations" => {
                            if f_visualizations.is_some() {
                                return Err(serde::de::Error::duplicate_field("visualizations"));
                            }
                            f_visualizations = Some(map.next_value::<Vec<crate::types::observability_common::Visualization>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(OperationalPanel {
                    application_id: f_application_id
                        .ok_or_else(|| serde::de::Error::missing_field("applicationId"))?,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    query_filter: f_query_filter
                        .ok_or_else(|| serde::de::Error::missing_field("queryFilter"))?,
                    time_range: f_time_range
                        .ok_or_else(|| serde::de::Error::missing_field("timeRange"))?,
                    visualizations: f_visualizations
                        .ok_or_else(|| serde::de::Error::missing_field("visualizations"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for OperationalPanel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("applicationId", &self.application_id)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("queryFilter", &self.query_filter)?;
        map.serialize_entry("timeRange", &self.time_range)?;
        map.serialize_entry("visualizations", &self.visualizations)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The filter configuration for queries."]
#[derive(Debug, Clone)]
pub struct QueryFilter {
    #[doc = "The query language used."]
    pub language: String,
    #[doc = "The query string."]
    pub query: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for QueryFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = QueryFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a QueryFilter object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_language: Option<String> = None;
                let mut f_query: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "language" => {
                            if f_language.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            f_language = Some(map.next_value::<String>()?);
                        }
                        "query" => {
                            if f_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            f_query = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(QueryFilter {
                    language: f_language
                        .ok_or_else(|| serde::de::Error::missing_field("language"))?,
                    query: f_query.ok_or_else(|| serde::de::Error::missing_field("query"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for QueryFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("language", &self.language)?;
        map.serialize_entry("query", &self.query)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for a saved query."]
#[derive(Debug, Clone)]
pub struct SavedQuery {
    #[doc = "The description of the saved query."]
    pub description: String,
    #[doc = "The name of the saved query."]
    pub name: String,
    #[doc = "The query string."]
    pub query: String,
    pub selected_date_range: crate::types::observability_common::SelectedDateRange,
    pub selected_fields: crate::types::observability_common::SelectedFields,
    pub selected_timestamp: crate::types::observability_common::SelectedTimestamp,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SavedQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SavedQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SavedQuery object")
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
                let mut f_name: Option<String> = None;
                let mut f_query: Option<String> = None;
                let mut f_selected_date_range: Option<
                    crate::types::observability_common::SelectedDateRange,
                > = None;
                let mut f_selected_fields: Option<
                    crate::types::observability_common::SelectedFields,
                > = None;
                let mut f_selected_timestamp: Option<
                    crate::types::observability_common::SelectedTimestamp,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if f_description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            f_description = Some(map.next_value::<String>()?);
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<String>()?);
                        }
                        "query" => {
                            if f_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            f_query = Some(map.next_value::<String>()?);
                        }
                        "selected_date_range" => {
                            if f_selected_date_range.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "selected_date_range",
                                ));
                            }
                            f_selected_date_range = Some(map.next_value::<crate::types::observability_common::SelectedDateRange>()?);
                        }
                        "selected_fields" => {
                            if f_selected_fields.is_some() {
                                return Err(serde::de::Error::duplicate_field("selected_fields"));
                            }
                            f_selected_fields = Some(map.next_value::<crate::types::observability_common::SelectedFields>()?);
                        }
                        "selected_timestamp" => {
                            if f_selected_timestamp.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "selected_timestamp",
                                ));
                            }
                            f_selected_timestamp = Some(map.next_value::<crate::types::observability_common::SelectedTimestamp>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SavedQuery {
                    description: f_description
                        .ok_or_else(|| serde::de::Error::missing_field("description"))?,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    query: f_query.ok_or_else(|| serde::de::Error::missing_field("query"))?,
                    selected_date_range: f_selected_date_range
                        .ok_or_else(|| serde::de::Error::missing_field("selected_date_range"))?,
                    selected_fields: f_selected_fields
                        .ok_or_else(|| serde::de::Error::missing_field("selected_fields"))?,
                    selected_timestamp: f_selected_timestamp
                        .ok_or_else(|| serde::de::Error::missing_field("selected_timestamp"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SavedQuery {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("description", &self.description)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("query", &self.query)?;
        map.serialize_entry("selected_date_range", &self.selected_date_range)?;
        map.serialize_entry("selected_fields", &self.selected_fields)?;
        map.serialize_entry("selected_timestamp", &self.selected_timestamp)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for a saved visualization."]
#[derive(Debug, Clone)]
pub struct SavedVisualization {
    #[doc = "The description of the saved visualization."]
    pub description: String,
    #[doc = "The name of the saved visualization."]
    pub name: String,
    #[doc = "The query associated with the visualization."]
    pub query: String,
    pub selected_date_range: crate::types::observability_common::SelectedDateRange,
    pub selected_fields: crate::types::observability_common::SelectedFields,
    pub selected_timestamp: crate::types::observability_common::SelectedTimestamp,
    #[doc = "The type of visualization."]
    pub r#type: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SavedVisualization {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SavedVisualization;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SavedVisualization object")
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
                let mut f_name: Option<String> = None;
                let mut f_query: Option<String> = None;
                let mut f_selected_date_range: Option<
                    crate::types::observability_common::SelectedDateRange,
                > = None;
                let mut f_selected_fields: Option<
                    crate::types::observability_common::SelectedFields,
                > = None;
                let mut f_selected_timestamp: Option<
                    crate::types::observability_common::SelectedTimestamp,
                > = None;
                let mut f_type: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "description" => {
                            if f_description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            f_description = Some(map.next_value::<String>()?);
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<String>()?);
                        }
                        "query" => {
                            if f_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            f_query = Some(map.next_value::<String>()?);
                        }
                        "selected_date_range" => {
                            if f_selected_date_range.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "selected_date_range",
                                ));
                            }
                            f_selected_date_range = Some(map.next_value::<crate::types::observability_common::SelectedDateRange>()?);
                        }
                        "selected_fields" => {
                            if f_selected_fields.is_some() {
                                return Err(serde::de::Error::duplicate_field("selected_fields"));
                            }
                            f_selected_fields = Some(map.next_value::<crate::types::observability_common::SelectedFields>()?);
                        }
                        "selected_timestamp" => {
                            if f_selected_timestamp.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "selected_timestamp",
                                ));
                            }
                            f_selected_timestamp = Some(map.next_value::<crate::types::observability_common::SelectedTimestamp>()?);
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
                Ok(SavedVisualization {
                    description: f_description
                        .ok_or_else(|| serde::de::Error::missing_field("description"))?,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    query: f_query.ok_or_else(|| serde::de::Error::missing_field("query"))?,
                    selected_date_range: f_selected_date_range
                        .ok_or_else(|| serde::de::Error::missing_field("selected_date_range"))?,
                    selected_fields: f_selected_fields
                        .ok_or_else(|| serde::de::Error::missing_field("selected_fields"))?,
                    selected_timestamp: f_selected_timestamp
                        .ok_or_else(|| serde::de::Error::missing_field("selected_timestamp"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SavedVisualization {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("description", &self.description)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("query", &self.query)?;
        map.serialize_entry("selected_date_range", &self.selected_date_range)?;
        map.serialize_entry("selected_fields", &self.selected_fields)?;
        map.serialize_entry("selected_timestamp", &self.selected_timestamp)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The date range selection configuration."]
#[derive(Debug, Clone)]
pub struct SelectedDateRange {
    #[doc = "The end date of the range."]
    pub end: String,
    #[doc = "The start date of the range."]
    pub start: String,
    #[doc = "The text representation of the date range."]
    pub text: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SelectedDateRange {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SelectedDateRange;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SelectedDateRange object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_end: Option<String> = None;
                let mut f_start: Option<String> = None;
                let mut f_text: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "end" => {
                            if f_end.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            f_end = Some(map.next_value::<String>()?);
                        }
                        "start" => {
                            if f_start.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            f_start = Some(map.next_value::<String>()?);
                        }
                        "text" => {
                            if f_text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            f_text = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SelectedDateRange {
                    end: f_end.ok_or_else(|| serde::de::Error::missing_field("end"))?,
                    start: f_start.ok_or_else(|| serde::de::Error::missing_field("start"))?,
                    text: f_text.ok_or_else(|| serde::de::Error::missing_field("text"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SelectedDateRange {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("end", &self.end)?;
        map.serialize_entry("start", &self.start)?;
        map.serialize_entry("text", &self.text)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The field selection configuration."]
#[derive(Debug, Clone)]
pub struct SelectedFields {
    #[doc = "The text representation of selected fields."]
    pub text: String,
    #[doc = "The list of field tokens."]
    pub tokens: Vec<crate::types::observability_common::Token>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SelectedFields {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SelectedFields;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SelectedFields object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_text: Option<String> = None;
                let mut f_tokens: Option<Vec<crate::types::observability_common::Token>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "text" => {
                            if f_text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            f_text = Some(map.next_value::<String>()?);
                        }
                        "tokens" => {
                            if f_tokens.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokens"));
                            }
                            f_tokens = Some(
                                map.next_value::<Vec<crate::types::observability_common::Token>>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SelectedFields {
                    text: f_text.ok_or_else(|| serde::de::Error::missing_field("text"))?,
                    tokens: f_tokens.ok_or_else(|| serde::de::Error::missing_field("tokens"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SelectedFields {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("text", &self.text)?;
        map.serialize_entry("tokens", &self.tokens)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The timestamp field selection configuration."]
#[derive(Debug, Clone)]
pub struct SelectedTimestamp {
    #[doc = "The name of the selected timestamp field."]
    pub name: String,
    #[doc = "The type of the timestamp field."]
    pub r#type: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SelectedTimestamp {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SelectedTimestamp;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SelectedTimestamp object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut f_type: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<String>()?);
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
                Ok(SelectedTimestamp {
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SelectedTimestamp {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The time range configuration for queries and visualizations."]
#[derive(Debug, Clone)]
pub struct TimeRange {
    #[doc = "The start time of the range."]
    pub from: String,
    #[doc = "The end time of the range."]
    pub to: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TimeRange {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TimeRange;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TimeRange object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_from: Option<String> = None;
                let mut f_to: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "from" => {
                            if f_from.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            f_from = Some(map.next_value::<String>()?);
                        }
                        "to" => {
                            if f_to.is_some() {
                                return Err(serde::de::Error::duplicate_field("to"));
                            }
                            f_to = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TimeRange {
                    from: f_from.ok_or_else(|| serde::de::Error::missing_field("from"))?,
                    to: f_to.ok_or_else(|| serde::de::Error::missing_field("to"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TimeRange {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("from", &self.from)?;
        map.serialize_entry("to", &self.to)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for a field token."]
#[derive(Debug, Clone)]
pub struct Token {
    #[doc = "The name of the token."]
    pub name: String,
    #[doc = "The type of the token."]
    pub r#type: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Token {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Token;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Token object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_name: Option<String> = None;
                let mut f_type: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<String>()?);
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
                Ok(Token {
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Token {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The configuration for a single visualization."]
#[derive(Debug, Clone)]
pub struct Visualization {
    #[doc = "The height of the visualization."]
    pub h: i64,
    #[doc = "The unique identifier of the visualization."]
    pub id: String,
    #[doc = "The identifier of the saved visualization."]
    pub saved_visualization_id: String,
    #[doc = "The width of the visualization."]
    pub w: i64,
    #[doc = "The x-coordinate position of the visualization."]
    pub x: i64,
    #[doc = "The y-coordinate position of the visualization."]
    pub y: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Visualization {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Visualization;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Visualization object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_h: Option<i64> = None;
                let mut f_id: Option<String> = None;
                let mut f_saved_visualization_id: Option<String> = None;
                let mut f_w: Option<i64> = None;
                let mut f_x: Option<i64> = None;
                let mut f_y: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "h" => {
                            if f_h.is_some() {
                                return Err(serde::de::Error::duplicate_field("h"));
                            }
                            f_h = Some(map.next_value::<i64>()?);
                        }
                        "id" => {
                            if f_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            f_id = Some(map.next_value::<String>()?);
                        }
                        "savedVisualizationId" => {
                            if f_saved_visualization_id.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "savedVisualizationId",
                                ));
                            }
                            f_saved_visualization_id = Some(map.next_value::<String>()?);
                        }
                        "w" => {
                            if f_w.is_some() {
                                return Err(serde::de::Error::duplicate_field("w"));
                            }
                            f_w = Some(map.next_value::<i64>()?);
                        }
                        "x" => {
                            if f_x.is_some() {
                                return Err(serde::de::Error::duplicate_field("x"));
                            }
                            f_x = Some(map.next_value::<i64>()?);
                        }
                        "y" => {
                            if f_y.is_some() {
                                return Err(serde::de::Error::duplicate_field("y"));
                            }
                            f_y = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Visualization {
                    h: f_h.ok_or_else(|| serde::de::Error::missing_field("h"))?,
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("id"))?,
                    saved_visualization_id: f_saved_visualization_id
                        .ok_or_else(|| serde::de::Error::missing_field("savedVisualizationId"))?,
                    w: f_w.ok_or_else(|| serde::de::Error::missing_field("w"))?,
                    x: f_x.ok_or_else(|| serde::de::Error::missing_field("x"))?,
                    y: f_y.ok_or_else(|| serde::de::Error::missing_field("y"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Visualization {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("h", &self.h)?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("savedVisualizationId", &self.saved_visualization_id)?;
        map.serialize_entry("w", &self.w)?;
        map.serialize_entry("x", &self.x)?;
        map.serialize_entry("y", &self.y)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
